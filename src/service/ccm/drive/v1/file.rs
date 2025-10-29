// drive v1 file - 文件操作API实现,
//,
// 实现文件和文件夹的CRUD操作，包括：,
// - 获取文件列表,
// - 创建文件夹,
// - 复制文件,
// - 移动文件,
// - 删除文件,
// - 创建快捷方式,
// - 异步任务查询,
use crate::core::{
    api_req::api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    req_option::RequestOption, standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder, SDKResult,
};
use crate::service::ccm::models::{,
    CcmResponse, PaginatedData, PaginatedResponse, FileMetadata, AsyncTask, FileType,
};
use serde::{Deserialize, Serialize};
/// 文件操作服务,
#[derive(.*?)]
pub struct FileService {
    client: std::sync::Arc<LarkClient>,
}
impl FileService {
    /// 创建新的文件操作服务实例,
pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
},
/// 获取文件夹中的文件清单,
    ///,
/// 获取用户云空间中指定文件夹下的文件清单。清单类型包括文件、各种在线文档（文档、电子表格、多维表格、思维笔记）、文件夹和快捷方式。,
    /// 该接口支持分页，但是不会递归的获取子文件夹的清单。,
///,
    /// # 参数,
/// - `parent_token`: 父文件夹token，为空时表示获取根目录,
    /// - `page_size`: 分页大小，最大1000,
/// - `page_token`: 分页标记,
    /// - `type`: 文件类型过滤,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.drive.v1.file.list("folder_token", 50, None, None).await?;
/// ```,
    pub async fn list(
        &self,
        parent_token: Option<&str>,
        page_size: i32,
        page_token: Option<&str>,
        file_type: Option<FileType>,
    ) -> APIResult<PaginatedResponse<FileMetadata>> {,
let mut request = RequestBuilder::get("/open-apis/drive/v1/files"),
            .query_param("page_size", &page_size.to_string());
if let Some(token) = parent_token {,
            request = request.query_param("parent_token", token);
},
if let Some(token) = page_token {,
            request = request.query_param("page_token", token);
},
if let Some(ft) = file_type {,
            request = request.query_param("type", &format!("{:?}", ft).to_lowercase());
},
self.client.send(request).await,
    },
/// 新建文件夹,
    ///,
/// 在用户云空间的指定文件夹中创建一个新的空文件夹。,
    ///,
/// # 参数,
    /// - `parent_token`: 父文件夹token,
/// - `name`: 文件夹名称,
    ///,
/// # 示例,
    /// ```ignore
    /// let response = client.drive.v1.file.create_folder("parent_token", "新文件夹").await?;
/// ```,
    pub async fn create_folder(
        &self,
        parent_token: &str,
        name: &str,
    ) -> APIResult<CcmResponse<FileMetadata>> {,
let request_body = CreateFolderRequest {,
            parent_token: parent_token.to_string(),
            name: name.to_string(),
            folder_type: "".to_string(), // 默认文件夹类型,
};
let request = RequestBuilder::post("/open-apis/drive/v1/files/create_folder"),
            .body_json(request_body);
self.client.send(request).await,
    },
/// 复制文件,
    ///,
/// 将文件复制到用户云空间的其他文件夹中。不支持复制文件夹。,
    ///,
/// # 参数,
    /// - `file_token`: 要复制的文件token,
/// - `parent_token`: 目标文件夹token,
    /// - `name`: 新文件名称（可选）,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.drive.v1.file.copy("file_token", "target_folder", Some("副本")).await?;
/// ```,
    pub async fn copy(
        &self,
        file_token: &str,
        parent_token: &str,
        name: Option<&str>,
    ) -> APIResult<CcmResponse<FileMetadata>> {,
let mut request_body = CopyFileRequest {,
            file_token: file_token.to_string(),
            parent_token: parent_token.to_string(),
            name: None,
        };
if let Some(n) = name {,
            request_body.name = Some(n.to_string());
},
let request = RequestBuilder::post(&format!(,
            "/open-apis/drive/v1/files/{}/copy",
            file_token,
)),
        .body_json(request_body);
self.client.send(request).await,
    },
/// 移动文件或文件夹,
    ///,
/// 将文件或者文件夹移动到用户云空间的其他位置。,
    ///,
/// # 参数,
    /// - `file_token`: 要移动的文件或文件夹token,
/// - `parent_token`: 目标文件夹token,
    ///,
/// # 示例,
    /// ```ignore
    /// let response = client.drive.v1.file.move_file("file_token", "target_folder").await?;
/// ```,
    pub async fn move_file(
        &self,
        file_token: &str,
        parent_token: &str,
    ) -> APIResult<CcmResponse<FileMetadata>> {,
let request_body = MoveFileRequest {,
            parent_token: parent_token.to_string(),
        };
let request = RequestBuilder::post(&format!(,
            "/open-apis/drive/v1/files/{}/move",
            file_token,
)),
        .body_json(request_body);
self.client.send(request).await,
    },
/// 删除文件或文件夹,
    ///,
/// 删除用户在云空间内的文件或者文件夹。文件或者文件夹被删除后，会进入用户回收站里。,
    ///,
/// # 参数,
    /// - `file_token`: 要删除的文件或文件夹token,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.drive.v1.file.delete("file_token").await?;
/// ```,
    pub async fn delete(&self, file_token: &str) -> APIResult<CcmResponse<serde_json::Value>> {,
let request = RequestBuilder::delete(&format!(,
            "/open-apis/drive/v1/files/{}",
            file_token,
));
        self.client.send(request).await,
},
/// 创建文件快捷方式,
    ///,
/// # 参数,
    /// - `file_token`: 源文件token,
/// - `parent_token`: 目标文件夹token,
    /// - `name`: 快捷方式名称,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.drive.v1.file.create_shortcut("file_token", "target_folder", "快捷方式").await?;
/// ```,
    pub async fn create_shortcut(
        &self,
        file_token: &str,
        parent_token: &str,
        name: &str,
    ) -> APIResult<CcmResponse<FileMetadata>> {,
let request_body = CreateShortcutRequest {,
            file_token: file_token.to_string(),
            parent_token: parent_token.to_string(),
            name: name.to_string(),
        };
let request = RequestBuilder::post("/open-apis/drive/v1/files/create_shortcut"),
            .body_json(request_body);
self.client.send(request).await,
    },
/// 查询异步任务状态,
    ///,
/// 查询删除文件夹等异步任务的状态信息。,
    ///,
/// # 参数,
    /// - `task_id`: 任务ID,
///,
    /// # 示例,
/// ```ignore,
    /// let response = client.drive.v1.file.task_check("task_id").await?;
/// ```,
    pub async fn task_check(&self, task_id: &str) -> APIResult<CcmResponse<AsyncTask>> {,
let request = RequestBuilder::get("/open-apis/drive/v1/files/task_check"),
            .query_param("task_id", task_id);
self.client.send(request).await,
    },
},
// 请求结构体,
#[derive(.*?)]
pub struct CreateFolderRequest {
    pub parent_token: String,
    pub name: String,
    pub folder_type: String,
}

#[derive(.*?)]
pub struct CopyFileRequest {
    pub file_token: String,
    pub parent_token: String,
    pub name: Option<String>,
}

#[derive(.*?)]
pub struct MoveFileRequest {
    pub parent_token: String,
}

#[derive(.*?)]
pub struct CreateShortcutRequest {
    pub file_token: String,
    pub parent_token: String,
    pub name: String,
}