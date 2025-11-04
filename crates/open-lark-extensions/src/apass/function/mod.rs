use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::apass::models::{FunctionInvokeRequest, FunctionInvokeResult},
};

/// 函数执行服务
pub struct FunctionService {
    pub config: Config,
}

/// 函数执行响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInvokeResponse {
    /// 函数执行结果
    #[serde(flatten)]
    pub invoke_result: FunctionInvokeResult,
}

impl ApiResponseTrait for FunctionInvokeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FunctionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行函数
    ///
    /// 该接口用于执行低代码平台中的自定义函数。
    ///
    /// # 参数
    ///
    /// - `request`: 函数执行请求参数
    /// - `option`: 可选的请求配置
    pub async fn invoke_function(
        &self,
        request: FunctionInvokeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FunctionInvokeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FUNCTION_INVOKE,
                &[
                    ("app_id", &request.app_id),
                    ("function_name", &request.function_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "parameters": request.parameters
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
