use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Stdout},
    rc::Rc,
    time::Duration,
};

use crossterm::event::{EventStream, KeyCode, KeyEventKind};
use modrpc_executor::ModrpcExecutor;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

struct ChatClientState {
    users: RefCell<HashMap<u64, String>>,
}

fn main() {
    let mut ex = modrpc_executor::TokioExecutor::new();
    let spawner = ex.spawner();
    let _guard = ex.tokio_runtime().enter();

    let (rt, rt_shutdown) = modrpc::RuntimeHandle::single_threaded(&mut ex);
    let buffer_pool = modrpc::HeapBufferPool::new(65536, 4, 4);

    let mut terminal = ratatui::init();
    let result = ex.run_until(async {
        // Connect to the chat server
        let stream = tokio::net::TcpStream::connect("127.0.0.1:9095").await
            .expect("tcp stream connect");
        let connection =
            modrpc::tcp_connect::<chat_modrpc::ChatClientRole>(
                &rt,
                buffer_pool.clone(),
                modrpc::WorkerId::local(),
                chat_modrpc::ChatClientConfig { },
                stream,
            )
            .await
            .unwrap();
        let modrpc::TcpConnection { endpoint, init, role_handle: chat_client, .. } = connection;

        let mut users = HashMap::new();
        for user in init.users {
            users.insert(user.endpoint, user.alias);
        }

        let state = Rc::new(ChatClientState {
            users: RefCell::new(users),
        });

        // Subscriber to listen for new user registrations
        chat_client.register.subscribe({
            let state = state.clone();
            async move |source, request, response_waiter| {
                let Ok(response) = response_waiter.wait().await else {
                    // Failed to decode response
                    return;
                };
                if response.is_ok() {
                    let Ok(alias) = request.alias() else {
                        return;
                    };
                    state.users.borrow_mut().insert(source.endpoint, alias.to_string());
                }
            }
        });

        // Subscriber to listen for messages sent by peers and relay them to the UI.
        let (recv_messages_tx, recv_messages_rx) = localq::mpsc::channel(16);
        chat_client.send_message.subscribe({
            let state = state.clone();
            async move |source, request, response_waiter| {
                if source == endpoint {
                    // Ignore requests from self
                    return;
                }
                let Ok(response) = response_waiter.wait().await else {
                    // Failed to decode response
                    return;
                };
                if response.is_ok() {
                    let users = state.users.borrow();
                    let alias = users.get(&source.endpoint)
                        .map(AsRef::as_ref)
                        .unwrap_or("<unknown>");
                    let content = request.content().unwrap();
                    let _ = recv_messages_tx.send(Message {
                        sender: alias.into(),
                        content: content.into(),
                    })
                    .await;
                }
            }
        });

        // Spawn a task to relay messages from the UI to the server.
        let (send_messages_tx, mut send_messages_rx) = localq::mpsc::channel(16);
        spawner.spawn(async move {
            while let Ok(content) = send_messages_rx.recv().await {
                let _response = chat_client.send_message.call(chat_modrpc::SendMessageRequest {
                    content,
                })
                .await;
            }
        })
        .expect("spawn send messages");

        let _response = chat_client.register.call(chat_modrpc::RegisterRequestGen {
            alias: "anonymous-rustacean",
        })
        .await;

        // Run the UI
        let app = App::new(send_messages_tx);
        let result = run_app(&mut terminal, app, recv_messages_rx).await;

        rt_shutdown.shutdown().await;

        result
    });
    ratatui::restore();

    if let Err(e) = result {
        println!("{e:?}");
    }
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    mut recv_messages: localq::mpsc::Receiver<Message>,
) -> io::Result<()> {
    use futures::StreamExt;

    let frames_per_second = 10.0;
    let period = Duration::from_secs_f32(1.0 / frames_per_second);
    let mut interval = tokio::time::interval(period);
    let mut events = EventStream::new();

    loop {
        tokio::select! {
            _ = interval.tick() => {
                terminal.draw(|f| ui(f, &mut app))?;
            },
            Ok(message) = recv_messages.recv() => {
                app.add_message(message.sender, message.content);
            },
            Some(Ok(event)) = events.next() => {
                if let Some(key) = event.as_key_press_event() {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => return Ok(()), // Quit on Esc
                            _ => app.handle_input(key.code),
                        }
                    }
                }
                terminal.draw(|f| ui(f, &mut app))?;
            },
        }
    }
}

struct Message {
    sender: String,
    content: String,
}

struct App {
    messages: Vec<Message>,
    message_list_state: ListState,
    messages_rect: Rect,
    input: String,
    send_messages: localq::mpsc::Sender<String>,
}

impl App {
    fn new(send_messages: localq::mpsc::Sender<String>) -> App {
        App {
            messages: vec![
                Message {
                    sender: "System".to_string(),
                    content: "Welcome to the modrpc chat!".to_string(),
                },
                Message {
                    sender: "System".to_string(),
                    content: "Type your message below and press Enter.".to_string(),
                },
                Message {
                    sender: "System".to_string(),
                    content: "Press Esc to exit.".to_string(),
                },
            ],
            message_list_state: ListState::default(),
            messages_rect: Rect::default(),
            input: String::new(),
            send_messages,
        }
    }

    fn add_message(&mut self, sender: String, content: String) {
        self.messages.push(Message { sender, content });
        // Scroll to the bottom when a new message is added
        *self.message_list_state.offset_mut() =
            self.messages.len().saturating_sub(self.messages_rect.height as usize);
    }

    fn handle_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => {
                if !self.input.trim().is_empty() {
                    let message = std::mem::take(&mut self.input);
                    if let Ok(()) = self.send_messages.try_send(message.clone()) {
                        self.add_message("You".to_string(), message);
                    }
                }
            }
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                let _ = self.input.pop();
            }
            KeyCode::Up => {
                *self.message_list_state.offset_mut() =
                    self.message_list_state.offset().saturating_sub(1);
            }
            KeyCode::Down => {
                *self.message_list_state.offset_mut() = std::cmp::min(
                    self.messages.len(),
                    self.message_list_state.offset() + 1,
                );
            }
            _ => {}
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(f.area());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let sender_style = match m.sender.as_str() {
                "You" => Style::default().fg(Color::LightBlue),
                "System" => Style::default().fg(Color::Yellow),
                _ => Style::default().fg(Color::Green),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{}: ", m.sender), sender_style),
                Span::raw(&m.content),
            ]))
        })
        .collect();

    let messages_block = Block::default().borders(Borders::ALL).title("Chat History");
    app.messages_rect = messages_block.inner(chunks[0]);
    let messages_list = List::new(messages)
        .block(messages_block)
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(messages_list, chunks[0], &mut app.message_list_state);

    // Input text box
    let input_block = Block::default().borders(Borders::ALL)
        .title("Input (Press Enter to send, Up/Down to scroll)");
    let input_paragraph = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(input_block);

    f.render_widget(input_paragraph, chunks[1]);

    // Position the cursor at the end of the input text
    f.set_cursor_position((
        chunks[1].x + app.input.len() as u16 + 1, // +1 for the border
        chunks[1].y + 1, // +1 for the border
    ));
}

