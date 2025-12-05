pub mod list;
pub mod models;
pub mod patch;

use openlark_core::config::Config;

// 明确导出以避免模糊重导出
pub use list::{
    list_form_field_questions, FormFieldQuestion, ListFormFieldQuestionRequest,
    ListFormFieldQuestionRequestBuilder, ListFormFieldQuestionResponse,
};
pub use models::{PatchFormFieldRequest};
pub use patch::{
    PatchFormFieldQuestionV1Request, PatchFormFieldQuestionV1Response,
};

/// 表单字段服务
pub struct FormFieldService {
    pub config: Config,
}

impl FormFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出表单问题
    pub async fn list(
        &self,
        request: ListFormFieldQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListFormFieldQuestionResponse>> {
        list::list_form_field_questions(request, &self.config, option).await
    }
}
