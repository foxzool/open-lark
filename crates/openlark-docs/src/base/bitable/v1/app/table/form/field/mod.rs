pub mod list;
pub mod models;
pub mod patch;

use openlark_core::config::Config;
// 明确导出以避免模糊重导出
pub use list::{FormFieldQuestion, ListFormFieldQuestionRequest, ListFormFieldQuestionResponse};
pub use models::PatchFormFieldRequest;
pub use patch::{
    PatchFormFieldQuestionBuilder, PatchFormFieldQuestionRequest, PatchFormFieldQuestionResponse,
    PatchedFormFieldQuestion,
};

/// 表单字段服务
pub struct FormFieldService {
    config: Config,
}

impl FormFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn list(&self) -> ListFormFieldQuestionRequest {
        ListFormFieldQuestionRequest::new(self.config.clone())
    }

    pub fn patch(&self) -> PatchFormFieldQuestionBuilder {
        PatchFormFieldQuestionBuilder::new(self.config.clone())
    }
}
