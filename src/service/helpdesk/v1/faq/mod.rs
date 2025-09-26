use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, BinaryResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{Faq, UserIdType},
};

/// 知识库管理服务
pub struct FaqService {
    pub config: Config,
}

/// 创建知识库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFaqRequest {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 创建知识库响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFaqResponse {
    /// 创建的知识库
    pub faq: Faq,
}

impl ApiResponseTrait for CreateFaqResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改知识库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFaqRequest {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 修改知识库响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFaqResponse {
    /// 更新后的知识库
    pub faq: Faq,
}

impl ApiResponseTrait for UpdateFaqResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识库详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetFaqResponse {
    /// 知识库详情
    pub faq: Faq,
}

impl ApiResponseTrait for GetFaqResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全部知识库详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFaqsResponse {
    /// 知识库列表
    pub faqs: Vec<Faq>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListFaqsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索知识库响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFaqsResponse {
    /// 搜索结果
    pub faqs: Vec<Faq>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchFaqsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FaqService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建知识库
    ///
    /// 该接口用于创建知识库条目。
    ///
    /// # 参数
    ///
    /// - `request`: 创建请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 分类不存在
    pub async fn create(
        &self,
        request: CreateFaqRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFaqResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: HELPDESK_V1_FAQ_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除知识库
    ///
    /// 该接口用于删除指定的知识库条目。
    ///
    /// # 参数
    ///
    /// - `faq_id`: 知识库ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 知识库不存在
    pub async fn delete(
        &self,
        faq_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(HELPDESK_V1_FAQ_DELETE, "faq_id", faq_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改知识库
    ///
    /// 该接口用于修改指定的知识库条目。
    ///
    /// # 参数
    ///
    /// - `faq_id`: 知识库ID
    /// - `request`: 修改请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 知识库不存在
    pub async fn patch(
        &self,
        faq_id: &str,
        request: UpdateFaqRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateFaqResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(HELPDESK_V1_FAQ_UPDATE, "faq_id", faq_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指定知识库详情
    ///
    /// 该接口用于获取指定知识库的详细信息。
    ///
    /// # 参数
    ///
    /// - `faq_id`: 知识库ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 知识库不存在
    pub async fn get(
        &self,
        faq_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFaqResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(HELPDESK_V1_FAQ_GET, "faq_id", faq_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取全部知识库详情
    ///
    /// 该接口用于获取全部知识库条目列表。
    ///
    /// # 参数
    ///
    /// - `user_id_type`: 用户ID类型
    /// - `category_id`: 分类ID
    /// - `status`: 状态
    /// - `page_token`: 分页标记
    /// - `page_size`: 分页大小
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        category_id: Option<&str>,
        status: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFaqsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(category_id) = category_id {
            query_params.insert("category_id", category_id.to_string());
        }
        if let Some(status) = status {
            query_params.insert("status", status.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: HELPDESK_V1_FAQS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取知识库图像
    ///
    /// 该接口用于获取知识库中的图像文件。
    ///
    /// # 参数
    ///
    /// - `faq_id`: 知识库ID
    /// - `image_key`: 图像key
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 图像不存在
    pub async fn faq_image(
        &self,
        faq_id: &str,
        image_key: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BinaryResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(HELPDESK_V1_FAQ_IMAGE, "faq_id", faq_id),
                "image_key",
                image_key,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索知识库
    ///
    /// 该接口用于搜索知识库条目。
    ///
    /// # 参数
    ///
    /// - `query`: 搜索关键词
    /// - `user_id_type`: 用户ID类型
    /// - `category_id`: 分类ID
    /// - `page_token`: 分页标记
    /// - `page_size`: 分页大小
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    pub async fn search(
        &self,
        query: &str,
        user_id_type: Option<UserIdType>,
        category_id: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchFaqsResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("query", query.to_string());
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(category_id) = category_id {
            query_params.insert("category_id", category_id.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: HELPDESK_V1_FAQS_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
