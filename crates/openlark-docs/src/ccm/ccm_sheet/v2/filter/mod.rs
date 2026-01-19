/// CCM Sheet V2 筛选API 模块
///
/// 筛选功能API实现，包含筛选的增删改查：
/// - create_filter: 创建筛选
/// - get_filter: 获取筛选
/// - update_filter: 更新筛选
/// - delete_filter: 删除筛选
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 筛选功能API结构体
#[derive(Debug, Clone)]
pub struct FilterApi {
    config: Config,
}

impl FilterApi {
    /// 创建新的筛选功能API实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出模型定义
pub mod models;
pub use models::*;

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选
///
/// 根据 spreadsheetToken 和 range 在指定范围创建筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/create-filter
pub async fn create_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateFilterParams,
) -> SDKResult<CreateFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "筛选范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::CreateFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<CreateFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建筛选")
}

/// 获取筛选
///
/// 根据 spreadsheetToken 和 filter_id 获取筛选信息。
/// docPath: /document/server-docs/docs/sheets-v3/filter/get-filter
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: GetFilterParams,
) -> SDKResult<GetFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "筛选范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "获取筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选")
}

/// 更新筛选
///
/// 根据 spreadsheetToken 和 filter_id 更新筛选条件。
/// docPath: /document/server-docs/docs/sheets-v3/filter/update-filter
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateFilterParams,
) -> SDKResult<UpdateFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.filter_id.trim(), "筛选ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "更新筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新筛选")
}

/// 删除筛选
///
/// 根据 spreadsheetToken 和 filter_id 删除筛选。
/// docPath: /document/server-docs/docs/sheets-v3/filter/delete-filter
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteFilterParams,
) -> SDKResult<DeleteFilterResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.filter_id.trim(), "筛选ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFilter(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "删除筛选")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除筛选")
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出
