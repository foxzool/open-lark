use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        trait_system::Service,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 文件服务 - 处理除上传下载外的其他文件操作
pub struct FileService {
    config: Config,
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文件元数据
    ///
    /// 该接口用于根据文件token获取文件的元数据信息。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/meta/batch_query>
    pub async fn get_file_meta(
        &self,
        request: GetFileMetaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetFileMetaRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_METAS_BATCH_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<GetFileMetaRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取文件统计信息
    ///
    /// 该接口用于根据文件token获取文件的统计信息，如浏览次数等。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-statistics/get>
    pub async fn get_file_statistics(
        &self,
        request: GetFileStatisticsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetFileStatisticsRespData> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FILE_STATISTICS.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp: BaseResponse<GetFileStatisticsRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取文件访问记录
    ///
    /// 该接口用于获取文件的访问记录列表。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-view_record/list>
    pub async fn list_file_view_records(
        &self,
        request: ListFileViewRecordsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListFileViewRecordsRespData> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FILE_VIEW_RECORDS.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        let api_resp: BaseResponse<ListFileViewRecordsRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 新建文件
    ///
    /// 该接口用于在指定文件夹中新建文件。
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN>
    pub async fn create_file(
        &self,
        request: CreateFileRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateFileRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<CreateFileRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 复制文件
    ///
    /// 该接口用于复制文件到指定文件夹。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy>
    pub async fn copy_file(
        &self,
        request: CopyFileRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CopyFileRespData> {
        // 构建请求体
        let body = serde_json::json!({
            "name": request.name,
            "type": request.copy_type,
            "parent_token": request.parent_token
        });

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILE_COPY.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&body)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<CopyFileRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 删除文件
    ///
    /// 该接口用于删除文件。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete>
    pub async fn delete_file(
        &self,
        request: DeleteFileRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<DeleteFileRespData> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: DRIVE_V1_FILE_GET.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp: BaseResponse<DeleteFileRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 创建文件快捷方式
    ///
    /// 该接口用于创建文件的快捷方式。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_shortcut>
    pub async fn create_file_shortcut(
        &self,
        request: CreateFileShortcutRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateFileShortcutRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES_CREATE_SHORTCUT.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<CreateFileShortcutRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 搜索文件
    ///
    /// 该接口用于搜索文件。
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN>
    pub async fn search_files(
        &self,
        request: SearchFilesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchFilesRespData> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FILES_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        // 添加查询参数
        api_req
            .query_params
            .insert("search_key", request.search_key);
        if let Some(count) = request.count {
            api_req.query_params.insert("count", count.to_string());
        }
        if let Some(offset) = request.offset {
            api_req.query_params.insert("offset", offset.to_string());
        }
        if let Some(owner_ids) = request.owner_ids {
            api_req
                .query_params
                .insert("owner_ids", owner_ids.join(","));
        }

        let api_resp: BaseResponse<SearchFilesRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 分片上传文件-预上传
    ///
    /// 该接口用于分片上传的预上传步骤，获取上传事务ID和分片信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare>
    pub async fn upload_prepare(
        &self,
        request: FileUploadPrepareRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<FileUploadPrepareRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES_UPLOAD_PREPARE.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<FileUploadPrepareRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 分片上传文件-上传分片
    ///
    /// 该接口用于上传文件分片。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_part>
    pub async fn upload_part(
        &self,
        request: FileUploadPartRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<FileUploadPartRespData> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DRIVE_V1_FILES_UPLOAD_PART.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp: BaseResponse<FileUploadPartRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 分片上传文件-完成上传
    ///
    /// 该接口用于完成分片上传。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_finish>
    pub async fn upload_finish(
        &self,
        request: FileUploadFinishRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<FileUploadFinishRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES_UPLOAD_FINISH.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<FileUploadFinishRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 创建导入任务
    ///
    /// 该接口用于创建文档导入任务。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create>
    pub async fn create_import_task(
        &self,
        request: CreateImportTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateImportTaskRespData> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_IMPORT_TASKS.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<CreateImportTaskRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 查询导入任务结果
    ///
    /// 该接口用于查询导入任务的执行结果。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get>
    pub async fn get_import_task(
        &self,
        request: GetImportTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetImportTaskRespData> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_IMPORT_TASK_GET.replace("{}", &request.ticket),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp: BaseResponse<GetImportTaskRespData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

// === 请求和响应数据结构 ===

/// 获取文件元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileMetaRequest {
    /// 文件token列表
    pub request_docs: Vec<RequestDoc>,
    /// 是否获取额外信息
    pub with_url: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoc {
    /// 文件token
    pub doc_token: String,
    /// 文件类型
    pub doc_type: String,
}

impl GetFileMetaRequest {
    pub fn new(docs: Vec<(String, String)>) -> Self {
        Self {
            request_docs: docs
                .into_iter()
                .map(|(token, doc_type)| RequestDoc {
                    doc_token: token,
                    doc_type,
                })
                .collect(),
            with_url: Some(true),
        }
    }
}

/// 获取文件元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFileMetaRespData {
    /// 文件元数据列表
    pub metas: Vec<FileMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMeta {
    /// 文件token
    pub doc_token: String,
    /// 文件类型
    pub doc_type: String,
    /// 文件标题
    pub title: String,
    /// 拥有者ID
    pub owner_id: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 文件URL
    pub url: Option<String>,
}

impl ApiResponseTrait for GetFileMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件统计信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileStatisticsRequest {
    /// 文件token
    pub file_token: String,
}

impl GetFileStatisticsRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 获取文件统计信息响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFileStatisticsRespData {
    /// 文件浏览次数
    pub uv: i64,
    /// 文件浏览人数
    pub pv: i64,
    /// 文件点赞数
    pub like_count: i64,
    /// 文件评论数
    pub comment_count: i64,
}

impl ApiResponseTrait for GetFileStatisticsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件访问记录请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileViewRecordsRequest {
    /// 文件token
    pub file_token: String,
    /// 分页token
    pub page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
}

impl ListFileViewRecordsRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            page_token: None,
            page_size: None,
        }
    }

    pub fn with_page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 获取文件访问记录响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFileViewRecordsRespData {
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页token
    pub page_token: Option<String>,
    /// 访问记录列表
    pub items: Vec<FileViewRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileViewRecord {
    /// 访问者ID
    pub viewer_id: String,
    /// 访问者名称
    pub viewer_name: String,
    /// 访问时间
    pub view_time: String,
    /// 访问设备
    pub view_device: String,
}

impl ApiResponseTrait for ListFileViewRecordsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileRequest {
    /// 文件名称
    pub title: String,
    /// 文件类型
    #[serde(rename = "type")]
    pub file_type: String,
    /// 父文件夹token
    pub parent_token: String,
}

impl CreateFileRequest {
    pub fn new(
        title: impl Into<String>,
        file_type: impl Into<String>,
        parent_token: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            file_type: file_type.into(),
            parent_token: parent_token.into(),
        }
    }
}

/// 新建文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFileRespData {
    /// 新建文件的token
    pub token: String,
    /// 新建文件的链接
    pub url: String,
}

impl ApiResponseTrait for CreateFileRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileRequest {
    /// 文件token
    pub file_token: String,
    /// 新文件名称
    pub name: String,
    /// 复制类型
    #[serde(rename = "type")]
    pub copy_type: String,
    /// 目标父文件夹token
    pub parent_token: String,
}

impl CopyFileRequest {
    pub fn new(
        file_token: impl Into<String>,
        name: impl Into<String>,
        parent_token: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            name: name.into(),
            copy_type: "copy".to_string(),
            parent_token: parent_token.into(),
        }
    }
}

/// 复制文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileRespData {
    /// 复制后文件的token
    pub token: String,
    /// 复制后文件的链接
    pub url: String,
}

impl ApiResponseTrait for CopyFileRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileRequest {
    /// 文件token
    pub file_token: String,
}

impl DeleteFileRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 删除文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileRespData {
    /// 异步任务ID
    pub task_id: Option<String>,
}

impl ApiResponseTrait for DeleteFileRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文件快捷方式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutRequest {
    /// 原文件token
    pub refer_entity: ReferEntity,
    /// 快捷方式名称
    pub name: String,
    /// 父文件夹token
    pub parent_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferEntity {
    /// 原文件类型
    #[serde(rename = "type")]
    pub entity_type: String,
    /// 原文件token
    pub token: String,
}

impl CreateFileShortcutRequest {
    pub fn new(
        file_type: impl Into<String>,
        file_token: impl Into<String>,
        name: impl Into<String>,
        parent_token: impl Into<String>,
    ) -> Self {
        Self {
            refer_entity: ReferEntity {
                entity_type: file_type.into(),
                token: file_token.into(),
            },
            name: name.into(),
            parent_token: parent_token.into(),
        }
    }
}

/// 创建文件快捷方式响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileShortcutRespData {
    /// 快捷方式token
    pub token: String,
    /// 快捷方式链接
    pub url: String,
}

impl ApiResponseTrait for CreateFileShortcutRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesRequest {
    /// 搜索关键词
    pub search_key: String,
    /// 返回数量
    pub count: Option<i32>,
    /// 偏移量
    pub offset: Option<i32>,
    /// 所有者ID列表
    pub owner_ids: Option<Vec<String>>,
}

impl SearchFilesRequest {
    pub fn new(search_key: impl Into<String>) -> Self {
        Self {
            search_key: search_key.into(),
            count: None,
            offset: None,
            owner_ids: None,
        }
    }

    pub fn with_count(mut self, count: i32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_owner_ids(mut self, owner_ids: Vec<String>) -> Self {
        self.owner_ids = Some(owner_ids);
        self
    }
}

/// 搜索文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesRespData {
    /// 搜索结果文件列表
    pub files: Vec<SearchFileItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFileItem {
    /// 文件token
    pub token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    #[serde(rename = "type")]
    pub file_type: String,
    /// 文件链接
    pub url: String,
    /// 拥有者ID
    pub owner_id: String,
}

impl ApiResponseTrait for SearchFilesRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传文件-预上传请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadPrepareRequest {
    /// 文件名称
    pub file_name: String,
    /// 父文件夹token
    pub parent_token: String,
    /// 文件大小
    pub size: i64,
    /// 分片大小（可选）
    pub block_size: Option<i32>,
    /// 文件校验和（可选）
    pub checksum: Option<String>,
}

impl FileUploadPrepareRequest {
    pub fn new(file_name: impl Into<String>, parent_token: impl Into<String>, size: i64) -> Self {
        Self {
            file_name: file_name.into(),
            parent_token: parent_token.into(),
            size,
            block_size: None,
            checksum: None,
        }
    }

    pub fn with_block_size(mut self, block_size: i32) -> Self {
        self.block_size = Some(block_size);
        self
    }

    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }
}

/// 分片上传文件-预上传响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadPrepareRespData {
    /// 上传事务ID
    pub upload_id: String,
    /// 分片大小
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for FileUploadPrepareRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传文件-上传分片请求参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileUploadPartRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 上传事务ID
    upload_id: String,
    /// 分片序号
    seq: i32,
    /// 分片大小
    size: i32,
    /// 分片校验和（可选）
    checksum: Option<String>,
}

impl FileUploadPartRequest {
    pub fn builder() -> FileUploadPartRequestBuilder {
        FileUploadPartRequestBuilder::default()
    }
}

/// 分片上传文件-上传分片请求构建器
#[derive(Default)]
pub struct FileUploadPartRequestBuilder {
    request: FileUploadPartRequest,
}

impl FileUploadPartRequestBuilder {
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.request.upload_id = upload_id.into();
        self
    }

    pub fn seq(mut self, seq: i32) -> Self {
        self.request.seq = seq;
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.request.size = size;
        self
    }

    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.request.checksum = Some(checksum.into());
        self
    }

    pub fn file_chunk(mut self, chunk: Vec<u8>) -> Self {
        self.request.api_req.file = chunk;
        self
    }

    pub fn build(mut self) -> FileUploadPartRequest {
        match serde_json::to_vec(&self.request) {
            Ok(bytes) => {
                self.request.api_req.body = bytes;
            }
            Err(e) => {
                error!("Failed to serialize file upload part request: {}", e);
                self.request.api_req.body = Vec::new();
            }
        }
        self.request
    }
}

impl_executable_builder_owned!(
    FileUploadPartRequestBuilder,
    FileService,
    FileUploadPartRequest,
    FileUploadPartRespData,
    upload_part
);

/// 分片上传文件-上传分片响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadPartRespData {
    /// 分片ETag
    pub etag: String,
}

impl ApiResponseTrait for FileUploadPartRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传文件-完成上传请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadFinishRequest {
    /// 上传事务ID
    pub upload_id: String,
    /// 分片信息列表
    pub block_infos: Vec<FileBlockInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBlockInfo {
    /// 分片ETag
    pub etag: String,
    /// 分片序号
    pub seq: i32,
}

impl FileUploadFinishRequest {
    pub fn new(upload_id: impl Into<String>, block_infos: Vec<FileBlockInfo>) -> Self {
        Self {
            upload_id: upload_id.into(),
            block_infos,
        }
    }
}

/// 分片上传文件-完成上传响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadFinishRespData {
    /// 文件token
    pub file_token: String,
}

impl ApiResponseTrait for FileUploadFinishRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建导入任务请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    /// 导入文件的token
    pub file_extension: String,
    /// 导入文件类型
    pub file_token: String,
    /// 导入文件类型
    #[serde(rename = "type")]
    pub import_type: String,
    /// 导入的目标文件夹token
    pub parent_token: String,
    /// 导入的文件名称
    pub file_name: String,
    /// 导入点类型
    pub parent_type: String,
}

impl CreateImportTaskRequest {
    pub fn new(
        file_extension: impl Into<String>,
        file_token: impl Into<String>,
        import_type: impl Into<String>,
        parent_token: impl Into<String>,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
    ) -> Self {
        Self {
            file_extension: file_extension.into(),
            file_token: file_token.into(),
            import_type: import_type.into(),
            parent_token: parent_token.into(),
            file_name: file_name.into(),
            parent_type: parent_type.into(),
        }
    }
}

/// 创建导入任务响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRespData {
    /// 导入任务ID
    pub ticket: String,
}

impl ApiResponseTrait for CreateImportTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询导入任务结果请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskRequest {
    /// 导入任务ID
    pub ticket: String,
}

impl GetImportTaskRequest {
    pub fn new(ticket: impl Into<String>) -> Self {
        Self {
            ticket: ticket.into(),
        }
    }
}

/// 查询导入任务结果响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskRespData {
    /// 任务结果
    pub result: ImportTaskResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskResult {
    /// 任务类型
    #[serde(rename = "type")]
    pub task_type: String,
    /// 任务ID
    pub ticket: String,
    /// 任务状态
    pub job_status: i32,
    /// 任务错误码
    pub job_error_msg: Option<String>,
    /// 导入结果
    pub token: Option<String>,
    /// 导入结果类型
    pub url: Option<String>,
}

impl ApiResponseTrait for GetImportTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Service for FileService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "file"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_resp::ResponseFormat;
    use rstest::rstest;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_service() -> FileService {
        FileService::new(create_test_config())
    }

    #[test]
    fn test_file_service_new() {
        let config = create_test_config();
        let service = FileService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(FileService::service_name(), "file");
        assert_eq!(FileService::service_version(), "v1");
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();
        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(FileService::service_name(), "file");
        assert_eq!(FileService::service_version(), "v1");
    }

    // === Request/Response Data Structure Tests ===

    #[test]
    fn test_get_file_meta_request() {
        let docs = vec![
            ("file_token_1".to_string(), "doc".to_string()),
            ("file_token_2".to_string(), "sheet".to_string()),
        ];
        let request = GetFileMetaRequest::new(docs.clone());

        assert_eq!(request.request_docs.len(), 2);
        assert_eq!(request.request_docs[0].doc_token, "file_token_1");
        assert_eq!(request.request_docs[0].doc_type, "doc");
        assert_eq!(request.request_docs[1].doc_token, "file_token_2");
        assert_eq!(request.request_docs[1].doc_type, "sheet");
        assert_eq!(request.with_url, Some(true));
    }

    #[test]
    fn test_get_file_meta_request_serialization() {
        let docs = vec![("test_token".to_string(), "doc".to_string())];
        let request = GetFileMetaRequest::new(docs);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("request_docs"));
        assert!(json.contains("with_url"));
        assert!(json.contains("test_token"));
        assert!(json.contains("doc"));

        let deserialized: GetFileMetaRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.request_docs.len(), 1);
        assert_eq!(deserialized.request_docs[0].doc_token, "test_token");
    }

    #[test]
    fn test_get_file_meta_resp_data_api_response_trait() {
        assert_eq!(GetFileMetaRespData::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_file_statistics_request() {
        let request = GetFileStatisticsRequest::new("test_file_token");
        assert_eq!(request.file_token, "test_file_token");

        let request2 = GetFileStatisticsRequest::new("another_token".to_string());
        assert_eq!(request2.file_token, "another_token");
    }

    #[test]
    fn test_get_file_statistics_resp_data() {
        let resp_data = GetFileStatisticsRespData {
            uv: 100,
            pv: 250,
            like_count: 15,
            comment_count: 8,
        };

        assert_eq!(resp_data.uv, 100);
        assert_eq!(resp_data.pv, 250);
        assert_eq!(resp_data.like_count, 15);
        assert_eq!(resp_data.comment_count, 8);
        assert_eq!(
            GetFileStatisticsRespData::data_format(),
            ResponseFormat::Data
        );

        // Test serialization
        let json = serde_json::to_string(&resp_data).unwrap();
        let deserialized: GetFileStatisticsRespData = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.uv, resp_data.uv);
        assert_eq!(deserialized.pv, resp_data.pv);
    }

    #[test]
    fn test_list_file_view_records_request_builder() {
        let request = ListFileViewRecordsRequest::new("test_token")
            .with_page_token("next_page")
            .with_page_size(20);

        assert_eq!(request.file_token, "test_token");
        assert_eq!(request.page_token, Some("next_page".to_string()));
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_list_file_view_records_request_minimal() {
        let request = ListFileViewRecordsRequest::new("minimal_token");
        assert_eq!(request.file_token, "minimal_token");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
    }

    #[test]
    fn test_list_file_view_records_resp_data() {
        let records = vec![
            FileViewRecord {
                viewer_id: "user1".to_string(),
                viewer_name: "John Doe".to_string(),
                view_time: "2023-12-01T10:00:00Z".to_string(),
                view_device: "web".to_string(),
            },
            FileViewRecord {
                viewer_id: "user2".to_string(),
                viewer_name: "Jane Smith".to_string(),
                view_time: "2023-12-01T11:00:00Z".to_string(),
                view_device: "mobile".to_string(),
            },
        ];

        let resp_data = ListFileViewRecordsRespData {
            has_more: true,
            page_token: Some("next_token".to_string()),
            items: records.clone(),
        };

        assert!(resp_data.has_more);
        assert_eq!(resp_data.page_token, Some("next_token".to_string()));
        assert_eq!(resp_data.items.len(), 2);
        assert_eq!(resp_data.items[0].viewer_name, "John Doe");
        assert_eq!(resp_data.items[1].view_device, "mobile");
        assert_eq!(
            ListFileViewRecordsRespData::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_create_file_request() {
        let request = CreateFileRequest::new("My Document", "doc", "parent_folder_token");

        assert_eq!(request.title, "My Document");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.parent_token, "parent_folder_token");
    }

    #[test]
    fn test_create_file_request_serialization() {
        let request = CreateFileRequest::new("Test File", "sheet", "folder123");
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("Test File"));
        assert!(json.contains("\"type\":\"sheet\""));
        assert!(json.contains("parent_token"));

        let deserialized: CreateFileRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "Test File");
        assert_eq!(deserialized.file_type, "sheet");
    }

    #[test]
    fn test_copy_file_request() {
        let request = CopyFileRequest::new("source_token", "Copy of Document", "target_folder");

        assert_eq!(request.file_token, "source_token");
        assert_eq!(request.name, "Copy of Document");
        assert_eq!(request.copy_type, "copy");
        assert_eq!(request.parent_token, "target_folder");
    }

    #[test]
    fn test_delete_file_request() {
        let request = DeleteFileRequest::new("file_to_delete");
        assert_eq!(request.file_token, "file_to_delete");

        let request2 = DeleteFileRequest::new("another_file".to_string());
        assert_eq!(request2.file_token, "another_file");
    }

    #[test]
    fn test_create_file_shortcut_request() {
        let request = CreateFileShortcutRequest::new(
            "doc",
            "original_file_token",
            "Shortcut to Document",
            "shortcut_folder",
        );

        assert_eq!(request.refer_entity.entity_type, "doc");
        assert_eq!(request.refer_entity.token, "original_file_token");
        assert_eq!(request.name, "Shortcut to Document");
        assert_eq!(request.parent_token, "shortcut_folder");
    }

    #[test]
    fn test_search_files_request_builder() {
        let request = SearchFilesRequest::new("important documents")
            .with_count(50)
            .with_offset(100)
            .with_owner_ids(vec!["user1".to_string(), "user2".to_string()]);

        assert_eq!(request.search_key, "important documents");
        assert_eq!(request.count, Some(50));
        assert_eq!(request.offset, Some(100));
        assert_eq!(
            request.owner_ids,
            Some(vec!["user1".to_string(), "user2".to_string()])
        );
    }

    #[test]
    fn test_search_files_request_minimal() {
        let request = SearchFilesRequest::new("test");
        assert_eq!(request.search_key, "test");
        assert_eq!(request.count, None);
        assert_eq!(request.offset, None);
        assert_eq!(request.owner_ids, None);
    }

    #[test]
    fn test_file_upload_prepare_request() {
        let request = FileUploadPrepareRequest::new("document.pdf", "upload_folder", 1024000)
            .with_block_size(4096)
            .with_checksum("sha256:abcdef123456");

        assert_eq!(request.file_name, "document.pdf");
        assert_eq!(request.parent_token, "upload_folder");
        assert_eq!(request.size, 1024000);
        assert_eq!(request.block_size, Some(4096));
        assert_eq!(request.checksum, Some("sha256:abcdef123456".to_string()));
    }

    #[test]
    fn test_file_upload_prepare_request_minimal() {
        let request = FileUploadPrepareRequest::new("simple.txt", "folder", 500);
        assert_eq!(request.file_name, "simple.txt");
        assert_eq!(request.size, 500);
        assert_eq!(request.block_size, None);
        assert_eq!(request.checksum, None);
    }

    #[test]
    fn test_file_upload_part_request_builder() {
        let test_chunk = vec![1, 2, 3, 4, 5];
        let request = FileUploadPartRequest::builder()
            .upload_id("upload_123")
            .seq(1)
            .size(5)
            .checksum("chunk_checksum")
            .file_chunk(test_chunk.clone())
            .build();

        assert_eq!(request.upload_id, "upload_123");
        assert_eq!(request.seq, 1);
        assert_eq!(request.size, 5);
        assert_eq!(request.checksum, Some("chunk_checksum".to_string()));
        assert_eq!(request.api_req.file, test_chunk);
    }

    #[test]
    fn test_file_upload_part_request_builder_minimal() {
        let request = FileUploadPartRequest::builder()
            .upload_id("minimal_upload")
            .seq(0)
            .size(100)
            .build();

        assert_eq!(request.upload_id, "minimal_upload");
        assert_eq!(request.seq, 0);
        assert_eq!(request.size, 100);
        assert_eq!(request.checksum, None);
    }

    #[test]
    fn test_file_upload_finish_request() {
        let block_infos = vec![
            FileBlockInfo {
                etag: "etag1".to_string(),
                seq: 1,
            },
            FileBlockInfo {
                etag: "etag2".to_string(),
                seq: 2,
            },
        ];
        let request = FileUploadFinishRequest::new("upload_123", block_infos.clone());

        assert_eq!(request.upload_id, "upload_123");
        assert_eq!(request.block_infos.len(), 2);
        assert_eq!(request.block_infos[0].etag, "etag1");
        assert_eq!(request.block_infos[1].seq, 2);
    }

    #[test]
    fn test_create_import_task_request() {
        let request = CreateImportTaskRequest::new(
            "pdf",
            "source_file_token",
            "import_type_doc",
            "target_folder",
            "imported_document.docx",
            "folder",
        );

        assert_eq!(request.file_extension, "pdf");
        assert_eq!(request.file_token, "source_file_token");
        assert_eq!(request.import_type, "import_type_doc");
        assert_eq!(request.parent_token, "target_folder");
        assert_eq!(request.file_name, "imported_document.docx");
        assert_eq!(request.parent_type, "folder");
    }

    #[test]
    fn test_get_import_task_request() {
        let request = GetImportTaskRequest::new("task_ticket_123");
        assert_eq!(request.ticket, "task_ticket_123");

        let request2 = GetImportTaskRequest::new("another_ticket".to_string());
        assert_eq!(request2.ticket, "another_ticket");
    }

    // === Serialization/Deserialization Tests ===

    #[rstest]
    #[case(GetFileMetaRespData { metas: vec![] })]
    #[case(GetFileStatisticsRespData { uv: 0, pv: 0, like_count: 0, comment_count: 0 })]
    #[case(ListFileViewRecordsRespData { has_more: false, page_token: None, items: vec![] })]
    #[case(CreateFileRespData { token: "test".to_string(), url: "http://test.com".to_string() })]
    fn test_response_data_serialization<T>(#[case] data: T)
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: T = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_file_meta_serialization() {
        let file_meta = FileMeta {
            doc_token: "test_token".to_string(),
            doc_type: "doc".to_string(),
            title: "Test Document".to_string(),
            owner_id: "owner123".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-02T00:00:00Z".to_string(),
            url: Some("https://example.com/doc".to_string()),
        };

        let json = serde_json::to_string(&file_meta).unwrap();
        let deserialized: FileMeta = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.doc_token, file_meta.doc_token);
        assert_eq!(deserialized.title, file_meta.title);
        assert_eq!(deserialized.url, file_meta.url);
    }

    #[test]
    fn test_search_file_item_serialization() {
        let item = SearchFileItem {
            token: "file_token".to_string(),
            name: "Important File".to_string(),
            file_type: "doc".to_string(),
            url: "https://example.com/file".to_string(),
            owner_id: "user123".to_string(),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"type\":\"doc\""));

        let deserialized: SearchFileItem = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.file_type, "doc");
        assert_eq!(deserialized.name, "Important File");
    }

    #[test]
    fn test_import_task_result_serialization() {
        let result = ImportTaskResult {
            task_type: "import".to_string(),
            ticket: "task_123".to_string(),
            job_status: 1,
            job_error_msg: Some("Error occurred".to_string()),
            token: Some("result_token".to_string()),
            url: Some("https://result.com".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"type\":\"import\""));

        let deserialized: ImportTaskResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.task_type, "import");
        assert_eq!(deserialized.job_status, 1);
        assert_eq!(
            deserialized.job_error_msg,
            Some("Error occurred".to_string())
        );
    }

    // === Edge Cases and Error Handling Tests ===

    #[test]
    fn test_empty_file_meta_request() {
        let request = GetFileMetaRequest::new(vec![]);
        assert_eq!(request.request_docs.len(), 0);
        assert_eq!(request.with_url, Some(true));
    }

    #[test]
    fn test_large_file_upload_prepare() {
        let large_size = i64::MAX;
        let request = FileUploadPrepareRequest::new("huge_file.dat", "folder", large_size);
        assert_eq!(request.size, large_size);
    }

    #[test]
    fn test_file_upload_part_zero_size() {
        let request = FileUploadPartRequest::builder()
            .upload_id("test")
            .seq(0)
            .size(0)
            .build();
        assert_eq!(request.size, 0);
    }

    #[test]
    fn test_search_files_empty_search_key() {
        let request = SearchFilesRequest::new("");
        assert_eq!(request.search_key, "");
    }

    #[test]
    fn test_search_files_negative_values() {
        let request = SearchFilesRequest::new("test")
            .with_count(-1)
            .with_offset(-10);
        assert_eq!(request.count, Some(-1));
        assert_eq!(request.offset, Some(-10));
    }

    #[test]
    fn test_list_file_view_records_empty_response() {
        let resp_data = ListFileViewRecordsRespData {
            has_more: false,
            page_token: None,
            items: vec![],
        };
        assert!(!resp_data.has_more);
        assert_eq!(resp_data.items.len(), 0);
    }

    #[test]
    fn test_file_upload_finish_empty_blocks() {
        let request = FileUploadFinishRequest::new("upload_id", vec![]);
        assert_eq!(request.block_infos.len(), 0);
    }

    // === API Response Trait Tests ===

    #[rstest]
    #[case(GetFileMetaRespData::data_format(), ResponseFormat::Data)]
    #[case(GetFileStatisticsRespData::data_format(), ResponseFormat::Data)]
    #[case(ListFileViewRecordsRespData::data_format(), ResponseFormat::Data)]
    #[case(CreateFileRespData::data_format(), ResponseFormat::Data)]
    #[case(CopyFileRespData::data_format(), ResponseFormat::Data)]
    #[case(DeleteFileRespData::data_format(), ResponseFormat::Data)]
    #[case(CreateFileShortcutRespData::data_format(), ResponseFormat::Data)]
    #[case(SearchFilesRespData::data_format(), ResponseFormat::Data)]
    #[case(FileUploadPrepareRespData::data_format(), ResponseFormat::Data)]
    #[case(FileUploadPartRespData::data_format(), ResponseFormat::Data)]
    #[case(FileUploadFinishRespData::data_format(), ResponseFormat::Data)]
    #[case(CreateImportTaskRespData::data_format(), ResponseFormat::Data)]
    #[case(GetImportTaskRespData::data_format(), ResponseFormat::Data)]
    fn test_api_response_trait_format(
        #[case] actual: ResponseFormat,
        #[case] expected: ResponseFormat,
    ) {
        assert_eq!(actual, expected);
    }

    // === Builder Pattern Tests ===

    #[test]
    fn test_list_file_view_records_builder_chain() {
        let request = ListFileViewRecordsRequest::new("token")
            .with_page_token("page1")
            .with_page_size(25)
            .with_page_token("page2"); // Override previous page_token

        assert_eq!(request.page_token, Some("page2".to_string()));
        assert_eq!(request.page_size, Some(25));
    }

    #[test]
    fn test_search_files_builder_chain() {
        let owners = vec![
            "user1".to_string(),
            "user2".to_string(),
            "user3".to_string(),
        ];
        let request = SearchFilesRequest::new("documents")
            .with_count(100)
            .with_offset(50)
            .with_owner_ids(owners.clone())
            .with_count(200); // Override previous count

        assert_eq!(request.count, Some(200));
        assert_eq!(request.offset, Some(50));
        assert_eq!(request.owner_ids, Some(owners));
    }

    #[test]
    fn test_file_upload_prepare_builder_chain() {
        let request = FileUploadPrepareRequest::new("file.dat", "folder", 1000)
            .with_block_size(512)
            .with_checksum("checksum1")
            .with_block_size(1024) // Override
            .with_checksum("checksum2"); // Override

        assert_eq!(request.block_size, Some(1024));
        assert_eq!(request.checksum, Some("checksum2".to_string()));
    }

    // === Unicode and Special Character Tests ===

    #[test]
    fn test_unicode_file_names() {
        let request = CreateFileRequest::new("文档测试🚀", "doc", "folder");
        assert_eq!(request.title, "文档测试🚀");

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateFileRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "文档测试🚀");
    }

    #[test]
    fn test_special_characters_in_search() {
        let request = SearchFilesRequest::new("file@#$%^&*()[]{}");
        assert_eq!(request.search_key, "file@#$%^&*()[]{}");
    }

    #[test]
    fn test_long_file_names() {
        let long_name = "a".repeat(1000);
        let request = CreateFileRequest::new(&long_name, "doc", "folder");
        assert_eq!(request.title.len(), 1000);
    }

    // === Default and Clone Tests ===

    #[test]
    fn test_file_upload_part_request_default() {
        let request = FileUploadPartRequest::default();
        assert_eq!(request.upload_id, "");
        assert_eq!(request.seq, 0);
        assert_eq!(request.size, 0);
        assert_eq!(request.checksum, None);
    }

    #[test]
    fn test_request_cloning() {
        let original = CreateFileRequest::new("Original", "doc", "folder");
        let cloned = original.clone();

        assert_eq!(original.title, cloned.title);
        assert_eq!(original.file_type, cloned.file_type);
        assert_eq!(original.parent_token, cloned.parent_token);
    }

    // === Error Serialization Tests ===

    #[test]
    fn test_file_upload_part_builder_serialization_error() {
        // Create a request that might cause serialization issues
        let request = FileUploadPartRequest::builder()
            .upload_id("test")
            .seq(1)
            .size(0)
            .build();

        // Even with potential serialization error, the request should be built
        assert_eq!(request.upload_id, "test");
        assert_eq!(request.seq, 1);
    }

    #[test]
    fn test_api_response_trait_consistency() {
        // Ensure all response types consistently use Data format
        let formats = vec![
            GetFileMetaRespData::data_format(),
            GetFileStatisticsRespData::data_format(),
            CreateFileRespData::data_format(),
            DeleteFileRespData::data_format(),
        ];

        for format in formats {
            assert_eq!(format, ResponseFormat::Data);
        }
    }
}
