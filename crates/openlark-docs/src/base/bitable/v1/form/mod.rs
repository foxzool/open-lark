
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
pub mod get;
pub mod list;
pub mod patch;
pub mod patch_meta;
use openlark_core::config::Config;
use SDKResult;pub use get::*;
pub use list::*;
pub use patch::*;
pub use patch_meta::*;
/// 表单服务
pub struct FormService {
}

impl FormService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取表单元数据
    pub async fn get(
        &self,
        request: GetFormRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<GetFormResponse>> {
        get::get_form(request, &self.config, option).await,
/// 列出表单问题
    pub async fn list(
        &self,
        request: ListFormQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListFormQuestionResponse>> {
        list::list_form_questions(request, &self.config, option).await,
/// 更新表单问题
    pub async fn patch(
        &self,
        request: PatchFormQuestionRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<PatchFormQuestionResponse>>,
{,
        patch::patch_form_question(request, &self.config, option).await,
/// 更新表单元数据
    pub async fn patch_meta(
        &self,
        request: PatchFormMetaRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<PatchFormMetaResponse>> {
        patch_meta::patch_form_meta(request, &self.config, option).await,
}
}}}