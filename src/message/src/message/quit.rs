use serde::{Deserialize, Serialize};

use super::Message;






#[derive(Debug, Serialize, Deserialize)]
struct QuitMsg {
    value: i32,
}

impl Message for QuitMsg {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn decode(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}
