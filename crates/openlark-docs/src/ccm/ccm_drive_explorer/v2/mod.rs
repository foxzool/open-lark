//! CCM Drive Explorer V2 API 模块
//!
//! 云盘浏览器API实现，包含8个API：
//! - root_folder_meta: 获取我的空间（根文件夹）元数据
//! - folder_meta: 获取文件夹元数据
//! - file: 新建文件
//! - file_copy: 复制文档
//! - file_docs: 删除Doc
//! - file_spreadsheets: 删除Sheet
//! - folder_children: 获取文件夹下的文档清单
//! - folder: 新建文件夹

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

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

/// 获取我的空间（根文件夹）元数据
///
/// 获取 "我的空间" 的元信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
pub async fn get_root_folder_meta(
    config: &Config,
) -> SDKResult<FolderMetaResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;

    // 创建API请求
    let api_request: ApiRequest<FolderMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取根文件夹元信息")
}

/// 获取文件夹元数据
///
/// 根据 folderToken 获取该文件夹的元信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-folder-meta
pub async fn get_folder_meta(
    config: &Config,
    folder_token: &str,
) -> SDKResult<FolderMetaResponse> {
    // 验证必填字段
    validate_required_field("文件夹Token", Some(folder_token), "文件夹Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(folder_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<FolderMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取文件夹元信息")
}

/// 新建文件
///
/// 根据 folderToken 创建 Doc、Sheet 或 Bitable。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-online-document
pub async fn create_file(
    config: &Config,
    folder_token: &str,
    params: CreateFileParams,
) -> SDKResult<CreateFileResponse> {
    // 验证必填字段
    validate_required_field("文件夹Token", Some(folder_token), "文件夹Token不能为空")?;
    validate_required_field("文件标题", Some(&params.title), "文件标题不能为空")?;
    validate_required_field("文件类型", Some(&params.parent_type), "文件类型不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::File(folder_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<CreateFileResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "新建文件")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "新建文件")
}

/// 复制文档
///
/// 根据文件 token 复制 Doc 或 Sheet 到目标文件夹中。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet
pub async fn copy_file(
    config: &Config,
    file_token: &str,
    params: CopyFileParams,
) -> SDKResult<CopyFileResponse> {
    // 验证必填字段
    validate_required_field("文件Token", Some(file_token), "文件Token不能为空")?;
    validate_required_field("目标文件夹Token", Some(&params.folder_token), "目标文件夹Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FileCopy(file_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<CopyFileResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "复制文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "复制文档")
}

/// 删除Doc
///
/// 根据 docToken 删除对应的 Docs 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc
pub async fn delete_doc(
    config: &Config,
    doc_token: &str,
) -> SDKResult<DeleteFileResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(doc_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FileDocs(doc_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteFileResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除Doc")
}

/// 删除Sheet
///
/// 根据 spreadsheetToken 删除对应的 sheet 文档。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-sheet
pub async fn delete_sheet(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<DeleteFileResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FileSpreadsheets(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteFileResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除Sheet")
}

/// 获取文件夹下的文档清单
///
/// 根据 folderToken 获取该文件夹的文档清单，如 doc、sheet、file、bitable、folder。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children
pub async fn get_folder_children(
    config: &Config,
    folder_token: &str,
    params: Option<GetFolderChildrenParams>,
) -> SDKResult<GetFolderChildrenResponse> {
    // 验证必填字段
    validate_required_field("文件夹Token", Some(folder_token), "文件夹Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FolderChildren(folder_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<GetFolderChildrenResponse> = if let Some(params) = params {
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "获取文件夹子项")?)
    } else {
        ApiRequest::get(&api_endpoint.to_url())
    };

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取文件夹子项")
}

/// 新建文件夹
///
/// 根据 folderToken 在该 folder 下创建文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder
pub async fn create_folder(
    config: &Config,
    folder_token: &str,
    params: CreateFolderParams,
) -> SDKResult<CreateFolderResponse> {
    // 验证必填字段
    validate_required_field("父文件夹Token", Some(folder_token), "父文件夹Token不能为空")?;
    validate_required_field("文件夹标题", Some(&params.title), "文件夹标题不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::Folder(folder_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<CreateFolderResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "新建文件夹")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "新建文件夹")
}

// API函数已经在模块中定义，不需要重复导出

// 重新导出模型
pub use models::*;