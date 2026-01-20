/// 任务模块测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        assert!(!openlark_task::VERSION.is_empty());
    }

    #[test]
    fn test_prelude_imports() {
        use openlark_task::prelude::*;
        use openlark_core::config::Config;
        use openlark_core::SDKResult;
    }
}
