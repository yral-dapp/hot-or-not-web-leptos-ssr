use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

struct DisplayMutedIconTimeout {
    // TODO: use TAIT once stable
    // instead of Dyn dispatch
    start: Box<dyn Fn(())>,
    _stop: Box<dyn Fn()>,
    _is_pending: Signal<bool>,
    show_mute_icon: RwSignal<bool>,
}

impl DisplayMutedIconTimeout {
    fn new(show_mute_icon: RwSignal<bool>) -> Self {
        let UseTimeoutFnReturn {
            start,
            stop,
            is_pending,
            ..
        } = use_timeout_fn(
            move |()| {
                show_mute_icon.set(false);
            },
            // 6 secs
            6000f64,
        );
        Self {
            start: Box::new(start),
            _stop: Box::new(stop),
            _is_pending: is_pending,
            show_mute_icon,
        }
    }

    fn flash(&self) {
        self.show_mute_icon.set(true);
        (self.start)(());
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AudioState {
    pub show_mute_icon: RwSignal<bool>,
    pub muted: RwSignal<bool>,
    display_flash: StoredValue<DisplayMutedIconTimeout>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioState {
    pub fn new() -> Self {
        let show_mute_icon = create_rw_signal(true);
        let display_flash = StoredValue::new(DisplayMutedIconTimeout::new(show_mute_icon));
        Self {
            show_mute_icon,
            muted: create_rw_signal(true),
            display_flash,
        }
    }

    pub fn get() -> Self {
        let this: Self = expect_context();

        if this.show_mute_icon.get_untracked() {
            this.display_flash.with_value(|d| d.flash());
        }

        this
    }

    pub fn toggle_mute() {
        let this: Self = expect_context();
        if !this.muted.get_untracked() {
            this.display_flash.with_value(|d| d.flash());
        } else {
            this.show_mute_icon.set(false);
        }
        this.muted.update(|m| *m = !*m);
    }
}
