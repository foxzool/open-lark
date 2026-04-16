# Generated Typed API SemVer 规则

本文档定义 OpenLark 中**生成型 typed API** 的语义化版本判断规则，用于评审、发布说明与兼容性核对。

## 1. 适用范围

本文档覆盖以下公开 surface：

- `v1/`、`v2/` 等目录下对外暴露的 request builder / body / response / model
- 公开 module path、类型名、builder 方法名
- 公开字段名、字段必填语义、枚举值解析语义
- 由生成流程产出的公开 trait / alias / re-export（如果被最终用户直接使用）

以下内容默认不在本规则直接承诺范围内：

- 代码生成脚本本身
- 仅用于内部生成流程的中间 schema
- 未导出、未文档化的内部辅助函数

## 2. 版本处理总则

### Patch (`0.15.x`)

patch 版本只允许发布以下类型的 generated typed API 变更：

- **兼容新增**：新增 endpoint、module、request builder、可选字段、可选 setter
- **兼容修复**：修正错误文档、补全漏掉的可选解析、增强别名解析容错
- **兼容收敛**：补文档、补 contract tests、补 snapshot / compile-check，不改变既有公开语义

patch 版本**不允许**发布会导致既有调用失效的 typed API 破坏性变更。

### Next minor (`0.15 -> 0.16`)

如果 generated typed API 必须发生 breaking change，应放到下一次计划中的 minor 版本窗口，并同时满足：

1. release note 明确说明 breaking surface
2. migration guide 给出 before / after 示例
3. `docs/api-compatibility-note-template.md` 中的兼容性说明被实际复用
4. 如果存在可桥接路径，优先先经历一个 deprecation 周期

## 3. 常见变更场景判定

| 变更场景 | 判定 | 允许的发布窗口 | 必要动作 |
| --- | --- | --- | --- |
| 新增一个新的 typed endpoint / module | 兼容新增 | patch / minor | changelog 记录新增能力 |
| 为 request builder 新增可选 setter | 兼容新增 | patch / minor | 无需迁移说明，必要时补示例 |
| 为 response model 新增可选字段 | 兼容新增 | patch / minor | 保持默认反序列化兼容 |
| 扩展枚举解析以接受别名或更宽松输入 | 兼容修复 | patch / minor | 说明兼容原因 |
| 修正文档或示例中的 typed API 调用方式 | 文档修复 | patch / minor | 更新 README / examples |
| 重命名公开 module path / 类型名 | breaking | next minor | release note + migration guide |
| 删除公开 endpoint 对应的 request / response 类型 | breaking | next minor | 迁移路径 + 兼容性说明 |
| 将原本可选字段改为必填 | breaking | next minor | 说明服务端契约变化与用户迁移动作 |
| 删除 builder 方法或修改其参数顺序 / 语义 | breaking | next minor | 迁移示例 + 兼容性说明 |
| 改变公开字段语义，导致旧代码行为变化 | breaking | next minor | 明确行为差异 |
| 将宽松反序列化收紧为可能拒绝旧 payload 的解析 | breaking | next minor | 解释风险与替代方案 |
| 仅调整内部生成实现，不影响导出 surface | 非 public API 变化 | 任意 | 无需兼容性说明 |

## 4. 评审时必须回答的问题

对任何 generated typed API 变更，评审至少应确认：

1. 是否改动了公开 module path、类型名、字段名或 builder 方法？
2. 是否把“过去可编译 / 可反序列化 / 可运行”的调用变成不可用？
3. 是否只是新增可选能力，而没有改变旧调用语义？
4. 是否需要 release note / migration guide / deprecation 记录？

如果前两项任一为“是”，默认按 **breaking** 处理，除非有明确证据证明旧调用仍完全兼容。

## 5. 发布与流程接入

发布前的兼容性核对至少应包含：

- 检查 typed API 相关 PR 是否标注为 `breaking` / `compatible extension` / `deprecation`
- 检查 contract tests、public examples、release notes 是否与新 surface 一致
- 对 breaking typed API 变更，确认 migration guide 中已有 before / after 示例
- 对 patch 版本，确认没有未记录的 typed API 破坏性改动

建议在 release checklist 和 pre-release compatibility workflow 中直接引用本文件。
