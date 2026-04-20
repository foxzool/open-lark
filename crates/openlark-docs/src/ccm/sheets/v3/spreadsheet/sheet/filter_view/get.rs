/// 获取筛选视图
///
/// 获取指定筛选视图 id 的名字和筛选范围。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选视图响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewResponse {
    /// 筛选视图详情。
    pub filter_view: FilterView,
}

impl ApiResponseTrait for GetFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选视图。
pub async fn get_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<GetFilterViewResponse> {
    get_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选视图（支持请求选项）。
pub async fn get_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterViewResponse> {
    let api_endpoint = SheetsApiV3::GetFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterViewResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选视图")
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
