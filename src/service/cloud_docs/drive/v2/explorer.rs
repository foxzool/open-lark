use std::fmt::Debug;

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
        SDKResult,
    },
    impl_executable_builder_owned,
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
    pub async fn root_folder_meta(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ExplorerRootMeta>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_EXPLORER_V2_ROOT_FOLDER_META.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// GET /open-apis/drive/explorer/v2/folder/:folderToken/meta
    ///
    /// 获取文件夹的元信息
    pub async fn folder_meta(
        &self,
        folder_token: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ExplorerFolderMeta>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_EXPLORER_V2_FOLDER_META.replace("{folder_token}", folder_token),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// POST /open-apis/drive/v1/files/create_folder
    /// 新建文件夹
    pub async fn create_folder(
        &self,
        create_folder_request: CreateFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFolderResponse>> {
        let mut api_req = create_folder_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DRIVE_V1_FILES_CREATE_FOLDER.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取文件夹下的清单
    ///
    /// <https://open.feishu.cn/open-apis/drive/v1/files>
    pub async fn list_folder(
        &self,
        list_folder_request: ListFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFolderResponse>> {
        let mut api_req = list_folder_request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = DRIVE_V1_FILES.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    pub fn list_folder_iter(
        &self,
        req: ListFolderRequest,
        option: Option<RequestOption>,
    ) -> ListFolderIterator<'_> {
        ListFolderIterator {
            explorer_service: self,
            req,
            option,
            has_more: true,
        }
    }
}

pub struct ListFolderIterator<'a> {
    explorer_service: &'a ExplorerService,
    req: ListFolderRequest,
    option: Option<RequestOption>,
    has_more: bool,
}

impl ListFolderIterator<'_> {
    pub async fn next(&mut self) -> Option<Vec<FileInFolder>> {
        if !self.has_more {
            return None;
        }

        match self
            .explorer_service
            .list_folder(self.req.clone(), self.option.clone())
            .await
        {
            Ok(resp) => match resp.data {
                Some(data) => {
                    self.has_more = data.has_more;
                    if data.has_more {
                        self.req
                            .api_req
                            .query_params
                            .insert("page_token", data.next_page_token.unwrap());
                        Some(data.files)
                    } else if data.files.is_empty() {
                        None
                    } else {
                        Some(data.files)
                    }
                }
                None => None,
            },
            Err(e) => {
                error!("Error: {e:?}");
                None
            }
        }
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
    /// 文件夹名称
    ///
    /// 示例值："New Folder"
    name: String,
    /// 父文件夹token。如果需要创建到「我的空间」作为顶级文件夹，请传入我的空间token
    folder_token: String,
}

impl CreateFolderRequest {
    pub fn builder() -> CreateFolderRequestBuilder {
        CreateFolderRequestBuilder::default()
    }
}

#[derive(Default)]
/// 创建文件夹请求体
pub struct CreateFolderRequestBuilder {
    request: CreateFolderRequest,
}

impl CreateFolderRequestBuilder {
    /// 文件夹名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 父文件夹token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.request.folder_token = folder_token.to_string();
        self
    }

    pub fn build(mut self) -> CreateFolderRequest {
        self.request.api_req.body = serde_json::to_vec(&self.request).unwrap();

        self.request
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出文件夹请求体
#[derive(Default)]
pub struct ListFolderRequestBuilder {
    request: ListFolderRequest,
}

impl ListFolderRequestBuilder {
    /// 页大小
    ///
    /// 示例值：50
    ///
    /// 数据校验规则：
    ///
    /// 最大值：200
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_size", page_size.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// 示例值："MTY1NTA3MTA1OXw3MTA4NDc2MDc1NzkyOTI0Nabcef"
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token", page_token.to_string());
        self
    }

    /// 文件夹的token（若不填写该参数或填写空字符串，则默认获取用户云空间下的清单，且不支持分页）
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a9abcef"
    pub fn folder_token(mut self, fold_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("folder_token", fold_token.to_string());
        self
    }

    /// 排序规则
    ///
    /// 示例值："EditedTime"
    ///
    /// 可选值有：
    ///
    /// - EditedTime：编辑时间排序
    /// - CreatedTime：创建时间排序
    ///
    /// 默认值：EditedTime
    pub fn order_by(mut self, order_by: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("order_by", order_by.to_string());
        self
    }

    /// 升序降序
    ///
    /// 默认值：DESC
    ///
    /// 可选值有：
    ///
    /// - ASC：升序
    /// - DESC：降序
    pub fn direction(mut self, direction: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("direction", direction.to_string());
        self
    }

    /// 用户 ID 类型
    ///
    /// 默认值：open_id
    ///
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("user_id_type", user_id_type.to_string());
        self
    }

    pub fn build(self) -> ListFolderRequest {
        self.request
    }
}

/// 列出文件夹查询参数
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ListFolderRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
}

impl ListFolderRequest {
    pub fn builder() -> ListFolderRequestBuilder {
        ListFolderRequestBuilder::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFolderResponse {
    /// 文件夹列表
    pub files: Vec<FileInFolder>,
    /// 分页标记，当 has_more 为 true 时，会同时返回下一次遍历的page_token，否则则不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

/// 文件夹清单列表
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInFolder {
    /// 文件标识
    pub token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    ///
    /// 可选值有：
    ///
    /// - doc：旧版文档
    ///
    /// - sheet：表格
    ///
    /// - mindnote：思维导图
    //
    /// - bitable：多维表格
    ///
    /// - file：文件
    ///
    /// - docx：新版文档
    ///
    /// - folder：文件夹
    ///
    /// - shortcut: 快捷方式
    pub r#type: String,
    /// 父文件夹标识
    pub parent_token: String,
    /// 在浏览器中查看的链接
    pub url: String,
    /// 快捷方式文件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<ShortcutInfo>,
    /// 文件创建时间
    pub created_time: String,
    /// 文件最近修改时间
    pub modified_time: String,
    /// 文件所有者
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortcutInfo {
    /// 快捷方式指向的原文件类型
    ///
    /// 可选值有：
    ///
    /// - doc：旧版文档
    ///
    /// - sheet：表格
    ///
    /// - mindnote：思维导图
    ///
    /// - bitable：多维表格
    ///
    /// - file：文件
    ///
    /// - docx：新版文档
    pub target_type: String,
    /// 快捷方式指向的原文件token
    pub target_token: String,
}

impl ApiResponseTrait for ListFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl_executable_builder_owned!(
    CreateFolderRequestBuilder,
    ExplorerService,
    CreateFolderRequest,
    BaseResponse<CreateFolderResponse>,
    create_folder
);

impl_executable_builder_owned!(
    ListFolderRequestBuilder,
    ExplorerService,
    ListFolderRequest,
    BaseResponse<ListFolderResponse>,
    list_folder
);
