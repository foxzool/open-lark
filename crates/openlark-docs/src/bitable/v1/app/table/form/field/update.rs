use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新表单字段问题请求
#[derive(Debug, Clone)]
pub struct UpdateFormFieldQuestionRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 表单ID
    form_id: String,
    /// 问题ID
    question_id: String,
    /// 问题标题
    title: String,
    /// 问题描述
    description: Option<String>,
    /// 是否必填
    required: Option<bool>,
    /// 是否可见
    visible: Option<bool>,
    /// 问题设置
    setting: Option<Value>,
}

impl Default for UpdateFormFieldQuestionRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::put(
                "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/forms/{}/fields/{}",
            ),
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

impl UpdateFormFieldQuestionRequest {
    pub fn new(_config: Config) -> Self {
        Self::default()
    }
}

pub struct UpdateFormFieldQuestionRequestBuilder {
    request: UpdateFormFieldQuestionRequest,
}

impl Default for UpdateFormFieldQuestionRequestBuilder {
    fn default() -> Self {
        Self {
            request: UpdateFormFieldQuestionRequest::default(),
        }
    }
}

impl UpdateFormFieldQuestionRequestBuilder {
    pub fn new() -> Self {
        Self::default()
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
    update
);

/// 更新表单字段问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFormFieldQuestionResponse {
    /// 是否成功
    pub success: bool,
    /// 更新后的问题信息
    pub question: Option<FormFieldQuestion>,
}

/// 表单字段问题信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/forms/{}/fields/{}",
        &request.app_token, &request.form_id, &request.question_id
    );
    // 创建新的请求，不使用api_path方法
    let api_req = ApiRequest::<()>::put(&url);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}
