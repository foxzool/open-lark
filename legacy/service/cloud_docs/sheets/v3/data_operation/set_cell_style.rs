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

impl DataOperationService {
    /// 设置单元格样式
    pub async fn set_cell_style(
        &self,
        request: SetCellStyleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SetCellStyleResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_STYLE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<SetCellStyleResponseData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 设置单元格样式请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 样式应用的范围
    ranges: Vec<String>,
    /// 样式信息
    style: CellStyle,
}

impl SetCellStyleRequest {
    pub fn builder() -> SetCellStyleRequestBuilder {
        SetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SetCellStyleRequestBuilder {
    request: SetCellStyleRequest,
}

impl SetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        self.request.ranges = ranges;
        self
    }

    pub fn style(mut self, style: CellStyle) -> Self {
        self.request.style = style;
        self
    }

    pub fn build(mut self) -> SetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    SetCellStyleRequestBuilder,
    DataOperationService,
    SetCellStyleRequest,
    SetCellStyleResponseData,
    set_cell_style
);

/// 单元格样式
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CellStyle {
    /// 字体
    pub font: Option<FontStyle>,
    /// 文本装饰
    pub text_decoration: Option<i32>,
    /// 格式化类型
    pub formatter: Option<String>,
    /// 对齐方式
    pub align: Option<i32>,
    /// 背景颜色
    pub back_color: Option<String>,
    /// 前景色
    pub fore_color: Option<String>,
    /// 边框
    pub border: Option<BorderStyle>,
    /// 清理信息
    pub clean: Option<i32>,
}

/// 字体样式
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FontStyle {
    /// 加粗
    pub bold: Option<bool>,
    /// 斜体
    pub italic: Option<bool>,
    /// 字体大小
    pub size: Option<String>,
    /// 字体名称
    pub name: Option<String>,
}

/// 边框样式
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BorderStyle {
    /// 边框类型
    pub border_type: Option<String>,
    /// 边框颜色
    pub color: Option<String>,
}

/// 设置单元格样式响应体最外层
#[derive(Deserialize, Debug)]
pub struct SetCellStyleResponseData {
    /// 更新的单元格数量
    pub updated_cells: i32,
    /// 更新的范围
    pub updated_ranges: Vec<String>,
}

impl ApiResponseTrait for SetCellStyleResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::SetCellStyleResponseData;

    #[test]
    fn test_set_cell_style_response() {
        let json = json!({
            "updated_cells": 10,
            "updated_ranges": ["A1:B5"]
        });

        let response: SetCellStyleResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.updated_cells, 10);
        assert_eq!(response.updated_ranges, vec!["A1:B5"]);
    }
}
