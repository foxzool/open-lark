pub mod field;
pub mod get;
pub mod models;
pub mod patch;

// field 模块显式导出
pub use field::{
    Form,
    FormFieldQuestion,
    GetFormRequest,
    GetFormResponse,
    ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
    PatchFormFieldQuestionBuilder,
    PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse,
    PatchFormFieldRequest,
    PatchFormRequest,
    PatchFormResponse,
    PatchedFormFieldQuestion,
    app_token,
    build,
    description,
    execute,
    execute_with_options,
    field_id,
    form_id,
    name,
    new,
    page_size,
    page_token,
    pre_field_id,
    required,
    shared,
    shared_limit,
    submit_limit_once,
    table_id,
    title,
    validate,
    visible,
};
// get 模块显式导出
pub use get::{
    Form,
    FormFieldQuestion,
    GetFormRequest,
    GetFormResponse,
    ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
    PatchFormFieldQuestionBuilder,
    PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse,
    PatchFormFieldRequest,
    PatchFormRequest,
    PatchFormResponse,
    PatchedFormFieldQuestion,
    app_token,
    build,
    description,
    execute,
    execute_with_options,
    field_id,
    form_id,
    name,
    new,
    page_size,
    page_token,
    pre_field_id,
    required,
    shared,
    shared_limit,
    submit_limit_once,
    table_id,
    title,
    validate,
    visible,
};
// models 模块显式导出
pub use models::{
    Form,
    FormFieldQuestion,
    GetFormRequest,
    GetFormResponse,
    ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
    PatchFormFieldQuestionBuilder,
    PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse,
    PatchFormFieldRequest,
    PatchFormRequest,
    PatchFormResponse,
    PatchedFormFieldQuestion,
    app_token,
    build,
    description,
    execute,
    execute_with_options,
    field_id,
    form_id,
    name,
    new,
    page_size,
    page_token,
    pre_field_id,
    required,
    shared,
    shared_limit,
    submit_limit_once,
    table_id,
    title,
    validate,
    visible,
};
use openlark_core::config::Config;
// patch 模块显式导出
pub use patch::{
    Form,
    FormFieldQuestion,
    GetFormRequest,
    GetFormResponse,
    ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
    PatchFormFieldQuestionBuilder,
    PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse,
    PatchFormFieldRequest,
    PatchFormRequest,
    PatchFormResponse,
    PatchedFormFieldQuestion,
    app_token,
    build,
    description,
    execute,
    execute_with_options,
    field_id,
    form_id,
    name,
    new,
    page_size,
    page_token,
    pre_field_id,
    required,
    shared,
    shared_limit,
    submit_limit_once,
    table_id,
    title,
    validate,
    visible,
};

/// 表单服务
pub struct FormService {
    pub config: Config,
}

impl FormService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取表单元数据
    pub async fn get(&self, request: GetFormRequest) -> openlark_core::SDKResult<GetFormResponse> {
        request.execute().await
    }

    /// 更新表单元数据
    pub async fn patch(
        &self,
        request: PatchFormRequest,
    ) -> openlark_core::SDKResult<PatchFormResponse> {
        request.execute().await
    }
}
