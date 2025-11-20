//! Sheets v2 表格导入服务
//!
//! 提供飞书电子表格v2版本的表格导入功能，支持：
//! - 多种文件格式导入（CSV、Excel等）
//! - 异步任务处理
//! - 导入进度跟踪
//! - 错误处理和结果查询

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 导入表格请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportRequest {
    /// 文件内容（Base64编码）
    pub file: String,
    /// 文件名称
    pub file_name: String,
    /// 文件类型（csv、xlsx、xls等）
    pub file_type: String,
    /// 目标文件夹token（可选）
    pub folder_token: Option<String>,
    /// 导入选项
    pub options: Option<ImportOptions>,
}

/// 导入选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportOptions {
    /// 工作表名称
    pub sheet_name: Option<String>,
    /// 是否跳过空行
    pub skip_empty_rows: Option<bool>,
    /// 编码格式
    pub encoding: Option<String>,
    /// 分隔符（CSV文件）
    pub delimiter: Option<String>,
    /// 是否包含表头
    pub header: Option<bool>,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            sheet_name: None,
            skip_empty_rows: Some(true),
            encoding: Some("UTF-8".to_string()),
            delimiter: Some(",".to_string()),
            header: Some(true),
        }
    }
}

/// 导入表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    /// 导入任务数据
    pub data: ImportData,
}

/// 导入任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    /// 任务票据，用于查询导入结果
    pub ticket: String,
    /// 任务状态
    pub status: String,
}

impl Default for ImportResponse {
    fn default() -> Self {
        Self {
            data: ImportData {
                ticket: String::new(),
                status: String::new(),
            },
        }
    }
}

impl ApiResponseTrait for ImportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入服务
#[derive(Clone, Debug)]
pub struct ImportService {
    config: Config,
}

impl ImportService {
    /// 创建新的导入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 导入表格
    ///
    /// # 参数
    /// - `request`: 导入请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::import::{ImportRequest, ImportOptions, ImportService};
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ImportService::new(config);
    ///
    /// let options = ImportOptions {
    ///     sheet_name: Some("工作表1".to_string()),
    ///     skip_empty_rows: Some(true),
    ///     encoding: Some("UTF-8".to_string()),
    ///     delimiter: Some(",".to_string()),
    ///     header: Some(true),
    /// };
    ///
    /// let request = ImportRequest {
    ///     file: "base64编码的文件内容".to_string(),
    ///     file_name: "data.csv".to_string(),
    ///     file_type: "csv".to_string(),
    ///     folder_token: Some("folder_token".to_string()),
    ///     options: Some(options),
    /// };
    ///
    /// let result = service.import(&request);
    /// ```
    pub async fn import(&self, request: &ImportRequest) -> SDKResult<ImportResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = "/open-apis/sheets/v2/import";

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let import_response: StandardResponse<ImportResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = import_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }
}

impl openlark_core::core::trait_system::Service for ImportService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ImportService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_import_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ImportService::new(config);
        assert_eq!(service.service_name(), "ImportService");
    }

    #[test]
    fn test_import_request_default() {
        let request = ImportRequest {
            file: "dGVzdCBjb250ZW50".to_string(), // base64编码的"test content"
            file_name: "test.csv".to_string(),
            file_type: "csv".to_string(),
            folder_token: None,
            options: Some(ImportOptions::default()),
        };

        assert_eq!(request.file_name, "test.csv");
        assert_eq!(request.file_type, "csv");
        assert!(request.options.is_some());

        let options = request.options.unwrap();
        assert_eq!(options.skip_empty_rows, Some(true));
        assert_eq!(options.encoding, Some("UTF-8".to_string()));
        assert_eq!(options.delimiter, Some(",".to_string()));
        assert_eq!(options.header, Some(true));
    }

    #[test]
    fn test_import_response_default() {
        let response = ImportResponse::default();
        assert_eq!(response.data.ticket, "");
        assert_eq!(response.data.status, "");
    }

    #[test]
    fn test_import_options_custom() {
        let options = ImportOptions {
            sheet_name: Some("自定义工作表".to_string()),
            skip_empty_rows: Some(false),
            encoding: Some("GBK".to_string()),
            delimiter: Some(";".to_string()),
            header: Some(false),
        };

        assert_eq!(options.sheet_name, Some("自定义工作表".to_string()));
        assert_eq!(options.skip_empty_rows, Some(false));
        assert_eq!(options.encoding, Some("GBK".to_string()));
        assert_eq!(options.delimiter, Some(";".to_string()));
        assert_eq!(options.header, Some(false));
    }

    #[test]
    fn test_import_request_serialization() {
        let request = ImportRequest {
            file: "dGVzdCBjb250ZW50".to_string(),
            file_name: "test.xlsx".to_string(),
            file_type: "xlsx".to_string(),
            folder_token: Some("folder_token_123".to_string()),
            options: Some(ImportOptions::default()),
        };

        // 测试序列化是否正常
        let json_str = serde_json::to_string(&request);
        assert!(json_str.is_ok());

        let parsed: Result<ImportRequest, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_api_response_trait() {
        assert_eq!(ImportResponse::data_format(), ResponseFormat::Data);
    }
}
