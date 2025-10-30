use reqwest::Method;
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
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppService;
impl AppService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 复制多维表格请求,
#[derive(Debug, Clone)]
pub struct CopyAppRequest {
    api_request: ApiRequest,
    /// 待复制的多维表格的 app_token
    app_token: String,
    /// 复制的多维表格 App 名字
    name: Option<String>,
    /// 复制的多维表格所在文件夹的 token
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>}
impl CopyAppRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CopyAppRequestBuilder {
    request: CopyAppRequest}
impl CopyAppRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CopyAppRequestBuilder,
    AppService,
    CopyAppRequest,
    BaseResponse<CopyAppResponse>,
    copy,
);
#[derive(Serialize)]
struct CopyAppRequestBody {,
#[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>}

#[derive(Debug, Clone)]
pub struct CopyAppResponse {
    /// 复制的多维表格的 app 信息
    pub app: CopyAppResponseData,

#[derive(Debug, Clone)]
pub struct CopyAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_copy_app_request() {
        let request = CopyAppRequest::builder(),
.app_token()
            .name()
.folder_token()
            .time_zone()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, Some("复制的多维表格".to_string()));
assert_eq!(,
            request.folder_token,
            Some("fldcnmBA*****yGehy8".to_string()),
);
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
#[test]
    fn test_copy_app_request_body_serialization() {
let body = CopyAppRequestBody {,
            name: Some("复制的多维表格".to_string()),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string())};
let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "复制的多维表格",
            "folder_token": "fldcnmBA*****yGehy8",
            "time_zone": "Asia/Shanghai"});

        assert_eq!(serialized, expected);
