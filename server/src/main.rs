use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

use modrpc_executor::ModrpcExecutor;

const MAX_USERS: usize = 1000;
const MAX_MESSAGE_LEN: usize = 500;

struct ChatHubDelegate {
    state: Rc<ChatServerState>,
}

impl modrpc_hub::AppHubDelegate for ChatHubDelegate {
    // :( one day maybe
    // type Init<'a> = impl mproto::Compatible<chat_modrpc::ChatInitState> + 'a;
    type Init<'a> = chat_modrpc::ChatInitStateGen<
        mproto::ListGen<std::collections::hash_map::Values<'a, u64, chat_modrpc::RegisteredUser>>,
    >;

    async fn client_handshake(
        &self,
        _: modrpc::EndpointAddr,
        handshake_fn: impl for<'a> AsyncFnOnce(Self::Init<'a>) -> std::io::Result<()>,
    ) -> std::io::Result<()> {
        let registered_users = self.state.registered_users.borrow();
        let init_state: Self::Init<'_> = chat_modrpc::ChatInitStateGen {
            users: mproto::ListGen(registered_users.values()),
        };
        handshake_fn(init_state).await
    }

    async fn client_disconnected(&self, endpoint_addr: modrpc::EndpointAddr) {
        if let Some(user) =
            self.state.registered_users.borrow_mut().remove(&endpoint_addr.endpoint)
        {
            log::info!("User disconnected: [endpoint={}] {}", endpoint_addr.endpoint, user.alias);
            self.state.registered_aliases.borrow_mut().remove(&user.alias);
        } else {
            log::info!("Unregistered client disconnected: [endpoint={}]", endpoint_addr.endpoint);
        }
    }
}

struct ChatServerState {
    // Map endpoint ID to its registered user object
    registered_users: RefCell<HashMap<u64, chat_modrpc::RegisteredUser>>,
    // Map user alias to registered endpoint ID
    registered_aliases: RefCell<HashMap<String, u64>>,
}

fn main() {
    env_logger::init();

    let state = Rc::new(ChatServerState {
        registered_users: RefCell::new(HashMap::new()),
        registered_aliases: RefCell::new(HashMap::new()),
    });

    let mut ex = modrpc_executor::TokioExecutor::new();
    let _guard = ex.tokio_runtime().enter();

    let buffer_pool = modrpc::HeapBufferPool::new(65536, 4, 4);
    let (rt, rt_shutdown) = modrpc::RuntimeHandle::single_threaded(&mut ex);

    ex.run_until(async move {
        let start_role = modrpc_hub::AppHubBuilder::new(
            buffer_pool.clone(),
            rt.clone(),
        )
        .with_tcp("0.0.0.0:9095".parse().unwrap())
        .with_websocket("0.0.0.0:9096".parse().unwrap())
        .build::<chat_modrpc::ChatServerRole, _>(
            ChatHubDelegate {
                state: state.clone(),
            },
            chat_modrpc::ChatServerConfig { },
            chat_modrpc::ChatInitState {
                users: vec![],
            },
        )
        .await;

        let shutdown_signal = start_role.role_shutdown_signal.clone();
        let _chat_server = start_role.local({
            let state = state.clone();
            move |cx| build_chat_server(cx, state.clone())
        });

        shutdown_signal.wait().await;

        rt_shutdown.shutdown().await;
    });
}

fn build_chat_server(
    cx: modrpc::RoleWorkerContext<chat_modrpc::ChatServerRole>,
    state: Rc<ChatServerState>,
) {
    cx.stubs.register.build_replier(cx.setup, {
        let state = state.clone();
        async move |mut cx, request| {
            if state.registered_aliases.borrow().len() >= MAX_USERS {
                cx.reply.send_err(chat_modrpc::RegisterError::ChatFull).await;
                return;
            }

            let Ok(alias) = request.alias() else {
                cx.reply.send_err(chat_modrpc::RegisterError::Internal).await;
                return;
            };

            log::info!("Registration request - endpoint={} alias={}", cx.source.endpoint, alias);

            if let Some(&existing_endpoint_with_alias) =
                state.registered_aliases.borrow().get(alias)
            {
                if existing_endpoint_with_alias == cx.source.endpoint {
                    // This endpoint is already registered as this user - let registration be
                    // idempotent.
                    cx.reply.send_ok(chat_modrpc::RegisterSuccess {}).await;
                    return;
                } else {
                    // Some other endpoint has already claimed this alias.
                    cx.reply.send_err(chat_modrpc::RegisterError::UserAlreadyExists).await;
                    return;
                }
            }

            state.registered_users.borrow_mut()
                .insert(cx.source.endpoint, chat_modrpc::RegisteredUser {
                    endpoint: cx.source.endpoint,
                    alias: alias.to_string(),
                });
            state.registered_aliases.borrow_mut().insert(alias.to_owned(), cx.source.endpoint);

            cx.reply.send_ok(chat_modrpc::RegisterSuccess {}).await;
        }
    });
    cx.stubs.send_message.build_replier(cx.setup, {
        let state = state.clone();
        async move |mut cx, request| {
            let Ok(content) = request.content() else {
                cx.reply.send_err(chat_modrpc::SendMessageError::Internal).await;
                return;
            };

            if content.len() > MAX_MESSAGE_LEN {
                cx.reply.send_err(chat_modrpc::SendMessageError::MessageTooLong).await;
                return;
            }

            let registered_users = state.registered_users.borrow();
            let Some(user) = registered_users.get(&cx.source.endpoint) else {
                cx.reply.send_err(chat_modrpc::SendMessageError::NotRegistered).await;
                return;
            };
            log::info!("[endpoint={}] {}: {content}", cx.source.endpoint, user.alias);
            drop(registered_users);

            cx.reply.send_ok(chat_modrpc::SendMessageSuccess {}).await;
        }
    });
}

