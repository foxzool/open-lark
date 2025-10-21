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

use super::batch_create::Permission;

/// 移除协作者权限请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct DeletePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 协作者ID类型
    #[serde(skip)]
    member_type: String,
    /// 协作者ID
    #[serde(skip)]
    member_id: String,
    /// 是否通知
    #[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>,
}

impl DeletePermissionMemberRequest {
    pub fn builder() -> DeletePermissionMemberRequestBuilder {
        DeletePermissionMemberRequestBuilder::default()
    }

    pub fn new(
        token: impl ToString,
        obj_type: impl ToString,
        member_type: impl ToString,
        member_id: impl ToString,
    ) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            member_type: member_type.to_string(),
            member_id: member_id.to_string(),
            ..Default::default()
        }
    }

    /// 移除用户权限
    pub fn for_user(token: impl ToString, obj_type: impl ToString, user_id: impl ToString) -> Self {
        Self::new(token, obj_type, "user", user_id)
    }

    /// 移除群组权限
    pub fn for_chat(token: impl ToString, obj_type: impl ToString, chat_id: impl ToString) -> Self {
        Self::new(token, obj_type, "chat", chat_id)
    }

    /// 移除部门权限
    pub fn for_department(
        token: impl ToString,
        obj_type: impl ToString,
        department_id: impl ToString,
    ) -> Self {
        Self::new(token, obj_type, "department", department_id)
    }
}

#[derive(Default)]
pub struct DeletePermissionMemberRequestBuilder {
    request: DeletePermissionMemberRequest,
}

impl DeletePermissionMemberRequestBuilder {
    /// 文档token
    pub fn token(mut self, token: impl ToString) -> Self {
        self.request.token = token.to_string();
        self
    }

    /// 文档类型
    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        self.request.obj_type = obj_type.to_string();
        self
    }

    /// 设置为文档类型
    pub fn as_doc(mut self) -> Self {
        self.request.obj_type = "doc".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn as_sheet(mut self) -> Self {
        self.request.obj_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn as_bitable(mut self) -> Self {
        self.request.obj_type = "bitable".to_string();
        self
    }

    /// 设置为知识库类型
    pub fn as_wiki(mut self) -> Self {
        self.request.obj_type = "wiki".to_string();
        self
    }

    /// 协作者类型和ID
    pub fn member(mut self, member_type: impl ToString, member_id: impl ToString) -> Self {
        self.request.member_type = member_type.to_string();
        self.request.member_id = member_id.to_string();
        self
    }

    /// 用户协作者
    pub fn user(mut self, user_id: impl ToString) -> Self {
        self.request.member_type = "user".to_string();
        self.request.member_id = user_id.to_string();
        self
    }

    /// 群组协作者
    pub fn chat(mut self, chat_id: impl ToString) -> Self {
        self.request.member_type = "chat".to_string();
        self.request.member_id = chat_id.to_string();
        self
    }

    /// 部门协作者
    pub fn department(mut self, department_id: impl ToString) -> Self {
        self.request.member_type = "department".to_string();
        self.request.member_id = department_id.to_string();
        self
    }

    /// 是否通知
    pub fn need_notification(mut self, need: bool) -> Self {
        self.request.need_notification = Some(need);
        self
    }

    /// 启用通知
    pub fn with_notification(mut self) -> Self {
        self.request.need_notification = Some(true);
        self
    }

    /// 禁用通知
    pub fn without_notification(mut self) -> Self {
        self.request.need_notification = Some(false);
        self
    }

    pub fn build(mut self) -> DeletePermissionMemberRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 协作者删除结果
#[derive(Debug, Deserialize)]
pub struct PermissionMemberDeleted {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 删除时间（毫秒时间戳）
    pub delete_time: Option<i64>,
    /// 删除前的权限（如果有）
    pub old_perm: Option<Permission>,
    /// 是否通知了用户
    pub notified: Option<bool>,
}

/// 移除协作者权限响应
#[derive(Debug, Deserialize)]
pub struct DeletePermissionMemberResponse {
    /// 协作者信息
    pub member: PermissionMemberDeleted,
}

impl ApiResponseTrait for DeletePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除协作者权限
pub async fn delete_permission_member(
    request: DeletePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeletePermissionMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = format!(
        "{}?type={}&member_type={}",
        EndpointBuilder::replace_params_from_array(
            DRIVE_V1_PERMISSIONS_MEMBER_GET,
            &[("token", &request.token), ("member_id", &request.member_id)]
        ),
        request.obj_type,
        request.member_type
    );

    // 添加通知参数
    if let Some(need_notification) = request.need_notification {
        api_req.api_path = format!(
            "{}&need_notification={}",
            api_req.api_path, need_notification
        );
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PermissionMemberDeleted {
    /// 获取成员ID
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// 获取成员类型
    pub fn member_type(&self) -> &str {
        &self.member_type
    }

    /// 获取删除前的权限
    pub fn old_permission(&self) -> Option<&Permission> {
        self.old_perm.as_ref()
    }

    /// 是否是用户
    pub fn is_user(&self) -> bool {
        self.member_type == "user"
    }

    /// 是否是群组
    pub fn is_chat(&self) -> bool {
        self.member_type == "chat"
    }

    /// 是否是部门
    pub fn is_department(&self) -> bool {
        self.member_type == "department"
    }

    /// 是否通知了用户
    pub fn was_notified(&self) -> bool {
        self.notified.unwrap_or(false)
    }

    /// 是否有删除时间
    pub fn has_delete_time(&self) -> bool {
        self.delete_time.is_some()
    }

    /// 是否有原权限信息
    pub fn has_old_permission(&self) -> bool {
        self.old_perm.is_some()
    }

    /// 原权限是否是所有者
    pub fn was_owner(&self) -> bool {
        if let Some(old_perm) = &self.old_perm {
            old_perm.is_owner()
        } else {
            false
        }
    }

    /// 原权限是否有编辑权限
    pub fn could_edit(&self) -> bool {
        if let Some(old_perm) = &self.old_perm {
            old_perm.can_edit()
        } else {
            false
        }
    }

    /// 获取删除时间的格式化字符串
    pub fn delete_time_formatted(&self) -> Option<String> {
        self.delete_time
            .map(|timestamp| format!("删除时间: {timestamp}"))
    }

    /// 获取成员类型描述
    pub fn member_type_description(&self) -> String {
        match self.member_type.as_str() {
            "user" => "用户".to_string(),
            "chat" => "群组".to_string(),
            "department" => "部门".to_string(),
            _ => "未知".to_string(),
        }
    }

    /// 获取原权限描述
    pub fn old_permission_description(&self) -> Option<String> {
        self.old_perm
            .as_ref()
            .map(|perm| perm.description().to_string())
    }

    /// 获取摘要信息
    pub fn summary(&self) -> String {
        let mut parts = vec![format!(
            "{} ({})",
            self.member_id,
            self.member_type_description()
        )];

        if let Some(old_perm_desc) = self.old_permission_description() {
            parts.push(format!("原权限: {old_perm_desc}"));
        }

        if let Some(time) = self.delete_time_formatted() {
            parts.push(time);
        }

        if self.was_notified() {
            parts.push("已通知".to_string());
        }

        parts.join(", ")
    }
}

impl DeletePermissionMemberResponse {
    /// 获取协作者ID
    pub fn member_id(&self) -> &str {
        self.member.member_id()
    }

    /// 获取协作者类型
    pub fn member_type(&self) -> &str {
        self.member.member_type()
    }

    /// 获取删除前的权限
    pub fn old_permission(&self) -> Option<&Permission> {
        self.member.old_permission()
    }

    /// 是否删除成功
    pub fn is_deleted(&self) -> bool {
        !self.member.member_id.is_empty()
    }

    /// 获取删除成功摘要
    pub fn success_summary(&self) -> String {
        format!("协作者权限移除成功: {}", self.member.summary())
    }

    /// 是否通知了用户
    pub fn was_notified(&self) -> bool {
        self.member.was_notified()
    }

    /// 删除的是否是所有者
    pub fn deleted_owner(&self) -> bool {
        self.member.was_owner()
    }

    /// 删除的是否有编辑权限
    pub fn deleted_editor(&self) -> bool {
        self.member.could_edit()
    }

    /// 获取删除时间
    pub fn delete_time(&self) -> Option<i64> {
        self.member.delete_time
    }

    /// 操作风险级别（基于删除的权限）
    pub fn risk_level(&self) -> &'static str {
        if self.deleted_owner() {
            "高风险" // 删除所有者
        } else if self.deleted_editor() {
            "中风险" // 删除编辑者
        } else {
            "低风险" // 删除查看者或评论者
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_permission_member_request_builder() {
        let request = DeletePermissionMemberRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .user("user123")
            .with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert_eq!(request.need_notification, Some(true));
    }

    #[test]
    fn test_delete_permission_member_convenience_methods() {
        let request = DeletePermissionMemberRequest::for_user("doccnxxxxxx", "doc", "user123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");

        let request = DeletePermissionMemberRequest::for_chat("doccnxxxxxx", "doc", "chat456");
        assert_eq!(request.member_type, "chat");
        assert_eq!(request.member_id, "chat456");

        let request =
            DeletePermissionMemberRequest::for_department("doccnxxxxxx", "doc", "dept789");
        assert_eq!(request.member_type, "department");
        assert_eq!(request.member_id, "dept789");
    }

    #[test]
    fn test_permission_member_deleted_methods() {
        let member = PermissionMemberDeleted {
            member_type: "user".to_string(),
            member_id: "user123".to_string(),
            delete_time: Some(1234567890),
            old_perm: Some(Permission::Edit),
            notified: Some(true),
        };

        assert!(member.is_user());
        assert!(!member.is_chat());
        assert!(!member.is_department());
        assert!(member.was_notified());
        assert!(member.has_delete_time());
        assert!(member.has_old_permission());
        assert!(!member.was_owner());
        assert!(member.could_edit());
        assert_eq!(member.member_type_description(), "用户");
        assert_eq!(
            member.old_permission_description(),
            Some("编辑者".to_string())
        );
    }

    #[test]
    fn test_delete_permission_member_response_risk_level() {
        let response_owner = DeletePermissionMemberResponse {
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::FullAccess),
                notified: Some(true),
            },
        };
        assert_eq!(response_owner.risk_level(), "高风险");

        let response_editor = DeletePermissionMemberResponse {
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::Edit),
                notified: Some(true),
            },
        };
        assert_eq!(response_editor.risk_level(), "中风险");

        let response_viewer = DeletePermissionMemberResponse {
            member: PermissionMemberDeleted {
                member_type: "user".to_string(),
                member_id: "user123".to_string(),
                delete_time: Some(1234567890),
                old_perm: Some(Permission::View),
                notified: Some(true),
            },
        };
        assert_eq!(response_viewer.risk_level(), "低风险");
    }
}
