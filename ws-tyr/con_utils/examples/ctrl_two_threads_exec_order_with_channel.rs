use std::sync::mpsc;
use std::thread;

/// 用一个 channel 来控制两个线程的执行顺序
/// Use a channel to control the order in which two threads are executed
fn main() {
    let (tx, rx) = mpsc::channel();

    // 克隆发送端以便在两个线程中使用
    let tx1 = tx.clone();
    let tx2 = tx;

    // 线程1
    let handle1 = thread::spawn(move || {
        println!("线程1开始执行");
        // 发送信号表示线程1已完成
        tx1.send(()).unwrap();
        println!("线程1结束执行");
    });

    // 线程2
    let handle2 = thread::spawn(move || {
        // 等待线程1的信号
        let res = rx.recv().unwrap();
        println!("recv: {:?}", res);

        println!("线程2开始执行");
        // 发送信号表示线程2已完成
        tx2.send(()).unwrap();
        println!("线程2结束执行");
    });

    // 等待两个线程完成
    handle1.join().unwrap();
    // handle2.join().unwrap();

    println!("所有线程执行完毕");
}