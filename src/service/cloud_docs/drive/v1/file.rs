use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
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
            api_path: Endpoints::DRIVE_V1_METAS_BATCH_QUERY.to_string(),
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
            api_path: Endpoints::DRIVE_V1_FILE_STATISTICS
                .replace("{}", &request.file_token),
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
            api_path: Endpoints::DRIVE_V1_FILE_VIEW_RECORDS
                .replace("{}", &request.file_token),
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
            api_path: Endpoints::DRIVE_V1_FILES.to_string(),
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
            api_path: Endpoints::DRIVE_V1_FILE_COPY.replace("{}", &request.file_token),
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
            api_path: Endpoints::DRIVE_V1_FILE_GET.replace("{}", &request.file_token),
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
            api_path: Endpoints::DRIVE_V1_FILES_CREATE_SHORTCUT.to_string(),
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
            api_path: Endpoints::DRIVE_V1_FILES_SEARCH.to_string(),
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
            api_path: Endpoints::DRIVE_V1_FILES_UPLOAD_PREPARE.to_string(),
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
        api_req.api_path = "/open-apis/drive/v1/files/upload_part".to_string();
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
            api_path: Endpoints::DRIVE_V1_FILES_UPLOAD_FINISH.to_string(),
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
            api_path: Endpoints::DRIVE_V1_IMPORT_TASKS.to_string(),
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
            api_path: Endpoints::DRIVE_V1_IMPORT_TASK_GET.replace("{}", &request.ticket),
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileMetaRespData {
    /// 文件元数据列表
    pub metas: Vec<FileMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileViewRecordsRespData {
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页token
    pub page_token: Option<String>,
    /// 访问记录列表
    pub items: Vec<FileViewRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
