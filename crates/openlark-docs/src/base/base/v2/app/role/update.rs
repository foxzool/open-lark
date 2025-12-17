//! 更新自定义角色
//!
//! 
//!
//! [官方文档](https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/update-2)

use crate::base::base::v2::models::*;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestBuilder, Send},
    config::Config,
    error::Error,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// 更新自定义角色
#[derive(Debug)]
pub struct Update {
    config: Config,
    app_token: String,
    role_id: String,
    req: UpdateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
    /// 自定义角色的名字
    pub role_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResp {
    /// 自定义角色
    pub role: AppRole,
}

impl Update {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            req: UpdateReq {
                role_name: String::new(),
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    pub async fn send(self) -> Result<Response<UpdateResp>, Error> {
        let url = format!(
            "{}/open-apis/base/v2/apps/{}/roles/{}",
            self.config.base_url, self.app_token, self.role_id
        );
        let request = ApiRequest::put(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}

impl ApiResponseTrait for UpdateResp {}
