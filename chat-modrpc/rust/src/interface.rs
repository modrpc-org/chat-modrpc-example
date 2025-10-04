use crate::proto::{RegisterError, RegisterRequest, RegisterSuccess, SendMessageError, SendMessageRequest, SendMessageSuccess};
use modrpc::{InterfaceBuilder, InterfaceSchema};
use std_modrpc::RequestInterface;

pub struct ChatInterface {
    pub register: RequestInterface<RegisterRequest, Result<RegisterSuccess, RegisterError>>,
    pub send_message: RequestInterface<SendMessageRequest, Result<SendMessageSuccess, SendMessageError>>,
}

impl InterfaceSchema for ChatInterface {
    fn new(ib: &mut InterfaceBuilder) -> Self {
        Self {
            register: RequestInterface::new(ib),
            send_message: RequestInterface::new(ib),
        }
    }
}
