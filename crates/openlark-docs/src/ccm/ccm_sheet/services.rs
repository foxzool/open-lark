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
        let mut query_params = HashMap::new();

        if let Some(include_format) = request.include_format {
            query_params.insert("include_format", include_format.to_string());
        }
        if let Some(ref render_option) = request.value_render_option {
            query_params.insert("valueRenderOption", render_option.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}/values/{}",
                request.spreadsheet_token, request.range
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<ReadSingleRangeResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "读取单个范围完成: spreadsheet_token={}, range={}",
            request.spreadsheet_token,
            request.range
        );

        Ok(response)
    }

    /// 读取多个范围
    ///
    /// 批量读取多个指定范围的数据
    ///
    /// # 参数
    /// * `request` - 读取多个范围请求
    ///
    /// # 返回
    /// 返回所有指定范围的数据
    pub async fn read_multiple_ranges(
        &self,
        request: &ReadMultipleRangesRequest,
    ) -> SDKResult<ReadMultipleRangesResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "读取多个范围: spreadsheet_token={}, range_count={}",
            request.spreadsheet_token,
            request.ranges.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("ranges", serde_json::to_value(&request.ranges)?);

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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<ReadMultipleRangesResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "读取多个范围完成: spreadsheet_token={}, range_count={}",
            request.spreadsheet_token,
            response.value_ranges.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 写入单个范围
    ///
    /// 向指定范围写入数据
    ///
    /// # 参数
    /// * `request` - 写入单个范围请求
    ///
    /// # 返回
    /// 返回写入操作的统计信息
    pub async fn write_single_range(
        &self,
        request: &WriteSingleRangeRequest,
    ) -> SDKResult<WriteSingleRangeResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "写入单个范围: spreadsheet_token={}, range={}, rows={}",
            request.spreadsheet_token,
            request.range,
            request.values.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("values", serde_json::to_value(&request.values)?);

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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<WriteSingleRangeResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "写入单个范围完成: spreadsheet_token={}, range={}, updated_cells={}",
            request.spreadsheet_token,
            request.range,
            response.updated_cells.unwrap_or(0)
        );

        Ok(response)
    }

    /// 写入多个范围
    ///
    /// 批量向多个范围写入数据
    ///
    /// # 参数
    /// * `request` - 写入多个范围请求
    ///
    /// # 返回
    /// 返回所有写入操作的统计信息
    pub async fn write_multiple_ranges(
        &self,
        request: &WriteMultipleRangesRequest,
    ) -> SDKResult<WriteMultipleRangesResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "写入多个范围: spreadsheet_token={}, data_count={}",
            request.spreadsheet_token,
            request.data.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("data", serde_json::to_value(&request.data)?);

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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<WriteMultipleRangesResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "写入多个范围完成: spreadsheet_token={}, reply_count={}",
            request.spreadsheet_token,
            response.replies.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 追加数据
    ///
    /// 在指定范围的末尾追加数据
    ///
    /// # 参数
    /// * `request` - 追加数据请求
    ///
    /// # 返回
    /// 返回追加操作的统计信息
    pub async fn append_data(&self, request: &AppendDataRequest) -> SDKResult<AppendDataResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "追加数据: spreadsheet_token={}, range={}, rows={}",
            request.spreadsheet_token,
            request.range,
            request.values.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("range", Value::String(request.range.clone()));
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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<AppendDataResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "追加数据完成: spreadsheet_token={}, range={}, updated_rows={}",
            request.spreadsheet_token,
            request.range,
            response
                .updates
                .as_ref()
                .and_then(|u| u.updated_rows)
                .unwrap_or(0)
        );

        Ok(response)
    }

    /// 插入行列
    ///
    /// 在指定位置插入空行或空列
    ///
    /// # 参数
    /// * `request` - 插入行列请求
    ///
    /// # 返回
    /// 返回插入操作的结果
    pub async fn insert_dimension(
        &self,
        request: &InsertDimensionRequest,
    ) -> SDKResult<InsertDimensionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "插入行列: spreadsheet_token={}, dimension={}, start_index={}, end_index={}",
            request.spreadsheet_token,
            request.dimension,
            request.start_index,
            request.end_index
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("dimension", Value::String(request.dimension.clone()));
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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<InsertDimensionResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "插入行列完成: spreadsheet_token={}, dimension={}",
            request.spreadsheet_token,
            request.dimension
        );

        Ok(response)
    }

    /// 删除行列
    ///
    /// 删除指定范围的行或列
    ///
    /// # 参数
    /// * `request` - 删除行列请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_dimension(
        &self,
        request: &DeleteDimensionRequest,
    ) -> SDKResult<DeleteDimensionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除行列: spreadsheet_token={}, dimension={}, start_index={}, end_index={}",
            request.spreadsheet_token,
            request.dimension,
            request.start_index,
            request.end_index
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("dimension", Value::String(request.dimension.clone()));
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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<DeleteDimensionResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除行列完成: spreadsheet_token={}, dimension={}",
            request.spreadsheet_token,
            request.dimension
        );

        Ok(response)
    }

    /// 批量更新工作表
    ///
    /// 批量执行工作表更新操作
    ///
    /// # 参数
    /// * `request` - 批量更新工作表请求
    ///
    /// # 返回
    /// 返回批量更新的结果
    pub async fn batch_update_sheet(
        &self,
        request: &BatchUpdateSheetRequest,
    ) -> SDKResult<BatchUpdateSheetResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量更新工作表: spreadsheet_token={}, request_count={}",
            request.spreadsheet_token,
            request.requests.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("requests", serde_json::to_value(&request.requests)?);

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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<BatchUpdateSheetResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量更新工作表完成: spreadsheet_token={}, reply_count={}",
            request.spreadsheet_token,
            response.replies.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 获取表格元数据
    ///
    /// 获取电子表格的详细元数据信息
    ///
    /// # 参数
    /// * `request` - 获取表格元数据请求
    ///
    /// # 返回
    /// 返回表格的元数据信息
    pub async fn get_sheet_meta(
        &self,
        request: &GetSheetMetaRequest,
    ) -> SDKResult<GetSheetMetaResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取表格元数据: spreadsheet_token={}",
            request.spreadsheet_token
        );

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(include_format) = request.include_format {
            query_params.insert("includeFormat", include_format.to_string());
        }
        if let Some(include_data_validation) = request.include_data_validation {
            query_params.insert("includeDataValidation", include_data_validation.to_string());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/sheets/v2/spreadsheets/{}",
                request.spreadsheet_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<GetSheetMetaResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取表格元数据完成: spreadsheet_token={}, sheet_count={}",
            request.spreadsheet_token,
            response.sheets.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 设置单元格样式
    ///
    /// 设置指定范围内单元格的样式
    ///
    /// # 参数
    /// * `request` - 设置单元格样式请求
    ///
    /// # 返回
    /// 返回样式设置的结果
    pub async fn set_cell_style(
        &self,
        request: &SetCellStyleRequest,
    ) -> SDKResult<SetCellStyleResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "设置单元格样式: spreadsheet_token={}, range={}",
            request.spreadsheet_token,
            request.range
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("range", Value::String(request.range.clone()));
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
            query_params: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<SetCellStyleResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "设置单元格样式完成: spreadsheet_token={}, range={}",
            request.spreadsheet_token,
            request.range
        );

        Ok(response)
    }

    /// 创建读取单个范围构建器
    pub fn read_single_range_builder(&self) -> ReadSingleRangeRequestBuilder {
        ReadSingleRangeRequestBuilder::new()
    }

    /// 创建写入单个范围构建器
    pub fn write_single_range_builder(&self) -> WriteSingleRangeRequestBuilder {
        WriteSingleRangeRequestBuilder::new()
    }

    /// 创建追加数据构建器
    pub fn append_data_builder(&self) -> AppendDataRequestBuilder {
        AppendDataRequestBuilder::new()
    }
}

// 构建器模式实现
pub struct ReadSingleRangeRequestBuilder {
    spreadsheet_token: Option<String>,
    range: Option<String>,
    include_format: Option<bool>,
    value_render_option: Option<String>,
}

impl ReadSingleRangeRequestBuilder {
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            range: None,
            include_format: None,
            value_render_option: None,
        }
    }

    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn include_format(mut self, include: bool) -> Self {
        self.include_format = Some(include);
        self
    }

    pub fn value_render_option(mut self, option: impl Into<String>) -> Self {
        self.value_render_option = Some(option.into());
        self
    }

    pub async fn execute(self, service: &CcmSheetService) -> SDKResult<ReadSingleRangeResponse> {
        let request = ReadSingleRangeRequest {
            spreadsheet_token: self.spreadsheet_token.ok_or_else(|| {
                LarkAPIError::illegal_param("spreadsheet_token is required".to_string())
            })?,
            range: self
                .range
                .ok_or_else(|| LarkAPIError::illegal_param("range is required".to_string()))?,
            include_format: self.include_format,
            value_render_option: self.value_render_option,
        };
        service.read_single_range(&request).await
    }
}

impl Default for ReadSingleRangeRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WriteSingleRangeRequestBuilder {
    spreadsheet_token: Option<String>,
    range: Option<String>,
    values: Option<Vec<Vec<serde_json::Value>>>,
    value_input_option: Option<String>,
}

impl WriteSingleRangeRequestBuilder {
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            range: None,
            values: None,
            value_input_option: None,
        }
    }

    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn values(mut self, values: Vec<Vec<serde_json::Value>>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn value_input_option(mut self, option: impl Into<String>) -> Self {
        self.value_input_option = Some(option.into());
        self
    }

    pub async fn execute(self, service: &CcmSheetService) -> SDKResult<WriteSingleRangeResponse> {
        let request = WriteSingleRangeRequest {
            spreadsheet_token: self.spreadsheet_token.ok_or_else(|| {
                LarkAPIError::illegal_param("spreadsheet_token is required".to_string())
            })?,
            range: self
                .range
                .ok_or_else(|| LarkAPIError::illegal_param("range is required".to_string()))?,
            values: self
                .values
                .ok_or_else(|| LarkAPIError::illegal_param("values is required".to_string()))?,
            value_input_option: self.value_input_option,
        };
        service.write_single_range(&request).await
    }
}

impl Default for WriteSingleRangeRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppendDataRequestBuilder {
    spreadsheet_token: Option<String>,
    range: Option<String>,
    values: Option<Vec<Vec<serde_json::Value>>>,
    value_input_option: Option<String>,
    insert_data_option: Option<String>,
}

impl AppendDataRequestBuilder {
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            range: None,
            values: None,
            value_input_option: None,
            insert_data_option: None,
        }
    }

    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn values(mut self, values: Vec<Vec<serde_json::Value>>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn value_input_option(mut self, option: impl Into<String>) -> Self {
        self.value_input_option = Some(option.into());
        self
    }

    pub fn insert_data_option(mut self, option: impl Into<String>) -> Self {
        self.insert_data_option = Some(option.into());
        self
    }

    pub async fn execute(self, service: &CcmSheetService) -> SDKResult<AppendDataResponse> {
        let request = AppendDataRequest {
            spreadsheet_token: self.spreadsheet_token.ok_or_else(|| {
                LarkAPIError::illegal_param("spreadsheet_token is required".to_string())
            })?,
            range: self
                .range
                .ok_or_else(|| LarkAPIError::illegal_param("range is required".to_string()))?,
            values: self
                .values
                .ok_or_else(|| LarkAPIError::illegal_param("values is required".to_string()))?,
            value_input_option: self.value_input_option,
            insert_data_option: self.insert_data_option,
        };
        service.append_data(&request).await
    }
}

impl Default for AppendDataRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
