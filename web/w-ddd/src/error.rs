use std::{error::Error, fmt::Display, sync::LazyLock};

use serde::{Deserialize, Serialize};

pub static INTERNAL_SERVER_ERROR: LazyLock<BizError> =
    LazyLock::new(|| BizError::new("internal_server_error", "Internal Server Error"));

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BizError {
    code: String,
    message: String,
}

unsafe impl Sync for BizError {}
unsafe impl Send for BizError {}

impl BizError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

impl Display for BizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ code: \"{}\", message: \"{}\" }}",
            self.code, self.message
        )
    }
}

impl Error for BizError {}
