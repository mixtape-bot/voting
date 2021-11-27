use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub(crate) message: String,
    pub(crate) success: bool
}

impl MessageResponse {
    pub(crate) fn new(message: String, success: bool) -> Self {
        Self { message, success }
    }
}
