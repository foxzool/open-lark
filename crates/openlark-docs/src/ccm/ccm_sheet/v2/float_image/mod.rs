/// CCM Sheet V2 浮图API 模块
///
/// 浮图功能API实现，包含浮图的增删改查：
/// - create_float_image: 创建浮图
/// - get_float_image: 获取浮图
/// - update_float_image: 更新浮图
/// - delete_float_image: 删除浮图
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 浮图功能API结构体
#[derive(Debug, Clone)]
pub struct FloatImageApi {
    config: Config,
}

impl FloatImageApi {
    /// 创建新的浮图功能API实例
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

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮图
///
/// 根据 spreadsheetToken 创建浮图，在工作表中插入浮动图片。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/create-float-image
pub async fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateFloatImageParams,
) -> SDKResult<CreateFloatImageResponse> {
    create_float_image_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 创建浮图（带选项）
///
/// 根据 spreadsheetToken 创建浮图，在工作表中插入浮动图片。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/create-float-image
pub async fn create_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateFloatImageParams,
    option: RequestOption,
) -> SDKResult<CreateFloatImageResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.image_token.trim(), "图片Token不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint =
        CcmSheetApiOld::CreateFloatImage(spreadsheet_token.to_string(), params.sheet_id.clone());

    // 创建API请求
    let api_request: ApiRequest<CreateFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建浮图")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "创建浮图")
}

/// 获取浮图
///
/// 根据 spreadsheetToken 和 float_image_id 获取浮图信息。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/get-float-image
pub async fn get_float_image(
    config: &Config,
    spreadsheet_token: &str,
    params: GetFloatImageParams,
) -> SDKResult<GetFloatImageResponse> {
    get_float_image_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 获取浮图（带选项）
///
/// 根据 spreadsheetToken 和 float_image_id 获取浮图信息。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/get-float-image
pub async fn get_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: GetFloatImageParams,
    option: RequestOption,
) -> SDKResult<GetFloatImageResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.float_image_id.trim(), "浮图ID不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFloatImage(
        spreadsheet_token.to_string(),
        params.sheet_id.clone(),
        params.float_image_id.clone(),
    );

    // 创建API请求
    let api_request: ApiRequest<GetFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "获取浮图")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取浮图")
}

/// 更新浮图
///
/// 根据 spreadsheetToken 和 float_image_id 更新浮图位置和属性。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/update-float-image
pub async fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateFloatImageParams,
) -> SDKResult<UpdateFloatImageResponse> {
    update_float_image_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 更新浮图（带选项）
///
/// 根据 spreadsheetToken 和 float_image_id 更新浮图位置和属性。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/update-float-image
pub async fn update_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateFloatImageParams,
    option: RequestOption,
) -> SDKResult<UpdateFloatImageResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.float_image_id.trim(), "浮图ID不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFloatImage(
        spreadsheet_token.to_string(),
        params.sheet_id.clone(),
        params.float_image_id.clone(),
    );

    // 创建API请求
    let api_request: ApiRequest<UpdateFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "更新浮图")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新浮图")
}

/// 删除浮图
///
/// 根据 spreadsheetToken 和 float_image_id 删除浮图。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/delete-float-image
pub async fn delete_float_image(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteFloatImageParams,
) -> SDKResult<DeleteFloatImageResponse> {
    delete_float_image_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 删除浮图（带选项）
///
/// 根据 spreadsheetToken 和 float_image_id 删除浮图。
/// docPath: /document/server-docs/docs/sheets-v3/float-image/delete-float-image
pub async fn delete_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteFloatImageParams,
    option: RequestOption,
) -> SDKResult<DeleteFloatImageResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.float_image_id.trim(), "浮图ID不能为空");
    validate_required!(params.sheet_id.trim(), "工作表ID不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFloatImage(
        spreadsheet_token.to_string(),
        params.sheet_id.clone(),
        params.float_image_id.clone(),
    );

    // 创建API请求
    let api_request: ApiRequest<DeleteFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "删除浮图")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除浮图")
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出
