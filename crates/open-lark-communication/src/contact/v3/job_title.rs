use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 职务服务
pub struct JobTitleService {
    config: Config,
}
impl JobTitleService {
    pub fn new(config: Config) -> Self {
    Self { config }
    }
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/get
