# Docs Contract Test Scope

本文档说明 `crates/openlark-docs/tests/docs_contract_models.rs` 当前锁定的 contract test 范围。

## 目标

本轮 contract tests 的目标不是一次性覆盖所有 Docs API，而是优先锁定**高频业务链路中最容易因重构破坏 wire format 的代表性模型**。

## 当前覆盖面

### 1. Drive 文件上传

- 请求：`UploadPrepareRequest`
- 响应：`UploadPrepareResponse`

为什么先测：

- Drive 上传是 docs helper 和入门工作流里的主路径之一
- `parent_type`、`parent_node`、`size` 这类字段一旦改名或序列化错位，回归影响直接可见

### 2. Docx 文档读取

- 请求：`GetDocumentRawContentParams`
- 响应：`GetDocumentRawContentResponse`

为什么先测：

- 文档读取是 Docs 主体能力
- `document_id / lang / content` 构成最稳定的文档读取契约
- 该接口不依赖复杂嵌套结构，适合作为第一批 contract test 样板

### 3. Sheets 电子表格

- 请求：`CreateSpreadsheetParams`
- 响应：`GetSpreadsheetResponse`

为什么先测：

- Spreadsheet 是 README 和 helper 场景里的高频入口
- 标题、folder token、token/url 等字段属于典型外部契约字段

### 4. Bitable 记录查询

- 请求：`SearchRecordRequestBody`
- 响应：`SearchRecordResponse`

为什么先测：

- Bitable 查询是典型高频读路径
- `sort` / `filter` / `automatic_fields` 组合最容易在重构时发生字段回归

### 5. Wiki 节点查询

- 请求：`GetWikiSpaceNodeParams`
- 响应：`GetWikiSpaceNodeResponse`

为什么先测：

- Wiki 节点导航是 docs helper 的稳定主路径
- `token / obj_type / obj_token / parent_node_token` 属于典型兼容性敏感字段

## 当前不覆盖的内容

本轮刻意**不**覆盖以下范围：

- 所有 Docs API 的全量 contract matrix
- helper 层输入输出 contract
- 网络层、鉴权层或端点 URL 集成验证
- 旧版本 API 的兼容 contract

这些内容应在后续质量里程碑中逐步扩展，而不是在本 issue 中一次性做大。

## 扩展原则

后续如要继续补充 Docs contract tests，优先顺序建议为：

1. README / examples 主推路径
2. helper 直接依赖的 request / response 模型
3. 高价值且字段复杂的写操作
4. 尾部低频接口

换言之：**先锁主路径，再锁长尾。**
