/// 获取知识空间列表
///
/// 此接口用于获取有权限访问的知识空间列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list

use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesRequest {
    /// 分页大小，默认20，最大100
    pub page_size: Option<i32>,
    /// 分页标记，第一页不填，后续页面使用上一页返回的page_token
    pub page_token: Option<String>,
}

impl ListWikiSpacesRequest {
    /// 创建获取知识空间列表请求
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

impl Default for ListWikiSpacesRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// 知识空间信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSpaceInfo {
    /// 知识空间ID
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述
    pub description: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 知识空间可见性
    pub visibility: String,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

/// 获取知识空间列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesResponse {
    /// 知识空间列表信息
    pub data: Option<ListWikiSpacesData>,
}

/// 知识空间列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesData {
    /// 知识空间列表
    pub items: Vec<WikiSpaceInfo>,
    /// 分页token
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
}

impl ApiResponseTrait for ListWikiSpacesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间列表
///
/// 此接口用于获取有权限访问的知识空间列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list
pub async fn list_wiki_spaces(
    config: &Config,
    params: ListWikiSpacesRequest,
) -> SDKResult<ListWikiSpacesResponse> {
    // 使用WikiApiV2枚举生成API端点
    let api_endpoint = WikiApiV2::SpaceList;

    // 创建API请求
    let api_request: ApiRequest<ListWikiSpacesResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query_params(serialize_params(&params, "获取知识空间列表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取知识空间列表")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_wiki_spaces_request_builder() {
        let request = ListWikiSpacesRequest::new()
            .page_size(20)
            .page_token("next_page_token");

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token.unwrap(), "next_page_token");
    }

    #[test]
    fn test_default_request() {
        let request = ListWikiSpacesRequest::default();

        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
    }

    #[test]
    fn test_wiki_space_info_structure() {
        let creator = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let space_info = WikiSpaceInfo {
            space_id: "space_id".to_string(),
            name: "知识空间".to_string(),
            description: Some("知识空间描述".to_string()),
            create_time: "2024-01-01T00:00:00Z".to_string(),
            update_time: "2024-01-02T00:00:00Z".to_string(),
            creator: Some(creator),
            visibility: "public".to_string(),
        };

        assert_eq!(space_info.space_id, "space_id");
        assert_eq!(space_info.name, "知识空间");
        assert_eq!(space_info.visibility, "public");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListWikiSpacesResponse::data_format(),
            ResponseFormat::Data
        );
    }
}