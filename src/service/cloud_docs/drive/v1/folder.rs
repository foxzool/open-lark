use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 文件夹服务
pub struct FolderService {
    config: Config,
}

impl FolderService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取我的空间（root folder）元数据
    ///
    /// 该接口用于根据用户的访问凭证获取用户的根目录信息，包括根目录的token等。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta>
    pub async fn get_root_folder_meta(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetRootFolderMetaRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/drive/v1/folders/root_folder_meta".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文件夹中的文件清单
    ///
    /// 该接口用于根据文件夹的token获取文件夹中的文件清单。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list>
    pub async fn list_files(
        &self,
        request: ListFilesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFilesRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/drive/v1/folders/{}/children",
            request.folder_token
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(order_by) = request.order_by {
            api_req
                .query_params
                .insert("order_by".to_string(), order_by);
        }
        if let Some(direction) = request.direction {
            api_req
                .query_params
                .insert("direction".to_string(), direction);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文件夹元数据
    ///
    /// 该接口用于根据文件夹的token获取文件夹的详细元数据信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-folder-meta>
    pub async fn get_folder_meta(
        &self,
        request: GetFolderMetaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFolderMetaRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/drive/v1/folders/{}", request.folder_token);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 新建文件夹
    ///
    /// 该接口用于根据父文件夹的token在其中创建一个新的空文件夹。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder>
    pub async fn create_folder(
        &self,
        request: CreateFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFolderRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/drive/v1/folders".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = serde_json::to_vec(&request)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 移动或删除文件夹
    ///
    /// 该接口用于根据文件夹的token移动或删除文件夹。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/move-delete-folder>
    pub async fn move_or_delete_folder(
        &self,
        request: MoveOrDeleteFolderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MoveOrDeleteFolderRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::POST;
        api_req.api_path = format!("/open-apis/drive/v1/folders/{}/move", request.folder_token);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        // 构建请求体，只包含需要的字段
        let body = serde_json::json!({
            "type": request.operation_type,
            "parent_token": request.parent_token
        });
        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询异步任务状态
    ///
    /// 该接口用于查询异步任务的执行状态，如移动或删除文件夹等操作。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/async-task/task_check>
    pub async fn check_async_task(
        &self,
        request: CheckAsyncTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CheckAsyncTaskRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/drive/v1/tasks/{}", request.task_id);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取我的空间（root folder）元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaRespData {
    /// 用户空间的根目录 token
    pub token: String,
    /// 用户 ID
    pub user_id: String,
}

impl ApiResponseTrait for GetRootFolderMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹中的文件清单请求参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListFilesRequest {
    /// 文件夹的token
    pub folder_token: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    pub page_token: Option<String>,
    /// 分页大小，最大200
    pub page_size: Option<i32>,
    /// 排序字段，支持：创建时间(created_time)、修改时间(edited_time)、文件类型(file_type)、大小(size)
    pub order_by: Option<String>,
    /// 排序方向，支持：升序(ASC)、降序(DESC)
    pub direction: Option<String>,
}

impl ListFilesRequest {
    pub fn new(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            ..Default::default()
        }
    }

    pub fn builder() -> ListFilesRequestBuilder {
        ListFilesRequestBuilder::default()
    }
}

/// 获取文件夹中的文件清单请求构建器
#[derive(Debug, Clone, Default)]
pub struct ListFilesRequestBuilder {
    request: ListFilesRequest,
}

impl ListFilesRequestBuilder {
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = folder_token.into();
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.request.order_by = Some(order_by.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.request.direction = Some(direction.into());
        self
    }

    pub fn build(self) -> ListFilesRequest {
        self.request
    }

    /// 直接执行列出文件夹中的文件请求
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.list_files()`
    pub async fn execute(
        self,
        service: &FolderService,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListFilesRespData>> {
        service.list_files(self.build(), None).await
    }

    /// 直接执行列出文件夹中的文件请求（带选项）
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.list_files()`
    pub async fn execute_with_options(
        self,
        service: &FolderService,
        option: crate::core::req_option::RequestOption,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListFilesRespData>> {
        service.list_files(self.build(), Some(option)).await
    }
}

/// 获取文件夹中的文件清单响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesRespData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 文件清单
    pub files: Vec<DriveFile>,
}

impl ApiResponseTrait for ListFilesRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 驱动文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveFile {
    /// 文件的token
    pub token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    #[serde(rename = "type")]
    pub file_type: String,
    /// 父文件夹token
    pub parent_token: Option<String>,
    /// 文件链接
    pub url: Option<String>,
    /// 文件短链接
    pub short_url: Option<String>,
    /// 文件大小（字节）
    pub size: Option<i64>,
    /// 文件mime类型
    pub mime_type: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 修改时间
    pub modified_time: Option<String>,
    /// 拥有者id
    pub owner_id: Option<String>,
}

/// 获取文件夹元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaRequest {
    /// 文件夹的token
    pub folder_token: String,
}

impl GetFolderMetaRequest {
    pub fn new(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
        }
    }
}

/// 获取文件夹元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaRespData {
    /// 文件夹token
    pub token: String,
    /// 文件夹ID
    pub id: String,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹token
    pub parent_token: Option<String>,
    /// 拥有者ID
    pub owner_id: String,
    /// 创建者ID
    pub creator_id: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub edit_time: String,
    /// 文件夹描述
    pub description: Option<String>,
    /// 文件夹链接
    pub url: String,
}

impl ApiResponseTrait for GetFolderMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 文件夹名称
    pub name: String,
    /// 父文件夹token
    pub parent_token: String,
}

impl CreateFolderRequest {
    pub fn new(name: impl Into<String>, parent_token: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent_token: parent_token.into(),
        }
    }
}

/// 新建文件夹响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRespData {
    /// 新创建文件夹的token
    pub token: String,
    /// 新创建文件夹的链接
    pub url: String,
}

impl ApiResponseTrait for CreateFolderRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动或删除文件夹请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOrDeleteFolderRequest {
    /// 文件夹token
    pub folder_token: String,
    /// 操作类型，move: 移动，delete: 删除
    #[serde(rename = "type")]
    pub operation_type: String,
    /// 移动的目标父文件夹token（删除操作时可以为空）
    pub parent_token: Option<String>,
}

impl MoveOrDeleteFolderRequest {
    /// 创建移动文件夹的请求
    pub fn move_folder(folder_token: impl Into<String>, parent_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            operation_type: "move".to_string(),
            parent_token: Some(parent_token.into()),
        }
    }

    /// 创建删除文件夹的请求
    pub fn delete_folder(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            operation_type: "delete".to_string(),
            parent_token: None,
        }
    }
}

/// 移动或删除文件夹响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOrDeleteFolderRespData {
    /// 异步任务ID，可以通过该ID查询任务执行状态
    pub task_id: Option<String>,
}

impl ApiResponseTrait for MoveOrDeleteFolderRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询异步任务状态请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAsyncTaskRequest {
    /// 任务ID
    pub task_id: String,
}

impl CheckAsyncTaskRequest {
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
        }
    }
}

/// 查询异步任务状态响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAsyncTaskRespData {
    /// 任务状态，PENDING: 等待中，SUCCESS: 成功，FAILURE: 失败
    pub status: String,
    /// 任务错误信息（如果失败）
    pub error_msg: Option<String>,
}

impl ApiResponseTrait for CheckAsyncTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
