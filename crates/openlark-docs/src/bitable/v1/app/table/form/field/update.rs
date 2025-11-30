#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use serde_json::Value;
use reqwest::Method;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新表单字段问题请求
#[derive(Clone)]
pub struct UpdateFormFieldQuestionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 表单ID
    #[serde(skip)]
    form_id: String,
    /// 问题ID
    #[serde(skip)]
    question_id: String,
    /// 问题标题
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 是否必填
    #[serde(skip)]
    required: Option<bool>,
    /// 是否可见
    #[serde(skip)]
    visible: Option<bool>,
    /// 问题设置
    #[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<Value>,
}

impl UpdateFormFieldQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            form_id: String::new(),
            question_id: String::new(),
            title: String::new(),
            description: None,
            required: None,
            visible: None,
            setting: None,
        }
    }
}

#[derive(Clone)]
pub struct UpdateFormFieldQuestionRequestBuilder {
    request: UpdateFormFieldQuestionRequest,
}

impl UpdateFormFieldQuestionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateFormFieldQuestionRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    pub fn question_id(mut self, question_id: impl Into<String>) -> Self {
        self.request.question_id = question_id.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.request.required = Some(required);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.request.visible = Some(visible);
        self
    }

    pub fn setting(mut self, setting: Value) -> Self {
        self.request.setting = Some(setting);
        self
    }

    pub fn build(self) -> UpdateFormFieldQuestionRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到UpdateFormFieldQuestionRequestBuilder
crate::impl_executable_builder_owned!(
    UpdateFormFieldQuestionRequestBuilder,
    super::FormFieldService,
    UpdateFormFieldQuestionRequest,
    Response<UpdateFormFieldQuestionResponse>,
    update,
);

/// 更新表单字段问题响应
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateFormFieldQuestionResponse {
    /// 是否成功
    pub success: bool,
    /// 更新后的问题信息
    pub question: Option<FormFieldQuestion>,
}

/// 表单字段问题信息
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormFieldQuestion {
    /// 问题ID
    pub question_id: String,
    /// 问题标题
    pub title: String,
    /// 问题描述
    pub description: Option<String>,
    /// 问题类型
    pub question_type: String,
    /// 是否必填
    pub required: bool,
    /// 是否可见
    pub visible: bool,
    /// 字段ID
    pub field_id: String,
    /// 问题设置
    pub setting: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单字段问题
pub async fn update_form_field_question(
    request: UpdateFormFieldQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<UpdateFormFieldQuestionResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
    api_req.api_path = BITABLE_V1_FORM_FIELD_QUESTION_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id)
        .replace("{question_id}", &request.question_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_form_field_question_request_builder() {
        let request = UpdateFormFieldQuestionRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .form_id("vewxxxxxx")
            .question_id("qstxxxxxx")
            .title("更新的问题标题")
            .required(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.question_id, "qstxxxxxx");
        assert_eq!(request.title, "更新的问题标题");
        assert_eq!(request.required, Some(true));
    }
}