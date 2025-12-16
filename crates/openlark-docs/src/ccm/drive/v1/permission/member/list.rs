use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取云文档协作者
///
/// 获取文件或文件夹的协作者列表
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/list
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取协作者列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionMembersRequest {
    /// 文件token
    pub token: String,
    /// 页码，从0开始
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 成员类型过滤
    pub r#type: Option<String>,
}

impl ListPermissionMembersRequest {
    /// 创建获取协作者列表请求
    ///
    /// # 参数
    /// * `token` - 文件token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            page_size: None,
            page_token: None,
            r#type: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置成员类型过滤
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
}

/// 协作者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMember {
    /// 成员ID
    pub member_id: String,
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 权限类型
    pub r#type: String,
    /// 创建时间
    pub create_time: i64,
}

/// 获取协作者列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionMembersResponse {
    /// 协作者列表
    pub data: Option<PermissionMembersData>,
}

/// 权限成员数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMembersData {
    /// 协作者列表
    pub items: Option<Vec<PermissionMember>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListPermissionMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档协作者
///
/// 获取文件或文件夹的协作者列表
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/list
pub async fn list_permission_members(
    request: ListPermissionMembersRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListPermissionMembersResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::ListPermissionMembers(request.token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<ListPermissionMembersResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }
    if let Some(r#type) = &request.r#type {
        api_request = api_request.query("type", r#type);
    }

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
    fn test_list_permission_members_request_builder() {
        let request = ListPermissionMembersRequest::new("file_token");

        assert_eq!(request.token, "file_token");
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_permission_members_request_with_params() {
        let request = ListPermissionMembersRequest::new("file_token")
            .page_size(20)
            .page_token("next_page_token")
            .r#type("admin");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.page_size.unwrap(), 20);
        assert_eq!(request.page_token.unwrap(), "next_page_token");
        assert_eq!(request.r#type.unwrap(), "admin");
    }

    #[test]
    fn test_permission_member_structure() {
        let member = PermissionMember {
            member_id: "member_id".to_string(),
            user_id: "user_id".to_string(),
            name: Some("用户名".to_string()),
            email: Some("user@example.com".to_string()),
            r#type: "admin".to_string(),
            create_time: 1640995200,
        };

        assert_eq!(member.member_id, "member_id");
        assert_eq!(member.user_id, "user_id");
        assert_eq!(member.name.unwrap(), "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListPermissionMembersResponse::data_format(),
            ResponseFormat::Data
        );
    }
}