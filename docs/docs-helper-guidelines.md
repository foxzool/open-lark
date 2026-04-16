# Docs Helper 设计与命名规范

本文档定义 `openlark-docs` 中 Docs helper 的准入标准、命名约定、返回策略与错误表达，用于约束后续新增 helper 的设计边界。

## 1. 设计目标

Docs helper 解决的是“多步、高频、易错”的任务型调用，而不是替代 typed API 本身。

核心目标：

- 把常见业务动作从零散 endpoint 拼装提升为可复用入口。
- 保持 helper 输入输出稳定、可预测、可组合。
- 明确 helper 与 typed API 的边界，避免职责漂移。
- 让示例、README 和业务代码优先展示任务型能力，而不是参数堆叠。

## 2. helper 与 typed API 的边界

| 场景 | 应新增 helper | 仅补 typed API |
|------|---------------|----------------|
| 需要 2 步及以上调用才能完成常见业务动作 | ✅ | - |
| 需要稳定默认参数（如 `parent_type = explorer`、自动分页） | ✅ | - |
| 主要价值是把用户输入转换为稳定中间表达（如 `SheetRange`、`WikiNodePath`） | ✅ | - |
| 只是单个 endpoint 的 1:1 暴露 | - | ✅ |
| 属于低频、强专业化、参数组合复杂的高级能力 | - | ✅ |
| 会引入完整 DSL / 本地缓存 / 状态同步 | - | ✅（除非另有明确需求） |

判定原则：

1. **优先补 typed API**，helper 只在重复度足够高时新增。
2. helper 必须能够说清楚“隐藏了什么复杂度”。
3. helper 不应绕过已有 typed API；应在 typed API 之上组合。
4. 如果 helper 只是给单个 endpoint 起了新名字，不应新增。

## 3. 命名规范

### 3.1 方法命名

| 模式 | 适用场景 | 示例 |
|------|----------|------|
| `list_*_all` | 自动处理分页并返回全集合 | `list_folder_children_all` |
| `find_*_by_*` | 通过业务键查找单个对象 | `find_sheet_by_title` / `find_wiki_node_by_title` |
| `resolve_*` | 将用户输入解析为稳定 helper 类型 | `resolve_sheet_range_by_title` |
| `read_*` | 读取型聚合 helper | `read_sheet_ranges` |
| `write_*` | 覆盖/批量写入型 helper | `write_sheet_ranges` |
| `append_*` | 追加型 helper | `append_sheet_range` |
| `upload_*` / `download_*` | 文件传输型 helper | `upload_drive_file` / `download_drive_file` |
| `query_*` | 带筛选条件的任务型读取 helper | `query_bitable_records` |

约束：

- 方法名优先表达“任务动作”，不要泄露底层 endpoint 命名细节。
- `find_*` 必须返回单个结果，并在 0 个或多个命中时显式报错。
- `list_*_all` 默认表示“自动处理分页”，不要再额外暴露 `has_more`。
- `resolve_*` 结果应可直接交给其他 helper 继续使用。

### 3.2 类型命名

helper 专用输入/中间表达建议使用名词结构：

- `SheetRange`
- `SheetWriteRange`
- `DriveUploadFile`
- `DriveDownloadRange`
- `BitableRecordQuery`
- `WikiNodePath`

约束：

- 类型名应表达“业务语义”，不要只是 `*Params` / `*Helper`。
- 如果类型可以安全序列化为底层输入，优先实现 `Display` / `From` / 构造器，而不是额外暴露可变字符串拼接接口。

## 4. 返回策略

helper 返回值必须与“任务完成”语义一致，而不是机械透传 `Response<T>`。

推荐规则：

1. **集合型 helper**：返回 `Vec<T>` 或 typed payload。
   - 例：`list_folder_children_all -> SDKResult<Vec<FileItem>>`
2. **单对象查找 helper**：返回 `SDKResult<T>`。
   - 例：`find_sheet_by_title -> SDKResult<SpreadsheetSheetInfo>`
3. **解析/导航 helper**：返回 helper 类型或目标 typed model。
   - 例：`resolve_sheet_range_by_title -> SDKResult<SheetRange>`
4. **批量读写 helper**：返回 typed API 的 `data` 载荷结构，而不是原始响应包裹层。
   - 例：`read_sheet_ranges -> SDKResult<MultipleRangeData>`
5. **文件下载 helper**：直接返回 `Vec<u8>` 等业务最终结果。

不要做的事：

- 不要让 helper 再返回 `Response<T>`，这会把底层协议细节重新暴露给调用方。
- 不要在 helper 中发明和 typed model 重复的新响应结构。

## 5. 错误表达规范

Docs helper 的错误分三层：

| 错误类型 | 何时使用 | 示例 |
|----------|----------|------|
| `validation_error` | helper 输入本地就能判定非法 | 空 `wiki_path`、空 `range_expr` |
| `business_error` | 业务语义不满足 | 未找到 sheet、命中多个同名 wiki 节点 |
| `api_data_error` | 底层调用成功但缺少期望 data | `response.data` 为空 |

约束：

- `find_*` / `resolve_*` 必须输出可读的业务错误，不能只返回“None”。
- 同名冲突等歧义必须报错，不能静默取第一个结果。
- helper 不应吞掉 typed API 错误；只对“helper 自己新增的语义层”补充更清晰的表达。

## 6. 当前 helper 家族的对齐建议

### 6.1 分页遍历

- `folder_children_pager`
- `list_folder_children_all`

建议：

- 分页型 helper 优先提供 `Pager + *_all` 双层入口。
- `TypedPage<T>` 作为统一分页返回体，避免每个资源再造一套 `next_page_token` 命名。

### 6.2 表格范围与写回

- `SheetRange`
- `SheetWriteRange`
- `find_sheet_by_title`
- `resolve_sheet_range_by_title`
- `read_sheet_ranges`
- `write_sheet_ranges`
- `append_sheet_range`

建议：

- 任何需要 `sheet_id!A1:C5` 的 helper，都优先复用 `SheetRange`。
- 批量写入统一走 `SheetWriteRange`，避免业务侧直接手拼 `data_range`。

### 6.3 Drive 文件传输

- `DriveUploadFile`
- `DriveDownloadRange`
- `upload_drive_file`
- `download_drive_file`
- `download_drive_file_range`

建议：

- helper 只固化高频默认值（如 `parent_type = explorer`）。
- 权限、分享、订阅等非高频高级能力仍保留在 typed API。

### 6.4 Bitable 查询

- `BitableRecordQuery`
- `query_bitable_records`
- `search_bitable_records_all`

建议：

- helper 只覆盖最常见筛选动作（精确匹配、包含、候选值列表）。
- 避免把所有过滤能力都提升为 DSL。

### 6.5 Wiki 导航

- `WikiNodePath`
- `list_wiki_space_nodes_all`
- `find_wiki_node_by_title`
- `find_wiki_node_by_path`

建议：

- 路径导航应显式按层级解析，不引入本地缓存。
- 对同级重名节点必须报错，避免隐式命中错误节点。

## 7. 何时新增 helper，何时只补 typed API

新增 helper 前，请至少确认以下 4 点：

1. **业务动作可复述**：一句话能说清楚“这个 helper 替调用方完成了什么任务”。
2. **调用频率足够高**：至少在示例、README 或业务接入里会重复出现。
3. **默认值合理且稳定**：helper 给出的默认策略不容易误导用户。
4. **边界清晰**：helper 不会吞并底层 typed API 的高级能力空间。

只补 typed API 的典型信号：

- 只缺一个 endpoint。
- 参数组合高度开放，不存在单一默认业务流。
- 需要复杂缓存/同步/状态机。
- 需求更像“覆盖率补齐”而不是“任务抽象”。

## 8. 对后续新增 helper 的准入清单

新增 Docs helper 时，PR/提交应至少回答：

- 名称是否符合本文档约定？
- 返回值是否面向业务结果，而不是底层响应壳？
- 是否复用了已有 helper 类型（如 `SheetRange` / `TypedPage`）？
- 是否通过示例或测试证明典型用法？
- 是否说明了与 typed API 的边界？

如果以上问题无法回答，优先回到 typed API 层而不是继续扩展 helper。
