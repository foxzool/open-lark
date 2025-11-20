//! 行列操作服务
//!
//! 提供飞书电子表格v2版本的行列操作功能，包括：
//! - 插入行或列
//! - 删除行或列
//! - 增加行列范围
//! - 更新行列属性
//! - 移动行列位置

use serde_json::Value;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

/// 增加行列范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeRequest {
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

impl AddDimensionRangeRequest {
    /// 创建新的增加行列请求
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
            return Err("单次增加行列数量不能超过1000".to_string());
        }

        Ok(())
    }
}

/// 增加行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeResponse {
    /// 增加的行列数量
    #[serde(rename = "dimensionLength")]
    pub dimension_length: i32,
}

/// 增加行列数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDimensionRangeData {
    /// 增加行列响应
    pub add_dimension_range: AddDimensionRangeResponse,
}

impl ApiResponseTrait for AddDimensionRangeData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新行列属性请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeRequest {
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
    /// 行列属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}

/// 行列属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// 是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_by_user: Option<bool>,
    /// 是否冻结
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<bool>,
    /// 行高（仅适用于行）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_size: Option<i32>,
    /// 列宽（仅适用于列）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_width: Option<f64>,
}

impl Default for DimensionProperties {
    fn default() -> Self {
        Self {
            hidden_by_user: None,
            frozen: None,
            pixel_size: None,
            column_width: None,
        }
    }
}

impl UpdateDimensionRangeRequest {
    /// 创建新的更新行列请求
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
            properties: None,
        }
    }

    /// 设置行列属性
    pub fn properties(mut self, properties: DimensionProperties) -> Self {
        self.properties = Some(properties);
        self
    }

    /// 设置隐藏状态
    pub fn hidden(mut self, hidden: bool) -> Self {
        let mut props = self.properties.unwrap_or_default();
        props.hidden_by_user = Some(hidden);
        self.properties = Some(props);
        self
    }

    /// 设置冻结状态
    pub fn frozen(mut self, frozen: bool) -> Self {
        let mut props = self.properties.unwrap_or_default();
        props.frozen = Some(frozen);
        self.properties = Some(props);
        self
    }

    /// 设置行高
    pub fn pixel_size(mut self, size: i32) -> Self {
        let mut props = self.properties.unwrap_or_default();
        props.pixel_size = Some(size);
        self.properties = Some(props);
        self
    }

    /// 设置列宽
    pub fn column_width(mut self, width: f64) -> Self {
        let mut props = self.properties.unwrap_or_default();
        props.column_width = Some(width);
        self.properties = Some(props);
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
            return Err("单次更新行列数量不能超过1000".to_string());
        }

        // 验证属性
        if let Some(ref props) = self.properties {
            if let Some(size) = props.pixel_size {
                if size <= 0 || size > 4096 {
                    return Err("行高必须在1-4096像素之间".to_string());
                }
            }

            if let Some(width) = props.column_width {
                if width <= 0.0 || width > 4096.0 {
                    return Err("列宽必须在0-4096像素之间".to_string());
                }
            }
        }

        Ok(())
    }
}

/// 更新行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeResponse {
    /// 更新的行列数量
    #[serde(rename = "dimensionLength")]
    pub dimension_length: i32,
}

/// 更新行列数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionRangeData {
    /// 更新行列响应
    pub update_dimension_range: UpdateDimensionRangeResponse,
}

impl ApiResponseTrait for UpdateDimensionRangeData {
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
#[derive(Clone, Debug)]
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
    ) -> SDKResult<Response<InsertDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/insert_dimension_range",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<InsertDimensionRangeResponse>::request(api_req, &self.config, option)
                .await?;

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
    ) -> SDKResult<Response<DeleteDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::DELETE,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<DeleteDimensionRangeResponse>::request(api_req, &self.config, option)
                .await?;

        Ok(api_resp)
    }

    /// 增加行列范围
    ///
    /// 在指定位置增加行或列，支持批量操作
    ///
    /// # 参数
    /// - `request`: 增加行列请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 增加操作结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::dimension_operations::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DimensionOperationsService::new(config);
    ///
    /// // 在第2行位置增加5行
    /// let request = AddDimensionRangeRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet_id",
    ///     Dimension::Rows,
    ///     2,
    ///     7,
    /// ).inherit_style(true);
    ///
    /// let response = service.add_dimension_range(request, None).await?;
    /// ```
    pub async fn add_dimension_range(
        &self,
        request: AddDimensionRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AddDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<AddDimensionRangeResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新行列属性
    ///
    /// 更新指定行或列的属性，包括隐藏、冻结、尺寸等
    ///
    /// # 参数
    /// - `request`: 更新行列请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新操作结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::dimension_operations::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DimensionOperationsService::new(config);
    ///
    /// // 更新第1列的宽度
    /// let request = UpdateDimensionRangeRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet_id",
    ///     Dimension::Columns,
    ///     1,
    ///     2,
    /// ).column_width(150.0);
    ///
    /// let response = service.update_dimension_range(request, None).await?;
    /// ```
    pub async fn update_dimension_range(
        &self,
        request: UpdateDimensionRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UpdateDimensionRangeResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::PUT,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dimension_range",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<UpdateDimensionRangeResponse>::request(api_req, &self.config, option)
                .await?;

        Ok(api_resp)
    }

    /// 创建增加行构建器
    pub fn add_rows_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> AddDimensionRangeBuilder {
        AddDimensionRangeBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Rows,
            start_index,
            count,
        )
    }

    /// 创建增加列构建器
    pub fn add_columns_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        count: i32,
    ) -> AddDimensionRangeBuilder {
        AddDimensionRangeBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Columns,
            start_index,
            count,
        )
    }

    /// 创建更新行构建器
    pub fn update_rows_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        end_index: i32,
    ) -> UpdateDimensionRangeBuilder {
        UpdateDimensionRangeBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Rows,
            start_index,
            end_index,
        )
    }

    /// 创建更新列构建器
    pub fn update_columns_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        start_index: i32,
        end_index: i32,
    ) -> UpdateDimensionRangeBuilder {
        UpdateDimensionRangeBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Columns,
            start_index,
            end_index,
        )
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
            self.config.clone()
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
            self.config.clone()
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
            self.config.clone()
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
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            Dimension::Columns,
            start_index,
            count,
        )
    }
}

/// 插入行列构建器
#[derive(Clone, Debug)]
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
    pub async fn execute(self) -> SDKResult<Response<InsertDimensionRangeResponse>> {
        let request = InsertDimensionRangeRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.dimension,
            self.start_index,
            self.end_index,
        )
        .inherit_style(self.inherit_style);

        let service = DimensionOperationsService {
            config: self.config,
        };
        service.insert_dimension_range(request, None).await
    }
}

/// 删除行列构建器
#[derive(Clone, Debug)]
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
    pub async fn execute(self) -> SDKResult<Response<DeleteDimensionRangeResponse>> {
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

/// 增加行列范围构建器
pub struct AddDimensionRangeBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    dimension: Dimension,
    start_index: i32,
    count: i32,
    inherit_style: Option<bool>,
}

impl AddDimensionRangeBuilder {
    /// 创建新的增加行列构建器实例
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
            count,
            inherit_style: None,
        }
    }

    /// 设置是否继承属性
    pub fn inherit_style(mut self, inherit_style: bool) -> Self {
        self.inherit_style = Some(inherit_style);
        self
    }

    /// 设置增加范围
    pub fn range(mut self, start_index: i32, count: i32) -> Self {
        self.start_index = start_index;
        self.count = count;
        self
    }

    /// 执行增加请求
    pub async fn execute(self) -> SDKResult<Response<AddDimensionRangeResponse>> {
        let mut request = AddDimensionRangeRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.dimension,
            self.start_index,
            self.start_index + self.count,
        );

        if let Some(inherit_style) = self.inherit_style {
            request = request.inherit_style(inherit_style);
        }

        let service = DimensionOperationsService {
            config: self.config,
        };
        service.add_dimension_range(request, None).await
    }
}

/// 更新行列属性构建器
pub struct UpdateDimensionRangeBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    dimension: Dimension,
    start_index: i32,
    end_index: i32,
    properties: DimensionProperties,
}

impl UpdateDimensionRangeBuilder {
    /// 创建新的更新行列构建器实例
    pub fn new(
        config: Config,
        spreadsheet_token: String,
        sheet_id: String,
        dimension: Dimension,
        start_index: i32,
        end_index: i32,
    ) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            dimension,
            start_index,
            end_index,
            properties: DimensionProperties::default(),
        }
    }

    /// 设置更新范围
    pub fn range(mut self, start_index: i32, end_index: i32) -> Self {
        self.start_index = start_index;
        self.end_index = end_index;
        self
    }

    /// 设置隐藏状态
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.properties.hidden_by_user = Some(hidden);
        self
    }

    /// 设置冻结状态
    pub fn frozen(mut self, frozen: bool) -> Self {
        self.properties.frozen = Some(frozen);
        self
    }

    /// 设置行高（仅适用于行）
    pub fn pixel_size(mut self, size: i32) -> Self {
        self.properties.pixel_size = Some(size);
        self
    }

    /// 设置列宽（仅适用于列）
    pub fn column_width(mut self, width: f64) -> Self {
        self.properties.column_width = Some(width);
        self
    }

    /// 设置自定义属性
    pub fn properties(mut self, properties: DimensionProperties) -> Self {
        self.properties = properties;
        self
    }

    /// 执行更新请求
    pub async fn execute(self) -> SDKResult<Response<UpdateDimensionRangeResponse>> {
        let request = UpdateDimensionRangeRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.dimension,
            self.start_index,
            self.end_index,
        )
        .properties(self.properties);

        let service = DimensionOperationsService {
            config: self.config,
        };
        service.update_dimension_range(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_dimension_range_request_creation() {
        let request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 1, 4)
                .inherit_style(true);

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
        let valid_request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 0, 5);
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_token_request =
            InsertDimensionRangeRequest::new("", "sheet1", Dimension::Rows, 1, 3);
        assert!(empty_token_request.validate().is_err());

        // 测试负数索引
        let negative_index_request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, -1, 3);
        assert!(negative_index_request.validate().is_err());

        // 测试结束索引小于起始索引
        let invalid_range_request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 5, 3);
        assert!(invalid_range_request.validate().is_err());

        // 测试范围过大
        let large_range_request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 0, 1001);
        assert!(large_range_request.validate().is_err());
    }

    #[test]
    fn test_delete_dimension_range_request() {
        let request =
            DeleteDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 2, 5);

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
        let config = openlark_core::config::Config::default();
        let builder = InsertDimensionRangeBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Rows,
            1,
            3,
        )
        .inherit_style(true);

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
        let config = openlark_core::config::Config::default();
        let builder = DeleteDimensionRangeBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Columns,
            0,
            2,
        )
        .range(5, 8);

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.dimension, Dimension::Columns);
        assert_eq!(builder.start_index, 5);
        assert_eq!(builder.end_index, 8);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_dimension_operations_service_creation() {
        let config = openlark_core::config::Config::builder()
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
        let original_request =
            InsertDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 2, 5)
                .inherit_style(true);

        let serialized = serde_json::to_string(&original_request).unwrap();
        let deserialized: InsertDimensionRangeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            original_request.spreadsheet_token,
            deserialized.spreadsheet_token
        );
        assert_eq!(original_request.sheet_id, deserialized.sheet_id);
        assert_eq!(original_request.dimension, deserialized.dimension);
        assert_eq!(original_request.start_index, deserialized.start_index);
        assert_eq!(original_request.end_index, deserialized.end_index);
        assert_eq!(original_request.inherit_style, deserialized.inherit_style);
    }

    #[test]
    fn test_builder_methods() {
        let config = openlark_core::config::Config::default();

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
        let min_insert = InsertDimensionRangeRequest::new("token", "sheet", Dimension::Rows, 0, 1);
        assert!(min_insert.validate().is_ok());

        let max_insert =
            InsertDimensionRangeRequest::new("token", "sheet", Dimension::Columns, 0, 1000);
        assert!(max_insert.validate().is_ok());

        // 测试单行/列插入
        let single_insert =
            InsertDimensionRangeRequest::new("token", "sheet", Dimension::Rows, 10, 11);
        assert!(single_insert.validate().is_ok());
    }

    #[test]
    fn test_add_dimension_range_request_creation() {
        let request = AddDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 2, 7)
            .inherit_style(true);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.dimension, Dimension::Rows);
        assert_eq!(request.start_index, 2);
        assert_eq!(request.end_index, 7);
        assert_eq!(request.inherit_style, Some(true));
    }

    #[test]
    fn test_add_dimension_range_request_validation() {
        // 测试正常请求
        let request =
            AddDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 1, 6);
        assert!(request.validate().is_ok());

        // 测试空token
        let request = AddDimensionRangeRequest::new("", "sheet1", Dimension::Columns, 1, 6);
        assert!(request.validate().is_err());

        // 测试无效索引
        let request =
            AddDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, -1, 6);
        assert!(request.validate().is_err());

        // 测试索引范围错误
        let request =
            AddDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 6, 1);
        assert!(request.validate().is_err());

        // 测试超过最大数量
        let request =
            AddDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 1, 1002);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_dimension_range_request_creation() {
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 1, 5)
                .pixel_size(30);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.dimension, Dimension::Rows);
        assert_eq!(request.start_index, 1);
        assert_eq!(request.end_index, 5);
        assert!(request.properties.is_some());
        assert_eq!(request.properties.as_ref().unwrap().pixel_size, Some(30));
    }

    #[test]
    fn test_update_dimension_range_request_with_properties() {
        let properties = DimensionProperties {
            hidden_by_user: Some(true),
            frozen: Some(false),
            pixel_size: Some(25),
            column_width: Some(120.0),
        };

        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 2, 4)
                .properties(properties);

        let props = request.properties.unwrap();
        assert_eq!(props.hidden_by_user, Some(true));
        assert_eq!(props.frozen, Some(false));
        assert_eq!(props.pixel_size, Some(25));
        assert_eq!(props.column_width, Some(120.0));
    }

    #[test]
    fn test_update_dimension_range_request_validation() {
        // 测试正常请求
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 1, 6);
        assert!(request.validate().is_ok());

        // 测试属性验证 - 无效行高
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 1, 6)
                .pixel_size(0);
        assert!(request.validate().is_err());

        // 测试属性验证 - 行高过大
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Rows, 1, 6)
                .pixel_size(5000);
        assert!(request.validate().is_err());

        // 测试属性验证 - 无效列宽
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 1, 6)
                .column_width(-1.0);
        assert!(request.validate().is_err());

        // 测试属性验证 - 列宽过大
        let request =
            UpdateDimensionRangeRequest::new("test_token", "sheet1", Dimension::Columns, 1, 6)
                .column_width(5000.0);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_dimension_properties_default() {
        let props = DimensionProperties::default();
        assert!(props.hidden_by_user.is_none());
        assert!(props.frozen.is_none());
        assert!(props.pixel_size.is_none());
        assert!(props.column_width.is_none());
    }

    #[test]
    fn test_add_dimension_range_builder() {
        let config = openlark_core::config::Config::default();
        let builder = AddDimensionRangeBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Rows,
            2,
            5,
        )
        .inherit_style(true);

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.dimension, Dimension::Rows);
        assert_eq!(builder.start_index, 2);
        assert_eq!(builder.count, 5);
        assert_eq!(builder.inherit_style, Some(true));

        // 测试范围设置
        let builder = builder.range(3, 10);
        assert_eq!(builder.start_index, 3);
        assert_eq!(builder.count, 10);
    }

    #[test]
    fn test_update_dimension_range_builder() {
        let config = openlark_core::config::Config::default();
        let builder = UpdateDimensionRangeBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Columns,
            1,
            4,
        );

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.dimension, Dimension::Columns);
        assert_eq!(builder.start_index, 1);
        assert_eq!(builder.end_index, 4);

        // 测试属性设置
        let builder = builder.hidden(true).frozen(false).column_width(150.0);

        assert_eq!(builder.properties.hidden_by_user, Some(true));
        assert_eq!(builder.properties.frozen, Some(false));
        assert_eq!(builder.properties.column_width, Some(150.0));

        // 测试范围设置
        let builder = builder.range(2, 6);
        assert_eq!(builder.start_index, 2);
        assert_eq!(builder.end_index, 6);
    }

    #[test]
    fn test_update_dimension_range_builder_with_custom_properties() {
        let config = openlark_core::config::Config::default();
        let custom_props = DimensionProperties {
            hidden_by_user: Some(false),
            frozen: Some(true),
            pixel_size: Some(28),
            column_width: Some(100.5),
        };

        let builder = UpdateDimensionRangeBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            Dimension::Rows,
            1,
            3,
        )
        .properties(custom_props);

        assert_eq!(builder.properties.hidden_by_user, Some(false));
        assert_eq!(builder.properties.frozen, Some(true));
        assert_eq!(builder.properties.pixel_size, Some(28));
        assert_eq!(builder.properties.column_width, Some(100.5));
    }

    #[test]
    fn test_dimension_operations_service_new_methods() {
        let config = openlark_core::config::Config::default();
        let service = DimensionOperationsService::new(config);

        // 测试增加行构建器
        let add_rows_builder = service.add_rows_builder("token", "sheet", 1, 5);
        assert_eq!(add_rows_builder.spreadsheet_token, "token");
        assert_eq!(add_rows_builder.sheet_id, "sheet");
        assert_eq!(add_rows_builder.dimension, Dimension::Rows);
        assert_eq!(add_rows_builder.start_index, 1);
        assert_eq!(add_rows_builder.count, 5);

        // 测试增加列构建器
        let add_cols_builder = service.add_columns_builder("token", "sheet", 2, 3);
        assert_eq!(add_cols_builder.spreadsheet_token, "token");
        assert_eq!(add_cols_builder.sheet_id, "sheet");
        assert_eq!(add_cols_builder.dimension, Dimension::Columns);
        assert_eq!(add_cols_builder.start_index, 2);
        assert_eq!(add_cols_builder.count, 3);

        // 测试更新行构建器
        let update_rows_builder = service.update_rows_builder("token", "sheet", 1, 5);
        assert_eq!(update_rows_builder.spreadsheet_token, "token");
        assert_eq!(update_rows_builder.sheet_id, "sheet");
        assert_eq!(update_rows_builder.dimension, Dimension::Rows);
        assert_eq!(update_rows_builder.start_index, 1);
        assert_eq!(update_rows_builder.end_index, 5);

        // 测试更新列构建器
        let update_cols_builder = service.update_columns_builder("token", "sheet", 2, 6);
        assert_eq!(update_cols_builder.spreadsheet_token, "token");
        assert_eq!(update_cols_builder.sheet_id, "sheet");
        assert_eq!(update_cols_builder.dimension, Dimension::Columns);
        assert_eq!(update_cols_builder.start_index, 2);
        assert_eq!(update_cols_builder.end_index, 6);
    }

    #[test]
    fn test_complex_dimension_operations() {
        // 测试复杂的增加行列操作
        let add_request = AddDimensionRangeRequest::new(
            "spreadsheet_token",
            "sheet_001",
            Dimension::Rows,
            10,
            15, // 增加5行
        )
        .inherit_style(false);

        assert!(add_request.validate().is_ok());
        assert_eq!(add_request.end_index - add_request.start_index, 5);

        // 测试复杂的更新行列操作
        let update_request = UpdateDimensionRangeRequest::new(
            "spreadsheet_token",
            "sheet_001",
            Dimension::Columns,
            3,
            8, // 更新5列
        )
        .frozen(true)
        .column_width(120.5);

        assert!(update_request.validate().is_ok());
        assert_eq!(update_request.end_index - update_request.start_index, 5);

        let props = update_request.properties.unwrap();
        assert_eq!(props.frozen, Some(true));
        assert_eq!(props.column_width, Some(120.5));
    }
}
