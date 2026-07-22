use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type ObserverCallback = Box<dyn Fn() + 'static>;

#[derive(Clone, Default)]
pub struct EntryObserverBus {
    subscribers: Rc<RefCell<HashMap<String, Vec<ObserverCallback>>>>,
}

impl EntryObserverBus {
    pub fn global() -> Self {
        thread_local! {
            static INSTANCE: EntryObserverBus = EntryObserverBus::default();
        }
        INSTANCE.with(|b| b.clone())
    }

    pub fn subscribe<F>(&self, entry_id: &str, callback: F)
    where
        F: Fn() + 'static,
    {
        self.subscribers
            .borrow_mut()
            .entry(entry_id.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn notify(&self, entry_id: &str) {
        if let Some(callbacks) = self.subscribers.borrow().get(entry_id) {
            for cb in callbacks {
                cb();
            }
        }
    }
}
