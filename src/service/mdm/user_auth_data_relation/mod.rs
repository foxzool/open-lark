use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::mdm::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::mdm::models::UserAuthDataRelation,
};
/// 用户数据维度管理服务
pub struct UserAuthDataRelationService {
}
    pub config: Config,
impl UserAuthDataRelationService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 用户数据维度绑定
    ///,
/// 将用户与指定的数据维度建立绑定关系，授予用户对该数据维度的访问权限。
    ///,
/// # Arguments
    ///,
/// * `request` - 数据维度绑定请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回绑定结果
pub async fn bind(,
        &self,
        request: UserDataRelationBindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserDataRelationBindResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: MDM_V1_USER_AUTH_DATA_RELATIONS_BIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 用户数据维度解绑
    ///,
/// 解除用户与指定数据维度的绑定关系，取消用户对该数据维度的访问权限。
    ///,
/// # Arguments
    ///,
/// * `request` - 数据维度解绑请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回解绑结果
pub async fn unbind(,
        &self,
        request: UserDataRelationUnbindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserDataRelationUnbindResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
impl Service for UserAuthDataRelationService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "user_auth_data_relation",
fn service_version() -> &'static str {,
        "v1",
/// 用户数据维度绑定请求
#[derive(Debug, Clone)]
}
pub struct UserDataRelationBindRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 用户数据维度解绑请求
#[derive(Debug, Clone)]
}
pub struct UserDataRelationUnbindRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}}}}}