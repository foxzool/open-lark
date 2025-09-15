use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::aily::models::{
        DataKnowledge, DataKnowledgeAnswer, DataKnowledgeAskRequest, DataKnowledgeCategory,
        DataKnowledgeCategoryListRequest, DataKnowledgeCreateRequest, DataKnowledgeDeleteRequest,
        DataKnowledgeFileUploadRequest, DataKnowledgeGetRequest, DataKnowledgeListRequest,
        FileUploadResult, PageResponse,
    },
};

/// 知识问答服务
pub struct KnowledgeService {
    pub config: Config,
}

/// 知识问答响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeAskResponse {
    /// 问答结果
    #[serde(flatten)]
    pub answer: DataKnowledgeAnswer,
}

impl ApiResponseTrait for DataKnowledgeAskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeFileUploadResponse {
    /// 文件上传结果
    #[serde(flatten)]
    pub upload_result: FileUploadResult,
}

impl ApiResponseTrait for DataKnowledgeFileUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据知识创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeCreateResponse {
    /// 数据知识信息
    #[serde(flatten)]
    pub knowledge: DataKnowledge,
}

impl ApiResponseTrait for DataKnowledgeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据知识查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeGetResponse {
    /// 数据知识信息
    #[serde(flatten)]
    pub knowledge: DataKnowledge,
}

impl ApiResponseTrait for DataKnowledgeGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据知识删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeDeleteResponse {
    /// 删除成功标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DataKnowledgeDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据知识列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<DataKnowledge>,
}

impl ApiResponseTrait for DataKnowledgeListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据知识分类列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnowledgeCategoryListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<DataKnowledgeCategory>,
}

impl ApiResponseTrait for DataKnowledgeCategoryListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl KnowledgeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行数据知识问答
    ///
    /// 该接口用于基于数据知识库进行智能问答。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识问答请求参数
    /// - `option`: 可选的请求配置
    pub async fn ask_data_knowledge(
        &self,
        request: DataKnowledgeAskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeAskResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::AILY_V1_DATA_KNOWLEDGE_ASK.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "question": request.question,
                "knowledge_base_ids": request.knowledge_base_ids,
                "chat_history": request.chat_history,
                "retrieval_config": request.retrieval_config
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传文件用于数据知识管理
    ///
    /// 该接口用于上传文件以创建数据知识。
    ///
    /// # 参数
    ///
    /// - `request`: 文件上传请求参数
    /// - `option`: 可选的请求配置
    pub async fn upload_file(
        &self,
        request: DataKnowledgeFileUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeFileUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "file": request.file,
                "file_name": request.file_name,
                "file_type": request.file_type
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建数据知识
    ///
    /// 该接口用于创建新的数据知识条目。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识创建请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_data_knowledge(
        &self,
        request: DataKnowledgeCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::AILY_V1_DATA_KNOWLEDGE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "title": request.title,
                "content": request.content,
                "category_id": request.category_id,
                "tags": request.tags,
                "metadata": request.metadata
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取数据知识
    ///
    /// 该接口用于获取指定的数据知识详情。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_data_knowledge(
        &self,
        request: DataKnowledgeGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeGetResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_DATA_KNOWLEDGE_OPERATION,
                "knowledge_id",
                &request.knowledge_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除数据知识
    ///
    /// 该接口用于删除指定的数据知识。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识删除请求参数
    /// - `option`: 可选的请求配置
    pub async fn delete_data_knowledge(
        &self,
        request: DataKnowledgeDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeDeleteResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_DATA_KNOWLEDGE_OPERATION,
                "knowledge_id",
                &request.knowledge_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询数据知识列表
    ///
    /// 该接口用于查询数据知识列表。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_data_knowledge(
        &self,
        request: DataKnowledgeListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::AILY_V1_DATA_KNOWLEDGE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(category_id) = request.category_id {
            api_req.query_params.insert("category_id", category_id);
        }
        if let Some(keyword) = request.keyword {
            api_req.query_params.insert("keyword", keyword);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取数据知识分类列表
    ///
    /// 该接口用于获取数据知识的分类列表。
    ///
    /// # 参数
    ///
    /// - `request`: 数据知识分类列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_data_knowledge_categories(
        &self,
        request: DataKnowledgeCategoryListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataKnowledgeCategoryListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::AILY_V1_DATA_KNOWLEDGE_CATEGORIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
