// 导出所有请求/响应结构体
pub use open_lark_core::core::contact::v3::{department::*, group::*, user::*, work_city::*};

/// Models服务
#[derive(Debug)]
pub struct Models {
    config: open_lark_core::core::config::Config,
}

impl Models {
    pub fn new(config: open_lark_core::core::config::Config) -> Self {
        Self { config }
    }
}

impl open_lark_core::core::trait_system::Service for Models {
    fn config(&self) -> &open_lark_core::core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "Models"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    // use super::*; // 暂时注释掉未使用的导入
    // use serde_json; // 暂时注释掉未使用的导入

    #[test]
    fn test_basic_functionality() {
        // 基础功能测试
        // TODO: Add actual test cases for contact models
    }
}
