use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::SheetRowColService,
};

impl SheetRowColService {
    /// 插入行列
    pub async fn insert_rows_or_columns(
        &self,
        request: InsertRowsOrColumnsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<InsertRowsOrColumnsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DIMENSION_RANGE_INSERT
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 插入行列请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InsertRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 插入位置的维度信息
    dimension_range: DimensionRange,
    /// 是否继承样式
    inherit_style: Option<String>,
}

impl InsertRowsOrColumnsRequest {
    pub fn builder() -> InsertRowsOrColumnsRequestBuilder {
        InsertRowsOrColumnsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct InsertRowsOrColumnsRequestBuilder {
    request: InsertRowsOrColumnsRequest,
}

impl InsertRowsOrColumnsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn dimension_range(
        mut self,
        dimension: impl ToString,
        start_index: i32,
        end_index: i32,
    ) -> Self {
        self.request.dimension_range = DimensionRange {
            dimension: dimension.to_string(),
            start_index,
            end_index,
        };
        self
    }

    /// 设置是否继承样式
    /// - BEFORE: 继承前一行/列的样式
    /// - AFTER: 继承后一行/列的样式
    pub fn inherit_style(mut self, inherit_style: impl ToString) -> Self {
        self.request.inherit_style = Some(inherit_style.to_string());
        self
    }

    pub fn build(mut self) -> InsertRowsOrColumnsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 维度范围
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionRange {
    /// 维度类型：ROWS（行）或 COLUMNS（列）
    pub dimension: String,
    /// 起始索引
    pub start_index: i32,
    /// 结束索引
    pub end_index: i32,
}

/// 插入行列响应体最外层
#[derive(Deserialize, Debug)]
pub struct InsertRowsOrColumnsResponseData {
    /// 插入行列后的信息
    pub insert_range: InsertRangeInfo,
}

impl ApiResponseTrait for InsertRowsOrColumnsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入范围信息
#[derive(Deserialize, Debug)]
pub struct InsertRangeInfo {
    /// 插入的维度
    pub dimension: String,
    /// 插入的起始位置
    pub start_index: i32,
    /// 插入的结束位置
    pub end_index: i32,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::InsertRowsOrColumnsResponseData;

    #[test]
    fn test_insert_rows_or_columns_response() {
        let json = json!({
            "insert_range": {
                "dimension": "COLUMNS",
                "start_index": 3,
                "end_index": 5
            }
        });

        let response: InsertRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.insert_range.dimension, "COLUMNS");
        assert_eq!(response.insert_range.start_index, 3);
        assert_eq!(response.insert_range.end_index, 5);
    }
}

// 实现ExecutableBuilder trait
impl_executable_builder_owned!(
    InsertRowsOrColumnsRequestBuilder,
    SheetRowColService,
    InsertRowsOrColumnsRequest,
    BaseResponse<InsertRowsOrColumnsResponseData>,
    insert_rows_or_columns
);
