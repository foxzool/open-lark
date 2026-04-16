# Helper 与 Convenience Method SemVer 规则

本文档定义 OpenLark 中 helper 与 convenience method 的 semver 判定规则，用于评审、发布说明与兼容性核对。

## 1. 适用范围

本文档覆盖以下公开 surface：

- 任务型 helper（如 `list_*_all`、`find_*`、`query_*`、`approve_*`）
- helper 输入类型（如 `SheetRange`、`MessageRecipient`、`WorkflowTaskListQuery`）
- convenience method（如 `send_text`、`upload_file`、`complete_task`）
- helper 的默认值策略、分页策略、歧义处理策略与返回结果语义

以下内容默认不在本规则直接承诺范围内：

- helper 内部复用的临时中间对象
- 不对外导出的实现细节
- 只影响日志、注释或内部追踪字段、且不改变公开行为的内部改造

## 2. 版本处理总则

### Patch (`0.15.x`)

patch 版本允许的 helper / convenience method 变更：

- **兼容新增**：新增 helper、新增可选参数、新增不改变旧行为的 convenience method
- **兼容修复**：修正错误文档、补充更清晰的错误上下文、修复对合法输入不应失败的 bug
- **兼容收敛**：补 snapshot / contract / example / compile-check，不改变旧 helper 语义

patch 版本**不允许**在没有明确说明的情况下改变既有 helper 的默认行为、返回语义或歧义处理结果。

### Next minor (`0.15 -> 0.16`)

如果 helper / convenience method 必须发生 breaking change，应放到下一次计划中的 minor 版本窗口，并同时满足：

1. release note 说明受影响 helper surface
2. migration guide 给出 before / after 示例
3. `docs/api-compatibility-note-template.md` 中的兼容性说明被实际复用
4. 如果可通过新增参数、保留旧默认值或保留旧 helper 名称桥接，则优先走非 breaking 路径

## 3. 行为变化 vs 接口变化

### 接口变化（默认按 breaking 处理）

以下变化默认按 breaking 处理：

- 删除或重命名 helper / convenience method
- 改变参数顺序、参数含义或返回类型
- 将原本可选参数改为必填
- 将单结果 helper 改成集合结果，或相反

### 行为变化（需单独审查）

以下变化即使函数签名没变，也可能是 breaking：

- 改变默认分页大小或是否自动翻页
- 改变“找不到 / 多结果命中”时的返回语义
- 改变默认 parent type / message target / workflow action 等业务默认值
- 改变返回结果的任务完成语义（例如原本返回业务结果，后来返回底层响应壳）

默认判断规则：**如果旧调用在不改代码的情况下会得到不同业务结果，应按 breaking 候选处理。**

## 4. 常见变更场景判定

| 变更场景 | 判定 | 允许的发布窗口 | 必要动作 |
| --- | --- | --- | --- |
| 新增一个新的 helper 或 convenience method | 兼容新增 | patch / minor | changelog 记录新增能力 |
| 为 helper 输入类型新增可选字段 / builder setter | 兼容新增 | patch / minor | 保持旧构造方式可用 |
| 加强输入校验，但不影响历史合法输入 | 兼容修复 | patch / minor | 说明校验目的 |
| 改善错误上下文，但保持错误类别与合法输入行为不变 | 兼容修复 | patch / minor | 无需迁移说明 |
| 删除 helper 或改 helper 名称 | breaking | next minor | release note + migration guide |
| 改变默认值，导致旧调用业务结果变化 | breaking | next minor | 解释行为差异与替代写法 |
| 改变分页 helper 的自动翻页 / 截断策略 | breaking | next minor | 说明结果集变化 |
| 改变“唯一命中 / 歧义 / 未命中”时的返回语义 | breaking | next minor | 给出 before / after 示例 |
| 将返回 `Vec<T>` 的 helper 改成返回底层响应结构 | breaking | next minor | 说明返回类型与迁移方式 |
| 仅补文档、示例、snapshot、contract tests | 非行为变化 | patch / minor | 无需兼容性说明 |

## 5. 评审时必须回答的问题

对任何 helper / convenience method 变更，评审至少应确认：

1. 是否改动了 helper 名称、参数形状或返回类型？
2. 是否改动了默认值、分页、歧义处理或任务完成语义？
3. 旧调用在不改代码的情况下，结果是否会发生业务层变化？
4. 是否需要 release note / migration guide / deprecation 记录？

如果前 3 项任一为“是”，默认按 **breaking 候选** 处理，除非有明确证据证明旧调用仍完全兼容。

## 6. 发布与流程接入

发布前的兼容性核对至少应包含：

- 检查 helper 相关 PR 是否标注为 `breaking` / `compatible extension` / `deprecation`
- 检查 helper snapshot、public examples、边界文档是否与新行为一致
- 对 breaking helper 变更，确认 migration guide 中已有 before / after 示例
- 对 patch 版本，确认没有未记录的默认值或行为语义变化

建议在 release checklist 和 pre-release compatibility workflow 中直接引用本文件。
