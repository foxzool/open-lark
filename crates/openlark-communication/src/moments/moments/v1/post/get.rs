//! 查询帖子信息
//!
//! docPath: https://open.feishu.cn/document/moments-v1/post/get

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 查询帖子信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostResponse {
    pub post: Post,
}

impl ApiResponseTrait for GetPostResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 帖子信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询帖子信息请求
pub struct GetPostRequest {
    config: Config,
    post_id: String,
    user_id_type: Option<String>,
}

impl GetPostRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            post_id: String::new(),
            user_id_type: None,
        }
    }

    /// 帖子 ID（路径参数）
    pub fn post_id(mut self, post_id: impl Into<String>) -> Self {
        self.post_id = post_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/moments-v1/post/get
    pub async fn execute(self) -> SDKResult<GetPostResponse> {
        validate_required!(self.post_id, "post_id 不能为空");

        // url: GET:/open-apis/moments/v1/posts/:post_id
        let mut req: ApiRequest<GetPostResponse> =
            ApiRequest::get(format!("/open-apis/moments/v1/posts/{}", self.post_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", &user_id_type);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询帖子信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let config = Config::default();
        let req = GetPostRequest::new(config)
            .post_id("post_id")
            .user_id_type("open_id");
        let _ = req;
    }
}
