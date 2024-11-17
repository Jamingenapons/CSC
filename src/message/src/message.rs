mod quit;
mod motor;


pub use quit::{*};
pub use motor::{*};


use std::fmt::Debug;


pub trait Message:Debug + Send {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: &[u8]) -> Option<Self> where Self: Sized;
}
