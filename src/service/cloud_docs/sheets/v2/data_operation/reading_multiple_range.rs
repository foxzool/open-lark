use crate::core::endpoints::Endpoints;
use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::{data_operation::ValueRangeResponse, SpreadsheetService},
};

/// 读取单个范围请求
#[derive(Debug, Default)]
pub struct ReadingMultipleRangeRequest {
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 多个查询范围，范围之间使用逗号分隔，如range1,range2。其中 range 包含 sheetId
    /// 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    /// 。若查询范围中使用形如`<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据
    ranges: String,
    /// 指定单元格数据的格式。可选值为如下所示。当参数缺省时，默认不进行公式计算，返回公式本身；
    /// 数值不进行数字格式化。
    ///
    /// - valueRenderOption=ToString：返回纯文本的值（数值类型除外）
    /// - valueRenderOption=FormattedValue：计算并格式化单元格
    /// - valueRenderOption=Formula：单元格中含有公式时，返回公式本身
    /// - valueRenderOption=UnformattedValue：计算但不对单元格进行格式化
    value_render_option: Option<String>,
    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式。
    ///
    /// - 当参数缺省时，默认返回浮点数值，整数部分为自 1899 年 12 月 30
    ///   日以来的天数；小数部分为该时间占 24 小时的份额。例如：若时间为 1900 年 1 月 1 日中午 12
    ///   点，则默认返回 2.5。其中，2 表示 1900 年 1 月 1 日为 1899 年12 月 30 日之后的 2 天；0.5
    ///   表示 12 点占 24 小时的二分之一，即 12/24=0.5。
    /// - dateTimeRenderOption=FormattedString：计算并对时间、日期类型数据进行格式化，
    ///   但不会对数字进行格式化。将返回格式化后的字符串。详见电子表格常见问题
    date_time_render_option: Option<String>,
    /// 当单元格中包含@用户等涉及用户信息的元素时，该参数可指定返回的用户 ID 类型。默认为
    /// lark_id，建议选择 open_id 或 union_id。了解更多，参考用户身份概述。
    ///
    /// - open_id：用户在应用内的身份。 同一个 user_id 在不同应用中的 open_id 不同，其值统一以 ou_
    ///   为前缀，如ou_c99c5f35d542efc7ee492afe11af19ef。
    /// - union_id：用户在同一应用服务商提供的多个应用间的统一身份。
    user_id_type: Option<String>,
}

impl ReadingMultipleRangeRequest {
    pub fn builder() -> ReadingMultiRangesRequestBuilder {
        ReadingMultiRangesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReadingMultiRangesRequestBuilder {
    request: ReadingMultipleRangeRequest,
}

impl ReadingMultiRangesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 多个查询范围，范围之间使用逗号分隔，如range1,range2。⁣其中 range 包含 sheetId
    /// 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    /// 。若查询范围中使用形如`<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据。
    pub fn ranges(mut self, ranges: impl ToString) -> Self {
        self.request.ranges = ranges.to_string();
        self.request
            .api_request
            .query_params
            .insert("ranges", ranges.to_string());
        self
    }

    /// 指定单元格数据的格式。可选值为如下所示。当参数缺省时，默认不进行公式计算，返回公式本身；
    /// 数值不进行数字格式化。
    ///
    /// - valueRenderOption=ToString：返回纯文本的值（数值类型除外）
    /// - valueRenderOption=FormattedValue：计算并格式化单元格
    /// - valueRenderOption=Formula：单元格中含有公式时，返回公式本身
    /// - valueRenderOption=UnformattedValue：计算但不对单元格进行格式化
    pub fn value_render_option(mut self, value_render_option: impl ToString) -> Self {
        self.request.value_render_option = Some(value_render_option.to_string());
        self.request
            .api_request
            .query_params
            .insert("valueRenderOption", value_render_option.to_string());
        self
    }

    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式。
    ///
    /// - 当参数缺省时，默认返回浮点数值，整数部分为自 1899 年 12 月 30
    ///   日以来的天数；小数部分为该时间占 24 小时的份额。例如：若时间为 1900 年 1 月 1 日中午 12
    ///   点，则默认返回 2.5。其中，2 表示 1900 年 1 月 1 日为 1899 年12 月 30 日之后的 2 天；0.5
    ///   表示 12 点占 24 小时的二分之一，即 12/24=0.5。
    /// - dateTimeRenderOption=FormattedString：计算并对时间、日期类型数据进行格式化，
    ///   但不会对数字进行格式化。将返回格式化后的字符串。详见电子表格常见问题
    pub fn date_time_render_option(mut self, date_time_render_option: impl ToString) -> Self {
        self.request.date_time_render_option = Some(date_time_render_option.to_string());
        self.request
            .api_request
            .query_params
            .insert("dateTimeRenderOption", date_time_render_option.to_string());
        self
    }

    /// 当单元格中包含@用户等涉及用户信息的元素时，该参数可指定返回的用户 ID 类型。默认为
    /// lark_id，建议选择 open_id 或 union_id。了解更多，参考用户身份概述。
    ///
    /// - open_id：用户在应用内的身份。 同一个 user_id 在不同应用中的 open_id 不同，其值统一以 ou_
    ///   为前缀，如ou_c99c5f35d542efc7ee492afe11af19ef。
    /// - union_id：用户在同一应用服务商提供的多个应用间的统一身份。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self.request
            .api_request
            .query_params
            .insert("user_id_type", user_id_type.to_string());
        self
    }

    pub fn build(self) -> ReadingMultipleRangeRequest {
        self.request
    }
}

/// 读取数据响应体
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ReadingMultiRangesResponse {
    /// sheet 的版本号
    pub revision: i32,
    /// spreadsheet 的 token，详见电子表格概述
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 读取的单元格总数
    #[serde(rename = "totalCells")]
    pub total_cells: i32,
    /// 值与范围
    #[serde(rename = "valueRanges")]
    pub value_ranges: Vec<ValueRangeResponse>,
}

impl ApiResponseTrait for ReadingMultiRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 读取多个范围
    pub async fn reading_multi_ranges(
        &self,
        request: ReadingMultipleRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingMultiRangesResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = Endpoints::SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET
            .replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
