use circular_buffer::CircularBuffer;
use leptos::prelude::*;

#[derive(Clone)]
pub struct HistoryCtx {
    pub history: RwSignal<CircularBuffer<3, String>>,
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

    pub fn is_empty(&self) -> bool {
        self.history.get_untracked().len() == 0
    }

    pub fn len(&self) -> usize {
        self.history.get_untracked().len()
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

    pub fn prev_url(&self) -> Option<String> {
        self.history.with(|h| h.back().cloned())
    }

    pub fn prev_url_untracked(&self) -> Option<String> {
        self.history.with_untracked(|h| h.back().cloned())
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
