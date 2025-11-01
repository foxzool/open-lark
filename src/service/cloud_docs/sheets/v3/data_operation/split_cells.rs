use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 拆分单元格请求,
#[derive(Debug, Clone)]
pub struct SplitCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 拆分范围
    range: String}
impl SplitCellsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct SplitCellsRequestBuilder {
    request: SplitCellsRequest}
impl SplitCellsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 拆分单元格响应体最外层,
#[derive(Debug, Clone)]
pub struct SplitCellsResponseData {
    /// 拆分后的范围
    pub unmerged_range: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl_executable_builder_owned!(,
    SplitCellsRequestBuilder,
    DataOperationService,
    SplitCellsRequest,
    SplitCellsResponseData,
    split_cells,
);
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::SplitCellsResponseData;
    #[test]
fn test_split_cells_response() {
        let json = json!({,
"unmerged_range": "A1:C3"});
let response: SplitCellsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.unmerged_range, "A1:C3");
