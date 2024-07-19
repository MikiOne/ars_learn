#[cfg(test)]
mod tests {
    use crate::mpsc;

    /// channel可以发送接收数据
    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = mpsc::unbounded();
        s.send("hello world".to_string()).unwrap();

        let result = r.recv().unwrap();
        assert_eq!(result, "hello world");
    }

    /// 多个sender可以发送数据
    #[test]
    fn multiple_senders_should_work() {
        let (mut s1, mut r) = mpsc::unbounded();
        let mut s2 = s1.clone();
        let mut s3 = s1.clone();

        let t1 = std::thread::spawn(move || {
            s1.send(1).unwrap();
        });
        let t2 = std::thread::spawn(move || {
            s2.send(2).unwrap();
        });
        let t3 = std::thread::spawn(move || {
            s3.send(3).unwrap();
        });
        for handler in [t1, t2, t3] {
            handler.join().unwrap();
        }

        let r1 = r.recv().unwrap();
        let r2 = r.recv().unwrap();
        let r3 = r.recv().unwrap();

        let mut res = [r1, r2, r3];
        res.sort();

        assert_eq!(res, [0, 1, 2, 3]);
    }

    /// 当队列为空的时候，receiver所在的线程会被阻塞
    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (s, mut r) = mpsc::unbounded();
        let mut s1 = s.clone();

        let t1 = std::thread::spawn(move || {
            for (idx, val) in r.into_iter().enumerate() {
                assert_eq!(idx, val);
            }
        });

        let t2 = std::thread::spawn(move || {
            for i in 0..100 {
                s.send(i).unwrap();
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1));
        // 线程t2处理完成
        assert!(t2.is_finished());

        for i in 100..200 {
            s1.send(i).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(1));

        // 线程t1处理完成
        assert!(!t1.is_finished());

        assert_eq!(s1.total_queued_items(), 0);
    }

    /// 如果现在所有 Sender 都退出作用域，Receiver 继续接收，直到没有数据可读了，则接收报错
    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (mut s, mut r) = mpsc::unbounded();
        let s1 = s.clone();

        let senders = [s, s1];
        let total = senders.len();

        // sender 即用即抛
        for sender in senders {
            std::thread::spawn(move || {
                sender.send().unwrap();
            }).join().unwrap();
        }

        // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而，读取更多数据时会出错
        assert!(r.recv().is_err());
    }

    /// 如果没有 Receiver了，Sender发送时应该错误返回
    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s, mut s1) = {
            let (mut s, _) = mpsc::unbounded();
            let s1 = s.clone();
            (s, s1)
        };

        assert!(s.send("hello").is_err());
        assert!(s1.send("hello").is_err());
    }
}