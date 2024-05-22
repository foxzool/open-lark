use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiReq,
    api_resp::{ApiResponse, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    SDKResult,
};

pub struct ExplorerService {
    config: Config,
}

impl ExplorerService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// GET /open-apis/drive/explorer/v2/root_folder/meta
    ///
    /// 获取云空间的根目录
    pub fn root_folder_meta(&self) -> SDKResult<ApiResponse<ExplorerRootMeta>> {
        let mut api_req = ApiReq::default();
        api_req.http_method = "GET".to_string();
        api_req.api_path = "/open-apis/drive/explorer/v2/root_folder/meta".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, None)?;

        Ok(api_resp)
    }

    /// GET /open-apis/drive/explorer/v2/folder/:folderToken/meta
    ///
    /// 获取文件夹的元信息
    pub fn folder_meta(&self, folder_token: &str) -> SDKResult<ApiResponse<ExplorerFolderMeta>> {
        let mut api_req = ApiReq::default();
        api_req.http_method = "GET".to_string();
        api_req.api_path = format!("/open-apis/drive/explorer/v2/folder/{folder_token}/meta");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, None)?;

        Ok(api_resp)
    }

    /// POST /open-apis/drive/v1/files/create_folder
    /// 新建文件夹
    pub fn create_folder(&self, mut api_req: ApiReq) -> SDKResult<ApiResponse<CreateFolderResponse>> {
        api_req.http_method = "POST".to_string();
        api_req.api_path = "/open-apis/drive/v1/files/create_folder".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, None)?;

        Ok(api_resp)
    }
}

/// 我的空间（root folder）元信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerRootMeta {
    /// 文件夹的 token
    pub token: String,
    /// 文件夹的 id
    pub id: String,
    /// 文件夹的所有者 id
    pub user_id: String,
}

impl ApiResponseTrait for ExplorerRootMeta {
    fn standard_data_format() -> bool {
        true
    }
}

/// 文件夹元信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerFolderMeta {
    /// 文件夹的 id
    pub id: String,
    /// 文件夹的标题
    pub name: String,
    /// 文件夹的 token
    pub token: String,
    /// 文件夹的创建者 id
    #[serde(rename = "createUid")]
    pub create_uid: String,
    /// 文件夹的最后编辑者 id
    #[serde(rename = "editUid")]
    pub edit_uid: String,
    /// 文件夹的上级目录 id
    #[serde(rename = "parentId")]
    pub parent_id: String,
    /// 文件夹为个人文件夹时，为文件夹的所有者 id；文件夹为共享文件夹时，为文件夹树id
    #[serde(rename = "ownUid")]
    pub own_uid: String,
}

impl ApiResponseTrait for ExplorerFolderMeta {
    fn standard_data_format() -> bool {
        true
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateFolderReqBody {
    /// 文件夹名称
    ///
    /// 示例值："New Folder"
    name: String,
    /// 父文件夹token。如果需要创建到「我的空间」作为顶级文件夹，请传入我的空间token
    folder_token: String,
}

#[derive(Default)]
/// 创建文件夹请求体
pub struct CreateFolderReq {
    req_body: CreateFolderReqBody,
    api_req: ApiReq,
}

impl CreateFolderReq {
    pub fn new() -> Self {
        Self::default()
    }

    /// 文件夹名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.req_body.name = name.to_string();
        self
    }

    /// 父文件夹token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.req_body.folder_token = folder_token.to_string();
        self
    }

    pub fn build(mut self) -> ApiReq {
        self.api_req.body = serde_json::to_vec(&self.req_body).unwrap().into();

        self.api_req
    }
}

/// 创建文件夹响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 创建文件夹的token
    pub token: String,
    /// 创建文件夹的访问url
    pub url: String,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn standard_data_format() -> bool {
        true
    }
}
