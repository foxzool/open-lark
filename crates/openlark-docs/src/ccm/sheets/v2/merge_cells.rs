//! 单元格合并服务
//!
//! 提供飞书电子表格v2版本的单元格合并功能，包括：
//! - 合并指定范围的单元格
//! - 取消单元格合并
//! - 查询合并状态
//! - 支持水平、垂直和矩形区域合并

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

/// 合并单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 合并范围
    pub range: String,
    /// 合并类型
    #[serde(rename = "mergeType")]
    pub merge_type: MergeType,
}

/// 合并类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MergeType {
    /// 全部合并
    MergeAll,
    /// 水平合并
    MergeRows,
    /// 垂直合并
    MergeColumns,
}

impl MergeCellsRequest {
    /// 创建新的合并请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: impl Into<String>,
        merge_type: MergeType,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            range: range.into(),
            merge_type,
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

        if self.range.trim().is_empty() {
            return Err("合并范围不能为空".to_string());
        }

        // 验证范围格式 (简单的 A1:B2 格式验证)
        if !is_valid_range_format(&self.range) {
            return Err("合并范围格式无效，应为类似 A1:B2 的格式".to_string());
        }

        Ok(())
    }
}

/// 验证范围格式
fn is_valid_range_format(range: &str) -> bool {
    if range.is_empty() {
        return false;
    }

    // 简单的范围格式验证，支持 A1 或 A1:B2 格式
    let range = range.trim();

    // 检查是否包含冒号（范围格式）或不包含（单个单元格）
    if range.contains(':') {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() != 2 {
            return false;
        }
        // 验证两个部分都是有效的单元格引用
        is_valid_cell_reference(parts[0]) && is_valid_cell_reference(parts[1])
    } else {
        is_valid_cell_reference(range)
    }
}

/// 验证单元格引用格式
fn is_valid_cell_reference(cell_ref: &str) -> bool {
    if cell_ref.is_empty() {
        return false;
    }

    let cell_ref = cell_ref.trim();
    let mut chars = cell_ref.chars();

    // 第一个字符必须是字母
    if !chars.next().map_or(false, |c| c.is_ascii_alphabetic()) {
        return false;
    }

    // 剩余字符应该是字母或数字，但至少要有一个数字
    let mut has_digit = false;
    for c in chars {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if !c.is_ascii_alphabetic() {
            return false;
        }
    }

    has_digit
}

/// 合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResponse {
    /// 响应数据
    pub data: MergeCellsData,
}

/// 合并单元格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 合并的信息
    pub merge_info: MergeInfo,
}

/// 合并信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeInfo {
    /// 合并ID
    #[serde(rename = "mergeId")]
    pub merge_id: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 合并范围
    pub range: String,
    /// 合并类型
    #[serde(rename = "mergeType")]
    pub merge_type: MergeType,
}

impl Default for MergeCellsResponse {
    fn default() -> Self {
        Self {
            data: MergeCellsData {
                spreadsheet_token: String::new(),
                merge_info: MergeInfo {
                    merge_id: String::new(),
                    sheet_id: String::new(),
                    range: String::new(),
                    merge_type: MergeType::MergeAll,
                },
            },
        }
    }
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消合并单元格请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 合并ID
    #[serde(rename = "mergeId")]
    pub merge_id: String,
}

impl UnmergeCellsRequest {
    /// 创建新的取消合并请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        merge_id: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            merge_id: merge_id.into(),
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

        if self.merge_id.trim().is_empty() {
            return Err("合并ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 取消合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsResponse {
    /// 响应数据
    pub data: UnmergeCellsData,
}

/// 取消合并单元格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmergeCellsData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作结果
    pub result: String,
}

impl Default for UnmergeCellsResponse {
    fn default() -> Self {
        Self {
            data: UnmergeCellsData {
                spreadsheet_token: String::new(),
                result: "success".to_string(),
            },
        }
    }
}

impl ApiResponseTrait for UnmergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单元格合并服务
#[derive(Clone, Debug)]
pub struct MergeCellsService {
    config: Config,
}

impl MergeCellsService {
    /// 创建单元格合并服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 合并单元格
    ///
    /// 将指定范围内的单元格合并为一个单元格。
    ///
    /// # 参数
    /// - `request`: 合并请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = MergeCellsRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     "sheet1",
    ///     "A1:C3",
    ///     MergeType::MergeAll
    /// );
    ///
    /// let response = service.merge_cells(request, None).await?;
    /// println!("合并ID: {}", response.data.merge_info.merge_id);
    /// ```
    pub async fn merge_cells(
        &self,
        request: MergeCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<MergeCellsResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/merge_cells",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<MergeCellsResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 取消单元格合并
    ///
    /// 取消指定合并的单元格，将其拆分为原来的多个单元格。
    ///
    /// # 参数
    /// - `request`: 取消合并请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let request = UnmergeCellsRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     "sheet1",
    ///     "merge_123456"
    /// );
    ///
    /// let response = service.unmerge_cells(request, None).await?;
    /// println!("取消合并结果: {}", response.data.result);
    /// ```
    pub async fn unmerge_cells(
        &self,
        request: UnmergeCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UnmergeCellsResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/unmerge_cells",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<UnmergeCellsResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 创建合并单元格构建器
    pub fn merge_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: impl Into<String>,
    ) -> MergeCellsBuilder {
        MergeCellsBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            range.into(),
        )
    }
}

/// 合并单元格构建器
#[derive(Clone, Debug)]
pub struct MergeCellsBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    range: String,
    merge_type: MergeType,
}

impl MergeCellsBuilder {
    /// 创建新的合并构建器实例
    pub fn new(config: Config, spreadsheet_token: String, sheet_id: String, range: String) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            range,
            merge_type: MergeType::MergeAll,
        }
    }

    /// 设置合并类型
    pub fn merge_type(mut self, merge_type: MergeType) -> Self {
        self.merge_type = merge_type;
        self
    }

    /// 设置为全部合并
    pub fn merge_all(mut self) -> Self {
        self.merge_type = MergeType::MergeAll;
        self
    }

    /// 设置为水平合并
    pub fn merge_rows(mut self) -> Self {
        self.merge_type = MergeType::MergeRows;
        self
    }

    /// 设置为垂直合并
    pub fn merge_columns(mut self) -> Self {
        self.merge_type = MergeType::MergeColumns;
        self
    }

    /// 执行合并请求
    pub async fn execute(self) -> SDKResult<Response<MergeCellsResponse>> {
        let request = MergeCellsRequest::new(
            self.spreadsheet_token,
            self.sheet_id,
            self.range,
            self.merge_type,
        );

        let service = MergeCellsService {
            config: self.config,
        };
        service.merge_cells(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_cells_request_creation() {
        let request = MergeCellsRequest::new("test_token", "sheet1", "A1:C3", MergeType::MergeAll);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.range, "A1:C3");
        assert_eq!(request.merge_type, MergeType::MergeAll);
    }

    #[test]
    fn test_merge_cells_request_validation() {
        // 测试正常请求
        let valid_request =
            MergeCellsRequest::new("test_token", "sheet1", "A1:B2", MergeType::MergeAll);
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_token_request =
            MergeCellsRequest::new("", "sheet1", "A1:B2", MergeType::MergeAll);
        assert!(empty_token_request.validate().is_err());

        // 测试空工作表ID
        let empty_sheet_request =
            MergeCellsRequest::new("test_token", "", "A1:B2", MergeType::MergeAll);
        assert!(empty_sheet_request.validate().is_err());

        // 测试空范围
        let empty_range_request =
            MergeCellsRequest::new("test_token", "sheet1", "", MergeType::MergeAll);
        assert!(empty_range_request.validate().is_err());
    }

    #[test]
    fn test_is_valid_cell_reference() {
        // 测试有效的单元格引用
        assert!(is_valid_cell_reference("A1"));
        assert!(is_valid_cell_reference("B10"));
        assert!(is_valid_cell_reference("Z100"));
        assert!(is_valid_cell_reference("AA1"));

        // 测试无效的单元格引用
        assert!(!is_valid_cell_reference(""));
        assert!(!is_valid_cell_reference("1"));
        assert!(!is_valid_cell_reference("A"));
        assert!(!is_valid_cell_reference("A1B2"));
    }

    #[test]
    fn test_is_valid_range_format() {
        // 测试有效的范围格式
        assert!(is_valid_range_format("A1"));
        assert!(is_valid_range_format("A1:B2"));
        assert!(is_valid_range_format("Sheet1!A1:B2"));

        // 测试无效的范围格式
        assert!(!is_valid_range_format(""));
        assert!(!is_valid_range_format("A1:"));
        assert!(!is_valid_range_format(":B2"));
        assert!(!is_valid_range_format("A1::B2"));
    }

    #[test]
    fn test_merge_type_serialization() {
        let merge_all = MergeType::MergeAll;
        let serialized = serde_json::to_string(&merge_all).unwrap();
        assert_eq!(serialized, "\"MERGE_ALL\"");

        let merge_rows = MergeType::MergeRows;
        let serialized = serde_json::to_string(&merge_rows).unwrap();
        assert_eq!(serialized, "\"MERGE_ROWS\"");

        let merge_columns = MergeType::MergeColumns;
        let serialized = serde_json::to_string(&merge_columns).unwrap();
        assert_eq!(serialized, "\"MERGE_COLUMNS\"");
    }

    #[test]
    fn test_unmerge_cells_request() {
        let request = UnmergeCellsRequest::new("test_token", "sheet1", "merge_123456");

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.merge_id, "merge_123456");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unmerge_cells_request_validation() {
        // �正常请求
        let valid_request = UnmergeCellsRequest::new("token", "sheet", "merge_id");
        assert!(valid_request.validate().is_ok());

        // 测试空merge_id
        let empty_merge_id = UnmergeCellsRequest::new("token", "sheet", "");
        assert!(empty_merge_id.validate().is_err());
    }

    #[test]
    fn test_merge_cells_builder() {
        let config = openlark_core::config::Config::default();
        let builder = MergeCellsBuilder::new(
            config.clone()
            "test_token".to_string(),
            "sheet1".to_string(),
            "A1:B2".to_string(),
        )
        .merge_rows();

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.range, "A1:B2");
        assert_eq!(builder.merge_type, MergeType::MergeRows);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_merge_cells_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = MergeCellsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_response_default() {
        let merge_response = MergeCellsResponse::default();
        assert!(merge_response.data.spreadsheet_token.is_empty());
        assert_eq!(
            merge_response.data.merge_info.merge_type,
            MergeType::MergeAll
        );

        let unmerge_response = UnmergeCellsResponse::default();
        assert!(unmerge_response.data.spreadsheet_token.is_empty());
        assert_eq!(unmerge_response.data.result, "success");
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(MergeCellsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UnmergeCellsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_request =
            MergeCellsRequest::new("test_token", "sheet1", "A1:C3", MergeType::MergeColumns);

        let serialized = serde_json::to_string(&original_request).unwrap();
        let deserialized: MergeCellsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            original_request.spreadsheet_token,
            deserialized.spreadsheet_token
        );
        assert_eq!(original_request.sheet_id, deserialized.sheet_id);
        assert_eq!(original_request.range, deserialized.range);
        assert_eq!(original_request.merge_type, deserialized.merge_type);
    }

    #[test]
    fn test_builder_merge_type_methods() {
        let config = openlark_core::config::Config::default();

        let all_merge_builder = MergeCellsBuilder::new(
            config.clone()
            "token".to_string(),
            "sheet".to_string(),
            "A1:A1".to_string(),
        )
        .merge_all();
        assert_eq!(all_merge_builder.merge_type, MergeType::MergeAll);

        let rows_merge_builder = MergeCellsBuilder::new(
            config.clone()
            "token".to_string(),
            "sheet".to_string(),
            "A1:A1".to_string(),
        )
        .merge_rows();
        assert_eq!(rows_merge_builder.merge_type, MergeType::MergeRows);

        let columns_merge_builder = MergeCellsBuilder::new(
            config,
            "token".to_string(),
            "sheet".to_string(),
            "A1:A1".to_string(),
        )
        .merge_columns();
        assert_eq!(columns_merge_builder.merge_type, MergeType::MergeColumns);
    }

    #[test]
    fn test_boundary_values() {
        // 测试边界情况
        let single_cell = "A1";
        assert!(is_valid_range_format(single_cell));

        let large_range = "A1:Z1000";
        assert!(is_valid_range_format(large_range));

        // 测试复杂单元格引用
        let complex_cell = "AA100";
        assert!(is_valid_cell_reference(complex_cell));
    }
}
