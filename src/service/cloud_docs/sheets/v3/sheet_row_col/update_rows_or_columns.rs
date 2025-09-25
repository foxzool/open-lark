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
    service::sheets::v3::SheetRowColService,
};

use super::insert_rows_or_columns::DimensionRange;

impl SheetRowColService {
    /// 更新行列
    pub async fn update_rows_or_columns(
        &self,
        request: UpdateRowsOrColumnsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateRowsOrColumnsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DIMENSION_RANGE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 更新行列请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 更新位置的维度信息
    dimension_range: DimensionRange,
    /// 维度的属性
    dimension_properties: DimensionProperties,
}

impl UpdateRowsOrColumnsRequest {
    pub fn builder() -> UpdateRowsOrColumnsRequestBuilder {
        UpdateRowsOrColumnsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateRowsOrColumnsRequestBuilder {
    request: UpdateRowsOrColumnsRequest,
}

impl UpdateRowsOrColumnsRequestBuilder {
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

    pub fn dimension_properties(mut self, visible: Option<bool>, pixel_size: Option<i32>) -> Self {
        self.request.dimension_properties = DimensionProperties {
            visible,
            pixel_size,
        };
        self
    }

    pub fn build(mut self) -> UpdateRowsOrColumnsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 维度属性
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// 是否可见
    pub visible: Option<bool>,
    /// 行高或列宽（像素）
    pub pixel_size: Option<i32>,
}

/// 更新行列响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateRowsOrColumnsResponseData {
    /// 更新行列后的信息
    pub update_range: UpdateRangeInfo,
}

impl ApiResponseTrait for UpdateRowsOrColumnsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新范围信息
#[derive(Deserialize, Debug)]
pub struct UpdateRangeInfo {
    /// 更新的维度
    pub dimension: String,
    /// 更新的起始位置
    pub start_index: i32,
    /// 更新的结束位置
    pub end_index: i32,
    /// 更新后的属性
    pub properties: DimensionProperties,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::UpdateRowsOrColumnsResponseData;

    #[test]
    fn test_update_rows_or_columns_response() {
        let json = json!({
            "update_range": {
                "dimension": "ROWS",
                "start_index": 1,
                "end_index": 3,
                "properties": {
                    "visible": true,
                    "pixel_size": 50
                }
            }
        });

        let response: UpdateRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.update_range.dimension, "ROWS");
        assert_eq!(response.update_range.start_index, 1);
        assert_eq!(response.update_range.end_index, 3);
        assert_eq!(response.update_range.properties.visible, Some(true));
        assert_eq!(response.update_range.properties.pixel_size, Some(50));
    }
}
