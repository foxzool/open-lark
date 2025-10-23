use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult},
    SDKResult,
},
use crate::impl_full_service;
use async_trait::async_trait;
/// 文件服务
pub struct FileService {
    pub config: Config,
}
/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// 文件的key
    pub file_key: String,
impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
/// 下载文件响应
pub struct GetFileResponse {
    /// 文件数据
    pub data: Vec<u8>,
impl ApiResponseTrait for GetFileResponse {
impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    /// 上传文件
    pub async fn create(
        &self,
        file_type: &str,
        file_name: &str,
        file_data: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateFileResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("file_type", file_type.to_string());
        query_params.insert("file_name", file_name.to_string());
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_FILES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = file_data;
        let api_resp: BaseResponse<CreateFileResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    /// 下载文件
    pub async fn get(
        file_key: &str,
    ) -> SDKResult<GetFileResponse> {
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::im::IM_V1_FILES_DOWNLOAD,
            "file_key",
            file_key,
        ));
        let api_resp: BaseResponse<GetFileResponse> =
    /// 创建文件上传Builder (推荐)
    pub fn upload_builder(&self) -> FileUploadBuilder {
        FileUploadBuilder::new()
    /// 创建文件下载Builder (推荐)
    pub fn download_builder(&self) -> FileDownloadBuilder {
        FileDownloadBuilder::new()
// 接入统一 Service 抽象（IM v1 - FileService）
impl_full_service!(FileService, "im.file", "v1");
/// 文件上传请求结构
#[derive(Debug, Clone, Default)]
pub struct FileUploadRequest {
    /// 文件类型
    pub file_type: String,
    /// 文件名
    pub file_name: String,
    pub file_data: Vec<u8>,
/// 文件上传Builder
#[derive(Default)]
pub struct FileUploadBuilder {
    request: FileUploadRequest,
impl FileUploadBuilder {
    pub fn new() -> Self {
        Self::default()
    /// 设置文件类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.request.file_type = file_type.to_string();
        self
    /// 设置文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.request.file_name = file_name.to_string();
    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.request.file_data = file_data;
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im
    /// 构建文件上传请求
    pub fn build(self) -> SDKResult<FileUploadRequest> {
        // 验证文件类型
        if self.request.file_type.is_empty() {
            return Err(LarkAPIError::illegal_param(
                "file_type is required".to_string(),
            ));
        }
        // 验证文件名
