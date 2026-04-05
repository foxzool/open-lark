# OpenLark Public API 稳定性与废弃策略

生效版本：`0.15`

## 目标

即使项目仍处于 `0.x` 阶段，也不应把“尚未 1.0”当作任意破坏公开 API 的理由。

从 `0.15` 开始，OpenLark 将 public API 的稳定性视为正式发布质量的一部分。

## Public API 边界

以下内容默认视为 public API：

- 根 crate `openlark` 导出的公开类型、模块与入口
- 各 published crate 暴露给最终用户直接使用的公开类型、函数、builder、trait
- 对外文档、README、示例中主推的调用路径
- 已发布到 crates.io 并在文档中公开声明的 feature 名称
- 公开 re-export 的路径与名称

以下内容默认不承诺稳定：

- `pub(crate)`、内部模块、内部工具函数
- 未在文档中承诺的实验性实现细节
- 仅用于测试或生成流程的内部脚本接口

## 版本语义

从 `0.15` 起，采用以下约束：

- `0.15.x` patch 版本必须保持向后兼容，除非是安全修复或严重错误修复且无法避免
- 跨 minor 版本的公开 API 调整，必须提供 changelog 和迁移说明
- 任何公开入口、re-export、feature 命名调整，都应被视为兼容性变更

## 什么属于 breaking change

以下行为默认视为 breaking change：

- 删除或重命名公开类型、函数、模块、trait、builder 方法
- 修改公开类型字段语义，导致现有调用无法保持原行为
- 修改主推示例路径，导致文档中的接入方式失效
- 删除已发布 feature，或改变其含义
- 移除公开 re-export，导致用户 import 路径失效

## 什么通常不属于 breaking change

以下行为通常可视为非 breaking：

- 新增公开 API
- 为现有 builder 增加可选方法
- 为响应结构新增可选字段
- 修正文档、示例、注释和错误消息
- 在不改变公开语义的前提下重构内部实现

## Deprecation 策略

当某个公开入口不再推荐使用时，默认应遵循以下步骤：

1. 先保留现有入口
2. 提供推荐替代路径
3. 在代码或文档中明确标注 deprecated
4. 在 changelog / release notes / migration guide 中记录迁移方式
5. 在至少一个公开发布周期后，再评估是否移除

如果因安全或严重正确性问题必须立即收缩接口，可以跳过完整废弃周期，但必须在发布说明中明确原因。

## Re-export 与入口规范

根 crate `openlark` 是普通用户的默认官方入口。

因此：

- 新公开能力优先考虑是否应通过根 crate 暴露
- 如果公开 re-export 已在 README、示例或 release note 中主推，则路径变化默认视为 breaking
- `openlark-client` 可继续作为高级入口存在，但不应再次回到“普通用户默认入口”的文档位置

## Feature 稳定性

已公开声明的 feature 应满足以下规则：

- 名称应表达业务能力或明确的技术能力
- 不应将内部实现分层直接暴露为长期公共心智
- 删除或重命名已公开 feature，默认视为 breaking
- 新 feature 发布时，应说明用途、边界与推荐组合

## 文档与示例要求

如果某个入口被视为公开稳定接口，则其对应文档至少应满足：

- README 示例与真实依赖名一致
- 主推代码路径可编译或有等价自动化校验
- 迁移路径在入口变化时同步更新

## 发布要求

涉及 public API 的发布前，至少应确认：

- changelog 已记录兼容性影响
- release notes 已说明升级关注点
- migration guide 已覆盖实际迁移路径
- CI 已覆盖关键公开示例或等价验证

## 例外原则

以下情况可以破例，但必须留下书面记录：

- 安全漏洞修复
- 严重正确性错误
- 上游 API 不可兼容变更且没有兼容实现空间

例外发布仍需说明：

- 为什么不能保持兼容
- 受影响范围
- 用户应如何迁移
