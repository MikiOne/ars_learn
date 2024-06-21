pub fn comsume_iterator<F, Iter, T>(mut f: F)
    where
        F: FnMut(i32) -> Iter, // F 是一个闭包，接受 i32，返回 Iter 类型
        Iter: Iterator<Item=T>, // Iter 是一个 Iterator，Item 是 T 类型
        T: std::fmt::Debug, // T 实现了 Debug trait
{
    // 根据 F 的类型，f(10) 返回 iterator，所以可以用 for 循环
    for item in f(10) {
        println!("{:?}", item); // item 实现了 Debug trait，所以可以用 {:?} 打印
    }
}

#[cfg(test)]
mod tests {
    use futures::{stream, StreamExt};
    use super::*;

    #[test]
    fn test_consume_iterator() {
        // 不会 panic 或者出错
        comsume_iterator(|i| (0..i).into_iter())
    }

    /// ```
    /// fn for_each_concurrent<Fut, F>(
    ///     self,
    ///     limit: impl Into<Option<usize>>,
    ///     f: F,
    /// ) -> ForEachConcurrent<Self, Fut, F>
    /// where
    ///     F: FnMut(Self::Item) -> Fut,
    ///     Fut: Future<Output = ()>,
    ///     Self: Sized,
    /// {
    /// ```
    #[tokio::test]
    async fn test_stream_ext_for_each_concurrent() {
        let stream = stream::iter(vec![1, 2, 3, 4, 5]);
        let fut = stream.for_each_concurrent(4, |num| async move {
            println!("{:?}", num);
        });

        fut.await;
    }
}

fn main() {}