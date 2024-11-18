use std::default;
use std::fmt::{self, Debug};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::de::{self, MapAccess};
use serde::ser::SerializeStruct;
use serde::{de::Visitor, Deserialize, Serialize};
use uuid::Uuid;

use crate::Message;

// 一个type只能与一个结构体一对一
#[derive(Debug, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[repr(u64)]
pub enum MessageType {
    #[default]
    None,
    Quit,
    Move,
    Join,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MsgInfo {
    pub msg_type: MessageType,
    pub uid: Uuid,
}

impl MsgInfo {
    pub fn new(msg_type:MessageType) -> Self {
        Self { msg_type, uid: Uuid::new_v4() }
    }
}

pub struct Msg {
    pub info: MsgInfo,
    pub data: Option<Box<dyn Message>>,
    pub row_data: Option<Vec<u8>>,
}

impl Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Msg")
            .field("info", &self.info)
            .field("data", &"Box<dyn Message>") // 这里手动处理
            .field("row_data", &self.row_data)
            .finish()
    }
}

impl Serialize for Msg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Msg", 4)?;
        state.serialize_field("info", &self.info)?;

        // 将 data 转换为 row_data
        if let Some(data) = &self.data {
            state.serialize_field("row_data", &data.encode())?;
        } else {
            state.serialize_field("row_data", &self.row_data)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Msg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Info,
            Data,
            RowData,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`info`, `data`, or `row_data`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "info" => Ok(Field::Info),
                            "data" => Ok(Field::Data),
                            "row_data" => Ok(Field::RowData),
                            _ => Err(de::Error::unknown_field(
                                value,
                                &["msg_type", "uid", "data", "row_data"],
                            )),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MsgVisitor;

        impl<'de> Visitor<'de> for MsgVisitor {
            type Value = Msg;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Msg")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Msg, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut info = None;
                let mut data = None;
                let mut row_data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Info => {
                            if info.is_some() {
                                return Err(de::Error::duplicate_field("info"));
                            }
                            info = Some(map.next_value()?);
                        }
                        Field::Data => {
                            // 确保 data 为 None
                            data = None;
                        }
                        Field::RowData => {
                            if row_data.is_some() {
                                return Err(de::Error::duplicate_field("row_data"));
                            }
                            row_data = Some(map.next_value()?);
                        }
                    }
                }

                let info = info.ok_or_else(|| de::Error::missing_field("info"))?;
                let row_data = row_data.ok_or_else(|| de::Error::missing_field("row_data"))?;

                Ok(Msg {
                    info,
                    data,
                    row_data: Some(row_data),
                })
            }
        }

        deserializer.deserialize_struct("Msg", &["msg_type", "uid", "data", "row_data"], MsgVisitor)
    }
}

impl Msg {
    pub fn new() -> Self {
        Msg {
            info: MsgInfo::default(),
            data: None,
            row_data: None,
        }
    }

    pub fn set_msg_type(&mut self, msg_type: MessageType) {
        self.info = MsgInfo::default();
    }

    pub fn set_data<T: Message + 'static>(&mut self, data: T) {
        let data = Box::new(data);
        self.data = Some(data);
    }

    pub fn set_row_data(&mut self, row_data: Vec<u8>) {
        self.row_data = Some(row_data);
    }

    pub fn get_msg_type(&self) -> MessageType {
        self.info.msg_type.clone()
    }

    pub fn get_uid(&self) -> Uuid {
        self.info.uid.clone()
    }

    pub fn get_data<T: Message + for<'de> Deserialize<'de>>(&self) -> Option<T> {
        if let Some(data) = &self.row_data {
            serde_json::from_slice(data).ok()
        } else {
            None
        }
    }
}

impl Default for Msg {
    fn default() -> Self {
        Self::new()
    }
}

