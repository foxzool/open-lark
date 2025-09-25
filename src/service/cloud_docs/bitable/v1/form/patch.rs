use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::FormService;
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
    impl_executable_builder_owned,
    service::bitable::v1::form::FormQuestion,
};

/// 更新表单问题请求
#[derive(Debug, Serialize, Default)]
pub struct PatchFormQuestionRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    /// 问题设置
    #[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<serde_json::Value>,
}

impl PatchFormQuestionRequest {
    pub fn builder() -> PatchFormQuestionRequestBuilder {
        PatchFormQuestionRequestBuilder::default()
    }

    pub fn new(
        app_token: impl ToString,
        form_id: impl ToString,
        question_id: impl ToString,
    ) -> Self {
        Self {
            app_token: app_token.to_string(),
            form_id: form_id.to_string(),
            question_id: question_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl PatchFormQuestionRequestBuilder {
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

    /// 问题ID
    pub fn question_id(mut self, question_id: impl ToString) -> Self {
        self.request.question_id = question_id.to_string();
        self
    }

    /// 问题标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    /// 问题描述
    pub fn description(mut self, description: impl ToString) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    /// 是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.request.required = Some(required);
        self
    }

    /// 是否可见
    pub fn visible(mut self, visible: bool) -> Self {
        self.request.visible = Some(visible);
        self
    }

    /// 问题设置
    pub fn setting(mut self, setting: serde_json::Value) -> Self {
        self.request.setting = Some(setting);
        self
    }

    pub fn build(mut self) -> PatchFormQuestionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    PatchFormQuestionRequestBuilder,
    FormService,
    PatchFormQuestionRequest,
    BaseResponse<PatchFormQuestionResponse>,
    patch
);

/// 更新表单问题响应
#[derive(Debug, Deserialize)]
pub struct PatchFormQuestionResponse {
    /// 更新后的问题信息
    pub question: FormQuestion,
}

impl ApiResponseTrait for PatchFormQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单问题
pub async fn patch_form_question(
    request: PatchFormQuestionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchFormQuestionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
    api_req.api_path = BITABLE_V1_FORM_PATCH
        .replace("{app_token}", &request.app_token)
        .replace("{form_id}", &request.form_id)
        .replace("{question_id}", &request.question_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_form_question_request_builder() {
        let request = PatchFormQuestionRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .form_id("vewxxxxxx")
            .question_id("qstxxxxxx")
            .title("更新后的问题标题")
            .required(true)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.form_id, "vewxxxxxx");
        assert_eq!(request.question_id, "qstxxxxxx");
        assert_eq!(request.title, Some("更新后的问题标题".to_string()));
        assert_eq!(request.required, Some(true));
    }
}
