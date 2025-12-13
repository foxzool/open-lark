/// 保护范围 API
///
/// 保护范围 - 管理表格的保护范围设置
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 增加保护范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtectedRangeParams {
    /// 保护范围列表
    #[serde(rename = "protected_ranges")]
    pub protected_ranges: Vec<ProtectedRange>,
}

/// 保护范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRange {
    /// 范围信息
    #[serde(rename = "range")]
    pub range: String,
    /// 保护范围描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 密码保护
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 提醒邮箱
    #[serde(rename = "warning_only", skip_serializing_if = "Option::is_none")]
    pub warning_only: Option<bool>,
    /// 审核人列表
    #[serde(rename = "reviewers", skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<String>>,
}

/// 保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeResponse {
    /// 保护范围结果
    pub data: Option<ProtectedRangeResult>,
}

/// 保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 保护范围列表
    #[serde(rename = "protected_ranges")]
    pub protected_ranges: Vec<ProtectedRangeInfo>,
}

/// 保护范围信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeInfo {
    /// 保护范围ID
    #[serde(rename = "protected_range_id")]
    pub protected_range_id: String,
    /// 范围信息
    #[serde(rename = "range")]
    pub range: String,
    /// 创建者
    #[serde(rename = "creator")]
    pub creator: CreatorInfo,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
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

impl ApiResponseTrait for ProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加保护范围
///
/// 根据配置的保护范围参数为电子表格添加保护区域。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::protected_range::*;
///
/// let params = AddProtectedRangeParams {
///     protected_ranges: vec![
///         ProtectedRange {
///             range: "Sheet1!A1:C10".to_string(),
///             description: Some("重要数据区域".to_string()),
///             password: Some("password123".to_string()),
///             warning_only: Some(false),
///             reviewers: Some(vec!["user_id_1".to_string()]),
///         }
///     ],
/// };
///
/// let result = add_protected_range(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn add_protected_range(
    config: &Config,
    spreadsheet_token: &str,
    params: AddProtectedRangeParams,
) -> SDKResult<ProtectedRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    if params.protected_ranges.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "protected_ranges",
            "保护范围列表不能为空",
        ));
    }

    // 验证每个保护范围
    for (index, range) in params.protected_ranges.iter().enumerate() {
        if range.range.is_empty() {
            return Err(openlark_core::error::CoreError::validation(
                &format!("protected_ranges[{}].range", index),
                "保护范围不能为空",
            ));
        }

        // 验证密码长度
        if let Some(ref password) = range.password {
            if password.len() < 6 || password.len() > 255 {
                return Err(openlark_core::error::CoreError::validation(
                    &format!("protected_ranges[{}].password", index),
                    "密码长度必须在6-255个字符之间",
                ));
            }
        }
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ProtectedDimension(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<ProtectedRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "增加保护范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "增加保护范围")
}

/// 获取保护范围
///
/// 获取指定电子表格的所有保护范围信息。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::protected_range::*;
///
/// let result = get_protected_ranges(&config, "spreadsheet_token").await?;
/// ```
pub async fn get_protected_ranges(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<ProtectedRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchGet(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<ProtectedRangeResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取保护范围")
}

/// 删除保护范围
///
/// 删除指定电子表格的保护范围。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::protected_range::*;
///
/// let params = DeleteProtectedRangeParams {
///     protected_range_ids: vec!["range_id_1".to_string(), "range_id_2".to_string()],
/// };
///
/// let result = delete_protected_ranges(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn delete_protected_ranges(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteProtectedRangeParams,
) -> SDKResult<ProtectedRangeResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    if params.protected_range_ids.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "protected_range_ids",
            "保护范围ID列表不能为空",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchDel(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<ProtectedRangeResponse> =
        ApiRequest::delete(&api_endpoint.to_url()).body(serialize_params(&params, "删除保护范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除保护范围")
}

/// 删除保护范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangeParams {
    /// 保护范围ID列表
    #[serde(rename = "protected_range_ids")]
    pub protected_range_ids: Vec<String>,
}
