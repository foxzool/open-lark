# Wave FINAL F3 覆盖率分析报告

## 执行信息
- 时间: 2026-02-17
- 包: `openlark-hr`
- 命令 1: `cargo llvm-cov --package openlark-hr --html --output-dir .sisyphus/evidence/coverage-report`
- 命令 2: `cargo llvm-cov --package openlark-hr --summary-only`

## 证据文件
- HTML 覆盖率报告目录: `.sisyphus/evidence/coverage-report`
- 覆盖率完整输出: `.sisyphus/evidence/final-f3-coverage.txt`
- 覆盖率摘要输出: `.sisyphus/evidence/final-f3-summary.txt`

## 关键结果

### A) `llvm-cov` TOTAL（来自 summary）
- Regions: `23.19%` (`24203 total`, `18591 missed`)
- Functions: `15.27%` (`4821 total`, `4085 missed`)
- Lines: `21.54%` (`21981 total`, `17247 missed`)

### B) openlark-hr 聚合（按 `openlark-hr/src/**` 行聚合）
- Regions: `25.67%` (`19527 total`, `14515 missed`)
- Functions: `15.58%` (`4319 total`, `3646 missed`)
- Lines: `23.29%` (`18310 total`, `14046 missed`)

### C) Core API 近似聚合（按 `openlark-hr/src/**/v1/**` 且排除 `mod.rs`）
- Regions: `27.28%` (`13416 total`, `9756 missed`)
- Functions: `17.74%` (`3004 total`, `2471 missed`)
- Lines: `25.33%` (`12610 total`, `9416 missed`)

## 目标对比
- Core API Coverage 目标: `>= 80%` -> **未达标**（当前约 `17.74%` functions / `25.33%` lines）
- Overall Coverage 目标: `>= 60%` -> **未达标**（TOTAL `21.54%` lines，openlark-hr 聚合 `23.29%` lines）

## 备注
- 两次覆盖率命令均执行成功，并生成 HTML 报告。
- 运行期间出现提示: `warning: 1 functions have mismatched data`。
