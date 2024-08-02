use std::fmt;

use crate::utils::posts::PostViewError;

// Define your error types
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    Post(PostViewError),
    Custom(String),
}

// Implement Display for AppError to provide user-friendly messages
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user_message = match self {
            AppError::NotFound => "The requested resource was not found.",
            AppError::Unauthorized => "You are not authorized to access this resource.",
            AppError::Post(post_view_error) => {
                match post_view_error {
                    _ => "Internal server error",
                    // PostViewError::Agent(agent) => "Internal server error",
                    // PostViewError::Canister(canister) => todo!(),
                    // PostViewError::HttpFetch(httpFetch) => todo!(),
                }
            }
            AppError::Custom(e) => e.as_str(),
        };
        write!(f, "{}", user_message)
    }
}
