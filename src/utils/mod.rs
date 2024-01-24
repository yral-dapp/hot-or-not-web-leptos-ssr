use web_time::{Duration, SystemTime};

pub mod route;

pub fn current_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}
