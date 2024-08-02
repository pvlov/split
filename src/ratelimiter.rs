use std::time::Duration;

#[allow(dead_code)]
struct RateLimiter {
    max_requests: usize,
    window: Duration,
}

#[allow(dead_code)]
impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        RateLimiter { max_requests, window }
    }
}
