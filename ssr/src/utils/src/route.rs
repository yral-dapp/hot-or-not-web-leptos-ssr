use std::fmt::Display;

#[macro_export]
macro_rules! try_or_redirect {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                use utils::route::failure_redirect;
                failure_redirect(e);
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! try_or_redirect_opt {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                use utils::route::failure_redirect;
                failure_redirect(e);
                return None;
            }
        }
    };
}

pub fn failure_redirect<E: Display>(err: E) {
    let nav = leptos_router::hooks::use_navigate();
    nav(&format!("/error?err={err}"), Default::default());
}

pub fn go_to_root() {
    let nav = leptos_router::hooks::use_navigate();
    nav("/", Default::default());
}
