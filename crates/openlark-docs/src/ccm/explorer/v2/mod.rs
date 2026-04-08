/// CCM Drive Explorer V2 API 模块
///
/// 云盘浏览器API实现，包含8个API：
/// - root_folder_meta: 获取我的空间（根文件夹）元数据
/// - folder_meta: 获取文件夹元数据
/// - file: 新建文件
/// - file_copy: 复制文档
/// - file_docs: 删除Doc
/// - file_spreadsheets: 删除Sheet
/// - folder_children: 获取文件夹下的文档清单
/// - folder: 新建文件夹
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::Deserialize;

use crate::common::{
    api_endpoints::{CcmDriveExplorerApiOld, DriveApi},
    api_utils::*,
};

#[derive(Debug, Clone, Deserialize)]
struct DriveListFilesData {
    files: Vec<DriveListFileItem>,
    next_page_token: Option<String>,
    has_more: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct DriveListFileItem {
    token: String,
    name: String,
    r#type: String,
    created_time: Option<String>,
    modified_time: Option<String>,
}

impl ApiResponseTrait for DriveListFilesData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 导出模型定义
pub mod models;

impl ApiResponseTrait for FolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone)]
pub struct GetRootFolderMetaRequest {
    config: Config,
}

impl GetRootFolderMetaRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<FolderMetaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FolderMetaResponse> {
        let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取根文件夹元信息")
    }
}

#[derive(Debug, Clone)]
pub struct GetFolderMetaRequest {
    config: Config,
    folder_token: String,
}

impl GetFolderMetaRequest {
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<FolderMetaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FolderMetaResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(self.folder_token);
        let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文件夹元信息")
    }
}

#[derive(Debug, Clone)]
pub struct CreateFileRequest {
    config: Config,
    folder_token: String,
    params: CreateFileParams,
}

impl CreateFileRequest {
    pub fn new(config: Config, folder_token: impl Into<String>, params: CreateFileParams) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    pub async fn execute(self) -> SDKResult<CreateFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");
        validate_required!(self.params.title.trim(), "文件标题不能为空");
        validate_required!(self.params.parent_type.trim(), "文件类型不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::File(self.folder_token);
        let api_request: ApiRequest<CreateFileResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.params, "新建文件")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件")
    }
}

#[derive(Debug, Clone)]
pub struct CopyFileRequest {
    config: Config,
    file_token: String,
    params: CopyFileParams,
}

impl CopyFileRequest {
    pub fn new(config: Config, file_token: impl Into<String>, params: CopyFileParams) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            params,
        }
    }

    pub async fn execute(self) -> SDKResult<CopyFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CopyFileResponse> {
        validate_required!(self.file_token.trim(), "文件Token不能为空");
        validate_required!(self.params.folder_token.trim(), "目标文件夹Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileCopy(self.file_token);
        let api_request: ApiRequest<CopyFileResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.params, "复制文档")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "复制文档")
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocRequest {
    config: Config,
    doc_token: String,
}

impl DeleteDocRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteFileResponse> {
        validate_required!(self.doc_token.trim(), "文档Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileDocs(self.doc_token);
        let api_request: ApiRequest<DeleteFileResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除Doc")
    }
}

#[derive(Debug, Clone)]
pub struct DeleteSheetRequest {
    config: Config,
    spreadsheet_token: String,
}

impl DeleteSheetRequest {
    pub fn new(config: Config, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            config,
            spreadsheet_token: spreadsheet_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteFileResponse> {
        validate_required!(self.spreadsheet_token.trim(), "表格Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileSpreadsheets(self.spreadsheet_token);
        let api_request: ApiRequest<DeleteFileResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除Sheet")
    }
}

#[derive(Debug, Clone)]
pub struct GetFolderChildrenRequest {
    config: Config,
    folder_token: String,
    params: Option<GetFolderChildrenParams>,
}

impl GetFolderChildrenRequest {
    pub fn new(
        config: Config,
        folder_token: impl Into<String>,
        params: Option<GetFolderChildrenParams>,
    ) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    pub async fn execute(self) -> SDKResult<GetFolderChildrenResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFolderChildrenResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");

        let mut api_request: ApiRequest<DriveListFilesData> =
            ApiRequest::get(&DriveApi::ListFiles.to_url()).query("folder_token", self.folder_token);

        if let Some(params) = self.params {
            api_request =
                api_request.query_opt("page_size", params.page_size.map(|v| v.to_string()));
            api_request = api_request.query_opt("page_token", params.page_token);
            if let Some(doc_type) = params.doc_type {
                api_request = api_request.query("type", doc_type);
            }
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        let data: DriveListFilesData = extract_response_data(response, "获取文件夹子项")?;

        let items = data
            .files
            .into_iter()
            .map(|item| {
                let doc_type = item.r#type;
                let is_folder = doc_type == "folder";
                FileItem {
                    file_token: item.token,
                    title: item.name,
                    doc_type,
                    is_folder,
                    create_time: item
                        .created_time
                        .as_deref()
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or(0),
                    update_time: item
                        .modified_time
                        .as_deref()
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

        Ok(GetFolderChildrenResponse {
            data: Some(FolderChildrenData {
                items,
                has_more: data.has_more,
                page_token: data.next_page_token,
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateFolderRequest {
    config: Config,
    folder_token: String,
    params: CreateFolderParams,
}

impl CreateFolderRequest {
    pub fn new(
        config: Config,
        folder_token: impl Into<String>,
        params: CreateFolderParams,
    ) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    pub async fn execute(self) -> SDKResult<CreateFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateFolderResponse> {
        validate_required!(self.folder_token.trim(), "父文件夹Token不能为空");
        validate_required!(self.params.title.trim(), "文件夹标题不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::Folder(self.folder_token);
        let api_request: ApiRequest<CreateFolderResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self.params, "新建文件夹")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件夹")
    }
}

/// 获取我的空间（根文件夹）元数据
///
/// 获取 "我的空间" 的元信息。
/// docPath: /document/server-docs/docs/drive-v1/folder/get-root-folder-meta
pub async fn get_root_folder_meta(config: &Config) -> SDKResult<FolderMetaResponse> {
    GetRootFolderMetaRequest::new(config.clone())
        .execute()
        .await
}

/// 获取文件夹元数据
///
/// 根据 folderToken 获取该文件夹的元信息。
/// docPath: /document/server-docs/docs/drive-v1/folder/get-folder-meta
pub async fn get_folder_meta(config: &Config, folder_token: &str) -> SDKResult<FolderMetaResponse> {
    GetFolderMetaRequest::new(config.clone(), folder_token)
        .execute()
        .await
}

/// 新建文件
///
/// 根据 folderToken 创建 Doc、Sheet 或 Bitable。
/// docPath: /document/server-docs/docs/drive-v1/file/create-online-document
pub async fn create_file(
    config: &Config,
    folder_token: &str,
    params: CreateFileParams,
) -> SDKResult<CreateFileResponse> {
    CreateFileRequest::new(config.clone(), folder_token, params)
        .execute()
        .await
}

/// 复制文档
///
/// 根据文件 token 复制 Doc 或 Sheet 到目标文件夹中。
/// docPath: /document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet
pub async fn copy_file(
    config: &Config,
    file_token: &str,
    params: CopyFileParams,
) -> SDKResult<CopyFileResponse> {
    CopyFileRequest::new(config.clone(), file_token, params)
        .execute()
        .await
}

/// 删除Doc
///
/// 根据 docToken 删除对应的 Docs 文档。
/// docPath: /document/server-docs/historic-version/docs/drive/file/delete-a-doc
pub async fn delete_doc(config: &Config, doc_token: &str) -> SDKResult<DeleteFileResponse> {
    DeleteDocRequest::new(config.clone(), doc_token)
        .execute()
        .await
}

/// 删除Sheet
///
/// 根据 spreadsheetToken 删除对应的 sheet 文档。
/// docPath: /document/server-docs/historic-version/docs/drive/file/delete-sheet
pub async fn delete_sheet(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<DeleteFileResponse> {
    DeleteSheetRequest::new(config.clone(), spreadsheet_token)
        .execute()
        .await
}

/// 获取文件夹下的文档清单
///
/// 根据 folderToken 获取该文件夹的文档清单，如 doc、sheet、file、bitable、folder。
/// docPath: /document/server-docs/historic-version/docs/drive/folder/get-folder-children
pub async fn get_folder_children(
    config: &Config,
    folder_token: &str,
    params: Option<GetFolderChildrenParams>,
) -> SDKResult<GetFolderChildrenResponse> {
    GetFolderChildrenRequest::new(config.clone(), folder_token, params)
        .execute()
        .await
}

/// 新建文件夹
///
/// 根据 folderToken 在该 folder 下创建文件夹。
/// docPath: /document/server-docs/historic-version/docs/drive/folder/create-a-new-folder
pub async fn create_folder(
    config: &Config,
    folder_token: &str,
    params: CreateFolderParams,
) -> SDKResult<CreateFolderResponse> {
    CreateFolderRequest::new(config.clone(), folder_token, params)
        .execute()
        .await
}

// API函数已经在模块中定义，不需要重复导出

// 重新导出模型
// models 模块显式导出
pub use models::{
    CopyFileParams, CopyFileResponse, CopyResult, CreateFileParams, CreateFileResponse,
    CreateFolderParams, CreateFolderResponse, DeleteFileResponse, DeleteResult, FileInfo, FileItem,
    FolderChildrenData, FolderMeta, FolderMetaResponse, GetFolderChildrenParams,
    GetFolderChildrenResponse, NewFolderInfo, UserInfo,
};
