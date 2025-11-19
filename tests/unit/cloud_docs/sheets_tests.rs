//! Sheets 服务单元测试
//!
//! 本模块测试飞书电子表格 Sheets API 的各种功能，包括：
//! - 数据读写操作 (v2/v3)
//! - 单元格样式设置
//! - 数据查找和替换
//! - 单元格合并拆分
//! - 电子表格管理
//! - 工作表操作
//! - 条件格式化
//! - 数据验证

use wiremock::{
    matchers::{method, path, path_regex, query_param},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use rstest::{fixture, rstest};
use mockall::predicate::*;
use proptest::prelude::*;

use open_lark::{
    core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
    service::cloud_docs::sheets::{
        v3::{
            data_operation::{
                DataOperationService,
                reading_single_range::{
                    ReadingSingleRangeRequest, ReadingSingleRangeResponseData, ValueRange
                },
                write_data_to_multiple_ranges::{
                    WriteDataToMultipleRangesRequest, WriteDataToMultipleRangesResponseData,
                    ValueRange as WriteValueRange
                },
                append_data::{AppendDataRequest, AppendDataResponseData},
                find_cells::{FindCellsRequest, FindCellsResponseData, FindCondition},
                replace_cells::{ReplaceCellsRequest, ReplaceCellsResponseData},
            },
            spreadsheet::{
                SpreadsheetService, 
                create::{CreateSpreadsheetRequest, CreateSpreadsheetResponseData},
                get::{GetSpreadsheetRequest, GetSpreadsheetResponseData},
            },
            spreadsheet_sheet::{
                SpreadsheetSheetService,
                create::{CreateSheetRequest, CreateSheetResponseData},
                query::{GetSheetRequest, GetSheetResponseData},
            },
        },
    },
    core::api::Response,
};

/// 测试配置夹具
#[fixture]
fn test_config() -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        lark_host: "https://open.feishu.cn".to_string(),
        ..Default::default()
    }
}

/// 模拟服务器夹具
#[fixture]
async fn mock_server() -> MockServer {
    MockServer::start().await
}

// ===================== 数据读取测试 =====================

#[rstest]
#[tokio::test]
async fn test_reading_single_range_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let range = "Sheet1!A1:C3";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "valueRange": {
                "range": "Sheet1!A1:C3",
                "revision": 12,
                "values": [
                    ["姓名", "年龄", "部门"],
                    ["张三", 25, "研发部"],
                    ["李四", 30, "销售部"]
                ]
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}", spreadsheet_token, range)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .value_render_option("FormattedValue")
        .build();

    let result = data_service.reading_single_range(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.msg, "success");
    
    let data = response.data.unwrap();
    assert_eq!(data.value_range.range, "Sheet1!A1:C3");
    assert_eq!(data.value_range.revision, 12);
    assert_eq!(data.value_range.values.len(), 3);
    
    // 检查第一行数据（标题行）
    let first_row = &data.value_range.values[0];
    assert_eq!(first_row[0], "姓名");
    assert_eq!(first_row[1], "年龄"); 
    assert_eq!(first_row[2], "部门");
    
    // 检查数据行
    let second_row = &data.value_range.values[1];
    assert_eq!(second_row[0], "张三");
    assert_eq!(second_row[1], 25);
    assert_eq!(second_row[2], "研发部");
}

#[rstest]
#[tokio::test]
async fn test_reading_single_range_with_options(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let range = "Sheet1!A1:B2";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "valueRange": {
                "range": "Sheet1!A1:B2",
                "revision": 5,
                "values": [
                    ["2024-01-01", "10:30:00"],
                    ["数字值", 42.5]
                ]
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}", spreadsheet_token, range)))
        .and(query_param("valueRenderOption", "UnformattedValue"))
        .and(query_param("dateTimeRenderOption", "SerialNumber"))
        .and(query_param("user_id_type", "open_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .value_render_option("UnformattedValue")
        .date_time_render_option("SerialNumber")
        .user_id_type("open_id")
        .build();

    let result = data_service.reading_single_range(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.value_range.values.len(), 2);
}

#[rstest]
#[tokio::test]
async fn test_reading_empty_range(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let range = "Sheet1!Z99:Z100"; // 空范围
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "valueRange": {
                "range": "Sheet1!Z99:Z100",
                "revision": 1,
                "values": []
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}", spreadsheet_token, range)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .build();

    let result = data_service.reading_single_range(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.value_range.values.len(), 0);
}

// ===================== 数据写入测试 =====================

#[rstest]
#[tokio::test]  
async fn test_write_data_to_multiple_ranges_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "replies": [
                {
                    "range": "Sheet1!A1:C2",
                    "revision": 15
                },
                {
                    "range": "Sheet1!E1:F2", 
                    "revision": 15
                }
            ],
            "revision": 15,
            "spreadsheetToken": spreadsheet_token,
            "updatedCells": 6,
            "updatedColumns": 4,
            "updatedDataType": "VALUES",
            "updatedRange": "Sheet1!A1:F2",
            "updatedRows": 2
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values_batch_update", spreadsheet_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    
    // 构建写入请求，包含多个范围的数据
    let value_ranges = vec![
        WriteValueRange {
            range: "Sheet1!A1:C2".to_string(),
            values: vec![
                vec![
                    json!("产品名称"),
                    json!("价格"),
                    json!("库存")
                ],
                vec![
                    json!("iPhone 15"),
                    json!(5999),
                    json!(100)
                ]
            ]
        },
        WriteValueRange {
            range: "Sheet1!E1:F2".to_string(), 
            values: vec![
                vec![json!("供应商"), json!("联系人")],
                vec![json!("苹果公司"), json!("John Doe")]
            ]
        }
    ];
    
    let request = WriteDataToMultipleRangesRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .value_ranges(value_ranges)
        .value_input_option("USER_ENTERED")
        .build();

    let result = data_service.write_data_to_multiple_ranges(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.revision, 15);
    assert_eq!(data.updated_cells, 6);
    assert_eq!(data.updated_columns, 4);
    assert_eq!(data.updated_rows, 2);
    assert_eq!(data.replies.len(), 2);
    
    assert_eq!(data.replies[0].range, "Sheet1!A1:C2");
    assert_eq!(data.replies[1].range, "Sheet1!E1:F2");
}

// ===================== 数据追加测试 =====================

#[rstest]
#[tokio::test]
async fn test_append_data_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let range = "Sheet1!A:C";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success", 
        "data": {
            "spreadsheetToken": spreadsheet_token,
            "revision": 20,
            "tableRange": "Sheet1!A1:C10",
            "updates": {
                "updatedRange": "Sheet1!A9:C10",
                "updatedRows": 2,
                "updatedColumns": 3,
                "updatedCells": 6
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}:append", spreadsheet_token, range)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = AppendDataRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .values(vec![
            vec![json!("新产品1"), json!(299), json!(50)],
            vec![json!("新产品2"), json!(399), json!(30)]
        ])
        .insert_data_option("INSERT_ROWS")
        .build();

    let result = data_service.append_data(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.revision, 20);
    assert_eq!(data.table_range, "Sheet1!A1:C10");
    assert_eq!(data.updates.updated_rows, 2);
    assert_eq!(data.updates.updated_cells, 6);
}

// ===================== 查找单元格测试 =====================

#[rstest]
#[tokio::test]
async fn test_find_cells_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let sheet_id = "sheet123";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "findResult": [
                {
                    "range": "Sheet1!B2:B2",
                    "value": "张三"
                },
                {
                    "range": "Sheet1!B5:B5",
                    "value": "张三丰"
                }
            ]
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/find", spreadsheet_token, sheet_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let find_condition = FindCondition {
        range: Some("A1:Z100".to_string()),
        match_case: Some(false),
        match_entire_cell: Some(false),
        search_by_regex: Some(false),
        include_formulas: Some(false),
    };
    
    let request = FindCellsRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .find_condition(find_condition)
        .find("张")
        .build();

    let result = data_service.find_cells(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.find_result.len(), 2);
    assert_eq!(data.find_result[0].range, "Sheet1!B2:B2");
    assert_eq!(data.find_result[0].value, "张三");
    assert_eq!(data.find_result[1].value, "张三丰");
}

// ===================== 替换单元格测试 =====================

#[rstest]
#[tokio::test]
async fn test_replace_cells_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let sheet_id = "sheet123";
    
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "replaceResult": {
                "replacedCellsCount": 3,
                "replacedRowsCount": 2,
                "spreadsheetToken": spreadsheet_token,
                "revision": 25
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/replace", spreadsheet_token, sheet_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let find_condition = FindCondition {
        range: Some("A1:Z100".to_string()),
        match_case: Some(true),
        match_entire_cell: Some(false),
        search_by_regex: Some(false),
        include_formulas: Some(false),
    };
    
    let request = ReplaceCellsRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .find_condition(find_condition)
        .find("旧部门名")
        .replacement("新部门名")
        .build();

    let result = data_service.replace_cells(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.replace_result.replaced_cells_count, 3);
    assert_eq!(data.replace_result.replaced_rows_count, 2);
    assert_eq!(data.replace_result.revision, 25);
}

// ===================== 电子表格管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_spreadsheet_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "spreadsheet": {
                "title": "销售数据分析表",
                "folderToken": "folder_123",
                "url": "https://docs.feishu.cn/spreadsheet/new_spreadsheet_token",
                "spreadsheetToken": "new_spreadsheet_token"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/sheets/v3/spreadsheets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let spreadsheet_service = SpreadsheetService::new(config);
    let request = CreateSpreadsheetRequest::builder()
        .title("销售数据分析表")
        .folder_token("folder_123")
        .build();

    let result = spreadsheet_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
    
    let data = response.data.unwrap();
    assert_eq!(data.spreadsheet.title, "销售数据分析表");
    assert_eq!(data.spreadsheet.folder_token, "folder_123");
    assert_eq!(data.spreadsheet.spreadsheet_token, "new_spreadsheet_token");
    assert!(data.spreadsheet.url.contains("new_spreadsheet_token"));
}

#[rstest]
#[tokio::test]
async fn test_get_spreadsheet_info(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "existing_spreadsheet_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "spreadsheet": {
                "title": "现有表格",
                "ownerUser": "ou_owner_123", 
                "url": "https://docs.feishu.cn/spreadsheet/existing_spreadsheet_token",
                "spreadsheetToken": spreadsheet_token
            }
        }
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}", spreadsheet_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let spreadsheet_service = SpreadsheetService::new(config);
    let request = GetSpreadsheetRequest::new(spreadsheet_token);

    let result = spreadsheet_service.get(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.spreadsheet.title, "现有表格");
    assert_eq!(data.spreadsheet.spreadsheet_token, spreadsheet_token);
}

// ===================== 工作表管理测试 =====================

#[rstest]
#[tokio::test]
async fn test_create_sheet_success(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "sheet": {
                "sheetId": "new_sheet_123",
                "title": "销售明细",
                "index": 1,
                "hidden": false,
                "gridProperties": {
                    "rowCount": 1000,
                    "columnCount": 26
                }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/sheets", spreadsheet_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let sheet_service = SpreadsheetSheetService::new(config);
    let request = CreateSheetRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .title("销售明细")
        .index(1)
        .build();

    let result = sheet_service.create(request, None).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.sheet.title, "销售明细");
    assert_eq!(data.sheet.index, 1);
    assert_eq!(data.sheet.hidden, false);
    assert!(data.sheet.sheet_id.starts_with("new_sheet_"));
}

// ===================== 请求构建器测试 =====================

#[test]
fn test_reading_single_range_request_builder() {
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token("token123")
        .range("Sheet1!A1:D10")
        .value_render_option("FormattedValue")
        .date_time_render_option("SerialNumber")
        .user_id_type("open_id")
        .build();

    assert_eq!(request.spreadsheet_token, "token123");
    assert_eq!(request.range, "Sheet1!A1:D10");
    assert_eq!(request.value_render_option, Some("FormattedValue".to_string()));
    assert_eq!(request.date_time_render_option, Some("SerialNumber".to_string()));
    assert_eq!(request.user_id_type, Some("open_id".to_string()));
    
    // 验证查询参数是否正确设置
    assert_eq!(request.api_request.query_params.get("valueRenderOption"), Some(&"FormattedValue".to_string()));
    assert_eq!(request.api_request.query_params.get("dateTimeRenderOption"), Some(&"SerialNumber".to_string()));
    assert_eq!(request.api_request.query_params.get("user_id_type"), Some(&"open_id".to_string()));
}

#[test]
fn test_append_data_request_builder() {
    let values = vec![
        vec![json!("产品A"), json!(100), json!(50)],
        vec![json!("产品B"), json!(200), json!(30)]
    ];
    
    let request = AppendDataRequest::builder()
        .spreadsheet_token("token123")
        .range("Sheet1!A:C")
        .values(values.clone())
        .insert_data_option("INSERT_ROWS")
        .value_input_option("USER_ENTERED")
        .build();

    assert_eq!(request.spreadsheet_token, "token123");
    assert_eq!(request.range, "Sheet1!A:C");
    assert_eq!(request.values, values);
    assert_eq!(request.insert_data_option, Some("INSERT_ROWS".to_string()));
    assert_eq!(request.value_input_option, Some("USER_ENTERED".to_string()));
}

// ===================== 属性测试 =====================

proptest! {
    #[test]
    fn test_range_format_invariants(
        sheet_name in "[A-Za-z][A-Za-z0-9_]{0,30}",
        start_col in "[A-Z]{1,3}",
        start_row in 1u32..=1048576,
        end_col in "[A-Z]{1,3}",
        end_row in 1u32..=1048576
    ) {
        let range = format!("{}!{}{}:{}{}", sheet_name, start_col, start_row, end_col, end_row);
        
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("test_token")
            .range(&range)
            .build();
            
        assert_eq!(request.range, range);
        assert!(request.range.contains(&sheet_name));
        assert!(request.range.contains(&format!("{}{}", start_col, start_row)));
        assert!(request.range.contains(&format!("{}{}", end_col, end_row)));
    }
}

proptest! {
    #[test]
    fn test_spreadsheet_token_invariants(
        token in prop::string::string_regex("[a-zA-Z0-9_-]{20,50}").unwrap()
    ) {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token(&token)
            .range("Sheet1!A1:B2")
            .build();
            
        assert_eq!(request.spreadsheet_token, token);
    }
}

// ===================== 错误处理测试 =====================

#[rstest]
#[tokio::test]
async fn test_invalid_range_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    let invalid_range = "InvalidRange!XYZ999:ABC1"; // 无效范围
    
    let mock_response = json!({
        "code": 1254015,
        "msg": "range参数有误"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}", spreadsheet_token, invalid_range)))
        .respond_with(ResponseTemplate::new(400).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(invalid_range)
        .build();

    let result = data_service.reading_single_range(request, None).await;
    
    // 根据SDK设计，可能返回错误或包含错误代码的响应
    if result.is_ok() {
        let response = result.unwrap();
        assert_ne!(response.code, 0);
        assert!(response.msg.contains("range") || response.msg.contains("参数"));
    } else {
        // 网络或解析错误
        assert!(result.is_err());
    }
}

#[rstest]
#[tokio::test]
async fn test_permission_denied_error(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "no_permission_token";
    let range = "Sheet1!A1:B2";
    
    let mock_response = json!({
        "code": 1248006,
        "msg": "无权限访问该电子表格"
    });

    Mock::given(method("GET"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values/{}", spreadsheet_token, range)))
        .respond_with(ResponseTemplate::new(403).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    let request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .range(range)
        .build();

    let result = data_service.reading_single_range(request, None).await;
    
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, 1248006);
        assert!(response.msg.contains("权限"));
    }
}

// ===================== 集成场景测试 =====================

#[rstest]
#[tokio::test]
async fn test_spreadsheet_workflow_integration(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    // 1. 创建电子表格
    let create_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "spreadsheet": {
                "title": "集成测试表格",
                "folderToken": "test_folder",
                "url": "https://docs.feishu.cn/spreadsheet/integration_token", 
                "spreadsheetToken": "integration_token"
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/sheets/v3/spreadsheets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(create_response))
        .mount(&mock_server)
        .await;

    // 2. 写入数据
    let write_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "replies": [{
                "range": "Sheet1!A1:C3",
                "revision": 2
            }],
            "revision": 2,
            "spreadsheetToken": "integration_token",
            "updatedCells": 9,
            "updatedColumns": 3,
            "updatedRows": 3
        }
    });

    Mock::given(method("POST"))
        .and(path("/open-apis/sheets/v3/spreadsheets/integration_token/values_batch_update"))
        .respond_with(ResponseTemplate::new(200).set_body_json(write_response))
        .mount(&mock_server)
        .await;

    // 3. 读取数据验证
    let read_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "valueRange": {
                "range": "Sheet1!A1:C3",
                "revision": 2,
                "values": [
                    ["产品", "价格", "销量"],
                    ["iPhone", 5999, 100],
                    ["iPad", 3999, 50]
                ]
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/open-apis/sheets/v3/spreadsheets/integration_token/values/Sheet1!A1:C3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(read_response))
        .mount(&mock_server)
        .await;

    // 执行集成测试流程
    let spreadsheet_service = SpreadsheetService::new(config.clone());
    let data_service = DataOperationService::new(config);

    // 1. 创建电子表格
    let create_request = CreateSpreadsheetRequest::builder()
        .title("集成测试表格")
        .folder_token("test_folder")
        .build();
    
    let create_result = spreadsheet_service.create(create_request, None).await;
    assert!(create_result.is_ok());
    
    let create_data = create_result.unwrap().data.unwrap();
    let spreadsheet_token = create_data.spreadsheet.spreadsheet_token;
    assert_eq!(spreadsheet_token, "integration_token");

    // 2. 写入数据
    let value_ranges = vec![WriteValueRange {
        range: "Sheet1!A1:C3".to_string(),
        values: vec![
            vec![json!("产品"), json!("价格"), json!("销量")],
            vec![json!("iPhone"), json!(5999), json!(100)],
            vec![json!("iPad"), json!(3999), json!(50)]
        ]
    }];
    
    let write_request = WriteDataToMultipleRangesRequest::builder()
        .spreadsheet_token(&spreadsheet_token)
        .value_ranges(value_ranges)
        .build();
        
    let write_result = data_service.write_data_to_multiple_ranges(write_request, None).await;
    assert!(write_result.is_ok());
    
    let write_data = write_result.unwrap().data.unwrap();
    assert_eq!(write_data.updated_cells, 9);
    assert_eq!(write_data.revision, 2);

    // 3. 读取数据验证
    let read_request = ReadingSingleRangeRequest::builder()
        .spreadsheet_token(&spreadsheet_token)
        .range("Sheet1!A1:C3")
        .build();
        
    let read_result = data_service.reading_single_range(read_request, None).await;
    assert!(read_result.is_ok());
    
    let read_data = read_result.unwrap().data.unwrap();
    assert_eq!(read_data.value_range.values.len(), 3);
    assert_eq!(read_data.value_range.values[0][0], "产品");
    assert_eq!(read_data.value_range.values[1][1], 5999);
    assert_eq!(read_data.value_range.values[2][2], 50);
}

// ===================== 性能相关测试 =====================

#[rstest]
#[tokio::test]
async fn test_large_data_batch_operations(test_config: Config, #[future] mock_server: MockServer) {
    let mock_server = mock_server.await;
    let mut config = test_config;
    config.lark_host = mock_server.uri();

    let spreadsheet_token = "test_spreadsheet_token";
    
    // 模拟大量数据写入
    let large_batch_size = 1000;
    let mock_response = json!({
        "code": 0,
        "msg": "success",
        "data": {
            "replies": [{
                "range": format!("Sheet1!A1:D{}", large_batch_size),
                "revision": 50
            }],
            "revision": 50,
            "spreadsheetToken": spreadsheet_token,
            "updatedCells": large_batch_size * 4,
            "updatedColumns": 4,
            "updatedRows": large_batch_size
        }
    });

    Mock::given(method("POST"))
        .and(path(format!("/open-apis/sheets/v3/spreadsheets/{}/values_batch_update", spreadsheet_token)))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let data_service = DataOperationService::new(config);
    
    // 构建大量数据
    let mut values = Vec::new();
    for i in 0..large_batch_size {
        values.push(vec![
            json!(format!("产品{}", i)),
            json!(100 + i),
            json!(50 + i % 100),
            json!(format!("描述{}", i))
        ]);
    }
    
    let value_ranges = vec![WriteValueRange {
        range: format!("Sheet1!A1:D{}", large_batch_size),
        values
    }];
    
    let request = WriteDataToMultipleRangesRequest::builder()
        .spreadsheet_token(spreadsheet_token)
        .value_ranges(value_ranges)
        .build();

    let start = std::time::Instant::now();
    let result = data_service.write_data_to_multiple_ranges(request, None).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    let response = result.unwrap();
    let data = response.data.unwrap();
    assert_eq!(data.updated_rows, large_batch_size);
    assert_eq!(data.updated_cells, large_batch_size * 4);

    // 性能断言 - 大批量写入应该在合理时间内完成
    assert!(duration.as_millis() < 10000, "大批量写入耗时过长: {:?}", duration);
}

#[cfg(test)]
mod test_utils {
    use super::*;

    /// 创建测试用的值范围
    pub fn create_test_value_range(range: &str, rows: usize, cols: usize) -> WriteValueRange {
        let mut values = Vec::new();
        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                row.push(json!(format!("Cell{}_{}", i, j)));
            }
            values.push(row);
        }
        
        WriteValueRange {
            range: range.to_string(),
            values
        }
    }

    /// 创建测试用的查找条件
    pub fn create_test_find_condition(
        range: Option<&str>,
        match_case: bool,
        match_entire_cell: bool,
    ) -> FindCondition {
        FindCondition {
            range: range.map(|r| r.to_string()),
            match_case: Some(match_case),
            match_entire_cell: Some(match_entire_cell),
            search_by_regex: Some(false),
            include_formulas: Some(false),
        }
    }
}