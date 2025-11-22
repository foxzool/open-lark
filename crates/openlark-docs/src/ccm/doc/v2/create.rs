//! 文档创建服务 - 简化版本
//!
//! 提供创建文档的基本功能

use crate::prelude::*;
use openlark_core::{api::ApiRequest, http::Transport, SDKResult};

use super::{requests::CreateDocV2Request, responses::CreateDocV2Response};

/// 文档创建服务
#[derive(Clone, Debug)]
pub struct CreateDocService {
    config: Config,
}

impl CreateDocService {
    /// 创建文档服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    pub async fn create(&self, req: &CreateDocV2Request) -> SDKResult<CreateDocV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始创建文档: title={:?}", req.title);

        let endpoint = "/open-apis/doc/v2/create".to_string();

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: endpoint,
            headers: std::collections::HashMap::new(),
            query: std::collections::HashMap::new(),
            body: Some(openlark_core::api::RequestData::Json(serde_json::to_value(
                req,
            )?)),
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        let resp = Transport::<CreateDocV2Response>::request(api_req, &self.config, None).await?;

        log::info!("创建文档完成: title={:?}", req.title);

        Ok(resp.data.unwrap_or_default())
    }
}

/// 创建文档构建器
#[derive(Debug, Clone)]
pub struct CreateDocBuilder {
    request: CreateDocV2Request,
}

impl CreateDocBuilder {
    /// 创建新的文档构建器
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            request: CreateDocV2Request {
                title: title.into(),
                doc_type: None,
                folder_token: None,
                template_id: None,
            },
        }
    }

    /// 设置文档类型
    pub fn doc_type(mut self, doc_type: super::models::DocType) -> Self {
        self.request.doc_type = Some(doc_type);
        self
    }

    /// 设置文件夹Token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 设置模板ID
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.request.template_id = Some(template_id.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> SDKResult<CreateDocV2Request> {
        self.request
            .validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        Ok(self.request)
    }
}

impl Default for CreateDocBuilder {
    fn default() -> Self {
        Self::new("")
    }
}
