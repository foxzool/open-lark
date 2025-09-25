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

use super::insert_rows_or_columns::DimensionRange;

impl SheetRowColService {
    /// 删除行列
    pub async fn delete_rows_or_columns(
        &self,
        request: DeleteRowsOrColumnsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteRowsOrColumnsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DIMENSION_RANGE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除行列请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 删除位置的维度信息
    dimension_range: DimensionRange,
}

impl DeleteRowsOrColumnsRequest {
    pub fn builder() -> DeleteRowsOrColumnsRequestBuilder {
        DeleteRowsOrColumnsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteRowsOrColumnsRequestBuilder {
    request: DeleteRowsOrColumnsRequest,
}

impl DeleteRowsOrColumnsRequestBuilder {
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

    pub fn build(mut self) -> DeleteRowsOrColumnsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 删除行列响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteRowsOrColumnsResponseData {
    /// 删除行列后的信息
    pub delete_range: DeleteRangeInfo,
}

impl ApiResponseTrait for DeleteRowsOrColumnsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除范围信息
#[derive(Deserialize, Debug)]
pub struct DeleteRangeInfo {
    /// 删除的维度
    pub dimension: String,
    /// 删除的起始位置
    pub start_index: i32,
    /// 删除的结束位置
    pub end_index: i32,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::DeleteRowsOrColumnsResponseData;

    #[test]
    fn test_delete_rows_or_columns_response() {
        let json = json!({
            "delete_range": {
                "dimension": "ROWS",
                "start_index": 2,
                "end_index": 4
            }
        });

        let response: DeleteRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.delete_range.dimension, "ROWS");
        assert_eq!(response.delete_range.start_index, 2);
        assert_eq!(response.delete_range.end_index, 4);
    }
}

// 实现ExecutableBuilder trait
impl_executable_builder_owned!(
    DeleteRowsOrColumnsRequestBuilder,
    SheetRowColService,
    DeleteRowsOrColumnsRequest,
    BaseResponse<DeleteRowsOrColumnsResponseData>,
    delete_rows_or_columns
);
