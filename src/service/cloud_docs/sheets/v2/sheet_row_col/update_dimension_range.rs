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

/// 更新行列请求
#[derive(Serialize, Debug, Default)]
pub struct UpdateDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要更新行列的维度信息
    dimension: UpdateDimension,
    /// 更新行或列的属性。至少写入以下参数之一
    #[serde(rename = "dimensionProperties")]
    dimension_properties: DimensionProperties,
}

/// 更新行或列的属性。至少写入以下参数之一
#[derive(Serialize, Debug, Default)]
struct DimensionProperties {
    visible: Option<bool>,
    #[serde(rename = "fixedSize")]
    fixed_size: Option<i32>,
}

impl UpdateDimensionRangeRequest {
    pub fn builder() -> UpdateDimensionRangeRequestBuilder {
        UpdateDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateDimensionRangeRequestBuilder {
    request: UpdateDimensionRangeRequest,
}

impl UpdateDimensionRangeRequestBuilder {
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

    /// 是否隐藏行或列。可选值：
    /// - true：显示行或列
    /// - false：隐藏行或列
    pub fn visible(mut self, visible: bool) -> Self {
        self.request.dimension_properties.visible = Some(visible);
        self
    }

    /// 行高或列宽。单位为像素。fixedSize 为 0 时，等价于隐藏行或列。
    pub fn fixed_size(mut self, fixed_size: i32) -> Self {
        self.request.dimension_properties.fixed_size = Some(fixed_size);
        self
    }

    pub fn build(mut self) -> UpdateDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateDimensionRangeRequestBuilder,
    SpreadsheetService,
    UpdateDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    update_dimension_range
);

impl SpreadsheetService {
    /// 该接口用于更新设置电子表格中行列的属性，包括是否隐藏行列和设置行高列宽。
    pub async fn update_dimension_range(
        &self,
        request: UpdateDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
