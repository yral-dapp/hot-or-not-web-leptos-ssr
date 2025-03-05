use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use candid::Principal;
use leptos::*;
use once_cell::sync::Lazy;
use yral_canisters_common::Canisters;
use yral_pump_n_dump_common::GameDirection;

use crate::state::canisters::authenticated_canisters;
use crate::utils::event_streaming::events::TokenPumpedDumped;

struct ActionBuffer {
    token_name: String,
    pump_count: usize,
    dump_count: usize,
    last_flush: Instant,
}

// Global buffer storage
static ACTION_BUFFERS: Lazy<Mutex<HashMap<Principal, ActionBuffer>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// Buffer flush interval (60 seconds)
const FLUSH_INTERVAL: Duration = Duration::from_secs(60);

pub fn record_action(token_name: String, token_root: Principal, direction: GameDirection) {
    let mut buffers = ACTION_BUFFERS.lock().unwrap();

    // Get or create buffer for this token
    let buffer = buffers.entry(token_root).or_insert_with(|| ActionBuffer {
        token_name: token_name.clone(),
        pump_count: 0,
        dump_count: 0,
        last_flush: Instant::now(),
    });

    // Update counts
    match direction {
        GameDirection::Pump => buffer.pump_count += 1,
        GameDirection::Dump => buffer.dump_count += 1,
    }

    // Check if it's time to flush
    if buffer.last_flush.elapsed() >= FLUSH_INTERVAL {
        flush_buffer(token_root);
    }
}

pub fn flush_buffer(token_root: Principal) {
    let mut buffers = ACTION_BUFFERS.lock().unwrap();

    if let Some(buffer) = buffers.get_mut(&token_root) {
        // Only send events if there are actions to report
        if buffer.pump_count > 0 || buffer.dump_count > 0 {
            // Send pump events if any
            if buffer.pump_count > 0 {
                spawn_local(send_buffered_event(
                    buffer.token_name.clone(),
                    token_root,
                    "pump".to_string(),
                    buffer.pump_count,
                ));
                buffer.pump_count = 0;
            }

            // Send dump events if any
            if buffer.dump_count > 0 {
                spawn_local(send_buffered_event(
                    buffer.token_name.clone(),
                    token_root,
                    "dump".to_string(),
                    buffer.dump_count,
                ));
                buffer.dump_count = 0;
            }
        }

        // Reset timer
        buffer.last_flush = Instant::now();
    }
}

async fn send_buffered_event(
    token_name: String,
    token_root: Principal,
    direction: String,
    count: usize,
) {
    #[cfg(feature = "hydrate")]
    {
        let cans_res = authenticated_canisters();
        if let Ok(cans_wire) = cans_res.wait_untracked().await {
            if let Ok(cans) = Canisters::from_wire(cans_wire, expect_context()) {
                // Add count information to the token name for analytics
                let token_name_with_count = format!("{} ({})", token_name, count);
                TokenPumpedDumped.send_event(cans, token_name_with_count, token_root, direction);
            }
        }
    }
}

pub fn setup_flush_timer() {
    #[cfg(feature = "hydrate")]
    {
        use gloo::timers::callback::Interval;

        let _interval = Interval::new(FLUSH_INTERVAL.as_millis() as u32, move || {
            let buffers = ACTION_BUFFERS.lock().unwrap();
            for token_root in buffers.keys().copied().collect::<Vec<_>>() {
                flush_buffer(token_root);
            }
        });
    }
}
