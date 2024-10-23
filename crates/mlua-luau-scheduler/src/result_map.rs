#![allow(clippy::inline_always)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    sync::Arc,
};

use event_listener::Event;

use crate::{thread_id::ThreadId, util::ThreadResult};

#[derive(Clone)]
pub(crate) struct ThreadResultMap {
    tracked: Arc<RefCell<HashSet<ThreadId>>>,
    results: Arc<RefCell<HashMap<ThreadId, ThreadResult>>>,
    events: Arc<RefCell<HashMap<ThreadId, Arc<Event>>>>,
}

unsafe impl Send for ThreadResultMap {}
unsafe impl Sync for ThreadResultMap {}

impl ThreadResultMap {
    pub fn new() -> Self {
        Self {
            tracked: Arc::new(RefCell::new(HashSet::default())),
            results: Arc::new(RefCell::new(HashMap::default())),
            events: Arc::new(RefCell::new(HashMap::default())),
        }
    }

    #[inline(always)]
    pub fn track(&self, id: ThreadId) {
        self.tracked.borrow_mut().insert(id);
    }

    #[inline(always)]
    pub fn is_tracked(&self, id: ThreadId) -> bool {
        self.tracked.borrow().contains(&id)
    }

    pub fn insert(&self, id: ThreadId, result: ThreadResult) {
        debug_assert!(self.is_tracked(id), "Thread must be tracked");
        self.results.borrow_mut().insert(id, result);
        if let Some(event) = self.events.borrow_mut().remove(&id) {
            event.notify(usize::MAX);
        }
    }

    pub async fn listen(&self, id: ThreadId) {
        debug_assert!(self.is_tracked(id), "Thread must be tracked");
        if !self.results.borrow().contains_key(&id) {
            let listener = {
                let mut events = self.events.borrow_mut();
                let event = events.entry(id).or_insert_with(|| Arc::new(Event::new()));
                event.listen()
            };
            listener.await;
        }
    }

    pub fn remove(&self, id: ThreadId) -> Option<ThreadResult> {
        let res = self.results.borrow_mut().remove(&id)?;
        self.tracked.borrow_mut().remove(&id);
        self.events.borrow_mut().remove(&id);
        Some(res)
    }
}
