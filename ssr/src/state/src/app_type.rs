use leptos::prelude::window;

use utils::host::show_pnd_condition;

#[derive(Clone, Debug, PartialEq)]
pub enum AppType {
    YRAL,
    HotOrNot,
    ICPump,
    Pumpdump,
}

impl AppType {
    pub fn from_host(host: &str) -> Self {
        if host.contains("hotornot") {
            AppType::HotOrNot
        } else if host.contains("icpump") {
            AppType::ICPump
        } else if show_pnd_condition(host) {
            AppType::Pumpdump
        } else {
            AppType::YRAL
        }
    }

    pub fn select() -> Self {
        #[cfg(feature = "hydrate")]
        {
            let hostname = window().location().hostname().unwrap_or_default();
            AppType::from_host(&hostname)
        }

        #[cfg(not(feature = "hydrate"))]
        {
            use utils::host::get_host;
            let host = get_host();
            AppType::from_host(&host)
        }
    }
}
