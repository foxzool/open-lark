# 入门基础示例

本层级的示例旨在帮助初学者了解Open-Lark SDK的基础概念，完成环境设置，并成功执行第一个API调用。

## 📚 学习目标

完成本层级学习后，你将能够：
- ✅ 理解Open-Lark SDK的基本架构
- ✅ 成功配置和初始化客户端
- ✅ 掌握飞书开放平台的认证流程
- ✅ 成功调用第一个飞书API
- ✅ 了解错误处理的基本方法

## 🎯 示例列表

| 示例 | 描述 | 预计时间 | 前置知识 |
|------|------|----------|----------|
| `client_setup` | 客户端初始化和配置 | 5分钟 | 基础Rust知识 |
| `authentication` | 认证流程详解 | 10分钟 | 了解飞书开放平台 |
| `first_api_call` | 第一个API调用 | 10分钟 | 前两个示例 |
| `error_handling` | 错误处理最佳实践 | 15分钟 | 前三个示例 |

## 🚀 开始之前

### 1. 环境要求
- Rust 1.70+
- 飞书开放平台开发者账号

### 2. 配置环境变量（支持 .env 文件）

#### 方式一：使用 .env 文件（推荐）
```bash
# 从根目录复制配置模板
cp examples/.env.example .env

# 编辑配置文件
nano .env
# 填入您的飞书应用信息：
# OPENLARK_APP_ID="cli_a1b2c3d4e5f6g7h8"
# OPENLARK_APP_SECRET="your_app_secret_here"
```

#### 方式二：使用系统环境变量
```bash
# 设置环境变量
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"

# 验证配置
echo $OPENLARK_APP_ID
echo $OPENLARK_APP_SECRET
```

### 3. 飞书应用设置
1. 登录[飞书开放平台](https://open.feishu.cn/)
2. 创建应用并获取App ID和App Secret
3. 配置应用权限（根据需要）

## 📖 学习路径

### 步骤1: 客户端设置
```bash
cargo run --example client_setup
```
学习如何初始化SDK客户端，理解配置选项。

### 步骤2: 认证流程
```bash
cargo run --example authentication
```
掌握应用认证和用户认证的流程。

### 步骤3: 第一个API调用
```bash
cargo run --example first_api_call
```
成功调用你的第一个飞书API。

### 步骤4: 错误处理
```bash
cargo run --example error_handling
```
学习如何优雅地处理API调用中的错误。

## 🔍 核心概念

### 客户端 (Client)
Open-Lark SDK的核心组件，负责：
- 管理HTTP连接
- 处理认证令牌
- 提供API访问接口

### 认证 (Authentication)
飞书开放平台支持多种认证方式：
- **应用认证**：使用App ID和App Secret
- **用户认证**：需要用户授权
- **OAuth认证**：标准OAuth 2.0流程

### API调用 (API Call)
所有API调用都遵循统一模式：
```rust
let result = client.service.module.api_name().await?;
```

## ❓ 常见问题

### Q: 如何获取飞书应用的App ID和App Secret？
A: 访问[飞书开放平台](https://open.feishu.cn/)，创建应用后在应用详情页面查看。

### Q: 示例运行失败怎么办？
A: 检查以下几点：
1. 环境变量是否正确设置
2. 网络连接是否正常
3. 飞书应用权限是否足够

### Q: 如何切换到不同的飞书环境？
A: 在客户端配置中设置不同的`base_url`。

## 📚 下一步

完成入门基础学习后，建议进入：
- **[核心功能层](../02_core_features/)**：学习P0模块的核心功能
- **[API文档](../../docs/api-reference.md)**：查看完整的API参考

## 💡 提示

- 所有示例都包含详细的错误处理
- 建议按照顺序完成示例学习
- 遇到问题时查看日志输出
- 可以使用测试环境进行实验

---

**开始你的Open-Lark之旅吧！** 🚀