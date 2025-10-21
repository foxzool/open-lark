use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};

/// 获取云文档权限设置请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetPermissionPublicRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
}

impl GetPermissionPublicRequest {
    pub fn builder() -> GetPermissionPublicRequestBuilder {
        GetPermissionPublicRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            ..Default::default()
        }
    }

    /// 获取文档权限设置
    pub fn for_doc(token: impl ToString) -> Self {
        Self::new(token, "doc")
    }

    /// 获取电子表格权限设置
    pub fn for_sheet(token: impl ToString) -> Self {
        Self::new(token, "sheet")
    }

    /// 获取多维表格权限设置
    pub fn for_bitable(token: impl ToString) -> Self {
        Self::new(token, "bitable")
    }

    /// 获取知识库权限设置
    pub fn for_wiki(token: impl ToString) -> Self {
        Self::new(token, "wiki")
    }
}

#[derive(Default)]
pub struct GetPermissionPublicRequestBuilder {
    request: GetPermissionPublicRequest,
}

impl GetPermissionPublicRequestBuilder {
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

    pub fn build(mut self) -> GetPermissionPublicRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 公开访问设置
#[derive(Debug, Deserialize)]
pub struct PublicSettings {
    /// 链接分享设置
    pub link_share_setting: String,
    /// 密码保护（如果有）
    pub password_switch: bool,
    /// 是否允许复制
    pub allow_copy: bool,
    /// 是否允许评论
    pub allow_comment: bool,
    /// 是否允许保存副本
    pub allow_save_copy: bool,
    /// 访问权限
    pub access_setting: Option<String>,
    /// 水印设置
    pub watermark_setting: Option<String>,
}

/// 获取云文档权限设置响应
#[derive(Debug, Deserialize)]
pub struct GetPermissionPublicResponse {
    /// 公开访问设置
    pub permission_public: PublicSettings,
    /// 外部访问设置（如果有）
    pub external_access: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置
pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetPermissionPublicResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path =
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_PUBLIC, "token", &request.token);

    // 添加查询参数
    api_req
        .query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PublicSettings {
    /// 是否开启了链接分享
    pub fn is_link_share_enabled(&self) -> bool {
        self.link_share_setting == "tenant_readable"
            || self.link_share_setting == "tenant_editable"
            || self.link_share_setting == "anyone_readable"
            || self.link_share_setting == "anyone_editable"
    }

    /// 是否允许组织内访问
    pub fn is_tenant_accessible(&self) -> bool {
        self.link_share_setting == "tenant_readable" || self.link_share_setting == "tenant_editable"
    }

    /// 是否允许任何人访问
    pub fn is_anyone_accessible(&self) -> bool {
        self.link_share_setting == "anyone_readable" || self.link_share_setting == "anyone_editable"
    }

    /// 是否允许编辑
    pub fn is_editable(&self) -> bool {
        self.link_share_setting == "tenant_editable" || self.link_share_setting == "anyone_editable"
    }

    /// 是否仅可读
    pub fn is_readonly(&self) -> bool {
        self.link_share_setting == "tenant_readable" || self.link_share_setting == "anyone_readable"
    }

    /// 是否开启密码保护
    pub fn has_password_protection(&self) -> bool {
        self.password_switch
    }

    /// 获取分享级别描述
    pub fn share_level_description(&self) -> &'static str {
        match self.link_share_setting.as_str() {
            "closed" => "关闭分享",
            "tenant_readable" => "组织内可读",
            "tenant_editable" => "组织内可编辑",
            "anyone_readable" => "任何人可读",
            "anyone_editable" => "任何人可编辑",
            _ => "未知设置",
        }
    }

    /// 获取权限摘要
    pub fn permissions_summary(&self) -> String {
        let mut features = Vec::new();

        if self.allow_copy {
            features.push("允许复制");
        }
        if self.allow_comment {
            features.push("允许评论");
        }
        if self.allow_save_copy {
            features.push("允许保存副本");
        }
        if self.password_switch {
            features.push("密码保护");
        }

        if features.is_empty() {
            "基础权限".to_string()
        } else {
            features.join(", ")
        }
    }

    /// 获取安全级别
    pub fn security_level(&self) -> &'static str {
        if self.link_share_setting == "closed" {
            "最安全"
        } else if self.password_switch {
            "较安全"
        } else if self.is_tenant_accessible() {
            "中等安全"
        } else if self.is_anyone_accessible() {
            "较低安全"
        } else {
            "未知"
        }
    }
}

impl GetPermissionPublicResponse {
    /// 获取公开设置
    pub fn public_settings(&self) -> &PublicSettings {
        &self.permission_public
    }

    /// 是否允许外部访问
    pub fn allows_external_access(&self) -> bool {
        self.permission_public.is_link_share_enabled()
    }

    /// 获取设置摘要
    pub fn settings_summary(&self) -> String {
        format!(
            "{} - {} (安全级别: {})",
            self.permission_public.share_level_description(),
            self.permission_public.permissions_summary(),
            self.permission_public.security_level()
        )
    }

    /// 是否有外部访问配置
    pub fn has_external_config(&self) -> bool {
        self.external_access.is_some()
    }

    /// 安全性建议
    pub fn security_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !self.permission_public.password_switch && self.permission_public.is_anyone_accessible()
        {
            recommendations.push("建议开启密码保护以提高安全性".to_string());
        }

        if self.permission_public.allow_copy && self.permission_public.is_anyone_accessible() {
            recommendations.push("建议限制复制权限以防止内容泄露".to_string());
        }

        if self.permission_public.is_editable() && self.permission_public.is_anyone_accessible() {
            recommendations.push("建议将编辑权限限制在组织内".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("当前权限设置合理".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_permission_public_request_builder() {
        let request = GetPermissionPublicRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
    }

    #[test]
    fn test_convenience_methods() {
        let request = GetPermissionPublicRequest::for_doc("doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");

        let request = GetPermissionPublicRequest::for_sheet("shtcnxxxxxx");
        assert_eq!(request.obj_type, "sheet");

        let request = GetPermissionPublicRequest::for_bitable("bblcnxxxxxx");
        assert_eq!(request.obj_type, "bitable");

        let request = GetPermissionPublicRequest::for_wiki("wikicnxxxxxx");
        assert_eq!(request.obj_type, "wiki");
    }

    #[test]
    fn test_public_settings_methods() {
        let settings = PublicSettings {
            link_share_setting: "tenant_editable".to_string(),
            password_switch: true,
            allow_copy: true,
            allow_comment: true,
            allow_save_copy: false,
            access_setting: None,
            watermark_setting: None,
        };

        assert!(settings.is_link_share_enabled());
        assert!(settings.is_tenant_accessible());
        assert!(!settings.is_anyone_accessible());
        assert!(settings.is_editable());
        assert!(!settings.is_readonly());
        assert!(settings.has_password_protection());
        assert_eq!(settings.share_level_description(), "组织内可编辑");
        assert_eq!(settings.security_level(), "较安全");
    }
}
