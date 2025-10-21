use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{
        spreadsheet_sheet::UpdateSheetProperty, SpreadsheetSheetService,
    },
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OperateSheetsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 支持增加、复制、和删除工作表。一次请求可以同时进行多个操作。
    requests: Vec<OperateSheetsRequestElem>,
}

#[derive(Serialize, Deserialize, Debug)]
/// 表格操作请求元素类型
///
/// 定义对表格进行操作的不同请求类型
pub enum OperateSheetsRequestElem {
    /// 增加工作表。
    #[serde(rename = "addSheet")]
    AddSheet {
        /// 工作表属性
        properties: AddSheetProperty,
    },
    /// 复制工作表。复制的新工作表位于源工作表索引位置之后。
    #[serde(rename = "copySheet")]
    CopySheet {
        /// 需要复制的工作表资源
        source: CopySheetSource,
        /// 新工作表的属性
        destination: CopySheetDestination,
    },
    /// 更新工作表
    #[serde(rename = "updateSheet")]
    UpdateSheet {
        /// 工作表属性
        properties: UpdateSheetProperty,
    },
    /// 删除工作表。
    #[serde(rename = "deleteSheet")]
    DeleteSheet {
        /// 要删除的工作表的 ID。调用获取工作表获取 ID
        #[serde(rename = "sheetId")]
        sheet_id: String,
    },
}

/// 工作表属性
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddSheetProperty {
    /// 新增工作表的标题
    pub title: String,
    /// 新增工作表的位置。不填默认在工作表的第 0 索引位置增加工作表。
    pub index: Option<i32>,
}

/// 需要复制的工作表资源
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopySheetSource {
    /// 源工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    sheet_id: String,
}

/// 新工作表的属性
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopySheetDestination {
    /// 新工作表名称。不填默认为“源工作表名称”+“(副本_源工作表的 index 值)”，如 “Sheet1(副本_0)”。
    title: Option<String>,
}

impl OperateSheetsRequest {
    pub fn builder() -> OperateSheetsRequestBuilder {
        OperateSheetsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct OperateSheetsRequestBuilder {
    request: OperateSheetsRequest,
}

impl OperateSheetsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 增加工作表。
    pub fn add_sheet(mut self, title: impl ToString, index: Option<i32>) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::AddSheet {
                properties: AddSheetProperty {
                    title: title.to_string(),
                    index,
                },
            });
        self
    }

    /// 复制工作表。复制的新工作表位于源工作表索引位置之后。
    pub fn copy_sheet(mut self, source: impl ToString, destination: Option<String>) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::CopySheet {
                source: CopySheetSource {
                    sheet_id: source.to_string(),
                },
                destination: CopySheetDestination { title: destination },
            });
        self
    }

    /// 删除工作表。
    pub fn delete_sheet(mut self, sheet_id: impl ToString) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::DeleteSheet {
                sheet_id: sheet_id.to_string(),
            });
        self
    }

    pub fn build(mut self) -> OperateSheetsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    OperateSheetsRequestBuilder,
    SpreadsheetSheetService,
    OperateSheetsRequest,
    BaseResponse<OperateSheetResponse>,
    operate
);

impl SpreadsheetSheetService {
    /// 操作工作表
    /// 新增、复制、删除工作表。
    pub async fn operate(
        &self,
        request: OperateSheetsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<OperateSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp =
            crate::core::http::Transport::request(api_req, &self.config_arc, option).await?;

        Ok(api_resp)
    }
}

#[derive(Deserialize, Debug)]
pub struct OperateSheetResponse {
    pub replies: Vec<OperateSheetReply>,
}

impl ApiResponseTrait for OperateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Deserialize, Debug)]
/// 表格操作响应类型
///
/// 定义表格操作请求的响应结果类型
pub enum OperateSheetReply {
    /// 新增工作表的属性
    #[serde(rename = "addSheet")]
    AddSheet { properties: SheetResponse },
    /// 复制工作表的结果
    #[serde(rename = "copySheet")]
    CopySheet { properties: SheetResponse },
    /// 更新工作表的结果
    #[serde(rename = "updateSheet")]
    UpdateSheet { properties: UpdateSheetProperty },
    /// 删除工作表的结果
    #[serde(rename = "deleteSheet")]
    DeleteSheet {
        /// 删除工作表是否成功
        result: bool,
        /// 被删除的工作表的 ID
        #[serde(rename = "sheetId")]
        sheet_id: String,
    },
}

#[derive(Deserialize, Debug)]
/// 工作表的属性
pub struct SheetResponse {
    /// 工作表的 sheetId
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 工作表的标题
    pub title: String,
    /// 工作表的位置
    pub index: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use rstest::rstest;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_service() -> SpreadsheetSheetService {
        SpreadsheetSheetService::new(create_test_config())
    }

    #[test]
    fn test_operate_sheets_request_builder_creation() {
        let builder = OperateSheetsRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_operate_sheets_request_builder_with_spreadsheet_token() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("test_spreadsheet_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
    }

    #[test]
    fn test_operate_sheets_request_builder_with_add_sheet() {
        let request = OperateSheetsRequest::builder()
            .add_sheet("New Sheet", Some(2))
            .build();

        assert_eq!(request.requests.len(), 1);
        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "New Sheet");
                assert_eq!(properties.index, Some(2));
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_builder_with_add_sheet_no_index() {
        let request = OperateSheetsRequest::builder()
            .add_sheet("Default Position Sheet", None)
            .build();

        assert_eq!(request.requests.len(), 1);
        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "Default Position Sheet");
                assert!(properties.index.is_none());
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_builder_with_copy_sheet() {
        let request = OperateSheetsRequest::builder()
            .copy_sheet("source_sheet_123", Some("Copied Sheet".to_string()))
            .build();

        assert_eq!(request.requests.len(), 1);
        match &request.requests[0] {
            OperateSheetsRequestElem::CopySheet {
                source,
                destination,
            } => {
                assert_eq!(source.sheet_id, "source_sheet_123");
                assert_eq!(destination.title, Some("Copied Sheet".to_string()));
            }
            _ => panic!("Expected CopySheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_builder_with_copy_sheet_no_title() {
        let request = OperateSheetsRequest::builder()
            .copy_sheet("source_sheet_456", None)
            .build();

        assert_eq!(request.requests.len(), 1);
        match &request.requests[0] {
            OperateSheetsRequestElem::CopySheet {
                source,
                destination,
            } => {
                assert_eq!(source.sheet_id, "source_sheet_456");
                assert!(destination.title.is_none());
            }
            _ => panic!("Expected CopySheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_builder_with_delete_sheet() {
        let request = OperateSheetsRequest::builder()
            .delete_sheet("sheet_to_delete_789")
            .build();

        assert_eq!(request.requests.len(), 1);
        match &request.requests[0] {
            OperateSheetsRequestElem::DeleteSheet { sheet_id } => {
                assert_eq!(sheet_id, "sheet_to_delete_789");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_builder_chaining_multiple_operations() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("my_spreadsheet")
            .add_sheet("Sheet 1", Some(0))
            .add_sheet("Sheet 2", Some(1))
            .copy_sheet("existing_sheet", Some("Copy of Existing".to_string()))
            .delete_sheet("old_sheet")
            .build();

        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.requests.len(), 4);

        // Verify each operation
        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "Sheet 1");
                assert_eq!(properties.index, Some(0));
            }
            _ => panic!("Expected AddSheet variant"),
        }

        match &request.requests[1] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "Sheet 2");
                assert_eq!(properties.index, Some(1));
            }
            _ => panic!("Expected AddSheet variant"),
        }

        match &request.requests[2] {
            OperateSheetsRequestElem::CopySheet {
                source,
                destination,
            } => {
                assert_eq!(source.sheet_id, "existing_sheet");
                assert_eq!(destination.title, Some("Copy of Existing".to_string()));
            }
            _ => panic!("Expected CopySheet variant"),
        }

        match &request.requests[3] {
            OperateSheetsRequestElem::DeleteSheet { sheet_id } => {
                assert_eq!(sheet_id, "old_sheet");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_default() {
        let request = OperateSheetsRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_operate_sheets_request_builder_default() {
        let builder = OperateSheetsRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_add_sheet_property_default() {
        let props = AddSheetProperty::default();

        assert_eq!(props.title, "");
        assert!(props.index.is_none());
    }

    #[test]
    fn test_copy_sheet_source_default() {
        let source = CopySheetSource::default();

        assert_eq!(source.sheet_id, "");
    }

    #[test]
    fn test_copy_sheet_destination_default() {
        let destination = CopySheetDestination::default();

        assert!(destination.title.is_none());
    }

    #[test]
    fn test_operate_sheets_request_serialization() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("token123")
            .add_sheet("Test Sheet", Some(1))
            .copy_sheet("source123", Some("Copy Title".to_string()))
            .delete_sheet("delete456")
            .build();

        // Test that the request can be serialized (this validates field names)
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("requests"));
        assert!(json_str.contains("addSheet"));
        assert!(json_str.contains("copySheet"));
        assert!(json_str.contains("deleteSheet"));
        assert!(json_str.contains("Test Sheet"));
        assert!(json_str.contains("source123"));
        assert!(json_str.contains("Copy Title"));
        assert!(json_str.contains("delete456"));
    }

    #[test]
    fn test_operate_sheets_request_serialization_empty() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("token123")
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("requests"));
        assert!(json_str.contains("[]")); // Empty array
    }

    #[test]
    fn test_operate_sheets_request_debug() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("debug_token")
            .add_sheet("Debug Sheet", Some(0))
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("OperateSheetsRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Debug Sheet"));
    }

    #[test]
    fn test_operate_sheets_request_with_empty_strings() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("")
            .add_sheet("", None)
            .copy_sheet("", None)
            .delete_sheet("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.requests.len(), 3);
    }

    #[test]
    fn test_operate_sheets_request_with_special_characters() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .add_sheet("工作表_📋_测试", Some(1))
            .copy_sheet("源工作表_🔗", Some("副本_🎨".to_string()))
            .delete_sheet("删除工作表_🗑️")
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");

        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "工作表_📋_测试");
            }
            _ => panic!("Expected AddSheet variant"),
        }

        match &request.requests[1] {
            OperateSheetsRequestElem::CopySheet {
                source,
                destination,
            } => {
                assert_eq!(source.sheet_id, "源工作表_🔗");
                assert_eq!(destination.title, Some("副本_🎨".to_string()));
            }
            _ => panic!("Expected CopySheet variant"),
        }

        match &request.requests[2] {
            OperateSheetsRequestElem::DeleteSheet { sheet_id } => {
                assert_eq!(sheet_id, "删除工作表_🗑️");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[rstest]
    #[case(-1)] // Negative index
    #[case(0)] // First position
    #[case(1)] // Second position
    #[case(10)] // High index
    #[case(1000)] // Very high index
    fn test_operate_sheets_request_with_various_indices(#[case] index: i32) {
        let request = OperateSheetsRequest::builder()
            .add_sheet("Test Sheet", Some(index))
            .build();

        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.index, Some(index));
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_with_maximum_values() {
        let request = OperateSheetsRequest::builder()
            .add_sheet("Max Index Sheet", Some(i32::MAX))
            .build();

        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.index, Some(i32::MAX));
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_with_minimum_values() {
        let request = OperateSheetsRequest::builder()
            .add_sheet("Min Index Sheet", Some(i32::MIN))
            .build();

        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.index, Some(i32::MIN));
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_api_request_body_serialization() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("body_test_token")
            .add_sheet("Body Test Sheet", Some(0))
            .delete_sheet("delete_me")
            .build();

        // Verify that api_request.body is set correctly
        assert!(!request.api_request.body.is_empty());

        // Verify that the body contains valid JSON
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed.get("requests").is_some());
        let requests = parsed.get("requests").unwrap().as_array().unwrap();
        assert_eq!(requests.len(), 2);

        // Check first request (AddSheet)
        assert!(requests[0].get("addSheet").is_some());
        let add_sheet = requests[0].get("addSheet").unwrap();
        assert!(add_sheet.get("properties").is_some());

        // Check second request (DeleteSheet)
        assert!(requests[1].get("deleteSheet").is_some());
        let delete_sheet = requests[1].get("deleteSheet").unwrap();
        assert_eq!(delete_sheet.get("sheetId").unwrap(), "delete_me");
    }

    #[test]
    fn test_operate_sheets_request_builder_multiple_calls() {
        let mut builder = OperateSheetsRequest::builder();

        // Test that multiple calls to spreadsheet_token override previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");

        // Add multiple operations
        builder = builder.add_sheet("Sheet 1", Some(0));
        builder = builder.add_sheet("Sheet 2", Some(1));

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.requests.len(), 2);
    }

    #[test]
    fn test_spreadsheet_sheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_operate_sheet_response_data_format() {
        assert_eq!(OperateSheetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_operate_sheet_reply_deserialization_add_sheet() {
        let json_response = r#"{
            "addSheet": {
                "properties": {
                    "sheetId": "sheet123",
                    "title": "New Sheet",
                    "index": 1
                }
            }
        }"#;

        let reply: OperateSheetReply = serde_json::from_str(json_response).unwrap();

        match reply {
            OperateSheetReply::AddSheet { properties } => {
                assert_eq!(properties.sheet_id, "sheet123");
                assert_eq!(properties.title, "New Sheet");
                assert_eq!(properties.index, Some(1));
            }
            _ => panic!("Expected AddSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheet_reply_deserialization_copy_sheet() {
        let json_response = r#"{
            "copySheet": {
                "properties": {
                    "sheetId": "copy456",
                    "title": "Copied Sheet",
                    "index": 2
                }
            }
        }"#;

        let reply: OperateSheetReply = serde_json::from_str(json_response).unwrap();

        match reply {
            OperateSheetReply::CopySheet { properties } => {
                assert_eq!(properties.sheet_id, "copy456");
                assert_eq!(properties.title, "Copied Sheet");
                assert_eq!(properties.index, Some(2));
            }
            _ => panic!("Expected CopySheet variant"),
        }
    }

    #[test]
    fn test_operate_sheet_reply_deserialization_delete_sheet() {
        let json_response = r#"{
            "deleteSheet": {
                "result": true,
                "sheetId": "deleted789"
            }
        }"#;

        let reply: OperateSheetReply = serde_json::from_str(json_response).unwrap();

        match reply {
            OperateSheetReply::DeleteSheet { result, sheet_id } => {
                assert!(result);
                assert_eq!(sheet_id, "deleted789");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheet_response_deserialization() {
        let json_response = r#"{
            "replies": [
                {
                    "addSheet": {
                        "properties": {
                            "sheetId": "new123",
                            "title": "Added Sheet",
                            "index": 0
                        }
                    }
                },
                {
                    "deleteSheet": {
                        "result": true,
                        "sheetId": "old456"
                    }
                }
            ]
        }"#;

        let response: OperateSheetResponse = serde_json::from_str(json_response).unwrap();

        assert_eq!(response.replies.len(), 2);

        match &response.replies[0] {
            OperateSheetReply::AddSheet { properties } => {
                assert_eq!(properties.sheet_id, "new123");
                assert_eq!(properties.title, "Added Sheet");
                assert_eq!(properties.index, Some(0));
            }
            _ => panic!("Expected AddSheet variant"),
        }

        match &response.replies[1] {
            OperateSheetReply::DeleteSheet { result, sheet_id } => {
                assert!(result);
                assert_eq!(sheet_id, "old456");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_edge_cases() {
        // Test with very long token
        let long_token = "a".repeat(10000);
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token(&long_token)
            .build();
        assert_eq!(request.spreadsheet_token, long_token);

        // Test with very long sheet titles
        let long_title = "Sheet_".repeat(1000);
        let request = OperateSheetsRequest::builder()
            .add_sheet(&long_title, Some(0))
            .build();

        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, long_title);
            }
            _ => panic!("Expected AddSheet variant"),
        }

        // Test with very long sheet IDs
        let long_sheet_id = "sheet_id_".repeat(500);
        let request = OperateSheetsRequest::builder()
            .copy_sheet(&long_sheet_id, None)
            .delete_sheet(&long_sheet_id)
            .build();

        match &request.requests[0] {
            OperateSheetsRequestElem::CopySheet { source, .. } => {
                assert_eq!(source.sheet_id, long_sheet_id);
            }
            _ => panic!("Expected CopySheet variant"),
        }
    }

    #[test]
    fn test_operate_sheets_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let mut builder = OperateSheetsRequest::builder().spreadsheet_token("memory_test");

        // Add many operations
        for i in 0..100 {
            builder = builder.add_sheet(format!("Sheet_{}", i), Some(i));
            builder = builder.copy_sheet(format!("source_{}", i), Some(format!("copy_{}", i)));
            builder = builder.delete_sheet(format!("delete_{}", i));
        }

        let request = builder.build();

        assert_eq!(request.requests.len(), 300); // 100 * 3 operations

        // Verify a few random operations
        match &request.requests[0] {
            OperateSheetsRequestElem::AddSheet { properties } => {
                assert_eq!(properties.title, "Sheet_0");
                assert_eq!(properties.index, Some(0));
            }
            _ => panic!("Expected AddSheet variant"),
        }

        match &request.requests[299] {
            OperateSheetsRequestElem::DeleteSheet { sheet_id } => {
                assert_eq!(sheet_id, "delete_99");
            }
            _ => panic!("Expected DeleteSheet variant"),
        }
    }

    #[test]
    fn test_sheet_response_debug() {
        let response = SheetResponse {
            sheet_id: "debug_sheet_123".to_string(),
            title: "Debug Sheet".to_string(),
            index: Some(5),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("SheetResponse"));
        assert!(debug_str.contains("debug_sheet_123"));
        assert!(debug_str.contains("Debug Sheet"));
        assert!(debug_str.contains("Some(5)"));
    }

    #[test]
    fn test_operate_sheets_request_unicode_handling() {
        let request = OperateSheetsRequest::builder()
            .spreadsheet_token("令牌_🔑_test")
            .add_sheet("工作表_📋_name", Some(0))
            .copy_sheet("源_🎯", Some("目标_🎪".to_string()))
            .delete_sheet("删除_🗑️")
            .build();

        assert_eq!(request.spreadsheet_token, "令牌_🔑_test");

        // Test serialization works with Unicode
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("工作表_📋_name"));
        assert!(json_str.contains("源_🎯"));
        assert!(json_str.contains("目标_🎪"));
        assert!(json_str.contains("删除_🗑️"));
    }

    #[test]
    fn test_operate_sheets_request_builder_partial_configuration() {
        // Test building with only some operations
        let request1 = OperateSheetsRequest::builder()
            .add_sheet("Only Add", Some(1))
            .build();

        assert_eq!(request1.requests.len(), 1);

        let request2 = OperateSheetsRequest::builder()
            .copy_sheet("source", None)
            .delete_sheet("target")
            .build();

        assert_eq!(request2.requests.len(), 2);

        let request3 = OperateSheetsRequest::builder()
            .spreadsheet_token("test")
            .build();

        assert!(request3.requests.is_empty());
    }
}
