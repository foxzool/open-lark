pub mod get;
pub mod list;
pub mod patch;
pub mod patch_meta;

use crate::core::config::Config;

pub use get::*;
pub use list::*;
pub use patch::*;
pub use patch_meta::*;

/// 表单服务
pub struct FormService {
    config: Config,
}

impl FormService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取表单元数据
    pub async fn get(
        &self,
        request: GetFormRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<GetFormResponse>> {
        get::get_form(request, &self.config, option).await
    }

    /// 列出表单问题
    pub async fn list(
        &self,
        request: ListFormQuestionRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListFormQuestionResponse>> {
        list::list_form_questions(request, &self.config, option).await
    }

    /// 更新表单问题
    pub async fn patch(
        &self,
        request: PatchFormQuestionRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<PatchFormQuestionResponse>>
    {
        patch::patch_form_question(request, &self.config, option).await
    }

    /// 更新表单元数据
    pub async fn patch_meta(
        &self,
        request: PatchFormMetaRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<PatchFormMetaResponse>> {
        patch_meta::patch_form_meta(request, &self.config, option).await
    }
}
