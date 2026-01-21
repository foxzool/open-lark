use openlark_core::config::Config;
use std::sync::Arc;

/// TaskService：任务服务的统一入口
///
/// 提供对任务 API v1 和 v2 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct TaskService {
    config: Arc<Config>,
}

impl TaskService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::v1::TaskV1 {
        crate::v1::TaskV1::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn v2(&self) -> crate::v2::TaskV2 {
        crate::v2::TaskV2::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn task(&self) -> crate::v2::task::Task {
        crate::v2::task::Task::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    pub fn tasklist(&self) -> crate::v2::tasklist::Tasklist {
        crate::v2::tasklist::Tasklist::new(self.config.clone())
    }
}
