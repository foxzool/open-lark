use std::collections::HashMap;
use crate::core::constants::FEISHU_BASE_URL;
use crate::core::constants::AppType;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
    /// 域名, 默认为 https://open.feishu.cn
    pub base_url: String,
    pub enable_token_cache: bool,
    /// 客户端超时时间, 单位秒, 默认永不超时
    pub timeout: Option<f32>,
    /// 应用类型, 默认为自建应用; 若设为 ISV 需在 request_option 中配置 tenant_key
    pub app_type: AppType,
    /// 获取 app_access_token 凭证, app_type = ISV 时需配置
    pub app_ticket: Option<String>,
    /// 是否允许手动设置 token, 默认不开启; 开启后需在 request_option 中配置 token
    pub enable_set_token: bool,
    pub http_client: reqwest::blocking::Client,
    pub header: HashMap<String, String>
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
            app_ticket: None,
            enable_set_token: false,
            http_client: Default::default(),
            header: Default::default(),
        }
    }
}
