//! Report Rule 报告规则API模块
//!
//! 提供报告规则管理相关的功能，包括：
//! - 规则查询和过滤
//! - 规则删除和清理
//! - 规则状态管理
//! - 完整的错误处理和参数验证
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::report::{ReportService, QueryRuleRequest};
//!
//! let service = ReportService::new(config);
//!
//! // 查询报告规则
//! let response = service
//!     .rule()
//!     .query_rule_builder()
//!     .page_size(20)
//!     .page_token("token")
//!     .execute(&service.rule())
//!     .await?;
//!
//! if let Some(rules) = response.data {
//!     println!("查询到规则数量: {}", rules.total);
//!     for rule in rules.items {
//!         println!("规则名称: {}", rule.name);
//!     }
//! }
//! ```

pub mod models;
pub mod services;

// 重新导出主要类型
pub use services::{QueryRuleRequestBuilder, RemoveRuleViewRequestBuilder, RuleService};
