use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};

/// 更新云文档权限设置请求 (v2)
#[derive(Debug, Serialize, Default, Clone)]
pub struct PatchPermissionPublicV2Request {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 链接分享设置
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_setting: Option<String>,
    /// 是否允许复制
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_copy: Option<bool>,
    /// 是否允许评论
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_comment: Option<bool>,
    /// 是否允许保存副本
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_save_copy: Option<bool>,
    /// 水印设置
    #[serde(skip_serializing_if = "Option::is_none")]
    watermark_setting: Option<String>,
    /// 是否允许分享到组织外
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_share_partner_tenant: Option<bool>,
    /// 访问权限设置
    #[serde(skip_serializing_if = "Option::is_none")]
    access_setting: Option<String>,
    /// 分享范围设置
    #[serde(skip_serializing_if = "Option::is_none")]
    share_scope: Option<String>,
    /// 过期时间 (Unix时间戳)
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<i64>,
}

impl PatchPermissionPublicV2Request {
    pub fn builder() -> PatchPermissionPublicV2RequestBuilder {
        PatchPermissionPublicV2RequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            ..Default::default()
        }
    }

    /// 更新文档权限设置
    pub fn for_doc(token: impl ToString) -> Self {
        Self::new(token, "doc")
    }

    /// 更新电子表格权限设置
    pub fn for_sheet(token: impl ToString) -> Self {
        Self::new(token, "sheet")
    }

    /// 更新多维表格权限设置
    pub fn for_bitable(token: impl ToString) -> Self {
        Self::new(token, "bitable")
    }

    /// 更新知识库权限设置
    pub fn for_wiki(token: impl ToString) -> Self {
        Self::new(token, "wiki")
    }
}

#[derive(Default)]
pub struct PatchPermissionPublicV2RequestBuilder {
    request: PatchPermissionPublicV2Request,
}

impl PatchPermissionPublicV2RequestBuilder {
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

    /// 设置链接分享设置
    pub fn link_share_setting(mut self, setting: impl ToString) -> Self {
        self.request.link_share_setting = Some(setting.to_string());
        self
    }

    /// 关闭分享
    pub fn close_sharing(mut self) -> Self {
        self.request.link_share_setting = Some("closed".to_string());
        self
    }

    /// 组织内可读
    pub fn tenant_readable(mut self) -> Self {
        self.request.link_share_setting = Some("tenant_readable".to_string());
        self
    }

    /// 组织内可编辑
    pub fn tenant_editable(mut self) -> Self {
        self.request.link_share_setting = Some("tenant_editable".to_string());
        self
    }

    /// 任何人可读
    pub fn anyone_readable(mut self) -> Self {
        self.request.link_share_setting = Some("anyone_readable".to_string());
        self
    }

    /// 任何人可编辑
    pub fn anyone_editable(mut self) -> Self {
        self.request.link_share_setting = Some("anyone_editable".to_string());
        self
    }

    /// 是否允许复制
    pub fn allow_copy(mut self, allow: bool) -> Self {
        self.request.allow_copy = Some(allow);
        self
    }

    /// 允许复制
    pub fn enable_copy(mut self) -> Self {
        self.request.allow_copy = Some(true);
        self
    }

    /// 禁止复制
    pub fn disable_copy(mut self) -> Self {
        self.request.allow_copy = Some(false);
        self
    }

    /// 是否允许评论
    pub fn allow_comment(mut self, allow: bool) -> Self {
        self.request.allow_comment = Some(allow);
        self
    }

    /// 允许评论
    pub fn enable_comment(mut self) -> Self {
        self.request.allow_comment = Some(true);
        self
    }

    /// 禁止评论
    pub fn disable_comment(mut self) -> Self {
        self.request.allow_comment = Some(false);
        self
    }

    /// 是否允许保存副本
    pub fn allow_save_copy(mut self, allow: bool) -> Self {
        self.request.allow_save_copy = Some(allow);
        self
    }

    /// 允许保存副本
    pub fn enable_save_copy(mut self) -> Self {
        self.request.allow_save_copy = Some(true);
        self
    }

    /// 禁止保存副本
    pub fn disable_save_copy(mut self) -> Self {
        self.request.allow_save_copy = Some(false);
        self
    }

    /// 水印设置
    pub fn watermark_setting(mut self, setting: impl ToString) -> Self {
        self.request.watermark_setting = Some(setting.to_string());
        self
    }

    /// 开启水印
    pub fn enable_watermark(mut self) -> Self {
        self.request.watermark_setting = Some("visible".to_string());
        self
    }

    /// 关闭水印
    pub fn disable_watermark(mut self) -> Self {
        self.request.watermark_setting = Some("none".to_string());
        self
    }

    /// 是否允许分享到组织外
    pub fn allow_share_partner_tenant(mut self, allow: bool) -> Self {
        self.request.allow_share_partner_tenant = Some(allow);
        self
    }

    /// 允许组织外分享
    pub fn enable_external_share(mut self) -> Self {
        self.request.allow_share_partner_tenant = Some(true);
        self
    }

    /// 禁止组织外分享
    pub fn disable_external_share(mut self) -> Self {
        self.request.allow_share_partner_tenant = Some(false);
        self
    }

    /// 访问权限设置
    pub fn access_setting(mut self, setting: impl ToString) -> Self {
        self.request.access_setting = Some(setting.to_string());
        self
    }

    /// 分享范围设置
    pub fn share_scope(mut self, scope: impl ToString) -> Self {
        self.request.share_scope = Some(scope.to_string());
        self
    }

    /// 设置过期时间 (Unix时间戳)
    pub fn expire_time(mut self, timestamp: i64) -> Self {
        self.request.expire_time = Some(timestamp);
        self
    }

    /// 设置过期时间 (从现在开始的秒数)
    pub fn expire_after_seconds(mut self, seconds: i64) -> Self {
        let expire_time = chrono::Utc::now().timestamp() + seconds;
        self.request.expire_time = Some(expire_time);
        self
    }

    /// 设置过期时间 (从现在开始的小时数)
    pub fn expire_after_hours(self, hours: i64) -> Self {
        self.expire_after_seconds(hours * 3600)
    }

    /// 设置过期时间 (从现在开始的天数)
    pub fn expire_after_days(self, days: i64) -> Self {
        self.expire_after_seconds(days * 86400)
    }

    /// 移除过期时间 (永久有效)
    pub fn never_expire(mut self) -> Self {
        self.request.expire_time = Some(0); // 0表示永久有效
        self
    }

    /// 设置为企业级安全模式
    pub fn enterprise_secure_mode(mut self) -> Self {
        self.request.link_share_setting = Some("tenant_readable".to_string());
        self.request.allow_copy = Some(false);
        self.request.allow_comment = Some(false);
        self.request.allow_save_copy = Some(false);
        self.request.watermark_setting = Some("visible".to_string());
        self.request.allow_share_partner_tenant = Some(false);
        self
    }

    /// 设置为协作模式
    pub fn collaboration_mode(mut self) -> Self {
        self.request.link_share_setting = Some("tenant_editable".to_string());
        self.request.allow_copy = Some(true);
        self.request.allow_comment = Some(true);
        self.request.allow_save_copy = Some(true);
        self.request.watermark_setting = Some("none".to_string());
        self.request.allow_share_partner_tenant = Some(false);
        self
    }

    /// 设置为公开分享模式
    pub fn public_share_mode(mut self) -> Self {
        self.request.link_share_setting = Some("anyone_readable".to_string());
        self.request.allow_copy = Some(false);
        self.request.allow_comment = Some(true);
        self.request.allow_save_copy = Some(false);
        self.request.watermark_setting = Some("visible".to_string());
        self.request.allow_share_partner_tenant = Some(true);
        self
    }

    /// 设置为开放编辑模式
    pub fn open_edit_mode(mut self) -> Self {
        self.request.link_share_setting = Some("anyone_editable".to_string());
        self.request.allow_copy = Some(true);
        self.request.allow_comment = Some(true);
        self.request.allow_save_copy = Some(true);
        self.request.watermark_setting = Some("none".to_string());
        self.request.allow_share_partner_tenant = Some(true);
        self
    }

    pub fn build(mut self) -> PatchPermissionPublicV2Request {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

crate::impl_executable_builder_owned!(
    PatchPermissionPublicV2RequestBuilder,
    crate::service::cloud_docs::permission::PermissionService,
    PatchPermissionPublicV2Request,
    BaseResponse<PatchPermissionPublicV2Response>,
    patch_permission_public_v2
);

/// 权限更新结果 (v2)
#[derive(Debug, Deserialize)]
pub struct PermissionUpdateResultV2 {
    /// 链接分享设置
    pub link_share_setting: Option<String>,
    /// 是否允许复制
    pub allow_copy: Option<bool>,
    /// 是否允许评论
    pub allow_comment: Option<bool>,
    /// 是否允许保存副本
    pub allow_save_copy: Option<bool>,
    /// 水印设置
    pub watermark_setting: Option<String>,
    /// 是否允许分享到组织外
    pub allow_share_partner_tenant: Option<bool>,
    /// 访问权限设置
    pub access_setting: Option<String>,
    /// 分享范围设置
    pub share_scope: Option<String>,
    /// 过期时间
    pub expire_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 更新云文档权限设置响应 (v2)
#[derive(Debug, Deserialize)]
pub struct PatchPermissionPublicV2Response {
    /// 更新后的权限设置
    pub permission_public: PermissionUpdateResultV2,
}

impl ApiResponseTrait for PatchPermissionPublicV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新云文档权限设置 (v2)
pub async fn patch_permission_public_v2(
    request: PatchPermissionPublicV2Request,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchPermissionPublicV2Response>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
    api_req.api_path = DRIVE_V2_PERMISSIONS_PUBLIC.replace("{}", &request.token);

    // 添加查询参数
    api_req
        .query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PermissionUpdateResultV2 {
    /// 获取分享级别描述
    pub fn share_level_description(&self) -> Option<&'static str> {
        self.link_share_setting
            .as_ref()
            .map(|setting| match setting.as_str() {
                "closed" => "关闭分享",
                "tenant_readable" => "组织内可读",
                "tenant_editable" => "组织内可编辑",
                "anyone_readable" => "任何人可读",
                "anyone_editable" => "任何人可编辑",
                _ => "未知设置",
            })
    }

    /// 是否有更新时间
    pub fn has_update_time(&self) -> bool {
        self.update_time.is_some()
    }

    /// 是否有过期时间
    pub fn has_expire_time(&self) -> bool {
        self.expire_time.is_some()
    }

    /// 获取更新时间格式化字符串
    pub fn update_time_formatted(&self) -> Option<String> {
        self.update_time
            .map(|timestamp| format!("更新时间: {timestamp}"))
    }

    /// 获取过期时间格式化字符串
    pub fn expire_time_formatted(&self) -> Option<String> {
        self.expire_time.map(|timestamp| {
            if timestamp == 0 {
                "永久有效".to_string()
            } else {
                let datetime =
                    chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
                format!("过期时间: {}", datetime.format("%Y-%m-%d %H:%M:%S"))
            }
        })
    }

    /// 获取权限变更摘要
    pub fn changes_summary(&self) -> Vec<String> {
        let mut changes = Vec::new();

        if let Some(ref setting) = self.link_share_setting {
            changes.push(format!(
                "分享设置: {}",
                match setting.as_str() {
                    "closed" => "关闭分享",
                    "tenant_readable" => "组织内可读",
                    "tenant_editable" => "组织内可编辑",
                    "anyone_readable" => "任何人可读",
                    "anyone_editable" => "任何人可编辑",
                    _ => setting,
                }
            ));
        }

        if let Some(allow_copy) = self.allow_copy {
            changes.push(format!(
                "复制权限: {}",
                if allow_copy { "允许" } else { "禁止" }
            ));
        }

        if let Some(allow_comment) = self.allow_comment {
            changes.push(format!(
                "评论权限: {}",
                if allow_comment { "允许" } else { "禁止" }
            ));
        }

        if let Some(allow_save_copy) = self.allow_save_copy {
            changes.push(format!(
                "保存副本: {}",
                if allow_save_copy { "允许" } else { "禁止" }
            ));
        }

        if let Some(ref watermark) = self.watermark_setting {
            changes.push(format!(
                "水印设置: {}",
                match watermark.as_str() {
                    "visible" => "显示水印",
                    "none" => "无水印",
                    _ => watermark,
                }
            ));
        }

        if let Some(allow_external) = self.allow_share_partner_tenant {
            changes.push(format!(
                "组织外分享: {}",
                if allow_external { "允许" } else { "禁止" }
            ));
        }

        if let Some(ref access) = self.access_setting {
            changes.push(format!("访问设置: {access}"));
        }

        if let Some(ref scope) = self.share_scope {
            changes.push(format!("分享范围: {scope}"));
        }

        if let Some(expire_time) = self.expire_time {
            if expire_time == 0 {
                changes.push("过期设置: 永久有效".to_string());
            } else {
                let datetime = chrono::DateTime::from_timestamp(expire_time, 0)
                    .unwrap_or_else(chrono::Utc::now);
                changes.push(format!(
                    "过期设置: {}",
                    datetime.format("%Y-%m-%d %H:%M:%S")
                ));
            }
        }

        changes
    }

    /// 计算安全级别
    pub fn security_level(&self) -> &'static str {
        if let Some(ref setting) = self.link_share_setting {
            match setting.as_str() {
                "closed" => "最安全",
                "tenant_readable" => {
                    if self.allow_share_partner_tenant == Some(false) {
                        "较安全"
                    } else {
                        "中等安全"
                    }
                }
                "tenant_editable" => "中等安全",
                "anyone_readable" => "较低安全",
                "anyone_editable" => "低安全",
                _ => "未知",
            }
        } else {
            "未变更"
        }
    }

    /// 获取高级功能变更
    pub fn advanced_changes(&self) -> Vec<String> {
        let mut changes = Vec::new();

        if self.access_setting.is_some() {
            changes.push("访问权限配置已更新".to_string());
        }

        if self.share_scope.is_some() {
            changes.push("分享范围已调整".to_string());
        }

        if self.allow_share_partner_tenant.is_some() {
            changes.push("组织外分享权限已变更".to_string());
        }

        if self.expire_time.is_some() {
            changes.push("过期时间已设置".to_string());
        }

        changes
    }
}

impl PatchPermissionPublicV2Response {
    /// 获取更新结果
    pub fn update_result(&self) -> &PermissionUpdateResultV2 {
        &self.permission_public
    }

    /// 是否更新成功
    pub fn is_updated(&self) -> bool {
        // 如果有任何字段有值，说明更新成功
        self.permission_public.link_share_setting.is_some()
            || self.permission_public.allow_copy.is_some()
            || self.permission_public.allow_comment.is_some()
            || self.permission_public.allow_save_copy.is_some()
            || self.permission_public.watermark_setting.is_some()
            || self.permission_public.allow_share_partner_tenant.is_some()
            || self.permission_public.access_setting.is_some()
            || self.permission_public.share_scope.is_some()
            || self.permission_public.expire_time.is_some()
    }

    /// 获取更新摘要
    pub fn update_summary(&self) -> String {
        let changes = self.permission_public.changes_summary();
        if changes.is_empty() {
            "权限设置无变更".to_string()
        } else {
            format!("权限设置已更新: {}", changes.join(", "))
        }
    }

    /// 获取安全性评估
    pub fn security_assessment(&self) -> String {
        format!(
            "安全级别: {} - {}",
            self.permission_public.security_level(),
            self.get_security_tips()
        )
    }

    /// 获取安全建议
    fn get_security_tips(&self) -> &'static str {
        if let Some(ref setting) = self.permission_public.link_share_setting {
            match setting.as_str() {
                "closed" => "文档仅限邀请用户访问，安全性最高",
                "tenant_readable" => {
                    if self.permission_public.allow_share_partner_tenant == Some(false) {
                        "组织内用户可查看，适合内部分享"
                    } else {
                        "允许组织外分享，注意内容安全"
                    }
                }
                "tenant_editable" => "组织内用户可编辑，注意权限管控",
                "anyone_readable" => "任何人可查看，建议开启密码保护",
                "anyone_editable" => "任何人可编辑，存在较高安全风险",
                _ => "权限设置需要进一步确认",
            }
        } else {
            "权限设置保持原有配置"
        }
    }

    /// 获取操作建议
    pub fn operation_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(ref setting) = self.permission_public.link_share_setting {
            if setting == "anyone_editable" || setting == "anyone_readable" {
                recommendations.push("建议设置密码保护".to_string());

                if self.permission_public.allow_copy == Some(true) {
                    recommendations.push("建议禁止复制以防止内容泄露".to_string());
                }

                if self.permission_public.watermark_setting != Some("visible".to_string()) {
                    recommendations.push("建议开启水印以标识来源".to_string());
                }
            }
        }

        if self.permission_public.allow_share_partner_tenant == Some(true) {
            recommendations.push("已开启组织外分享，请确保内容合规".to_string());
        }

        if let Some(expire_time) = self.permission_public.expire_time {
            if expire_time > 0 {
                let remaining = expire_time - chrono::Utc::now().timestamp();
                if remaining < 86400 {
                    recommendations.push("分享即将过期，请注意及时续期".to_string());
                }
            }
        }

        let advanced_changes = self.permission_public.advanced_changes();
        if !advanced_changes.is_empty() {
            recommendations.push("已应用高级配置，请验证功能是否符合预期".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("当前权限配置合理".to_string());
        }

        recommendations
    }

    /// 获取高级功能报告
    pub fn advanced_features_report(&self) -> String {
        let changes = self.permission_public.advanced_changes();
        if changes.is_empty() {
            "未更新高级功能".to_string()
        } else {
            format!("高级功能更新: {}", changes.join(", "))
        }
    }

    /// 获取过期状态报告
    pub fn expiration_report(&self) -> Option<String> {
        self.permission_public.expire_time_formatted()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_permission_public_v2_request_builder() {
        let request = PatchPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .tenant_readable()
            .disable_copy()
            .enable_comment()
            .enable_watermark()
            .disable_external_share()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(
            request.link_share_setting,
            Some("tenant_readable".to_string())
        );
        assert_eq!(request.allow_copy, Some(false));
        assert_eq!(request.allow_comment, Some(true));
        assert_eq!(request.watermark_setting, Some("visible".to_string()));
        assert_eq!(request.allow_share_partner_tenant, Some(false));
    }

    #[test]
    fn test_convenience_modes() {
        let enterprise_request = PatchPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .enterprise_secure_mode()
            .build();

        assert_eq!(
            enterprise_request.link_share_setting,
            Some("tenant_readable".to_string())
        );
        assert_eq!(enterprise_request.allow_copy, Some(false));
        assert_eq!(enterprise_request.allow_comment, Some(false));
        assert_eq!(enterprise_request.allow_save_copy, Some(false));
        assert_eq!(
            enterprise_request.watermark_setting,
            Some("visible".to_string())
        );
        assert_eq!(enterprise_request.allow_share_partner_tenant, Some(false));

        let public_request = PatchPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .public_share_mode()
            .build();

        assert_eq!(
            public_request.link_share_setting,
            Some("anyone_readable".to_string())
        );
        assert_eq!(public_request.allow_copy, Some(false));
        assert_eq!(public_request.allow_comment, Some(true));
        assert_eq!(public_request.allow_save_copy, Some(false));
        assert_eq!(
            public_request.watermark_setting,
            Some("visible".to_string())
        );
        assert_eq!(public_request.allow_share_partner_tenant, Some(true));
    }

    #[test]
    fn test_expiration_settings() {
        let request = PatchPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .expire_after_days(7)
            .build();

        assert!(request.expire_time.is_some());
        assert!(request.expire_time.unwrap() > chrono::Utc::now().timestamp());

        let permanent_request = PatchPermissionPublicV2Request::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .never_expire()
            .build();

        assert_eq!(permanent_request.expire_time, Some(0));
    }

    #[test]
    fn test_permission_update_result_v2_methods() {
        let result = PermissionUpdateResultV2 {
            link_share_setting: Some("tenant_editable".to_string()),
            allow_copy: Some(false),
            allow_comment: Some(true),
            allow_save_copy: Some(false),
            watermark_setting: Some("visible".to_string()),
            allow_share_partner_tenant: Some(false),
            access_setting: Some("advanced".to_string()),
            share_scope: Some("limited".to_string()),
            expire_time: Some(chrono::Utc::now().timestamp() + 86400),
            update_time: Some(chrono::Utc::now().timestamp()),
        };

        assert_eq!(result.share_level_description(), Some("组织内可编辑"));
        assert!(result.has_update_time());
        assert!(result.has_expire_time());
        assert_eq!(result.security_level(), "中等安全");

        let changes = result.changes_summary();
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| c.contains("组织内可编辑")));
        assert!(changes.iter().any(|c| c.contains("复制权限: 禁止")));
        assert!(changes.iter().any(|c| c.contains("评论权限: 允许")));

        let advanced_changes = result.advanced_changes();
        assert!(!advanced_changes.is_empty());
        assert!(advanced_changes
            .iter()
            .any(|c| c.contains("访问权限配置已更新")));
    }
}
