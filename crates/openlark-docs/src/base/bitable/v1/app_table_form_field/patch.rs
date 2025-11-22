#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use super::FormService;
use openlark_core::,
{
    core::{
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    };
    impl_executable_builder_owned,
    service::bitable::v1::app_table_form_field::FormQuestion,
};

/// 更新表单问题请求
#[derive(Clone)]
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
}

impl PatchFormQuestionRequest {
    pub fn builder() -> PatchFormQuestionRequestBuilder {
        PatchFormQuestionRequestBuilder::default()
    }
}

/// 更新表单问题请求构建器
#[derive(Default, Clone)]
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl PatchFormQuestionRequestBuilder {
    /// 设置多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    /// 设置表单ID
    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    /// 设置问题ID
    pub fn question_id(mut self, question_id: impl Into<String>) -> Self {
        self.request.question_id = question_id.into();
        self
    }

    /// 设置问题标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置问题描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.request.required = Some(required);
        self
    }

    /// 设置是否可见
    pub fn visible(mut self, visible: bool) -> Self {
        self.request.visible = Some(visible);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> PatchFormQuestionRequest {
        self.request
    }
}

/// 更新表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionResponse {
    /// 问题ID
    pub question_id: String,
    /// 问题标题
    pub title: String,
    /// 问题描述
    pub description: Option<String>,
    /// 是否必填
    pub required: bool,
    /// 是否可见
    pub visible: bool,
    /// 更新时间
    pub updated_at: String,
}

/// 更新表单问题响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionResponseData {
    /// 更新的表单问题信息
    pub question: PatchFormQuestionResponse,
}

/// 更新表单问题API响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionResponseBody {
    /// 响应数据
    pub data: PatchFormQuestionResponseData,
}

/// BaseResponse包装的更新表单问题响应
pub type PatchFormQuestionApiResponse = BaseResponse<PatchFormQuestionResponseBody>;

impl ApiResponseTrait for PatchFormQuestionApiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表单问题服务实现
impl FormService {
    /// 更新表单问题
    ///
    /// 更新指定多维表格表单中的问题配置
    ///
    /// # 参数
    ///
    /// * `app_token` - 多维表格的唯一标识符
    /// * `form_id` - 表单ID
    /// * `question_id` - 问题ID
    /// * `request` - 更新表单问题请求
    ///
    /// # 返回
    ///
    /// 返回更新后的表单问题信息
    pub async fn patch_form_question(
        &self,
        app_token: impl Into<String>,
        form_id: impl Into<String>,
        question_id: impl Into<String>,
        request: PatchFormQuestionRequest,
    ) -> SDKResult<PatchFormQuestionApiResponse> {
        let app_token = app_token.into();
        let form_id = form_id.into();
        let question_id = question_id.into();

        let api_request = ApiRequest::builder()
            .method(Method::PATCH)
            .url(&format!(
                "{}/open-apis/bitable/v1/apps/{}/forms/{}/questions/{}",
                self.config.base_url, app_token, form_id, question_id
            ))
            .header("Content-Type", "application/json")
            .body(request)
            .build()?;

        let response = self
            .transport
            .execute::<PatchFormQuestionApiResponse>(api_request)
            .await?;

        Ok(response)
    }

    /// 获取更新表单问题构建器
    pub fn patch_form_question_builder(&self) -> PatchFormQuestionRequestBuilder {
        PatchFormQuestionRequestBuilder::default()
    }
}

/// Builder模式实现
impl PatchFormQuestionRequest {
    /// 创建更新表单问题构建器
    pub fn builder() -> PatchFormQuestionRequestBuilder {
        PatchFormQuestionRequestBuilder::default()
    }
}