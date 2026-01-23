# openlark-docs

飞书开放平台云文档服务模块 - 文档、表格、知识库 API（202 APIs，100% 覆盖，不含旧版本）

## 功能概览

| 模块 | API 数量 | 描述 |
|------|---------|------|
| CCM | 174 | 云文档协同（文档、云盘、表格、知识库） |
| Bitable | 49 | 多维表格 |
| Base | 0 | 基础服务 |
| Baike | 27 | 知识库 |
| Minutes | 4 | 会议纪要 |
| **总计** | **202** | **100% 覆盖** |

## 特性

- ✅ **完整覆盖**: 202 个 API，100% 实现覆盖率（排除 old 版本）
- ✅ **类型安全**: 基于 enum 的端点系统，编译时检查
- ✅ **流式 API**: Builder 模式，链式调用
- ✅ **统一入口**: `DocsClient` 作为唯一公开入口
- ✅ **按需编译**: 细粒度 feature，减少编译时间和二进制体积

## 功能选择（Feature Selection）

### 使用方式

在 `Cargo.toml` 中添加所需的 features：

```toml
[dependencies]
open-lark = { version = "0.15", features = ["your-features"] }
```

### 可用功能（Features）

#### 核心功能

| Feature | 说明 | 依赖 |
|---------|------|------|
| `core` | 核心功能 | 无 |

#### 云文档协同（CCM）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `ccm` | 完整云文档协同功能 | 174 APIs |
| `ccm-doc` | 文档处理 | 文档创建、获取、更新、删除 |
| `ccm-docx` | DOCX 文档 | DOCX 文档处理 |
| `ccm-drive` | 云盘文件 | 文件上传、下载、删除 |
| `ccm-sheets` | 表格（默认 v3） | 电子表格完整功能 |
| `ccm-sheets-v3` | 表格 v3（推荐） | 稳定版本，现代化 API |
| `ccm-sheets-v2` | 表格 v2（实验性） | 基础表格操作（已弃用） |
| `ccm-wiki` | 知识库 | Wiki 空间、页面管理 |

#### 多维表格（Bitable）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `bitable` | 多维表格 | 49 APIs |
| `base` | 基础服务 | 基础服务支持 |

#### 知识库（Baike）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `baike` | 知识库 | 27 APIs |
| `wiki` | Wiki | Wiki 空间、页面 |

#### 语言服务（Lingo）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `lingo` | 飞书词典 | 词条高亮、实体提取 |

#### 会议纪要（Minutes）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `minutes` | 会议纪要 | 4 APIs |

#### 云文档管理（Docs）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `docs` | 云文档管理 | 文档管理 |

#### 公告管理（Docx）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `docx` | 公告管理 | 公告发布、查询 |

#### 版本支持

| Feature | 说明 |
|---------|------|
| `v1` | API v1 版本 |
| `v2` | API v2 版本 |
| `v3` | API v3 版本 |

#### 功能组合

| Feature | 说明 | 包含的模块 |
|---------|------|-----------|
| `full` | 完整云文档功能 | ccm, bitable, base, baike, minutes, v3 |

#### 向后兼容别名

| Feature | 说明 |
|---------|------|
| `cloud-docs` | 云文档协同（别名） |
| `all-cloud-docs` | 所有云文档功能（别名） |

## 快速开始

### 基础使用

```rust
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 1. 创建客户端
    let client = LarkClient::builder("app_id", "app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    // 2. 使用 DocsClient 访问云文档功能
    let doc = client.docs.ccm_doc.get_document("doc_token").await?;
    println!("Document: {:?}", doc);

    Ok(())
}
```

### 多维表格使用

```toml
# Cargo.toml
[dependencies]
open-lark = { version = "0.15", features = ["bitable"] }
```

```rust
use open_lark::prelude::*;

let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .build();

// 访问多维表格
let app = client.docs.base.bitable.create_app(&CreateAppRequest { ... }).await?;
```

### 云文档协同使用

```toml
# Cargo.toml
[dependencies]
open-lark = { version = "0.15", features = ["ccm"] }
```

```rust
use open_lark::prelude::*;

let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .build();

// 访问云文档协同
let file = client.docs.ccm.drive.get_file("file_token").await?;
let sheet = client.docs.ccm.sheets.get_sheet("spreadsheet_token", "sheet_id").await?;
```

### 知识库使用

```toml
# Cargo.toml
[dependencies]
open-lark = { version = "0.15", features = ["baike"] }
```

```rust
use open_lark::prelude::*;

let client = LarkClient::builder("app_id", "app_secret")
    .with_app_type(AppType::SelfBuild)
    .build();

// 访问知识库
let entity = client.docs.baike.create_entity(&CreateEntityRequest { ... }).await?;
```

## 性能优化建议

### 1. 选择性启用功能

仅启用您需要的功能模块：

```toml
# ❌ 不推荐：启用所有功能
open-lark = { version = "0.15", features = ["full"] }

# ✅ 推荐：仅启用所需功能
open-lark = { version = "0.15", features = ["bitable"] }  # 仅多维表格
```

### 2. 编译优化

```bash
# 发布版本使用更优化的编译配置
cargo build --release

# 检查优化后的二进制大小
ls -lh target/release/
```

### 3. 使用流式 Builder 模式

```rust
// ✅ 推荐：流式构建器
let request = CreateDocRequest::new(config, doc_body)
    .user_id_type(UserIdType::OpenId)
    .folder_token("folder_token");

// ❌ 不推荐：多次对象创建
let mut request = CreateDocRequest::new(config, doc_body);
request = request.user_id_type(UserIdType::OpenId);
request = request.folder_token("folder_token");
```

## 文档

- [飞书开放平台文档](https://open.feishu.cn/document/home/)
- [API 文档](https://open.feishu.cn/document/server-docs/)
- [SDK 源码](https://github.com/your-org/open-lark)

## 许可证

[项目许可证](LICENSE)

## 贡献

欢迎贡献！请参阅 [贡献指南](CONTRIBUTING.md)。

## 支持与反馈

- 问题反馈：[GitHub Issues](https://github.com/your-org/open-lark/issues)
- 功能请求：[GitHub Discussions](https://github.com/your-org/open-lark/discussions)
