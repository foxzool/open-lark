pub mod field;
pub mod get;
pub mod list;
pub mod patch;

use openlark_core::config::Config;

pub use field::*;
pub use get::*;
pub use list::*;
pub use patch::*;

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

    /// 列出表单问题
    pub async fn list(
        &self,
        request: ListFormQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListFormQuestionResponse>> {
        list::list_form_questions(request, &self.config, option).await
    }

    /// 更新表单问题
    pub async fn patch(
        &self,
        request: PatchFormQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<PatchFormQuestionResponse>> {
        patch::patch_form_question(request, &self.config, option).await
    }
}
