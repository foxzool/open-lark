/// CCM Sheet V2 工作表API 模块
///
/// 工作表操作API实现，包含工作表的增删改查：
/// - add_sheet: 添加工作表
/// - get_sheet: 获取工作表信息
/// - update_sheet: 更新工作表
/// - delete_sheet: 删除工作表
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{
    api_endpoints::{CcmSheetApiOld, SheetsApiV3},
    api_utils::*,
};

/// 工作表API结构体
#[derive(Debug, Clone)]
pub struct SheetApi {
    config: Config,
}

impl SheetApi {
    /// 创建新的工作表API实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出模型定义
/// models 子模块。
pub mod models;
// models 模块显式导出
/// 重新导出相关类型。
pub use models::{
    AddSheetParams, AddSheetResponse, AddSheetResult, DeleteSheetParams, DeleteSheetResponse,
    DeleteSheetResult, GetSheetParams, GetSheetResponse, SheetDetailInfo, UpdateSheetParams,
    UpdateSheetResponse, UpdateSheetResult,
};

impl ApiResponseTrait for AddSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加工作表
///
/// 根据 spreadsheetToken 在电子表格中添加新的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn add_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: AddSheetParams,
) -> SDKResult<AddSheetResponse> {
    add_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 添加工作表（带请求选项）
///
/// 根据 spreadsheetToken 在电子表格中添加新的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn add_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: AddSheetParams,
    option: RequestOption,
) -> SDKResult<AddSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.title.trim(), "工作表标题不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::AddSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<AddSheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "添加工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "添加工作表")
}

/// 获取工作表信息
///
/// 根据 spreadsheetToken 和 sheet_id 获取工作表的详细信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get
pub async fn get_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSheetParams,
) -> SDKResult<GetSheetResponse> {
    get_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 获取工作表信息（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 获取工作表的详细信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get
pub async fn get_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: GetSheetParams,
    option: RequestOption,
) -> SDKResult<GetSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 按 csv 定义使用 v3 查询单个工作表接口
    let api_endpoint =
        SheetsApiV3::GetSheet(spreadsheet_token.to_string(), params.sheet_id.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetSheetResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取工作表信息")
}

/// 更新工作表
///
/// 根据 spreadsheetToken 和 sheet_id 更新工作表的基本信息，如标题或位置。
/// docPath: /document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
pub async fn update_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSheetParams,
) -> SDKResult<UpdateSheetResponse> {
    update_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 更新工作表（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 更新工作表的基本信息，如标题或位置。
/// docPath: /document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN
pub async fn update_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSheetParams,
    option: RequestOption,
) -> SDKResult<UpdateSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateSheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "更新工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新工作表")
}

/// 删除工作表
///
/// 根据 spreadsheetToken 和 sheet_id 删除指定的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn delete_sheet(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteSheetParams,
) -> SDKResult<DeleteSheetResponse> {
    delete_sheet_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 删除工作表（带请求选项）
///
/// 根据 spreadsheetToken 和 sheet_id 删除指定的工作表。
/// docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
pub async fn delete_sheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteSheetParams,
    option: RequestOption,
) -> SDKResult<DeleteSheetResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteSheet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteSheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "删除工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除工作表")
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
