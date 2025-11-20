//! 电子表格创建服务
//!
//! 提供SHEETS v3电子表格创建功能，支持：
//! - 创建新的空白电子表格
//! - 设置电子表格标题和文件夹路径
//! - 设置电子表格时区和语言
//! - 指定初始工作表配置
//! - 自定义电子表格属性
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Map;

use openlark_core::trait_system::Service;
// use openlark_core::SDKResult;

/// 工作表初始配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperty {
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表ID
    pub sheet_id: Option<String>,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 工作表是否隐藏
    pub hidden: Option<bool>,
    /// 网格线是否显示
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结行数
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    pub frozen_column_count: Option<i32>,
    /// 是否隐藏网格线
    pub hide_gridlines: Option<bool>,
}

/// 时区配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeZone {
    /// 时区标识符，如 "Asia/Shanghai"
    pub time_zone: String,
}

/// 语言配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    /// 语言代码，如 "zh_CN"
    pub locale: String,
}

/// 创建电子表格请求
#[derive(Clone, Debug)]
pub struct CreateSpreadsheetRequest {
    /// 电子表格标题
    pub title: String,
    /// 工作表初始配置列表
    pub sheets: Option<Vec<SheetProperty>>,
    /// 时区设置
    pub time_zone: Option<TimeZone>,
    /// 语言设置
    pub locale: Option<Locale>,
    /// 文件夹路径（可选）
    pub folder_path: Option<String>,
    /// 自定义属性
    pub properties: Option<HashMap<String, Value>>,
}

impl Default for CreateSpreadsheetRequest {
    fn default() -> Self {
        Self {
            title: "未命名电子表格".to_string(),
            sheets: None,
            time_zone: None,
            locale: None,
            folder_path: None,
            properties: None,
        }
    }
}

impl CreateSpreadsheetRequest {
    /// 创建新的电子表格创建请求
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            
        }
    }

    /// 设置工作表
    pub fn sheets(mut self, sheets: Vec<SheetProperty>) -> Self {
        self.sheets = Some(sheets);
        self
    }

    /// 添加工作表
    pub fn add_sheet(mut self, sheet: SheetProperty) -> Self {
        match &mut self.sheets {
            Some(sheets) => sheets.push(sheet),
            None => self.sheets = Some(vec![sheet]),
        }
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(TimeZone {
            time_zone: time_zone.into(),
        });
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(Locale {
            locale: locale.into(),
        });
        self
    }

    /// 设置文件夹路径
    pub fn folder_path(mut self, folder_path: impl Into<String>) -> Self {
        self.folder_path = Some(folder_path.into());
        self
    }

    /// 设置自定义属性
    pub fn properties(mut self, properties: HashMap<String, Value>) -> Self {
        self.properties = Some(properties);
        self
    }

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut map = Map::new();

        // 设置标题
        map.insert("title".to_string(), Value::String(self.title.clone()));

        // 设置工作表
        if let Some(sheets) = &self.sheets {
            let sheets_value: Value = serde_json::to_value(sheets).map_err(|e| {
                LarkAPIError::IllegalParamError(format!("工作表配置序列化失败: {}", e))
            })?;
            map.insert("sheets".to_string(), sheets_value);
        }

        // 设置时区
        if let Some(time_zone) = &self.time_zone {
            let tz_value: Value = serde_json::to_value(time_zone).map_err(|e| {
                LarkAPIError::IllegalParamError(format!("时区配置序列化失败: {}", e))
            })?;
            map.insert("time_zone".to_string(), tz_value);
        }

        // 设置语言
        if let Some(locale) = &self.locale {
            let locale_value: Value = serde_json::to_value(locale).map_err(|e| {
                LarkAPIError::IllegalParamError(format!("语言配置序列化失败: {}", e))
            })?;
            map.insert("locale".to_string(), locale_value);
        }

        // 设置文件夹路径
        if let Some(folder_path) = &self.folder_path {
            map.insert(
                "folder_path".to_string(),
                Value::String(folder_path.clone()),
            );
        }

        // 设置自定义属性
        if let Some(properties) = &self.properties {
            let props_value: Value = serde_json::to_value(properties).map_err(|e| {
                LarkAPIError::IllegalParamError(format!("自定义属性序列化失败: {}", e))
            })?;
            map.insert("properties".to_string(), props_value);
        }

        Ok(Value::Object(map))
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证标题
        if self.title.trim().is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格标题不能为空".to_string(),
            ));
        }

        if self.title.len() > 200 {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格标题长度不能超过200个字符".to_string(),
            ));
        }

        // 验证工作表配置
        if let Some(sheets) = &self.sheets {
            if sheets.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "工作表列表不能为空".to_string(),
                ));
            }

            if sheets.len() > 100 {
                return Err(LarkAPIError::IllegalParamError(
                    "工作表数量不能超过100个".to_string(),
                ));
            }

            for (index, sheet) in sheets.iter().enumerate() {
                if sheet.title.trim().is_empty() {
                    return Err(LarkAPIError::IllegalParamError(format!(
                        "工作表{}标题不能为空",
                        index + 1
                    )));
                }

                if sheet.title.len() > 100 {
                    return Err(LarkAPIError::IllegalParamError(format!(
                        "工作表{}标题长度不能超过100个字符",
                        index + 1
                    )));
                }

                if sheet.index < 0 || sheet.index >= 100 {
                    return Err(LarkAPIError::IllegalParamError(format!(
                        "工作表{}索引必须在0-99之间",
                        index + 1
                    )));
                }
            }
        }

        // 验证时区格式
        if let Some(time_zone) = &self.time_zone {
            // 简单的时区格式验证
            if !time_zone.time_zone.contains('/') && !time_zone.time_zone.starts_with("UTC") {
                return Err(LarkAPIError::IllegalParamError(
                    "时区格式不正确，请使用如'Asia/Shanghai'或'UTC+8'格式".to_string(),
                ));
            }
        }

        // 验证语言格式
        if let Some(locale) = &self.locale {
            if !locale.locale.contains('_') && locale.locale.len() != 2 {
                return Err(LarkAPIError::IllegalParamError(
                    "语言格式不正确，请使用如'zh_CN'或'en'格式".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 电子表格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// 电子表格ID
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言
    pub locale: Option<String>,
    /// 所有者
    pub owner: Option<String>,
    /// 是否已删除
    pub is_deleted: Option<bool>,
    /// 修订历史ID
    pub revision_id: Option<String>,
}

/// 创建电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 工作表列表
    pub sheets: Option<Vec<SheetProperty>>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponseBody {
    /// 创建的电子表格信息
    pub data: CreateSpreadsheetResponse,
}

/// 基础API响应
// 使用openlark_core::api::Response，避免重复定义

/// 电子表格创建服务
#[derive(Clone, Debug)]
pub struct SpreadsheetCreateService {
    config: Config,
}

impl SpreadsheetCreateService {
    /// 创建电子表格创建服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建电子表格
    ///
    /// # 参数
    /// - `request`: 创建电子表格请求
    ///
    /// # 返回
    /// 创建的电子表格信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::spreadsheet_create::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpreadsheetCreateService::new(config);
    ///
    /// // 创建简单电子表格
    /// let request = CreateSpreadsheetRequest::new("我的电子表格")
    ///     .time_zone("Asia/Shanghai")
    ///     .locale("zh_CN");
    ///
    /// // 或者创建带工作表的电子表格
    /// let sheet1 = SheetProperty {
    ///     title: "工作表1".to_string(),
    ///     index: 0,
    ///     sheet_id: None,
    ///     sheet_color: None,
    ///     hidden: None,
    ///     grid_properties: None,
    /// };
    ///
    /// let request_with_sheets = CreateSpreadsheetRequest::new("带工作表的电子表格")
    ///     .add_sheet(sheet1)
    ///     .time_zone("Asia/Shanghai")
    ///     .locale("zh_CN");
    /// ```
    pub async fn create(
        &self,
        request: CreateSpreadsheetRequest,
    ) -> SDKResult<CreateSpreadsheetResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 发送HTTP请求
        let url = format!("{}/open-apis/sheets/v3/spreadsheets", self.config.base_url);

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &url);
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?;

        let base_response =
            Transport::<CreateSpreadsheetResponseBody>::request(api_request, &self.config, None)
                .await?;

        // 处理响应
        if base_response.code() == 0 {
            match base_response.data {
                Some(response_body) => Ok(response_body.data),
                None => Err(LarkAPIError::APIError {
                    code: -1,
                    msg: "响应数据为空".to_string(),
                    error: None,
                }),
            }
        } else {
            Err(LarkAPIError::APIError {
                code: base_response.code(),
                msg: base_response.msg().to_string(),
                error: None,
            })
        }
    }

    /// 创建电子表格构建器
    pub fn create_spreadsheet_builder(&self) -> CreateSpreadsheetBuilder {
        CreateSpreadsheetBuilder::new(self.clone())
    }
}

impl Service for SpreadsheetCreateService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SpreadsheetCreateService"
    }
}

/// 电子表格创建构建器
pub struct CreateSpreadsheetBuilder {
    service: SpreadsheetCreateService,
    title: Option<String>,
    sheets: Vec<SheetProperty>,
    time_zone: Option<String>,
    locale: Option<String>,
    folder_path: Option<String>,
    properties: HashMap<String, Value>,
}

impl CreateSpreadsheetBuilder {
    /// 创建新的构建器
    pub fn new(service: SpreadsheetCreateService) -> Self {
        Self {
            service,
            title: None,
            sheets: vec![],
            time_zone: None,
            locale: None,
            folder_path: None,
            properties: HashMap::new(),
        }
    }

    /// 设置电子表格标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 添加工作表
    pub fn add_sheet(mut self, title: impl Into<String>, index: i32) -> Self {
        let sheet = SheetProperty {
            title: title.into(),
            index,
            sheet_id: None,
            sheet_color: None,
            hidden: None,
            grid_properties: None,
        };
        self.sheets.push(sheet);
        self
    }

    /// 添加带颜色的的工作表
    pub fn add_colored_sheet(
        mut self,
        title: impl Into<String>,
        index: i32,
        color: impl Into<String>,
    ) -> Self {
        let sheet = SheetProperty {
            title: title.into(),
            index,
            sheet_id: None,
            sheet_color: Some(color.into()),
            hidden: None,
            grid_properties: None,
        };
        self.sheets.push(sheet);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// 设置文件夹路径
    pub fn folder_path(mut self, folder_path: impl Into<String>) -> Self {
        self.folder_path = Some(folder_path.into());
        self
    }

    /// 添加自定义属性
    pub fn add_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// 执行创建操作
    pub async fn execute(self) -> SDKResult<CreateSpreadsheetResponse> {
        let title = self
            .title
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格标题不能为空".to_string()))?;

        let mut request = CreateSpreadsheetRequest::new(title);

        if !self.sheets.is_empty() {
            request = request.sheets(self.sheets);
        }

        if let Some(time_zone) = self.time_zone {
            request = request.time_zone(time_zone);
        }

        if let Some(locale) = self.locale {
            request = request.locale(locale);
        }

        if let Some(folder_path) = self.folder_path {
            request = request.folder_path(folder_path);
        }

        if !self.properties.is_empty() {
            request = request.properties(self.properties);
        }

        self.service.create(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_spreadsheet_request_validation() {
        // 测试空标题
        let request = CreateSpreadsheetRequest::new("");
        assert!(request.validate().is_err());

        // 测试标题过长
        let long_title = "a".repeat(201);
        let request = CreateSpreadsheetRequest::new(long_title);
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = CreateSpreadsheetRequest::new("正常标题");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_sheet_property_validation() {
        let sheet = SheetProperty {
            title: "".to_string(),
            index: 0,
            sheet_id: None,
            sheet_color: None,
            hidden: None,
            grid_properties: None,
        };

        let request = CreateSpreadsheetRequest::new("测试").add_sheet(sheet);
        assert!(request.validate().is_err());

        // 测试工作表标题过长
        let long_title = "a".repeat(101);
        let sheet = SheetProperty {
            title: long_title,
            index: 0,
            sheet_id: None,
            sheet_color: None,
            hidden: None,
            grid_properties: None,
        };

        let request = CreateSpreadsheetRequest::new("测试").add_sheet(sheet);
        assert!(request.validate().is_err());

        // 测试工作表索引超出范围
        let sheet = SheetProperty {
            title: "正常标题".to_string(),
            index: 100,
            sheet_id: None,
            sheet_color: None,
            hidden: None,
            grid_properties: None,
        };

        let request = CreateSpreadsheetRequest::new("测试").add_sheet(sheet);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_time_zone_validation() {
        // 测试无效时区
        let request = CreateSpreadsheetRequest::new("测试").time_zone("invalid_timezone");
        assert!(request.validate().is_err());

        // 测试有效时区
        let request = CreateSpreadsheetRequest::new("测试").time_zone("Asia/Shanghai");
        assert!(request.validate().is_ok());

        // 测试UTC时区
        let request = CreateSpreadsheetRequest::new("测试").time_zone("UTC+8");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_locale_validation() {
        // 测试无效语言
        let request = CreateSpreadsheetRequest::new("测试").locale("invalid");
        assert!(request.validate().is_err());

        // 测试有效语言
        let request = CreateSpreadsheetRequest::new("测试").locale("zh_CN");
        assert!(request.validate().is_ok());

        // 测试简单语言代码
        let request = CreateSpreadsheetRequest::new("测试").locale("en");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_to_request_body() {
        let request = CreateSpreadsheetRequest::new("测试电子表格")
            .time_zone("Asia/Shanghai")
            .locale("zh_CN");

        let body = request.to_request_body().unwrap();

        assert_eq!(body["title"], "测试电子表格");
        assert_eq!(body["time_zone"]["time_zone"], "Asia/Shanghai");
        assert_eq!(body["locale"]["locale"], "zh_CN");
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetCreateService::new(config);

        let builder = service
            .create_spreadsheet_builder()
            .title("测试电子表格")
            .add_sheet("工作表1", 0)
            .add_colored_sheet("工作表2", 1, "#FF0000")
            .time_zone("Asia/Shanghai")
            .locale("zh_CN");

        // 这里只测试构建器创建，不执行实际请求
        assert_eq!(builder.title.as_ref().unwrap(), "测试电子表格");
        assert_eq!(builder.sheets.len(), 2);
        assert_eq!(builder.sheets[0].title, "工作表1");
        assert_eq!(builder.sheets[1].title, "工作表2");
        assert_eq!(builder.sheets[1].sheet_color.as_ref().unwrap(), "#FF0000");
    }

    #[test]
    fn test_spreadsheet_create_service() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetCreateService::new(config);

        assert_eq!(service.service_name(), "SpreadsheetCreateService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_complex_create_request() {
        let sheet1 = SheetProperty {
            title: "销售数据".to_string(),
            index: 0,
            sheet_id: None,
            sheet_color: Some("#4CAF50".to_string()),
            hidden: Some(false),
            grid_properties: Some(GridProperties {
                frozen_row_count: Some(1),
                frozen_column_count: Some(1),
                hide_gridlines: Some(false),
            }),
        };

        let sheet2 = SheetProperty {
            title: "统计数据".to_string(),
            index: 1,
            sheet_id: None,
            sheet_color: Some("#2196F3".to_string()),
            hidden: Some(false),
            grid_properties: Some(GridProperties {
                frozen_row_count: Some(2),
                frozen_column_count: Some(0),
                hide_gridlines: Some(true),
            }),
        };

        let mut properties = HashMap::new();
        properties.insert(
            "category".to_string(),
            Value::String("销售报表".to_string()),
        );
        properties.insert(
            "department".to_string(),
            Value::String("销售部".to_string()),
        );

        let request = CreateSpreadsheetRequest::new("2024年销售报表")
            .sheets(vec![sheet1, sheet2])
            .time_zone("Asia/Shanghai")
            .locale("zh_CN")
            .folder_path("/销售报表/2024")
            .properties(properties);

        assert!(request.validate().is_ok());

        let body = request.to_request_body().unwrap();
        assert_eq!(body["title"], "2024年销售报表");
        assert!(body["sheets"].is_array());
        assert_eq!(body["sheets"].as_array().unwrap().len(), 2);
        assert_eq!(body["time_zone"]["time_zone"], "Asia/Shanghai");
        assert_eq!(body["locale"]["locale"], "zh_CN");
        assert_eq!(body["folder_path"], "/销售报表/2024");
        assert!(body["properties"].is_object());
    }
}

// ApiResponseTrait implementation for Transport compatibility
impl ApiResponseTrait for CreateSpreadsheetResponseBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
