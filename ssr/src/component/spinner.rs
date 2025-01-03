use leptos::*;
use leptos_icons::Icon;

use crate::utils::icon::icon_gen;

/// Spinner with constant size
#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <div class="animate-spin border-solid rounded-full border-t-transparent border-white border-8 h-32 w-32"></div>
    }
}

/// Spinner that fits container
#[component]
pub fn SpinnerFit() -> impl IntoView {
    view! {
        <div class="animate-spin border-solid rounded-full border-t-transparent border-primary-600 border-8 w-full h-full"></div>
    }
}

/// Spinner that takes up the whole screen with black background
#[component]
pub fn FullScreenSpinner() -> impl IntoView {
    view! {
        <div class="h-screen w-screen grid grid-cols-1 bg-black justify-items-center place-content-center">
            <Spinner />
        </div>
    }
}

icon_gen!(
    SpinnerCircleIcon,
    view_box = "0 0 21 21",
    r###"
<path opacity="0.38" d="M20.5 10.5C20.5 16.0228 16.0228 20.5 10.5 20.5C4.97715 20.5 0.5 16.0228 0.5 10.5C0.5 4.97715 4.97715 0.5 10.5 0.5C16.0228 0.5 20.5 4.97715 20.5 10.5ZM3.61817 10.5C3.61817 14.3007 6.69927 17.3818 10.5 17.3818C14.3007 17.3818 17.3818 14.3007 17.3818 10.5C17.3818 6.69927 14.3007 3.61817 10.5 3.61817C6.69927 3.61817 3.61817 6.69927 3.61817 10.5Z" fill="#FCE6F2" style="fill:#FCE6F2;fill:color(display-p3 0.9882 0.9020 0.9490);fill-opacity:1;"/>
<path d="M18.9409 10.5C19.802 10.5 20.5124 9.79783 20.3787 8.94721C20.2563 8.16837 20.0419 7.40492 19.7388 6.67317C19.2362 5.45991 18.4997 4.35752 17.5711 3.42893C16.6425 2.50035 15.5401 1.76375 14.3268 1.2612C13.5951 0.9581 12.8316 0.743716 12.0528 0.621293C11.2022 0.487588 10.5 1.19803 10.5 2.05908C10.5 2.92014 11.2067 3.60061 12.0458 3.79402C12.4165 3.87948 12.7804 3.99574 13.1336 4.14202C13.9685 4.48786 14.7272 4.99477 15.3662 5.63381C16.0052 6.27285 16.5121 7.0315 16.858 7.86644C17.0043 8.21959 17.1205 8.58347 17.206 8.95421C17.3994 9.79326 18.0799 10.5 18.9409 10.5Z" fill="#FAFAFA" style="fill:#FAFAFA;fill:color(display-p3 0.9804 0.9804 0.9804);fill-opacity:1;"/>
"###
);
#[component]
pub fn SpinnerCircle() -> impl IntoView {
    view! {
        <Icon icon=SpinnerCircleIcon class="animate-spin w-full h-full"/>
    }
}
