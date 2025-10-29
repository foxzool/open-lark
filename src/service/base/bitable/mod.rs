// bitable - 多维表格API服务
//
// 提供多维表格相关的完整功能

use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    standard_response::StandardResponse,
    SDKResult,
};
use open_lark_core::core::api_req::ApiRequest;
use crate::service::base::models::*;

/// 多维表格服务
#[derive(Debug, Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建多维表格
    pub async fn create_app(&self, request: CreateAppRequest) -> SDKResult<BitableApp> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path("/open-apis/bitable/v1/apps".to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let response: BaseResponse<BitableApp> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 复制多维表格
    pub async fn copy_app(&self, app_token: &str, request: CopyAppRequest) -> SDKResult<BitableApp> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/copy", app_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let response: BaseResponse<BitableApp> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 获取多维表格元数据
    pub async fn get_app(&self, app_token: &str) -> SDKResult<BitableApp> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}", app_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let response: BaseResponse<BitableApp> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 更新多维表格元数据
    pub async fn update_app(&self, app_token: &str, request: UpdateAppRequest) -> SDKResult<BitableApp> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}", app_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);
        api_req.http_method = reqwest::Method::PUT;

        let response: BaseResponse<BitableApp> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 新增一个数据表
    pub async fn create_table(&self, app_token: &str, request: CreateTableRequest) -> SDKResult<BitableTable> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables", app_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let response: BaseResponse<BitableTable> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 列出数据表
    pub async fn list_tables(&self, app_token: &str) -> SDKResult<Vec<BitableTable>> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables", app_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let response: BaseResponse<Vec<BitableTable>> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 新增记录
    pub async fn create_record(&self, app_token: &str, table_id: &str, request: CreateRecordRequest) -> SDKResult<BitableRecord> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", app_token, table_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let response: BaseResponse<BitableRecord> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 查询记录
    pub async fn search_records(&self, app_token: &str, table_id: &str, request: SearchRecordsRequest) -> SDKResult<PaginatedData<BitableRecord>> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/search", app_token, table_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let response: BaseResponse<PaginatedData<BitableRecord>> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 列出字段
    pub async fn list_fields(&self, app_token: &str, table_id: &str) -> SDKResult<Vec<FieldInfo>> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", app_token, table_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let response: BaseResponse<Vec<FieldInfo>> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    // 其他API方法可以根据需要逐步添加
    // 这里实现了最重要的9个核心API，覆盖主要功能场景

    /// 删除数据表
    pub async fn delete_table(&self, app_token: &str, table_id: &str) -> SDKResult<()> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.http_method = reqwest::Method::DELETE;

        let response: BaseResponse<()> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }

    /// 删除记录
    pub async fn delete_record(&self, app_token: &str, table_id: &str, record_id: &str) -> SDKResult<()> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.http_method = reqwest::Method::DELETE;

        let response: BaseResponse<()> = Transport::request(api_req, &self.config, None).await?;
        response.into_result()
    }
}