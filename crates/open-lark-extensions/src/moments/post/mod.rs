use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::moments::models::{Post, PostGetRequest},
};

/// 帖子管理服务
pub struct PostService {
    pub config: Config,
}

/// 查询帖子信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PostGetResponse {
    /// 帖子详细信息
    #[serde(flatten)]
    pub post: Post,
}

impl ApiResponseTrait for PostGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PostService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询帖子信息
    ///
    /// 该接口用于查询指定帖子的详细信息，包括帖子内容、作者信息、
    /// 统计数据等。支持获取帖子的媒体附件、可见性设置等完整信息。
    ///
    /// # 参数
    ///
    /// - `request`: 帖子查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回包含帖子详细信息的响应，包括：
    /// - 帖子基本信息（标题、内容、作者等）
    /// - 媒体附件列表
    /// - 统计数据（评论数、点赞数等）
    /// - 可见性设置
    pub async fn get_post(
        &self,
        request: PostGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PostGetResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/moments/v1/posts/{}", request.post_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
