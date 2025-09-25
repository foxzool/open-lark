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

/// 获取云文档权限设置请求 (v2)
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetPermissionPublicV2Request {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
}

impl GetPermissionPublicV2Request {
    pub fn builder() -> GetPermissionPublicV2RequestBuilder {
        GetPermissionPublicV2RequestBuilder::default()
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
pub struct GetPermissionPublicV2RequestBuilder {
    request: GetPermissionPublicV2Request,
}

impl GetPermissionPublicV2RequestBuilder {
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

    pub fn build(mut self) -> GetPermissionPublicV2Request {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

crate::impl_executable_builder_owned!(
    GetPermissionPublicV2RequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    GetPermissionPublicV2Request,
    BaseResponse<GetPermissionPublicV2Response>,
    get_permission_public_v2
);

/// 公开访问设置 (v2)
#[derive(Debug, Deserialize)]
pub struct PublicSettingsV2 {
    /// 链接分享设置
    pub link_share_setting: String,
    /// 密码保护开关
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
    /// 是否允许分享到组织外
    pub allow_share_partner_tenant: Option<bool>,
    /// 外部分享设置
    pub external_access_entity: Option<serde_json::Value>,
    /// 评论权限设置
    pub comment_entity: Option<serde_json::Value>,
    /// 分享范围设置
    pub share_scope: Option<String>,
    /// 到期时间
    pub expire_time: Option<i64>,
}

/// 获取云文档权限设置响应 (v2)
#[derive(Debug, Deserialize)]
pub struct GetPermissionPublicV2Response {
    /// 公开访问设置
    pub permission_public: PublicSettingsV2,
}

impl ApiResponseTrait for GetPermissionPublicV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置 (v2)
pub async fn get_permission_public_v2(
    request: GetPermissionPublicV2Request,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetPermissionPublicV2Response>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path =
        EndpointBuilder::replace_param(DRIVE_V2_PERMISSIONS_PUBLIC, "token", &request.token);

    // 添加查询参数
    api_req
        .query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PublicSettingsV2 {
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

    /// 是否允许分享到组织外
    pub fn allows_external_share(&self) -> bool {
        self.allow_share_partner_tenant.unwrap_or(false)
    }

    /// 是否有过期时间
    pub fn has_expire_time(&self) -> bool {
        self.expire_time.is_some()
    }

    /// 是否已过期
    pub fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            let now = chrono::Utc::now().timestamp();
            now > expire_time
        } else {
            false
        }
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
        if self.allows_external_share() {
            features.push("组织外分享");
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
        } else if self.is_tenant_accessible() && !self.allows_external_share() {
            "中等安全"
        } else if self.is_anyone_accessible() || self.allows_external_share() {
            "较低安全"
        } else {
            "未知"
        }
    }

    /// 获取分享范围描述
    pub fn share_scope_description(&self) -> Option<&str> {
        self.share_scope.as_deref()
    }

    /// 获取过期时间格式化字符串
    pub fn expire_time_formatted(&self) -> Option<String> {
        self.expire_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_valid_time(&self) -> Option<i64> {
        if let Some(expire_time) = self.expire_time {
            let now = chrono::Utc::now().timestamp();
            if expire_time > now {
                Some(expire_time - now)
            } else {
                Some(0) // 已过期
            }
        } else {
            None // 永久有效
        }
    }

    /// 获取高级功能状态
    pub fn advanced_features_status(&self) -> Vec<String> {
        let mut features = Vec::new();

        if let Some(access_setting) = &self.access_setting {
            features.push(format!("访问设置: {access_setting}"));
        }

        if let Some(watermark) = &self.watermark_setting {
            features.push(format!("水印: {watermark}"));
        }

        if self.comment_entity.is_some() {
            features.push("自定义评论权限".to_string());
        }

        if self.external_access_entity.is_some() {
            features.push("外部访问配置".to_string());
        }

        features
    }
}

impl GetPermissionPublicV2Response {
    /// 获取公开设置
    pub fn public_settings(&self) -> &PublicSettingsV2 {
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

    /// 是否有高级配置
    pub fn has_advanced_config(&self) -> bool {
        self.permission_public.external_access_entity.is_some()
            || self.permission_public.comment_entity.is_some()
            || self.permission_public.share_scope.is_some()
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

        if self.permission_public.allows_external_share() {
            recommendations.push("开启了组织外分享，请确保内容安全".to_string());
        }

        if self.permission_public.is_expired() {
            recommendations.push("文档分享已过期，需要重新设置".to_string());
        } else if let Some(remaining) = self.permission_public.remaining_valid_time() {
            if remaining < 86400 {
                // 少于24小时
                recommendations.push("文档分享即将过期，请注意及时续期".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("当前权限设置合理".to_string());
        }

        recommendations
    }

    /// 获取高级功能报告
    pub fn advanced_features_report(&self) -> String {
        let features = self.permission_public.advanced_features_status();
        if features.is_empty() {
            "未启用高级功能".to_string()
        } else {
            format!("高级功能: {}", features.join(", "))
        }
    }

    /// 过期状态检查
    pub fn expiration_status(&self) -> String {
        if let Some(expire_time) = self.permission_public.expire_time_formatted() {
            if self.permission_public.is_expired() {
                format!("已过期: {expire_time}")
            } else if let Some(remaining) = self.permission_public.remaining_valid_time() {
                let days = remaining / 86400;
                let hours = (remaining % 86400) / 3600;
                if days > 0 {
                    format!("剩余: {days}天{hours}小时 (过期时间: {expire_time})")
                } else {
                    format!("剩余: {hours}小时 (过期时间: {expire_time})")
                }
            } else {
                format!("过期时间: {expire_time}")
            }
        } else {
            "永久有效".to_string()
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_permission_public_v2_request_builder() {
        let request = GetPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
    }

    #[test]
    fn test_convenience_methods() {
        let request = GetPermissionPublicV2Request::for_doc("doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");

        let request = GetPermissionPublicV2Request::for_sheet("shtcnxxxxxx");
        assert_eq!(request.obj_type, "sheet");

        let request = GetPermissionPublicV2Request::for_bitable("bblcnxxxxxx");
        assert_eq!(request.obj_type, "bitable");

        let request = GetPermissionPublicV2Request::for_wiki("wikicnxxxxxx");
        assert_eq!(request.obj_type, "wiki");
    }

    #[test]
    fn test_public_settings_v2_methods() {
        let settings = PublicSettingsV2 {
            link_share_setting: "tenant_editable".to_string(),
            password_switch: true,
            allow_copy: true,
            allow_comment: true,
            allow_save_copy: false,
            access_setting: Some("advanced".to_string()),
            watermark_setting: Some("visible".to_string()),
            allow_share_partner_tenant: Some(true),
            external_access_entity: Some(serde_json::json!({})),
            comment_entity: None,
            share_scope: Some("limited".to_string()),
            expire_time: Some(chrono::Utc::now().timestamp() + 86400), // 24小时后过期
        };

        assert!(settings.is_link_share_enabled());
        assert!(settings.is_tenant_accessible());
        assert!(!settings.is_anyone_accessible());
        assert!(settings.is_editable());
        assert!(!settings.is_readonly());
        assert!(settings.has_password_protection());
        assert!(settings.allows_external_share());
        assert!(settings.has_expire_time());
        assert!(!settings.is_expired());
        assert_eq!(settings.share_level_description(), "组织内可编辑");
        assert_eq!(settings.security_level(), "较安全");
        assert_eq!(settings.share_scope_description(), Some("limited"));

        let features = settings.advanced_features_status();
        assert!(!features.is_empty());
        assert!(features.iter().any(|f| f.contains("访问设置")));
        assert!(features.iter().any(|f| f.contains("水印")));
    }

    #[test]
    fn test_expiration_logic() {
        let expired_settings = PublicSettingsV2 {
            link_share_setting: "anyone_readable".to_string(),
            password_switch: false,
            allow_copy: false,
            allow_comment: false,
            allow_save_copy: false,
            access_setting: None,
            watermark_setting: None,
            allow_share_partner_tenant: None,
            external_access_entity: None,
            comment_entity: None,
            share_scope: None,
            expire_time: Some(chrono::Utc::now().timestamp() - 3600), // 1小时前过期
        };

        assert!(expired_settings.is_expired());
        assert_eq!(expired_settings.remaining_valid_time(), Some(0));

        let permanent_settings = PublicSettingsV2 {
            link_share_setting: "tenant_readable".to_string(),
            password_switch: false,
            allow_copy: false,
            allow_comment: false,
            allow_save_copy: false,
            access_setting: None,
            watermark_setting: None,
            allow_share_partner_tenant: None,
            external_access_entity: None,
            comment_entity: None,
            share_scope: None,
            expire_time: None,
        };

        assert!(!permanent_settings.is_expired());
        assert_eq!(permanent_settings.remaining_valid_time(), None);
    }
}
