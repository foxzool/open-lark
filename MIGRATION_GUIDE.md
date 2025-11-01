# 迁移指南 - 从 0.14.x 到 0.15.0

本文档指导您从 open-lark 0.14.x 版本升级到 0.15.0 版本。0.15.0 版本引入了重大的多crate架构重构，需要一些迁移工作。

## 🏗️ 主要变更

### 多Crate架构重构

**0.15.0 版本最重大的变化是从单体库重构为多crate workspace架构：**

```
open-lark/
├── Cargo.toml (workspace root)
├── open-lark/ (主SDK库)
│   ├── Cargo.toml (依赖 open-lark-core)
│   └── src/
├── open-lark-core/ (核心库)
│   ├── Cargo.toml (独立发布)
│   └── src/
└── scripts/ (验证和工具脚本)
```

### 功能模块化改进

- ✅ **更好的模块化编译** - 只编译您使用的功能模块
- ✅ **减少二进制文件大小** - 通过条件编译优化
- ✅ **更清晰的依赖关系** - 核心功能与应用功能分离
- ✅ **更快的构建时间** - workspace级别的增量编译

### 文档数据透明化

- 📊 **准确的API统计数据** - 1,134个API方法，86.3%覆盖率
- 📋 **透明的实现状态** - 四级分类体系明确展示每个模块状态
- 🎯 **清晰的发展路线图** - 详细的改进计划和优先级指导

## 📋 迁移步骤

### 1. 更新依赖配置

**如果您使用 Cargo.toml：**

```toml
# 旧版本 (0.14.x)
[dependencies]
open-lark = "0.14"

# 新版本 (0.15.0)
[dependencies]
open-lark = "0.15"
```

**如果您需要核心库的特定功能：**

```toml
[dependencies]
open-lark = "0.15"
open-lark-core = "0.15"  # 可选，只有需要直接访问核心功能时才需要
```

### 2. 功能标志更新

**默认功能保持不变：**

```toml
[dependencies]
open-lark = { version = "0.15", features = ["default"] }
# default = ["im", "cloud-docs", "contact", "group", "authentication", "search"]
```

**新增功能标志支持：**

```toml
# 完整功能（较大二进制文件）
open-lark = { version = "0.15", features = ["full"] }

# 自定义功能组合
open-lark = { version = "0.15", features = ["im", "contact", "approval"] }

# 最小化配置
open-lark = { version = "0.15", features = ["im", "authentication"] }
```

### 3. 代码兼容性

**好消息：** 0.15.0 版本保持了 **100% 的API兼容性**！您现有的代码无需修改。

```rust
// 这些代码在 0.14.x 和 0.15.0 中完全相同
use open_lark::prelude::*;

let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .build();

let response = client
    .contact
    .v3
    .user
    .create_user_builder()
    .user(user)
    .user_id_type("open_id")
    .execute(&client.contact.v3.user)
    .await?;
```

### 4. 构建配置更新

**如果您使用 just 构建工具：**

```bash
# 现有的命令保持不变
just build
just test
just docs
just check-all
```

**如果您使用 cargo 命令：**

```bash
# 新增workspace支持
cargo build --workspace  # 构建所有crate
cargo test --workspace   # 测试所有crate
```

## 🔧 常见迁移场景

### 场景1：基本应用迁移

```bash
# 1. 更新版本号
cargo update open-lark --to-version 0.15.0

# 2. 验证编译
cargo check

# 3. 运行测试
cargo test

# 4. 一切正常！无需代码修改
```

### 场景2：自定义功能配置

```bash
# 1. 检查当前使用的功能
cargo tree | grep open-lark

# 2. 更新Cargo.toml中的功能配置
# 确保包含您需要的所有功能标志

# 3. 验证特定功能编译
cargo check --features your-custom-features
```

### 场景3：大型项目迁移

```bash
# 1. 创建新的分支
git checkout -b upgrade-to-0.15.0

# 2. 更新依赖
# 更新所有workspace成员的Cargo.toml文件

# 3. 渐进式测试
# 先测试基础功能，再测试复杂功能

# 4. 性能验证
# 比较构建时间和二进制文件大小
```

## 📊 性能改进

### 构建时间优化

- **增量编译** - workspace级别的依赖缓存
- **并行构建** - 多crate并行编译
- **条件编译** - 只编译使用的功能模块

### 二进制文件大小

通过功能标志优化，您可以显著减少最终的二进制文件大小：

```bash
# 完整功能：~50MB
cargo build --release --all-features

# 基本功能：~15MB
cargo build --release --features="im,contact,authentication"

# 最小配置：~8MB
cargo build --release --features="im"
```

## ⚠️ 注意事项

### 1. 功能标志必需

在 0.15.0 中，某些功能现在需要明确的功能标志：

```rust
// 确保在 Cargo.toml 中启用所需功能
[dependencies]
open-lark = { version = "0.15", features = ["im", "contact"] }
```

### 2. 文档更新

查看更新后的文档以了解准确的API覆盖率和实现状态：

- [API覆盖率报告](docs/API_COVERAGE_REPORT.md)
- [待实现模块清单](docs/PENDING_MODULES.md)
- [项目文档更新摘要](docs/DOCUMENTATION_UPDATE_SUMMARY.md)

### 3. 错误处理改进

如果您之前使用了错误处理的内部API，可能需要更新：

```rust
// 0.14.x
use open_lark::core::error::LarkAPIError;

// 0.15.0 (推荐)
use open_lark::prelude::*;  // 统一的导出
```

## 🆘 故障排除

### 编译错误

**问题：** `could not find 'v1' in 'im'`

**解决方案：** 确保启用了相应的功能标志

```toml
[dependencies]
open-lark = { version = "0.15", features = ["im"] }
```

**问题：** 功能找不到

**解决方案：** 检查功能标志配置

```bash
# 查看可用功能
cargo search open-lark

# 检查当前配置
cargo tree | grep open-lark
```

### 运行时错误

**问题：** 某些API调用失败

**解决方案：** 检查API实现状态

```bash
# 运行验证脚本
./scripts/verify_api_stats.sh

# 查看待实现模块
cat docs/PENDING_MODULES.md
```

## 📞 获取帮助

如果在迁移过程中遇到问题：

1. **查看文档** - [完整的文档更新摘要](docs/DOCUMENTATION_UPDATE_SUMMARY.md)
2. **运行验证脚本** - `./scripts/verify_api_stats.sh`
3. **检查GitHub Issues** - [项目Issues页面](https://github.com/foxzool/open-lark/issues)
4. **创建新Issue** - 详细描述您的迁移问题

## 🎯 迁移检查清单

- [ ] 更新 Cargo.toml 中的版本号
- [ ] 确认所需的功能标志已启用
- [ ] 运行 `cargo check` 验证编译
- [ ] 运行 `cargo test` 验证测试
- [ ] 检查API覆盖率报告了解功能状态
- [ ] 验证构建时间有所改善
- [ ] 确认二进制文件大小符合预期

---

**恭喜！** 完成这些步骤后，您就成功迁移到了 open-lark 0.15.0 版本，可以享受多crate架构带来的所有改进了！