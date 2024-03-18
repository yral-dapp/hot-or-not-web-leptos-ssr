use circular_buffer::CircularBuffer;
use leptos::{RwSignal, SignalGet, SignalUpdate};

#[derive(Clone)]
pub struct HistoryCtx {
    history: RwSignal<CircularBuffer<3, String>>,
    fallback: String,
}

impl Default for HistoryCtx {
    fn default() -> Self {
        Self {
            history: RwSignal::new(CircularBuffer::<3, String>::new()),
            fallback: "/".to_string(),
        }
    }
}

impl HistoryCtx {
    pub fn new(fallback: String) -> Self {
        Self {
            history: RwSignal::new(CircularBuffer::<3, String>::new()),
            fallback,
        }
    }

    pub fn push(&self, url: &str) {
        self.history.update(move |h| h.push_back(url.to_string()));
    }

    pub fn back(&self) -> Option<String> {
        self.history.update(move |h| {
            h.pop_back();
        });

        let history = self.history.get();
        let url = history.back();
        if url.is_none() {
            Some(self.fallback.clone())
        } else {
            self.history.update(move |h| {
                h.pop_back();
            });
            url.cloned()
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
