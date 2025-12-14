use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 查询工作表列表请求
#[derive(Debug, Serialize, Default)]
pub struct QuerySheetsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 查询工作表列表响应
#[derive(Debug, Deserialize, Default)]
pub struct QuerySheetsResponse {
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 操作结果
    pub result: String,
}

/// 工作表信息
#[derive(Debug, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
}

/// 网格属性
#[derive(Debug, Deserialize, Default)]
pub struct GridProperties {
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 冻结行数
    pub frozen_row_count: i32,
    /// 冻结列数
    pub frozen_column_count: i32,
}

/// 查询工作表列表
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query
pub async fn query_sheets(
    request: QuerySheetsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<QuerySheetsResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/query",
        config.base_url, request.spreadsheet_token
    );

    let mut query_params = vec![];

    if let Some(page_size) = request.page_size {
        query_params.push(("page_size".to_string(), page_size.to_string()));
    }

    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token".to_string(), page_token.clone()));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params,
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_query_sheets() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = QuerySheetsRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            page_size: Some(20),
            page_token: None,
        };

        let result = query_sheets(request, &config, None).await;
        assert!(result.is_ok());
    }
}