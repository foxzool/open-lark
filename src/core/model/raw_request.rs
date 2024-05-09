use std::collections::HashMap;

pub struct RawRequest {
    pub uri: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}
