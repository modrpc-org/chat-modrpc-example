#![allow(unused_variables)]

use crate::interface::ChatInterface;
use crate::proto::{ChatInitState, ChatServerConfig, RegisterError, RegisterRequest, RegisterSuccess, SendMessageError, SendMessageRequest, SendMessageSuccess};
use modrpc::{InterfaceRole, RoleSetup};
use std_modrpc::{RequestInitState, RequestServer, RequestServerBuilder, RequestServerConfig, RequestServerRole};

pub struct ChatServerHooks {
    pub register: RequestServer<RegisterRequest, Result<RegisterSuccess, RegisterError>>,
    pub send_message: RequestServer<SendMessageRequest, Result<SendMessageSuccess, SendMessageError>>,
}

pub struct ChatServerStubs {
    pub register: RequestServerBuilder<RegisterRequest, Result<RegisterSuccess, RegisterError>>,
    pub send_message: RequestServerBuilder<SendMessageRequest, Result<SendMessageSuccess, SendMessageError>>,
}

pub struct ChatServerRole {}

impl InterfaceRole for ChatServerRole {
    type Interface = ChatInterface;
    type Config = ChatServerConfig;
    type Init = ChatInitState;
    type Stubs = ChatServerStubs;
    type Hooks = ChatServerHooks;

    fn setup_worker(
        i: &Self::Interface,
        setup: &mut RoleSetup,
        config: &Self::Config,
        init: &Self::Init,
    ) -> (Self::Stubs, Self::Hooks) {
        setup.push_object_path("register");
        let (register_stubs, register_hooks) =
            RequestServerRole::setup_worker(
                &i.register, setup, &RequestServerConfig { }, &RequestInitState { },
            );
        let register_builder = RequestServerBuilder::new(
            "chat_server.register",
            register_hooks,
            register_stubs,
            &RequestServerConfig { },
            RequestInitState { }.clone(),
        );
        let register = register_builder.create_handle(setup);
        setup.pop_object_path();
        setup.push_object_path("send_message");
        let (send_message_stubs, send_message_hooks) =
            RequestServerRole::setup_worker(
                &i.send_message, setup, &RequestServerConfig { }, &RequestInitState { },
            );
        let send_message_builder = RequestServerBuilder::new(
            "chat_server.send_message",
            send_message_hooks,
            send_message_stubs,
            &RequestServerConfig { },
            RequestInitState { }.clone(),
        );
        let send_message = send_message_builder.create_handle(setup);
        setup.pop_object_path();

        (
            Self::Stubs {
                register: register_builder,
                send_message: send_message_builder,
            },
            Self::Hooks {
                register,
                send_message,
            },
        )
    }
}

impl Clone for ChatServerHooks {
    fn clone(&self) -> Self {
        Self {
            register: self.register.clone(),
            send_message: self.send_message.clone(),
        }
    }
}
