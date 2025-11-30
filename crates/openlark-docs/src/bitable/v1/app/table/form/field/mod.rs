#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod list;
pub mod update;

use openlark_core::config::Config;

pub use list::*;
pub use update::*;

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

    /// 更新表单问题
    pub async fn update(
        &self,
        request: UpdateFormFieldQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<UpdateFormFieldQuestionResponse>> {
        update::update_form_field_question(request, &self.config, option).await
    }
}