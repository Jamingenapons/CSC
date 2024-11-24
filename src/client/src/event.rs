pub(crate) use std::{any::Any, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    None,
    KeyPress,
    MouseClick,
    HomeEvent,
    MotorEvent,
}


/// A trait that all events must implement.
pub trait Event: Any + Send + Sync + Debug {
    /// Returns a reference to the event as a trait object.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the event as a trait object.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_type(&self) -> EventType;
}

/// A struct to manage input events.
pub struct EventManager {
    events: Vec<Box<dyn Event>>,
}

impl EventManager {
    /// Creates a new `EventManager`.
    pub fn new() -> Self {
        EventManager { events: Vec::new() }
    }

    /// Adds an event to the manager.
    pub fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }

    /// Removes an event from the manager.
    pub fn remove_event(&mut self, event: Box<dyn Event>) {
        self.events.retain(|e| e.get_type() != event.get_type());
    }

    /// Returns an iterator over the events.
    pub fn iter(&self) -> impl Iterator<Item = &dyn Event> {
        self.events.iter().map(|e| &**e)
    }

    /// Pops the last event from the manager.
    pub fn pop(&mut self) -> Option<Box<dyn Event>> {
        self.events.pop()
    }
    
    /// Clears all events from the manager.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Returns the number of events in the manager.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Checks if the manager is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

// Example implementation of the Event trait
#[derive(Debug, PartialEq)]
pub struct KeyPressEvent {
    key_code: u32,
}

impl Event for KeyPressEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn get_type(&self) -> EventType {
        EventType::KeyPress
    }
}

// Another example implementation of the Event trait
#[derive(Debug, PartialEq)]
pub struct MouseClickEvent {
    x: i32,
    y: i32,
}

impl Event for MouseClickEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn get_type(&self) -> EventType {
        EventType::MouseClick
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct MotorEvent {
    x: i32,
    y: i32,
}

impl Event for MotorEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn get_type(&self) -> EventType {
        EventType::MotorEvent
    }
}

