use crate::core::constants::FEISHU_DOMAIN;
use crate::core::enum_type::AppType;

pub struct Config {
    pub app_id: Option<String>,
    pub app_secret: Option<String>,
    /// 域名, 默认为 https://open.feishu.cn
    pub domain: String,
    /// 客户端超时时间, 单位秒, 默认永不超时
    pub timeout: Option<f32>,
    /// 应用类型, 默认为自建应用; 若设为 ISV 需在 request_option 中配置 tenant_key
    pub app_type: AppType,
    /// 获取 app_access_token 凭证, app_type = ISV 时需配置
    pub app_ticket: Option<String>,
    /// 是否允许手动设置 token, 默认不开启; 开启后需在 request_option 中配置 token
    pub enable_set_token: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: None,
            app_secret: None,
            domain: FEISHU_DOMAIN.to_string(),
            timeout: None,
            app_type: AppType::SELF,
            app_ticket: None,
            enable_set_token: false,
        }
    }
}
