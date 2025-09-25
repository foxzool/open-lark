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

/// 更新云文档权限设置请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct PatchPermissionPublicRequest {
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
}

impl PatchPermissionPublicRequest {
    pub fn builder() -> PatchPermissionPublicRequestBuilder {
        PatchPermissionPublicRequestBuilder::default()
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
pub struct PatchPermissionPublicRequestBuilder {
    request: PatchPermissionPublicRequest,
}

impl PatchPermissionPublicRequestBuilder {
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

    /// 设置为安全模式（限制性设置）
    pub fn secure_mode(mut self) -> Self {
        self.request.link_share_setting = Some("tenant_readable".to_string());
        self.request.allow_copy = Some(false);
        self.request.allow_comment = Some(false);
        self.request.allow_save_copy = Some(false);
        self.request.watermark_setting = Some("visible".to_string());
        self
    }

    /// 设置为开放模式（宽松设置）
    pub fn open_mode(mut self) -> Self {
        self.request.link_share_setting = Some("anyone_editable".to_string());
        self.request.allow_copy = Some(true);
        self.request.allow_comment = Some(true);
        self.request.allow_save_copy = Some(true);
        self.request.watermark_setting = Some("none".to_string());
        self
    }

    pub fn build(mut self) -> PatchPermissionPublicRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 权限更新结果
#[derive(Debug, Deserialize)]
pub struct PermissionUpdateResult {
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
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 更新云文档权限设置响应
#[derive(Debug, Deserialize)]
pub struct PatchPermissionPublicResponse {
    /// 更新后的权限设置
    pub permission_public: PermissionUpdateResult,
}

impl ApiResponseTrait for PatchPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新云文档权限设置
pub async fn patch_permission_public(
    request: PatchPermissionPublicRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchPermissionPublicResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
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

impl PermissionUpdateResult {
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

    /// 获取更新时间格式化字符串
    pub fn update_time_formatted(&self) -> Option<String> {
        self.update_time
            .map(|timestamp| format!("更新时间: {timestamp}"))
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

        changes
    }

    /// 计算安全级别
    pub fn security_level(&self) -> &'static str {
        if let Some(ref setting) = self.link_share_setting {
            match setting.as_str() {
                "closed" => "最安全",
                "tenant_readable" => "较安全",
                "tenant_editable" => "中等安全",
                "anyone_readable" => "较低安全",
                "anyone_editable" => "低安全",
                _ => "未知",
            }
        } else {
            "未变更"
        }
    }
}

impl PatchPermissionPublicResponse {
    /// 获取更新结果
    pub fn update_result(&self) -> &PermissionUpdateResult {
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
                "tenant_readable" => "组织内用户可查看，适合内部分享",
                "tenant_editable" => "组织内用户可编辑，注意权限管控",
                "anyone_readable" => "任何人可查看，建议开启密码保护",
                "anyone_editable" => "任何人可编辑，存在安全风险",
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

        if recommendations.is_empty() {
            recommendations.push("当前权限配置合理".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_permission_public_request_builder() {
        let request = PatchPermissionPublicRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .tenant_readable()
            .disable_copy()
            .enable_comment()
            .enable_watermark()
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
    }

    #[test]
    fn test_convenience_modes() {
        let secure_request = PatchPermissionPublicRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .secure_mode()
            .build();

        assert_eq!(
            secure_request.link_share_setting,
            Some("tenant_readable".to_string())
        );
        assert_eq!(secure_request.allow_copy, Some(false));
        assert_eq!(secure_request.allow_comment, Some(false));
        assert_eq!(secure_request.allow_save_copy, Some(false));
        assert_eq!(
            secure_request.watermark_setting,
            Some("visible".to_string())
        );

        let open_request = PatchPermissionPublicRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .open_mode()
            .build();

        assert_eq!(
            open_request.link_share_setting,
            Some("anyone_editable".to_string())
        );
        assert_eq!(open_request.allow_copy, Some(true));
        assert_eq!(open_request.allow_comment, Some(true));
        assert_eq!(open_request.allow_save_copy, Some(true));
        assert_eq!(open_request.watermark_setting, Some("none".to_string()));
    }

    #[test]
    fn test_permission_update_result_methods() {
        let result = PermissionUpdateResult {
            link_share_setting: Some("tenant_editable".to_string()),
            allow_copy: Some(false),
            allow_comment: Some(true),
            allow_save_copy: Some(false),
            watermark_setting: Some("visible".to_string()),
            update_time: Some(1234567890),
        };

        assert_eq!(result.share_level_description(), Some("组织内可编辑"));
        assert!(result.has_update_time());
        assert_eq!(result.security_level(), "中等安全");

        let changes = result.changes_summary();
        assert_eq!(changes.len(), 5);
        assert!(changes.iter().any(|c| c.contains("组织内可编辑")));
        assert!(changes.iter().any(|c| c.contains("复制权限: 禁止")));
        assert!(changes.iter().any(|c| c.contains("评论权限: 允许")));
    }
}
