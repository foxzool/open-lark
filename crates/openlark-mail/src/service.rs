use openlark_core::config::Config;
use std::sync::Arc;

/// MailService：邮件服务的统一入口
///
/// 提供对邮件 API v1 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct MailService {
    config: Arc<Config>,
}

impl MailService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn mail(&self) -> crate::mail::mail::Mail {
        crate::mail::mail::Mail::new(self.config.clone())
    }

    #[cfg(feature = "v1")]
    pub fn mailgroup(&self) -> crate::mail::mail::v1::mailgroup::MailGroup {
        crate::mail::mail::v1::mailgroup::MailGroup::new(self.config.clone())
    }
}
