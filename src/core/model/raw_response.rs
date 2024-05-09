use std::collections::HashMap;

use bytes::Bytes;
use serde::Deserialize;

use crate::core::constants::CONTENT_TYPE;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RawResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    #[serde(skip)]
    pub content: Option<Bytes>,
}

impl RawResponse {
    pub fn set_content_type(&mut self, content_type: String) {
        self.headers
            .insert(CONTENT_TYPE.to_string(), content_type.into());
    }
}
