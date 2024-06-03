pub enum ReportOption {
    Nudity,
    Violence,
    Offensive,
    Spam,
    Other,
}

impl ReportOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReportOption::Nudity => "Nudity/Porn",
            ReportOption::Violence => "Violence/Gore",
            ReportOption::Offensive => "Offensive",
            ReportOption::Spam => "Spam/Ad",
            ReportOption::Other => "Others",
        }
    }
}
