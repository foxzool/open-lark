//! CCM Sheet API 服务实现
//!
//! 提供电子表格操作相关的API服务，包括：
//! - 单元格读写操作
//! - 批量数据处理
//! - 工作表管理
//! - 样式设置
//! - 维度操作（行列增删）
//! - 表格元数据管理
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// CCM Sheet API服务
#[derive(Debug, Clone)]
pub struct CcmSheetService {
    config: Config,
}

impl CcmSheetService {
    /// 创建新的CCM Sheet服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 读取单个范围
    ///
    /// 读取指定范围的单元格数据
    ///
    /// # 参数
    /// * `request` - 读取单个范围请求
    ///
    /// # 返回
    /// 返回读取范围内的数据
    ///
    /// # 示例
    /// ```rust
    /// let request = ReadSingleRangeRequest {
    ///     spreadsheet_token: "token_xxx".to_string(),
    ///     range: "Sheet1!A1:C10".to_string(),
    ///     include_format: Some(true),
    ///     value_render_option: Some("FORMATTED_VALUE".to_string()),
    /// };
    ///
    /// let response = service.read_single_range(&request).await?;
    /// if let Some(value_range) = response.value_range {
    ///     println!("读取到 {} 行 {} 列数据",
    ///              value_range.values.as_ref().map(|v| v.len()).unwrap_or(0),
    ///              value_range.values.as_ref().and_then(|v| v.first()).map(|r| r.len()).unwrap_or(0));
    /// }
    /// ```
    pub async fn read_single_range(
        &self,
        request: &ReadSingleRangeRequest,
    ) -> SDKResult<ReadSingleRangeResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "读取单个范围: spreadsheet_token={}, range={}",
            request.spreadsheet_token,
            request.range
        );

        // 构建查询参数
        }
        if let Some(ref render_option) = request.value_render_option {

        if let Some(include_format) = request.include_format {
            body.insert("include_format", Value::Bool(include_format));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/values:batchGet",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,

        if let Some(ref input_option) = request.value_input_option {
            body.insert("valueInputOption", Value::String(input_option.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/values/{}",
                request.spreadsheet_token, request.range
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,

        if let Some(ref input_option) = request.value_input_option {
            body.insert("valueInputOption", Value::String(input_option.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/values:batchUpdate",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
        body.insert("values", serde_json::to_value(&request.values)?);

        if let Some(ref input_option) = request.value_input_option {
            body.insert("valueInputOption", Value::String(input_option.clone()));
        }
        if let Some(ref insert_option) = request.insert_data_option {
            body.insert("insertDataOption", Value::String(insert_option.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/values:append",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
        body.insert("startIndex", Value::Number(request.start_index.into()));
        body.insert("endIndex", Value::Number(request.end_index.into()));

        if let Some(inherit_style) = request.inherit_style_before {
            body.insert("inheritStyleBefore", Value::Bool(inherit_style));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension:insert",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
        body.insert("startIndex", Value::Number(request.start_index.into()));
        body.insert("endIndex", Value::Number(request.end_index.into()));

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension:delete",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,

        if let Some(include_format) = request.include_format {
            body.insert("includeFormat", Value::Bool(include_format));
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/sheets:batchUpdate",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
        }
        if let Some(include_data_validation) = request.include_data_validation {
        body.insert("style", serde_json::to_value(&request.style)?);

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/style:set",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
