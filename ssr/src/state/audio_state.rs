use leptos::*;

#[derive(Clone, Copy, Debug)]
pub struct AudioState {
    pub show_mute_icon: RwSignal<bool>,
    pub muted: RwSignal<bool>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            show_mute_icon: create_rw_signal(true),
            muted: create_rw_signal(true),
        }
    }
}
