//! 工作表行列移动服务
//!
//! 提供电子表格工作表行列移动功能，包括：
//! - 移动行和列到指定位置
//! - 批量行列移动操作
//! - 移动位置调整和冲突处理
//! - 移动操作状态跟踪

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    error::LarkAPIError,
    http::Transport,
};
use reqwest::Method;

use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;

// v3模块核心类型定义
pub type SpreadsheetToken = String;
pub type SheetId = String;
pub type CellValue = serde_json::Value;
pub type SheetPagedResponse<T> = Vec<T>;

/// 工作表行列移动服务
///
/// 提供电子表格工作表行列移动管理功能，支持工作表内行列的灵活重排。
/// 通过移动行列可以重新组织工作表结构，实现数据布局的动态调整。
///
/// # 功能特性
///
/// ## 行列移动类型
/// - **行移动**: 移动整行到指定位置
/// - **列移动**: 移动整列到指定位置
/// - **批量移动**: 一次性移动多个连续行列
///
/// ## 移动控制
/// - **源范围**: 指定要移动的行列范围
/// - **目标位置**: 设置移动的目标位置
/// - **插入模式**: 控制目标位置的插入方式
///
/// # 常见应用场景
///
/// ```rust
/// # use open_lark::prelude::*;
/// # use open_lark::service::sheets::v3::MoveDimensionService;
/// # use open_lark::service::sheets::v3::models::spreadsheet::SpreadsheetToken;
/// # use open_lark::service::sheets::v3::models::sheet::SheetId;
///
/// let service = MoveDimensionService::new(client_config);
///
/// // 场景1: 移动行到新位置
/// let request = MoveDimensionRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .dimension("ROWS".to_string())
///     .source_start_index(5)
///     .source_end_index(10)
///     .destination_index(2)
///     .build()?;
///
/// let response = service.move_dimension(&request).await?;
///
/// // 场景2: 移动列到表格末尾
/// let request = MoveDimensionRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .sheet_id("0XXXXXXXXXX".to_string())
///     .dimension("COLUMNS".to_string())
///     .source_start_index(0)
///     .source_end_index(3)
///     .destination_index(20) // 移动到第21列
///     .build()?;
///
/// let response = service.move_dimension(&request).await?;
/// ```
#[derive(Clone, Debug)]
pub struct MoveDimensionService {
    config: openlark_core::config::Config,
}

/// 移动行列请求
///
/// 用于移动工作表中的行或列到指定位置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: SheetId,
    /// 移动维度（ROWS或COLUMNS）
    pub dimension: String,
    /// 源起始索引（从0开始）
    #[serde(rename = "sourceStartIndex")]
    pub source_start_index: i32,
    /// 源结束索引（从0开始）
    #[serde(rename = "sourceEndIndex")]
    pub source_end_index: i32,
    /// 目标位置索引（从0开始）
    #[serde(rename = "destinationIndex")]
    pub destination_index: i32,
}

impl MoveDimensionRequest {
    /// 创建移动行列请求构建器
    pub fn builder() -> MoveDimensionRequestBuilder {
        MoveDimensionRequestBuilder::new()
    }
}

/// 移动行列请求构建器
pub struct MoveDimensionRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    dimension: Option<String>,
    source_start_index: Option<i32>,
    source_end_index: Option<i32>,
    destination_index: Option<i32>,
}

impl MoveDimensionRequestBuilder {
    /// 创建新的移动行列请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            sheet_id: None,
            dimension: None,
            source_start_index: None,
            source_end_index: None,
            destination_index: None,
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置移动维度
    pub fn dimension(mut self, dimension: String) -> Self {
        self.dimension = Some(dimension);
        self
    }

    /// 设置行移动
    pub fn rows(self) -> Self {
        self.dimension("ROWS".to_string())
    }

    /// 设置列移动
    pub fn columns(self) -> Self {
        self.dimension("COLUMNS".to_string())
    }

    /// 设置源起始索引
    pub fn source_start_index(mut self, source_start_index: i32) -> Self {
        self.source_start_index = Some(source_start_index);
        self
    }

    /// 设置源结束索引
    pub fn source_end_index(mut self, source_end_index: i32) -> Self {
        self.source_end_index = Some(source_end_index);
        self
    }

    /// 设置源范围（包含起始和结束索引）
    pub fn source_range(mut self, start_index: i32, end_index: i32) -> Self {
        self.source_start_index = Some(start_index);
        self.source_end_index = Some(end_index);
        self
    }

    /// 设置目标位置索引
    pub fn destination_index(mut self, destination_index: i32) -> Self {
        self.destination_index = Some(destination_index);
        self
    }

    /// 构建移动行列请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<MoveDimensionRequest> {
        // 验证必需参数
        match (
            &self.spreadsheet_token,
            &self.sheet_id,
            &self.dimension,
            self.source_start_index,
            self.source_end_index,
            self.destination_index,
        ) {
            (
                Some(_),
                Some(_),
                Some(dimension),
                Some(source_start),
                Some(source_end),
                Some(dest_index),
            ) => {
                // 验证维度类型
                if dimension != "ROWS" && dimension != "COLUMNS" {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "维度必须是ROWS或COLUMNS".to_string(),
                    ));
                }

                // 验证索引范围
                if source_start < 0 || source_end < 0 || dest_index < 0 {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "索引不能为负数".to_string(),
                    ));
                }

                if source_start > source_end {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "源起始索引不能大于源结束索引".to_string(),
                    ));
                }

                // 验证移动范围大小（避免过大的操作）
                let range_size = source_end - source_start + 1;
                if range_size > 1000 {
                    return Err(openlark_core::error::LarkAPIError::IllegalParamError(
                        "单次移动范围不能超过1000行/列".to_string(),
                    ));
                }

                Ok(MoveDimensionRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    sheet_id: self.sheet_id.unwrap(),
                    dimension: dimension.clone(),
                    source_start_index: source_start,
                    source_end_index: source_end,
                    destination_index: dest_index,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID、维度、源索引和目标索引都是必需的".to_string(),
            )),
        }
    }
}

/// 移动行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResponse {
    /// 移动操作结果
    pub data: MoveDimensionResult,
}

/// 移动行列结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResult {
    /// 是否成功
    pub success: bool,
    /// 移动的行数或列数
    #[serde(rename = "movedCount")]
    pub moved_count: i32,
    /// 源起始索引
    #[serde(rename = "sourceStartIndex")]
    pub source_start_index: i32,
    /// 源结束索引
    #[serde(rename = "sourceEndIndex")]
    pub source_end_index: i32,
    /// 目标索引
    #[serde(rename = "destinationIndex")]
    pub destination_index: i32,
    /// 移动维度
    pub dimension: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
}

impl MoveDimensionService {
    /// 创建移动行列服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 移动工作表行列
    ///
    /// 在指定的工作表中移动行或列到目标位置。
    ///
    /// # 参数
    /// - `request`: 移动行列请求，包含移动配置信息
    ///
    /// # 返回
    /// 返回移动操作的结果信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::{MoveDimensionService, MoveDimensionRequest};
    ///
    /// let service = MoveDimensionService::new(client_config);
    ///
    /// // 移动第5-10行到第2行位置
    /// let request = MoveDimensionRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .rows()
    ///     .source_range(5, 10)
    ///     .destination_index(2)
    ///     .build()?;
    ///
    /// let response = service.move_dimension(&request).await?;
    ///
    /// println!("成功移动 {} 行", response.data.moved_count);
    /// println!("从位置 {}-{} 移动到 {}",
    ///     response.data.source_start_index,
    ///     response.data.source_end_index,
    ///     response.data.destination_index);
    /// ```
    ///
    /// # 移动逻辑
    /// - 行列被移动到目标位置后，原本在目标位置的行列会对应右移或下移
    /// - 源范围和目标位置可以重叠
    /// - 移动操作会保持行列中的所有数据和格式
    ///
    /// # 错误处理
    /// - 如果电子表格不存在，返回相应错误
    /// - 如果工作表不存在，返回相应错误
    /// - 如果索引超出范围，返回相应错误
    /// - 如果移动范围过大，返回相应错误
    pub async fn move_dimension(
        &self,
        request: &MoveDimensionRequest,
    ) -> openlark_core::error::SDKResult<Response<MoveDimensionResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/move_dimension",
            self.config.base_url,
            request.spreadsheet_token.as_str(),
            request.sheet_id.as_str()
        );

        // 创建HTTP请求并序列化请求体
        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        // 发送请求并获取响应
        let response =
            Transport::<MoveDimensionResponse>::request(api_request, &self.config, None).await?;

        // 检查响应是否成功
        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        Ok(response)
    }

    /// 移动行构建器
    ///
    /// 提供链式调用的构建器模式，便于快速移动行。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::MoveDimensionService;
    ///
    /// let service = MoveDimensionService::new(client_config);
    ///
    /// let response = service
    ///     .move_rows_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .from_range(3, 7)  // 移动第4-8行
    ///     .to_position(1)    // 移动到第2行位置
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn move_rows_builder(&self) -> MoveDimensionServiceBuilder<'_> {
        MoveDimensionServiceBuilder {
            service: self,
            spreadsheet_token: None,
            sheet_id: None,
            dimension: "ROWS".to_string(),
            source_start_index: None,
            source_end_index: None,
            destination_index: None,
        }
    }

    /// 移动列构建器
    ///
    /// 提供链式调用的构建器模式，便于快速移动列。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v3::MoveDimensionService;
    ///
    /// let service = MoveDimensionService::new(client_config);
    ///
    /// let response = service
    ///     .move_columns_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .sheet_id("0XXXXXXXXXX".to_string())
    ///     .from_range(2, 5)  // 移动第3-6列
    ///     .to_position(0)    // 移动到第1列位置
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn move_columns_builder(&self) -> MoveDimensionServiceBuilder<'_> {
        MoveDimensionServiceBuilder {
            service: self,
            spreadsheet_token: None,
            sheet_id: None,
            dimension: "COLUMNS".to_string(),
            source_start_index: None,
            source_end_index: None,
            destination_index: None,
        }
    }
}

/// 工作表移动服务构建器
///
/// 提供流畅的API用于构建移动行列请求。
pub struct MoveDimensionServiceBuilder<'a> {
    service: &'a MoveDimensionService,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    dimension: String,
    source_start_index: Option<i32>,
    source_end_index: Option<i32>,
    destination_index: Option<i32>,
}

impl<'a> MoveDimensionServiceBuilder<'a> {
    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::from(spreadsheet_token));
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(SheetId::from(sheet_id));
        self
    }

    /// 设置源范围（从起始索引到结束索引）
    pub fn from_range(mut self, start_index: i32, end_index: i32) -> Self {
        self.source_start_index = Some(start_index);
        self.source_end_index = Some(end_index);
        self
    }

    /// 设置目标位置
    pub fn to_position(mut self, destination_index: i32) -> Self {
        self.destination_index = Some(destination_index);
        self
    }

    /// 执行移动操作
    pub async fn execute(self) -> openlark_core::error::SDKResult<Response<MoveDimensionResponse>> {
        match (
            self.spreadsheet_token,
            self.sheet_id,
            self.source_start_index,
            self.source_end_index,
            self.destination_index,
        ) {
            (
                Some(spreadsheet_token),
                Some(sheet_id),
                Some(source_start),
                Some(source_end),
                Some(dest_index),
            ) => {
                let request = MoveDimensionRequest {
                    spreadsheet_token,
                    sheet_id,
                    dimension: self.dimension,
                    source_start_index: source_start,
                    source_end_index: source_end,
                    destination_index: dest_index,
                };

                self.service.move_dimension(&request).await
            }
            _ => Err(openlark_core::error::LarkAPIError::IllegalParamError(
                "电子表格Token、工作表ID、源范围和目标位置都是必需的".to_string(),
            )),
        }
    }
}

// 为响应类型实现 ApiResponseTrait
impl ApiResponseTrait for MoveDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_move_dimension_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = MoveDimensionService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_move_dimension_request_builder() {
        // 测试行移动请求
        let row_request = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(3, 7)
            .destination_index(1)
            .build();

        assert!(row_request.is_ok());
        let req = row_request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.sheet_id.as_str(), "0XXXXXXXXXX");
        assert_eq!(req.dimension, "ROWS");
        assert_eq!(req.source_start_index, 3);
        assert_eq!(req.source_end_index, 7);
        assert_eq!(req.destination_index, 1);

        // 测试列移动请求
        let col_request = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .columns()
            .source_range(0, 2)
            .destination_index(5)
            .build();

        assert!(col_request.is_ok());
        let req = col_request.unwrap();
        assert_eq!(req.dimension, "COLUMNS");
        assert_eq!(req.source_start_index, 0);
        assert_eq!(req.source_end_index, 2);
        assert_eq!(req.destination_index, 5);
    }

    #[test]
    fn test_move_dimension_request_validation() {
        // 测试缺少必需参数
        let invalid_request = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .build();
        assert!(invalid_request.is_err());

        // 测试无效维度
        let invalid_dimension = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .dimension("INVALID".to_string())
            .source_range(0, 5)
            .destination_index(10)
            .build();
        assert!(invalid_dimension.is_err());

        // 测试负数索引
        let negative_index = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(-1, 5)
            .destination_index(10)
            .build();
        assert!(negative_index.is_err());

        // 测试起始索引大于结束索引
        let invalid_range = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(10, 5)
            .destination_index(1)
            .build();
        assert!(invalid_range.is_err());

        // 测试移动范围过大
        let too_large_range = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(0, 1001)
            .destination_index(2000)
            .build();
        assert!(too_large_range.is_err());
    }

    #[test]
    fn test_move_dimension_service_builder() {
        let config = openlark_core::config::Config::default();
        let service = MoveDimensionService::new(config);

        // 测试移动行构建器
        let row_builder = service
            .move_rows_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .from_range(2, 6)
            .to_position(0);

        assert_eq!(
            row_builder.spreadsheet_token.unwrap().as_str(),
            "shtcnmBRWQKbsJRHXXXXXXXXXX"
        );
        assert_eq!(row_builder.sheet_id.unwrap().as_str(), "0XXXXXXXXXX");
        assert_eq!(row_builder.dimension, "ROWS");
        assert_eq!(row_builder.source_start_index, Some(2));
        assert_eq!(row_builder.source_end_index, Some(6));
        assert_eq!(row_builder.destination_index, Some(0));

        // 测试移动列构建器
        let col_builder = service
            .move_columns_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .from_range(1, 3)
            .to_position(5);

        assert_eq!(col_builder.dimension, "COLUMNS");
        assert_eq!(col_builder.source_start_index, Some(1));
        assert_eq!(col_builder.source_end_index, Some(3));
        assert_eq!(col_builder.destination_index, Some(5));
    }

    #[test]
    fn test_move_dimension_edge_cases() {
        // 测试移动单行
        let single_row = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(5, 5) // 移动第6行
            .destination_index(0)
            .build();
        assert!(single_row.is_ok());

        // 测试移动单列
        let single_col = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .columns()
            .source_range(3, 3) // 移动第4列
            .destination_index(0)
            .build();
        assert!(single_col.is_ok());

        // 测试最大允许范围
        let max_range = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(0, 999) // 1000行
            .destination_index(1001)
            .build();
        assert!(max_range.is_ok());
    }

    #[test]
    fn test_complete_move_workflow() {
        let config = openlark_core::config::Config::default();
        let service = MoveDimensionService::new(config);

        // 场景1: 向前移动行（将第10-15行移动到第3行位置）
        let forward_move = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(10, 15)
            .destination_index(3)
            .build()
            .unwrap();

        assert_eq!(forward_move.dimension, "ROWS");
        assert_eq!(forward_move.source_start_index, 10);
        assert_eq!(forward_move.source_end_index, 15);
        assert_eq!(forward_move.destination_index, 3);

        // 场景2: 向后移动列（将第2-4列移动到第10列位置）
        let backward_move = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .columns()
            .source_range(2, 4)
            .destination_index(10)
            .build()
            .unwrap();

        assert_eq!(backward_move.dimension, "COLUMNS");
        assert_eq!(backward_move.source_start_index, 2);
        assert_eq!(backward_move.source_end_index, 4);
        assert_eq!(backward_move.destination_index, 10);

        // 场景3: 相邻移动（将第5行移动到第6行位置）
        let adjacent_move = MoveDimensionRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .sheet_id("0XXXXXXXXXX".to_string())
            .rows()
            .source_range(5, 5)
            .destination_index(6)
            .build()
            .unwrap();

        assert_eq!(adjacent_move.source_start_index, 5);
        assert_eq!(adjacent_move.source_end_index, 5);
        assert_eq!(adjacent_move.destination_index, 6);
    }
}
