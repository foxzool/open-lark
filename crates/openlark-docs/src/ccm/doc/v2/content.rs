//! 文档内容服务 - 简化版本
//!
//! 提供获取文档内容的基本功能

use crate::prelude::*;
use openlark_core::{api::ApiRequest, http::Transport, SDKResult};

use super::{
    requests::{GetDocContentV2Request, GetDocRawContentV2Request},
    responses::{GetDocContentV2Response, GetDocRawContentV2Response},
};

/// 文档内容服务
#[derive(Clone, Debug)]
pub struct ContentDocService {
    config: Config,
}

impl ContentDocService {
    /// 创建内容服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档纯文本内容
    pub async fn get_raw_content(
        &self,
        req: &GetDocRawContentV2Request,
    ) -> SDKResult<GetDocRawContentV2Response> {
        log::debug!("获取文档纯文本内容: doc_token={}", req.doc_token);

        // 构建动态端点路径
        let endpoint = format!("/open-apis/doc/v2/{}/raw_content", req.doc_token);

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

        let resp =
            Transport::<GetDocRawContentV2Response>::request(api_req, &self.config, None).await?;
        log::info!("获取文档纯文本内容完成: doc_token={}", req.doc_token);

        Ok(resp.data.unwrap_or_default())
    }

    /// 获取文档富文本内容
    pub async fn get(&self, req: &GetDocContentV2Request) -> SDKResult<GetDocContentV2Response> {
        log::debug!("获取文档富文本内容: doc_token={}", req.doc_token);

        // 构建动态端点路径
        let endpoint = format!("/open-apis/doc/v2/{}/content", req.doc_token);

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

        let resp =
            Transport::<GetDocContentV2Response>::request(api_req, &self.config, None).await?;
        log::info!("获取文档富文本内容完成: doc_token={}", req.doc_token);

        Ok(resp.data.unwrap_or_default())
    }
}

/// 获取文档纯文本内容构建器
#[derive(Debug, Clone)]
pub struct GetDocRawContentV2Builder {
    request: GetDocRawContentV2Request,
}

impl GetDocRawContentV2Builder {
    /// 创建新的获取纯文本内容构建器
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            request: GetDocRawContentV2Request {
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
    pub fn build(self) -> GetDocRawContentV2Request {
        self.request
    }
}

impl Default for GetDocRawContentV2Builder {
    fn default() -> Self {
        Self::new("")
    }
}

/// 获取文档富文本内容构建器
#[derive(Debug, Clone)]
pub struct GetDocContentV2Builder {
    request: GetDocContentV2Request,
}

impl GetDocContentV2Builder {
    /// 创建新的获取富文本内容构建器
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            request: GetDocContentV2Request {
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
    pub fn build(self) -> GetDocContentV2Request {
        self.request
    }
}

impl Default for GetDocContentV2Builder {
    fn default() -> Self {
        Self::new("")
    }
}
