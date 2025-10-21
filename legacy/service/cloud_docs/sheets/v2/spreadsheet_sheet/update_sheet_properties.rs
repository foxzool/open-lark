use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType,
        endpoints::cloud_docs::*, req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{
        spreadsheet_sheet::{OperateSheetResponse, OperateSheetsRequestElem},
        SpreadsheetSheetService,
    },
};

#[derive(Serialize, Debug, Default)]
pub struct UpdateSheetPropertiesRequest {
    #[serde(skip)]
    api_request: ApiRequest,

    /// 用户 ID 类型。默认值为 open_id。可选值有：
    ///
    /// * open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    ///
    /// * union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID
    #[serde(skip)]
    user_id_type: Option<String>,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 更新工作表属性的请求
    requests: Vec<OperateSheetsRequestElem>,
}

/// 工作表属性
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateSheetProperty {
    /// 要更新的工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 工作表的标题
    pub title: String,
    /// 工作表的位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 是否要隐藏表格。默认值为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// 要冻结的工作表的行数。小于或等于工作表的最大行数，0 表示取消冻结行
    #[serde(rename = "frozenColCount", skip_serializing_if = "Option::is_none")]
    pub frozen_col_count: Option<i32>,
    /// 要冻结的工作表的列数。小于等于工作表的最大列数，0 表示取消冻结列
    #[serde(rename = "frozenRowCount", skip_serializing_if = "Option::is_none")]
    pub frozen_row_count: Option<i32>,
    /// 是否要保护该工作表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect: Option<UpdateSheetPropertyProtect>,
}

/// 是否要保护该工作表
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSheetPropertyProtect {
    /// 是否要保护该工作表。可选值：
    ///
    /// LOCK：保护
    /// UNLOCK：取消保护
    pub lock: bool,
    /// 保护工作表的备注信息
    #[serde(rename = "lockInfo", skip_serializing_if = "Option::is_none")]
    pub lock_info: Option<String>,
    /// 除了本人与所有者外，添加其他人员的 ID，为其它人员添加保护范围的编辑权限。ID 类型由查询参数
    /// user_id_type 决定。user_id_type 不为空时，该字段生效。
    #[serde(rename = "userIDs", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl UpdateSheetPropertiesRequest {
    pub fn builder() -> UpdateSheetPropertiesRequestBuilder {
        UpdateSheetPropertiesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateSheetPropertiesRequestBuilder {
    request: UpdateSheetPropertiesRequest,
}

impl UpdateSheetPropertiesRequestBuilder {
    pub fn user_id_type(mut self, user_id_type: Option<String>) -> Self {
        self.request.user_id_type = user_id_type;
        self
    }

    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_request(mut self, update_property: UpdateSheetProperty) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::UpdateSheet {
                properties: update_property,
            });
        self
    }

    pub fn build(mut self) -> UpdateSheetPropertiesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateSheetPropertiesRequestBuilder,
    SpreadsheetSheetService,
    UpdateSheetPropertiesRequest,
    BaseResponse<OperateSheetResponse>,
    update_sheet_properties
);

impl SpreadsheetSheetService {
    pub async fn update_sheet_properties(
        &self,
        request: UpdateSheetPropertiesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OperateSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp =
            crate::core::http::Transport::request(api_req, &self.config_arc, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{api_resp::BaseResponse, config::Config};
    use rstest::*;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    // Test UpdateSheetProperty struct
    #[test]
    fn test_update_sheet_property_default() {
        let property = UpdateSheetProperty::default();
        assert_eq!(property.sheet_id, "");
        assert_eq!(property.title, "");
        assert!(property.index.is_none());
        assert!(property.hidden.is_none());
        assert!(property.frozen_col_count.is_none());
        assert!(property.frozen_row_count.is_none());
        assert!(property.protect.is_none());
    }

    #[test]
    fn test_update_sheet_property_creation() {
        let property = UpdateSheetProperty {
            sheet_id: "sheet123".to_string(),
            title: "Test Sheet".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: Some(2),
            frozen_row_count: Some(1),
            protect: None,
        };

        assert_eq!(property.sheet_id, "sheet123");
        assert_eq!(property.title, "Test Sheet");
        assert_eq!(property.index, Some(0));
        assert_eq!(property.hidden, Some(false));
        assert_eq!(property.frozen_col_count, Some(2));
        assert_eq!(property.frozen_row_count, Some(1));
        assert!(property.protect.is_none());
    }

    #[test]
    fn test_update_sheet_property_with_protect() {
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Protected by system".to_string()),
            user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
        };

        let property = UpdateSheetProperty {
            sheet_id: "sheet456".to_string(),
            title: "Protected Sheet".to_string(),
            index: Some(1),
            hidden: Some(true),
            frozen_col_count: Some(0),
            frozen_row_count: Some(0),
            protect: Some(protect),
        };

        assert_eq!(property.sheet_id, "sheet456");
        assert_eq!(property.title, "Protected Sheet");
        assert!(property.protect.is_some());
        let protect_ref = property.protect.as_ref().unwrap();
        assert!(protect_ref.lock);
        assert_eq!(
            protect_ref.lock_info,
            Some("Protected by system".to_string())
        );
        assert_eq!(
            protect_ref.user_ids,
            Some(vec!["user1".to_string(), "user2".to_string()])
        );
    }

    #[test]
    fn test_update_sheet_property_serialization() {
        let property = UpdateSheetProperty {
            sheet_id: "sheet789".to_string(),
            title: "Serialization Test".to_string(),
            index: Some(2),
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let json = serde_json::to_string(&property).unwrap();
        assert!(json.contains("\"sheetId\":\"sheet789\""));
        assert!(json.contains("\"title\":\"Serialization Test\""));
        assert!(json.contains("\"index\":2"));
        assert!(json.contains("\"hidden\":false"));
        // Optional None fields should not appear in JSON
        assert!(!json.contains("frozenColCount"));
        assert!(!json.contains("frozenRowCount"));
        assert!(!json.contains("protect"));
    }

    #[test]
    fn test_update_sheet_property_deserialization() {
        let json = r#"{
            "sheetId": "deserial123",
            "title": "Deserialized Sheet",
            "index": 3,
            "hidden": true,
            "frozenColCount": 5,
            "frozenRowCount": 2
        }"#;

        let property: UpdateSheetProperty = serde_json::from_str(json).unwrap();
        assert_eq!(property.sheet_id, "deserial123");
        assert_eq!(property.title, "Deserialized Sheet");
        assert_eq!(property.index, Some(3));
        assert_eq!(property.hidden, Some(true));
        assert_eq!(property.frozen_col_count, Some(5));
        assert_eq!(property.frozen_row_count, Some(2));
        assert!(property.protect.is_none());
    }

    #[test]
    fn test_update_sheet_property_debug() {
        let property = UpdateSheetProperty {
            sheet_id: "debug_sheet".to_string(),
            title: "Debug Test".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: Some(1),
            frozen_row_count: Some(1),
            protect: None,
        };

        let debug_str = format!("{:?}", property);
        assert!(debug_str.contains("UpdateSheetProperty"));
        assert!(debug_str.contains("debug_sheet"));
        assert!(debug_str.contains("Debug Test"));
    }

    // Test UpdateSheetPropertyProtect struct
    #[test]
    fn test_update_sheet_property_protect_creation() {
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Protected sheet".to_string()),
            user_ids: Some(vec!["user123".to_string()]),
        };

        assert!(protect.lock);
        assert_eq!(protect.lock_info, Some("Protected sheet".to_string()));
        assert_eq!(protect.user_ids, Some(vec!["user123".to_string()]));
    }

    #[test]
    fn test_update_sheet_property_protect_unlock() {
        let protect = UpdateSheetPropertyProtect {
            lock: false,
            lock_info: None,
            user_ids: None,
        };

        assert!(!protect.lock);
        assert!(protect.lock_info.is_none());
        assert!(protect.user_ids.is_none());
    }

    #[test]
    fn test_update_sheet_property_protect_serialization() {
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Test protection".to_string()),
            user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
        };

        let json = serde_json::to_string(&protect).unwrap();
        assert!(json.contains("\"lock\":true"));
        assert!(json.contains("\"lockInfo\":\"Test protection\""));
        assert!(json.contains("\"userIDs\":[\"user1\",\"user2\"]"));
    }

    #[test]
    fn test_update_sheet_property_protect_serialization_none_fields() {
        let protect = UpdateSheetPropertyProtect {
            lock: false,
            lock_info: None,
            user_ids: None,
        };

        let json = serde_json::to_string(&protect).unwrap();
        assert!(json.contains("\"lock\":false"));
        // Optional None fields should not appear in JSON
        assert!(!json.contains("lockInfo"));
        assert!(!json.contains("userIDs"));
    }

    #[test]
    fn test_update_sheet_property_protect_deserialization() {
        let json = r#"{
            "lock": true,
            "lockInfo": "Deserialized protection",
            "userIDs": ["user_a", "user_b", "user_c"]
        }"#;

        let protect: UpdateSheetPropertyProtect = serde_json::from_str(json).unwrap();
        assert!(protect.lock);
        assert_eq!(
            protect.lock_info,
            Some("Deserialized protection".to_string())
        );
        assert_eq!(
            protect.user_ids,
            Some(vec![
                "user_a".to_string(),
                "user_b".to_string(),
                "user_c".to_string()
            ])
        );
    }

    #[test]
    fn test_update_sheet_property_protect_debug() {
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Debug protection".to_string()),
            user_ids: Some(vec!["debug_user".to_string()]),
        };

        let debug_str = format!("{:?}", protect);
        assert!(debug_str.contains("UpdateSheetPropertyProtect"));
        assert!(debug_str.contains("lock: true"));
        assert!(debug_str.contains("Debug protection"));
        assert!(debug_str.contains("debug_user"));
    }

    // Test UpdateSheetPropertiesRequest struct
    #[test]
    fn test_update_sheet_properties_request_default() {
        let request = UpdateSheetPropertiesRequest::default();
        assert!(request.user_id_type.is_none());
        assert_eq!(request.spreadsheet_token, "");
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_update_sheet_properties_request_debug() {
        let request = UpdateSheetPropertiesRequest::default();
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("UpdateSheetPropertiesRequest"));
        assert!(debug_str.contains("user_id_type"));
        assert!(debug_str.contains("spreadsheet_token"));
        assert!(debug_str.contains("requests"));
    }

    // Test UpdateSheetPropertiesRequestBuilder
    #[test]
    fn test_update_sheet_properties_request_builder_creation() {
        let builder = UpdateSheetPropertiesRequest::builder();
        assert!(builder.request.user_id_type.is_none());
        assert_eq!(builder.request.spreadsheet_token, "");
        assert!(builder.request.requests.is_empty());
    }

    #[test]
    fn test_update_sheet_properties_request_builder_with_user_id_type() {
        let request = UpdateSheetPropertiesRequest::builder()
            .user_id_type(Some("open_id".to_string()))
            .spreadsheet_token("token123")
            .build();

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.spreadsheet_token, "token123");
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_update_sheet_properties_request_builder_with_spreadsheet_token() {
        let request = UpdateSheetPropertiesRequest::builder()
            .spreadsheet_token("spreadsheet_token_123")
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_token_123");
        assert!(request.user_id_type.is_none());
        assert!(request.requests.is_empty());
    }

    #[test]
    fn test_update_sheet_properties_request_builder_add_request() {
        let property = UpdateSheetProperty {
            sheet_id: "sheet123".to_string(),
            title: "New Title".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let request = UpdateSheetPropertiesRequest::builder()
            .spreadsheet_token("token456")
            .add_request(property)
            .build();

        assert_eq!(request.spreadsheet_token, "token456");
        assert_eq!(request.requests.len(), 1);

        // Check if the request was properly wrapped in OperateSheetsRequestElem::UpdateSheet
        match &request.requests[0] {
            OperateSheetsRequestElem::UpdateSheet { properties } => {
                assert_eq!(properties.sheet_id, "sheet123");
                assert_eq!(properties.title, "New Title");
                assert_eq!(properties.index, Some(0));
                assert_eq!(properties.hidden, Some(false));
            }
            _ => panic!("Expected UpdateSheet variant"),
        }
    }

    #[test]
    fn test_update_sheet_properties_request_builder_multiple_requests() {
        let property1 = UpdateSheetProperty {
            sheet_id: "sheet1".to_string(),
            title: "Sheet One".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let property2 = UpdateSheetProperty {
            sheet_id: "sheet2".to_string(),
            title: "Sheet Two".to_string(),
            index: Some(1),
            hidden: Some(true),
            frozen_col_count: Some(2),
            frozen_row_count: Some(1),
            protect: None,
        };

        let request = UpdateSheetPropertiesRequest::builder()
            .spreadsheet_token("multi_token")
            .add_request(property1)
            .add_request(property2)
            .build();

        assert_eq!(request.spreadsheet_token, "multi_token");
        assert_eq!(request.requests.len(), 2);

        // Check first request
        match &request.requests[0] {
            OperateSheetsRequestElem::UpdateSheet { properties } => {
                assert_eq!(properties.sheet_id, "sheet1");
                assert_eq!(properties.title, "Sheet One");
                assert_eq!(properties.index, Some(0));
                assert_eq!(properties.hidden, Some(false));
            }
            _ => panic!("Expected UpdateSheet variant for first request"),
        }

        // Check second request
        match &request.requests[1] {
            OperateSheetsRequestElem::UpdateSheet { properties } => {
                assert_eq!(properties.sheet_id, "sheet2");
                assert_eq!(properties.title, "Sheet Two");
                assert_eq!(properties.index, Some(1));
                assert_eq!(properties.hidden, Some(true));
                assert_eq!(properties.frozen_col_count, Some(2));
                assert_eq!(properties.frozen_row_count, Some(1));
            }
            _ => panic!("Expected UpdateSheet variant for second request"),
        }
    }

    #[test]
    fn test_update_sheet_properties_request_builder_chaining() {
        let property = UpdateSheetProperty {
            sheet_id: "chain_sheet".to_string(),
            title: "Chained Sheet".to_string(),
            index: Some(2),
            hidden: Some(false),
            frozen_col_count: Some(3),
            frozen_row_count: Some(2),
            protect: None,
        };

        let request = UpdateSheetPropertiesRequest::builder()
            .user_id_type(Some("union_id".to_string()))
            .spreadsheet_token("chain_token")
            .add_request(property)
            .build();

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(request.spreadsheet_token, "chain_token");
        assert_eq!(request.requests.len(), 1);
    }

    #[test]
    fn test_update_sheet_properties_request_serialization() {
        let property = UpdateSheetProperty {
            sheet_id: "serialize_sheet".to_string(),
            title: "Serialize Test".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let request = UpdateSheetPropertiesRequest::builder()
            .spreadsheet_token("serialize_token")
            .add_request(property)
            .build();

        // The request.api_request.body should contain the serialized data
        assert!(!request.api_request.body.is_empty());

        // Parse the body back to verify serialization
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed["requests"].is_array());
        assert_eq!(parsed["requests"].as_array().unwrap().len(), 1);

        let update_request = &parsed["requests"][0];
        assert!(update_request["updateSheet"].is_object());

        let properties = &update_request["updateSheet"]["properties"];
        assert_eq!(properties["sheetId"].as_str().unwrap(), "serialize_sheet");
        assert_eq!(properties["title"].as_str().unwrap(), "Serialize Test");
        assert_eq!(properties["index"].as_i64().unwrap(), 0);
        assert!(!properties["hidden"].as_bool().unwrap());
    }

    // Test edge cases
    #[test]
    fn test_update_sheet_property_with_unicode() {
        let property = UpdateSheetProperty {
            sheet_id: "unicode_sheet".to_string(),
            title: "测试工作表 🧪".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let json = serde_json::to_string(&property).unwrap();
        assert!(json.contains("测试工作表 🧪"));

        // Test round-trip
        let parsed: UpdateSheetProperty = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.title, "测试工作表 🧪");
    }

    #[test]
    fn test_update_sheet_property_with_extreme_values() {
        let property = UpdateSheetProperty {
            sheet_id: "extreme_sheet".to_string(),
            title: "Extreme Test".to_string(),
            index: Some(i32::MAX),
            hidden: Some(true),
            frozen_col_count: Some(i32::MAX),
            frozen_row_count: Some(i32::MIN),
            protect: None,
        };

        let json = serde_json::to_string(&property).unwrap();
        let parsed: UpdateSheetProperty = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.index, Some(i32::MAX));
        assert_eq!(parsed.frozen_col_count, Some(i32::MAX));
        assert_eq!(parsed.frozen_row_count, Some(i32::MIN));
    }

    #[test]
    fn test_update_sheet_property_with_empty_strings() {
        let property = UpdateSheetProperty {
            sheet_id: "".to_string(),
            title: "".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: Some(0),
            frozen_row_count: Some(0),
            protect: None,
        };

        let json = serde_json::to_string(&property).unwrap();
        let parsed: UpdateSheetProperty = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.sheet_id, "");
        assert_eq!(parsed.title, "");
        assert_eq!(parsed.index, Some(0));
        assert_eq!(parsed.frozen_col_count, Some(0));
        assert_eq!(parsed.frozen_row_count, Some(0));
    }

    #[test]
    fn test_update_sheet_property_protect_with_multiple_users() {
        let user_ids = vec![
            "user1".to_string(),
            "user2".to_string(),
            "user3".to_string(),
            "user4".to_string(),
            "user5".to_string(),
        ];

        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Multi-user protection".to_string()),
            user_ids: Some(user_ids.clone()),
        };

        let json = serde_json::to_string(&protect).unwrap();
        let parsed: UpdateSheetPropertyProtect = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.user_ids, Some(user_ids));
    }

    #[test]
    fn test_update_sheet_property_protect_with_empty_user_list() {
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Empty user list".to_string()),
            user_ids: Some(vec![]),
        };

        let json = serde_json::to_string(&protect).unwrap();
        let parsed: UpdateSheetPropertyProtect = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.user_ids, Some(vec![]));
    }

    #[rstest]
    #[case("open_id", "Open ID type")]
    #[case("union_id", "Union ID type")]
    #[case("", "Empty ID type")]
    fn test_update_sheet_properties_request_with_various_user_id_types(
        #[case] user_id_type: &str,
        #[case] description: &str,
    ) {
        let request = UpdateSheetPropertiesRequest::builder()
            .user_id_type(Some(user_id_type.to_string()))
            .spreadsheet_token("test_token")
            .build();

        assert_eq!(request.user_id_type, Some(user_id_type.to_string()));
        println!("Test passed for: {}", description);
    }

    #[rstest]
    #[case(Some(0), "First position")]
    #[case(Some(1), "Second position")]
    #[case(Some(10), "Tenth position")]
    #[case(None, "No position specified")]
    fn test_update_sheet_property_with_various_indices(
        #[case] index: Option<i32>,
        #[case] description: &str,
    ) {
        let property = UpdateSheetProperty {
            sheet_id: "test_sheet".to_string(),
            title: "Test Sheet".to_string(),
            index,
            hidden: Some(false),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        assert_eq!(property.index, index);
        println!("Test passed for: {}", description);
    }

    #[test]
    fn test_update_sheet_properties_request_memory_efficiency() {
        let mut builder =
            UpdateSheetPropertiesRequest::builder().spreadsheet_token("memory_test_token");

        // Add 100 sheet property updates
        for i in 0..100 {
            let property = UpdateSheetProperty {
                sheet_id: format!("sheet_{}", i),
                title: format!("Sheet {}", i),
                index: Some(i),
                hidden: Some(i % 2 == 0),
                frozen_col_count: Some(i % 5),
                frozen_row_count: Some(i % 3),
                protect: None,
            };
            builder = builder.add_request(property);
        }

        let request = builder.build();
        assert_eq!(request.requests.len(), 100);
        assert_eq!(request.spreadsheet_token, "memory_test_token");
    }

    // Test SpreadsheetSheetService integration
    #[test]
    fn test_spreadsheet_sheet_service_creation() {
        let config = create_test_config();
        let _service = SpreadsheetSheetService::new(config);

        // Just verify the service can be created without panicking
        // Test passes if we reach this point without panicking
    }

    #[test]
    fn test_update_sheet_properties_response_format() {
        // Test that we can parse the expected response format
        let json_response = r#"{
            "code": 0,
            "msg": "success",
            "data": {
                "replies": [
                    {
                        "updateSheet": {
                            "properties": {
                                "sheetId": "updated_sheet",
                                "title": "Updated Title",
                                "index": 0,
                                "hidden": false
                            }
                        }
                    }
                ]
            }
        }"#;

        let response: BaseResponse<OperateSheetResponse> =
            serde_json::from_str(json_response).unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(response.msg(), "success");
        assert!(response.data.is_some());

        let data = response.data.unwrap();
        assert_eq!(data.replies.len(), 1);
    }

    #[test]
    fn test_update_sheet_properties_complex_scenario() {
        // Test a complex scenario with protection, freezing, and positioning
        let protect = UpdateSheetPropertyProtect {
            lock: true,
            lock_info: Some("Sensitive data protection".to_string()),
            user_ids: Some(vec!["admin1".to_string(), "admin2".to_string()]),
        };

        let property1 = UpdateSheetProperty {
            sheet_id: "sheet_complex_1".to_string(),
            title: "Financial Data".to_string(),
            index: Some(0),
            hidden: Some(false),
            frozen_col_count: Some(3),
            frozen_row_count: Some(1),
            protect: Some(protect),
        };

        let property2 = UpdateSheetProperty {
            sheet_id: "sheet_complex_2".to_string(),
            title: "Backup Sheet".to_string(),
            index: Some(10),
            hidden: Some(true),
            frozen_col_count: None,
            frozen_row_count: None,
            protect: None,
        };

        let request = UpdateSheetPropertiesRequest::builder()
            .user_id_type(Some("union_id".to_string()))
            .spreadsheet_token("complex_scenario_token")
            .add_request(property1)
            .add_request(property2)
            .build();

        assert_eq!(request.requests.len(), 2);
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(request.spreadsheet_token, "complex_scenario_token");

        // Verify serialization works for complex nested structures
        assert!(!request.api_request.body.is_empty());

        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(parsed["requests"].as_array().unwrap().len(), 2);
    }
}
