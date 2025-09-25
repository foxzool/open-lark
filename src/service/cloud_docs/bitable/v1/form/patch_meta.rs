use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::bitable::v1::form::Form,
};

/// 更新表单元数据请求
#[derive(Debug, Serialize, Default)]
pub struct PatchFormMetaRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 表单ID
    #[serde(skip)]
    form_id: String,
    /// 表单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 表单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 是否允许重复提交
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_resubmit: Option<bool>,
    /// 是否显示提交按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    show_submit_button: Option<bool>,
    /// 提交按钮文本
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_button_text: Option<String>,
    /// 是否分享表单
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    /// 是否需要登录
    #[serde(skip_serializing_if = "Option::is_none")]
    need_login: Option<bool>,
    /// 状态：启用/禁用
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

impl PatchFormMetaRequest {
    pub fn builder() -> PatchFormMetaRequestBuilder {
        PatchFormMetaRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, form_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            form_id: form_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct PatchFormMetaRequestBuilder {
    request: PatchFormMetaRequest,
}

impl PatchFormMetaRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表单ID
    pub fn form_id(mut self, form_id: impl ToString) -> Self {
        self.request.form_id = form_id.to_string();
        self
    }

    /// 表单名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 表单描述
    pub fn description(mut self, description: impl ToString) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    /// 是否允许重复提交
    pub fn allow_resubmit(mut self, allow_resubmit: bool) -> Self {
        self.request.allow_resubmit = Some(allow_resubmit);
        self
    }

    /// 是否显示提交按钮
    pub fn show_submit_button(mut self, show_submit_button: bool) -> Self {
        self.request.show_submit_button = Some(show_submit_button);
        self
    }

    /// 提交按钮文本
    pub fn submit_button_text(mut self, submit_button_text: impl ToString) -> Self {
        self.request.submit_button_text = Some(submit_button_text.to_string());
        self
    }

    /// 是否分享表单
    pub fn shared(mut self, shared: bool) -> Self {
        self.request.shared = Some(shared);
        self
    }

    /// 是否需要登录
    pub fn need_login(mut self, need_login: bool) -> Self {
        self.request.need_login = Some(need_login);
        self
    }

    /// 状态：启用/禁用
    pub fn status(mut self, status: impl ToString) -> Self {
        self.request.status = Some(status.to_string());
        self
    }

    pub fn build(mut self) -> PatchFormMetaRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新表单元数据响应
#[derive(Debug, Deserialize)]
pub struct PatchFormMetaResponse {
    /// 更新后的表单信息
    pub form: Form,
}

impl ApiResponseTrait for PatchFormMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单元数据
pub async fn patch_form_meta(
    request: PatchFormMetaRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchFormMetaResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
    api_req.api_path = BITABLE_V1_FORM_PATCH
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_form_meta_request_builder() {
        let request = PatchFormMetaRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .form_id("vewxxxxxx")
            .name("更新后的表单名称")
            .description("更新后的表单描述")
            .allow_resubmit(true)
            .shared(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.name, Some("更新后的表单名称".to_string()));
        assert_eq!(request.description, Some("更新后的表单描述".to_string()));
        assert_eq!(request.allow_resubmit, Some(true));
        assert_eq!(request.shared, Some(true));
    }
}
