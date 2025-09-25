use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService},
};

/// 删除行列请求
#[derive(Serialize, Debug, Default)]
pub struct DeleteDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要删除行列的范围信息
    dimension: UpdateDimension,
}

impl DeleteDimensionRangeRequest {
    pub fn builder() -> DeleteDimensionRangeRequestBuilder {
        DeleteDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteDimensionRangeRequestBuilder {
    request: DeleteDimensionRangeRequest,
}

impl DeleteDimensionRangeRequestBuilder {
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

    pub fn build(mut self) -> DeleteDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteDimensionRangeRequestBuilder,
    SpreadsheetService,
    DeleteDimensionRangeRequest,
    BaseResponse<DeleteDimensionRangeResponse>,
    delete_dimension_range
);

impl SpreadsheetService {
    /// 该接口用于删除电子表格中的指定行或列。
    pub async fn delete_dimension_range(
        &self,
        request: DeleteDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteDimensionRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::DELETE;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除行列响应体
#[derive(Deserialize, Debug)]
pub struct DeleteDimensionRangeResponse {
    /// 一共删除的行数或列数
    #[serde(rename = "delCount")]
    pub del_count: i32,
    /// 删除的维度。枚举值：
    /// - ROWS：行
    /// - COLUMNS：列
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
