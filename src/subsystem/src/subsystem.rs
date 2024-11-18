
use std::collections::HashMap;

use crossbeam::channel::{self, Receiver};
use message::{MessageType, Msg};

pub enum SystemType {
    
}


pub trait SubSystem {
    type Msg;
    fn exec(&mut self, msg:Msg);
    fn rollup(&mut self);
}


pub struct SubSystemA {
    receiver:Receiver<Msg>,
    msgs:Vec<Msg>
}

impl SubSystemA {
    pub fn new(receiver:Receiver<Msg>) -> Self {
        SubSystemA {
            receiver,
            msgs:Vec::new()
        }
    }
}

impl SubSystem for SubSystemA {
    type Msg = Msg;
    fn exec(&mut self, msg:Msg) {
        match msg.get_msg_type() {
            message::MessageType::None => todo!(),
            message::MessageType::Quit => todo!(),
            message::MessageType::Move => todo!(),
            message::MessageType::Join => todo!(),
        }
    }

    fn rollup(&mut self) {
        self.msgs.clear();
    }
}

impl SubSystemA {
    fn run(&mut self){
        while let Some(msg) = self.msgs.pop() {
            self.exec(msg);
        }
    }
}

pub struct CenterSubsystem{
    bus:Vec<Msg>,
    subsystems:HashMap<SystemType,(Box<dyn SubSystem<Msg = Msg>>, Vec<MessageType>)>,
}

impl CenterSubsystem{
    fn dispatch(&mut self){
        for msg in self.bus.drain(..) {
            for (system_type,(subsystem,msg_types)) in self.subsystems.iter_mut() {
                if msg_types.contains(&msg.get_msg_type()) {
                    subsystem.exec(msg);
                }
            }
        }
    }
}

