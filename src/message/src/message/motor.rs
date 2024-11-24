
pub(crate) use serde::{Deserialize, Serialize};

use super::Message;



#[derive(Debug, Serialize, Deserialize, Default)]
pub enum MoveDirection {
    #[default]
    Up,
}



#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MotorMsg {
    id: i32,
    direction:MoveDirection,
}

impl Message for MotorMsg {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn decode(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}


impl MotorMsg {
    pub fn new(id:i32, direction:MoveDirection) -> Self {
        Self {
            id: 0,
            direction: MoveDirection::Up,
        }
    }
}