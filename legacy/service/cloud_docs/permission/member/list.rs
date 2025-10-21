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

/// 获取协作者列表请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListPermissionMembersRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

impl ListPermissionMembersRequest {
    pub fn builder() -> ListPermissionMembersRequestBuilder {
        ListPermissionMembersRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListPermissionMembersRequestBuilder {
    request: ListPermissionMembersRequest,
}

impl ListPermissionMembersRequestBuilder {
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

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    pub fn build(mut self) -> ListPermissionMembersRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

crate::impl_executable_builder_owned!(
    ListPermissionMembersRequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    ListPermissionMembersRequest,
    BaseResponse<ListPermissionMembersResponse>,
    list_members
);

/// 协作者信息
#[derive(Debug, Deserialize)]
pub struct PermissionMember {
    /// 协作者ID类型
    pub member_type: String,
    /// 协作者ID
    pub member_id: String,
    /// 权限
    pub perm: Permission,
    /// 协作者名称（如果有）
    pub name: Option<String>,
    /// 协作者头像（如果有）
    pub avatar: Option<String>,
    /// 协作者类型描述（如果有）
    pub type_str: Option<String>,
    /// 是否继承权限
    pub is_inherited: Option<bool>,
    /// 继承来源（如果有）
    pub inherit_info: Option<String>,
}

/// 获取协作者列表响应
#[derive(Debug, Deserialize)]
pub struct ListPermissionMembersResponse {
    /// 协作者列表
    pub members: Vec<PermissionMember>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPermissionMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取协作者列表
pub async fn list_permission_members(
    request: ListPermissionMembersRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListPermissionMembersResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "{}?type={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS, "token", &request.token),
        request.obj_type
    );

    // 构建查询参数
    let mut query_params = Vec::new();
    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
    }
    if let Some(page_token) = request.page_token {
        query_params.push(format!("page_token={page_token}"));
    }

    if !query_params.is_empty() {
        api_req.api_path = format!("{}&{}", api_req.api_path, query_params.join("&"));
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PermissionMember {
    /// 获取显示名称
    pub fn display_name(&self) -> String {
        self.name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| self.member_id.clone())
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

    /// 是否继承权限
    pub fn has_inherited_permission(&self) -> bool {
        self.is_inherited.unwrap_or(false)
    }

    /// 获取权限描述
    pub fn permission_description(&self) -> String {
        let mut desc = self.perm.description().to_string();

        if self.has_inherited_permission() {
            desc.push_str(" (继承)");
            if let Some(inherit_info) = &self.inherit_info {
                desc.push_str(&format!(" 来源: {inherit_info}"));
            }
        }

        desc
    }

    /// 获取成员类型描述
    pub fn member_type_description(&self) -> String {
        self.type_str
            .as_ref()
            .cloned()
            .unwrap_or_else(|| match self.member_type.as_str() {
                "user" => "用户".to_string(),
                "chat" => "群组".to_string(),
                "department" => "部门".to_string(),
                _ => "未知".to_string(),
            })
    }

    /// 获取成员摘要信息
    pub fn summary(&self) -> String {
        format!(
            "{} ({}) - {} - {}",
            self.display_name(),
            self.member_id,
            self.member_type_description(),
            self.permission_description()
        )
    }
}

impl ListPermissionMembersResponse {
    /// 获取成员数量
    pub fn count(&self) -> usize {
        self.members.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// 获取所有者
    pub fn owners(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| member.is_owner())
            .collect()
    }

    /// 获取编辑者
    pub fn editors(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| matches!(member.perm, Permission::Edit))
            .collect()
    }

    /// 获取评论者
    pub fn commenters(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| matches!(member.perm, Permission::Comment))
            .collect()
    }

    /// 获取阅读者
    pub fn viewers(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| matches!(member.perm, Permission::View))
            .collect()
    }

    /// 按权限类型分组
    pub fn group_by_permission(&self) -> std::collections::HashMap<String, Vec<&PermissionMember>> {
        let mut groups = std::collections::HashMap::new();

        for member in &self.members {
            let perm_key = member.perm.description().to_string();
            groups.entry(perm_key).or_insert_with(Vec::new).push(member);
        }

        groups
    }

    /// 按成员类型分组
    pub fn group_by_member_type(
        &self,
    ) -> std::collections::HashMap<String, Vec<&PermissionMember>> {
        let mut groups = std::collections::HashMap::new();

        for member in &self.members {
            groups
                .entry(member.member_type.clone())
                .or_insert_with(Vec::new)
                .push(member);
        }

        groups
    }

    /// 获取继承权限的成员
    pub fn inherited_members(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| member.has_inherited_permission())
            .collect()
    }

    /// 获取直接权限的成员
    pub fn direct_members(&self) -> Vec<&PermissionMember> {
        self.members
            .iter()
            .filter(|member| !member.has_inherited_permission())
            .collect()
    }

    /// 权限统计摘要
    pub fn permission_summary(&self) -> String {
        let owners = self.owners().len();
        let editors = self.editors().len();
        let commenters = self.commenters().len();
        let viewers = self.viewers().len();

        format!(
            "协作者总数: {}, 所有者: {}, 编辑者: {}, 评论者: {}, 阅读者: {}",
            self.count(),
            owners,
            editors,
            commenters,
            viewers
        )
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_permission_members_request_builder() {
        let request = ListPermissionMembersRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .page_size(20)
            .page_token("token123")
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_permission_members_new() {
        let request = ListPermissionMembersRequest::new("doccnxxxxxx", "doc");
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
    }
}
