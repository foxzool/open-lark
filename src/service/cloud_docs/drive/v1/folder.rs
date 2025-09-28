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
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FOLDERS_ROOT_FOLDER_META.to_string(),
            supported_access_token_types: vec![AccessTokenType::User],
            ..Default::default()
        };

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
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FOLDER_CHILDREN.replace("{}", &request.folder_token),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(order_by) = request.order_by {
            api_req.query_params.insert("order_by", order_by);
        }
        if let Some(direction) = request.direction {
            api_req.query_params.insert("direction", direction);
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
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FOLDER_GET.replace("{}", &request.folder_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

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
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FOLDERS.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

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
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FOLDER_MOVE.replace("{}", &request.folder_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

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
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_TASK_GET.replace("{}", &request.task_id),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

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

// === 宏实现 ===

impl_executable_builder_owned!(
    ListFilesRequestBuilder,
    FolderService,
    ListFilesRequest,
    BaseResponse<ListFilesRespData>,
    list_files
);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn mock_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    // === FolderService Tests ===

    #[test]
    fn test_folder_service_new() {
        let config = mock_config();
        let service = FolderService::new(config.clone());
        assert_eq!(service.config.app_id, config.app_id);
    }

    // === Request Data Structure Tests ===

    #[test]
    fn test_list_files_request_new() {
        let request = ListFilesRequest::new("test_folder_token");
        assert_eq!(request.folder_token, "test_folder_token");
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.order_by.is_none());
        assert!(request.direction.is_none());
    }

    #[test]
    fn test_list_files_request_builder() {
        let request = ListFilesRequest::builder()
            .folder_token("folder123")
            .page_token("page123")
            .page_size(100)
            .order_by("created_time")
            .direction("ASC")
            .build();

        assert_eq!(request.folder_token, "folder123");
        assert_eq!(request.page_token, Some("page123".to_string()));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.order_by, Some("created_time".to_string()));
        assert_eq!(request.direction, Some("ASC".to_string()));
    }

    #[test]
    fn test_list_files_request_builder_fluent() {
        let request = ListFilesRequest::builder()
            .folder_token("test")
            .page_size(50)
            .order_by("modified_time")
            .build();

        assert_eq!(request.folder_token, "test");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.order_by, Some("modified_time".to_string()));
        assert!(request.page_token.is_none());
        assert!(request.direction.is_none());
    }

    #[test]
    fn test_get_folder_meta_request_new() {
        let request = GetFolderMetaRequest::new("folder_token_123");
        assert_eq!(request.folder_token, "folder_token_123");
    }

    #[test]
    fn test_create_folder_request_new() {
        let request = CreateFolderRequest::new("My Folder", "parent_token_456");
        assert_eq!(request.name, "My Folder");
        assert_eq!(request.parent_token, "parent_token_456");
    }

    #[test]
    fn test_move_or_delete_folder_request_move() {
        let request = MoveOrDeleteFolderRequest::move_folder("folder123", "new_parent456");
        assert_eq!(request.folder_token, "folder123");
        assert_eq!(request.operation_type, "move");
        assert_eq!(request.parent_token, Some("new_parent456".to_string()));
    }

    #[test]
    fn test_move_or_delete_folder_request_delete() {
        let request = MoveOrDeleteFolderRequest::delete_folder("folder789");
        assert_eq!(request.folder_token, "folder789");
        assert_eq!(request.operation_type, "delete");
        assert!(request.parent_token.is_none());
    }

    #[test]
    fn test_check_async_task_request_new() {
        let request = CheckAsyncTaskRequest::new("task_id_abc");
        assert_eq!(request.task_id, "task_id_abc");
    }

    // === Response Data Structure Tests ===

    #[test]
    fn test_get_root_folder_meta_resp_data() {
        let data = GetRootFolderMetaRespData {
            token: "root_token".to_string(),
            user_id: "user123".to_string(),
        };
        assert_eq!(data.token, "root_token");
        assert_eq!(data.user_id, "user123");
    }

    #[test]
    fn test_list_files_resp_data() {
        let file = DriveFile {
            token: "file_token".to_string(),
            name: "document.pdf".to_string(),
            file_type: "pdf".to_string(),
            parent_token: Some("parent123".to_string()),
            url: Some("https://example.com/file".to_string()),
            short_url: Some("https://short.ly/abc".to_string()),
            size: Some(1024000),
            mime_type: Some("application/pdf".to_string()),
            created_time: Some("2023-01-01T00:00:00Z".to_string()),
            modified_time: Some("2023-01-02T00:00:00Z".to_string()),
            owner_id: Some("owner123".to_string()),
        };

        let data = ListFilesRespData {
            has_more: true,
            page_token: Some("next_page".to_string()),
            files: vec![file.clone()],
        };

        assert!(data.has_more);
        assert_eq!(data.page_token, Some("next_page".to_string()));
        assert_eq!(data.files.len(), 1);
        assert_eq!(data.files[0].token, "file_token");
        assert_eq!(data.files[0].name, "document.pdf");
        assert_eq!(data.files[0].size, Some(1024000));
    }

    #[test]
    fn test_drive_file_optional_fields() {
        let file = DriveFile {
            token: "minimal_file".to_string(),
            name: "simple.txt".to_string(),
            file_type: "txt".to_string(),
            parent_token: None,
            url: None,
            short_url: None,
            size: None,
            mime_type: None,
            created_time: None,
            modified_time: None,
            owner_id: None,
        };

        assert_eq!(file.token, "minimal_file");
        assert_eq!(file.name, "simple.txt");
        assert!(file.parent_token.is_none());
        assert!(file.url.is_none());
        assert!(file.size.is_none());
    }

    #[test]
    fn test_get_folder_meta_resp_data() {
        let data = GetFolderMetaRespData {
            token: "folder_token".to_string(),
            id: "folder_id".to_string(),
            name: "My Documents".to_string(),
            parent_token: Some("parent_folder".to_string()),
            owner_id: "owner123".to_string(),
            creator_id: Some("creator456".to_string()),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            edit_time: "2023-01-02T00:00:00Z".to_string(),
            description: Some("Folder description".to_string()),
            url: "https://example.com/folder".to_string(),
        };

        assert_eq!(data.token, "folder_token");
        assert_eq!(data.name, "My Documents");
        assert_eq!(data.parent_token, Some("parent_folder".to_string()));
        assert_eq!(data.description, Some("Folder description".to_string()));
    }

    #[test]
    fn test_create_folder_resp_data() {
        let data = CreateFolderRespData {
            token: "new_folder_token".to_string(),
            url: "https://example.com/new-folder".to_string(),
        };
        assert_eq!(data.token, "new_folder_token");
        assert_eq!(data.url, "https://example.com/new-folder");
    }

    #[test]
    fn test_move_or_delete_folder_resp_data() {
        let data = MoveOrDeleteFolderRespData {
            task_id: Some("async_task_123".to_string()),
        };
        assert_eq!(data.task_id, Some("async_task_123".to_string()));

        let data_no_task = MoveOrDeleteFolderRespData { task_id: None };
        assert!(data_no_task.task_id.is_none());
    }

    #[test]
    fn test_check_async_task_resp_data() {
        let success_data = CheckAsyncTaskRespData {
            status: "SUCCESS".to_string(),
            error_msg: None,
        };
        assert_eq!(success_data.status, "SUCCESS");
        assert!(success_data.error_msg.is_none());

        let failure_data = CheckAsyncTaskRespData {
            status: "FAILURE".to_string(),
            error_msg: Some("Task failed due to insufficient permissions".to_string()),
        };
        assert_eq!(failure_data.status, "FAILURE");
        assert_eq!(
            failure_data.error_msg,
            Some("Task failed due to insufficient permissions".to_string())
        );
    }

    // === Serialization Tests ===

    #[rstest]
    #[case("list_files_request")]
    #[case("get_folder_meta_request")]
    #[case("create_folder_request")]
    #[case("move_or_delete_folder_request")]
    #[case("check_async_task_request")]
    fn test_request_serialization_roundtrip(#[case] request_type: &str) {
        match request_type {
            "list_files_request" => {
                let original = ListFilesRequest::builder()
                    .folder_token("test123")
                    .page_size(50)
                    .order_by("created_time")
                    .direction("DESC")
                    .build();
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: ListFilesRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.folder_token, deserialized.folder_token);
                assert_eq!(original.page_size, deserialized.page_size);
                assert_eq!(original.order_by, deserialized.order_by);
            }
            "get_folder_meta_request" => {
                let original = GetFolderMetaRequest::new("folder123");
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: GetFolderMetaRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.folder_token, deserialized.folder_token);
            }
            "create_folder_request" => {
                let original = CreateFolderRequest::new("Test Folder", "parent123");
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: CreateFolderRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.name, deserialized.name);
                assert_eq!(original.parent_token, deserialized.parent_token);
            }
            "move_or_delete_folder_request" => {
                let original = MoveOrDeleteFolderRequest::move_folder("folder123", "new_parent");
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: MoveOrDeleteFolderRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.folder_token, deserialized.folder_token);
                assert_eq!(original.operation_type, deserialized.operation_type);
                assert_eq!(original.parent_token, deserialized.parent_token);
            }
            "check_async_task_request" => {
                let original = CheckAsyncTaskRequest::new("task123");
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: CheckAsyncTaskRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.task_id, deserialized.task_id);
            }
            _ => panic!("Unknown request type: {}", request_type),
        }
    }

    #[rstest]
    #[case("get_root_folder_meta_resp")]
    #[case("list_files_resp")]
    #[case("get_folder_meta_resp")]
    #[case("create_folder_resp")]
    #[case("move_or_delete_folder_resp")]
    #[case("check_async_task_resp")]
    fn test_response_serialization_roundtrip(#[case] response_type: &str) {
        match response_type {
            "get_root_folder_meta_resp" => {
                let original = GetRootFolderMetaRespData {
                    token: "root123".to_string(),
                    user_id: "user456".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: GetRootFolderMetaRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.token, deserialized.token);
                assert_eq!(original.user_id, deserialized.user_id);
            }
            "list_files_resp" => {
                let original = ListFilesRespData {
                    has_more: false,
                    page_token: None,
                    files: vec![],
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: ListFilesRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.has_more, deserialized.has_more);
                assert_eq!(original.page_token, deserialized.page_token);
                assert_eq!(original.files.len(), deserialized.files.len());
            }
            "get_folder_meta_resp" => {
                let original = GetFolderMetaRespData {
                    token: "folder123".to_string(),
                    id: "id123".to_string(),
                    name: "Test".to_string(),
                    parent_token: None,
                    owner_id: "owner123".to_string(),
                    creator_id: None,
                    create_time: "2023-01-01T00:00:00Z".to_string(),
                    edit_time: "2023-01-01T00:00:00Z".to_string(),
                    description: None,
                    url: "https://example.com".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: GetFolderMetaRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.token, deserialized.token);
                assert_eq!(original.name, deserialized.name);
            }
            "create_folder_resp" => {
                let original = CreateFolderRespData {
                    token: "new123".to_string(),
                    url: "https://example.com/new".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: CreateFolderRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.token, deserialized.token);
                assert_eq!(original.url, deserialized.url);
            }
            "move_or_delete_folder_resp" => {
                let original = MoveOrDeleteFolderRespData {
                    task_id: Some("task123".to_string()),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: MoveOrDeleteFolderRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.task_id, deserialized.task_id);
            }
            "check_async_task_resp" => {
                let original = CheckAsyncTaskRespData {
                    status: "PENDING".to_string(),
                    error_msg: None,
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: CheckAsyncTaskRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.status, deserialized.status);
                assert_eq!(original.error_msg, deserialized.error_msg);
            }
            _ => panic!("Unknown response type: {}", response_type),
        }
    }

    // === ApiResponseTrait Tests ===

    #[rstest]
    #[case("GetRootFolderMetaRespData")]
    #[case("ListFilesRespData")]
    #[case("GetFolderMetaRespData")]
    #[case("CreateFolderRespData")]
    #[case("MoveOrDeleteFolderRespData")]
    #[case("CheckAsyncTaskRespData")]
    fn test_api_response_trait(#[case] response_type: &str) {
        let format = match response_type {
            "GetRootFolderMetaRespData" => GetRootFolderMetaRespData::data_format(),
            "ListFilesRespData" => ListFilesRespData::data_format(),
            "GetFolderMetaRespData" => GetFolderMetaRespData::data_format(),
            "CreateFolderRespData" => CreateFolderRespData::data_format(),
            "MoveOrDeleteFolderRespData" => MoveOrDeleteFolderRespData::data_format(),
            "CheckAsyncTaskRespData" => CheckAsyncTaskRespData::data_format(),
            _ => panic!("Unknown response type: {}", response_type),
        };
        assert_eq!(format, ResponseFormat::Data);
    }

    // === Edge Cases and Validation Tests ===

    #[test]
    fn test_empty_folder_token() {
        let request = ListFilesRequest::new("");
        assert_eq!(request.folder_token, "");
    }

    #[test]
    fn test_very_long_folder_token() {
        let long_token = "a".repeat(1000);
        let request = ListFilesRequest::new(&long_token);
        assert_eq!(request.folder_token, long_token);
    }

    #[test]
    fn test_unicode_folder_names() {
        let unicode_name = "文件夹测试🗂️";
        let request = CreateFolderRequest::new(unicode_name, "parent123");
        assert_eq!(request.name, unicode_name);
    }

    #[test]
    fn test_special_characters_in_names() {
        let special_name = "Folder with spaces & symbols @#$%";
        let request = CreateFolderRequest::new(special_name, "parent");
        assert_eq!(request.name, special_name);
    }

    #[test]
    fn test_large_page_size() {
        let request = ListFilesRequest::builder()
            .folder_token("test")
            .page_size(999999)
            .build();
        assert_eq!(request.page_size, Some(999999));
    }

    #[test]
    fn test_negative_page_size() {
        let request = ListFilesRequest::builder()
            .folder_token("test")
            .page_size(-1)
            .build();
        assert_eq!(request.page_size, Some(-1));
    }

    #[test]
    fn test_drive_file_with_large_size() {
        let file = DriveFile {
            token: "large_file".to_string(),
            name: "huge_video.mp4".to_string(),
            file_type: "mp4".to_string(),
            parent_token: Some("parent".to_string()),
            url: None,
            short_url: None,
            size: Some(i64::MAX),
            mime_type: Some("video/mp4".to_string()),
            created_time: None,
            modified_time: None,
            owner_id: None,
        };
        assert_eq!(file.size, Some(i64::MAX));
    }

    #[test]
    fn test_drive_file_zero_size() {
        let file = DriveFile {
            token: "empty_file".to_string(),
            name: "empty.txt".to_string(),
            file_type: "txt".to_string(),
            parent_token: None,
            url: None,
            short_url: None,
            size: Some(0),
            mime_type: Some("text/plain".to_string()),
            created_time: None,
            modified_time: None,
            owner_id: None,
        };
        assert_eq!(file.size, Some(0));
    }

    #[test]
    fn test_task_status_variations() {
        let statuses = ["PENDING", "SUCCESS", "FAILURE", "RUNNING", "CANCELLED"];
        for status in statuses {
            let resp = CheckAsyncTaskRespData {
                status: status.to_string(),
                error_msg: None,
            };
            assert_eq!(resp.status, status);
        }
    }

    #[test]
    fn test_long_error_message() {
        let long_error = "Error: ".repeat(100);
        let resp = CheckAsyncTaskRespData {
            status: "FAILURE".to_string(),
            error_msg: Some(long_error.clone()),
        };
        assert_eq!(resp.error_msg, Some(long_error));
    }

    // === Builder Pattern Edge Cases ===

    #[test]
    fn test_list_files_builder_chaining() {
        let builder = ListFilesRequest::builder();
        let request = builder
            .folder_token("test")
            .page_size(10)
            .page_size(20) // Override previous
            .order_by("name")
            .build();
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.order_by, Some("name".to_string()));
    }

    #[test]
    fn test_empty_builder() {
        let request = ListFilesRequest::builder().build();
        assert_eq!(request.folder_token, "");
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    // === Sorting and Direction Tests ===

    #[rstest]
    #[case("created_time", "ASC")]
    #[case("edited_time", "DESC")]
    #[case("file_type", "ASC")]
    #[case("size", "DESC")]
    #[case("name", "ASC")]
    fn test_valid_sort_combinations(#[case] order_by: &str, #[case] direction: &str) {
        let request = ListFilesRequest::builder()
            .folder_token("test")
            .order_by(order_by)
            .direction(direction)
            .build();
        assert_eq!(request.order_by, Some(order_by.to_string()));
        assert_eq!(request.direction, Some(direction.to_string()));
    }

    #[test]
    fn test_invalid_sort_parameters() {
        let request = ListFilesRequest::builder()
            .folder_token("test")
            .order_by("invalid_field")
            .direction("INVALID_DIRECTION")
            .build();
        assert_eq!(request.order_by, Some("invalid_field".to_string()));
        assert_eq!(request.direction, Some("INVALID_DIRECTION".to_string()));
    }

    // === Clone and Debug Tests ===

    #[test]
    fn test_request_clone() {
        let original = ListFilesRequest::builder()
            .folder_token("test")
            .page_size(50)
            .build();
        let cloned = original.clone();
        assert_eq!(original.folder_token, cloned.folder_token);
        assert_eq!(original.page_size, cloned.page_size);
    }

    #[test]
    fn test_response_debug() {
        let data = CreateFolderRespData {
            token: "debug_test".to_string(),
            url: "https://test.com".to_string(),
        };
        let debug_str = format!("{:?}", data);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("https://test.com"));
    }
}
