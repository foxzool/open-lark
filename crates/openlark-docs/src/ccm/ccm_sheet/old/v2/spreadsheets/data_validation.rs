/// 数据验证 API
///
/// 数据验证 - 管理表格的数据验证规则（如下拉列表）
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 创建数据验证规则请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataValidationParams {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: i32,
    /// 数据验证规则
    #[serde(rename = "data_validation")]
    pub data_validation: DataValidationRule,
}

/// 数据验证规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationRule {
    /// 验证范围
    #[serde(rename = "condition")]
    pub condition: ValidationCondition,
    /// 输入提示
    #[serde(rename = "input_message", skip_serializing_if = "Option::is_none")]
    pub input_message: Option<String>,
    /// 验证失败提示
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 是否严格模式
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// 验证条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    /// 条件类型
    #[serde(rename = "condition_type")]
    pub condition_type: String,
    /// 值范围
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<serde_json::Value>>,
    /// 内容类型
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// 数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationResponse {
    /// 数据验证结果
    pub data: Option<DataValidationResult>,
}

/// 数据验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 数据验证规则信息
    #[serde(rename = "data_validation")]
    pub data_validation: DataValidationInfo,
}

/// 数据验证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationInfo {
    /// 验证ID
    #[serde(rename = "data_validation_id")]
    pub data_validation_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: i32,
    /// 范围
    #[serde(rename = "ranges")]
    pub ranges: Vec<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 创建者
    #[serde(rename = "creator")]
    pub creator: CreatorInfo,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "name")]
    pub name: String,
}

impl ApiResponseTrait for DataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建数据验证规则
///
/// 为指定的工作表创建数据验证规则，支持下拉列表、数值范围、文本长度等验证类型。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::data_validation::*;
///
/// let params = CreateDataValidationParams {
///     sheet_id: 0,
///     data_validation: DataValidationRule {
///         condition: ValidationCondition {
///             condition_type: "DROPDOWN".to_string(),
///             values: Some(vec![
///                 serde_json::Value::String("选项1".to_string()),
///                 serde_json::Value::String("选项2".to_string()),
///                 serde_json::Value::String("选项3".to_string()),
///             ]),
///             content_type: None,
///         },
///         input_message: Some("请选择有效选项".to_string()),
///         error_message: Some("选择的选项无效".to_string()),
///         strict: Some(true),
///     },
/// };
///
/// let result = create_data_validation(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn create_data_validation(
    config: &Config,
    spreadsheet_token: &str,
    params: CreateDataValidationParams,
) -> SDKResult<DataValidationResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field("工作表ID", Some(&params.sheet_id.to_string()), "工作表ID不能为空")?;

    // 验证条件类型
    if ![
        "DROPDOWN",
        "NUMBER",
        "TEXT_LENGTH",
        "DATE",
        "CUSTOM_FORMULA",
    ]
    .contains(&params.data_validation.condition.condition_type.as_str())
    {
        return Err(openlark_core::error::CoreError::validation(
            "condition_type",
            "条件类型必须是 DROPDOWN、NUMBER、TEXT_LENGTH、DATE 或 CUSTOM_FORMULA 之一",
        ));
    }

    // 验证下拉列表选项
    if params.data_validation.condition.condition_type == "DROPDOWN" {
        if let Some(ref values) = params.data_validation.condition.values {
            if values.is_empty() {
                return Err(openlark_core::error::CoreError::validation(
                    "values",
                    "下拉列表必须提供选项值",
                ));
            }

            if values.len() > 100 {
                return Err(openlark_core::error::CoreError::validation(
                    "values",
                    "下拉列表选项不能超过100个",
                ));
            }
        } else {
            return Err(openlark_core::error::CoreError::validation(
                "values",
                "下拉列表必须提供选项值",
            ));
        }
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DataValidationCreate(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DataValidationResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(serialize_params(&params, "创建数据验证规则")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建数据验证规则")
}

/// 获取数据验证规则
///
/// 获取指定工作表的数据验证规则列表。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::data_validation::*;
///
/// let result = get_data_validations(&config, "spreadsheet_token").await?;
/// ```
pub async fn get_data_validations(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<DataValidationResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DataValidation(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DataValidationResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取数据验证规则")
}

/// 删除数据验证规则
///
/// 删除指定的数据验证规则。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::data_validation::*;
///
/// let result = delete_data_validation(&config, "spreadsheet_token", 0, "validation_id").await?;
/// ```
pub async fn delete_data_validation(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: i32,
    validation_id: &str,
) -> SDKResult<DataValidationResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field("验证ID", Some(validation_id), "验证ID不能为空")?;

    if sheet_id < 0 {
        return Err(openlark_core::error::CoreError::validation(
            "sheet_id",
            "工作表ID必须大于等于0",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DataValidationDelete(
        spreadsheet_token.to_string(),
        validation_id.to_string(),
    );

    // 创建API请求
    let api_request: ApiRequest<DataValidationResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除数据验证规则")
}
