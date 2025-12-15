/// 批量增加协作者权限
///
/// 批量为文件或文件夹添加协作者权限
/// docPath: https://open.feishu.cn/document/docs/permission/permission-member/batch_create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 批量增加协作者权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberRequest {
    /// 文件token
    pub token: String,
    /// 成员权限列表
    pub members: Vec<MemberPermission>,
    /// 是否发送通知
    pub notify: Option<bool>,
}

/// 成员权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermission {
    /// 成员信息
    pub member: MemberInfo,
    /// 权限类型
    pub r#type: String,
}

/// 成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户类型
    pub user_type: String,
}

impl BatchCreatePermissionMemberRequest {
    /// 创建批量增加协作者权限请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `members` - 成员权限列表
    pub fn new(
        token: impl Into<String>,
        members: Vec<MemberPermission>,
    ) -> Self {
        Self {
            token: token.into(),
            members,
            notify: None,
        }
    }

    /// 设置是否发送通知
    pub fn notify(mut self, notify: bool) -> Self {
        self.notify = Some(notify);
        self
    }
}

impl MemberPermission {
    /// 创建成员权限
    ///
    /// # 参数
    /// * `member` - 成员信息
    /// * `type` - 权限类型
    pub fn new(
        member: MemberInfo,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            member,
            r#type: r#type.into(),
        }
    }
}

impl MemberInfo {
    /// 创建成员信息
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `user_type` - 用户类型
    pub fn new(
        user_id: impl Into<String>,
        user_type: impl Into<String>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            user_type: user_type.into(),
        }
    }
}

/// 批量增加协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberResponse {
    /// 批量操作结果
    pub batch_result: BatchOperationResult,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failed_count: i32,
    /// 总数量
    pub total_count: i32,
    /// 结果列表
    pub results: Vec<BatchItemResult>,
}

/// 批量操作单项结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult {
    /// 是否成功
    pub success: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

impl ApiResponseTrait for BatchCreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量增加协作者权限
///
/// 批量为文件或文件夹添加协作者权限
/// docPath: https://open.feishu.cn/document/docs/permission/permission-member/batch_create
pub async fn batch_create_permission_member(
    request: BatchCreatePermissionMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<BatchCreatePermissionMemberResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "members": request.members
    });

    if let Some(notify) = request.notify {
        body["notify"] = serde_json::json!(notify);
    }

    // 创建API请求
    let mut api_request: ApiRequest<BatchCreatePermissionMemberResponse> =
        ApiRequest::post(&format!("/open-apis/drive/v1/permissions/{}/members/batch_create", request.token))
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
    fn test_batch_create_permission_member_request_builder() {
        let member = MemberInfo::new("user_123", "user");
        let permission = MemberPermission::new(member, "view");
        let request = BatchCreatePermissionMemberRequest::new("file_token", vec![permission])
            .notify(true);

        assert_eq!(request.token, "file_token");
        assert_eq!(request.members.len(), 1);
        assert_eq!(request.notify, Some(true));
    }

    #[test]
    fn test_member_permission_builder() {
        let member = MemberInfo::new("user_123", "user");
        let permission = MemberPermission::new(member, "edit");

        assert_eq!(permission.member.user_id, "user_123");
        assert_eq!(permission.r#type, "edit");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(BatchCreatePermissionMemberResponse::data_format(), ResponseFormat::Data);
    }
}