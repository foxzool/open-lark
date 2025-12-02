
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    api::,
{,
        BaseResponse,
        BinaryResponse,
        ResponseFormat, HttpMethod,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult};
        SDKResult,
    }
    impl_executable_builder_owned,
};
use log;
/// 素材服务,
pub struct MediaService {
    config: Config}
impl MediaService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建上传素材Builder,
    pub fn w+.*{
UploadMediaRequestBuilder::default()}
/// 使用Builder上传素材（带验证）,
    pub async fn upload_all_with_builder(
        &self,
        builder_result: SDKResult<UploadMediaRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadMediaRespData>> {,
let request = builder_result?;
        self.upload_all(request, option).await}
/// 上传素材,
    ///,
/// 该接口用于上传素材文件。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn upload_all(,
        &self,
        request: UploadMediaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadMediaRespData>> {,
let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
api_req.set_api_path(DRIVE_V1_MEDIAS_UPLOAD_ALL.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp)}
/// 分片上传素材-预上传,
    ///,
/// 该接口用于分片上传的预上传步骤。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn upload_prepare(,
        &self,
        request: UploadPrepareRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadPrepareRespData>> {,
let api_req = ApiRequest {,
            http_http_method: Method::POST,
            url: DRIVE_V1_MEDIAS_UPLOAD_PREPARE.to_string(),
            // supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            body: Some(openlark_core::api::RequestData::Json(&request))?,
            };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 分片上传素材-上传分片,
    ///,
/// 该接口用于上传文件分片。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn upload_part(,
        &self,
        request: UploadPartRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadPartRespData>> {,
let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
api_req.set_api_path(DRIVE_V1_MEDIAS_UPLOAD_PART.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp)}
/// 分片上传素材-完成上传,
    ///,
/// 该接口用于完成分片上传。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn upload_finish(,
        &self,
        request: UploadFinishRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadFinishRespData>> {,
let api_req = ApiRequest {,
            http_http_method: Method::POST,
            url: DRIVE_V1_MEDIAS_UPLOAD_FINISH.to_string(),
            // supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            body: Some(openlark_core::api::RequestData::Json(&request))?,
            };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 下载素材,
    ///,
/// 该接口用于下载素材文件。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn download(,
        &self,
        request: DownloadMediaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BinaryResponse>> {,
let api_req = ApiRequest {,
            http_http_method: Method::GET,
            url: DRIVE_V1_MEDIAS_DOWNLOAD.replace("{}", &request.file_token),
            // supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            
};

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 获取素材临时下载链接,
    ///,
/// 该接口用于获取素材的临时下载链接。,
    ///,
/// # API文档,
    ///,
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme,
pub async fn batch_get_tmp_download_url(,
        &self,
        request: BatchGetTmpDownloadUrlRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BatchGetTmpDownloadUrlRespData>> {,
let mut api_req = ApiRequest {,
            http_http_method: Method::GET,
            url: DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL.to_string(),
            // supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            };
// 添加查询参数,
        let file_tokens = request.file_tokens.join(",");
