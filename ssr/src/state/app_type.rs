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
        } else if host.contains("pumpdump") {
            AppType::Pumpdump
        } else {
            AppType::YRAL
        }
    }
}
