use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

struct RateLimiter {
    // 存储每个窗口的开始时间和请求计数
    window_start_time: u64,
    request_count: u32,
    window_size: u64,
    // 窗口大小，以秒为单位
    max_requests: u32, // 每个窗口允许的最大请求
}

impl RateLimiter {
    fn new(window_size: u64, max_requests: u32) -> RateLimiter {
        RateLimiter { window_start_time: 0, request_count: 0, window_size, max_requests }
    }

    fn check(&mut self) -> bool {
        let now =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

        // 检查是否还在当前窗口内
        if now < self.window_start_time + self.window_size {
            // 如果在当前窗口内，则增加请求计数
            self.request_count += 1;

            // 检查请求计数是否超过限制
            if self.request_count > self.max_requests {
                // 超过限制，拒绝请求
                false
            } else {
                // 未超过限制，允许请求
                true
            }
        } else {
            // 不在当前窗口，重置窗口开始时间和请求计数
            self.window_start_time = now;
            self.request_count = 1;
            true
        }
    }
}

struct SafeRateLimiter {
    inner: Mutex<RateLimiter>,
}

impl SafeRateLimiter {
    fn new(window_size: u64, max_requests: u32) -> SafeRateLimiter {
        SafeRateLimiter { inner: Mutex::new(RateLimiter::new(window_size, max_requests)) }
    }

    fn check(&self) -> bool {
        let mut limiter = self.inner.lock().unwrap();
        limiter.check()
    }
}

fn main() {
    let limiter = SafeRateLimiter::new(60, 100); // 每60秒不超过100个请求

    // 模拟请求检查
    for _ in 0..105 {
        if limiter.check() {
            println!("Request allowed");
        } else {
            println!("Request denied");
        }
    }
}
