use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use openlark_core::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::directory::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::directory::v1::department::models::Department;

/// 创建部门请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    /// 部门名称
    pub name: String,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 部门父ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门主管ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,


/// 创建部门响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepartmentResponse {
    /// 部门信息
    pub department: Department,


impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }


impl crate::directory::v1::DepartmentService {
    /// 创建部门
    pub async fn create(
        &self,
        request: CreateDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateDepartmentResponse>> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: openlark_core::endpoints::directory::DIRECTORY_V1_DEPARTMENT_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
