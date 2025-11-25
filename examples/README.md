# Open-Lark Examples 示例代码库

这是一个全面的Open-Lark SDK示例代码库，采用分层递进式学习路径设计，帮助开发者从入门到专家掌握飞书开放平台API的使用。

## 🏗️ 架构设计

### 分层学习路径

```
examples/
├── 01_getting_started/           # 🟢 入门级 - 基础概念和设置
├── 02_core_features/             # 🟡 核心功能 - P0模块示例
├── 03_business_scenarios/        # 🔵 业务场景 - 跨模块集成
├── 04_advanced_features/         # 🟠 高级功能 - P1-P3模块
└── 05_enterprise_solutions/      # 🔴 企业级解决方案
```

### 支持的功能模块

| 模块            | API数量 | 优先级 | 示例覆盖   |
|---------------|-------|-----|--------|
| communication | 153   | P0  | ✅ 完整   |
| docs          | 254   | P0  | ✅ 完整   |
| security      | 20    | P0  | ✅ 完整   |
| workflow      | 134   | P1  | 🔄 开发中 |
| meeting       | 117   | P1  | 🔄 开发中 |
| hr            | 484   | P2  | 🔄 开发中 |
| 其他模块          | 560+  | P3  | 📋 计划中 |

## 🚀 快速开始

### 环境准备

1. **安装Rust环境**
   ```bash
   rustup update stable
   ```

2. **配置环境变量**

   Open-Lark SDK 现在支持 `.env` 文件和系统环境变量两种配置方式：

   ### 方式一：使用 .env 文件（推荐）
   ```bash
   # 复制配置模板（从根目录）
   cp examples/.env.example .env
   # 编辑配置文件
   nano .env
   ```

   ### 方式二：使用系统环境变量
   ```bash
   # 设置环境变量
   export OPENLARK_APP_ID="your_app_id"
   export OPENLARK_APP_SECRET="your_app_secret"
   ```

   📖 **详细配置指南**：查看 [配置管理文档](docs/configuration.md)

### 运行第一个示例

```bash
# 入门示例 - 客户端设置（支持 .env 文件）
cp examples/.env.example .env
# 编辑 .env 文件，填入您的应用信息
cargo run --example client_setup --features client

# 认证流程示例
cargo run --example authentication --features auth

# 发送第一条消息
cargo run --example send_message --features communication
```

## 🔧 环境变量配置（.env 支持）

### 新特性：分层配置管理

Open-Lark SDK 现在支持智能的 .env 文件管理和分层配置：

#### 配置文件优先级

1. **项目根配置**：`.env` （推荐）
2. **通用示例配置**：`examples/.env`
3. **系统环境变量**：`export OPENLARK_*`

#### 快速配置

```bash
# 从根目录复制配置模板
cp examples/.env.example .env
```

**注意**：只需从 `examples/.env.example` 复制一个配置文件到项目根目录即可。

#### 配置诊断

示例程序现在包含自动配置诊断：

```bash
# 运行时会自动显示配置状态
cargo run --example client_setup --features client

# 输出示例：
# ✅ 已加载配置文件: examples/.env
# ✅ 所有必需的环境变量已正确设置
# ✅ 客户端创建成功（已自动加载配置）
```

详细配置指南请查看：[配置管理文档](docs/configuration.md)

## 📚 学习路径

### 第1层：入门基础 (01_getting_started)

**目标**：了解SDK基础概念，完成环境设置

**包含示例**：

- `client_setup` - 客户端初始化
- `authentication` - 认证流程
- `first_api_call` - 第一个API调用
- `error_handling` - 错误处理

**学习时长**：30分钟

### 第2层：核心功能 (02_core_features)

**目标**：掌握最常用的P0模块功能

**包含模块**：

- **Communication (通讯协作)**
    - 发送和接收消息
    - 用户和群组管理
    - 通讯录操作

- **Docs (文档协作)**
    - 创建和管理文档
    - 多维表格操作
    - 知识库管理

- **Security (安全认证)**
    - 权限验证
    - 令牌管理
    - 访问控制

**学习时长**：2小时

### 第3层：业务场景 (03_business_scenarios)

**目标**：学习真实业务场景的完整解决方案

**包含场景**：

- 员工入职流程
- 会议管理工作流
- 文档协作流程
- 任务审批流程

**学习时长**：3小时

### 第4层：高级功能 (04_advanced_features)

**目标**：掌握复杂模块和高级用法

**包含模块**：

- HR人力资源管理
- 工作流自动化
- AI功能集成
- 数据分析和报表

**学习时长**：4小时

### 第5层：企业解决方案 (05_enterprise_solutions)

**目标**：生产环境的最佳实践和解决方案

**包含内容**：

- 多租户设置
- 性能监控
- 安全最佳实践
- 部署指南

**学习时长**：2小时

## 🔧 功能标志

### 默认配置

```bash
# 默认启用核心功能
cargo run --example send_message  # 自动启用 communication, docs, security
```

### 按需启用

```bash
# 仅启用通讯功能
cargo run --example send_message --features communication

# 启用多个功能模块
cargo run --example complex_workflow --features "communication,docs,hr"

# 启用所有功能
cargo run --example full_demo --all-features
```

### 功能列表

- `communication` - 通讯协作模块 (153 APIs)
- `docs` - 文档协作模块 (254 APIs)
- `security` - 安全认证模块 (20 APIs)
- `workflow` - 工作流模块 (134 APIs)
- `meeting` - 会议模块 (117 APIs)
- `hr` - 人力资源模块 (484 APIs)
- `ai` - AI功能模块 (23 APIs)
- `analytics` - 数据分析模块 (38 APIs)

## 🛠️ 开发工具

### 构建所有示例

```bash
# 构建所有示例
just build-examples

# 运行示例测试
just test-examples

# 检查代码质量
just check-examples
```

### 示例管理

```bash
# 列出所有可用示例
cargo run --bin example_manager list

# 运行特定层级的所有示例
cargo run --bin example_manager run --level 02_core_features

# 验证示例代码质量
cargo run --bin example_manager validate
```

## 📖 文档资源

### API参考

- [完整的API文档](../docs/api-reference.md)
- [模块映射关系](../docs/module-mapping.md)
- [架构设计文档](../ARCHITECTURE.md)

### 最佳实践

- [错误处理指南](docs/error-handling.md)
- [性能优化建议](docs/performance.md)
- [安全最佳实践](docs/security.md)

## 🤝 贡献指南

### 贡献新示例

1. 确定示例所属的层级和模块
2. 遵循代码规范和文档标准
3. 添加必要的测试和文档
4. 提交Pull Request

### 示例规范

- 代码必须有清晰的注释和文档
- 提供完整的使用说明
- 包含错误处理和最佳实践
- 支持功能标志的条件编译

## 🆘 获取帮助

- **GitHub Issues**: [提交问题](https://github.com/open-lark/open-lark/issues)
- **文档**: [在线文档](https://open-lark.github.io/docs/)
- **社区**: [讨论区](https://github.com/open-lark/open-lark/discussions)

## 📄 许可证

本项目遵循 [MIT License](../LICENSE.md)。

---

**开始你的Open-Lark开发之旅吧！** 🚀

如果遇到问题，请查看[故障排除指南](docs/troubleshooting.md)或在社区寻求帮助。