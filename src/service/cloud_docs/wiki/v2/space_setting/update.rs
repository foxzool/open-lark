use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 更新知识空间设置请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateSpaceSettingRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 是否开启评论：true(开启)、false(关闭)
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_enabled: Option<bool>,
    /// 是否开启复制：true(开启)、false(关闭)
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_enabled: Option<bool>,
}

impl UpdateSpaceSettingRequest {
    pub fn builder() -> UpdateSpaceSettingRequestBuilder {
        UpdateSpaceSettingRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateSpaceSettingRequestBuilder {
    request: UpdateSpaceSettingRequest,
}

impl UpdateSpaceSettingRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 是否开启评论
    pub fn comment_enabled(mut self, enabled: bool) -> Self {
        self.request.comment_enabled = Some(enabled);
        self
    }

    /// 开启评论
    pub fn enable_comment(mut self) -> Self {
        self.request.comment_enabled = Some(true);
        self
    }

    /// 关闭评论
    pub fn disable_comment(mut self) -> Self {
        self.request.comment_enabled = Some(false);
        self
    }

    /// 是否开启复制
    pub fn copy_enabled(mut self, enabled: bool) -> Self {
        self.request.copy_enabled = Some(enabled);
        self
    }

    /// 开启复制
    pub fn enable_copy(mut self) -> Self {
        self.request.copy_enabled = Some(true);
        self
    }

    /// 关闭复制
    pub fn disable_copy(mut self) -> Self {
        self.request.copy_enabled = Some(false);
        self
    }

    pub fn build(mut self) -> UpdateSpaceSettingRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 知识空间设置
#[derive(Debug, Deserialize)]
pub struct SpaceSetting {
    /// 知识空间id
    pub space_id: String,
    /// 是否开启评论
    pub comment_enabled: Option<bool>,
    /// 是否开启复制
    pub copy_enabled: Option<bool>,
}

/// 更新知识空间设置响应
#[derive(Debug, Deserialize)]
pub struct UpdateSpaceSettingResponse {
    /// 更新后的空间设置
    pub setting: SpaceSetting,
}

impl ApiResponseTrait for UpdateSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间设置
pub async fn update_space_setting(
    request: UpdateSpaceSettingRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateSpaceSettingResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path =
        EndpointBuilder::replace_param(WIKI_V2_SPACE_SETTING_UPDATE, "space_id", &request.space_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_update_space_setting_request_builder() {
        let request = UpdateSpaceSettingRequest::builder()
            .space_id("spcxxxxxx")
            .enable_comment()
            .disable_copy()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.comment_enabled, Some(true));
        assert_eq!(request.copy_enabled, Some(false));
    }
}
