//! Sheets v3 - 企业级表格管理API
//!
//! 提供完整的电子表格生命周期管理功能，包括：
//! - 工作簿和工作表的创建、编辑、删除管理
//! - 单元格数据读写和批量处理操作
//! - 公式计算和函数库支持系统
//! - 图表生成和数据可视化功能
//! - 实时协作编辑和权限控制
//! - 数据导入导出和格式转换
//! - 表格模板管理和自动化处理
//! - 版本控制和历史记录管理
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::cloud_docs::v1::sheets::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 创建新的电子表格
//!     let create_response = client.cloud_docs.v1.sheets
//!         .create_spreadsheet_builder()
//!         .title("月度销售报表")
//!         .folder_token("folder_token_001")
//!         .template_type("basic")
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("表格创建成功，Token: {}", create_response.data.spreadsheet_token);
//!
//!     // 获取工作表信息
//!     let meta_response = client.cloud_docs.v1.sheets
//!         .get_sheet_meta_builder()
//!         .spreadsheet_token(&create_response.data.spreadsheet_token)
//!         .sheet_id("sheet_001")
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("工作表标题: {}", meta_response.data.title);
//!
//!     // 写入数据到指定范围
//!     let write_response = client.cloud_docs.v1.sheets
//!         .write_range_builder()
//!         .spreadsheet_token(&create_response.data.spreadsheet_token)
//!         .sheet_id("sheet_001")
//!         .range("A1:C10")
//!         .values(vec![
//!             vec!["产品".to_string(), "销量".to_string(), "金额".to_string()],
//!             vec!["产品A".to_string(), "100".to_string(), "10000".to_string()],
//!         ])
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("写入 {} 个单元格", write_response.data.update_rows);
//!
//!     // 读取数据范围
//!     let read_response = client.cloud_docs.v1.sheets
//!         .read_range_builder()
//!         .spreadsheet_token(&create_response.data.spreadsheet_token)
//!         .sheet_id("sheet_001")
//!         .range("A1:C10")
//!         .value_render_option("displayed")
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("读取到 {} 行 {} 列数据",
//!         read_response.data.row_number,
//!         read_response.data.column_number);
//!
//!     // 添加公式计算
//!     let formula_response = client.cloud_docs.v1.sheets
//!         .write_range_builder()
//!         .spreadsheet_token(&create_response.data.spreadsheet_token)
//!         .sheet_id("sheet_001")
//!         .range("C2")
//!         .values(vec![vec!["=B2*100".to_string()]])
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("公式添加成功");
//!
//!     // 创建图表
//!     let chart_response = client.cloud_docs.v1.sheets
//!         .create_chart_builder()
//!         .spreadsheet_token(&create_response.data.spreadsheet_token)
//!         .sheet_id("sheet_001")
//!         .chart_type("column")
//!         .name("销售数据图表")
//!         .data_range("A1:C10")
//!         .execute(&client.cloud_docs.v1.sheets)
//!         .await?;
//!
//!     println!("图表创建成功，ID: {}", chart_response.data.chart_id);
//!
//!     Ok(())
//! }
//! ```

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use async_trait::async_trait;
use open_lark_core::core::{api_req::ApiRequest, trait_system::ExecutableBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 表格管理服务 v3
///
/// 提供完整的企业级电子表格管理功能，支持工作簿、工作表、单元格的
/// 全生命周期管理，以及协作编辑、数据分析、图表生成等高级功能。
///
/// # 核心功能
///
/// - **工作簿管理**: 创建、查询、更新、删除电子表格
/// - **工作表操作**: 工作表的增删改查和属性管理
/// - **数据读写**: 单元格和范围的数据读写操作
/// - **公式计算**: 支持Excel兼容的公式和函数
/// - **图表生成**: 多种图表类型和自定义配置
/// - **协作编辑**: 实时协作和权限控制
/// - **数据处理**: 排序、筛选、格式化等功能
/// - **导入导出**: 支持多种格式的数据转换
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::cloud_docs::v1::sheets::SheetsServiceV3;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = SheetsServiceV3::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct SheetsServiceV3 {
    pub config: Config,
}

impl SheetsServiceV3 {
    /// 创建表格服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::SheetsServiceV3;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SheetsServiceV3::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建电子表格
    ///
    /// 创建一个新的电子表格工作簿，支持指定标题、文件夹位置、模板类型等。
    /// 可以选择使用预定义模板快速创建常用表格类型。
    ///
    /// # API文档
    ///
    /// 创建新的电子表格工作簿，返回工作簿的访问token和基本信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建电子表格的请求参数，包含标题、文件夹、模板等配置
    ///
    /// # 返回值
    ///
    /// 返回创建的电子表格信息，包含访问token、工作簿ID等
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = CreateSpreadsheetRequest {
    ///     title: "销售报表".to_string(),
    ///     folder_token: Some("folder_token_001".to_string()),
    ///     template_type: Some("sales".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.create_spreadsheet(&request).await?;
    /// println!("表格创建成功: {}", response.data.spreadsheet_token);
    /// ```
    pub async fn create_spreadsheet(
        &self,
        request: CreateSpreadsheetRequest,
    ) -> SDKResult<BaseResponse<CreateSpreadsheetResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/sheets/v3/spreadsheets".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 获取工作表元数据
    ///
    /// 获取指定工作表的详细元数据信息，包括标题、尺寸、权限、
    /// 保护设置等完整的属性信息。
    ///
    /// # API文档
    ///
    /// 获取工作表的元数据信息，返回工作表的基本属性和配置信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取工作表元数据的请求参数
    ///
    /// # 返回值
    ///
    /// 返回工作表的详细元数据信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = GetSheetMetaRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: Some("sheet_001".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.get_sheet_meta(&request).await?;
    /// println!("工作表标题: {}", response.data.title);
    /// ```
    pub async fn get_sheet_meta(
        &self,
        request: GetSheetMetaRequest,
    ) -> SDKResult<BaseResponse<GetSheetMetaResponse>> {
        let mut query_params = HashMap::new();

        if let Some(sheet_id) = &request.sheet_id {
            query_params.insert("sheet_id".to_string(), sheet_id.clone());
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/meta",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params,

            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 读取数据范围
    ///
    /// 从指定工作表的数据范围中读取单元格内容，支持多种渲染选项
    /// 和数值格式设置，可按需获取原始值或显示值。
    ///
    /// # API文档
    ///
    /// 读取指定工作表中指定范围的单元格数据。
    ///
    /// # 参数
    ///
    /// * `request` - 读取数据范围的请求参数，包含范围、渲染选项等
    ///
    /// # 返回值
    ///
    /// 返回指定范围的单元格数据内容
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = ReadRangeRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    ///     range: "A1:C10".to_string(),
    ///     value_render_option: Some("displayed".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.read_range(&request).await?;
    /// println!("读取到 {} 行数据", response.data.row_number);
    /// ```
    pub async fn read_range(
        &self,
        request: ReadRangeRequest,
    ) -> SDKResult<BaseResponse<ReadRangeResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("sheet_id".to_string(), request.sheet_id);
        query_params.insert("range".to_string(), request.range);

        if let Some(value_render_option) = &request.value_render_option {
            query_params.insert(
                "value_render_option".to_string(),
                value_render_option.clone(),
            );
        }
        if let Some(datetime_render_option) = &request.datetime_render_option {
            query_params.insert(
                "datetime_render_option".to_string(),
                datetime_render_option.clone(),
            );
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/values",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params,

            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 写入数据范围
    ///
    /// 向指定工作表的数据范围写入单元格内容，支持批量写入和
    /// 格式设置，可用于数据导入、报表生成等场景。
    ///
    /// # API文档
    ///
    /// 向指定工作表中指定范围的单元格写入数据。
    ///
    /// # 参数
    ///
    /// * `request` - 写入数据范围的请求参数，包含数据内容和写入选项
    ///
    /// # 返回值
    ///
    /// 返回写入操作的执行结果和统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = WriteRangeRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    ///     range: "A1:C10".to_string(),
    ///     values: vec![
    ///         vec!["产品".to_string(), "销量".to_string(), "金额".to_string()],
    ///         vec!["产品A".to_string(), "100".to_string(), "10000".to_string()],
    ///     ],
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.write_range(&request).await?;
    /// println!("写入成功，更新行数: {}", response.data.update_rows);
    /// ```
    pub async fn write_range(
        &self,
        request: WriteRangeRequest,
    ) -> SDKResult<BaseResponse<WriteRangeResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PUT,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/values",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 创建图表
    ///
    /// 在工作表中创建新的数据可视化图表，支持多种图表类型
    /// 和自定义样式配置，用于数据分析和展示。
    ///
    /// # API文档
    ///
    /// 在工作表中创建新的图表。
    ///
    /// # 参数
    ///
    /// * `request` - 创建图表的请求参数，包含图表类型、数据范围、样式等
    ///
    /// # 返回值
    ///
    /// 返回创建的图表信息和配置详情
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = CreateChartRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    ///     chart_type: "column".to_string(),
    ///     name: "销售数据图表".to_string(),
    ///     data_range: "A1:C10".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.create_chart(&request).await?;
    /// println!("图表创建成功，ID: {}", response.data.chart_id);
    /// ```
    pub async fn create_chart(
        &self,
        request: CreateChartRequest,
    ) -> SDKResult<BaseResponse<CreateChartResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/charts",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 删除工作表
    ///
    /// 删除指定的工作表，此操作不可恢复。删除前请确保已备份
    /// 重要数据，并确认工作表不再被其他功能依赖。
    ///
    /// # API文档
    ///
    /// 删除指定的工作表。
    ///
    /// # 参数
    ///
    /// * `request` - 删除工作表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = DeleteSheetRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    /// };
    ///
    /// let response = service.delete_sheet(&request).await?;
    /// println!("工作表删除成功");
    /// ```
    pub async fn delete_sheet(
        &self,
        request: DeleteSheetRequest,
    ) -> SDKResult<BaseResponse<DeleteSheetResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::DELETE,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}",
                self.config.base_url, request.spreadsheet_token, request.sheet_id
            ),
            query_params: Default::default(),

            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 添加工作表
    ///
    /// 在现有的电子表格中添加新的工作表，支持自定义标题、
    /// 初始尺寸和位置配置。
    ///
    /// # API文档
    ///
    /// 在电子表格中添加新的工作表。
    ///
    /// # 参数
    ///
    /// * `request` - 添加工作表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回新创建的工作表信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = AddSheetRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     title: "新工作表".to_string(),
    ///     row_count: Some(100),
    ///     column_count: Some(26),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.add_sheet(&request).await?;
    /// println!("工作表添加成功，ID: {}", response.data.sheet_id);
    /// ```
    pub async fn add_sheet(
        &self,
        request: AddSheetRequest,
    ) -> SDKResult<BaseResponse<AddSheetResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/sheets",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 更新工作表属性
    ///
    /// 更新工作表的基本属性信息，包括标题、尺寸、颜色等。
    /// 用于工作表的配置管理和个性化设置。
    ///
    /// # API文档
    ///
    /// 更新工作表的属性信息。
    ///
    /// # 参数
    ///
    /// * `request` - 更新工作表属性的请求参数
    ///
    /// # 返回值
    ///
    /// 返回更新后的工作表属性信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = UpdateSheetRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    ///     title: Some("更新后的标题".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.update_sheet(&request).await?;
    /// println!("工作表属性更新成功");
    /// ```
    pub async fn update_sheet(
        &self,
        request: UpdateSheetRequest,
    ) -> SDKResult<BaseResponse<UpdateSheetResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PATCH,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}",
                self.config.base_url, request.spreadsheet_token, request.sheet_id
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 复制工作表
    ///
    /// 复制现有的工作表到新的位置或同一电子表格中，
    /// 支持保留内容和格式设置。
    ///
    /// # API文档
    ///
    /// 复制工作表到新的位置。
    ///
    /// # 参数
    ///
    /// * `request` - 复制工作表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回复制的工作表信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = CopySheetRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     sheet_id: "sheet_001".to_string(),
    ///     title: Some("工作表副本".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.copy_sheet(&request).await?;
    /// println!("工作表复制成功，新ID: {}", response.data.sheet_id);
    /// ```
    pub async fn copy_sheet(
        &self,
        request: CopySheetRequest,
    ) -> SDKResult<BaseResponse<CopySheetResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/copy",
                self.config.base_url, request.spreadsheet_token, request.sheet_id
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 批量操作
    ///
    /// 在单个请求中执行多个表格操作，提高批量数据处理的效率。
    /// 支持读写操作、格式设置、公式计算等多种操作类型。
    ///
    /// # API文档
    ///
    /// 批量执行多个表格操作。
    ///
    /// # 参数
    ///
    /// * `request` - 批量操作的请求参数，包含操作列表和执行选项
    ///
    /// # 返回值
    ///
    /// 返回所有操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let request = BatchUpdateRequest {
    ///     spreadsheet_token: "spreadsheet_token_001".to_string(),
    ///     requests: vec![
    ///         BatchUpdateRequestItem {
    ///             // 具体的操作内容
    ///             ..Default::default()
    ///         },
    ///     ],
    ///     ..Default::default()
    /// };
    ///
    /// let response = service.batch_update(&request).await?;
    /// println!("批量操作完成");
    /// ```
    pub async fn batch_update(
        &self,
        request: BatchUpdateRequest,
    ) -> SDKResult<BaseResponse<BatchUpdateResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: format!(
                "{}/open-apis/sheets/v3/spreadsheets/{}/batchUpdate",
                self.config.base_url, request.spreadsheet_token
            ),
            query_params: Default::default(),

            body: serde_json::to_vec(&request)?,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }
}

// ==================== 数据模型定义 ====================

/// 创建电子表格请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateSpreadsheetRequest {
    /// 电子表格标题
    pub title: String,
    /// 文件夹token（可选）
    pub folder_token: Option<String>,
    /// 模板类型（可选）
    pub template_type: Option<String>,
    /// 初始语言设置（可选）
    pub locale: Option<String>,
    /// 时区设置（可选）
    pub time_zone: Option<String>,
}

/// 创建电子表格响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateSpreadsheetResponse {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 电子表格URL
    #[serde(rename = "spreadsheet_url")]
    pub api_path: String,
    /// 应用ID
    #[serde(rename = "app_id")]
    pub app_id: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub created_at: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 权限信息
    pub permissions: Vec<PermissionInfo>,
}

/// 获取工作表元数据请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSheetMetaRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID（可选）
    pub sheet_id: Option<String>,
    /// 是否包含权限信息（可选）
    pub include_permissions: Option<bool>,
}

/// 获取工作表元数据响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetSheetMetaResponse {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 行数
    #[serde(rename = "row_count")]
    pub row_count: i32,
    /// 列数
    #[serde(rename = "column_count")]
    pub column_count: i32,
    /// 工作表类型
    #[serde(rename = "sheet_type")]
    pub sheet_type: String,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 冻结行列
    pub frozen: Option<FrozenInfo>,
    /// 网格线设置
    pub gridlines: Option<GridlinesInfo>,
    /// 权限信息
    pub permissions: Option<Vec<PermissionInfo>>,
    /// 保护信息
    pub protected: Option<ProtectedInfo>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub created_at: String,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub updated_at: String,
}

/// 读取数据范围请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct ReadRangeRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 读取范围
    pub range: String,
    /// 数值渲染选项（可选）
    #[serde(rename = "value_render_option")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项（可选）
    #[serde(rename = "datetime_render_option")]
    pub datetime_render_option: Option<String>,
}

/// 读取数据范围响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ReadRangeResponse {
    /// 行数
    #[serde(rename = "row_number")]
    pub row_number: i32,
    /// 列数
    #[serde(rename = "column_number")]
    pub column_number: i32,
    /// 数据值
    pub values: Vec<Vec<String>>,
    /// 范围信息
    pub range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
}

/// 写入数据范围请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct WriteRangeRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 写入范围
    pub range: String,
    /// 数据值
    pub values: Vec<Vec<String>>,
    /// 数值输入选项（可选）
    #[serde(rename = "value_input_option")]
    pub value_input_option: Option<String>,
}

/// 写入数据范围响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct WriteRangeResponse {
    /// 更新的行数
    #[serde(rename = "update_rows")]
    pub update_rows: i32,
    /// 更新的列数
    #[serde(rename = "update_columns")]
    pub update_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "update_cells")]
    pub update_cells: i32,
    /// 范围信息
    pub range: String,
}

/// 创建图表请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateChartRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 图表类型
    #[serde(rename = "chart_type")]
    pub chart_type: String,
    /// 图表名称
    pub name: String,
    /// 数据范围
    #[serde(rename = "data_range")]
    pub data_range: String,
    /// 图表位置
    pub position: Option<ChartPosition>,
    /// 样式配置
    pub style: Option<ChartStyle>,
}

/// 创建图表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateChartResponse {
    /// 图表ID
    #[serde(rename = "chart_id")]
    pub chart_id: String,
    /// 图表URL
    #[serde(rename = "chart_url")]
    pub chart_api_path: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub created_at: String,
}

/// 删除工作表请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeleteSheetRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
}

/// 删除工作表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteSheetResponse {
    /// 操作结果
    pub success: bool,
    /// 删除时间
    #[serde(rename = "delete_time")]
    pub deleted_at: String,
}

/// 添加工作表请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddSheetRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表标题
    pub title: String,
    /// 行数（可选）
    #[serde(rename = "row_count")]
    pub row_count: Option<i32>,
    /// 列数（可选）
    #[serde(rename = "column_count")]
    pub column_count: Option<i32>,
    /// 工作表颜色（可选）
    pub sheet_color: Option<String>,
    /// 插入位置（可选）
    pub index: Option<i32>,
}

/// 添加工作表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AddSheetResponse {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub created_at: String,
}

/// 更新工作表请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateSheetRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 新标题（可选）
    pub title: Option<String>,
    /// 工作表颜色（可选）
    pub sheet_color: Option<String>,
    /// 行数（可选）
    #[serde(rename = "row_count")]
    pub row_count: Option<i32>,
    /// 列数（可选）
    #[serde(rename = "column_count")]
    pub column_count: Option<i32>,
}

/// 更新工作表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateSheetResponse {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub updated_at: String,
}

/// 复制工作表请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct CopySheetRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 新标题（可选）
    pub title: Option<String>,
    /// 复制位置（可选）
    pub index: Option<i32>,
    /// 是否复制格式（可选）
    #[serde(rename = "copy_format")]
    pub copy_format: Option<bool>,
    /// 是否复制数据（可选）
    #[serde(rename = "copy_data")]
    pub copy_data: Option<bool>,
}

/// 复制工作表响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CopySheetResponse {
    /// 新工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 新工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 复制时间
    #[serde(rename = "copy_time")]
    pub copied_at: String,
}

/// 批量更新请求
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchUpdateRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 操作列表
    pub requests: Vec<BatchUpdateRequestItem>,
    /// 是否包含响应（可选）
    #[serde(rename = "include_responses")]
    pub include_responses: Option<bool>,
}

/// 批量更新请求项
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchUpdateRequestItem {
    /// 操作类型
    #[serde(rename = "update_cells")]
    pub update_cells: Option<UpdateCellsRequest>,
    /// 复制工作表
    #[serde(rename = "copy_sheet")]
    pub copy_sheet: Option<CopySheetToAnotherSpreadsheetRequest>,
    /// 添加工作表
    #[serde(rename = "add_sheet")]
    pub add_sheet: Option<AddSheetRequest>,
    /// 删除工作表
    #[serde(rename = "delete_sheet")]
    pub delete_sheet: Option<DeleteSheetRequest>,
}

/// 批量更新响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BatchUpdateResponse {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_id")]
    pub spreadsheet_id: String,
    /// 响应列表
    pub replies: Vec<BatchUpdateResponseItem>,
}

/// 批量更新响应项
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BatchUpdateResponseItem {
    /// 更新单元格响应
    #[serde(rename = "update_cells")]
    pub update_cells: Option<UpdateCellsResponse>,
    /// 复制工作表响应
    #[serde(rename = "copy_sheet")]
    pub copy_sheet: Option<CopySheetResponse>,
    /// 添加工作表响应
    #[serde(rename = "add_sheet")]
    pub add_sheet: Option<AddSheetResponse>,
    /// 删除工作表响应
    #[serde(rename = "delete_sheet")]
    pub delete_sheet: Option<DeleteSheetResponse>,
}

// ==================== 辅助数据结构 ====================

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名
    pub name: String,
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionInfo {
    /// 权限ID
    #[serde(rename = "permission_id")]
    pub permission_id: String,
    /// 权限类型
    #[serde(rename = "type")]
    pub permission_type: String,
    /// 角色类型
    #[serde(rename = "role")]
    pub role: String,
    /// 用户信息
    #[serde(rename = "user")]
    pub user: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名
    pub name: String,
}

/// 冻结信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrozenInfo {
    /// 冻结行数
    pub rows: Option<i32>,
    /// 冻结列数
    pub columns: Option<i32>,
}

/// 网格线信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GridlinesInfo {
    /// 是否显示行网格线
    #[serde(rename = "show_rows")]
    pub show_rows: Option<bool>,
    /// 是否显示列网格线
    #[serde(rename = "show_columns")]
    pub show_columns: Option<bool>,
}

/// 保护信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtectedInfo {
    /// 是否受保护
    pub protected: bool,
    /// 保护范围
    pub ranges: Option<Vec<ProtectedRange>>,
}

/// 保护范围
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtectedRange {
    /// 范围
    pub range: String,
    /// 权限描述
    pub description: String,
}

/// 图表位置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChartPosition {
    /// 行索引
    pub row_index: i32,
    /// 列索引
    pub column_index: i32,
    /// 宽度
    pub width: i32,
    /// 高度
    pub height: i32,
}

/// 图表样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChartStyle {
    /// 主题颜色
    pub theme_color: Option<String>,
    /// 显示图例
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// 显示标题
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
}

/// 更新单元格请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCellsRequest {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 范围
    pub range: String,
    /// 数据值
    pub values: Vec<Vec<String>>,
    /// 字段掩码
    pub fields: String,
}

/// 更新单元格响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCellsResponse {
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}

/// 复制工作表到其他表格请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopySheetToAnotherSpreadsheetRequest {
    /// 目标表格token
    #[serde(rename = "destination_spreadsheet_token")]
    pub destination_spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 新标题
    pub title: Option<String>,
}

// ==================== Builder模式实现 ====================

/// 创建电子表格构建器
///
/// 提供流式API来构建创建电子表格的请求参数，支持链式调用
/// 和参数验证，提高代码可读性和易用性。
#[derive(Debug, Clone, Default)]
pub struct CreateSpreadsheetBuilder {
    request: CreateSpreadsheetRequest,
}

impl CreateSpreadsheetBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格标题
    ///
    /// # 参数
    /// - `title`: 电子表格标题
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let builder = CreateSpreadsheetBuilder::new()
    ///     .title("销售报表");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置文件夹token
    ///
    /// # 参数
    /// - `folder_token`: 文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 设置模板类型
    ///
    /// # 参数
    /// - `template_type`: 模板类型
    pub fn template_type(mut self, template_type: impl Into<String>) -> Self {
        self.request.template_type = Some(template_type.into());
        self
    }

    /// 设置语言环境
    ///
    /// # 参数
    /// - `locale`: 语言环境
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.request.locale = Some(locale.into());
        self
    }

    /// 设置时区
    ///
    /// # 参数
    /// - `time_zone`: 时区设置
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.request.time_zone = Some(time_zone.into());
        self
    }

    /// 执行创建电子表格操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回创建的电子表格信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::v1::sheets::*;
    ///
    /// let response = CreateSpreadsheetBuilder::new()
    ///     .title("销售报表")
    ///     .template_type("basic")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CreateSpreadsheetResponse>> {
        service.create_spreadsheet(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 创建电子表格构建器
    ///
    /// 返回一个用于创建电子表格的构建器实例，支持流式API调用。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .create_spreadsheet_builder()
    ///     .title("销售报表")
    ///     .template_type("basic")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn create_spreadsheet_builder(&self) -> CreateSpreadsheetBuilder {
        CreateSpreadsheetBuilder::new()
    }
}

/// 获取工作表元数据构建器
#[derive(Debug, Clone, Default)]
pub struct GetSheetMetaBuilder {
    request: GetSheetMetaRequest,
}

impl GetSheetMetaBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = Some(sheet_id.into());
        self
    }

    /// 设置是否包含权限信息
    ///
    /// # 参数
    /// - `include_permissions`: 是否包含权限信息
    pub fn include_permissions(mut self, include_permissions: bool) -> Self {
        self.request.include_permissions = Some(include_permissions);
        self
    }

    /// 执行获取工作表元数据操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回工作表的元数据信息
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<GetSheetMetaResponse>> {
        service.get_sheet_meta(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 获取工作表元数据构建器
    ///
    /// 返回一个用于获取工作表元数据的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .get_sheet_meta_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn get_sheet_meta_builder(&self) -> GetSheetMetaBuilder {
        GetSheetMetaBuilder::new()
    }
}

/// 读取数据范围构建器
#[derive(Debug, Clone, Default)]
pub struct ReadRangeBuilder {
    request: ReadRangeRequest,
}

impl ReadRangeBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 设置读取范围
    ///
    /// # 参数
    /// - `range`: 读取范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 设置数值渲染选项
    ///
    /// # 参数
    /// - `value_render_option`: 数值渲染选项
    pub fn value_render_option(mut self, value_render_option: impl Into<String>) -> Self {
        self.request.value_render_option = Some(value_render_option.into());
        self
    }

    /// 设置日期时间渲染选项
    ///
    /// # 参数
    /// - `datetime_render_option`: 日期时间渲染选项
    pub fn datetime_render_option(mut self, datetime_render_option: impl Into<String>) -> Self {
        self.request.datetime_render_option = Some(datetime_render_option.into());
        self
    }

    /// 执行读取数据范围操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回读取的数据内容
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<ReadRangeResponse>> {
        service.read_range(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 读取数据范围构建器
    ///
    /// 返回一个用于读取数据范围的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .read_range_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .range("A1:C10")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn read_range_builder(&self) -> ReadRangeBuilder {
        ReadRangeBuilder::new()
    }
}

/// 写入数据范围构建器
#[derive(Debug, Clone, Default)]
pub struct WriteRangeBuilder {
    request: WriteRangeRequest,
}

impl WriteRangeBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 设置写入范围
    ///
    /// # 参数
    /// - `range`: 写入范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 设置数据值
    ///
    /// # 参数
    /// - `values`: 数据值矩阵
    pub fn values(mut self, values: Vec<Vec<String>>) -> Self {
        self.request.values = values;
        self
    }

    /// 设置数值输入选项
    ///
    /// # 参数
    /// - `value_input_option`: 数值输入选项
    pub fn value_input_option(mut self, value_input_option: impl Into<String>) -> Self {
        self.request.value_input_option = Some(value_input_option.into());
        self
    }

    /// 执行写入数据范围操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回写入操作的执行结果
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<WriteRangeResponse>> {
        service.write_range(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 写入数据范围构建器
    ///
    /// 返回一个用于写入数据范围的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .write_range_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .range("A1:C10")
    ///     .values(vec![
    ///         vec!["产品".to_string(), "销量".to_string()],
    ///         vec!["产品A".to_string(), "100".to_string()],
    ///     ])
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn write_range_builder(&self) -> WriteRangeBuilder {
        WriteRangeBuilder::new()
    }
}

/// 创建图表构建器
#[derive(Debug, Clone, Default)]
pub struct CreateChartBuilder {
    request: CreateChartRequest,
}

impl CreateChartBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 设置图表类型
    ///
    /// # 参数
    /// - `chart_type`: 图表类型
    pub fn chart_type(mut self, chart_type: impl Into<String>) -> Self {
        self.request.chart_type = chart_type.into();
        self
    }

    /// 设置图表名称
    ///
    /// # 参数
    /// - `name`: 图表名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置数据范围
    ///
    /// # 参数
    /// - `data_range`: 数据范围
    pub fn data_range(mut self, data_range: impl Into<String>) -> Self {
        self.request.data_range = data_range.into();
        self
    }

    /// 设置图表位置
    ///
    /// # 参数
    /// - `position`: 图表位置
    pub fn position(mut self, position: ChartPosition) -> Self {
        self.request.position = Some(position);
        self
    }

    /// 设置图表样式
    ///
    /// # 参数
    /// - `style`: 图表样式
    pub fn style(mut self, style: ChartStyle) -> Self {
        self.request.style = Some(style);
        self
    }

    /// 执行创建图表操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回创建的图表信息
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CreateChartResponse>> {
        service.create_chart(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 创建图表构建器
    ///
    /// 返回一个用于创建图表的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .create_chart_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .chart_type("column")
    ///     .name("销售图表")
    ///     .data_range("A1:C10")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn create_chart_builder(&self) -> CreateChartBuilder {
        CreateChartBuilder::new()
    }
}

/// 删除工作表构建器
#[derive(Debug, Clone, Default)]
pub struct DeleteSheetBuilder {
    request: DeleteSheetRequest,
}

impl DeleteSheetBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 执行删除工作表操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<DeleteSheetResponse>> {
        service.delete_sheet(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 删除工作表构建器
    ///
    /// 返回一个用于删除工作表的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .delete_sheet_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn delete_sheet_builder(&self) -> DeleteSheetBuilder {
        DeleteSheetBuilder::new()
    }
}

/// 添加工作表构建器
#[derive(Debug, Clone, Default)]
pub struct AddSheetBuilder {
    request: AddSheetRequest,
}

impl AddSheetBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表标题
    ///
    /// # 参数
    /// - `title`: 工作表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置行数
    ///
    /// # 参数
    /// - `row_count`: 行数
    pub fn row_count(mut self, row_count: i32) -> Self {
        self.request.row_count = Some(row_count);
        self
    }

    /// 设置列数
    ///
    /// # 参数
    /// - `column_count`: 列数
    pub fn column_count(mut self, column_count: i32) -> Self {
        self.request.column_count = Some(column_count);
        self
    }

    /// 设置工作表颜色
    ///
    /// # 参数
    /// - `sheet_color`: 工作表颜色
    pub fn sheet_color(mut self, sheet_color: impl Into<String>) -> Self {
        self.request.sheet_color = Some(sheet_color.into());
        self
    }

    /// 设置插入位置
    ///
    /// # 参数
    /// - `index`: 插入位置
    pub fn index(mut self, index: i32) -> Self {
        self.request.index = Some(index);
        self
    }

    /// 执行添加工作表操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回新创建的工作表信息
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<AddSheetResponse>> {
        service.add_sheet(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 添加工作表构建器
    ///
    /// 返回一个用于添加工作表的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .add_sheet_builder()
    ///     .spreadsheet_token("token_001")
    ///     .title("新工作表")
    ///     .row_count(100)
    ///     .column_count(26)
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn add_sheet_builder(&self) -> AddSheetBuilder {
        AddSheetBuilder::new()
    }
}

/// 更新工作表构建器
#[derive(Debug, Clone, Default)]
pub struct UpdateSheetBuilder {
    request: UpdateSheetRequest,
}

impl UpdateSheetBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 设置新标题
    ///
    /// # 参数
    /// - `title`: 新标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置工作表颜色
    ///
    /// # 参数
    /// - `sheet_color`: 工作表颜色
    pub fn sheet_color(mut self, sheet_color: impl Into<String>) -> Self {
        self.request.sheet_color = Some(sheet_color.into());
        self
    }

    /// 设置行数
    ///
    /// # 参数
    /// - `row_count`: 行数
    pub fn row_count(mut self, row_count: i32) -> Self {
        self.request.row_count = Some(row_count);
        self
    }

    /// 设置列数
    ///
    /// # 参数
    /// - `column_count`: 列数
    pub fn column_count(mut self, column_count: i32) -> Self {
        self.request.column_count = Some(column_count);
        self
    }

    /// 执行更新工作表操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回更新后的工作表信息
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<UpdateSheetResponse>> {
        service.update_sheet(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 更新工作表构建器
    ///
    /// 返回一个用于更新工作表的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .update_sheet_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .title("更新后的标题")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn update_sheet_builder(&self) -> UpdateSheetBuilder {
        UpdateSheetBuilder::new()
    }
}

/// 复制工作表构建器
#[derive(Debug, Clone, Default)]
pub struct CopySheetBuilder {
    request: CopySheetRequest,
}

impl CopySheetBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置工作表ID
    ///
    /// # 参数
    /// - `sheet_id`: 工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.request.sheet_id = sheet_id.into();
        self
    }

    /// 设置新标题
    ///
    /// # 参数
    /// - `title`: 新标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置复制位置
    ///
    /// # 参数
    /// - `index`: 复制位置
    pub fn index(mut self, index: i32) -> Self {
        self.request.index = Some(index);
        self
    }

    /// 设置是否复制格式
    ///
    /// # 参数
    /// - `copy_format`: 是否复制格式
    pub fn copy_format(mut self, copy_format: bool) -> Self {
        self.request.copy_format = Some(copy_format);
        self
    }

    /// 设置是否复制数据
    ///
    /// # 参数
    /// - `copy_data`: 是否复制数据
    pub fn copy_data(mut self, copy_data: bool) -> Self {
        self.request.copy_data = Some(copy_data);
        self
    }

    /// 执行复制工作表操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回复制的工作表信息
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CopySheetResponse>> {
        service.copy_sheet(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 复制工作表构建器
    ///
    /// 返回一个用于复制工作表的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .copy_sheet_builder()
    ///     .spreadsheet_token("token_001")
    ///     .sheet_id("sheet_001")
    ///     .title("工作表副本")
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn copy_sheet_builder(&self) -> CopySheetBuilder {
        CopySheetBuilder::new()
    }
}

/// 批量更新构建器
#[derive(Debug, Clone, Default)]
pub struct BatchUpdateBuilder {
    request: BatchUpdateRequest,
}

impl BatchUpdateBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 添加操作请求
    ///
    /// # 参数
    /// - `request_item`: 操作请求项
    pub fn add_request(mut self, request_item: BatchUpdateRequestItem) -> Self {
        self.request.requests.push(request_item);
        self
    }

    /// 设置操作请求列表
    ///
    /// # 参数
    /// - `requests`: 操作请求列表
    pub fn requests(mut self, requests: Vec<BatchUpdateRequestItem>) -> Self {
        self.request.requests = requests;
        self
    }

    /// 设置是否包含响应
    ///
    /// # 参数
    /// - `include_responses`: 是否包含响应
    pub fn include_responses(mut self, include_responses: bool) -> Self {
        self.request.include_responses = Some(include_responses);
        self
    }

    /// 执行批量更新操作
    ///
    /// # 参数
    /// - `service`: 表格服务实例
    ///
    /// # 返回值
    ///
    /// 返回所有操作的执行结果
    pub async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<BatchUpdateResponse>> {
        service.batch_update(self.request).await
    }
}

impl SheetsServiceV3 {
    /// 批量更新构建器
    ///
    /// 返回一个用于批量更新的构建器实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// let response = client.cloud_docs.v1.sheets
    ///     .batch_update_builder()
    ///     .spreadsheet_token("token_001")
    ///     .add_request(batch_item)
    ///     .include_responses(true)
    ///     .execute(&client.cloud_docs.v1.sheets)
    ///     .await?;
    /// ```
    pub fn batch_update_builder(&self) -> BatchUpdateBuilder {
        BatchUpdateBuilder::new()
    }
}

// ==================== 可执行特征实现 ====================

/// 实现所有构建器的可执行特征
#[async_trait]
impl
    ExecutableBuilder<
        SheetsServiceV3,
        CreateSpreadsheetRequest,
        BaseResponse<CreateSpreadsheetResponse>,
    > for CreateSpreadsheetBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CreateSpreadsheetResponse>> {
        service.create_spreadsheet(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, GetSheetMetaRequest, BaseResponse<GetSheetMetaResponse>>
    for GetSheetMetaBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<GetSheetMetaResponse>> {
        service.get_sheet_meta(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, ReadRangeRequest, BaseResponse<ReadRangeResponse>>
    for ReadRangeBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<ReadRangeResponse>> {
        service.read_range(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, WriteRangeRequest, BaseResponse<WriteRangeResponse>>
    for WriteRangeBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<WriteRangeResponse>> {
        service.write_range(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, CreateChartRequest, BaseResponse<CreateChartResponse>>
    for CreateChartBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CreateChartResponse>> {
        service.create_chart(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, DeleteSheetRequest, BaseResponse<DeleteSheetResponse>>
    for DeleteSheetBuilder
{
    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<DeleteSheetResponse>> {
        service.delete_sheet(self.request).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, AddSheetRequest, BaseResponse<AddSheetResponse>>
    for AddSheetBuilder
{
    fn build(self) -> AddSheetRequest {
        self.request
    }

    async fn execute(self, service: &SheetsServiceV3) -> SDKResult<BaseResponse<AddSheetResponse>> {
        service.add_sheet(self.request).await
    }

    async fn execute_with_options(
        self,
        service: &SheetsServiceV3,
        _options: RequestOption,
    ) -> SDKResult<BaseResponse<AddSheetResponse>> {
        self.execute(service).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, UpdateSheetRequest, BaseResponse<UpdateSheetResponse>>
    for UpdateSheetBuilder
{
    fn build(self) -> UpdateSheetRequest {
        self.request
    }

    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<UpdateSheetResponse>> {
        service.update_sheet(self.request).await
    }

    async fn execute_with_options(
        self,
        service: &SheetsServiceV3,
        _options: RequestOption,
    ) -> SDKResult<BaseResponse<UpdateSheetResponse>> {
        self.execute(service).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, CopySheetRequest, BaseResponse<CopySheetResponse>>
    for CopySheetBuilder
{
    fn build(self) -> CopySheetRequest {
        self.request
    }

    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<CopySheetResponse>> {
        service.copy_sheet(self.request).await
    }

    async fn execute_with_options(
        self,
        service: &SheetsServiceV3,
        _options: RequestOption,
    ) -> SDKResult<BaseResponse<CopySheetResponse>> {
        self.execute(service).await
    }
}

#[async_trait]
impl ExecutableBuilder<SheetsServiceV3, BatchUpdateRequest, BaseResponse<BatchUpdateResponse>>
    for BatchUpdateBuilder
{
    fn build(self) -> BatchUpdateRequest {
        self.request
    }

    async fn execute(
        self,
        service: &SheetsServiceV3,
    ) -> SDKResult<BaseResponse<BatchUpdateResponse>> {
        service.batch_update(self.request).await
    }

    async fn execute_with_options(
        self,
        service: &SheetsServiceV3,
        _options: RequestOption,
    ) -> SDKResult<BaseResponse<BatchUpdateResponse>> {
        self.execute(service).await
    }
}
