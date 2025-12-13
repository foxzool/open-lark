use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// Bitable 更新表单问题API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/form/patch
use serde::{Deserialize, Serialize};

/// 更新表单问题请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionRequest {
    /// 应用ID
    pub app_id: String,
    /// 表格ID
    pub table_id: String,
    /// 表单ID
    pub form_id: String,
    /// 字段ID
    pub field_id: String,
    /// 更新的字段信息
    pub field_info: FormFieldInfo,
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldInfo {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: bool,
}

/// 更新表单问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFormQuestionResponse {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
}

/// 表单问题请求构建器
#[derive(Debug, Clone)]
pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl PatchFormQuestionRequestBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            request: PatchFormQuestionRequest {
                app_id: String::new(),
                table_id: String::new(),
                form_id: String::new(),
                field_id: String::new(),
                field_info: FormFieldInfo {
                    field_name: String::new(),
                    field_type: String::new(),
                    required: false,
                },
            },
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置表格ID
    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    /// 设置表单ID
    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request.form_id = form_id.into();
        self
    }

    /// 设置字段ID
    pub fn field_id(mut self, field_id: impl Into<String>) -> Self {
        self.request.field_id = field_id.into();
        self
    }

    /// 设置字段信息
    pub fn field_info(mut self, field_info: FormFieldInfo) -> Self {
        self.request.field_info = field_info;
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchFormQuestionRequest {
        self.request
    }
}

impl Default for PatchFormQuestionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 更新表单问题
pub async fn patch_form_question(
    request: PatchFormQuestionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<PatchFormQuestionResponse>> {
    // TODO: 实现API调用
    unimplemented!("API implementation pending")
}
