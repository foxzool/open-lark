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

/// 协作者权限
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum Permission {
    /// 所有者
    FullAccess,
    /// 编辑者
    Edit,
    /// 阅读者
    #[default]
    View,
    /// 评论者
    Comment,
}

/// 协作者信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collaborator {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
}

/// 批量增加协作者权限请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct BatchCreatePermissionMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 协作者列表
    members: Vec<Collaborator>,
    /// 是否通知
    #[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>,
}

impl BatchCreatePermissionMemberRequest {
    pub fn builder() -> BatchCreatePermissionMemberRequestBuilder {
        BatchCreatePermissionMemberRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString, members: Vec<Collaborator>) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            members,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BatchCreatePermissionMemberRequestBuilder {
    request: BatchCreatePermissionMemberRequest,
}

impl BatchCreatePermissionMemberRequestBuilder {
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

    /// 协作者列表
    pub fn members(mut self, members: Vec<Collaborator>) -> Self {
        self.request.members = members;
        self
    }

    /// 添加协作者
    pub fn add_member(mut self, member: Collaborator) -> Self {
        self.request.members.push(member);
        self
    }

    /// 添加用户协作者
    pub fn add_user(mut self, user_id: impl ToString, permission: Permission) -> Self {
        self.request.members.push(Collaborator {
            member_type: "user".to_string(),
            member_id: user_id.to_string(),
            perm: permission,
        });
        self
    }

    /// 添加群组协作者
    pub fn add_chat(mut self, chat_id: impl ToString, permission: Permission) -> Self {
        self.request.members.push(Collaborator {
            member_type: "chat".to_string(),
            member_id: chat_id.to_string(),
            perm: permission,
        });
        self
    }

    /// 添加部门协作者
    pub fn add_department(mut self, department_id: impl ToString, permission: Permission) -> Self {
        self.request.members.push(Collaborator {
            member_type: "department".to_string(),
            member_id: department_id.to_string(),
            perm: permission,
        });
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

    pub fn build(mut self) -> BatchCreatePermissionMemberRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

crate::impl_executable_builder_owned!(
    BatchCreatePermissionMemberRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    BatchCreatePermissionMemberRequest,
    BaseResponse<BatchCreatePermissionMemberResponse>,
    batch_create_member
);

/// 成员操作结果
#[derive(Debug, Deserialize)]
pub struct MemberResult {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 操作结果
    pub result: String,
    /// 错误码（如果有）
    pub code: Option<i32>,
    /// 错误信息（如果有）
    pub msg: Option<String>,
}

/// 批量增加协作者权限响应
#[derive(Debug, Deserialize)]
pub struct BatchCreatePermissionMemberResponse {
    /// 操作结果列表
    pub members: Vec<MemberResult>,
}

impl ApiResponseTrait for BatchCreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量增加协作者权限
pub async fn batch_create_permission_member(
    request: BatchCreatePermissionMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchCreatePermissionMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!(
        "{}?type={}",
        EndpointBuilder::replace_param(
            DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE,
            "token",
            &request.token
        ),
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

impl Permission {
    /// 权限级别（数值越大权限越高）
    pub fn level(&self) -> u8 {
        match self {
            Permission::View => 1,
            Permission::Comment => 2,
            Permission::Edit => 3,
            Permission::FullAccess => 4,
        }
    }

    /// 是否有编辑权限
    pub fn can_edit(&self) -> bool {
        matches!(self, Permission::Edit | Permission::FullAccess)
    }

    /// 是否有评论权限
    pub fn can_comment(&self) -> bool {
        !matches!(self, Permission::View)
    }

    /// 是否是所有者
    pub fn is_owner(&self) -> bool {
        matches!(self, Permission::FullAccess)
    }

    /// 权限描述
    pub fn description(&self) -> &'static str {
        match self {
            Permission::FullAccess => "所有者",
            Permission::Edit => "编辑者",
            Permission::Comment => "评论者",
            Permission::View => "阅读者",
        }
    }
}

impl MemberResult {
    /// 操作是否成功
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }

    /// 是否有错误
    pub fn has_error(&self) -> bool {
        self.code.is_some() || self.msg.is_some()
    }

    /// 获取错误信息
    pub fn error_message(&self) -> Option<String> {
        if let (Some(code), Some(msg)) = (self.code, &self.msg) {
            Some(format!("错误码: {code}, 错误信息: {msg}"))
        } else if let Some(msg) = &self.msg {
            Some(msg.clone())
        } else {
            self.code.map(|code| format!("错误码: {code}"))
        }
    }
}

impl BatchCreatePermissionMemberResponse {
    /// 获取成功的操作数量
    pub fn success_count(&self) -> usize {
        self.members
            .iter()
            .filter(|member| member.is_success())
            .count()
    }

    /// 获取失败的操作数量
    pub fn failed_count(&self) -> usize {
        self.members
            .iter()
            .filter(|member| !member.is_success())
            .count()
    }

    /// 获取成功的操作
    pub fn successful_members(&self) -> Vec<&MemberResult> {
        self.members
            .iter()
            .filter(|member| member.is_success())
            .collect()
    }

    /// 获取失败的操作
    pub fn failed_members(&self) -> Vec<&MemberResult> {
        self.members
            .iter()
            .filter(|member| !member.is_success())
            .collect()
    }

    /// 操作摘要
    pub fn summary(&self) -> String {
        format!(
            "总计: {}, 成功: {}, 失败: {}",
            self.members.len(),
            self.success_count(),
            self.failed_count()
        )
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_permission_member_request_builder() {
        let request = BatchCreatePermissionMemberRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .add_user("user123", Permission::Edit)
            .add_chat("chat456", Permission::View)
            .with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.need_notification, Some(true));
    }

    #[test]
    fn test_permission_methods() {
        assert!(Permission::Edit.can_edit());
        assert!(Permission::FullAccess.can_edit());
        assert!(!Permission::View.can_edit());

        assert!(Permission::Edit.can_comment());
        assert!(!Permission::View.can_comment());

        assert!(Permission::FullAccess.is_owner());
        assert!(!Permission::Edit.is_owner());

        assert_eq!(Permission::FullAccess.level(), 4);
        assert_eq!(Permission::View.level(), 1);
    }
}
