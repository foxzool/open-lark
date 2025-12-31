//! 版本管理模块
//!
//! 提供文档服务的版本管理功能。

/// 版本信息结构
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// 主版本号
    pub major: u32,
    /// 次版本号
    pub minor: u32,
    /// 修订版本号
    pub patch: u32,
    /// 版本标签
    pub label: String,
}

impl VersionInfo {
    /// 创建新的版本信息
    pub fn new(major: u32, minor: u32, patch: u32, label: impl Into<String>) -> Self {
        Self {
            major,
            minor,
            patch,
            label: label.into(),
        }
    }

    /// 获取版本字符串
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// 获取完整版本字符串
    pub fn full_version_string(&self) -> String {
        format!("{}-{}", self.version_string(), self.label)
    }
}

/// 获取当前版本信息
pub fn get_current_version() -> VersionInfo {
    VersionInfo::new(0, 15, 0, "dev")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let version = VersionInfo::new(1, 2, 3, "stable");
        assert_eq!(version.version_string(), "1.2.3");
        assert_eq!(version.full_version_string(), "1.2.3-stable");
    }

    #[test]
    fn test_current_version() {
        let current = get_current_version();
        assert_eq!(current.major, 0);
        assert_eq!(current.minor, 15);
        assert_eq!(current.patch, 0);
        assert_eq!(current.label, "dev");
    }
}
