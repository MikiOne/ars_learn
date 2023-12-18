use std::time::Instant;

use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn sum(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

/// 参考：https://mp.weixin.qq.com/s/rKURimh_Tp-C2krvbLdhZw
fn main() {
    let start = Instant::now();
    let mut data = Vec::new();
    for _i in 1..100000000 {
        data.push(1);
    }
    let res = sum_of_squares(&data);
    // let res = sum(&data);
    println!("the res is {},cost is {:?}", res, start.elapsed());
}