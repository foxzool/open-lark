# Pre-release Compatibility Workflow

本文档定义 OpenLark 在正式发布前使用的 compatibility verification workflow。

## 1. 验证范围

该 workflow 目前覆盖以下兼容性信号：

1. Public API 稳定性 / deprecation / migration 关键文档存在
2. `cargo fmt --all -- --check`
3. `cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs`
4. `cargo test --workspace --all-features`
5. `bash scripts/check-public-examples.sh`
6. 常见 feature 组合回归：
   - `essential`
   - `enterprise`
   - `communication,websocket`
7. typed API coverage 汇总重新生成
8. crate 级质量状态摘要重新生成

## 2. 触发条件

工作流文件：`.github/workflows/pre-release-compatibility.yml`

当前支持以下触发方式：

- `workflow_dispatch`：发布前人工执行
- `workflow_call`：供 release 流程或其他 workflow 复用
- `pull_request`：当兼容性策略、release 流程、兼容性脚本或根入口相关文件改动时自动执行

## 3. 输出结果

workflow 成功后会产生以下输出：

- GitHub Step Summary：兼容性核对摘要
- Artifact `pre-release-compatibility-report`
  - `reports/pre_release_compatibility/summary.md`
  - `reports/pre_release_compatibility/release_quality_status.md`
  - `reports/api_validation/summary.json`
  - `reports/api_validation/summary.md`
  - `reports/api_validation/dashboards/core_business.json`
  - `reports/api_validation/dashboards/core_business.md`

## 4. 与发布流程的关系

该 workflow 的目标不是替代所有 CI，而是在正式 release 前集中验证**公开 API 与关键行为兼容性**。

建议使用方式：

1. 发布前先手动执行一次 `Pre-release Compatibility Verification`
2. 检查 artifact 与 Step Summary
3. 再执行 `docs/api-compatibility-release-checklist.md`
4. 最后执行正式 release 流程

如果未来需要更强的自动化约束，可由 release workflow 通过 `workflow_call` 直接调用本流程。
