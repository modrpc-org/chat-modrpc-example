#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use modrpc_executor::ModrpcExecutor;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[derive(Clone)]
struct Message {
    sender: String,
    content: String,
}

#[derive(Copy, Clone)]
pub struct AppContext {
    compose_message: Signal<String>,
    users: Signal<HashMap<u64, String>>,
    my_alias: Signal<Option<String>>,
    messages: Signal<Vec<Message>>,
}

fn App() -> Element {
    use_context_provider(|| AppContext {
        compose_message: Signal::new("".to_string()),
        users: Signal::new(HashMap::new()),
        my_alias: Signal::new(None),
        messages: Signal::new(vec![]),
    });

    let mut cx = consume_context::<AppContext>();
    let chat_client = use_resource(move || async move {
        let mut ex = modrpc_executor::DioxusExecutor::new();
        let spawner = ex.spawner();
        let (rt, _rt_shutdown) = modrpc::RuntimeHandle::single_threaded(&mut ex);
        let buffer_pool = modrpc::HeapBufferPool::new(65536, 4, 4);

        let connection =
            modrpc::web_ws_connect::<chat_modrpc::ChatClientRole>(
                &rt, buffer_pool.clone(), "ws://127.0.0.1:9096",
                chat_modrpc::ChatClientConfig {},
            )
            .await
            .unwrap();
        let modrpc::WebSocketConnection { init, role_handle: chat_client, .. } = connection;

        for user in init.users {
            info!("Registered user: {}", user.alias);
            cx.users.write().insert(user.endpoint, user.alias);
        }

        let local_endpoint = connection.endpoint;
        chat_client.register.subscribe(async move |source, request, response_waiter| {
            let Ok(response) = response_waiter.wait().await else {
                // Failed to decode response
                return;
            };
            if response.is_ok() {
                let Ok(alias) = request.alias() else {
                    return;
                };
                cx.users.write().insert(source.endpoint, alias.into());

                // XXX: Doing this directly in the task that makes the request causes this
                // subscription future to be cancelled before it can handle the response, so
                // handling the response to the local registration request here instead.
                if source == local_endpoint {
                    *cx.my_alias.write() = Some(alias.into());
                }
            }
        });
        chat_client.send_message.subscribe(async move |source, request, response_waiter| {
            let Ok(response) = response_waiter.wait().await else {
                // Failed to decode response
                return;
            };
            if response.is_ok() {
                let users = cx.users.read();
                let alias = users.get(&source.endpoint)
                    .map(AsRef::as_ref)
                    .unwrap_or("<unknown>");
                let Ok(content) = request.content() else {
                    return;
                };
                cx.messages.push(Message {
                    sender: alias.into(),
                    content: content.into(),
                });
            }
        });

        chat_client
    });
    use_context_provider(|| chat_client);

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        if chat_client.read().is_some() {
            if cx.my_alias.read().is_some() {
                Chat {}
            } else {
                Registration {}
            }
        } else {
            div {
                class: "connecting",
                "Connecting..."
            }
        }
    }
}

#[component]
fn Registration() -> Element {
    let mut cx = consume_context::<AppContext>();
    let mut chat_client_cx = consume_context::<Resource<chat_modrpc::ChatClientHooks>>();
    let mut register_alias = use_signal(|| String::new());

    let mut register = move || {
        let alias = register_alias.read().clone();
        if alias.is_empty() {
            return;
        }

        spawn(async move {
            let Some(chat_client) = &*chat_client_cx.read() else { return; };
            info!("Sending registration request");
            let response = chat_client.register.call(chat_modrpc::RegisterRequestGen {
                alias: &alias,
            })
            .await;
            info!("Got registration response: {:?}", response);
            if let Err(_) = response {
                // TODO display a helpful error
            }
        });
    };

    rsx! {
        div {
            class: "chat-container",
            div {
                class: "chat-header",
                "modrpc chat"
            }
            div {
                class: "input-form",
                input {
                    class: "input-field",
                    placeholder: "your desired alias",
                    value: "{register_alias}",
                    oninput: move |event| {
                        register_alias.set(event.value());
                    },
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            register();
                        }
                    }
                }
                button {
                    class: "send-button",
                    onclick: move |_| {
                        register();
                    },
                    "Register"
                }
            }
        }
    }
}

#[component]
fn MessageBubble(sender: String, content: String) -> Element {
    rsx! {
        div {
            class: "message-bubble",
            onmounted: move |cx| {
                spawn(async move {
                    let _ = cx.data.scroll_to(ScrollBehavior::Smooth).await;
                });
            },
            b { "{sender}: " } "{content}"
        }
    }
}

#[component]
fn Chat() -> Element {
    let mut cx = consume_context::<AppContext>();
    let mut chat_client_cx = consume_context::<Resource<chat_modrpc::ChatClientHooks>>();

    let mut send_message = move || {
        let message = cx.compose_message.read().clone();
        if message.is_empty() {
            return;
        }

        spawn(async move {
            let Some(chat_client) = &*chat_client_cx.read() else { return; };
            let response = chat_client.send_message.call(chat_modrpc::SendMessageRequest {
                content: message,
            })
            .await;
        });

        cx.compose_message.set("".to_string());
    };

    rsx! {
        div {
            class: "chat-container",
            div {
                class: "chat-header",
                "modrpc chat"
            }
            div {
                class: "message-area",
                for message in (cx.messages)() {
                    MessageBubble {
                        sender: message.sender.clone(),
                        content: message.content.clone(),
                    }
                }
            }
            div {
                class: "input-form",
                input {
                    class: "input-field",
                    placeholder: "Type a message...",
                    value: "{cx.compose_message}",
                    oninput: move |event| {
                        cx.compose_message.set(event.value());
                    },
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            send_message();
                        }
                    }
                }
                button {
                    class: "send-button",
                    onclick: move |_| {
                        send_message();
                    },
                    "Send"
                }
            }
        }
    }
}
