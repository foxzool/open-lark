
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;//! 云盘文件夹服务 v1
//!
//! 提供企业级文件夹管理功能，支持：
//! - 文件夹创建、查询、更新、删除
//! - 文件夹内容管理和权限控制
//! - 文件夹元数据和分享功能
//! - 完整的Builder模式API设计

use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    core::{
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 文件夹服务
///
/// 提供企业级文件夹管理功能，支持文件夹的完整生命周期管理。
/// 专为企业文档管理设计，具备完善的错误处理和性能优化。
#[derive(Clone, Debug)]
pub struct FolderService {
    pub config: Config,
}

impl FolderService {
    /// 创建文件夹服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息，包含应用ID、密钥等
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::drive::v1::folder::FolderService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = FolderService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取我的空间（root folder）元数据
    ///
    /// 该接口用于根据用户的访问凭证获取用户的根目录信息，包括根目录的token等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/get-root-folder-meta
    ///
    /// # 参数
    ///
    /// * `option` - 可选的请求配置，如超时时间、重试次数等
    ///
    /// # 返回值
    ///
    /// 返回根目录的元数据信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .get_root_folder_meta(None)
    ///     .await?;
    ///
    /// println!("根目录token: {}", response.data.folder_token);
    /// ```
    pub async fn get_root_folder_meta(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetRootFolderMetaRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(DRIVE_V1_FOLDERS_ROOT_FOLDER_META.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::User]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文件夹中的文件清单
    ///
    /// 该接口用于根据文件夹的token获取文件夹中的文件清单。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/list-children
    ///
    /// # 参数
    ///
    /// * `request` - 列表文件请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回文件夹中的文件列表
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = ListFilesRequestBuilder::new()
    ///     .folder_token("folder_token")
    ///     .page_size(20)
    ///     .order_by("created_time")
    ///     .build();
    ///
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .list_files(request, None)
    ///     .await?;
    ///
    /// for file in response.data.items {
    ///     println!("文件: {} ({})", file.name, file.file_token);
    /// }
    /// ```
    pub async fn list_files(
        &self,
        request: ListFilesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ListFilesRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(DRIVE_V1_FOLDER_CHILDREN.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_token) = &request.page_token {
            api_req.query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(page_size) = request.page_size {
            api_req.query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(order_by) = &request.order_by {
            api_req.query_params.insert("order_by".to_string(), order_by.clone());
        }
        if let Some(direction) = &request.direction {
            api_req.query_params.insert("direction".to_string(), direction.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文件夹元数据
    ///
    /// 该接口用于根据文件夹的token获取文件夹的详细元数据信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/get-folder
    ///
    /// # 参数
    ///
    /// * `request` - 获取文件夹元数据请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回文件夹的详细元数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = GetFolderMetaRequestBuilder::new()
    ///     .folder_token("folder_token")
    ///     .build();
    ///
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .get_folder_meta(request, None)
    ///     .await?;
    ///
    /// println!("文件夹名称: {}", response.data.name);
    /// println!("创建时间: {}", response_data.create_time);
    /// ```
    pub async fn get_folder_meta(
        &self,
        request: GetFolderMetaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetFolderMetaRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(DRIVE_V1_FOLDER_GET.to_string().replace("{}", &request.folder_token));
        api_req.set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 新建文件夹
    ///
    /// 根据指定的参数创建新的文件夹。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/create
    ///
    /// # 参数
    ///
    /// * `request` - 创建文件夹请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回新创建的文件夹信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = CreateFolderRequestBuilder::new()
    ///     .name("项目文档")
    ///     .parent_folder_token("parent_token")
    ///     .build();
    ///
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .create_folder(request, None)
    ///     .await?;
    ///
    /// println!("新文件夹token: {}", response.data.folder_token);
    /// ```
    pub async fn create_folder(
        &self,
        request: CreateFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CreateFolderRespData>> {
        let api_req = request.api_req;
        Transport::request(api_req, &self.config, option).await
    }

    /// 更新文件夹
    ///
    /// 更新文件夹的名称和其他属性。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/update
    ///
    /// # 参数
    ///
    /// * `request` - 更新文件夹请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回更新后的文件夹信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = UpdateFolderRequestBuilder::new()
    ///     .folder_token("folder_token")
    ///     .name("更新后的文件夹名称")
    ///     .build();
    ///
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .update_folder(request, None)
    ///     .await?;
    ///
    /// println!("更新成功，文件夹token: {}", response.data.folder_token);
    /// ```
    pub async fn update_folder(
        &self,
        request: UpdateFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UpdateFolderRespData>> {
        let api_req = request.api_req;
        Transport::request(api_req, &self.config, option).await
    }

    /// 删除文件夹
    ///
    /// 根据文件夹token删除指定的文件夹及其所有内容。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive-v1/folder/delete
    ///
    /// # 参数
    ///
    /// * `request` - 删除文件夹请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = DeleteFolderRequestBuilder::new()
    ///     .folder_token("folder_token")
    ///     .build();
    ///
    /// let response = client.cloud_docs.v1.drive.folder
    ///     .delete_folder(request, None)
    ///     .await?;
    ///
    /// println!("文件夹删除成功");
    /// ```
    pub async fn delete_folder(
        &self,
        request: DeleteFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DeleteFolderRespData>> {
        let api_req = request.api_req;
        Transport::request(api_req, &self.config, option).await
    }

    // ==================== Builder模式实现 ====================

    /// 创建文件夹构建器
    ///
    /// 提供流式API来构建创建文件夹的请求参数。
    /// 支持链式调用，方便配置各种参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.cloud_docs.v1.drive.folder
    ///     .create_folder_builder()
    ///     .name("项目文档")
    ///     .parent_folder_token("parent_token");
    ///
    /// let response = builder.execute(&client.cloud_docs.v1.drive.folder).await?;
    /// ```
    pub fn create_folder_builder(&self) -> CreateFolderRequestBuilder {
        CreateFolderRequestBuilder::new()
    }

    /// 更新文件夹构建器
    ///
    /// 提供流式API来构建更新文件夹的请求参数。
    /// 支持链式调用，方便配置各种参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.cloud_docs.v1.drive.folder
    ///     .update_folder_builder()
    ///     .folder_token("folder_token")
    ///     .name("更新后的名称");
    ///
    /// let response = builder.execute(&client.cloud_docs.v1.drive.folder).await?;
    /// ```
    pub fn update_folder_builder(&self) -> UpdateFolderRequestBuilder {
        UpdateFolderRequestBuilder::new()
    }

    /// 删除文件夹构建器
    ///
    /// 提供流式API来构建删除文件夹的请求参数。
    /// 支持链式调用，方便配置各种参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.cloud_docs.v1.drive.folder
    ///     .delete_folder_builder()
    ///     .folder_token("folder_token");
    ///
    /// let response = builder.execute(&client.cloud_docs.v1.drive.folder).await?;
    /// ```
    pub fn delete_folder_builder(&self) -> DeleteFolderRequestBuilder {
        DeleteFolderRequestBuilder::new()
    }

    /// 获取文件夹元数据构建器
    ///
    /// 提供流式API来构建获取文件夹元数据的请求参数。
    /// 支持链式调用，方便配置各种参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.cloud_docs.v1.drive.folder
    ///     .get_folder_meta_builder()
    ///     .folder_token("folder_token");
    ///
    /// let response = builder.execute(&client.cloud_docs.v1.drive.folder).await?;
    /// ```
    pub fn get_folder_meta_builder(&self) -> GetFolderMetaRequestBuilder {
        GetFolderMetaRequestBuilder::new()
    }

    /// 列表文件构建器
    ///
    /// 提供流式API来构建列表文件的请求参数。
    /// 支持链式调用，方便配置各种参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.cloud_docs.v1.drive.folder
    ///     .list_files_builder()
    ///     .folder_token("folder_token")
    ///     .page_size(20)
    ///     .order_by("created_time");
    ///
    /// let response = builder.execute(&client.cloud_docs.v1.drive.folder).await?;
    /// ```
    pub fn list_files_builder(&self) -> ListFilesRequestBuilder {
        ListFilesRequestBuilder::new()
    }
}

// ==================== 请求和响应结构体 ====================

/// 获取根文件夹元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaRespData {
    /// 根目录token
    pub folder_token: String,
    /// 根目录名称
    pub name: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 根目录类型
    pub folder_type: String,
    /// 拥有者信息
    pub owner: Option<OwnerInfo>,
}

/// 列表文件请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListFilesRequest {
    /// 文件夹token
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 分页标记
    pub page_token: Option<String>,
    /// 页面大小
    pub page_size: Option<u32>,
    /// 排序字段
    pub order_by: Option<String>,
    /// 排序方向
    pub direction: Option<String>,
}

/// 列表文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesRespData {
    /// 文件项目列表
    pub items: Vec<FileItem>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
}

/// 文件项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 拥有者信息
    pub owner: Option<OwnerInfo>,
    /// URL链接
    pub url: Option<String>,
    /// 缩略图
    pub thumbnail: Option<String>,
}

/// 获取文件夹元数据请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFolderMetaRequest {
    /// 文件夹token
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件夹token
    pub folder_token: String,
}

/// 获取文件夹元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaRespData {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 父级文件夹token
    pub parent_folder_token: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 拥有者信息
    pub owner: Option<OwnerInfo>,
    /// 权限信息
    pub permission: Option<String>,
    /// 分享信息
    pub share_info: Option<ShareInfo>,
}

/// 创建文件夹请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 创建文件夹请求
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件夹名称
    pub name: String,
    /// 父级文件夹token
    pub parent_folder_token: Option<String>,
    /// 文件夹类型
    pub folder_type: Option<String>,
}

/// 创建文件夹响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRespData {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 父级文件夹token
    pub parent_folder_token: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 拥有者信息
    pub owner: Option<OwnerInfo>,
}

/// 更新文件夹请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFolderRequest {
    /// 更新文件夹请求
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: Option<String>,
    /// 文件夹类型
    pub folder_type: Option<String>,
}

/// 更新文件夹响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFolderRespData {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 父级文件夹token
    pub parent_folder_token: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 拥有者信息
    pub owner: Option<OwnerInfo>,
}

/// 删除文件夹请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteFolderRequest {
    /// 删除文件夹请求
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件夹token
    pub folder_token: String,
}

/// 删除文件夹响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFolderRespData {
    /// 操作结果
    pub success: bool,
    /// 消息
    pub message: String,
}

/// 拥有者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}

/// 分享信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareInfo {
    /// 分享类型
    pub share_type: String,
    /// 分享链接
    pub share_url: String,
    /// 分享密码
    pub share_password: Option<String>,
    /// 过期时间
    pub expire_time: Option<String>,
}

// ==================== Builder模式实现 ====================

/// 创建文件夹请求构建器
#[derive(Debug, Clone, Default)]
pub struct CreateFolderRequestBuilder {
    request: CreateFolderRequest,
}

impl CreateFolderRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateFolderRequest::default(),
        }
    }

    /// 设置文件夹名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 设置父文件夹token
    pub fn parent_folder_token(mut self, parent_folder_token: &str) -> Self {
        self.request.parent_folder_token = Some(parent_folder_token.to_string());
        self
    }

    /// 设置文件夹类型
    pub fn folder_type(mut self, folder_type: &str) -> Self {
        self.request.folder_type = Some(folder_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateFolderRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder_owned!(
    CreateFolderRequestBuilder,
    FolderService,
    CreateFolderRequest,
    Response<CreateFolderRespData>,
    create_folder
);

/// 更新文件夹请求构建器
#[derive(Debug, Clone, Default)]
pub struct UpdateFolderRequestBuilder {
    request: UpdateFolderRequest,
}

impl UpdateFolderRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: UpdateFolderRequest::default(),
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: &str) -> Self {
        self.request.folder_token = folder_token.to_string();
        self
    }

    /// 设置文件夹名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 设置文件夹类型
    pub fn folder_type(mut self, folder_type: &str) -> Self {
        self.request.folder_type = Some(folder_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> UpdateFolderRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder_owned!(
    UpdateFolderRequestBuilder,
    FolderService,
    UpdateFolderRequest,
    Response<UpdateFolderRespData>,
    update_folder
);

/// 删除文件夹请求构建器
#[derive(Debug, Clone, Default)]
pub struct DeleteFolderRequestBuilder {
    request: DeleteFolderRequest,
}

impl DeleteFolderRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: DeleteFolderRequest::default(),
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: &str) -> Self {
        self.request.folder_token = folder_token.to_string();
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> DeleteFolderRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder_owned!(
    DeleteFolderRequestBuilder,
    FolderService,
    DeleteFolderRequest,
    Response<DeleteFolderRespData>,
    delete_folder
);

/// 获取文件夹元数据请求构建器
#[derive(Debug, Clone, Default)]
pub struct GetFolderMetaRequestBuilder {
    request: GetFolderMetaRequest,
}

impl GetFolderMetaRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetFolderMetaRequest::default(),
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: &str) -> Self {
        self.request.folder_token = folder_token.to_string();
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetFolderMetaRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder_owned!(
    GetFolderMetaRequestBuilder,
    FolderService,
    GetFolderMetaRequest,
    Response<GetFolderMetaRespData>,
    get_folder_meta
);

/// 列表文件请求构建器
#[derive(Debug, Clone, Default)]
pub struct ListFilesRequestBuilder {
    request: ListFilesRequest,
}

impl ListFilesRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: ListFilesRequest::default(),
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: &str) -> Self {
        self.request.folder_token = Some(folder_token.to_string());
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置排序字段
    pub fn order_by(mut self, order_by: &str) -> Self {
        self.request.order_by = Some(order_by.to_string());
        self
    }

    /// 设置排序方向
    pub fn direction(mut self, direction: &str) -> Self {
        self.request.direction = Some(direction.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> ListFilesRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder_owned!(
    ListFilesRequestBuilder,
    FolderService,
    ListFilesRequest,
    Response<ListFilesRespData>,
    list_files
);

// ==================== ApiResponseTrait实现 ====================

impl ApiResponseTrait for GetRootFolderMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListFilesRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFolderMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateFolderRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateFolderRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFolderRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}