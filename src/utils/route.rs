use std::error::Error;

use leptos_router::use_navigate;

#[macro_export]
macro_rules! try_or_redirect {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                use $crate::utils::route::failure_redirect;
                failure_redirect(e);
                return;
            }
        }
    };
}

pub fn failure_redirect<E: Error>(err: E) {
    let nav = use_navigate();
    nav(&format!("/error?err={err}"), Default::default());
}

pub fn go_to_root() {
    let nav = use_navigate();
    nav("/", Default::default());
}
