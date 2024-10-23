use std::{cell::Cell, sync::Arc};

use event_listener::Event;

#[derive(Debug, Clone)]
pub(crate) struct Exit {
    code: Arc<Cell<Option<u8>>>,
    event: Arc<Event>,
}

unsafe impl Send for Exit {}
unsafe impl Sync for Exit {}

impl Exit {
    pub fn new() -> Self {
        Self {
            code: Arc::new(Cell::new(None)),
            event: Arc::new(Event::new()),
        }
    }

    pub fn set(&self, code: u8) {
        self.code.set(Some(code));
        self.event.notify(usize::MAX);
    }

    pub fn get(&self) -> Option<u8> {
        self.code.get()
    }

    pub async fn listen(&self) {
        self.event.listen().await;
    }
}
