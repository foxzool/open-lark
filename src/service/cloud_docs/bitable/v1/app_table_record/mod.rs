pub mod batch_create;
pub mod batch_delete;
pub mod batch_get;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod search;
pub mod update;

use crate::core::config::Config;

pub use batch_create::*;
pub use batch_delete::*;
pub use batch_get::*;
pub use batch_update::*;
pub use create::*;
pub use delete::*;
pub use search::*;
pub use update::*;

/// 记录服务
pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增记录
    pub async fn create(
        &self,
        request: &CreateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateRecordResponse>> {
        create::create_record(request.clone(), &self.config, option).await
    }

    /// 更新记录
    pub async fn update(
        &self,
        request: &UpdateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateRecordResponse>> {
        update::update_record(request.clone(), &self.config, option).await
    }

    /// 查询记录
    pub async fn search(
        &self,
        request: &SearchRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<SearchRecordResponse>> {
        search::search_record(request.clone(), &self.config, option).await
    }

    /// 删除记录
    pub async fn delete(
        &self,
        request: &DeleteRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteRecordResponse>> {
        delete::delete_record(request.clone(), &self.config, option).await
    }

    /// 新增多条记录
    pub async fn batch_create(
        &self,
        request: &BatchCreateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchCreateRecordResponse>>
    {
        batch_create::batch_create_record(request.clone(), &self.config, option).await
    }

    /// 更新多条记录
    pub async fn batch_update(
        &self,
        request: &BatchUpdateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchUpdateRecordResponse>>
    {
        batch_update::batch_update_record(request.clone(), &self.config, option).await
    }

    /// 批量获取记录
    pub async fn batch_get(
        &self,
        request: &BatchGetRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchGetRecordResponse>> {
        batch_get::batch_get_record(request.clone(), &self.config, option).await
    }

    /// 删除多条记录
    pub async fn batch_delete(
        &self,
        request: &BatchDeleteRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchDeleteRecordResponse>>
    {
        batch_delete::batch_delete_record(request.clone(), &self.config, option).await
    }
}
