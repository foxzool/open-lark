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
        trait_system::Service,
        SDKResult,
    },
    service::okr::models::{ProgressAttachment, ProgressRecord, ProgressRecordType},
};

/// OKR 进展记录管理服务
pub struct ProgressRecordService {
    pub config: Config,
}

/// 进展记录创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressRecordCreateResponse {
    /// 创建的进展记录
    pub progress_record: ProgressRecord,
}

impl ApiResponseTrait for ProgressRecordCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 进展记录删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressRecordDeleteResponse {
    /// 删除结果
    pub success: bool,
}

impl ApiResponseTrait for ProgressRecordDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 进展记录更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressRecordUpdateResponse {
    /// 更新后的进展记录
    pub progress_record: ProgressRecord,
}

impl ApiResponseTrait for ProgressRecordUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 进展记录详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressRecordGetResponse {
    /// 进展记录详情
    pub progress_record: ProgressRecord,
}

impl ApiResponseTrait for ProgressRecordGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 图片上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadResponse {
    /// 上传的图片信息
    pub attachment: ProgressAttachment,
}

impl ApiResponseTrait for ImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ProgressRecordService {
    /// 创建 OKR 进展记录管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建 OKR 进展记录
    ///
    /// 为指定的 OKR 创建新的进展记录，记录目标的最新进展情况。
    ///
    /// # Arguments
    ///
    /// * `request` - 创建请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的进展记录信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::okr::progress_record::*;
    /// use open_lark::service::okr::models::ProgressRecordType;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = ProgressRecordCreateRequest {
    ///         okr_id: "okr_123".to_string(),
    ///         content: "本周完成了核心功能开发，目标完成度达到80%".to_string(),
    ///         progress_rate: Some(80.0),
    ///         record_type: Some(ProgressRecordType::Detail),
    ///         attachment_ids: Some(vec![]),
    ///     };
    ///
    ///     let response = client.okr.progress_record.create_progress_record(request, None).await?;
    ///     println!("进展记录ID: {}", response.data.unwrap().progress_record.progress_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_progress_record(
        &self,
        request: ProgressRecordCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ProgressRecordCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::OKR_V1_PROGRESS_RECORDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除 OKR 进展记录
    ///
    /// 删除指定的进展记录。
    ///
    /// # Arguments
    ///
    /// * `progress_id` - 进展记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_progress_record(
        &self,
        progress_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ProgressRecordDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::OKR_V1_PROGRESS_RECORD_OPERATION,
                "progress_id",
                progress_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新 OKR 进展记录
    ///
    /// 更新指定进展记录的内容和进度信息。
    ///
    /// # Arguments
    ///
    /// * `progress_id` - 进展记录ID
    /// * `request` - 更新请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新后的进展记录信息
    pub async fn update_progress_record(
        &self,
        progress_id: &str,
        request: ProgressRecordUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ProgressRecordUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                Endpoints::OKR_V1_PROGRESS_RECORD_OPERATION,
                "progress_id",
                progress_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取 OKR 进展记录
    ///
    /// 获取指定进展记录的详细信息。
    ///
    /// # Arguments
    ///
    /// * `progress_id` - 进展记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回进展记录详细信息
    pub async fn get_progress_record(
        &self,
        progress_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ProgressRecordGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::OKR_V1_PROGRESS_RECORD_OPERATION,
                "progress_id",
                progress_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传进展记录图片
    ///
    /// 上传图片到进展记录中作为附件。
    ///
    /// # Arguments
    ///
    /// * `request` - 图片上传请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回上传的图片信息
    pub async fn upload_progress_image(
        &self,
        request: ImageUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ImageUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::OKR_V1_PROGRESS_RECORDS_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for ProgressRecordService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "progress_record"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 进展记录创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecordCreateRequest {
    /// OKR ID
    pub okr_id: String,
    /// 进展内容
    pub content: String,
    /// 记录类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<ProgressRecordType>,
    /// 进度百分比 (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 附件ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

/// 进展记录更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecordUpdateRequest {
    /// 进展内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 进度百分比 (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 附件ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

/// 图片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUploadRequest {
    /// 图片文件名
    pub file_name: String,
    /// 图片内容 (base64编码)
    pub file_content: String,
    /// 图片类型 (如: image/png, image/jpeg)
    pub file_type: String,
}
