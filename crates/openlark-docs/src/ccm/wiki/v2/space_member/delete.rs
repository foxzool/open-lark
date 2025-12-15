/// 删除知识空间成员
///
/// 此接口用于从知识空间中删除成员。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除知识空间成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpaceMemberRequest {
    /// 空间ID
    pub space_id: String,
    /// 成员类型：user(用户)、group(群组)、department(部门)
    pub member_type: String,
    /// 成员ID
    pub member_id: String,
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpaceMemberResponse {
    /// 操作结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除知识空间成员
///
/// 此接口用于从知识空间中删除成员。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/delete
pub async fn delete_space_member(
    request: DeleteSpaceMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteSpaceMemberResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "space_id": request.space_id,
        "member_type": request.member_type,
        "member_id": request.member_id
    });

    // 创建API请求
    let mut api_request: ApiRequest<DeleteSpaceMemberResponse> =
        ApiRequest::delete("/open-apis/wiki/v2/spaces/members")
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
    fn test_delete_space_member_request() {
        let request = DeleteSpaceMemberRequest {
            space_id: "space_123".to_string(),
            member_type: "user".to_string(),
            member_id: "user_456".to_string(),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user_456");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteSpaceMemberResponse::data_format(), ResponseFormat::Data);
    }
}
