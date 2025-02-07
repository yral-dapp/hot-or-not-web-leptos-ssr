#[derive(Clone, Debug, PartialEq)]
pub enum AppType {
    YRAL,
    HotOrNot,
    ICPump,
}

impl AppType {
    pub fn from_host(host: &str) -> Self {
        if host.contains("hotornot") {
            AppType::HotOrNot
        } else if host.contains("icpump") {
            AppType::ICPump
        } else {
            AppType::YRAL
        }
    }
}
