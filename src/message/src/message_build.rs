use std::fmt;

use serde::Serialize;
use uuid::Uuid;

use crate::{msg::Msg, Message, MessageType, MsgInfo};

pub struct MsgBuilder {
    info: Option<MsgInfo>,
    data: Option<Box<dyn Message>>,
    row_data: Option<Vec<u8>>,
}

impl MsgBuilder {
    pub fn new() -> Self {
        MsgBuilder {
            info: None,
            data: None,
            row_data: None,
        }
    }

    pub fn msg_type(mut self, msg_type: MessageType) -> Self {
        self.info = Some(MsgInfo::new(msg_type));
        self
    }

    pub fn data(mut self, data: Box<dyn Message>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Msg, &'static str> {
        let info = self.info.ok_or("info is required")?;
        let data = self.data.ok_or("data is required")?;

        Ok(Msg {
            info,
            data: Some(data),
            row_data: Some(Vec::new()),
        })
    }
}
