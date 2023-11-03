use tokio::task;
use tokio::time::{sleep, Duration};

async fn task1() -> Result<String, String> {
    sleep(Duration::from_secs(1)).await;
    Ok("Task 1 completed".to_string())
}

async fn task2() -> Result<String, String> {
    sleep(Duration::from_secs(2)).await;
    Err("Task 2 failed".to_string())
}

/// 使用tokio::try_join!(task1(), task2())
/// 和
/// tokio::try_join!(task::spawn(task1()), task::spawn(task2()))
/// 的区别
#[tokio::main]
async fn main() {
    match tokio::try_join!(task1(), task2()) {
        Ok((result1, result2)) => {
            println!("Result 1: {:?}", result1);
            println!("Result 2: {:?}", result2);
        }
        Err(err) => {
            eprintln!("error: {:?}", err)
        }
    }
    // 以上返回：error: "Task 2 failed"

    let (result1, result2) = tokio::try_join!(task::spawn(task1()), task::spawn(task2())).unwrap();
    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);
    // 以上返回：
    // Result 1: Ok("Task 1 completed")
    // Result 2: Err("Task 2 failed")
}
