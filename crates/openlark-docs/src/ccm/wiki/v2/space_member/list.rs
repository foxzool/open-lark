/// 获取知识空间成员列表
///
/// 此接口用于获取知识空间成员列表，包含成员信息和权限角色。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取知识空间成员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceMembersRequest {
    /// 空间ID
    pub space_id: String,
    /// 页面大小，默认20
    pub page_size: Option<i32>,
    /// 页码，从1开始
    pub page_token: Option<String>,
}

/// 空间成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMemberItem {
    /// 成员ID
    pub member_id: String,
    /// 成员类型：user(用户)、group(群组)、department(部门)
    pub member_type: String,
    /// 成员名称
    pub name: Option<String>,
    /// 成员邮箱
    pub email: Option<String>,
    /// 权限角色：admin(管理员)、editor(编辑者)、viewer(查看者)
    pub perm: String,
    /// 加入时间
    pub join_time: Option<i64>,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageToken {
    /// 是否还有下一页
    pub has_more: Option<bool>,
    /// 页码
    pub page_token: Option<String>,
}

/// 获取知识空间成员列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceMembersResponse {
    /// 成员列表
    pub data: Option<ListSpaceMembersData>,
}

/// 成员列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceMembersData {
    /// 成员列表
    pub items: Option<Vec<SpaceMemberItem>>,
    /// 分页信息
    pub page_token: Option<PageToken>,
}

impl ApiResponseTrait for ListSpaceMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间成员列表
///
/// 此接口用于获取知识空间成员列表，包含成员信息和权限角色。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/list
pub async fn list_space_members(
    request: ListSpaceMembersRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ListSpaceMembersResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id
    });

    if let Some(page_size) = request.page_size {
        body["page_size"] = serde_json::json!(page_size);
    }
    if let Some(page_token) = request.page_token {
        body["page_token"] = serde_json::json!(page_token);
    }

    // 创建API请求
    let mut api_request: ApiRequest<ListSpaceMembersResponse> =
        ApiRequest::get("/open-apis/wiki/v2/spaces/members")
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
    fn test_list_space_members_request() {
        let request = ListSpaceMembersRequest {
            space_id: "space_123".to_string(),
            page_size: Some(20),
            page_token: Some("token_456".to_string()),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token_456"));
    }

    #[test]
    fn test_space_member_item() {
        let member = SpaceMemberItem {
            member_id: "user_123".to_string(),
            member_type: "user".to_string(),
            name: Some("张三".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            perm: "editor".to_string(),
            join_time: Some(1609459200),
        };

        assert_eq!(member.member_id, "user_123");
        assert_eq!(member.member_type, "user");
        assert_eq!(member.name, Some("张三"));
        assert_eq!(member.perm, "editor");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListSpaceMembersResponse::data_format(), ResponseFormat::Data);
    }
}
