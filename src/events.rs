use std::{collections::VecDeque, default};

use rocket::{http::Header, Route};

use crate::_State;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }
}

impl Default for Coordinates {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}


#[derive(Debug, Clone)]
pub struct EventQueue {
    latest_rev: i128,
    events: Vec<Event>,
    events_stack: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            latest_rev: 0,
            events: Vec::new(),
            events_stack: VecDeque::new(),
        }
    }
    
    pub fn addEvent(&mut self, event_type: EventType) {
        let next = self.latest_rev + 1;
        self.latest_rev = next;
        let event = Event::new(self.latest_rev, event_type);
        self.events.push(event.clone());
        self.events_stack.push_back(event);
    }
    
    pub fn remove_next_event(&mut self) -> Result<Event, &str> {
        self.events_stack.pop_front().ok_or("No events in the queue")
    }
    
    pub fn select_tile(&mut self, c: Coordinates){
        self.addEvent(EventType::SelectedTile(c));
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub order: i128,
    pub event_type: EventType,
}

impl Event {
    pub fn new(order: i128, event_type: EventType) -> Event {
        Event { order, event_type }
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
pub enum EventType {
    SelectedTile(Coordinates),
}

#[get("/")]
pub async fn event(state: &_State) -> htmx:: {
    state.get().await.remove_next_event();
    HXResponder::new("Event removed")
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/next_event", routes![event])
}




