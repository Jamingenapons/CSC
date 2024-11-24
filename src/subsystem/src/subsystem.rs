
use std::{collections::HashMap, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use crossbeam::channel::{self, Receiver};
use message::{MessageType, Msg};

mod tcpsystem;

pub enum SystemType {
    
}


pub trait SubSystem {
    type Msg;
    fn exec(&mut self, msg:&mut Msg){
        match msg.get_msg_type() {
            _ => {}
        }
    }
    fn rollup(&mut self);
}

pub struct CenterSubsystem{
    bus:Vec<Arc<Mutex<Msg>>>,
    subsystems:HashMap<SystemType,(Box<dyn SubSystem<Msg = Msg>>, Vec<MessageType>)>,
}

impl CenterSubsystem{
    async fn dispatch(&mut self){
        for msg in self.bus.drain(..) {
            let msg = msg.clone();
            for (system_type,(subsystem,msg_types)) in self.subsystems.iter_mut() {
                let mut inner_msg = msg.lock().unwrap();
                if msg_types.contains(&inner_msg.get_msg_type()) {
                    subsystem.exec(inner_msg.deref_mut());
                }
            }
        }
    }
}

