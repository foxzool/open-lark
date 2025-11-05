# 功能标志迁移指南

**目标用户**: 从旧版本升级的现有开发者
**最后更新**: 2025-11-05

## 🎯 迁移概述

本指南帮助您从旧版本的功能标志迁移到新的统一系统。新的URL路径映射功能标志系统提供了更好的组织结构和向后兼容性。

## 📊 主要变更

### 功能标志重命名和别名

| 旧功能标志 | 新功能标志 | 状态 | 说明 |
|-----------|-----------|------|------|
| `authen` | `auth` | ✅ 别名支持 | `authen` 自动映射到 `auth` |
| `docx` | `cloud-docs` | ✅ 别名支持 | `docx` 自动映射到 `cloud-docs` |
| `drive` | `cloud-docs` | ✅ 别名支持 | `drive` 自动映射到 `cloud-docs` |
| `personal_settings` | `personal-settings` | ✅ 直接重命名 | 使用连字符格式 |
| `speech_to_text` | `speech-to-text` | ✅ 直接重命名 | 使用连字符格式 |
| `optical_char_recognition` | `ocr` | ✅ 直接重命名 | 使用缩写形式 |

### 新增服务支持

新增了15个服务模块的支持，确保所有1551个API都有正确的功能标志映射。

## 🔧 迁移步骤

### 1. 立即可用的别名（零代码变更）

以下功能标志继续工作，无需修改代码：

```toml
# 这些别名自动映射到新的功能标志
authen = ["auth"]           # 映射到 auth
docx = ["cloud-docs"]       # 映射到 cloud-docs
drive = ["cloud-docs"]      # 映射到 cloud-docs
```

```rust
// 现有代码继续工作
let client = LarkClient::builder("app_id", "app_secret")
    .with_feature("authen")  // 自动映射到 auth
    .with_feature("docx")    // 自动映射到 cloud-docs
    .build();
```

### 2. 推荐的逐步迁移

虽然别名提供了向后兼容性，但建议逐步迁移到新的功能标志名称：

#### 步骤 1: 更新 Cargo.toml（可选）

```toml
# 旧配置（仍然有效）
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = ["authen", "docx", "drive"]

# 新配置（推荐）
[dependencies.open-lark]
version = "0.1.0"
default-features = false
features = ["auth", "cloud-docs"]
```

#### 步骤 2: 更新代码（可选）

```rust
// 旧代码（仍然有效）
#[cfg(feature = "authen")]
fn auth_operations() {
    // 使用 client.authen
}

#[cfg(feature = "docx")]
fn document_operations() {
    // 使用 client.docx
}

// 新代码（推荐）
#[cfg(feature = "auth")]
fn auth_operations() {
    // 使用 client.auth
}

#[cfg(feature = "cloud-docs")]
fn document_operations() {
    // 使用 client.cloud_docs
}
```

### 3. 云文档服务统一

所有文档相关功能现在统一在 `cloud-docs` 功能标志下：

```rust
// 所有这些功能标志都提供相同的 cloud_docs 服务
#[cfg(any(feature = "cloud-docs", feature = "docx", feature = "drive"))]
{
    // 文档操作
    let files = client.cloud_docs.v1.drive.file_list(request).await?;
    let doc = client.cloud_docs.v1.docx.create_document(request).await?;
    let comments = client.cloud_docs.v1.comments.add_comment(request).await?;
}
```

## 🚀 新功能优势

### 1. 统一的API路径映射

- **覆盖率**: 100% (1551/1551 APIs)
- **一致性**: 所有API路径遵循 `/open-apis/{service}/{version}/{endpoint}` 映射
- **可预测性**: 服务名称直接对应功能标志

### 2. 改进的错误处理

```rust
// 新的标准化错误处理
match result {
    Ok(response) => println!("✅ 成功: {:?}", response),
    Err(error) => {
        println!("❌ {}", error.user_friendly_message());
        // 提供具体的错误分析和修复建议
    }
}
```

### 3. 共享数据模型

减少代码重复，提供一致的数据结构：

```rust
use open_lark::prelude::*;

// 标准化的类型别名
pub type UserId = String;
pub type TenantId = String;
pub type SDKResult<T> = Result<T, SdkError>;

// 通用的验证工具
let is_valid = utils::validate_user_id(&user_id);
```

## ⚠️ 破坏性变更

### 需要注意的变更

1. **功能标志名称标准化**:
   ```toml
   # 需要更新
   personal_settings → personal-settings
   speech_to_text → speech-to-text
   optical_char_recognition → ocr
   ```

2. **服务访问方式**:
   ```rust
   // 某些服务的访问方式可能改变
   // 请查阅具体服务的文档了解最新用法
   ```

### 兼容性保证

- ✅ 所有别名在当前版本中继续工作
- ✅ 现有代码无需修改即可编译
- ✅ API调用方式保持不变
- ⚠️ 建议在未来版本中迁移到新的功能标志名称

## 🧪 测试迁移

### 验证功能标志

使用内置的验证工具确保迁移成功：

```bash
# 运行功能标志验证
cargo run --bin feature_flag_validator

# 输出示例
# ✅ 总API数量: 1551
# ✅ 有效映射: 1551
# ✅ 成功率: 100.0%
```

### 测试特定功能

```bash
# 测试不同的功能组合
cargo test --no-default-features --features "auth,cloud-docs"
cargo test --no-default-features --features "authen,docx"  # 使用别名
```

## 📋 迁移检查清单

### 迁移前检查

- [ ] 备份当前项目
- [ ] 记录当前使用的功能标志
- [ ] 运行现有测试确保基线正常

### 迁移步骤

- [ ] 更新 Cargo.toml 中的功能标志名称（可选）
- [ ] 更新代码中的条件编译指令（可选）
- [ ] 运行 `cargo run --bin feature_flag_validator` 验证
- [ ] 执行完整测试套件
- [ ] 验证所有功能正常工作

### 迁移后验证

- [ ] 所有测试通过
- [ ] 编译无警告
- [ ] 功能标志验证报告显示100%成功率
- [ ] 应用程序运行正常

## 🆘 常见问题

### Q: 我必须立即迁移吗？
A: 不，所有旧功能标志通过别名继续工作。您可以逐步迁移。

### Q: 如何检查我使用了哪些功能标志？
A: 运行 `cargo run --bin feature_flag_validator` 查看详细的功能映射报告。

### Q: 迁移后包大小会增加吗？
A: 不会，功能标志只是重新组织，不会改变编译后的二进制大小。

### Q: 如何报告迁移问题？
A: 请在 GitHub 仓库创建 issue，包含：
- 使用的功能标志列表
- 错误信息和堆栈跟踪
- 最小可复现示例

## 📞 获取帮助

如果在迁移过程中遇到问题：

1. **查阅文档**: [完整技术规范](docs/feature_flag_mapping_spec.md)
2. **运行验证**: 使用内置验证工具诊断问题
3. **社区支持**: 在 GitHub Discussions 提问
4. **创建 Issue**: 报告具体的技术问题

---

**迁移完成后**，您将获得：
- 🎯 100% API覆盖率
- 🔧 更好的代码组织
- 📚 更清晰的文档
- 🚀 更好的开发体验

*最后更新: 2025-11-05*