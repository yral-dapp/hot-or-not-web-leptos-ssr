use circular_buffer::CircularBuffer;
use leptos::{RwSignal, SignalGet, SignalUpdate, SignalWith};

#[derive(Clone)]
pub struct HistoryCtx {
    history: RwSignal<CircularBuffer<3, String>>,
}

impl Default for HistoryCtx {
    fn default() -> Self {
        Self {
            history: RwSignal::new(CircularBuffer::<3, String>::new()),
        }
    }
}

impl HistoryCtx {
    pub fn new() -> Self {
        Self {
            history: RwSignal::new(CircularBuffer::<3, String>::new()),
        }
    }

    pub fn push(&self, url: &str) {
        self.history.update(move |h| h.push_back(url.to_string()));
    }

    pub fn back(&self, fallback: &str) -> String {
        self.history.update(move |h| {
            h.pop_back();
        });

        let url = self.history.with(|h| h.back().cloned());
        if let Some(url) = url {
            self.history.update(move |h| {
                h.pop_back();
            });
            url
        } else {
            fallback.to_string()
        }
    }

    pub fn log_history(&self) -> String {
        let history = self.history.get();
        let history_str = history
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" -> ");
        history_str
    }
}
