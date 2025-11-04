# 迁移指南：从旧版 open-lark 升级到模块化架构

本文档帮助您从旧版本的单体架构迁移到新的模块化架构。

## 🚀 新架构概览

新的模块化架构将原来的单一 crate 分解为多个专门的业务域 crate：

### 🏗️ 架构变化

```
旧架构 (v0.13.x):
open-lark/
├── src/
│   ├── core/           # 核心功能
│   ├── service/        # 所有51个服务模块
│   ├── client/         # 主客户端
│   ├── event/          # 事件处理
│   ├── card/           # 卡片组件
│   └── ...
└── Cargo.toml         # 单一依赖配置

新架构 (v0.14.0):
open-lark/                    # 元包 (API兼容层)
├── crates/
│   ├── open-lark-core/           # 核心功能
│   ├── open-lark-communication/  # 通信功能 (im, mail, group)
│   ├── open-lark-docs/            # 文档功能 (cloud-docs, drive, sheets...)
│   ├── open-lark-hr/              # HR功能 (attendance, hire, corehr...)
│   ├── open-lark-management/       # 管理功能 (admin, tenant, directory...)
│   ├── open-lark-advanced/         # 高级功能 (ai, search, calendar...)
│   └── protobuf/                 # Protocol Buffers
└── legacy/                     # 旧代码备份
    ├── src/                    # 原有代码结构
    └── ...
└── Cargo.toml                 # Workspace 配置
```

## 🔄 迁移步骤

### 1. 更新 Cargo.toml

将旧的依赖配置更改为新的模块化配置：

**旧配置:**
```toml
[dependencies]
open-lark = "0.13.2"
```

**新配置:**
```toml
[dependencies]
open-lark = { version = "0.14", features = ["core", "im", "cloud-docs"] }
```

### 2. 更新导入语句

**旧代码:**
```rust
use open_lark::prelude::*;
```

**新代码:**
```rust
// 导入保持不变，内部自动重新导出
use open_lark::prelude::*;
```

### 3. 功能标志变更

某些功能被重新分组到不同的 crate：

| 旧功能 | 新功能 | 所属 crate |
|---------|---------|-----------|
| `im` | `im` | open-lark-communication |
| `mail` | `mail` | open-lark-communication |
| `group` | `group` | open-lark-communication |
| `cloud-docs` | `cloud-docs` | open-lark-docs |
| `drive` | `drive` | open-lark-docs |
| `sheets` | `sheets` | open-lark-docs |
| `wiki` | `wiki` | open-lark-docs |
| `bitable` | `bitable` | open-lark-docs |
| `attendance` | `attendance` | open-lark-hr |
| `hire` | `hire` | open-lark-hr |
| `corehr` | `corehr` | open-lark-hr |
| `ehr` | `ehr` | open-lark-hr |
| `admin` | `admin` | open-lark-management |
| `tenant` | `tenant` | open-lark-management |
| `directory` | `directory` | open-lark-management |
| `address` | `address` | open-lark-management |
| `contact` | 从 management 移至 communication | open-lark-communication |
| `ai` | `ai` | open-lark-advanced |
| `search` | `search` | open-lark-advanced |
| `calendar` | `calendar` | open-lark-advanced |
| `approval` | `approval` | open-lark-advanced |
| `application` | `application` | open-lark-advanced |
| `meeting` | `meeting` | open-lark-advanced |
| `contacts` | `contacts` | open-lark-advanced |

### 4. 代码兼容性

新架构保持 100% 的 API 兼容性：

- **相同的导出**: 所有原有的类型和函数都通过 open-lark 元包重新导出
- **相同的接口**: `LarkClient` 和其他公共接口保持不变
- **类型兼容**: 所有类型定义保持兼容
- **功能名称**: 大部分功能名称保持不变

## 📝 具体迁移示例

### 基础应用

**迁移前:**
```toml
[dependencies]
open-lark = "0.13.2"
```

**迁移后:**
```toml
[dependencies]
open-lark = { version = "0.14", features = ["core", "im"] }
```

代码无需修改，新版本会自动处理重新导出。

### IM 专用应用

**迁移前:**
```toml
[dependencies]
open-lark = { version = "0.13.2", features = ["im"] }
```

**迁移后:**
```toml
[dependencies]
open-lark = { version = "0.14", features = ["core", "im"] }
```

### 文档 + IM 应用

**迁移前:**
```toml
[dependencies]
open-lark = { version = "0.13.2", features = ["im", "cloud-docs"] }
```

**迁移后 (两种方式):**

**方式一 - 使用组合功能:**
```toml
[dependencies]
open-lark = { version = "0.14", features = ["core", "communication-full"] }
```

**方式二 - 使用具体功能:**
```toml
[dependencies]
open-lark = { version = "0.14", features = ["core", "im", "cloud-docs"] }
```

### 企业级应用

**迁移前:**
```toml
[dependencies]
open-lark = { version = "0.13.2", features = ["full"] }
```

**迁移后:**
```toml
[dependencies]
open-lark = { version = "0.14", features = [
    "core",
    "communication-full",  # im, mail, group
    "docs-full",          # cloud-docs, drive, sheets...
    "hr-full",           # attendance, hire, corehr, ehr
    "management-full",      # admin, tenant, directory, address
    "advanced-full"        # ai, search, calendar, meeting, contacts
] }
```

## 🎯 迁移检查清单

### ✅ 编译检查

- [ ] 代码能够无警告编译
- [ ] 所有功能正常工作
- [ ] 测试通过
- [ ] 文档生成正常

### ✅ 功能验证

- [ ] IM 消息发送接收正常
- [ ] 文档上传下载正常
- [ ] 考勤记录查询正常
- [ ] 其他依赖功能正常

### ✅ 性能对比

- [ ] 编译时间对比
- [ ] 二进制大小对比
- [ ] 内存使用对比
- [ ] 运行时性能对比

## 🔧 常见问题和解决方案

### 1. 功能标志错误

**问题**: 编译时提示未知功能标志
**解决方案**: 确保使用新架构中的功能标志名称

### 2. 依赖冲突

**问题**: 多个版本冲突
**解决方案**:
```bash
cargo clean
cargo update
```

### 3. 类型不兼容

**问题**: 类型定义发生变化
**解决方案**: 检查是否使用了内部类型，应该只使用公开 API

### 4. WebSocket 连接问题

**问题**: WebSocket 连接失败
**解决方案**: 确保启用了 `websocket` 功能

## 📊 性能提升预期

- **编译时间**: 减少 30-50%（特别是增量编译）
- **二进制大小**: 根据功能选择，减少 40-70%
- **内存使用**: 减少 10-20%（更好的模块化）
- **启动时间**: 减少 20-30%

## 🆘 新功能优势

### 按需编译

只编译需要的功能模块，减少编译时间和二进制大小：

```toml
# 只有核心功能
open-lark = { version = "0.14", features = ["core"] }

# IM + 文档功能
open-lark = { version = "0.14", features = ["core", "im", "cloud-docs"] }
```

### 模块化测试

新架构支持更好的模块化测试：

```bash
# 只测试特定模块
cargo test --features "core,im"

# 测试特定组合
cargo test --features "communication-full"

# 测试所有功能（较慢）
cargo test --features "full"
```

## 🚨 注意事项

### 破坏性变更

虽然新架构保持了 API 兼容性，但有一些内部变更：

1. **内部结构重组**: 模块内部实现完全重写
2. **依赖关系变化**: 模块间依赖关系发生变化
3. **编译单元变化**: 从单一 crate 变为 workspace

### 兼容性保证

- ✅ **公开 API 不变**: 所有公开接口保持兼容
- ✅ **类型定义不变**: 所有公开类型保持兼容
- ✅ **功能标志不变**: 大部分功能标志名称保持不变
- ✅ **使用方式不变**: 现有代码无需修改即可使用

### 需要更新的事项

- [ ] README 中的功能示例
- [ ] CI/CD 配置
- [ ] 文档中的架构说明
- [ ] 示例代码库
- [ ] 发布脚本

## 📞 获取帮助

如果在迁移过程中遇到问题：

1. **查看文档**:
   - [新架构文档](https://docs.rs/open-lark)
   - [示例代码](examples/)

2. **检查编译错误**:
   ```bash
   cargo check --features "your,features"
   ```

3. **运行测试**:
   ```bash
   cargo test --features "your,features"
   ```

4. **提交 Issue**:
   - [问题反馈](https://github.com/foxzool/open-lark/issues)
   - 包含详细的错误信息和复现步骤

## 🎉 迁移完成

完成迁移后，你将获得：

- ✅ **更好的编译性能**: 显著提升的编译速度
- ✅ **更小的二进制**: 按需编译，减少二进制文件大小
- ✅ **更好的缓存**: 改进的增量编译支持
- ✅ **模块化设计**: 清晰的功能边界和依赖关系
- ✅ **向后兼容**: 无需修改现有代码

---

*这个迁移指南将帮助你顺利升级到新的模块化架构，享受更好的开发体验。*