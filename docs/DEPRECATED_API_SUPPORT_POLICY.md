# Deprecated API Support Policy

本文档定义 OpenLark 对 deprecated APIs 的支持周期、移除条件和通知方式。

目标：让用户在看到 `#[deprecated]` 后，能够明确判断“还能用多久、何时必须迁移、升级时去哪里看说明”。

## 1. 适用范围

本文档适用于以下已明确标记为 deprecated 的公开 surface：

- public entrypoints
- typed APIs
- helpers / convenience methods
- features
- re-exports / compatibility aliases

## 2. 默认支持周期

默认规则：

- **至少保留一个公开发布周期**
- 对 `0.15.x` patch 线新增的 deprecated API，默认在 `0.15.x` 内继续保留
- 如需移除，最早应在下一次计划 minor 版本窗口评估

换句话说：

- `deprecated in 0.15.x` ≠ `remove in next patch`
- patch 版本默认只发出迁移信号，不执行真正删除

## 3. 允许移除的条件

只有当以下条件同时满足时，才允许真正移除 deprecated API：

1. 已经给出推荐替代路径
2. migration guide / release notes 已记录迁移动作
3. 至少经历一个明确公开发布周期
4. 维护者确认继续保留只会制造更大歧义或长期维护负担

如果任一条件不满足，默认继续保留 deprecated 状态，而不是直接删除。

## 4. 例外情况

以下情况可跳过完整支持周期，但必须给出书面解释：

- 安全漏洞修复
- 严重正确性错误
- 上游官方 API 不兼容变化，且无合理兼容实现空间

即使是例外，也必须说明：

- 为什么不能继续兼容
- 受影响 surface
- 用户应如何迁移

## 5. 通知方式

每次新增 deprecated API，至少应在以下位置中的两处出现说明：

1. Rust 代码中的 `#[deprecated]`
2. changelog 的 `Deprecations` 分类
3. release notes / compatibility note
4. migration guide 或专门迁移说明文档

如果该 deprecated API 影响面较大（例如根入口、主推 helper、主 feature），建议四处全部覆盖。

## 6. 对外说明最小字段

每个 deprecated API 记录至少应包含：

- Deprecated path / feature
- Replacement
- First deprecated in
- Earliest removal
- Reason

推荐模板见：

- `docs/PUBLIC_API_STABILITY_POLICY.md`
- `docs/api-compatibility-note-template.md`

## 7. 发布前检查

发布前应确认：

- 本版本新增的 deprecated APIs 都有替代路径
- `Earliest removal` 没有与实际发布计划冲突
- release notes / migration docs 中能找到对应说明
- 如果计划移除某个已 deprecated API，已经满足本文档第 3 节条件

## 8. 当前项目约束

结合当前 `0.15` 兼容性收敛策略：

- 根 crate legacy entrypoints 已可进入 deprecated 状态
- `openlark-client` crate 本身仍是高级 canonical 入口，不属于应直接淘汰对象
- 对 typed API / helper 的 deprecated 处理，必须同时参考：
  - `docs/TYPED_API_SEMVER_RULES.md`
  - `docs/HELPER_SEMVER_RULES.md`
