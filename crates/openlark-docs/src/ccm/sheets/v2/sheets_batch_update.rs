//! 工作表批量更新服务
//!
//! 提供SHEETS v2工作表批量更新功能，支持：
//! - 批量更新工作表属性
//! - 添加和删除工作表
//! - 修改工作表标题和索引
//! - 设置工作表颜色和隐藏状态
//! - 调整工作表顺序
use serde_json::Value;

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::Endpoints,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use serde_json::Map;

use openlark_core::trait_system::Service;
// use openlark_core::SDKResult;

/// 工作表更新请求类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSheetType {
    /// 添加工作表
    AddSheet,
    /// 更新工作表属性
    UpdateSheetProperties,
    /// 删除工作表
    DeleteSheet,
    /// 复制工作表
    DuplicateSheet,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperties {
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 工作表标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作表索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 工作表颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_color: Option<String>,
    /// 工作表是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// 网格属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_column_count: Option<i32>,
    /// 是否隐藏网格线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_gridlines: Option<bool>,
    /// 默认行高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_row_height: Option<i32>,
    /// 默认列宽
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_column_width: Option<f32>,
}

/// 工作表更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetRequest {
    /// 更新类型
    pub update_type: UpdateSheetType,
    /// 工作表属性
    pub properties: SheetProperties,
    /// 复制源工作表ID（仅在DuplicateSheet时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_sheet_id: Option<String>,
    /// 新工作表标题（仅在AddSheet和DuplicateSheet时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
}

impl UpdateSheetRequest {
    /// 创建添加工作表请求
    pub fn add_sheet(title: impl Into<String>, index: i32) -> Self {
        Self {
            update_type: UpdateSheetType::AddSheet,
            properties: SheetProperties {
                sheet_id: None,
                title: Some(title.into()),
                index: Some(index),
                sheet_color: None,
                hidden: None,
                grid_properties: None,
            },
            source_sheet_id: None,
            new_title: None,
        }
    }

    /// 创建更新工作表属性请求
    pub fn update_properties(sheet_id: impl Into<String>) -> Self {
        Self {
            update_type: UpdateSheetType::UpdateSheetProperties,
            properties: SheetProperties {
                sheet_id: Some(sheet_id.into()),
                title: None,
                index: None,
                sheet_color: None,
                hidden: None,
                grid_properties: None,
            },
            source_sheet_id: None,
            new_title: None,
        }
    }

    /// 创建删除工作表请求
    pub fn delete_sheet(sheet_id: impl Into<String>) -> Self {
        Self {
            update_type: UpdateSheetType::DeleteSheet,
            properties: SheetProperties {
                sheet_id: Some(sheet_id.into()),
                title: None,
                index: None,
                sheet_color: None,
                hidden: None,
                grid_properties: None,
            },
            source_sheet_id: None,
            new_title: None,
        }
    }

    /// 创建复制工作表请求
    pub fn duplicate_sheet(
        source_sheet_id: impl Into<String>,
        new_title: impl Into<String>,
        index: i32,
    ) -> Self {
        Self {
            update_type: UpdateSheetType::DuplicateSheet,
            properties: SheetProperties {
                sheet_id: None,
                title: Some(new_title.into()),
                index: Some(index),
                sheet_color: None,
                hidden: None,
                grid_properties: None,
            },
            source_sheet_id: Some(source_sheet_id.into()),
            new_title: None,
        }
    }

    /// 设置工作表标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        if self.update_type == UpdateSheetType::UpdateSheetProperties {
            self.properties.title = Some(title.into());
        }
        self
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> Self {
        self.properties.index = Some(index);
        self
    }

    /// 设置工作表颜色
    pub fn sheet_color(mut self, color: impl Into<String>) -> Self {
        self.properties.sheet_color = Some(color.into());
        self
    }

    /// 设置隐藏状态
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.properties.hidden = Some(hidden);
        self
    }

    /// 设置网格属性
    pub fn grid_properties(mut self, grid_properties: GridProperties) -> Self {
        self.properties.grid_properties = Some(grid_properties);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证工作表ID（对于需要ID的操作）
        match &self.update_type {
            UpdateSheetType::UpdateSheetProperties | UpdateSheetType::DeleteSheet => {
                if self.properties.sheet_id.is_none()
                    || self.properties.sheet_id.as_ref().unwrap().trim().is_empty()
                {
                    return Err(LarkAPIError::InvalidParameter(
                        "工作表ID不能为空".to_string(),
                    ));
                }
            }
            UpdateSheetType::DuplicateSheet => {
                if self.source_sheet_id.is_none()
                    || self.source_sheet_id.as_ref().unwrap().trim().is_empty()
                {
                    return Err(LarkAPIError::InvalidParameter(
                        "源工作表ID不能为空".to_string(),
                    ));
                }
            }
            UpdateSheetType::AddSheet => {
                if self.properties.title.is_none()
                    || self.properties.title.as_ref().unwrap().trim().is_empty()
                {
                    return Err(LarkAPIError::InvalidParameter(
                        "工作表标题不能为空".to_string(),
                    ));
                }
            }
        }

        // 验证标题长度
        if let Some(title) = &self.properties.title {
            if title.trim().is_empty() {
                return Err(LarkAPIError::InvalidParameter(
                    "工作表标题不能为空".to_string(),
                ));
            }
            if title.len() > 100 {
                return Err(LarkAPIError::InvalidParameter(
                    "工作表标题长度不能超过100个字符".to_string(),
                ));
            }
        }

        // 验证索引范围
        if let Some(index) = self.properties.index {
            if index < 0 || index > 1000 {
                return Err(LarkAPIError::InvalidParameter(
                    "工作表索引必须在0-1000之间".to_string(),
                ));
            }
        }

        // 验证新标题长度（用于复制操作）
        if let Some(new_title) = &self.new_title {
            if new_title.trim().is_empty() {
                return Err(LarkAPIError::InvalidParameter(
                    "新工作表标题不能为空".to_string(),
                ));
            }
            if new_title.len() > 100 {
                return Err(LarkAPIError::InvalidParameter(
                    "新工作表标题长度不能超过100个字符".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 工作表批量更新请求
#[derive(Clone, Debug)]
pub struct SheetsBatchUpdateRequest {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 更新请求列表
    pub requests: Vec<UpdateSheetRequest>,
}

impl SheetsBatchUpdateRequest {
    /// 创建新的批量更新请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            requests: vec![],
        }
    }

    /// 添加更新请求
    pub fn add_request(mut self, request: UpdateSheetRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// 批量添加更新请求
    pub fn add_requests(mut self, requests: Vec<UpdateSheetRequest>) -> Self {
        self.requests.extend(requests);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格令牌
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "电子表格令牌不能为空".to_string(),
            ));
        }

        // 验证请求列表
        if self.requests.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "更新请求列表不能为空".to_string(),
            ));
        }

        if self.requests.len() > 100 {
            return Err(LarkAPIError::InvalidParameter(
                "批量更新请求数量不能超过100个".to_string(),
            ));
        }

        // 验证每个更新请求
        for (index, request) in self.requests.iter().enumerate() {
            if let Err(e) = request.validate() {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "更新请求{}验证失败: {}",
                    index + 1,
                    e
                )));
            }
        }

        Ok(())
    }

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut map = Map::new();

        // 转换更新请求列表
        let requests_value: Vec<Value> = self
            .requests
            .iter()
            .map(|req| {
                serde_json::to_value(req).map_err(|e| {
                    LarkAPIError::InvalidParameter(format!("更新请求序列化失败: {}", e))
                })
            })
            .collect::<Result<_, _>>()?;

        map.insert("requests".to_string(), Value::Array(requests_value));

        Ok(Value::Object(map))
    }
}

/// 更新响应中的工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedSheet {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 工作表是否隐藏
    pub hidden: Option<bool>,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
}

/// 工作表批量更新响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetsBatchUpdateResponse {
    /// 更新的工作表列表
    pub updated_sheets: Option<Vec<UpdatedSheet>>,
    /// 操作统计信息
    pub statistics: Option<BatchUpdateStatistics>,
}

/// 批量更新统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStatistics {
    /// 总请求数
    pub total_requests: i32,
    /// 成功请求数
    pub successful_requests: i32,
    /// 失败请求数
    pub failed_requests: i32,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetsBatchUpdateResponseBody {
    /// 批量更新结果
    pub data: SheetsBatchUpdateResponse,
}

// 移除重复的BaseResponse定义，使用openlark_core中的版本

/// 工作表批量更新服务
#[derive(Clone, Debug)]
pub struct SheetsBatchUpdateService {
    config: Config,
}

impl SheetsBatchUpdateService {
    /// 创建工作表批量更新服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量更新工作表
    ///
    /// # 参数
    /// - `request`: 批量更新请求
    ///
    /// # 返回
    /// 更新结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::sheets_batch_update::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SheetsBatchUpdateService::new(config);
    ///
    /// // 创建批量更新请求
    /// let batch_request = SheetsBatchUpdateRequest::new("spreadsheet_token")
    ///     .add_request(UpdateSheetRequest::add_sheet("新工作表", 0))
    ///     .add_request(UpdateSheetRequest::update_properties("sheet_id")
    ///         .title("重命名的工作表")
    ///         .sheet_color("#FF0000"));
    ///
    /// let response = service.batch_update(batch_request).await?;
    /// ```
    pub async fn batch_update(
        &self,
        request: SheetsBatchUpdateRequest,
    ) -> SDKResult<SheetsBatchUpdateResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 发送HTTP请求
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update",
            self.config.base_url, request.spreadsheet_token
        );

        let response = self
            .config
            .transport
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: Response<SheetsBatchUpdateResponseBody> = response
                .json()
                .await
                .map_err(|e| LarkAPIError::JsonParseError(format!("响应解析失败: {}", e)))?;

            if base_response.code == 0 {
                Ok(base_response.data.data)
            } else {
                Err(LarkAPIError::APIError(
                    base_response.code,
                    base_response.msg,
                ))
            }
        } else {
            Err(LarkAPIError::HTTPError(
                response.status().as_u16(),
                "批量更新工作表失败".to_string(),
            ))
        }
    }

    /// 创建工作表批量更新构建器
    pub fn batch_update_builder(&self, spreadsheet_token: &str) -> SheetsBatchUpdateBuilder {
        SheetsBatchUpdateBuilder::new(self.clone() spreadsheet_token)
    }
}

impl Service for SheetsBatchUpdateService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsBatchUpdateService"
    }
}

/// 工作表批量更新构建器
pub struct SheetsBatchUpdateBuilder {
    service: SheetsBatchUpdateService,
    spreadsheet_token: String,
    requests: Vec<UpdateSheetRequest>,
}

impl SheetsBatchUpdateBuilder {
    /// 创建新的构建器
    pub fn new(service: SheetsBatchUpdateService, spreadsheet_token: &str) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.to_string(),
            requests: vec![],
        }
    }

    /// 添加工作表
    pub fn add_sheet(mut self, title: impl Into<String>, index: i32) -> Self {
        let request = UpdateSheetRequest::add_sheet(title, index);
        self.requests.push(request);
        self
    }

    /// 添加带颜色的工作表
    pub fn add_colored_sheet(
        mut self,
        title: impl Into<String>,
        index: i32,
        color: impl Into<String>,
    ) -> Self {
        let request = UpdateSheetRequest::add_sheet(title, index).sheet_color(color);
        self.requests.push(request);
        self
    }

    /// 添加隐藏的工作表
    pub fn add_hidden_sheet(mut self, title: impl Into<String>, index: i32) -> Self {
        let request = UpdateSheetRequest::add_sheet(title, index).hidden(true);
        self.requests.push(request);
        self
    }

    /// 更新工作表属性
    pub fn update_sheet(mut self, sheet_id: impl Into<String>) -> UpdateSheetBuilder {
        UpdateSheetBuilder {
            batch_builder: self,
            sheet_id: sheet_id.into(),
        }
    }

    /// 删除工作表
    pub fn delete_sheet(mut self, sheet_id: impl Into<String>) -> Self {
        let request = UpdateSheetRequest::delete_sheet(sheet_id);
        self.requests.push(request);
        self
    }

    /// 复制工作表
    pub fn duplicate_sheet(
        mut self,
        source_sheet_id: impl Into<String>,
        new_title: impl Into<String>,
        index: i32,
    ) -> Self {
        let request = UpdateSheetRequest::duplicate_sheet(source_sheet_id, new_title, index);
        self.requests.push(request);
        self
    }

    /// 添加自定义更新请求
    pub fn add_request(mut self, request: UpdateSheetRequest) -> Self {
        self.requests.push(request);
        self
    }

    /// 执行批量更新
    pub async fn execute(self) -> SDKResult<SheetsBatchUpdateResponse> {
        let batch_request = SheetsBatchUpdateRequest {
            spreadsheet_token: self.spreadsheet_token,
            requests: self.requests,
        };

        self.service.batch_update(batch_request).await
    }
}

/// 工作表属性更新构建器
pub struct UpdateSheetBuilder {
    batch_builder: SheetsBatchUpdateBuilder,
    sheet_id: String,
}

impl UpdateSheetBuilder {
    /// 设置工作表标题
    pub fn title(mut self, title: impl Into<String>) -> SheetsBatchUpdateBuilder {
        let request = UpdateSheetRequest::update_properties(&self.sheet_id).title(title);
        self.batch_builder.requests.push(request);
        self.batch_builder
    }

    /// 设置工作表索引
    pub fn index(mut self, index: i32) -> SheetsBatchUpdateBuilder {
        let request = UpdateSheetRequest::update_properties(&self.sheet_id).index(index);
        self.batch_builder.requests.push(request);
        self.batch_builder
    }

    /// 设置工作表颜色
    pub fn sheet_color(mut self, color: impl Into<String>) -> SheetsBatchUpdateBuilder {
        let request = UpdateSheetRequest::update_properties(&self.sheet_id).sheet_color(color);
        self.batch_builder.requests.push(request);
        self.batch_builder
    }

    /// 设置隐藏状态
    pub fn hidden(mut self, hidden: bool) -> SheetsBatchUpdateBuilder {
        let request = UpdateSheetRequest::update_properties(&self.sheet_id).hidden(hidden);
        self.batch_builder.requests.push(request);
        self.batch_builder
    }

    /// 设置网格属性
    pub fn grid_properties(mut self, grid_properties: GridProperties) -> SheetsBatchUpdateBuilder {
        let request =
            UpdateSheetRequest::update_properties(&self.sheet_id).grid_properties(grid_properties);
        self.batch_builder.requests.push(request);
        self.batch_builder
    }

    /// 同时设置多个属性
    pub fn properties(
        mut self,
        title: Option<String>,
        index: Option<i32>,
        sheet_color: Option<String>,
        hidden: Option<bool>,
    ) -> SheetsBatchUpdateBuilder {
        let mut request = UpdateSheetRequest::update_properties(&self.sheet_id);

        if let Some(t) = title {
            request = request.title(t);
        }
        if let Some(i) = index {
            request = request.index(i);
        }
        if let Some(c) = sheet_color {
            request = request.sheet_color(c);
        }
        if let Some(h) = hidden {
            request = request.hidden(h);
        }

        self.batch_builder.requests.push(request);
        self.batch_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_sheet_request_add_sheet() {
        let request = UpdateSheetRequest::add_sheet("测试工作表", 0);
        assert!(matches!(request.update_type, UpdateSheetType::AddSheet));
        assert_eq!(request.properties.title.as_ref().unwrap(), "测试工作表");
        assert_eq!(request.properties.index.unwrap(), 0);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_sheet_request_invalid_title() {
        let request = UpdateSheetRequest::add_sheet("", 0);
        assert!(request.validate().is_err());

        let long_title = "a".repeat(101);
        let request = UpdateSheetRequest::add_sheet(long_title, 0);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_sheet_request_invalid_index() {
        let request = UpdateSheetRequest::add_sheet("测试", -1);
        assert!(request.validate().is_err());

        let request = UpdateSheetRequest::add_sheet("测试", 1001);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_sheet_request_update_properties() {
        let request = UpdateSheetRequest::update_properties("sheet123")
            .title("新标题")
            .index(1)
            .sheet_color("#FF0000")
            .hidden(true);

        assert!(matches!(
            request.update_type,
            UpdateSheetType::UpdateSheetProperties
        ));
        assert_eq!(request.properties.sheet_id.as_ref().unwrap(), "sheet123");
        assert_eq!(request.properties.title.as_ref().unwrap(), "新标题");
        assert_eq!(request.properties.index.unwrap(), 1);
        assert_eq!(request.properties.sheet_color.as_ref().unwrap(), "#FF0000");
        assert_eq!(request.properties.hidden.unwrap(), true);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_sheet_request_delete_sheet() {
        let request = UpdateSheetRequest::delete_sheet("sheet123");
        assert!(matches!(request.update_type, UpdateSheetType::DeleteSheet));
        assert_eq!(request.properties.sheet_id.as_ref().unwrap(), "sheet123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_sheet_request_duplicate_sheet() {
        let request = UpdateSheetRequest::duplicate_sheet("source_sheet", "复制的工作表", 2);
        assert!(matches!(
            request.update_type,
            UpdateSheetType::DuplicateSheet
        ));
        assert_eq!(request.source_sheet_id.as_ref().unwrap(), "source_sheet");
        assert_eq!(request.properties.title.as_ref().unwrap(), "复制的工作表");
        assert_eq!(request.properties.index.unwrap(), 2);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_sheets_batch_update_request() {
        let batch_request = SheetsBatchUpdateRequest::new("test_token")
            .add_request(UpdateSheetRequest::add_sheet("工作表1", 0))
            .add_request(UpdateSheetRequest::add_sheet("工作表2", 1))
            .add_request(
                UpdateSheetRequest::update_properties("sheet1")
                    .title("重命名")
                    .sheet_color("#FF0000"),
            );

        assert_eq!(batch_request.spreadsheet_token, "test_token");
        assert_eq!(batch_request.requests.len(), 3);
        assert!(batch_request.validate().is_ok());
    }

    #[test]
    fn test_batch_update_validation() {
        // 测试空令牌
        let batch_request =
            SheetsBatchUpdateRequest::new("").add_request(UpdateSheetRequest::add_sheet("测试", 0));
        assert!(batch_request.validate().is_err());

        // 测试空请求列表
        let batch_request = SheetsBatchUpdateRequest::new("test_token");
        assert!(batch_request.validate().is_err());

        // 测试请求数量超限
        let mut batch_request = SheetsBatchUpdateRequest::new("test_token");
        for i in 0..101 {
            batch_request =
                batch_request.add_request(UpdateSheetRequest::add_sheet(format!("工作表{}", i), i));
        }
        assert!(batch_request.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = SheetsBatchUpdateService::new(config);

        let grid_props = GridProperties {
            frozen_row_count: Some(1),
            frozen_column_count: Some(1),
            hide_gridlines: Some(false),
            default_row_height: Some(25),
            default_column_width: Some(100.0),
        };

        let builder = service
            .batch_update_builder("spreadsheet_token")
            .add_sheet("工作表1", 0)
            .add_colored_sheet("工作表2", 1, "#FF0000")
            .add_hidden_sheet("隐藏工作表", 2)
            .update_sheet("sheet1")
            .title("重命名的工作表")
            .index(0)
            .sheet_color("#4CAF50")
            .grid_properties(grid_props)
            .duplicate_sheet("source_sheet", "复制的工作表", 3)
            .delete_sheet("old_sheet");

        assert_eq!(builder.requests.len(), 6);
        assert_eq!(builder.spreadsheet_token, "spreadsheet_token");

        // 验证第一个请求是添加工作表
        assert!(matches!(
            builder.requests[0].update_type,
            UpdateSheetType::AddSheet
        ));
        assert_eq!(
            builder.requests[0].properties.title.as_ref().unwrap(),
            "工作表1"
        );

        // 验证第二个请求是有颜色的添加工作表
        assert!(matches!(
            builder.requests[1].update_type,
            UpdateSheetType::AddSheet
        ));
        assert_eq!(
            builder.requests[1].properties.sheet_color.as_ref().unwrap(),
            "#FF0000"
        );

        // 验证第三个请求是隐藏的添加工作表
        assert!(matches!(
            builder.requests[2].update_type,
            UpdateSheetType::AddSheet
        ));
        assert_eq!(builder.requests[2].properties.hidden.unwrap(), true);

        // 验证第四个请求是更新属性（包含多个更新）
        assert!(matches!(
            builder.requests[3].update_type,
            UpdateSheetType::UpdateSheetProperties
        ));
        assert_eq!(
            builder.requests[3].properties.title.as_ref().unwrap(),
            "重命名的工作表"
        );

        // 验证第五个请求是复制工作表
        assert!(matches!(
            builder.requests[4].update_type,
            UpdateSheetType::DuplicateSheet
        ));
        assert_eq!(
            builder.requests[4].source_sheet_id.as_ref().unwrap(),
            "source_sheet"
        );

        // 验证第六个请求是删除工作表
        assert!(matches!(
            builder.requests[5].update_type,
            UpdateSheetType::DeleteSheet
        ));
    }

    #[test]
    fn test_complex_batch_update() {
        let config = openlark_core::config::Config::default();
        let service = SheetsBatchUpdateService::new(config);

        let grid_props = GridProperties {
            frozen_row_count: Some(2),
            frozen_column_count: Some(1),
            hide_gridlines: Some(true),
            default_row_height: Some(30),
            default_column_width: Some(120.0),
        };

        let batch_request = SheetsBatchUpdateRequest::new("spreadsheet_token")
            .add_request(
                UpdateSheetRequest::add_sheet("数据表", 0)
                    .sheet_color("#2196F3")
                    .grid_properties(grid_props.clone()),
            )
            .add_request(
                UpdateSheetRequest::add_sheet("统计表", 1)
                    .sheet_color("#4CAF50")
                    .grid_properties(grid_props),
            )
            .add_request(
                UpdateSheetRequest::update_properties("existing_sheet")
                    .title("重命名的旧表")
                    .index(2)
                    .sheet_color("#FF9800")
                    .hidden(false),
            )
            .add_request(UpdateSheetRequest::duplicate_sheet(
                "template_sheet",
                "基于模板的表",
                3,
            ))
            .add_request(UpdateSheetRequest::delete_sheet("obsolete_sheet"));

        assert!(batch_request.validate().is_ok());

        let body = batch_request.to_request_body().unwrap();
        assert!(body["requests"].is_array());
        assert_eq!(body["requests"].as_array().unwrap().len(), 5);
    }

    #[test]
    fn test_sheets_batch_update_service() {
        let config = openlark_core::config::Config::default();
        let service = SheetsBatchUpdateService::new(config);

        assert_eq!(service.service_name(), "SheetsBatchUpdateService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_grid_properties() {
        let grid_props = GridProperties {
            frozen_row_count: Some(1),
            frozen_column_count: Some(2),
            hide_gridlines: Some(true),
            default_row_height: Some(25),
            default_column_width: Some(100.5),
        };

        let serialized = serde_json::to_value(&grid_props).unwrap();
        assert_eq!(serialized["frozen_row_count"], 1);
        assert_eq!(serialized["frozen_column_count"], 2);
        assert_eq!(serialized["hide_gridlines"], true);
        assert_eq!(serialized["default_row_height"], 25);
        assert_eq!(serialized["default_column_width"], 100.5);
    }

    #[test]
    fn test_update_sheet_builder_with_combined_properties() {
        let config = openlark_core::config::Config::default();
        let service = SheetsBatchUpdateService::new(config);

        let builder = service
            .batch_update_builder("token")
            .update_sheet("sheet123")
            .properties(
                Some("新标题".to_string()),
                Some(1),
                Some("#FF0000".to_string()),
                Some(false),
            );

        assert_eq!(builder.requests.len(), 1);
        let request = &builder.requests[0];
        assert_eq!(request.properties.title.as_ref().unwrap(), "新标题");
        assert_eq!(request.properties.index.unwrap(), 1);
        assert_eq!(request.properties.sheet_color.as_ref().unwrap(), "#FF0000");
        assert_eq!(request.properties.hidden.unwrap(), false);
    }
}
