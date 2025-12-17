/// 获取文件夹下的文档清单
///
/// 根据 folderToken 获取该文件夹的文档清单，如 doc、sheet、file、bitable、folder。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 获取文件夹下的文档清单请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenParams {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 页面大小，最大值：100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 排序字段，可选值：name、create_time、update_time
    #[serde(rename = "order_by")]
    pub order_by: Option<String>,
    /// 排序方向，可选值：asc、desc
    #[serde(rename = "direction")]
    pub direction: Option<String>,
}

/// 获取文件夹下的文档清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenResponse {
    /// 文件夹内容
    pub data: Option<FolderChildren>,
}

/// 文件夹内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderChildren {
    /// 文件和文件夹列表
    pub items: Vec<DriveItem>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 分页token
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 云盘项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveItem {
    /// token
    pub token: String,
    /// 名称
    pub name: String,
    /// 类型，可选值：doc、sheet、bitable、mindnote、file、folder
    #[serde(rename = "type")]
    pub item_type: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for GetFolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹下的文档清单请求
pub struct GetFolderChildrenRequest {
    config: Config,
}

impl GetFolderChildrenRequest {
    /// 创建获取文件夹下的文档清单请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children
    pub async fn execute(
        self,
        params: GetFolderChildrenParams,
    ) -> SDKResult<GetFolderChildrenResponse> {
        // 验证必填字段
        validate_required!(params.folder_token, "文件夹token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::FolderChildren(params.folder_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetFolderChildrenResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 添加查询参数
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &params.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(order_by) = &params.order_by {
            api_request = api_request.query("order_by", order_by);
        }
        if let Some(direction) = &params.direction {
            api_request = api_request.query("direction", direction);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
