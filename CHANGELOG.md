# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2025-06-20

### Added - 完整考勤模块实现 🎉
- **考勤班次管理** (5个API) - 班次的创建、删除、查询、修改
- **考勤排版管理** (3个API) - 排班表管理和临时排班
- **考勤组管理** (6个API) - 考勤组的完整CRUD操作
- **考勤用户设置** (4个API) - 人脸识别设置和照片管理
- **考勤统计** (4个API) - 统计数据查询和设置管理
- **假勤审批** (3个API) - 审批流程和状态管理
- **考勤补卡** (3个API) - 补卡申请和记录查询
- **归档报表** (4个API) - 报表数据管理和归档规则
- **打卡信息管理** (5个API) - 打卡流水的完整生命周期管理
- **休假管理** (2个API) - 休假发放记录和过期记录查询
- **考勤事件** (2个事件处理器) - 实时考勤事件监听和处理

### Enhanced - 技术特性
- **完整性** - 41个API + 2个事件处理器，覆盖飞书考勤API的所有功能
- **类型安全** - Rust强类型系统保证编译时安全
- **模块化设计** - 清晰的模块组织和服务分离
- **示例代码** - 43个完整示例，每个功能都有详细的中文注释和使用演示
- **错误处理** - 完善的错误提示和处理逻辑，支持异步并发操作

### Changed
- 重构考勤模块架构，采用统一的服务设计模式
- 优化数据模型定义，支持复杂的业务场景和状态管理
- 改进事件处理机制，支持实时考勤数据监听

### Technical
- 添加完整的考勤模块文档和开发计划
- 实现标准化的请求/响应处理模式
- 支持分页查询、批量操作和状态枚举
- 集成时间处理和格式化功能

## [0.4.1] - 2025-01-21

### Added
- 新增详细的 SDK 完成进度评估报告 (`docs/sdk-completion-assessment.md`)
- 添加完整的 `prelude` 模块，导出常用类型和 trait
- 新增项目基础设施文件：GitHub 工作流、Issue 模板、PR 模板、安全政策等

### Changed
- 重构 `FeishuCard::header()` 和 `FeishuCard::elements()` 方法返回 `Result` 类型
- 改进事件注册器方法返回 `Result` 避免重复注册时的 panic
- 优化事件分发器错误处理，移除 panic 改为返回错误和记录警告
- 修复 WebSocket 示例中的异步运行时反模式问题
- 增强 `justfile` 构建脚本，添加完整的发布流程

### Removed
- 删除空的 `src/card/color.rs` 文件

### Fixed
- 修复所有卡片示例以正确处理新的 `Result` 返回类型
- 改善库代码的健壮性，消除潜在的运行时崩溃

## [0.4.0] - 2025-01-20

### Added
- 新增 `ApiError` 错误类型，包含 code、message 和 request_id 上下文信息
- 增加便利方法 `LarkAPIError::api_error()` 和 `LarkAPIError::illegal_param()`
- 改进 CLAUDE.md 文档，增加详细的测试命令和使用模式

### Changed
- 优化错误处理，提供更丰富的调试信息
- 改进 WebSocket 示例代码，移除冗余的全局客户端
- 统一错误处理模式，使用新的便利方法
- 标准化代码导入格式
- 增强事件分发器的调试日志

### Improved
- 更好的错误上下文信息，便于调试和监控
- 更清晰的示例代码结构
- 提升开发者体验和代码质量
- 基于深度架构分析的优化

### Technical
- 基于 Zen 架构分析工具的全面代码审查
- 优化错误类型定义，增强错误信息的可读性
- 改进 WebSocket 客户端示例的最佳实践

## [0.3.6]

- 添加多维表格类型：查找引用、流程、按钮

## [0.3.5]

### Added

- 添加一些辅助方法

## [0.3.4]

### Added

- 飞书多维表格更新记录

### Removed

- 去除无用的crate

## [0.3.3]

### Added

- 飞书多维表格添加记录

## [0.3.2] - 2024-08-22

### Changed

- reqwest default use rust-tls
- Update dependencies: quick_cache, prost
