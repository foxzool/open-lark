//! 新增自定义角色
//!
//! 
//!
//! [官方文档](https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2)

use crate::base::base::v2::models::*;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestBuilder, Send},
    config::Config,
    error::Error,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// 新增自定义角色
#[derive(Debug)]
pub struct Create {
    config: Config,
    app_token: String,
    req: CreateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
    /// 自定义角色的名字
    pub role_name: String,
    /// 角色ID，以 "custom_" 开头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResp {
    /// 自定义角色
    pub role: AppRole,
}

impl Create {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CreateReq {
                role_name: String::new(),
                role_id: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.req.role_id = Some(role_id.into());
        self
    }

    pub async fn send(self) -> Result<Response<CreateResp>, Error> {
        let url = format!(
            "{}/open-apis/base/v2/apps/{}/roles",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}

impl ApiResponseTrait for CreateResp {}
