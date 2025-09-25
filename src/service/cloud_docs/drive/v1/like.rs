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

/// 点赞服务
pub struct LikeService {
    config: Config,
}

impl LikeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取云文档的点赞者列表
    ///
    /// 该接口用于获取云文档的点赞者列表。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/file-like_record/list>
    pub async fn list_file_likes(
        &self,
        request: ListFileLikesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFileLikesRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FILE_LIKE_RECORDS.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取云文档的点赞者列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFileLikesRequest {
    /// 文件token
    pub file_token: String,
    /// 分页token
    pub page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
}

impl ListFileLikesRequest {
    pub fn builder() -> ListFileLikesRequestBuilder {
        ListFileLikesRequestBuilder::default()
    }

    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            page_token: None,
            page_size: None,
        }
    }

    pub fn with_page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 获取云文档的点赞者列表请求构建器
#[derive(Default)]
pub struct ListFileLikesRequestBuilder {
    request: ListFileLikesRequest,
}

impl ListFileLikesRequestBuilder {
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
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

    pub fn build(self) -> ListFileLikesRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    ListFileLikesRequestBuilder,
    LikeService,
    ListFileLikesRequest,
    BaseResponse<ListFileLikesRespData>,
    list_file_likes
);

/// 获取云文档的点赞者列表响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileLikesRespData {
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页token
    pub page_token: Option<String>,
    /// 点赞记录列表
    pub items: Vec<FileLikeRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLikeRecord {
    /// 点赞者ID
    pub user_id: String,
    /// 点赞者姓名
    pub user_name: String,
    /// 点赞时间（时间戳，单位：秒）
    pub like_time: String,
    /// 点赞者头像
    pub avatar_url: Option<String>,
}

impl ApiResponseTrait for ListFileLikesRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
