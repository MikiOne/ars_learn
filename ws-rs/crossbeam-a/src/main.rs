use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Display, EnumString)]
enum Event {
    Default,
    EventTest,
    DBEvent,
}

type Payload = Vec<u8>;

trait EventHandler: Send + Sync {
    fn handle(&self, event: &Event, payload: &Payload) {
        println!("Default event {} => {}", event, String::from_utf8(payload.clone()).unwrap());
    }
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
                    if let Some(handlers) = reg_map.get(&event) {
                        for handler in handlers {
                            handler.handle(&event, &payload);
                        }
                    }
                }
            }
        });
    }
}

struct TestEventHandler;
impl EventHandler for TestEventHandler {
    fn handle(&self, event: &Event, payload: &Payload) {
        println!("Test event: {:?}, payload: {:?}", event, payload);
    }
}

struct DBEventHandler;
impl EventHandler for DBEventHandler {
    fn handle(&self, event: &Event, payload: &Payload) {
        println!("DB event: {:?}, payload: {:?}", event, payload);
    }
}


#[tokio::main]
async fn main() {
    let dispatcher = Dispatcher::new();
    dispatcher.register_handler(Event::EventTest, Arc::new(TestEventHandler));
    dispatcher.register_handler(Event::DBEvent, Arc::new(DBEventHandler));
    dispatcher.start();

    dispatcher.trigger_event(Event::Default, vec![1, 2, 3]);
    dispatcher.trigger_event(Event::EventTest, vec![4, 5, 6]);
    dispatcher.trigger_event(Event::DBEvent, vec![7, 8, 9]);
    dispatcher.trigger_event(Event::EventTest, vec![10, 11, 12]);
    dispatcher.trigger_event(Event::DBEvent, vec![13, 14, 15]);
    dispatcher.trigger_event(Event::EventTest, vec![16, 17, 18]);

    tokio::time::sleep(Duration::from_millis(20)).await
}