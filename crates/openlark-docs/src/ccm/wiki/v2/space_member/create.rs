/// 创建知识空间成员
///
/// 此接口用于向知识空间添加成员，支持用户类型成员和不同权限角色。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建知识空间成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceMemberRequest {
    /// 空间ID
    pub space_id: String,
    /// 成员类型：user(用户)、group(群组)、department(部门)
    pub member_type: String,
    /// 成员ID
    pub member_id: String,
    /// 权限角色：admin(管理员)、editor(编辑者)、viewer(查看者)
    pub perm: String,
}

/// 创建知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceMemberResponse {
    /// 创建的成员信息
    pub data: Option<SpaceMemberInfo>,
}

/// 空间成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMemberInfo {
    /// 成员ID
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 权限角色
    pub perm: String,
}

impl ApiResponseTrait for CreateSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间成员
///
/// 此接口用于向知识空间添加成员，支持用户类型成员和不同权限角色。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/create
pub async fn create_space_member(
    request: CreateSpaceMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateSpaceMemberResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "space_id": request.space_id,
        "member_type": request.member_type,
        "member_id": request.member_id,
        "perm": request.perm
    });

    // 创建API请求
    let mut api_request: ApiRequest<CreateSpaceMemberResponse> =
        ApiRequest::post("/open-apis/wiki/v2/spaces/members")
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
    fn test_create_space_member_request() {
        let request = CreateSpaceMemberRequest {
            space_id: "space_123".to_string(),
            member_type: "user".to_string(),
            member_id: "user_456".to_string(),
            perm: "editor".to_string(),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user_456");
        assert_eq!(request.perm, "editor");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateSpaceMemberResponse::data_format(), ResponseFormat::Data);
    }
}
