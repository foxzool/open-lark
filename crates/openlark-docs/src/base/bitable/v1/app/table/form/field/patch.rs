/// Bitable 更新表单问题API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/form/field/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::list::FormFieldQuestion;
use super::models::PatchFormFieldRequest as PatchFormFieldRequestBody;
use super::FormFieldService;

/// 更新表单问题请求
#[allow(dead_code)]
pub struct PatchFormFieldQuestionRequest {
    api_request: ApiRequest<PatchFormFieldQuestionResponse>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 表格ID
    table_id: String,
    /// 表单ID
    form_id: String,
    /// 字段ID
    field_id: String,
    /// 问题标题
    title: Option<String>,
    /// 问题描述
    description: Option<String>,
    /// 是否必填
    required: Option<bool>,
    /// 是否可见
    visible: Option<bool>,
    /// 问题设置
    setting: Option<serde_json::Value>,
    /// 配置信息
    config: Config,
}

/// 更新表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldQuestionResponse {
    /// 问题信息
    pub data: FormFieldQuestion,
}

impl ApiResponseTrait for PatchFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchFormFieldQuestionRequest {
    /// 创建更新表单问题请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields/:field_id"),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            field_id: String::new(),
            title: None,
            description: None,
            required: None,
            visible: None,
            setting: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置表格ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置表单ID
    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    /// 设置字段ID
    pub fn field_id(mut self, field_id: String) -> Self {
        self.field_id = field_id;
        self
    }

    /// 设置问题标题
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置问题描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// 设置是否可见
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// 设置问题设置
    pub fn setting(mut self, setting: serde_json::Value) -> Self {
        self.setting = Some(setting);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchFormFieldQuestionResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "表格ID不能为空"));
        }
        if self.form_id.trim().is_empty() {
            return Err(validation_error("form_id", "表单ID不能为空"));
        }
        if self.field_id.trim().is_empty() {
            return Err(validation_error("field_id", "字段ID不能为空"));
        }

        // 构建请求体
        let request_body = PatchFormFieldRequestBody {
            title: self.title.clone(),
            description: self.description.clone(),
            required: self.required,
            visible: self.visible,
            setting: self.setting.clone(),
        };

        // 构建API路径
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields/{}",
            self.app_token, self.table_id, self.form_id, self.field_id
        );

        // 创建API请求
        let api_request: ApiRequest<PatchFormFieldQuestionResponse> = ApiRequest::post(&path).body(
            openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?),
        );

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 更新表单问题Builder
pub struct PatchFormFieldQuestionBuilder {
    request: PatchFormFieldQuestionRequest,
}

impl PatchFormFieldQuestionBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchFormFieldQuestionRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置表格ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置表单ID
    pub fn form_id(mut self, form_id: String) -> Self {
        self.request = self.request.form_id(form_id);
        self
    }

    /// 设置字段ID
    pub fn field_id(mut self, field_id: String) -> Self {
        self.request = self.request.field_id(field_id);
        self
    }

    /// 设置问题标题
    pub fn title(mut self, title: String) -> Self {
        self.request = self.request.title(title);
        self
    }

    /// 设置问题描述
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.request = self.request.required(required);
        self
    }

    /// 设置是否可见
    pub fn visible(mut self, visible: bool) -> Self {
        self.request = self.request.visible(visible);
        self
    }

    /// 设置问题设置
    pub fn setting(mut self, setting: serde_json::Value) -> Self {
        self.request = self.request.setting(setting);
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchFormFieldQuestionRequest {
        self.request
    }
}

impl FormFieldService {
    /// 创建更新表单问题请求构建器
    pub fn patch_form_field_question_builder(&self) -> PatchFormFieldQuestionBuilder {
        PatchFormFieldQuestionBuilder::new(self.config.clone())
    }

    /// 创建更新表单问题请求
    pub fn patch_form_field_question_v1(
        &self,
        app_token: String,
        table_id: String,
        form_id: String,
        field_id: String,
        title: Option<String>,
        description: Option<String>,
        required: Option<bool>,
        visible: Option<bool>,
        setting: Option<serde_json::Value>,
    ) -> PatchFormFieldQuestionRequest {
        let mut request = PatchFormFieldQuestionRequest::new(self.config.clone())
            .app_token(app_token)
            .table_id(table_id)
            .form_id(form_id)
            .field_id(field_id);

        if let Some(title) = title {
            request = request.title(title);
        }

        if let Some(description) = description {
            request = request.description(description);
        }

        if let Some(required) = required {
            request = request.required(required);
        }

        if let Some(visible) = visible {
            request = request.visible(visible);
        }

        if let Some(setting) = setting {
            request = request.setting(setting);
        }

        request
    }
}
