# Open-Lark SDK 配置管理指南

本文档介绍如何在 Open-Lark SDK 示例中使用环境变量和 .env 文件进行配置管理。

## 📋 目录

- [快速开始](#快速开始)
- [配置文件系统](#配置文件系统)
- [环境变量说明](#环境变量说明)
- [配置加载优先级](#配置加载优先级)
- [配置诊断](#配置诊断)
- [最佳实践](#最佳实践)
- [故障排除](#故障排除)

## 🚀 快速开始

### 1. 基础配置

最简单的配置方式是使用 .env 文件：

```bash
# 复制配置模板
cp examples/.env.example .env

# 编辑配置文件
nano .env
```

### 2. 运行示例

```bash
# 基础客户端示例
cargo run --example client_setup --features client

# 通讯功能示例
cargo run --example send_message --features communication
```

## 📁 配置文件系统

Open-Lark SDK 支持分层配置管理，按优先级自动加载配置：

### 配置文件搜索顺序

1. **特定示例配置**：`examples/01_getting_started/.env`
2. **通用示例配置**：`examples/.env`
3. **项目根配置**：`.env`
4. **系统环境变量**

### 配置文件模板

#### 通用配置 (`examples/.env.example`)

```bash
# 飞书应用配置 (必需)
OPENLARK_APP_ID="your_app_id_here"
OPENLARK_APP_SECRET="your_app_secret_here"

# 可选配置
OPENLARK_BASE_URL="https://open.feishu.cn"
OPENLARK_TIMEOUT=30
OPENLARK_RETRY_COUNT=3
```

#### 入门示例配置 (`examples/01_getting_started/.env.example`)

```bash
# 最简配置，适合入门学习
OPENLARK_APP_ID="your_app_id_here"
OPENLARK_APP_SECRET="your_app_secret_here"

# 学习模式配置
OPENLARK_DEBUG=false
OPENLARK_VERBOSE_OUTPUT=true
```

## 🔧 环境变量说明

### 必需环境变量

| 变量名 | 描述 | 示例值 |
|--------|------|--------|
| `OPENLARK_APP_ID` | 飞书应用ID | `cli_a1b2c3d4e5f6g7h8` |
| `OPENLARK_APP_SECRET` | 飞书应用密钥 | `your_app_secret_here` |

### 可选环境变量

| 变量名 | 描述 | 默认值 | 示例值 |
|--------|------|--------|--------|
| `OPENLARK_BASE_URL` | API基础URL | `https://open.feishu.cn` | `https://open.larksuite.com` |
| `OPENLARK_TIMEOUT` | 请求超时时间(秒) | `30` | `60` |
| `OPENLARK_RETRY_COUNT` | 重试次数 | `3` | `5` |
| `OPENLARK_DEBUG` | 调试模式 | `false` | `true` |
| `OPENLARK_TRACE_REQUESTS` | 请求跟踪 | `false` | `true` |

## 📊 配置加载优先级

配置按以下优先级加载，后加载的配置会覆盖先加载的：

```
高优先级
    ↓
1. 系统环境变量
2. examples/.env (通用配置)
3. examples/01_getting_started/.env (示例特定配置)
    ↓
低优先级
```

### 配置加载示例

```rust
use openlark_examples_common::{load_config_with_diagnostics};

// 自动搜索并加载配置
let result = load_config_with_diagnostics(&["01_getting_started", "examples", "."]);
result.print_result();
```

输出示例：
```
✅ 已加载配置文件: examples/01_getting_started/.env
✅ 配置完整性检查通过
```

## 🔍 配置诊断

### 自动诊断

SDK 提供自动配置诊断功能：

```rust
use openlark_examples_common::{run_config_diagnostics};

let diagnostics = run_config_diagnostics(true);
diagnostics.print_diagnostics();
```

### 增强环境变量检查

```rust
use openlark_examples_common::{check_env_vars_enhanced};

let required_vars = ["OPENLARK_APP_ID", "OPENLARK_APP_SECRET"];
let search_dirs = ["01_getting_started", "examples"];

match check_env_vars_enhanced(&required_vars, &search_dirs, true) {
    Ok(result) => {
        result.print_detailed_result();
        if !result.is_complete() {
            // 显示修复建议
            for suggestion in result.generate_fix_suggestions() {
                println!("💡 {}", suggestion);
            }
        }
    }
    Err(e) => {
        println!("❌ 配置检查失败: {}", e);
    }
}
```

## 💡 最佳实践

### 1. 安全配置

```bash
# ✅ 正确：使用 .env 文件
echo "OPENLARK_APP_ID=your_app_id" > .env

# ✅ 正确：设置适当的文件权限
chmod 600 .env

# ❌ 错误：在代码中硬编码
let app_id = "your_hardcoded_app_id";
```

### 2. 环境分离

```bash
# 开发环境
cp examples/.env.example .env.development
echo "OPENLARK_DEBUG=true" >> .env.development

# 生产环境
export OPENLARK_APP_ID="prod_app_id"
export OPENLARK_APP_SECRET="prod_app_secret"
```

### 3. 配置验证

在应用启动时验证配置：

```rust
use openlark_examples_common::{create_client_with_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_dirs = ["01_getting_started", "examples", "."];
    let client = create_client_with_config(&search_dirs)?;

    println!("✅ 客户端创建成功，配置验证通过");
    // 继续应用逻辑...

    Ok(())
}
```

### 4. 团队协作

在 `.gitignore` 中包含：

```gitignore
# 环境变量文件
.env
.env.local
.env.*.local

# 敏感配置
*.key
*.pem
secrets/
```

## 🛠️ 故障排除

### 常见问题

#### 1. 配置文件未找到

**错误信息**：
```
ℹ️  未找到 .env 文件，使用系统环境变量
⚠️  环境变量检查不完整: 2 个缺失
```

**解决方案**：
```bash
# 复制配置模板
cp examples/01_getting_started/.env.example .env

# 编辑配置
nano .env
```

#### 2. 应用ID无效

**错误信息**：
```
❌ 客户端配置验证失败: Invalid configuration
可能的原因:
  1. 应用ID或密钥不正确或无效
```

**解决方案**：
1. 检查飞书开放平台中的应用ID和密钥
2. 确认应用状态为"已发布"
3. 验证应用权限配置

#### 3. 网络连接问题

**错误信息**：
```
❌ 客户端配置验证失败: Network error
```

**解决方案**：
```bash
# 测试网络连接
curl -I https://open.feishu.cn

# 检查代理设置
echo $HTTP_PROXY
echo $HTTPS_PROXY
```

### 调试技巧

#### 1. 启用详细日志

```bash
# 设置调试模式
export OPENLARK_DEBUG=true
export OPENLARK_TRACE_REQUESTS=true

# 运行示例
cargo run --example client_setup --features client
```

#### 2. 配置验证工具

```rust
// 在示例代码中添加配置验证
use openlark_examples_common::*;

fn debug_config() {
    let diagnostics = run_config_diagnostics(false);
    diagnostics.print_diagnostics();

    let result = check_env_vars_enhanced(
        &["OPENLARK_APP_ID", "OPENLARK_APP_SECRET"],
        &["examples"],
        true
    );

    if let Ok(result) = {
        result.print_detailed_result();
        for suggestion in result.generate_fix_suggestions() {
            println!("修复建议: {}", suggestion);
        }
    }
}
```

#### 3. 配置文件生成

```bash
# 生成标准配置模板
cargo run --example client_setup --features client \
  -- --generate-config my-config.env
```

## 📚 相关文档

- [飞书开放平台文档](https://open.feishu.cn/document/)
- [Open-Lark SDK 主文档](../../../README.md)
- [示例代码说明](../README.md)
- [认证服务文档](../01_getting_started/authentication.md)

## 🤝 贡献

如果您发现配置管理相关的问题或有改进建议，欢迎：

1. 提交 Issue 报告问题
2. 提交 Pull Request 改进配置管理
3. 分享您的配置最佳实践

---

*最后更新：2025年11月25日*