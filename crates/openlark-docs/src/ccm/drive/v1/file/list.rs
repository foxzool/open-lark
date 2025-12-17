use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取文件夹中的文件清单
///
/// 获取用户云空间中指定文件夹下的文件清单。清单类型包括文件、各种在线文档（文档、电子表格、多维表格、思维笔记）、文件夹和快捷方式。
/// 该接口支持分页，但是不会递归的获取子文件夹的清单。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取文件夹中的文件清单请求
#[derive(Debug)]
pub struct ListFilesRequest {
    config: Config,
    /// 文件夹token，不填则获取根目录
    pub parent_folder_token: Option<String>,
    /// 分页大小，默认50，最大1000
    pub page_size: Option<i32>,
    /// 分页标记，第一页不填，后续页面使用上一页返回的page_token
    pub page_token: Option<String>,
    /// 排序字段：name|modified_time|created_time|size
    pub order_by: Option<String>,
    /// 排序方向：asc|desc
    pub direction: Option<String>,
    /// 搜索关键字
    pub search_key: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 创建时间
    pub created_time: String,
    /// 修改时间
    pub modified_time: String,
    /// 创建者信息
    pub creator: Option<serde_json::Value>,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 获取文件夹中的文件清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// 文件列表信息
    pub data: Option<ListFilesData>,
}

/// 文件列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesData {
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 分页token
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
}

impl ApiResponseTrait for ListFilesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListFilesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            parent_folder_token: None,
            page_size: None,
            page_token: None,
            order_by: None,
            direction: None,
            search_key: None,
            file_type: None,
        }
    }

    pub fn parent_folder_token(mut self, parent_folder_token: impl Into<String>) -> Self {
        self.parent_folder_token = Some(parent_folder_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn search_key(mut self, search_key: impl Into<String>) -> Self {
        self.search_key = Some(search_key.into());
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<ListFilesResponse>> {
        let api_endpoint = DriveApi::ListFiles;
        let mut request = ApiRequest::<ListFilesResponse>::get(&api_endpoint.to_url());

        if let Some(token) = &self.parent_folder_token {
            request = request.query("folder_token", token);
        }
        if let Some(size) = self.page_size {
            request = request.query("page_size", size.to_string());
        }
        if let Some(token) = &self.page_token {
            request = request.query("page_token", token);
        }
        if let Some(order) = &self.order_by {
            request = request.query("order_by", order);
        }
        if let Some(direction) = &self.direction {
            request = request.query("direction", direction);
        }
        if let Some(key) = &self.search_key {
            request = request.query("search_key", key);
        }
        if let Some(t) = &self.file_type {
            request = request.query("file_type", t);
        }

        Transport::request(request, &self.config, None).await
    }
}
