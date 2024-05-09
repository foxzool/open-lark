use serde::Deserialize;

use crate::core::model::RawResponse;

#[derive(Default, Debug, Deserialize)]
pub struct BaseResponse {
    pub(crate) raw: Option<RawResponse>,
    pub(crate) code: Option<i32>,
    msg: Option<String>,
}

pub trait BaseResponseTrait {
    fn success(&self) -> bool;
    // fn get_log_id(&self) -> Option<String>;
}
