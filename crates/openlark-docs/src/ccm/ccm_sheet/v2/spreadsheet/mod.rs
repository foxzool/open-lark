/// CCM Sheet V2 表格基础API 模块
///
/// 表格基础操作API实现，包含表格的增删改查：
/// - get_spreadsheet: 获取表格信息
/// - create_spreadsheet: 创建表格
/// - update_spreadsheet: 更新表格
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 表格基础API结构体
#[derive(Debug, Clone)]
pub struct SpreadsheetApi {
    config: Config,
}

impl SpreadsheetApi {
    /// 创建新的表格基础API实例
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

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格信息
///
/// 根据 spreadsheetToken 获取表格的详细信息，包括工作表信息。
/// docPath: /document/server-docs/docs/sheets-v3/spreadsheet/get-spreadsheet
pub async fn get_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSpreadsheetParams,
) -> SDKResult<GetSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetSpreadsheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "获取表格信息")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取表格信息")
}

/// 创建表格
///
/// 创建新的电子表格，支持指定标题和文件夹位置。
/// docPath: /document/server-docs/docs/sheets-v3/spreadsheet/create-spreadsheet
pub async fn create_spreadsheet(
    config: &Config,
    params: CreateSpreadsheetParams,
) -> SDKResult<CreateSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(params.title.trim(), "表格标题不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::CreateSpreadsheet;

    // 创建API请求
    let api_request: ApiRequest<CreateSpreadsheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建表格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建表格")
}

/// 更新表格
///
/// 根据 spreadsheetToken 更新表格的基本信息，如标题。
/// docPath: /document/server-docs/docs/sheets-v3/spreadsheet/update-spreadsheet
pub async fn update_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSpreadsheetParams,
) -> SDKResult<UpdateSpreadsheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateSpreadsheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "更新表格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新表格")
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出
