use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Display, EnumString)]
enum Event {
    Default,
    EventTest,
}

type Payload = Vec<u8>;

trait EventHandler: Send + Sync {
    fn handle(&self, event: &Event, payload: &Payload);
}

struct Dispatcher {
    tx: Sender<(Event, Payload)>,
    rx: Receiver<(Event, Payload)>,
    registry: Arc<Mutex<HashMap<Event, Vec<Arc<dyn EventHandler>>>>>,
}

impl Dispatcher {
    fn new() -> Self {
        let (tx, rx) = channel::unbounded();
        Dispatcher {
            tx,
            rx,
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn register_handler(&self, event: Event, handler: Arc<dyn EventHandler>) {
        let mut registry = self.registry.lock().expect("注册表还未初始化");
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }

    fn trigger_event(&self, event: Event, payload: Payload) {
        self.tx.send((event, payload)).unwrap()
    }

    fn start(&self) {
        let registry = self.registry.clone();
        let rx = self.rx.clone();

        tokio::spawn(async move {
            loop {
                if let Ok((event, payload)) = rx.recv() {
                    let reg_map = registry.lock().unwrap();
                    let handlers = reg_map.get(&event).unwrap();
                    for handler in handlers {
                        handler.handle(&event, &payload);
                    }
                }
            }
        });
    }
}


#[tokio::main]
async fn main() {}