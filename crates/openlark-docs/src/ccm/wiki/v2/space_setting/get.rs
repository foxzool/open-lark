/// 获取知识空间设置
///
/// 此接口用于获取知识空间的详细设置信息。
/// 注意：实际上知识空间的基础设置信息通过 get space info 接口获取。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取知识空间设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceSettingRequest {
    /// 空间ID
    pub space_id: String,
}

/// 知识空间设置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceSettingData {
    /// 空间ID
    pub space_id: String,
    /// 是否允许评论
    pub comment_enabled: Option<bool>,
    /// 是否允许复制
    pub copy_enabled: Option<bool>,
    /// 更新时间
    pub updated_at: Option<String>,
}

/// 获取知识空间设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceSettingResponse {
    /// 设置信息
    pub data: Option<SpaceSettingData>,
}

impl ApiResponseTrait for GetSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间设置
///
/// 此接口用于获取知识空间的详细设置信息。
/// 注意：实际上知识空间的基础设置信息通过 get space info 接口获取。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get
pub async fn get_space_setting(
    request: GetSpaceSettingRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetSpaceSettingResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "space_id": request.space_id
    });

    // 创建API请求
    let mut api_request: ApiRequest<GetSpaceSettingResponse> =
        ApiRequest::get("/open-apis/wiki/v2/spaces/get_space_setting")
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
    fn test_get_space_setting_request() {
        let request = GetSpaceSettingRequest {
            space_id: "space_123".to_string(),
        };

        assert_eq!(request.space_id, "space_123");
    }

    #[test]
    fn test_space_setting_data() {
        let data = SpaceSettingData {
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
        assert_eq!(GetSpaceSettingResponse::data_format(), ResponseFormat::Data);
    }
}
