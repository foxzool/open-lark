//! 行列操作服务
//!
//! 提供飞书电子表格v2版本的行列操作功能，包括：
//! - 插入行或列
//! - 删除行或列
//! - 增加行列范围
//! - 更新行列属性
//! - 移动行列位置

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    ApiRequest, SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 行列维度类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Dimension {
    /// 行
    Rows,
    /// 列
    Columns,
}

/// 插入行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 维度类型 (行或列)
    pub dimension: Dimension,
    /// 起始索引（从0开始）
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引（不包含）
    #[serde(rename = "endIndex")]
    pub end_index: i32,
    /// 是否继承属性
    #[serde(rename = "inheritStyle", skip_serializing_if = "Option::is_none")]
    pub inherit_style: Option<bool>,
}

impl InsertDimensionRangeRequest {
    /// 创建新的插入行列请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        dimension: Dimension,
        start_index: i32,
        end_index: i32,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            dimension,
            start_index,
            end_index,
            inherit_style: None,
        }
    }

    /// 设置是否继承属性
    pub fn inherit_style(mut self, inherit_style: bool) -> Self {
        self.inherit_style = Some(inherit_style);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.start_index < 0 {
            return Err("起始索引不能小于0".to_string());
        }

        if self.end_index <= self.start_index {
            return Err("结束索引必须大于起始索引".to_string());
        }

        // 检查索引范围合理性 (最多1000行/列)
        if (self.end_index - self.start_index) > 1000 {
            return Err("单次插入行列数量不能超过1000".to_string());
        }

        Ok(())
    }
}

/// 插入行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeResponse {
    /// 响应数据
    pub data: InsertDimensionRangeData,
}

/// 插入行列数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 插入的维度信息
    pub dimension: Dimension,
    /// 起始索引
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引
    #[serde(rename = "endIndex")]
    pub end_index: i32,
    /// 是否继承属性
    #[serde(rename = "inheritStyle")]
    pub inherit_style: bool,
}

impl Default for InsertDimensionRangeResponse {
    fn default() -> Self {
        Self {
            data: InsertDimensionRangeData {
                spreadsheet_token: String::new(),
                dimension: Dimension::Rows,
                start_index: 0,
                end_index: 0,
                inherit_style: false,
            },
        }
    }
}

impl ApiResponseTrait for InsertDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 维度类型 (行或列)
    pub dimension: Dimension,
    /// 起始索引（从0开始）
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引（不包含）
    #[serde(rename = "endIndex")]
    pub end_index: i32,
}

impl DeleteDimensionRangeRequest {
    /// 创建新的删除行列请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        dimension: Dimension,
        start_index: i32,
        end_index: i32,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            dimension,
            start_index,
            end_index,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.start_index < 0 {
            return Err("起始索引不能小于0".to_string());
        }

        if self.end_index <= self.start_index {
            return Err("结束索引必须大于起始索引".to_string());
        }

        // 检查索引范围合理性 (最多1000行/列)
        if (self.end_index - self.start_index) > 1000 {
            return Err("单次删除行列数量不能超过1000".to_string());
        }

        Ok(())
    }
}

/// 删除行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeResponse {
    /// 响应数据
    pub data: DeleteDimensionRangeData,
}

/// 删除行列数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDimensionRangeData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 删除的维度信息
    pub dimension: Dimension,
    /// 起始索引
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    /// 结束索引
    #[serde(rename = "endIndex")]
    pub end_index: i32,
}

impl Default for DeleteDimensionRangeResponse {
    fn default() -> Self {
        Self {
            data: DeleteDimensionRangeData {
                spreadsheet_token: String::new(),
                dimension: Dimension::Rows,
                start_index: 0,
                end_index: 0,
            },
        }
    }
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 行列操作服务
#[derive(Debug, Clone)]
pub struct DimensionOperationsService {
    config: Config,
}

impl DimensionOperationsService {
    /// 创建行列操作服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 插入行列
    ///
    /// 在指定位置插入新的行或列。
    ///
    /// # 参数
    /// - `request`: 插入行列请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// // 在第2行位置插入3行
    /// let request = InsertDimensionRangeRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     "sheet1",
    ///     Dimension::Rows,
    ///     1, // 第2行（索引从0开始）
    ///     4  // 插入到第4行之前（不包含第4行）
    /// ).inherit_style(true);
    ///
    /// let response = service.insert_dimension_range(request, None).await?;
    /// println!("插入了 {} 行", response.data.end_index - response.data.start_index);
    /// ```
    pub async fn insert_dimension_range(
        &self,
        request: InsertDimensionRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<InsertDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/insert_dimension_range",
                &request.spreadsheet_token
            )
        );
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);
        api_req.body = serde_json::to_vec(&request)?;

        // 发送请求
        let api_resp = Transport::<InsertDimensionRangeResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 删除行列
    ///
    /// 删除指定范围的行或列。
    ///
    /// # 参数
    /// - `request`: 删除行列请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// // 删除第3-5行
    /// let request = DeleteDimensionRangeRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     "sheet1",
    ///     Dimension::Rows,
    ///     2, // 第3行（索引从0开始）
    ///     5  // 删除到第5行之前（不包含第5行）
    /// );
    ///
    /// let response = service.delete_dimension_range(request, None).await?;
    /// println!("删除了 {} 行", response.data.end_index - response.data.start_index);
    /// ```
    pub async fn delete_dimension_range(
        &self,
        request: DeleteDimensionRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::DELETE,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
                &request.spreadsheet_token
            )
        );
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);
        api_req.body = serde_json::to_vec(&request)?;

        // 发送请求
        let api_resp = Transport::<DeleteDimensionRangeResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 创建插入行构建器
    pub fn insert_rows_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> InsertDimensionRangeBuilder {
        InsertDimensionRangeBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Rows,
            start_index,
            count,
        )
    }

    /// 创建插入列构建器
    pub fn insert_columns_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> InsertDimensionRangeBuilder {
        InsertDimensionRangeBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Columns,
            start_index,
            count,
        )
    }

    /// 创建删除行构建器
    pub fn delete_rows_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> DeleteDimensionRangeBuilder {
        DeleteDimensionRangeBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Rows,
            start_index,
            count,
        )
    }

    /// 创建删除列构建器
    pub fn delete_columns_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> DeleteDimensionRangeBuilder {
        DeleteDimensionRangeBuilder::new(
            self.config.clone(),
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Columns,
            start_index,
            count,
        )
    }
}

/// 插入行列构建器
#[derive(Debug, Clone)]
pub struct InsertDimensionRangeBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    dimension: Dimension,
    start_index: i32,
    end_index: i32,
    inherit_style: bool,
}

impl InsertDimensionRangeBuilder {
    /// 创建新的插入行列构建器实例
    pub fn new(
        config: Config,
        spreadsheet_token: String,
        sheet_id: String,
        dimension: Dimension,
        start_index: i32,
        count: i32,
    ) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            dimension,
            start_index,
            end_index: start_index + count,
            inherit_style: false,
        }
    }

    /// 设置是否继承属性
    pub fn inherit_style(mut self, inherit_style: bool) -> Self {
        self.inherit_style = inherit_style;
        self
    }

    /// 设置插入范围
    pub fn range(mut self, start_index: i32, end_index: i32) -> Self {
        self.start_index = start_index;
        self.end_index = end_index;
        self
    }

    /// 执行插入请求
    pub async fn execute(self) -> SDKResult<BaseResponse<InsertDimensionRangeResponse>> {
        let request = InsertDimensionRangeRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.dimension,
            self.start_index,
            self.end_index,
        ).inherit_style(self.inherit_style);

        let service = DimensionOperationsService {
            config: self.config,
        };
        service.insert_dimension_range(request, None).await
    }
}

/// 删除行列构建器
#[derive(Debug, Clone)]
pub struct DeleteDimensionRangeBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    dimension: Dimension,
    start_index: i32,
    end_index: i32,
}

impl DeleteDimensionRangeBuilder {
    /// 创建新的删除行列构建器实例
    pub fn new(
        config: Config,
        spreadsheet_token: String,
        sheet_id: String,
        dimension: Dimension,
        start_index: i32,
        count: i32,
    ) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            dimension,
            start_index,
            end_index: start_index + count,
        }
    }

    /// 设置删除范围
    pub fn range(mut self, start_index: i32, end_index: i32) -> Self {
        self.start_index = start_index;
        self.end_index = end_index;
        self
    }

    /// 执行删除请求
    pub async fn execute(self) -> SDKResult<BaseResponse<DeleteDimensionRangeResponse>> {
        let request = DeleteDimensionRangeRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.dimension,
            self.start_index,
            self.end_index,
        );

        let service = DimensionOperationsService {
            config: self.config,
        };
        service.delete_dimension_range(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_dimension_range_request_creation() {
        let request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Rows,
            1,
            4,
        ).inherit_style(true);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.dimension, Dimension::Rows);
        assert_eq!(request.start_index, 1);
        assert_eq!(request.end_index, 4);
        assert_eq!(request.inherit_style, Some(true));
    }

    #[test]
    fn test_insert_dimension_range_request_validation() {
        // 测试正常请求
        let valid_request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Columns,
            0,
            5,
        );
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_token_request = InsertDimensionRangeRequest::new(
            "",
            "sheet1",
            Dimension::Rows,
            1,
            3,
        );
        assert!(empty_token_request.validate().is_err());

        // 测试负数索引
        let negative_index_request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Rows,
            -1,
            3,
        );
        assert!(negative_index_request.validate().is_err());

        // 测试结束索引小于起始索引
        let invalid_range_request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Rows,
            5,
            3,
        );
        assert!(invalid_range_request.validate().is_err());

        // 测试范围过大
        let large_range_request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Rows,
            0,
            1001,
        );
        assert!(large_range_request.validate().is_err());
    }

    #[test]
    fn test_delete_dimension_range_request() {
        let request = DeleteDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Columns,
            2,
            5,
        );

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.dimension, Dimension::Columns);
        assert_eq!(request.start_index, 2);
        assert_eq!(request.end_index, 5);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_dimension_serialization() {
        let rows = Dimension::Rows;
        let serialized = serde_json::to_string(&rows).unwrap();
        assert_eq!(serialized, "\"ROWS\"");

        let columns = Dimension::Columns;
        let serialized = serde_json::to_string(&columns).unwrap();
        assert_eq!(serialized, "\"COLUMNS\"");
    }

    #[test]
    fn test_insert_dimension_range_builder() {
        let config = Config::default();
        let builder = InsertDimensionRangeBuilder::new(
            config.clone(),
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Rows,
            1,
            3,
        ).inherit_style(true);

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.dimension, Dimension::Rows);
        assert_eq!(builder.start_index, 1);
        assert_eq!(builder.end_index, 4);
        assert_eq!(builder.inherit_style, true);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_delete_dimension_range_builder() {
        let config = Config::default();
        let builder = DeleteDimensionRangeBuilder::new(
            config.clone(),
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Columns,
            0,
            2,
        ).range(5, 8);

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.dimension, Dimension::Columns);
        assert_eq!(builder.start_index, 5);
        assert_eq!(builder.end_index, 8);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_dimension_operations_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DimensionOperationsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_response_default() {
        let insert_response = InsertDimensionRangeResponse::default();
        assert!(insert_response.data.spreadsheet_token.is_empty());
        assert_eq!(insert_response.data.dimension, Dimension::Rows);

        let delete_response = DeleteDimensionRangeResponse::default();
        assert!(delete_response.data.spreadsheet_token.is_empty());
        assert_eq!(delete_response.data.dimension, Dimension::Rows);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            InsertDimensionRangeResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteDimensionRangeResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_request = InsertDimensionRangeRequest::new(
            "test_token",
            "sheet1",
            Dimension::Columns,
            2,
            5,
        ).inherit_style(true);

        let serialized = serde_json::to_string(&original_request).unwrap();
        let deserialized: InsertDimensionRangeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_request.spreadsheet_token, deserialized.spreadsheet_token);
        assert_eq!(original_request.sheet_id, deserialized.sheet_id);
        assert_eq!(original_request.dimension, deserialized.dimension);
        assert_eq!(original_request.start_index, deserialized.start_index);
        assert_eq!(original_request.end_index, deserialized.end_index);
        assert_eq!(original_request.inherit_style, deserialized.inherit_style);
    }

    #[test]
    fn test_builder_methods() {
        let config = Config::default();

        // 测试插入构建器方法
        let insert_builder = DimensionOperationsService::new(config.clone())
            .insert_rows_builder("token".to_string(), "sheet".to_string(), 1, 3)
            .inherit_style(true);
        assert_eq!(insert_builder.inherit_style, true);

        let insert_columns_builder = DimensionOperationsService::new(config.clone())
            .insert_columns_builder("token".to_string(), "sheet".to_string(), 0, 2);
        assert_eq!(insert_columns_builder.dimension, Dimension::Columns);

        // 测试删除构建器方法
        let delete_rows_builder = DimensionOperationsService::new(config.clone())
            .delete_rows_builder("token".to_string(), "sheet".to_string(), 5, 2)
            .range(3, 6);
        assert_eq!(delete_rows_builder.start_index, 3);
        assert_eq!(delete_rows_builder.end_index, 6);

        let delete_columns_builder = DimensionOperationsService::new(config)
            .delete_columns_builder("token".to_string(), "sheet".to_string(), 1, 1);
        assert_eq!(delete_columns_builder.dimension, Dimension::Columns);
    }

    #[test]
    fn test_boundary_values() {
        // 测试边界值
        let min_insert = InsertDimensionRangeRequest::new(
            "token",
            "sheet",
            Dimension::Rows,
            0,
            1,
        );
        assert!(min_insert.validate().is_ok());

        let max_insert = InsertDimensionRangeRequest::new(
            "token",
            "sheet",
            Dimension::Columns,
            0,
            1000,
        );
        assert!(max_insert.validate().is_ok());

        // 测试单行/列插入
        let single_insert = InsertDimensionRangeRequest::new(
            "token",
            "sheet",
            Dimension::Rows,
            10,
            11,
        );
        assert!(single_insert.validate().is_ok());
    }
}