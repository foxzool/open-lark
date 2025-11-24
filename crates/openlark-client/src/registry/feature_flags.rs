//! 功能标志管理器
//!
//! 提供动态功能控制、A/B测试和渐进式功能发布功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

/// 功能标志错误
#[derive(Error, Debug, Clone)]
pub enum FeatureFlagError {
    /// 功能标志不存在错误
    ///
    /// 当尝试访问不存在的功能标志时触发
    #[error("功能标志 '{name}' 不存在")]
    FlagNotFound {
        /// 不存在的功能标志名称
        name: String,
    },

    /// 无效功能标志值错误
    ///
    /// 当设置了无效的功能标志值时触发
    #[error("无效的功能标志值: {value}")]
    InvalidValue {
        /// 无效的功能标志值
        value: String,
    },

    /// 功能标志被锁定错误
    ///
    /// 当尝试修改被锁定的功能标志时触发
    #[error("功能标志 '{name}' 已被锁定")]
    FlagLocked {
        /// 被锁定的功能标志名称
        name: String,
    },
}

/// 功能标志结果类型
pub type FeatureResult<T> = Result<T, FeatureFlagError>;

/// 功能标志值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlagValue {
    /// 布尔值
    Bool(bool),
    /// 字符串值
    String(String),
    /// 整数值
    Integer(i64),
    /// 浮点数值
    Float(f64),
    /// 百分比值 (0.0-1.0)
    Percentage(f64),
}

impl FlagValue {
    /// 转换为布尔值
    pub fn as_bool(&self) -> bool {
        match self {
            FlagValue::Bool(value) => *value,
            FlagValue::String(s) => !s.is_empty(),
            FlagValue::Integer(i) => *i != 0,
            FlagValue::Float(f) => *f != 0.0,
            FlagValue::Percentage(p) => *p > 0.0,
        }
    }

    /// 转换为字符串
    pub fn as_string(&self) -> String {
        match self {
            FlagValue::Bool(b) => b.to_string(),
            FlagValue::String(s) => s.clone(),
            FlagValue::Integer(i) => i.to_string(),
            FlagValue::Float(f) => f.to_string(),
            FlagValue::Percentage(p) => format!("{}%", *p * 100.0),
        }
    }

    /// 转换为整数
    pub fn as_integer(&self) -> i64 {
        match self {
            FlagValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            FlagValue::String(_) => 0,
            FlagValue::Integer(i) => *i,
            FlagValue::Float(f) => *f as i64,
            FlagValue::Percentage(p) => (*p * 100.0) as i64,
        }
    }

    /// 转换为浮点数
    pub fn as_float(&self) -> f64 {
        match self {
            FlagValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            FlagValue::String(_) => 0.0,
            FlagValue::Integer(i) => *i as f64,
            FlagValue::Float(f) => *f,
            FlagValue::Percentage(p) => *p,
        }
    }
}

/// 功能标志元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagMetadata {
    /// 标志名称
    pub name: String,
    /// 标志描述
    pub description: Option<String>,
    /// 默认值
    pub default_value: FlagValue,
    /// 当前值
    pub current_value: FlagValue,
    /// 是否锁定（不可修改）
    pub locked: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 标签
    pub tags: Vec<String>,
    /// 所属组
    pub group: Option<String>,
}

/// 功能标志管理器
#[derive(Debug)]
pub struct FeatureFlagManager {
    /// 功能标志存储
    flags: RwLock<HashMap<String, FlagMetadata>>,
    /// 用户分段（用于A/B测试）
    user_segments: RwLock<HashMap<String, UserSegment>>,
}

/// 用户分段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSegment {
    /// 分段名称
    pub name: String,
    /// 分段描述
    pub description: Option<String>,
    /// 分段条件
    pub conditions: Vec<SegmentCondition>,
    /// 权重
    pub weight: f64,
}

/// 分段条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentCondition {
    /// 用户ID匹配
    UserId(String),
    /// 用户ID前缀匹配
    UserIdPrefix(String),
    /// 用户ID后缀匹配
    UserIdSuffix(String),
    /// 邮箱域名匹配
    EmailDomain(String),
    /// 自定义属性
    /// 自定义属性条件
    ///
    /// 根据用户自定义属性进行分段
    CustomAttribute {
        /// 自定义属性键名
        key: String,
        /// 自定义属性值
        value: String,
    },
    /// 随机比例
    RandomPercentage(f64),
}

impl FeatureFlagManager {
    /// 创建新的功能标志管理器
    pub fn new() -> Self {
        Self {
            flags: RwLock::new(HashMap::new()),
            user_segments: RwLock::new(HashMap::new()),
        }
    }

    /// 注册功能标志
    pub fn register_flag(&self, metadata: FlagMetadata) -> FeatureResult<()> {
        let mut flags = self.flags.write().unwrap();
        flags.insert(metadata.name.clone(), metadata);
        Ok(())
    }

    /// 设置功能标志值
    pub fn set_flag(&self, name: &str, value: FlagValue) -> FeatureResult<()> {
        let mut flags = self.flags.write().unwrap();
        let flag = flags
            .get_mut(name)
            .ok_or_else(|| FeatureFlagError::FlagNotFound {
                name: name.to_string(),
            })?;

        if flag.locked {
            return Err(FeatureFlagError::FlagLocked {
                name: name.to_string(),
            });
        }

        flag.current_value = value.clone();
        flag.updated_at = chrono::Utc::now();

        Ok(())
    }

    /// 设置布尔功能标志
    pub fn set_bool_flag(&self, name: &str, value: bool) -> FeatureResult<()> {
        self.set_flag(name, FlagValue::Bool(value))
    }

    /// 设置字符串功能标志
    pub fn set_string_flag(&self, name: &str, value: &str) -> FeatureResult<()> {
        self.set_flag(name, FlagValue::String(value.to_string()))
    }

    /// 设置整数功能标志
    pub fn set_integer_flag(&self, name: &str, value: i64) -> FeatureResult<()> {
        self.set_flag(name, FlagValue::Integer(value))
    }

    /// 设置百分比功能标志
    pub fn set_percentage_flag(&self, name: &str, value: f64) -> FeatureResult<()> {
        if !(0.0..=1.0).contains(&value) {
            return Err(FeatureFlagError::InvalidValue {
                value: value.to_string(),
            });
        }
        self.set_flag(name, FlagValue::Percentage(value))
    }

    /// 获取功能标志值
    pub fn get_flag(&self, name: &str) -> FeatureResult<FlagValue> {
        let flags = self.flags.read().unwrap();
        let flag = flags
            .get(name)
            .ok_or_else(|| FeatureFlagError::FlagNotFound {
                name: name.to_string(),
            })?;

        Ok(flag.current_value.clone())
    }

    /// 检查功能标志是否启用
    pub fn is_enabled(&self, name: &str) -> bool {
        self.get_flag(name).map(|v| v.as_bool()).unwrap_or(false)
    }

    /// 检查功能标志是否对特定用户启用
    pub fn is_enabled_for_user(&self, name: &str, user_id: &str) -> bool {
        let base_value = self.get_flag(name).map(|v| v.as_bool()).unwrap_or(false);

        if !base_value {
            return false;
        }

        // 检查用户分段
        let segments = self.user_segments.read().unwrap();
        for (_, segment) in segments.iter() {
            if self.user_matches_segment(user_id, segment) {
                // 如果用户匹配分段，使用分段的权重决定
                return segment.weight > 0.5; // 简化逻辑
            }
        }

        // 默认行为：使用用户ID哈希来决定
        let hash = self.hash_string(user_id);
        let threshold = self
            .get_flag(name)
            .map(|v| {
                if let FlagValue::Percentage(p) = v {
                    p
                } else if v.as_bool() {
                    1.0
                } else {
                    0.0
                }
            })
            .unwrap_or(1.0);

        (hash % 1000) as f64 / 1000.0 < threshold
    }

    /// 获取所有功能标志
    pub fn list_flags(&self) -> Vec<FlagMetadata> {
        let flags = self.flags.read().unwrap();
        flags.values().cloned().collect()
    }

    /// 获取指定组的功能标志
    pub fn list_flags_by_group(&self, group: &str) -> Vec<FlagMetadata> {
        let flags = self.flags.read().unwrap();
        flags
            .values()
            .filter(|flag| flag.group.as_ref().is_some_and(|g| g == group))
            .cloned()
            .collect()
    }

    /// 锁定功能标志
    pub fn lock_flag(&self, name: &str) -> FeatureResult<()> {
        let mut flags = self.flags.write().unwrap();
        let flag = flags
            .get_mut(name)
            .ok_or_else(|| FeatureFlagError::FlagNotFound {
                name: name.to_string(),
            })?;

        flag.locked = true;
        Ok(())
    }

    /// 解锁功能标志
    pub fn unlock_flag(&self, name: &str) -> FeatureResult<()> {
        let mut flags = self.flags.write().unwrap();
        let flag = flags
            .get_mut(name)
            .ok_or_else(|| FeatureFlagError::FlagNotFound {
                name: name.to_string(),
            })?;

        flag.locked = false;
        Ok(())
    }

    /// 添加用户分段
    pub fn add_user_segment(&self, segment: UserSegment) {
        let mut segments = self.user_segments.write().unwrap();
        segments.insert(segment.name.clone(), segment);
    }

    /// 初始化默认功能标志
    pub fn init_default_flags(&self) {
        let default_flags = vec![
            FlagMetadata {
                name: "communication".to_string(),
                description: Some("通讯服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["core".to_string(), "service".to_string()],
                group: Some("core-layer".to_string()),
            },
            FlagMetadata {
                name: "docs".to_string(),
                description: Some("文档服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["core".to_string(), "service".to_string()],
                group: Some("core-layer".to_string()),
            },
            FlagMetadata {
                name: "auth".to_string(),
                description: Some("认证服务功能".to_string()),
                default_value: FlagValue::Bool(true),
                current_value: FlagValue::Bool(true),
                locked: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec![
                    "core".to_string(),
                    "service".to_string(),
                    "required".to_string(),
                ],
                group: Some("core-layer".to_string()),
            },
            FlagMetadata {
                name: "hr".to_string(),
                description: Some("人力资源服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["professional".to_string(), "service".to_string()],
                group: Some("professional-layer".to_string()),
            },
            FlagMetadata {
                name: "ai".to_string(),
                description: Some("AI服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["professional".to_string(), "service".to_string()],
                group: Some("professional-layer".to_string()),
            },
            FlagMetadata {
                name: "calendar".to_string(),
                description: Some("日历服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["professional".to_string(), "service".to_string()],
                group: Some("professional-layer".to_string()),
            },
            FlagMetadata {
                name: "admin".to_string(),
                description: Some("管理服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["enterprise".to_string(), "service".to_string()],
                group: Some("enterprise-layer".to_string()),
            },
            FlagMetadata {
                name: "approval".to_string(),
                description: Some("审批服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["enterprise".to_string(), "service".to_string()],
                group: Some("enterprise-layer".to_string()),
            },
            FlagMetadata {
                name: "helpdesk".to_string(),
                description: Some("帮助台服务功能".to_string()),
                default_value: FlagValue::Bool(false),
                current_value: FlagValue::Bool(false),
                locked: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                tags: vec!["enterprise".to_string(), "service".to_string()],
                group: Some("enterprise-layer".to_string()),
            },
        ];

        for flag in default_flags {
            let _ = self.register_flag(flag);
        }
    }

    /// 检查用户是否匹配分段
    fn user_matches_segment(&self, user_id: &str, segment: &UserSegment) -> bool {
        segment.conditions.iter().any(|condition| {
            match condition {
                SegmentCondition::UserId(id) => user_id == id,
                SegmentCondition::UserIdPrefix(prefix) => user_id.starts_with(prefix),
                SegmentCondition::UserIdSuffix(suffix) => user_id.ends_with(suffix),
                SegmentCondition::EmailDomain(_) => false, // 需要更多信息
                SegmentCondition::CustomAttribute { .. } => false, // 需要更多信息
                SegmentCondition::RandomPercentage(percentage) => {
                    let hash = self.hash_string(user_id);
                    (hash % 1000) as f64 / 1000.0 < *percentage
                }
            }
        })
    }

    /// 简单字符串哈希
    fn hash_string(&self, s: &str) -> u32 {
        let mut hash = 0u32;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
}

impl Default for FeatureFlagManager {
    fn default() -> Self {
        let manager = Self::new();
        manager.init_default_flags();
        manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_operations() {
        let manager = FeatureFlagManager::new();

        // 注册标志
        let metadata = FlagMetadata {
            name: "test-flag".to_string(),
            description: Some("测试标志".to_string()),
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(false),
            locked: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(metadata).unwrap();

        // 测试获取和设置
        assert!(!manager.is_enabled("test-flag"));
        manager.set_bool_flag("test-flag", true).unwrap();
        assert!(manager.is_enabled("test-flag"));
    }

    #[test]
    fn test_locked_flag() {
        let manager = FeatureFlagManager::new();

        let metadata = FlagMetadata {
            name: "locked-flag".to_string(),
            description: None,
            default_value: FlagValue::Bool(false),
            current_value: FlagValue::Bool(false),
            locked: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: vec![],
            group: None,
        };

        manager.register_flag(metadata).unwrap();

        // 尝试修改锁定标志应该失败
        let result = manager.set_bool_flag("locked-flag", true);
        assert!(matches!(result, Err(FeatureFlagError::FlagLocked { .. })));
    }
}
