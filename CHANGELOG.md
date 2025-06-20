# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
