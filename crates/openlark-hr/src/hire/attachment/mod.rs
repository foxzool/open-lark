use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::hire::*,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::hire::models::{CommonResponse, PageResponse};

/// 附件服务
pub struct AttachmentService {
    pub config: Config,
}

/// 附件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// 附件ID
    pub id: String,
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub attachment_type: String,
    /// 文件类型
    pub file_type: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 附件URL
    pub url: String,
    /// 下载URL
    pub download_url: Option<String>,
    /// 预览URL
    pub preview_url: Option<String>,
    /// 关联对象类型
    pub object_type: String,
    /// 关联对象ID
    pub object_id: String,
    /// 上传人ID
    pub uploader_id: String,
    /// 附件描述
    pub description: Option<String>,
    /// 是否公开
    pub is_public: bool,
    /// 过期时间
    pub expiration_time: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 附件上传请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttachmentUploadRequest {
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub attachment_type: String,
    /// 关联对象类型
    pub object_type: String,
    /// 关联对象ID
    pub object_id: String,
    /// 附件描述
    pub description: Option<String>,
    /// 是否公开
    pub is_public: Option<bool>,
    /// 过期时间
    pub expiration_time: Option<String>,
}

/// 附件上传信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentUploadInfo {
    /// 上传URL
    pub upload_url: String,
    /// 上传方法
    pub upload_method: String,
    /// 上传头部信息
    pub upload_headers: Option<std::collections::HashMap<String, String>>,
    /// 附件ID
    pub attachment_id: String,
    /// 过期时间
    pub expires_at: String,
}

/// 附件更新请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttachmentUpdateRequest {
    /// 附件名称
    pub name: Option<String>,
    /// 附件描述
    pub description: Option<String>,
    /// 是否公开
    pub is_public: Option<bool>,
    /// 过期时间
    pub expiration_time: Option<String>,
}

/// 附件列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttachmentListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 关联对象类型
    pub object_type: Option<String>,
    /// 关联对象ID
    pub object_id: Option<String>,
    /// 附件类型
    pub attachment_type: Option<String>,
    /// 上传人ID
    pub uploader_id: Option<String>,
    /// 创建时间开始
    pub created_start_time: Option<String>,
    /// 创建时间结束
    pub created_end_time: Option<String>,
}

/// 批量下载请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchDownloadRequest {
    /// 附件ID列表
    pub attachment_ids: Vec<String>,
    /// 压缩包名称
    pub archive_name: Option<String>,
}

/// 附件列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentListResponse {
    /// 附件列表
    #[serde(flatten)]
    pub attachments: PageResponse<Attachment>,
}

impl ApiResponseTrait for AttachmentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentDetailResponse {
    /// 附件信息
    pub attachment: Attachment,
}

impl ApiResponseTrait for AttachmentDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentUploadResponse {
    /// 上传信息
    pub upload_info: AttachmentUploadInfo,
}

impl ApiResponseTrait for AttachmentUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDownloadResponse {
    /// 下载链接
    pub download_url: String,
    /// 过期时间
    pub expires_at: String,
}

impl ApiResponseTrait for BatchDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
}

impl ApiResponseTrait for AttachmentOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AttachmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建附件上传任务
    ///
    /// 该接口用于创建附件上传任务，获取上传URL和相关
    /// 上传信息。客户端可以使用返回的信息直接上传文件。
    ///
    /// # 参数
    ///
    /// - `request`: 附件上传请求参数，包括：
    ///   - `name`: 附件名称（必填）
    ///   - `attachment_type`: 附件类型（必填）
    ///   - `object_type`: 关联对象类型（必填）
    ///   - `object_id`: 关联对象ID（必填）
    ///   - `description`: 附件描述
    ///   - `is_public`: 是否公开
    ///   - `expiration_time`: 过期时间
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回附件上传信息，包括：
    /// - 上传URL和方法
    /// - 上传头部信息
    /// - 附件ID和过期时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::attachment::AttachmentUploadRequest;
    ///
    /// let request = AttachmentUploadRequest {
    ///     name: "候选人简历.pdf".to_string(),
    ///     attachment_type: "resume".to_string(),
    ///     object_type: "talent".to_string(),
    ///     object_id: "talent_123456".to_string(),
    ///     description: Some("候选人的最新简历".to_string()),
    ///     is_public: Some(false),
    ///     expiration_time: Some("2024-12-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.attachment.create_upload_task(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("上传URL: {}", data.upload_info.upload_url);
    ///     println!("附件ID: {}", data.upload_info.attachment_id);
    /// }
    /// ```
    pub async fn create_upload_task(
        &self,
        request: AttachmentUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentUploadResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_ATTACHMENT_UPLOAD.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件详情
    ///
    /// 该接口用于获取指定附件的详细信息，包括附件
    /// 基本信息、下载链接、预览链接等完整数据。
    ///
    /// # 参数
    ///
    /// - `attachment_id`: 附件ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回附件详细信息，包括：
    /// - 附件基本信息（名称、类型、大小等）
    /// - 下载和预览链接
    /// - 关联对象信息
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let attachment_id = "attachment_123456";
    /// let response = client.hire.attachment.get_attachment_detail(attachment_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("附件名称: {}", data.attachment.name);
    ///     println!("文件大小: {} 字节", data.attachment.file_size);
    ///     println!("下载链接: {:?}", data.attachment.download_url);
    /// }
    /// ```
    pub async fn get_attachment_detail(
        &self,
        attachment_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_ATTACHMENT_GET,
            "attachment_id",
            attachment_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件列表
    ///
    /// 该接口用于获取附件列表，支持按关联对象、
    /// 附件类型、上传人、时间等条件筛选。返回的
    /// 列表包含附件基本信息，可用于附件管理。
    ///
    /// # 参数
    ///
    /// - `request`: 附件列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `object_type`: 关联对象类型筛选
    ///   - `object_id`: 关联对象ID筛选
    ///   - `attachment_type`: 附件类型筛选
    ///   - `uploader_id`: 上传人ID筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的附件列表，包括：
    /// - 附件基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::attachment::AttachmentListRequest;
    ///
    /// let request = AttachmentListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     object_type: Some("talent".to_string()),
    ///     object_id: Some("talent_123456".to_string()),
    ///     attachment_type: Some("resume".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.attachment.list_attachments(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("附件总数: {}", data.attachments.items.len());
    ///     for attachment in &data.attachments.items {
    ///         println!("附件: {} ({})", attachment.name, attachment.file_type);
    ///     }
    /// }
    /// ```
    pub async fn list_attachments(
        &self,
        request: AttachmentListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_ATTACHMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(object_type) = request.object_type {
            api_req.query_params.insert("object_type", object_type);
        }

        if let Some(object_id) = request.object_id {
            api_req.query_params.insert("object_id", object_id);
        }

        if let Some(attachment_type) = request.attachment_type {
            api_req
                .query_params
                .insert("attachment_type", attachment_type);
        }

        if let Some(uploader_id) = request.uploader_id {
            api_req.query_params.insert("uploader_id", uploader_id);
        }

        if let Some(created_start_time) = request.created_start_time {
            api_req
                .query_params
                .insert("created_start_time", created_start_time);
        }

        if let Some(created_end_time) = request.created_end_time {
            api_req
                .query_params
                .insert("created_end_time", created_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新附件信息
    ///
    /// 该接口用于更新附件的基本信息，包括名称、
    /// 描述、公开状态等属性。
    ///
    /// # 参数
    ///
    /// - `attachment_id`: 附件ID
    /// - `request`: 附件更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::attachment::AttachmentUpdateRequest;
    ///
    /// let attachment_id = "attachment_123456";
    /// let request = AttachmentUpdateRequest {
    ///     name: Some("更新后的简历.pdf".to_string()),
    ///     description: Some("候选人的最新简历版本".to_string()),
    ///     is_public: Some(true),
    ///     expiration_time: Some("2025-12-31T23:59:59Z".to_string()),
    /// };
    ///
    /// let response = client.hire.attachment.update_attachment(attachment_id, request, None).await?;
    /// ```
    pub async fn update_attachment(
        &self,
        attachment_id: &str,
        request: AttachmentUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_ATTACHMENT_GET,
            "attachment_id",
            attachment_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();

        Transport::request(api_req, &self.config, option).await
    }
    /// 删除附件
    ///
    /// 该接口用于删除指定的附件文件，删除后
    /// 附件将无法再次访问和下载。
    ///
    /// # 参数
    ///
    /// - `attachment_id`: 附件ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let attachment_id = "attachment_123456";
    /// let response = client.hire.attachment.delete_attachment(attachment_id, None).await?;
    /// ```
    pub async fn delete_attachment(
        &self,
        attachment_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_ATTACHMENT_GET,
            "attachment_id",
            attachment_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件下载链接
    ///
    /// 该接口用于获取附件的临时下载链接，
    /// 链接具有一定的有效期。
    ///
    /// # 参数
    ///
    /// - `attachment_id`: 附件ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let attachment_id = "attachment_123456";
    /// let response = client.hire.attachment.get_download_url(attachment_id, None).await?;
    /// ```
    pub async fn get_download_url(
        &self,
        attachment_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_ATTACHMENT_DOWNLOAD,
            "attachment_id",
            attachment_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件预览链接
    ///
    /// 该接口用于获取附件的预览链接，支持在线
    /// 预览文档、图片等格式的附件。
    ///
    /// # 参数
    ///
    /// - `attachment_id`: 附件ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let attachment_id = "attachment_123456";
    /// let response = client.hire.attachment.get_preview_url(attachment_id, None).await?;
    /// ```
    pub async fn get_preview_url(
        &self,
        attachment_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_ATTACHMENT_PREVIEW,
            "attachment_id",
            attachment_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量下载附件
    ///
    /// 该接口用于批量下载多个附件，系统会将
    /// 选中的附件打包成压缩文件供下载。
    ///
    /// # 参数
    ///
    /// - `request`: 批量下载请求参数，包括：
    ///   - `attachment_ids`: 附件ID列表（必填）
    ///   - `archive_name`: 压缩包名称
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回批量下载信息，包括：
    /// - 下载链接和过期时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::attachment::BatchDownloadRequest;
    ///
    /// let request = BatchDownloadRequest {
    ///     attachment_ids: vec![
    ///         "attachment_001".to_string(),
    ///         "attachment_002".to_string(),
    ///         "attachment_003".to_string(),
    ///     ],
    ///     archive_name: Some("候选人材料.zip".to_string()),
    /// };
    ///
    /// let response = client.hire.attachment.batch_download(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("下载链接: {}", data.download_url);
    ///     println!("过期时间: {}", data.expires_at);
    /// }
    /// ```
    pub async fn batch_download(
        &self,
        request: BatchDownloadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BatchDownloadResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_ATTACHMENTS_BATCH_DOWNLOAD.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量删除附件
    ///
    /// 该接口用于批量删除多个附件，
    /// 提高操作效率。
    ///
    /// # 参数
    ///
    /// - `attachment_ids`: 附件ID列表
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let attachment_ids = vec![
    ///     "attachment_001".to_string(),
    ///     "attachment_002".to_string(),
    ///     "attachment_003".to_string(),
    /// ];
    ///
    /// let response = client.hire.attachment.batch_delete(attachment_ids, None).await?;
    /// ```
    pub async fn batch_delete(
        &self,
        attachment_ids: Vec<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AttachmentOperationResponse>> {
        #[derive(Serialize)]
        struct BatchDeleteRequest {
            attachment_ids: Vec<String>,
        }

        let request = BatchDeleteRequest { attachment_ids };

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_ATTACHMENTS_BATCH_DELETE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件统计信息
    ///
    /// 该接口用于获取附件相关的统计数据，包括
    /// 附件总数、存储容量、文件类型分布等信息。
    ///
    /// # 参数
    ///
    /// - `object_type`: 关联对象类型（可选）
    /// - `start_date`: 统计开始日期（可选）
    /// - `end_date`: 统计结束日期（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let object_type = Some("talent".to_string());
    /// let start_date = Some("2024-01-01".to_string());
    /// let end_date = Some("2024-01-31".to_string());
    ///
    /// let response = client.hire.attachment.get_attachment_statistics(
    ///     object_type,
    ///     start_date,
    ///     end_date,
    ///     None
    /// ).await?;
    /// ```
    pub async fn get_attachment_statistics(
        &self,
        object_type: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<serde_json::Value>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_ATTACHMENT_STATISTICS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(object_type) = object_type {
            api_req.query_params.insert("object_type", object_type);
        }

        if let Some(start_date) = start_date {
            api_req.query_params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            api_req.query_params.insert("end_date", end_date);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
