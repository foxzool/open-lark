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
    impl_executable_builder_config,
};

/// 删除表单问题请求
#[derive(Clone)]
pub struct DeleteFormQuestionRequest {
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
}

impl DeleteFormQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder"),
            app_token: String::new(),
            form_id: String::new(),
            question_id: String::new(),
        }
    }

    pub fn builder() -> DeleteFormQuestionRequestBuilder {
        DeleteFormQuestionRequestBuilder::default()
    }
}

/// 删除表单问题请求构建器
#[derive(Default, Clone)]
pub struct DeleteFormQuestionRequestBuilder {
    request: DeleteFormQuestionRequest,
}

impl DeleteFormQuestionRequestBuilder {
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

    /// 构建请求对象
    pub fn build(self) -> DeleteFormQuestionRequest {
        self.request
    }
}

/// 删除表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFormQuestionResponse {
    /// 问题ID
    pub question_id: String,
    /// 删除状态
    pub deleted: bool,
    /// 删除时间
    pub deleted_at: String,
}

/// 删除表单问题响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFormQuestionResponseData {
    /// 删除的表单问题信息
    pub question: DeleteFormQuestionResponse,
}

/// 删除表单问题API响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFormQuestionResponseBody {
    /// 响应数据
    pub data: DeleteFormQuestionResponseData,
}

/// BaseResponse包装的删除表单问题响应
pub type DeleteFormQuestionApiResponse = BaseResponse<DeleteFormQuestionResponseBody>;

impl ApiResponseTrait for DeleteFormQuestionApiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除表单问题服务实现
impl FormService {
    /// 删除表单问题
    ///
    /// 删除指定多维表格表单中的问题
    ///
    /// # 参数
    ///
    /// * `app_token` - 多维表格的唯一标识符
    /// * `form_id` - 表单ID
    /// * `question_id` - 问题ID
    ///
    /// # 返回
    ///
    /// 返回删除操作的结果
    pub async fn delete_form_question(
        &self,
        app_token: impl Into<String>,
        form_id: impl Into<String>,
        question_id: impl Into<String>,
    ) -> SDKResult<DeleteFormQuestionApiResponse> {
        let app_token = app_token.into();
        let form_id = form_id.into();
        let question_id = question_id.into();

        let api_request = ApiRequest::builder()
            .method(Method::DELETE)
            .url(&format!(
                "{}/open-apis/bitable/v1/apps/{}/forms/{}/questions/{}",
                self.config.base_url, app_token, form_id, question_id
            ))
            .build()?;

        let response = self
            .transport
            .execute::<DeleteFormQuestionApiResponse>(api_request)
            .await?;

        Ok(response)
    }

    /// 获取删除表单问题构建器
    pub fn delete_form_question_builder(&self) -> DeleteFormQuestionRequestBuilder {
        DeleteFormQuestionRequestBuilder::default()
    }
}

/// Builder模式实现
impl DeleteFormQuestionRequest {
    /// 创建删除表单问题构建器
    pub fn builder() -> DeleteFormQuestionRequestBuilder {
        DeleteFormQuestionRequestBuilder::default()
    }
}