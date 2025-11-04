use serde::{Deserialize, Serialize};

use open_lark_core::core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::directory::v1::employee::models::Employee;

/// 即将离职员工请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ToBeResignedRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,


/// 即将离职员工响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ToBeResignedResponse {
    /// 处理结果
    pub failed: Vec<FailedItem>,


/// 失败项
#[derive(Debug, Serialize, Deserialize)]
pub struct FailedItem {
    /// 用户ID
    pub user_id: String,
    /// 错误信息
    pub error_message: String,


impl ApiResponseTrait for ToBeResignedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }


impl crate::directory::v1::EmployeeService {
    /// 即将离职员工
    pub async fn to_be_resigned(
        &self,
        request: ToBeResignedRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ToBeResignedResponse>> {
        let mut api_req = open_lark_core::core::api_req::ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: open_lark_core::core::endpoints::directory::DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED.to_string(),
            supported_access_token_types: vec![
                open_lark_core::core::constants::AccessTokenType::Tenant,
                open_lark_core::core::constants::AccessTokenType::User
            ],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
