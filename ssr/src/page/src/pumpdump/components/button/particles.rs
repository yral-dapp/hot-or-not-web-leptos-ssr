use std::ops::Range;

use leptos::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use yral_canisters_common::utils::time::current_epoch;

#[component]
fn BubblingParticles(
    rng: StoredValue<SmallRng>,
    #[prop(into)] img_url: String,
    on_anim_end: impl Fn() + 'static,
) -> impl IntoView {
    let rand_range = move |r: Range<f64>| {
        let mut res = 0.0;
        rng.update_value(|rng| {
            res = rng.random_range(r);
        });
        res
    };
    let styles: Vec<_> = (0..3)
        .map(|_| {
            let r_x1 = rand_range(-4f64..4.0);
            let r_x2 = rand_range(-5f64..5.0);
            let r_x3 = rand_range(-3f64..3.0);
            let r_x4 = rand_range(-6f64..6.0);
            let r_x5 = rand_range(-4f64..4.0);
            let r_x6 = rand_range(-5f64..5.0);
            let r_x7 = rand_range(-3f64..3.0);
            let r_x8 = rand_range(-2f64..2.0);

            format!(
                "--random-x1: {r_x1}px;
        --random-x2: {r_x2}px;
        --random-x3: {r_x3}px;
        --random-x4: {r_x4}px;
        --random-x5: {r_x5}px;
        --random-x6: {r_x6}px;
        --random-x7: {r_x7}px;
        --random-x8: {r_x8}px;
        background-image: url('{img_url}')
        "
            )
        })
        .collect();

    view! {
        <div style=styles[0].clone() class="opacity-0 w-3.5 h-3.5 bg-contain bg-no-repeat absolute left-1/2 animate-bubble-1"></div>
        <div style=styles[1].clone() class="opacity-0 w-3.5 h-3.5 bg-contain bg-no-repeat absolute left-1/2 animate-bubble-2"></div>
        <div style=styles[2].clone() on:animationend=move |_| on_anim_end() class="opacity-0 w-3.5 h-3.5 bg-contain bg-no-repeat absolute left-1/2 animate-bubble-3"></div>
    }
}

#[component]
fn BubblingParticlesQueue(
    #[prop(into)] img_url: String,
    spawn_bubbles: RwSignal<u32>,
) -> impl IntoView {
    let offset = StoredValue::new(0u32);
    let on_anim_end = move || {
        offset.update_value(|o| *o += 1);
        spawn_bubbles.update(|v| *v -= 1);
    };
    let rng = StoredValue::new(SmallRng::seed_from_u64(current_epoch().as_nanos() as u64));

    view! {
        <For each=move || (0..spawn_bubbles.get()) key=move |i| offset.get_value() + i let:_d>
            <BubblingParticles rng img_url=img_url.clone() on_anim_end />
        </For>
    }
}

#[component]
pub fn FireBubbles(spawn_bubbles: RwSignal<u32>) -> impl IntoView {
    view! {
        <BubblingParticlesQueue img_url="/img/pumpdump/fire.webp" spawn_bubbles />
    }
}

#[component]
pub fn SkullBubbles(spawn_bubbles: RwSignal<u32>) -> impl IntoView {
    view! {
        <BubblingParticlesQueue img_url="/img/pumpdump/skull.webp" spawn_bubbles/>
    }
}
