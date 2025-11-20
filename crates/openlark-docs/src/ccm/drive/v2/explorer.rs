
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use std::fmt::Debug;
use openlark_core::api::ApiRequest;
use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
pub struct ExplorerService {
    config: Config}
impl ExplorerService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// GET /open-apis/drive/explorer/v2/root_folder/meta,
    ///,
/// 获取云空间的根目录,
    pub async fn root_folder_meta(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExplorerRootMeta>> {,
let api_req = ApiRequest {,
            http_http_method: Method::GET,
            url: DRIVE_EXPLORER_V2_ROOT_FOLDER_META.to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// GET /open-apis/drive/explorer/v2/folder/:folderToken/meta,
    ///,
/// 获取文件夹的元信息,
    pub async fn folder_meta(
        &self,
        folder_token: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExplorerFolderMeta>> {,
let api_req = ApiRequest {,
            http_http_method: Method::GET,
            url: DRIVE_EXPLORER_V2_FOLDER_META.replace("{folder_token}", folder_token),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            
};

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// POST /open-apis/drive/v1/files/create_folder,
    /// 新建文件夹,
pub async fn create_folder(,
        &self,
        create_folder_request: CreateFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateFolderResponse>> {,
let mut api_req = create_folder_request.api_req;
        api_req.set_http_method(Method::POST);
api_req.set_api_path(DRIVE_V1_FILES_CREATE_FOLDER.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp)}
/// 获取文件夹下的清单,
    ///,
/// <https://open.feishu.cn/open-apis/drive/v1/files>,
    pub async fn list_folder(
        &self,
        list_folder_request: ListFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ListFolderResponse>> {,
let mut api_req = list_folder_request.api_req;
        api_req.set_http_method(Method::GET);
api_req.set_api_path(DRIVE_V1_FILES.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp)}
pub fn list_folder_iter(,
        &self,
        req: ListFolderRequest,
        option: Option<RequestOption>,
    ) -> ListFolderIterator<'_> {,
ListFolderIterator {,
            explorer_service: self,
            req,
            option,
            has_more: true}
pub struct ListFolderIterator<'a> {,
    explorer_service: &'a ExplorerService,
    req: ListFolderRequest,
    option: Option<RequestOption>,
    has_more: bool}
impl ListFolderIterator<'_> {,
    /// # API文档,
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM,
pub async fn next(&mut self) -> Option<Vec<FileInFolder>> {,
        if !self.has_more {,
return None;
        }
match self,
            .explorer_service
            .list_folder(self.req.clone() self.option.clone()),
.await,
        {,
Ok(resp) => match resp.data {,
                Some(data) => {,
self.has_more = data.has_more;
                    if data.has_more {,
self.req,
                            .api_req
.query_params
                            .insert("page_token", data.next_page_token.unwrap());
Some(data.files)} else if data.files.is_empty() {,
None} else {,
Some(data.files)}
                None => None,
            }
            Err(e) => {
                error!("Error: {e:?}");
None,
}
/// 我的空间（root folder）元信息,
#[derive(Clone, Debug)]
pub struct ExplorerRootMeta {
    /// 文件夹的 token
    pub token: String,
    /// 文件夹的 id
    pub id: String,
    /// 文件夹的所有者 id
    pub user_id: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 文件夹元信息,
#[derive(Clone, Debug)]
pub struct ExplorerFolderMeta {
    /// 文件夹的 id
    pub id: String,
    /// 文件夹的标题
    pub name: String,
    /// 文件夹的 token
    pub token: String,
    /// 文件夹的创建者 id,
#[serde(rename = "createUid")]
    pub create_uid: String,
    /// 文件夹的最后编辑者 id,
#[serde(rename = "editUid")]
    pub edit_uid: String,
    /// 文件夹的上级目录 id,
#[serde(rename = "parentId")]
    pub parent_id: String,
    /// 文件夹为个人文件夹时，为文件夹的所有者 id；文件夹为共享文件夹时，为文件夹树id,
#[serde(rename = "ownUid")]
    pub own_uid: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }

#[derive(Clone, Debug)]
pub struct CreateFolderRequest {
    /// 请求体,
#[serde(skip)]
    api_req: ApiRequest,
    /// 文件夹名称,
///,
    /// 示例值："New Folder"
    name: String,
    /// 父文件夹token。如果需要创建到「我的空间」作为顶级文件夹，请传入我的空间token
    folder_token: String}
impl CreateFolderRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
/// 创建文件夹请求体,
pub struct CreateFolderRequestBuilder {
    request: CreateFolderRequest}
impl CreateFolderRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建文件夹响应体,
#[derive(Clone, Debug)]
pub struct CreateFolderResponse {
    /// 创建文件夹的token
    pub token: String,
    /// 创建文件夹的访问url
    pub url: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 列出文件夹请求体,
#[derive(Default)]
pub struct ListFolderRequestBuilder {
    request: ListFolderRequest}
impl ListFolderRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 列出文件夹查询参数,
#[derive(Clone, Debug)]
pub struct ListFolderRequest {
    /// 请求体,
#[serde(skip)]
    api_req: ApiRequest}
impl ListFolderRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}
#[derive(Clone, Debug)]
pub struct ListFolderResponse {
    /// 文件夹列表
    pub files: Vec<FileInFolder>,
    /// 分页标记，当 has_more 为 true 时，会同时返回下一次遍历的page_token，否则则不返回,
#[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
/// 文件夹清单列表,
#[derive(Clone, Debug)]
pub struct FileInFolder {
    /// 文件标识
    pub token: String,
    /// 文件名
    pub name: String,
    /// 文件类型,
///,
    /// 可选值有：,
///,
    /// - doc：旧版文档,
///,
    /// - sheet：表格,
///,
    /// - mindnote：思维导图,
//,
    /// - bitable：多维表格,
///,
    /// - file：文件,
///,
    /// - docx：新版文档,
///,
    /// - folder：文件夹,
///,
    /// - shortcut: 快捷方式
    pub r#type: String,
    /// 父文件夹标识
    pub parent_token: String,
    /// 在浏览器中查看的链接
    pub url: String,
    /// 快捷方式文件信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<ShortcutInfo>,
    /// 文件创建时间
    pub created_time: String,
    /// 文件最近修改时间
    pub modified_time: String,
    /// 文件所有者
    pub owner_id: String,

#[derive(Clone, Debug)]
pub struct ShortcutInfo {
    /// 快捷方式指向的原文件类型,
///,
    /// 可选值有：,
///,
    /// - doc：旧版文档,
///,
    /// - sheet：表格,
///,
    /// - mindnote：思维导图,
///,
    /// - bitable：多维表格,
///,
    /// - file：文件,
///,
    /// - docx：新版文档
    pub target_type: String,
    /// 快捷方式指向的原文件token
    pub target_token: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl_executable_builder_owned!(,
    CreateFolderRequestBuilder,
    ExplorerService,
    CreateFolderRequest,
    Response<CreateFolderResponse>,
    create_folder,
);
impl_executable_builder_owned!(
    ListFolderRequestBuilder,
    ExplorerService,
    ListFolderRequest,
    Response<ListFolderResponse>,
    list_folder,
);
