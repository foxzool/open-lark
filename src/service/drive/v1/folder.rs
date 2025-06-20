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
    pub fn builder(folder_token: impl Into<String>) -> ListFilesRequestBuilder {
        ListFilesRequestBuilder {
            request: ListFilesRequest {
                folder_token: folder_token.into(),
                ..Default::default()
            },
        }
    }
}

/// 获取文件夹中的文件清单请求构建器
#[derive(Debug, Clone, Default)]
pub struct ListFilesRequestBuilder {
    request: ListFilesRequest,
}

impl ListFilesRequestBuilder {
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
