//

use std::collections::VecDeque;

pub struct EventBuffer {
    buffer: VecDeque<String>,
    capacity: usize,
    last_event: *const String,
}

impl EventBuffer {
    pub fn new(capacity: usize) -> Self {
        EventBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            last_event: std::ptr::null(),
        }
    }

    pub fn push(&mut self, event: String) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(event);
        self.last_event = self.buffer.back().unwrap() as *const String;
    }

    pub fn get_last_event(&self) -> Option<&str> {
        if self.last_event.is_null() {
            return None;
        }
        unsafe { Some((*self.last_event).as_str()) }
    }

    pub fn drain_old_events(&mut self, keep: usize) {
        while self.buffer.len() > keep {
            self.buffer.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
