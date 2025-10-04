#![allow(unused_variables)]

use crate::interface::ChatInterface;
use crate::proto::{ChatClientConfig, ChatInitState, RegisterError, RegisterRequest, RegisterSuccess, SendMessageError, SendMessageRequest, SendMessageSuccess};
use modrpc::{InterfaceRole, RoleSetup};
use std_modrpc::{RequestClient, RequestClientBuilder, RequestClientConfig, RequestClientRole, RequestInitState};

pub struct ChatClientHooks {
    pub register: RequestClient<RegisterRequest, Result<RegisterSuccess, RegisterError>>,
    pub send_message: RequestClient<SendMessageRequest, Result<SendMessageSuccess, SendMessageError>>,
}

pub struct ChatClientStubs {}

pub struct ChatClientRole {}

impl InterfaceRole for ChatClientRole {
    type Interface = ChatInterface;
    type Config = ChatClientConfig;
    type Init = ChatInitState;
    type Stubs = ChatClientStubs;
    type Hooks = ChatClientHooks;

    fn setup_worker(
        i: &Self::Interface,
        setup: &mut RoleSetup,
        config: &Self::Config,
        init: &Self::Init,
    ) -> (Self::Stubs, Self::Hooks) {
        setup.push_object_path("register");
        let (register_stubs, register_hooks) =
            RequestClientRole::setup_worker(
                &i.register, setup, &RequestClientConfig { }, &RequestInitState { },
            );
        let register_builder = RequestClientBuilder::new(
            "chat_client.register",
            register_hooks,
            register_stubs,
            &RequestClientConfig { },
            RequestInitState { }.clone(),
        );
        let register = register_builder.create_handle(setup);
        register_builder.build(setup);
        setup.pop_object_path();
        setup.push_object_path("send_message");
        let (send_message_stubs, send_message_hooks) =
            RequestClientRole::setup_worker(
                &i.send_message, setup, &RequestClientConfig { }, &RequestInitState { },
            );
        let send_message_builder = RequestClientBuilder::new(
            "chat_client.send_message",
            send_message_hooks,
            send_message_stubs,
            &RequestClientConfig { },
            RequestInitState { }.clone(),
        );
        let send_message = send_message_builder.create_handle(setup);
        send_message_builder.build(setup);
        setup.pop_object_path();

        (
            Self::Stubs {},
            Self::Hooks {
                register,
                send_message,
            },
        )
    }
}

impl Clone for ChatClientHooks {
    fn clone(&self) -> Self {
        Self {
            register: self.register.clone(),
            send_message: self.send_message.clone(),
        }
    }
}
