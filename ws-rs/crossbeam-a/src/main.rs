use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel::{Receiver, Sender, unbounded};
use strum_macros::{Display, EnumString};

/// 如何在Rust中实现事件循环模式？
///
/// https://mp.weixin.qq.com/s/KH_OTqC_4Au8FcjhZ7Jl3g
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Display, EnumString)]
pub enum Event {
    Dummy,
    TestEvent,
}
pub type Payload = Vec<u8>;

pub trait Handler: Send + Sync {
    fn handle_event(&self, event: Event, payload: Payload);
}
#[derive(Clone)]
pub struct Listener {
    pub event: Event,
    pub handler: Arc<dyn Handler>,
}

pub struct Dispatcher {
    tx: Sender<(Event, Payload)>,
    rx: Receiver<(Event, Payload)>,
    // 事件注册表
    registry: Arc<Mutex<HashMap<Event, Vec<Arc<dyn Handler>>>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Dispatcher {
            tx,
            rx,
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 向注册表中注册事件及其对应的处理程序
    pub fn register_handler(&mut self, event: Event, handler: Arc<dyn Handler>) {
        let mut registry = self.registry.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }

    // 向通道发送事件
    pub fn trigger_event(&self, event: Event, payload: Payload) {
        self.tx.send((event, payload)).unwrap();
    }

    // 接收通道中的事件并进行处理
    pub fn start(&self) {
        let registry = Arc::clone(&self.registry);
        let rx = self.rx.clone();

        thread::spawn(move || loop {
            if let Ok((event, payload)) = rx.recv() {
                let registry = registry.lock().unwrap();
                if let Some(handlers) = registry.get(&event) {
                    for handler in handlers {
                        handler.handle_event(event.clone(), payload.clone());
                    }
                }
            }
        });
    }
}
pub struct TestEventHandler;

impl Handler for TestEventHandler {
    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);

        println!("TestEvent: {}", message);
    }
}

pub struct DBTestEventHandler;

impl Handler for DBTestEventHandler {
    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);

        // 将数据持久化到db中
        println!("Data: {} saved on DB!", message);
    }
}
fn main() {
    let mut event_loop = Dispatcher::new();

    event_loop.register_handler(Event::TestEvent, Arc::new(TestEventHandler));
    event_loop.register_handler(Event::TestEvent, Arc::new(DBTestEventHandler));

    // 启动事件循环
    event_loop.start();

    loop {
        println!("Give me some input, type 'exit' to quit");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Error during input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut split = input.split_whitespace();
        let name_data = (
            split.next().unwrap_or_default().to_string(),
            split.next().unwrap_or_default().to_string(),
        );
        println!("name_data: {:?}", name_data);

        let event = Event::from_str(&name_data.0).unwrap_or_else(|_| Event::Dummy);
        event_loop.trigger_event(event, name_data.1.as_bytes().to_vec());
    }
}