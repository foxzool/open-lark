# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2025-06-26

### Added - 🛡️ 企业级错误处理系统重构 ⭐

#### 🎯 核心模块新增
- **📊 错误统计和监控系统** (`error_metrics.rs`) - 实时错误分析、智能告警、性能统计
  - 实时错误率计算 (精确到分钟级)
  - 错误分类统计和趋势分析
  - 可配置的智能告警系统 (错误率阈值、严重错误计数)
  - 综合错误报告生成 (支持文件导出)
- **🔄 错误恢复和自动重试中间件** (`retry_middleware.rs`) - 智能重试策略、指数退避算法
  - 基于错误类型的智能重试判断
  - 指数退避算法避免系统过载
  - 重试统计监控和性能指标
  - 可配置的重试条件和回调机制
- **📝 错误日志记录系统** (`error_logger.rs`) - 多格式输出、结构化日志、级别控制
  - 支持简单文本、JSON、结构化三种格式
  - 彩色控制台输出，不同错误级别可视化区分
  - 灵活输出目标 (控制台、文件、多目标)
  - Debug/Info/Warn/Error/Critical 五级日志控制
- **🧠 增强的错误分析系统** (`error_helper.rs`) - 智能错误分析、用户友好消息
  - 基于错误码的智能分析和处理建议
  - 技术错误转换为用户可理解的友好提示
  - 8大维度错误分类管理 (认证、权限、网络等)
  - 具体的修复步骤和最佳实践建议
- **🏷️ 扩展的错误码支持系统** (`error_codes.rs`) - 30+业务错误码、语义化分类
  - 30+ 个业务特定错误码，覆盖飞书生态全域
  - 语义化错误码枚举，从数字到有意义的类型
  - 详细的中文错误描述和解决方案
  - 错误码与重试策略的智能关联
  - 自动关联飞书官方帮助文档链接

#### 🎨 系统架构设计
- **分层错误处理架构** - 用户界面层 → 错误管理层 → 中间件层 → 核心错误层 → 传输层
- **类型安全设计** - 利用 Rust 强类型系统防止错误，统一的 `SDKResult<T>` 类型
- **并发安全保证** - Arc/Mutex 确保多线程环境安全
- **模块化设计** - 每个模块可独立使用和扩展
- **零配置使用** - 开箱即用的合理默认配置

#### 📈 用户体验革命
- **从技术错误到用户友好** - 将 `println!("Error: {:?}", e)` 升级为完整的企业级错误管理
- **智能错误分析** - 自动生成具体的修复建议和操作步骤
- **自动重试机制** - 根据错误类型智能判断是否重试
- **实时错误监控** - 错误率、成功率、性能指标实时统计
- **结构化日志记录** - 便于分析和调试的详细上下文信息

#### 🧪 测试和质量保证
- **24个错误处理相关测试** - 100% 通过率，覆盖所有核心功能
- **5个完整演示程序** - 验证功能完整性和用户体验
- **核心库代码质量检查** - Clippy 检查通过，符合 Rust 最佳实践
- **类型安全保证** - 编译时检查，防止潜在错误

#### 📚 文档和示例
- **ERROR_HANDLING_BEST_PRACTICES.md** (62页) - 完整的开发指导和最佳实践
- **ERROR_HANDLING_FEATURES.md** - 错误处理功能快速上手指南
- **comprehensive_error_codes_demo.rs** - 扩展错误码系统完整演示
- **项目完成报告** - 详细的技术架构和成果总结

#### 🚀 影响和价值
- **从工具到平台** - 将 SDK 从基础工具升级为企业级开发平台
- **开发效率提升** - 快速定位和解决问题，减少调试时间
- **用户体验改善** - 清晰的错误提示和可操作的建议
- **系统可靠性** - 智能重试和自动恢复机制
- **运维友好** - 完整的监控和告警体系

### Enhanced - 核心模块优化

#### 🔧 错误处理核心增强
- **error.rs** - 新增 `Hash` trait 支持，优化错误类型比较
- **error_helper.rs** - 优化字段初始化，使用结构体字面量避免不必要的赋值
- **retry_middleware.rs** - 实现 `Default` trait，优化 API 设计
- **error_logger.rs** - 实现 `Default` trait，提供更简洁的初始化方式
- **error_metrics.rs** - 实现 `Default` trait，修复测试中的字段赋值问题

#### ♻️ 大规模Trait系统重构 - 第六阶段完成
- **全面迁移到owned参数模式** - 完成从 `&Request` 到 `Request` 的全项目迁移
  - 迁移68个文件，涉及IM、考勤、Bitable、Comments、Board、Permission、Search模块
  - 替换所有 `impl_executable_builder!` 为 `impl_executable_builder_owned!`
  - 消除约2000+个不必要的 `.clone()` 调用
  - 更新37个IM方法签名、47个考勤方法、51个Bitable文件
- **性能优化收益** - 减少内存分配，提高请求处理效率，改善API人机工程学
- **代码质量提升** - 统一参数传递模式，更好地遵循Rust所有权原则

#### 📝 文档和开发体验优化
- **官方文档链接集成** - 为所有service API接口添加飞书官方文档链接
  - 更新20个文件，30个API方法的文档链接
  - 统一使用 https://open.feishu.cn 官方文档格式
  - 涉及IM、考勤、云文档、身份验证、搜索等模块
- **代码清理优化** - 清理未使用的导入和警告
  - 删除15个bitable模块文件中未使用的导入
  - 修复编译警告，保持代码库清洁
  - 通过271个单元测试，实现零编译警告

#### 📦 配置和示例更新
- **Cargo.toml** - 新增 `comprehensive_error_codes_demo` 示例配置
- **permission_owned_demo.rs** - 新增演示owned参数模式的示例程序

### Fixed - 问题修复
- 修复 Clippy 警告: `field_reassign_with_default`，优化结构体初始化
- 修复 Clippy 警告: `should_implement_trait`，为多个类型实现 `Default` trait
- 修复 Clippy 警告: `implicit_saturating_sub`，使用 `saturating_sub` 避免溢出
- 修复测试中的生命周期问题，使用 `Arc<AtomicU32>` 替代可变引用
- 修复死代码警告，为未使用的枚举变体添加 `#[allow(dead_code)]`

#### 📅 日历模块 v4 API 支持 - 新增

- **📦 完整的日历服务架构** (`src/service/calendar/`) - 企业级日历管理系统
  - 支持10个子服务模块：日历管理、访问控制、日程管理、会议群、会议纪要等
  - 60个新文件，1355行新代码，完整的模块化设计
  - 集成到 `LarkClient` 主客户端，支持 `client.calendar.v4.*` 访问模式

- **🛠️ 核心API功能实现**
  - **日历管理** (`calendar/`) - 创建、查询、列表日历 (create, get, list)
  - **数据模型** (`models.rs`) - Calendar, CalendarEvent, UserIdType 等核心数据结构
  - **Builder模式** - 支持 `ExecutableBuilder` trait，链式调用和 `.execute()` 方法
  - **标准化响应** - 统一的 `BaseResponse<T>` 格式和错误处理

- **📋 服务模块架构**
  ```
  client.calendar.v4.calendar          // 日历管理 ✅
  client.calendar.v4.calendar_acl      // 访问控制 🔧
  client.calendar.v4.calendar_event    // 日程管理 🔧
  client.calendar.v4.meeting_chat      // 会议群 🔧
  client.calendar.v4.meeting_minute    // 会议纪要 🔧
  client.calendar.v4.timeoff_event     // 请假日程 🔧
  client.calendar.v4.meeting_room_event // 会议室日程 🔧
  client.calendar.v4.attendee          // 参与人管理 🔧
  client.calendar.v4.setting           // 设置 🔧
  client.calendar.v4.exchange_binding  // Exchange绑定 🔧
  ```

- **🎯 使用示例** (`examples/api/calendar_demo.rs`)
  ```rust
  // 创建日历
  let response = CreateCalendarRequest::builder()
      .summary("团队日历")
      .description("团队日程安排")
      .color(1)
      .execute(&client.calendar.v4.calendar)
      .await?;
  
  // 查询日历
  let response = GetCalendarRequest::builder("calendar_id")
      .execute(&client.calendar.v4.calendar)
      .await?;
  
  // 获取列表
  let response = ListCalendarRequest::builder()
      .page_size(20)
      .execute(&client.calendar.v4.calendar)
      .await?;
  ```

- **🔧 技术特性**
  - 支持 Tenant Access Token 和 User Access Token 认证
  - 完整的查询参数支持 (user_id_type, page_size, page_token等)
  - 类型安全的枚举定义 (UserIdType, CalendarType, EventStatus等)
  - 自动的请求体构建和查询参数处理
  - 通过所有 lint 检查，零编译警告

### Technical Details - 技术细节
- **新增依赖**: 无新的外部依赖，基于现有的 `tokio`, `serde`, `reqwest` 等
- **性能优化**: 全异步处理，零阻塞操作，内存效率优化
- **兼容性**: 向后兼容，现有 API 保持不变
- **最低 Rust 版本**: 1.70+ (保持不变)

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
