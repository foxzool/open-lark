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

/// 转移所有者请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct TransferOwnerRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 新所有者ID类型
    member_type: String,
    /// 新所有者ID
    member_id: String,
    /// 是否移除当前所有者的权限
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_old_owner: Option<bool>,
    /// 是否通知
    #[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>,
}

impl TransferOwnerRequest {
    pub fn builder() -> TransferOwnerRequestBuilder {
        TransferOwnerRequestBuilder::default()
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

    /// 转移给用户
    pub fn to_user(token: impl ToString, obj_type: impl ToString, user_id: impl ToString) -> Self {
        Self::new(token, obj_type, "user", user_id)
    }
}

#[derive(Default)]
pub struct TransferOwnerRequestBuilder {
    request: TransferOwnerRequest,
}

impl TransferOwnerRequestBuilder {
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

    /// 新所有者类型和ID
    pub fn new_owner(mut self, member_type: impl ToString, member_id: impl ToString) -> Self {
        self.request.member_type = member_type.to_string();
        self.request.member_id = member_id.to_string();
        self
    }

    /// 转移给用户
    pub fn to_user(mut self, user_id: impl ToString) -> Self {
        self.request.member_type = "user".to_string();
        self.request.member_id = user_id.to_string();
        self
    }

    /// 转移给群组
    pub fn to_chat(mut self, chat_id: impl ToString) -> Self {
        self.request.member_type = "chat".to_string();
        self.request.member_id = chat_id.to_string();
        self
    }

    /// 转移给部门
    pub fn to_department(mut self, department_id: impl ToString) -> Self {
        self.request.member_type = "department".to_string();
        self.request.member_id = department_id.to_string();
        self
    }

    /// 是否移除当前所有者的权限
    pub fn remove_old_owner(mut self, remove: bool) -> Self {
        self.request.remove_old_owner = Some(remove);
        self
    }

    /// 移除当前所有者权限
    pub fn remove_current_owner(mut self) -> Self {
        self.request.remove_old_owner = Some(true);
        self
    }

    /// 保留当前所有者权限
    pub fn keep_current_owner(mut self) -> Self {
        self.request.remove_old_owner = Some(false);
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

    pub fn build(mut self) -> TransferOwnerRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 转移结果
#[derive(Debug, Deserialize)]
pub struct TransferResult {
    /// 新所有者ID类型
    pub member_type: String,
    /// 新所有者ID
    pub member_id: String,
    /// 转移时间（毫秒时间戳）
    pub transfer_time: Option<i64>,
    /// 原所有者ID类型
    pub old_owner_type: Option<String>,
    /// 原所有者ID
    pub old_owner_id: Option<String>,
}

/// 转移所有者响应
#[derive(Debug, Deserialize)]
pub struct TransferOwnerResponse {
    /// 转移结果
    pub member: TransferResult,
}

impl ApiResponseTrait for TransferOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移所有者
pub async fn transfer_owner(
    request: TransferOwnerRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<TransferOwnerResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!(
        "{}?type={}",
        EndpointBuilder::replace_param(
            DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER,
            "token",
            &request.token
        ),
        request.obj_type
    );

    // 添加其他查询参数
    let mut query_params = Vec::new();
    if let Some(remove_old_owner) = request.remove_old_owner {
        query_params.push(format!("remove_old_owner={remove_old_owner}"));
    }
    if let Some(need_notification) = request.need_notification {
        query_params.push(format!("need_notification={need_notification}"));
    }

    if !query_params.is_empty() {
        api_req.api_path = format!("{}&{}", api_req.api_path, query_params.join("&"));
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl TransferResult {
    /// 是否有转移时间
    pub fn has_transfer_time(&self) -> bool {
        self.transfer_time.is_some()
    }

    /// 是否有原所有者信息
    pub fn has_old_owner_info(&self) -> bool {
        self.old_owner_type.is_some() && self.old_owner_id.is_some()
    }

    /// 获取转移时间的格式化字符串
    pub fn transfer_time_formatted(&self) -> Option<String> {
        self.transfer_time
            .map(|timestamp| format!("转移时间: {timestamp}"))
    }

    /// 获取新所有者信息
    pub fn new_owner_info(&self) -> String {
        format!("新所有者: {} ({})", self.member_id, self.member_type)
    }

    /// 获取原所有者信息
    pub fn old_owner_info(&self) -> Option<String> {
        if let (Some(old_type), Some(old_id)) = (&self.old_owner_type, &self.old_owner_id) {
            Some(format!("原所有者: {old_id} ({old_type})"))
        } else {
            None
        }
    }

    /// 获取转移摘要
    pub fn summary(&self) -> String {
        let mut parts = vec![self.new_owner_info()];

        if let Some(old_info) = self.old_owner_info() {
            parts.push(old_info);
        }

        if let Some(time_info) = self.transfer_time_formatted() {
            parts.push(time_info);
        }

        parts.join(", ")
    }
}

impl TransferOwnerResponse {
    /// 获取新所有者ID
    pub fn new_owner_id(&self) -> &str {
        &self.member.member_id
    }

    /// 获取新所有者类型
    pub fn new_owner_type(&self) -> &str {
        &self.member.member_type
    }

    /// 获取转移时间
    pub fn transfer_time(&self) -> Option<i64> {
        self.member.transfer_time
    }

    /// 是否成功转移
    pub fn is_transferred(&self) -> bool {
        !self.member.member_id.is_empty()
    }

    /// 获取转移成功摘要
    pub fn success_summary(&self) -> String {
        format!("所有者转移成功: {}", self.member.summary())
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_owner_request_builder() {
        let request = TransferOwnerRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .to_user("user123")
            .remove_current_owner()
            .with_notification()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert_eq!(request.remove_old_owner, Some(true));
        assert_eq!(request.need_notification, Some(true));
    }

    #[test]
    fn test_transfer_owner_convenience_method() {
        let request = TransferOwnerRequest::to_user("doccnxxxxxx", "doc", "user123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
    }
}
