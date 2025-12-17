pub mod list;
pub mod models;
pub mod patch;

use openlark_core::config::Config;
// 明确导出以避免模糊重导出
pub use list::{
    list_form_field_questions, FormFieldQuestion, ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
};
pub use models::PatchFormFieldRequest;

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
        _option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<
        crate::bitable::v1::app::table::form::field::list::ListFormFieldQuestionData,
    > {
        // 忽略 option 参数，因为它在请求结构体中已经处理了
        request.execute(&self.config).await
    }
}
