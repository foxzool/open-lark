# AGENTS.md (openlark-docs)

## OVERVIEW
飞书云文档（CCM）与百科（Baike）服务实现。涵盖 533 个 API（pub async fn），455 个源码文件。核心特征是多版本（v1-v4）共存与严格的资源路径映射。

## STRUCTURE
- **CCM (Cloud Docs Management)**
  - `sheets/`: v2 (33 APIs, 已弃用) / v3 (27 APIs, 推荐)
  - `drive/`: v1 (59 APIs, 基础/权限) / v2 (增强功能)
  - `wiki/`: v2 (16 APIs, 空间/节点管理)
  - `bitable/`: v1 (46 APIs, 多维表格核心)
- **Baike & Lingo**
  - `baike/`: v1 (13 APIs) 词条/草稿/分类
  - `lingo/`: 词典系统（共享百科 CRUD 模式）

## WHERE TO LOOK
- **多维表格记录**: `src/base/bitable/v1/app/table/record/`
- **电子表格（最新）**: `src/ccm/sheets/v3/`
- **云盘权限控制**: `src/ccm/drive/v1/permission/`
- **百科词条管理**: `src/baike/v1/entity/`
- **Endpoint 定义**: `src/common/api_endpoints.rs` (基于 Enum 的类型安全映射)

## CONVENTIONS
- **路径规范**: `src/{bizTag}/{project}/{version}/{resource}/{name}.rs`
  - 示例: `src/base/bitable/v1/app/table/record/create.rs`
- **版本策略**:
  - `v1`: bitable, baike, lingo, drive/v1, wiki(旧)
  - `v2`: wiki(现行), drive/v2, base/v2
  - `v3`: sheets(现行/推荐)
  - `v4`: 预留 feature 定义，暂未实现
- **URL 映射**: 严格遵循 `/open-apis/{project}/v{version}/{resource}/{action}`
- **编译控制**: 细粒度 Feature flags (如 `ccm-sheets-v3`, `baike-v1`)

## ANTI-PATTERNS
- **版本混用**: 在 `v2` 目录下实现 `v3` 逻辑。
- **过时依赖**: 新功能开发仍调用 `sheets/v2`。
- **硬编码 URL**: 绕过 `ApiEndpoint` 枚举手动拼接字符串。
- **Tag 混淆**: 忽视 `bizTag`（如 `base`, `ccm`）的分类前缀。
