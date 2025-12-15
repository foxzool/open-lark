/// 更新知识空间设置
///
/// 此接口用于更新知识空间的设置，如评论权限、复制权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_setting/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新知识空间设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceSettingRequest {
    /// 空间ID
    pub space_id: String,
    /// 是否允许评论
    pub comment_enabled: Option<bool>,
    /// 是否允许复制
    pub copy_enabled: Option<bool>,
}

/// 更新的知识空间设置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedSpaceSetting {
    /// 空间ID
    pub space_id: String,
    /// 是否允许评论
    pub comment_enabled: Option<bool>,
    /// 是否允许复制
    pub copy_enabled: Option<bool>,
    /// 更新时间
    pub updated_at: Option<String>,
}

/// 更新知识空间设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceSettingResponse {
    /// 更新后的设置信息
    pub data: Option<UpdatedSpaceSetting>,
}

impl ApiResponseTrait for UpdateSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间设置
///
/// 此接口用于更新知识空间的设置，如评论权限、复制权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_setting/update
pub async fn update_space_setting(
    request: UpdateSpaceSettingRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateSpaceSettingResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id
    });

    if let Some(comment_enabled) = request.comment_enabled {
        body["comment_enabled"] = serde_json::json!(comment_enabled);
    }
    if let Some(copy_enabled) = request.copy_enabled {
        body["copy_enabled"] = serde_json::json!(copy_enabled);
    }

    // 创建API请求
    let mut api_request: ApiRequest<UpdateSpaceSettingResponse> =
        ApiRequest::patch("/open-apis/wiki/v2/spaces/space_setting")
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_space_setting_request() {
        let request = UpdateSpaceSettingRequest {
            space_id: "space_123".to_string(),
            comment_enabled: Some(true),
            copy_enabled: Some(false),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.comment_enabled, Some(true));
        assert_eq!(request.copy_enabled, Some(false));
    }

    #[test]
    fn test_updated_space_setting() {
        let data = UpdatedSpaceSetting {
            space_id: "space_123".to_string(),
            comment_enabled: Some(true),
            copy_enabled: Some(false),
            updated_at: Some("2021-01-01T00:00:00Z".to_string()),
        };

        assert_eq!(data.space_id, "space_123");
        assert_eq!(data.comment_enabled, Some(true));
        assert_eq!(data.copy_enabled, Some(false));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateSpaceSettingResponse::data_format(), ResponseFormat::Data);
    }
}
