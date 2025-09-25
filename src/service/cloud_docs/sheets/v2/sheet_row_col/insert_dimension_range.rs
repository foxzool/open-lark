use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService},
};

/// 插入行列请求
#[derive(Serialize, Default, Debug)]
pub struct InsertDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要插入行列的维度信息
    dimension: UpdateDimension,
    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。
    /// 可选值：
    /// - BEFORE：继承起始位置的单元格的样式
    /// - AFTER：继承结束位置的单元格的样式
    #[serde(rename = "inheritStyle")]
    inherit_style: Option<String>,
}

impl InsertDimensionRangeRequest {
    pub fn builder() -> InsertDimensionRangeRequestBuilder {
        InsertDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct InsertDimensionRangeRequestBuilder {
    request: InsertDimensionRangeRequest,
}

impl InsertDimensionRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.dimension.sheet_id = sheet_id.to_string();
        self
    }

    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    pub fn major_dimension(mut self, major_dimension: impl ToString) -> Self {
        self.request.dimension.major_dimension = major_dimension.to_string();
        self
    }

    /// 插入的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4
    /// 行或列开始插入空行或列。包含第 4 行或列。
    pub fn start_index(mut self, start_index: i32) -> Self {
        self.request.dimension.start_index = start_index;
        self
    }

    /// 插入的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则从第 8 行结束插入行。第 8
    /// 行不再插入空行。 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7
    /// 时，则在第 4、5、6、7 行插入空白行，共插入 4 行。
    pub fn end_index(mut self, end_index: i32) -> Self {
        self.request.dimension.end_index = end_index;
        self
    }

    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。
    /// 可选值：
    /// - BEFORE：继承起始位置的单元格的样式
    /// - AFTER：继承结束位置的单元格的样式
    pub fn inherit_style(mut self, inherit_style: impl ToString) -> Self {
        self.request.inherit_style = Some(inherit_style.to_string());
        self
    }

    pub fn build(mut self) -> InsertDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    InsertDimensionRangeRequestBuilder,
    SpreadsheetService,
    InsertDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    insert_dimension_range
);

impl SpreadsheetService {
    /// 插入行列
    pub async fn insert_dimension_range(
        &self,
        request: InsertDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
