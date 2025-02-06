use super::app_type::AppType;

#[derive(Clone)]
pub struct AppState {
    pub app_type: AppType,
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub theme_color: &'static str,
    pub manifest_path: &'static str,
}

impl AppState {
    pub fn from_type(app_type: &AppType) -> Self {
        match app_type {
            AppType::HotOrNot => Self {
                app_type: AppType::HotOrNot,
                id: "hotornot",
                name: "Hot Or Not",
                description: "Vote on the hottest content and earn rewards",
                theme_color: "#FF4500",
                manifest_path: "hotornot.json",
            },
            AppType::ICPump => Self {
                app_type: AppType::ICPump,
                id: "icpump",
                name: "ICPump Token Platform",
                description: "Create and trade tokens on the Internet Computer",
                theme_color: "#4CAF50",
                manifest_path: "icpump.json",
            },
            AppType::YRAL => Self {
                app_type: AppType::YRAL,
                id: "yral",
                name: "YRAL Platform",
                description: "The First App to Host Creative Short Video Challenges",
                theme_color: "#E20479",
                manifest_path: "yral.json",
            },
        }
    }

    pub fn manifest_config(&self) -> &'static str {
        self.manifest_path
    }
}
