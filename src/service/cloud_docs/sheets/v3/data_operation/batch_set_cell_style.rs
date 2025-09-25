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
        standard_response::StandardResponse,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

use super::set_cell_style::CellStyle;

impl DataOperationService {
    /// 批量设置单元格样式
    pub async fn batch_set_cell_style(
        &self,
        request: BatchSetCellStyleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BatchSetCellStyleResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<BatchSetCellStyleResponseData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 批量设置单元格样式请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BatchSetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 批量样式数据
    data: Vec<RangeStyleData>,
}

impl BatchSetCellStyleRequest {
    pub fn builder() -> BatchSetCellStyleRequestBuilder {
        BatchSetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BatchSetCellStyleRequestBuilder {
    request: BatchSetCellStyleRequest,
}

impl BatchSetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn add_range_style(mut self, range: impl ToString, style: CellStyle) -> Self {
        self.request.data.push(RangeStyleData {
            range: range.to_string(),
            style,
        });
        self
    }

    pub fn range_styles(mut self, data: Vec<RangeStyleData>) -> Self {
        self.request.data = data;
        self
    }

    pub fn build(mut self) -> BatchSetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    BatchSetCellStyleRequestBuilder,
    DataOperationService,
    BatchSetCellStyleRequest,
    BatchSetCellStyleResponseData,
    batch_set_cell_style
);

/// 范围样式数据
#[derive(Debug, Serialize, Deserialize)]
pub struct RangeStyleData {
    /// 单元格范围
    pub range: String,
    /// 样式信息
    pub style: CellStyle,
}

/// 批量设置单元格样式响应体最外层
#[derive(Deserialize, Debug)]
pub struct BatchSetCellStyleResponseData {
    /// 更新的单元格总数
    pub updated_cells: i32,
    /// 更新的范围列表
    pub updated_ranges: Vec<String>,
    /// 执行成功的批次数
    pub success_count: i32,
    /// 执行失败的批次数
    pub failure_count: i32,
}

impl ApiResponseTrait for BatchSetCellStyleResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::{
        super::set_cell_style::{CellStyle, FontStyle},
        BatchSetCellStyleResponseData, RangeStyleData,
    };

    #[test]
    fn test_batch_set_cell_style_response() {
        let json = json!({
            "updated_cells": 25,
            "updated_ranges": ["A1:B5", "C1:D5"],
            "success_count": 2,
            "failure_count": 0
        });

        let response: BatchSetCellStyleResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.updated_cells, 25);
        assert_eq!(response.updated_ranges.len(), 2);
        assert_eq!(response.success_count, 2);
        assert_eq!(response.failure_count, 0);
    }

    #[test]
    fn test_range_style_data_serialization() {
        let style = CellStyle {
            font: Some(FontStyle {
                bold: Some(true),
                italic: Some(false),
                size: Some("12".to_string()),
                name: Some("Arial".to_string()),
            }),
            text_decoration: Some(1),
            formatter: Some("text".to_string()),
            align: Some(1),
            back_color: Some("#FFFFFF".to_string()),
            fore_color: Some("#000000".to_string()),
            border: None,
            clean: None,
        };

        let range_style = RangeStyleData {
            range: "A1:B5".to_string(),
            style,
        };

        let json = serde_json::to_value(&range_style).unwrap();
        assert_eq!(json["range"], "A1:B5");
        assert_eq!(json["style"]["font"]["bold"], true);
    }
}
