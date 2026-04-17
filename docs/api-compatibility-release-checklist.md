# API Compatibility Release Checklist

本文档定义 OpenLark 在**准备发布一个公开版本前**应执行的 API compatibility review checklist。

它面向发布流程，而不是单个 PR。自动化检查与人工复核都以本清单为准。

## 1. 触发时机

以下任一情况成立时，应执行本清单：

1. 准备发布正式版或 RC 版本
2. 发布前怀疑存在 public API / helper / feature / re-export 调整
3. 一个版本周期内累计了 deprecated 入口、helper 行为变化或 typed API 扩张

建议执行顺序：

- **第一次执行**：进入 release freeze 或准备打 tag 前
- **第二次执行**：最终创建 release tag 前，确认没有新引入兼容性漂移

## 2. 责任分工

| 角色 | 责任 |
| --- | --- |
| 发布负责人 | 触发 checklist、收集结论、决定是否允许发布 |
| 变更作者 / 模块 owner | 提供 breaking / deprecation / migration 说明 |
| reviewer / maintainer | 对照 semver / compatibility policy 做人工判定 |
| CI workflow | 提供自动化验证证据与 artifact |

如果一个人同时承担多个角色，也必须分别完成“执行”和“复核”视角的检查。

## 3. 发布前兼容性检查项

### A. Public surface inventory

- [ ] 本版本涉及的 public surface 已识别：
  - entrypoints
  - typed APIs
  - helpers / convenience methods
  - features
  - re-exports
- [ ] 每项变化都能映射到对应策略文档：
  - `docs/PUBLIC_API_STABILITY_POLICY.md`
  - `docs/TYPED_API_SEMVER_RULES.md`
  - `docs/HELPER_SEMVER_RULES.md`
  - `docs/PUBLIC_REEXPORT_POLICY.md`

### B. Breaking / non-breaking 判定

- [ ] 已检查是否存在未记录的 breaking change
- [ ] typed API 变化已按 `docs/TYPED_API_SEMVER_RULES.md` 判定
- [ ] helper / convenience method 变化已按 `docs/HELPER_SEMVER_RULES.md` 判定
- [ ] re-export / legacy entrypoint 变化已按 `docs/PUBLIC_REEXPORT_POLICY.md` 判定
- [ ] 如果签名没变但行为变了，已把它当作 breaking 候选单独审查

### C. Deprecation / support / migration

- [ ] 新 deprecated 入口已带有替代路径
- [ ] deprecated API 的支持周期与移除条件清晰
- [ ] migration guide 或 compatibility note 已包含 before / after 示例
- [ ] legacy entrypoint 变化已同步到：
  - `docs/migration-guide.md`
  - `docs/legacy-entrypoint-migration-notes.md`

### D. Release note / changelog completeness

- [ ] changelog 已按稳定分类记录影响范围
- [ ] breaking / deprecated / compatible extension 在 release notes 中可快速识别
- [ ] crate 级质量状态摘要已附加到 release notes
- [ ] 如需额外兼容性说明，已复用 `docs/api-compatibility-note-template.md`
- [ ] changelog 分类已对齐 `docs/changelog-compatibility-categories.md`

### E. Evidence / verification

- [ ] `Pre-release Compatibility Verification` workflow 已执行
- [ ] artifact 与 Step Summary 已审阅
- [ ] README / public examples compile-check 仍然通过
- [ ] common feature combinations regression 仍然通过
- [ ] typed coverage / crate quality status 报告已重新生成并审阅

## 4. 允许发布的结论标准

只有当以下条件同时满足时，compatibility review 才应给出“允许发布”结论：

1. 没有未记录的 breaking change
2. 所有 deprecated 入口都有替代路径与支持说明
3. release notes / changelog / migration docs 已能解释升级影响
4. 自动化兼容性验证为绿色，且 artifact 没有明显异常

如果任一条件不满足，默认结论应为：

- **阻塞发布**，或
- **显式记录 waiver / 风险说明后再继续**

## 5. 输出记录

每次执行本清单时，至少应留下以下输出：

- 执行时间
- 执行人 / 复核人
- 本次涉及的兼容性风险摘要
- 是否允许发布
- 若不允许发布，阻塞项是什么

建议把结果挂到：

- release issue / release PR
- GitHub Release 准备说明
- 版本发布记录或内部发布 checklist
