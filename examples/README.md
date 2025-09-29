# 飞书SDK示例代码

本目录包含open-lark飞书SDK的各种使用示例，帮助开发者快速上手和掌握不同功能。

## 📁 目录结构

### 🎯 [core/](./core/) - 核心功能示例 (13个)
**最常用的基础API操作，每个示例专注单一功能，代码简洁（通常<100行）**

#### IM消息服务
- `send_message.rs` - 发送文本和富文本消息，包含完整错误处理
- `get_chat_history.rs` - 获取聊天历史记录

#### 云空间服务  
- `upload_file.rs` - 文件上传操作
- `download_file.rs` - 文件下载操作
- `list_folder.rs` - 文件夹内容列表

#### 多维表格服务
- `query_records.rs` - 记录查询和过滤
- `create_record.rs` - 创建新记录

#### 电子表格服务
- `read_write_sheet.rs` - 电子表格读写操作
- `create_sheet.rs` - 创建新电子表格

#### 考勤服务
- `query_attendance.rs` - 查询考勤统计数据
- `process_leave_approval.rs` - 处理请假审批流程

#### 搜索与认证
- `search_user.rs` - 用户搜索和分页查询
- `refresh_token.rs` - 用户身份验证和令牌管理

### 🚀 [api/](./api/) - 完整API功能演示 (64个)
**按服务分类的详细功能演示，包含复杂业务场景和最佳实践**

#### 🏢 企业核心服务
- `hire_v1_example.rs` - 飞书招聘完整功能演示
- `payroll_demo.rs` - 薪资管理系统演示  
- `corehr_demo.rs` - 人事管理系统演示
- `attendance-v1-demo.rs` - 考勤管理系统演示
- `approval_demo.rs` - 审批工作流演示
- `calendar_demo.rs` - 日历管理功能演示
- `task_demo.rs` - 任务管理功能演示
- `okr_example.rs` - OKR目标管理演示
- `performance_example.rs` - 绩效管理演示

#### 📁 云文档生态
- `enhanced_drive_operations.rs` - 云空间高级操作
- `ai_comprehensive.rs` - AI助手功能集成
- `search_v2_comprehensive.rs` - 搜索服务完整功能

#### 👥 组织与通讯录
- `contact_v3_comprehensive.rs` - 通讯录管理完整演示
- `contact_v3_role_management.rs` - 角色权限管理
- `contact_v3_user_management.rs` - 用户管理操作
- `directory_demo.rs` - 组织架构基础操作
- `directory_v1_management.rs` - 组织架构管理
- `directory_v1_department_extended.rs` - 部门扩展操作
- `directory_v1_employee_extended.rs` - 员工扩展操作

#### 💬 即时通讯
- `im_v1_demo.rs` - IM v1消息功能
- `im_v2_demo.rs` - IM v2消息功能  
- `group_demo.rs` - 群组管理
- `bot_demo.rs` - 机器人功能

#### 🔐 认证与安全
- `human_authentication_example.rs` - 实名认证功能
- `security_and_compliance_audit_log.rs` - 安全审计日志
- `security_and_compliance_openapi_log.rs` - OpenAPI日志
- `verification_v1_get.rs` - 身份验证

#### 🎓 学习与协作
- `elearning_v2_management.rs` - 在线学习管理
- `elearning_v2_course_registration.rs` - 课程注册功能
- `lingo_entity.rs` - 飞书词典实体管理
- `lingo_classification_repo.rs` - 词典分类仓库
- `lingo_draft.rs` - 词典草稿功能

#### 🔧 系统集成
- `application_demo.rs` - 应用管理演示
- `admin_demo.rs` - 管理后台功能
- `tenant_v2_query.rs` - 租户查询操作
- `tenant_tag_demo.rs` - 租户标签管理
- `workplace_access_data.rs` - 工作台数据访问
- `workplace_app_recommend.rs` - 应用推荐

#### 📧 通讯与协作
- `mail_demo.rs` - 邮件系统演示
- `minutes_v1.rs` - 会议纪要功能
- `moments_demo.rs` - 朋友圈功能
- `vc_v1.rs` - 视频会议功能

#### 📊 报表与分析
- `report_v1_query_rules.rs` - 报表查询规则
- `report_v1_query_tasks.rs` - 报表查询任务
- `report_v1_remove_rule_view.rs` - 移除规则视图

#### 🛡️ 错误处理与监控
- `enhanced_error_handling.rs` - 增强错误处理演示
- `multi_service_integration_enhanced.rs` - 多服务集成
- `enterprise_scenario_with_enhanced_builder.rs` - 企业场景演示

#### 🔌 其他专项功能
- `acs_example.rs` - 智能门禁系统
- `aily_v1_example.rs` - 智能伙伴功能
- `apass_example.rs` - 低代码平台
- `cardkit_demo.rs` - 卡片组件
- `helpdesk_demo.rs` - 服务台系统
- `mdm_country_region.rs` - 设备管理-国家地区
- `mdm_user_data_relation.rs` - 设备管理-用户数据关联
- `personal_settings_v1_system_status.rs` - 个人设置-系统状态
- `trust_party_organization.rs` - 三方信任-组织
- `trust_party_rules.rs` - 三方信任-规则

### 🔰 [basic/](./basic/) - 基础入门 (2个)
**最简单的入门示例，适合初学者**
- `client_setup.rs` - 客户端基础配置
- `websocket_client.rs` - WebSocket连接与完整IM事件监控示例

### 📊 [benchmarks/](./benchmarks/) - 性能测试 (2个)
**性能基准测试和监控示例**
- `performance_benchmark.rs` - HTTP客户端性能基准测试
- `performance_test.rs` - API响应时间性能测试

### 🔍 [tracing/](./tracing/) - 可观测性 (2个)
**分布式追踪和监控示例**
- `opentelemetry_demo.rs` - OpenTelemetry集成演示
- `service_observability_demo.rs` - 服务可观测性演示

## 🚀 快速开始

### 1. 环境配置
复制 `.env-example` 到 `.env` 并填入您的应用凭据（`.env` 已在 `.gitignore` 中，切勿提交）：
```bash
APP_ID=your_app_id
APP_SECRET=your_app_secret
USER_ACCESS_TOKEN=your_user_access_token  # 某些操作需要
```

常见可选变量（按需）:
```bash
RECEIVE_ID=target_user_open_id_or_chat_id
RUST_LOG=info
SPREADSHEET_TOKEN=...
TABLE_ID=...
FILE_TOKEN=...
FOLDER_TOKEN=...
```

### 2. 运行示例
```bash
# 运行核心功能示例
cargo run --example send_message

# 运行完整API演示
cargo run --example hire_v1_example

# 运行基础入门示例
cargo run --example client_setup
```

### 2.1 按功能启用 features
部分示例依赖特性开关。默认启用的特性为 `im, cloud-docs, contact, group, authentication, search`；
如需运行其它服务示例，请显式启用对应 feature：

```bash
# WebSocket 示例 (包含完整IM事件监控)
cargo run --example websocket_client --features websocket,im

# 招聘（hire）示例
cargo run --example hire_v1_example --features hire

# 全量启用（构建时间较长，不推荐常用）
cargo run --example directory_v1_management --features full
```

特性列表详见项目根目录 `Cargo.toml` 的 `[features]` 段，按需选择能显著提升编译速度与体验。

### 3. 选择适合的示例
- **新手开发者**: 从 `basic/` 和 `core/` 目录开始
- **企业开发**: 重点关注 `api/` 目录中的企业服务
- **特定功能**: 在 `api/` 目录中查找对应服务的演示

## 📋 示例分类指南

### 🎯 按使用场景分类

#### 新手入门路径
1. `basic/client_setup.rs` - 了解基础配置
2. `core/send_message.rs` - 学习基本API调用
3. `core/refresh_token.rs` - 掌握认证机制
4. `api/enhanced_error_handling.rs` - 学习错误处理

#### 企业应用开发
1. `api/hire_v1_example.rs` - 招聘系统集成
2. `api/contact_v3_comprehensive.rs` - 通讯录管理
3. `api/approval_demo.rs` - 审批流程
4. `api/calendar_demo.rs` - 日历集成

#### 云文档协作
1. `core/create_sheet.rs` - 基础表格操作
2. `core/upload_file.rs` - 文件管理
3. `api/enhanced_drive_operations.rs` - 高级云空间操作
4. `api/ai_comprehensive.rs` - AI助手集成

### 🔧 按技术特性分类

#### Builder模式演示
- `api/enterprise_scenario_with_enhanced_builder.rs`
- `core/send_message.rs`
- 大部分 `api/` 目录中的示例

#### 错误处理最佳实践
- `api/enhanced_error_handling.rs`
- `core/send_message.rs`
- `api/multi_service_integration_enhanced.rs`

#### 异步编程模式
- 所有示例都使用 `async/await`
- 特别推荐查看 `api/multi_service_integration_enhanced.rs`

## 💡 代码质量标准

### Core目录标准
- ✅ 单一职责，每个示例专注一个功能
- ✅ 代码简洁，通常在100行以内
- ✅ 完整的错误处理
- ✅ 详细的文档注释

### API目录标准  
- ✅ 功能完整，展示真实业务场景
- ✅ 最佳实践演示
- ✅ 包含复杂错误处理
- ✅ 200-500行为宜，复杂功能可适当增加

### 统一规范
- ✅ 使用 `dotenvy` 加载环境变量
- ✅ 统一错误类型 `Box<dyn std::error::Error>`
- ✅ 启用token缓存功能
- ✅ 包含中文输出和详细说明

## 📚 相关文档

- [飞书开放平台文档](https://open.feishu.cn/document/)
- [项目API文档](../docs/)
- [核心功能使用指南](./core/README.md)
- [项目完整API实现分析](../reports/complete_api_implementation_analysis.md)

## 🔄 维护说明

### 文档清理 (2025-09-28)
- ✅ 移除重复和过时示例 (11个文件清理)
- ✅ 统一命名规范和代码标准
- ✅ 按功能重新分类组织
- ✅ 新增性能测试和可观测性示例目录
- ✅ 更新文档索引和使用指南

### 添加新示例的原则
1. **避免重复**: 检查是否已有类似功能的示例
2. **选择合适目录**: core(基础功能) vs api(完整演示)
3. **遵循规范**: 使用统一的代码模板和错误处理
4. **文档完整**: 包含详细的注释和使用说明

---

**最后更新**: 2025-09-28
**示例总数**: 83个示例 (core: 13, api: 64, basic: 2, benchmarks: 2, tracing: 2)
**覆盖范围**: 42个API服务，100%功能覆盖率
**维护状态**: ✅ 已清理重组，结构优化完成
