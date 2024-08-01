use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
// use std::mpsc::{channel, Sender, Receiver};

type ActorMessage = String;

struct Actor<T> {
    state: Arc<Mutex<T>>,
    cv: Arc<Condvar>,
    inbox: Receiver<ActorMessage>,
    outbox: Sender<ActorMessage>, // 可选，如果需要与其他 Actor 通信
}

impl<T> Actor<T>
where
    T: Default,
{
    fn new(state: T, inbox: Receiver<ActorMessage>, outbox: Sender<ActorMessage>) -> Self {
        let state = Arc::new(Mutex::new(state));
        let cv = Arc::new(Condvar::new());
        Actor {
            state,
            cv,
            inbox,
            outbox,
        }
    }

    fn run(&self) {
        loop {
            let msg = self.inbox.recv().unwrap();
            // 处理消息...
            // 可能需要修改 state 并使用 cv 进行同步
            // ...

            // 发送响应（如果需要）
            // self.outbox.send("response").unwrap();
        }
    }
}

fn main() {
    // let (tx_a, rx_a) = channel();
    // let (tx_b, rx_b) = channel();
    //
    // let actor_a = Actor::new(rx_a, tx_b);
    // let actor_b = Actor::new(rx_b, tx_a); // 假设 actor_b 响应 actor_a 的消息
    //
    // // 在单独的线程中运行 Actor
    // thread::spawn(move || actor_a.run());
    // thread::spawn(move || actor_b.run());
    //
    // // 发送消息到 Actor A
    // tx_a.send("Hello, Actor A!".to_owned()).unwrap();
}

// 注意：在实际应用中，你需要一种方式来优雅地关闭 Actor 线程，
// 比如使用另一个 Channel 发送一个特殊的关闭消息，并在 Actor 内部处理这个消息来退出循环。

// 由于上面的代码示例是简化的，并没有包含完整的错误处理和优雅的关闭逻辑，
// 所以在实际应用中需要进一步完善。

// 另外，如果你想要实现更复杂的 Actor 模型，比如支持 Actor 的父子关系、
// 生命周期管理、错误处理、重试机制等，你可能需要查看一些现成的 Rust Actor 框架，
// 如 Actix、Bastion 等，这些框架提供了更丰富的功能和更好的抽象。

// 总结一下，虽然 Rust 标准库中没有直接提供 Actor 模型的实现，
// 但你完全可以使用 Mutex、RwLock、Condvar 和 Channel 等并发原语来自己实现一个简单的 Actor 系统。
// 然而，对于复杂的并发系统，使用现成的 Actor 框架通常是更好的选择，因为它们提供了更高级别的抽象和更多的功能。

// 思考题答案（针对你提出的思考题）：
//
// 1. 使用 `std::sync::mpsc::channel` 在两个线程中来回发送消息：
//
// ```rust
// use std::sync::mpsc::{channel, Sender, Receiver};
// use std::thread;
// use std::time::Duration;

// let (tx_to_b, rx_from_b) = channel();
// let (tx_to_a, rx_from_a) = channel();

// let handle_a = thread::spawn(move || {
//     rx_from_b.recv().unwrap(); // 等待从 B 接收消息
//     println("Received from B: Hello, world!");
//     tx_to_b.send("Goodbye!").unwrap(); // 发送消息给 B
// });

// let handle_b = thread::spawn(move || {
//     std::thread::sleep(Duration::from_millis(100)); // 假设 A 需要一点时间来启动
//     tx_to_a.send("Hello, world!").unwrap(); // 发送消息给 A
//     let response = rx_from_a.recv().unwrap(); // 等待 A 的响应
//     println("Received from A: {}", response);
// });

// handle_a.join().unwrap();
// handle_b.join().unwrap();
// ```

// 2. 如果要实现 Actor 模型，可以利用 `Mutex`、`Condvar` 和 `Channel` 来实现 Actor 的状态保护、消息队列和线程间通信。
//   但更简单的做法是使用现成的 Actor 框架，如 Actix，它提供了更高级的抽象和更多的功能。