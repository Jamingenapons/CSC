use std::fmt::{self, Debug};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::ser::SerializeStruct;
use serde::{de::Visitor, Deserialize, Serialize};
use serde::de::{self, MapAccess};
use uuid::Uuid;

use crate::Message;

#[derive(Debug, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u64)]
pub enum MessageType {
    Quit,
    Move,
    Join,
}

pub struct Msg {
    pub msg_type: MessageType,
    pub uid: Uuid, // 用来确保发送和响应的是同一条消息
    pub data: Option<Box<dyn Message>>,
    pub row_data: Option<Vec<u8>>,
}

impl Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Msg")
            .field("msg_type", &self.msg_type)
            .field("uid", &self.uid)
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
        state.serialize_field("msg_type", &self.msg_type)?;
        state.serialize_field("uid", &self.uid)?;

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
        enum Field { MsgType, Uid, Data, RowData }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`msg_type`, `uid`, `data`, or `row_data`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "msg_type" => Ok(Field::MsgType),
                            "uid" => Ok(Field::Uid),
                            "data" => Ok(Field::Data),
                            "row_data" => Ok(Field::RowData),
                            _ => Err(de::Error::unknown_field(value, &["msg_type", "uid", "data", "row_data"])),
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
                let mut msg_type = None;
                let mut uid = None;
                let mut data = None;
                let mut row_data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::MsgType => {
                            if msg_type.is_some() {
                                return Err(de::Error::duplicate_field("msg_type"));
                            }
                            msg_type = Some(map.next_value()?);
                        }
                        Field::Uid => {
                            if uid.is_some() {
                                return Err(de::Error::duplicate_field("uid"));
                            }
                            uid = Some(map.next_value()?);
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

                let msg_type = msg_type.ok_or_else(|| de::Error::missing_field("msg_type"))?;
                let uid = uid.ok_or_else(|| de::Error::missing_field("uid"))?;
                let row_data = row_data.ok_or_else(|| de::Error::missing_field("row_data"))?;

                Ok(Msg {
                    msg_type,
                    uid,
                    data,
                    row_data: Some(row_data),
                })
            }
        }

        deserializer.deserialize_struct("Msg", &["msg_type", "uid", "data", "row_data"], MsgVisitor)
    }
}