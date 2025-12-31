//! 更新免审词条
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/update
//! doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::{Entity, OuterInfo, RelatedMeta, Term, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateEntityReq {
    /// 词条名
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本格式词条释义（description 与 rich_text 至少有一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更多相关信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 外部系统关联数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 富文本格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

/// 更新免审词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for UpdateEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新免审词条请求
pub struct UpdateEntityRequest {
    config: Config,
    entity_id: String,
    user_id_type: Option<UserIdType>,
    req: UpdateEntityReq,
}

impl UpdateEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>, req: UpdateEntityReq) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            user_id_type: None,
            req,
        }
    }

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn send(self) -> SDKResult<UpdateEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");
        validate_required!(self.req.main_keys, "main_keys 不能为空");
        if self.req.main_keys.len() > 1 {
            return Err(openlark_core::error::validation_error(
                "main_keys",
                "main_keys 最大长度为 1",
            ));
        }
        for (idx, term) in self.req.main_keys.iter().enumerate() {
            if term.key.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    &format!("main_keys[{}].key", idx),
                    "key 不能为空",
                ));
            }
        }
        if let Some(aliases) = &self.req.aliases {
            for (idx, term) in aliases.iter().enumerate() {
                if term.key.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("aliases[{}].key", idx),
                        "key 不能为空",
                    ));
                }
            }
        }
        if self
            .req
            .description
            .as_deref()
            .unwrap_or_default()
            .is_empty()
            && self.req.rich_text.as_deref().unwrap_or_default().is_empty()
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "description 与 rich_text 至少填写一个",
            ));
        }

        let mut api_request: ApiRequest<UpdateEntityResp> =
            ApiRequest::put(&BaikeApiV1::EntityUpdate(self.entity_id).to_url())
                .body(serde_json::to_value(&self.req)?);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<UpdateEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
