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

/// 增加协作者权限请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreatePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 协作者ID类型
    member_type: String,
    /// 协作者ID
    member_id: String,
    /// 权限
    perm: Permission,
    /// 是否通知
    #[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>,
}

impl CreatePermissionMemberRequest {
    pub fn builder() -> CreatePermissionMemberRequestBuilder {
        CreatePermissionMemberRequestBuilder::default()
    }

    pub fn new(
        token: impl ToString,
        obj_type: impl ToString,
        member_type: impl ToString,
        member_id: impl ToString,
        permission: Permission,
    ) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            member_type: member_type.to_string(),
            member_id: member_id.to_string(),
            perm: permission,
            ..Default::default()
        }
    }

    /// 为用户添加权限
    pub fn for_user(
        token: impl ToString,
        obj_type: impl ToString,
        user_id: impl ToString,
        permission: Permission,
    ) -> Self {
        Self::new(token, obj_type, "user", user_id, permission)
    }

    /// 为群组添加权限
    pub fn for_chat(
        token: impl ToString,
        obj_type: impl ToString,
        chat_id: impl ToString,
        permission: Permission,
    ) -> Self {
        Self::new(token, obj_type, "chat", chat_id, permission)
    }

    /// 为部门添加权限
    pub fn for_department(
        token: impl ToString,
        obj_type: impl ToString,
        department_id: impl ToString,
        permission: Permission,
    ) -> Self {
        Self::new(token, obj_type, "department", department_id, permission)
    }
}

#[derive(Default)]
pub struct CreatePermissionMemberRequestBuilder {
    request: CreatePermissionMemberRequest,
}

impl CreatePermissionMemberRequestBuilder {
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

    /// 权限
    pub fn permission(mut self, permission: Permission) -> Self {
        self.request.perm = permission;
        self
    }

    /// 设置为所有者权限
    pub fn as_owner(mut self) -> Self {
        self.request.perm = Permission::FullAccess;
        self
    }

    /// 设置为编辑权限
    pub fn as_editor(mut self) -> Self {
        self.request.perm = Permission::Edit;
        self
    }

    /// 设置为评论权限
    pub fn as_commenter(mut self) -> Self {
        self.request.perm = Permission::Comment;
        self
    }

    /// 设置为查看权限
    pub fn as_viewer(mut self) -> Self {
        self.request.perm = Permission::View;
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

    pub fn build(mut self) -> CreatePermissionMemberRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

crate::impl_executable_builder_owned!(
    CreatePermissionMemberRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    CreatePermissionMemberRequest,
    BaseResponse<CreatePermissionMemberResponse>,
    create_member
);

/// 协作者创建结果
#[derive(Debug, Deserialize)]
pub struct PermissionMemberCreated {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 创建时间（毫秒时间戳）
    pub create_time: Option<i64>,
    /// 是否通知了用户
    pub notified: Option<bool>,
}

/// 增加协作者权限响应
#[derive(Debug, Deserialize)]
pub struct CreatePermissionMemberResponse {
    /// 协作者信息
    pub member: PermissionMemberCreated,
}

impl ApiResponseTrait for CreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加协作者权限
pub async fn create_permission_member(
    request: CreatePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreatePermissionMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!(
        "{}?type={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS, "token", &request.token),
        request.obj_type
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

impl PermissionMemberCreated {
    /// 获取成员ID
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// 获取成员类型
    pub fn member_type(&self) -> &str {
        &self.member_type
    }

    /// 获取权限
    pub fn permission(&self) -> &Permission {
        &self.perm
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

    /// 是否有编辑权限
    pub fn can_edit(&self) -> bool {
        self.perm.can_edit()
    }

    /// 是否是所有者
    pub fn is_owner(&self) -> bool {
        self.perm.is_owner()
    }

    /// 是否通知了用户
    pub fn was_notified(&self) -> bool {
        self.notified.unwrap_or(false)
    }

    /// 是否有创建时间
    pub fn has_create_time(&self) -> bool {
        self.create_time.is_some()
    }

    /// 获取创建时间的格式化字符串
    pub fn create_time_formatted(&self) -> Option<String> {
        self.create_time
            .map(|timestamp| format!("创建时间: {timestamp}"))
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

    /// 获取权限描述
    pub fn permission_description(&self) -> String {
        self.perm.description().to_string()
    }

    /// 获取摘要信息
    pub fn summary(&self) -> String {
        let mut parts = vec![
            format!("{} ({})", self.member_id, self.member_type_description()),
            format!("权限: {}", self.permission_description()),
        ];

        if let Some(time) = self.create_time_formatted() {
            parts.push(time);
        }

        if self.was_notified() {
            parts.push("已通知".to_string());
        }

        parts.join(", ")
    }
}

impl CreatePermissionMemberResponse {
    /// 获取协作者ID
    pub fn member_id(&self) -> &str {
        self.member.member_id()
    }

    /// 获取协作者类型
    pub fn member_type(&self) -> &str {
        self.member.member_type()
    }

    /// 获取权限
    pub fn permission(&self) -> &Permission {
        self.member.permission()
    }

    /// 是否创建成功
    pub fn is_created(&self) -> bool {
        !self.member.member_id.is_empty()
    }

    /// 获取创建成功摘要
    pub fn success_summary(&self) -> String {
        format!("协作者添加成功: {}", self.member.summary())
    }

    /// 是否通知了用户
    pub fn was_notified(&self) -> bool {
        self.member.was_notified()
    }

    /// 权限级别
    pub fn permission_level(&self) -> u8 {
        self.member.perm.level()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_permission_member_request_builder() {
        let request = CreatePermissionMemberRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .user("user123")
            .as_editor()
            .with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert!(matches!(request.perm, Permission::Edit));
        assert_eq!(request.need_notification, Some(true));
    }

    #[test]
    fn test_create_permission_member_convenience_methods() {
        let request = CreatePermissionMemberRequest::for_user(
            "doccnxxxxxx",
            "doc",
            "user123",
            Permission::Edit,
        );
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");

        let request = CreatePermissionMemberRequest::for_chat(
            "doccnxxxxxx",
            "doc",
            "chat456",
            Permission::View,
        );
        assert_eq!(request.member_type, "chat");
        assert_eq!(request.member_id, "chat456");

        let request = CreatePermissionMemberRequest::for_department(
            "doccnxxxxxx",
            "doc",
            "dept789",
            Permission::Comment,
        );
        assert_eq!(request.member_type, "department");
        assert_eq!(request.member_id, "dept789");
    }

    #[test]
    fn test_permission_member_created_methods() {
        let member = PermissionMemberCreated {
            member_type: "user".to_string(),
            member_id: "user123".to_string(),
            perm: Permission::Edit,
            create_time: Some(1234567890),
            notified: Some(true),
        };

        assert!(member.is_user());
        assert!(!member.is_chat());
        assert!(!member.is_department());
        assert!(member.can_edit());
        assert!(!member.is_owner());
        assert!(member.was_notified());
        assert!(member.has_create_time());
        assert_eq!(member.member_type_description(), "用户");
        assert_eq!(member.permission_description(), "编辑者");
    }
}
