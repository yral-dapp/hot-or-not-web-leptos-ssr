use std::time::Duration;

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

    pub fn get() -> Self {
        let Self {
            muted,
            show_mute_icon,
        } = expect_context();

        if show_mute_icon.get() {
            Self::display_flash_for_6_secs(show_mute_icon);
        }

        Self {
            muted,
            show_mute_icon,
        }
    }

    fn display_flash_for_6_secs(show_mute_icon: RwSignal<bool>) {
        show_mute_icon.set(true);
        set_timeout(
            move || {
                show_mute_icon.set(false);
            },
            Duration::from_secs(6),
        );
    }

    pub fn toggle_mute() {
        let Self {
            muted,
            show_mute_icon,
        } = expect_context();
        println!("Toggle Mute");
        if !muted.get() {
            Self::display_flash_for_6_secs(show_mute_icon);
        } else {
            show_mute_icon.set(false);
        }
        muted.update(|m| *m = !*m);
    }
}
