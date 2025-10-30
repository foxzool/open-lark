use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::apass::models::{FunctionInvokeRequest, FunctionInvokeResult}
};
/// 函数执行服务
pub struct FunctionService {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl FunctionService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 执行函数
    ///,
/// 该接口用于执行低代码平台中的自定义函数。
    ///,
/// # 参数
    ///,
/// - `request`: 函数执行请求参数
    /// - `option`: 可选的请求配置
pub async fn invoke_function(,
        &self,
        request: FunctionInvokeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FunctionInvokeResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FUNCTION_INVOKE,
                &[
                    ("app_id", &request.app_id),
                    ("function_name", &request.function_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({,
"parameters": request.parameters,
            }))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}