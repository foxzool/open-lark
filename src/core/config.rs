use std::collections::HashMap;
use std::time::Duration;
use ureq::Agent;

use crate::core::constants::AppType;
use crate::core::constants::FEISHU_BASE_URL;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
    /// 域名, 默认为 https://open.feishu.cn
    pub base_url: String,
    pub enable_token_cache: bool,
    /// 客户端超时时间, 单位秒, 默认永不超时
    pub timeout: Option<f32>,
    /// 应用类型, 默认为自建应用
    pub app_type: AppType,
    pub http_client: ureq::Agent,
    pub req_timeout: Option<Duration>,
    pub header: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: FEISHU_BASE_URL.to_string(),
            enable_token_cache: true,
            timeout: None,
            app_type: AppType::SelfBuild,
            http_client: Agent::new(),
            req_timeout: None,
            header: Default::default(),
        }
    }
}