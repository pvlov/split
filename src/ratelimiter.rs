use chrono::{DateTime, Duration};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

#[allow(dead_code)]
struct RateLimiter {
    max_requests: usize,
    window: Duration,
    clients: Arc<RwLock<HashMap<String, VecDeque<DateTime<chrono_tz::Tz>>>>>,
}

#[allow(dead_code)]
impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        RateLimiter {
            max_requests,
            window,
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
