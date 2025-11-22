//! 文档元信息服务 - 修复版本
//!
//! 提供获取文档元信息的功能，包括文档标题、创建时间、
//! 更新时间、作者等信息。

use crate::prelude::*;
use openlark_core::{api::ApiRequest, constants::AccessTokenType, http::Transport, SDKResult};

use super::{requests::GetDocMetaV2Request, responses::GetDocMetaV2Response};

/// 文档元信息服务
#[derive(Clone, Debug)]
pub struct MetaDocService {
    config: Config,
}

impl MetaDocService {
    /// 创建元信息服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档元信息
    ///
    /// 获取文档的基本信息，包括标题、创建时间、更新时间等。
    ///
    /// # 参数
    /// - `req`: 获取文档元信息请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocMetaV2Response>`: 文档元信息响应
    pub async fn get(&self, req: &GetDocMetaV2Request) -> SDKResult<GetDocMetaV2Response> {
        log::debug!("获取文档元信息: doc_token={}", req.doc_token);

        // 构建动态端点路径，替换doc_token参数
        let endpoint = format!("/open-apis/doc/v2/{}", req.doc_token);

        let mut query = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query.insert("user_id_type".to_string(), user_id_type.clone());
        }

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: endpoint,
            headers: std::collections::HashMap::new(),
            query,
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        let resp = Transport::<GetDocMetaV2Response>::request(api_req, &self.config, None).await?;

        log::info!("获取文档元信息完成: doc_token={}", req.doc_token);

        Ok(resp.data.unwrap_or_default())
    }
}

/// 获取文档元信息构建器
#[derive(Debug, Clone)]
pub struct GetDocMetaV2Builder {
    request: GetDocMetaV2Request,
}

impl GetDocMetaV2Builder {
    /// 创建新的获取文档元信息构建器
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            request: GetDocMetaV2Request {
                doc_token: doc_token.into(),
                user_id_type: None,
            },
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> GetDocMetaV2Request {
        self.request
    }
}

impl Default for GetDocMetaV2Builder {
    fn default() -> Self {
        Self::new("")
    }
}
