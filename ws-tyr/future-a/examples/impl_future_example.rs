use futures::executor::block_on;
use std::future::Future;

#[tokio::main]
async fn main() {
    let name1 = "Miki".to_string();
    say_hello1(&name1).await;
    say_hello2(&name1).await;

    // Future 除了可以用 await 来执行外，还可以直接用 executor 执行
    block_on(say_hello1(&name1));
    block_on(say_hello2(&name1));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello {}", name);
    77
}

fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output=usize> + 'fut {
    async move {
        println!("Hello {}", name);
        77
    }
}