use std::fmt;

use serde::Serialize;
use uuid::Uuid;

use crate::{msg::Msg, Message, MessageType};





pub struct MsgBuilder {
    msg_type: Option<MessageType>,
    uid: Option<Uuid>,
    data: Option<Box<dyn Message>>,
    row_data: Option<Vec<u8>>,
}

impl MsgBuilder {
    pub fn new() -> Self {
        MsgBuilder {
            msg_type: None,
            uid: None,
            data: None,
            row_data: None,
        }
    }

    pub fn msg_type(mut self, msg_type: MessageType) -> Self {
        self.msg_type = Some(msg_type);
        self
    }

    pub fn data(mut self, data: Box<dyn Message>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Msg, &'static str> {
        let msg_type = self.msg_type.ok_or("msg_type is required")?;
        let data = self.data.ok_or("data is required")?;

        Ok(Msg {
            msg_type,
            uid:Uuid::new_v4(),
            data:Some(data),
            row_data:Some(Vec::new()),
        })
    }
}

