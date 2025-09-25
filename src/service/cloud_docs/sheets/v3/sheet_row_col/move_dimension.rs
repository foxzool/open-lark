use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::sheets::v3::SpreadsheetSheetService,
};

/// 移动行列请求
#[derive(Serialize, Default, Debug)]
pub struct MoveDimensionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 电子表格的 token
    #[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的 ID。调用获取工作表获取 ID
    #[serde(skip)]
    sheet_id: String,
    /// 移动源位置信息
    source: Dimension,
    /// 移动的目标位置行或者列
    destination_index: Option<i32>,
}

#[derive(Serialize, Default, Debug)]
struct Dimension {
    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    major_dimension: String,
    /// 要移动的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4
    /// 行或列开始移动。包含第 4 行或列。
    start_index: i32,
    /// 要移动的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则要移动的范围至第 8
    /// 行或列结束。不包含第 8 行或列。
    ///
    /// 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7 时，则移动第 4、5、6、7
    /// 行，共 4 行。
    end_index: i32,
}

impl MoveDimensionRequest {
    pub fn builder() -> MoveDimensionRequestBuilder {
        MoveDimensionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct MoveDimensionRequestBuilder {
    request: MoveDimensionRequest,
}

impl MoveDimensionRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    pub fn major_dimension(mut self, major_dimension: impl ToString) -> Self {
        self.request.source.major_dimension = major_dimension.to_string();
        self
    }

    /// 要移动的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4 行或列开始移动。包含第
    /// 4 行或列。
    pub fn start_index(mut self, start_index: i32) -> Self {
        self.request.source.start_index = start_index;
        self
    }

    /// 要移动的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则要移动的范围至第 8
    /// 行或列结束。不包含第 8 行或列。
    ///
    /// 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7 时，则移动第 4、5、6、7
    /// 行，共 4 行。
    pub fn end_index(mut self, end_index: i32) -> Self {
        self.request.source.end_index = end_index;
        self
    }

    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。
    /// 可选值：
    /// - BEFORE：继承起始位置的单元格的样式
    /// - AFTER：继承结束位置的单元格的样式
    pub fn destination_index(mut self, index: i32) -> Self {
        self.request.destination_index = Some(index);
        self
    }

    pub fn build(mut self) -> MoveDimensionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetService {
    /// 移动行列
    pub async fn move_dimension(
        &self,
        request: MoveDimensionRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_MOVE_DIMENSION
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
