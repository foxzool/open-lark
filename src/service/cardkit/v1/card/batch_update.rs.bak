use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::{cardkit::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::cardkit::v1::models::{BatchUpdateOperation, Card, UserIdType},
};

use super::CardService;

/// 批量更新卡片实体请求
#[derive(Default, Clone)]
pub struct BatchUpdateCardRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 批量更新操作列表
    pub operations: Vec<BatchUpdateOperation>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl BatchUpdateCardRequest {
    /// 创建批量更新卡片请求的构建器
    pub fn builder(card_id: impl ToString) -> BatchUpdateCardRequestBuilder {
        BatchUpdateCardRequestBuilder {
            request: BatchUpdateCardRequest {
                card_id: card_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 批量更新卡片实体请求构建器
pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest,
}

impl BatchUpdateCardRequestBuilder {
    /// 添加更新操作
    pub fn add_operation(mut self, operation: BatchUpdateOperation) -> Self {
        self.request.operations.push(operation);
        self
    }

    /// 添加多个更新操作
    pub fn add_operations(mut self, operations: Vec<BatchUpdateOperation>) -> Self {
        self.request.operations.extend(operations);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> BatchUpdateCardRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let body = json!({
            "operations": self.request.operations
        });

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 批量更新卡片实体响应数据
#[derive(Debug, Deserialize)]
pub struct BatchUpdateCardResponseData {
    /// 更新后的卡片信息
    pub card: Card,
}

/// 批量更新卡片实体响应
#[derive(Debug, Deserialize)]
pub struct BatchUpdateCardResponse {
    /// 响应数据
    pub data: BatchUpdateCardResponseData,
}

impl ApiResponseTrait for BatchUpdateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 批量更新卡片实体
    ///
    /// 对指定卡片执行批量更新操作。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 批量更新卡片请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新后的卡片信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let operations = vec![
    ///     BatchUpdateOperation {
    ///         operation: "replace".to_string(),
    ///         path: "/title".to_string(),
    ///         value: Some(serde_json::json!("新标题")),
    ///     }
    /// ];
    ///
    /// let response = client.cardkit.v1.card.batch_update(
    ///     BatchUpdateCardRequest::builder("card_id")
    ///         .add_operations(operations)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>
    pub async fn batch_update(
        &self,
        request: BatchUpdateCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchUpdateCardResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PATCH;
        api_req.api_path = EndpointBuilder::replace_param(
            CARDKIT_V1_CARD_BATCH_UPDATE,
            "card_id",
            &request.card_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    BatchUpdateCardRequestBuilder,
    CardService,
    BatchUpdateCardRequest,
    BaseResponse<BatchUpdateCardResponse>,
    batch_update
);
