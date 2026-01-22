---
name: openlark-design-review
description: OpenLark Rust SDK 的代码设计/公共 API 规范审查技能（面向 crate/模块）。用于系统化检查入口设计、feature gating、Request/Service/Builder 一致性、端点体系、Config/错误处理、导出与文档同步、测试与告警控制，并输出按优先级排序的整改清单与可落地改造方案。触发关键词：设计审查、crate 设计、API 设计、public API、feature flag、端点、Builder/Service、架构一致性
argument-hint: "[crate-name|path]"
allowed-tools: Read, Grep, Glob, Edit, Bash
---

# OpenLark 代码设计规范审查（Skill）

## 目标

把“设计审查”变成**可重复执行**的流程，而不是随手点评。

你要输出：
- 一份 **可执行的整改清单**（P0~P3，含证据与影响）
- 一份 **收敛方向**（选定 1 套对外范式，并说明迁移策略）
- 一段 **接口完成度量化**（完成/缺失/完成率，排除 `meta.Version=old`）
- 如用户同意：直接落地修复（优先处理 P0/P1）

## 适用范围

- 任意 crate（如 `crates/openlark-docs/`）或任意模块树（如 `src/ccm/wiki/v2/`）
- 重点面向 **public API** 与 **跨模块一致性**

## 0. 完成度统计（必须先做）

> 目的：把“我觉得实现了很多”变成可验证数据。统计口径默认排除 `meta.Version=old`。

### 0.1 以 crate 为单位统计（推荐）

使用仓库已有脚本直接对比 CSV 与落盘实现（strict 命名规范）：

```bash
python3 tools/validate_apis.py --crate <crate-name>
```

输出包含：API 总数/已实现/未实现/完成率，以及按 bizTag 的统计表；同时会生成默认报告到 `reports/api_validation/<crate>.md`。

### 0.2 以 bizTag 为单位统计（当用户只关心某些业务域）

```bash
python3 tools/validate_apis.py --src <crate-src-path> --filter <bizTag...>
```

### 0.3 预期总量参考（快速对齐）

若需要快速确认“某 bizTag 的有效 API 总数（排除 old）”，可参考仓库文档 `crates.md` 的统计表（数据源为 `api_list_export.csv`）。

如需从 `api_list_export.csv` 重新生成 bizTag 统计（排除 old），可运行：

```bash
python3 - <<'PY'
import csv
from collections import Counter

counts = Counter()
counts_all = Counter()
with open("api_list_export.csv", newline="", encoding="utf-8") as f:
    r = csv.DictReader(f)
    for row in r:
        biz = row.get("bizTag")
        ver = row.get("meta.Version")
        if not biz:
            continue
        counts_all[biz] += 1
        if ver != "old":
            counts[biz] += 1

print("bizTag,total,exclude_old,old")
for biz in sorted(counts_all.keys()):
    total = counts_all[biz]
    ex = counts[biz]
    print(f"{biz},{total},{ex},{total-ex}")
print(f"TOTAL,{sum(counts_all.values())},{sum(counts.values())},{sum(counts_all.values())-sum(counts.values())}")
PY
```

## 审查输入（先问清楚）

如果用户没说清楚范围，必须先追问 2 个问题：
1) 审查目标：仅该 crate，还是需要和全仓库一致（对齐 `openlark-client/openlark-core`）？
2) 改造约束：允许 breaking change 吗？（例如移除旧入口、调整 re-export 路径）

## 输出模板（强制）

按以下结构输出，不得随意变形：

1) **结论概览**：用 3~6 条 bullet 总结现状与最大风险
2) **接口完成度（排除 old）**：必须给出完成/缺失/完成率（来源见“0. 完成度统计”）
2) **问题清单（P0~P3）**：
   - 每条问题必须包含：`现象` → `证据(文件:行)` → `影响` → `建议`
3) **收敛方案**：
   - 选定 1 套“对外调用范式”（见 §1）
   - 给出迁移步骤与兼容策略（保留旧 API、deprecated 周期等）
4) **可执行 TODO**：
   - 最多 10 条，按优先级排序，能拆 PR 的粒度

> 注：证据必须精确到 `path:line`，避免“我觉得/好像”。

## 1. 对外范式（必须选 1 套，避免混用）

### 范式 A：Request 自持 Config（流式 Builder）

适用：大量端点、调用侧更偏“链式设置参数”。

特征：
- `Request::new(Config)` 保存 `Config`
- `Request::execute()` / `execute_with_options(RequestOption)`
- Service（若存在）只负责“分组/版本入口”，不承载网络逻辑

### 范式 B：Builder → build(Request) → execute(Service)

适用：希望把“执行上下文（Config/Transport）”都集中在 Service 上，便于 mock/注入。

特征：
- Builder 只拼 Request
- `ExecutableBuilder`/trait_system 统一 execute
- Service 持有 Config，并负责实际请求发送

**规则**：同一个 project/version 内不得同时出现 A+B；若历史原因混用，必须定义清晰的迁移路线。

## 2. 设计检查清单（按重要性）

### 2.1 Public API 入口与导出

- 是否存在多个“同义入口”（例如 `Client`/`Service`/`MainService`）导致用户困惑？
- 是否存在“占位/空实现”的 public API（例如 builder setter 不生效）？
- re-export 是否稳定、是否引入 `ambiguous_glob_reexports` 需要大量 allow？
- `prelude` 是否只导出“高频且稳定”的类型，避免把内部实现细节暴露出去？

### 2.2 Feature gating 一致性（Cargo.toml ↔ cfg）

- `Cargo.toml` 的 feature 是否与 `lib.rs/mod.rs` 的 `#[cfg(feature = "...")]` 对齐？
- 子模块是否按 feature 做最小编译单元，避免“开了一个 feature 实际编进来一大坨”？
- `default` features 是否合理（默认开启过多会放大编译成本与 API 面积）？

### 2.3 端点体系收敛

OpenLark 约束（必须遵守）：
- 禁止在实现文件里手写 `"/open-apis/..."`（除非集中在 `endpoints`/enum 中用于测试/示例）
- 生产调用必须通过“唯一端点来源”（enum 或常量系统二选一）

检查点：
- 是否出现多套端点系统并存（enum/const/path-template），且彼此不互用？
- 端点定义是否可被静态检查（避免遗漏、避免 typo）？

### 2.4 Config/生命周期与性能

- `openlark-core::Config` 本身已使用 `Arc` 共享；crate 内再包一层 `Arc<Config>` 通常是冗余设计。
- Service/Request 是否在不必要的地方 clone 大对象（如 http client）？
- `RequestOption` 是否在所有对外执行入口都可用并被透传？

### 2.5 错误处理与类型边界

- crate 自定义 Error 是否真正被使用？还是存在“孤儿文件/未被 mod 引入”的失效实现？
- 错误是否统一携带上下文（operation/resource_id/request_id）？
- validate 规则是否与全仓一致（必要时使用 `Skill(openlark-validation-style)`）

### 2.6 测试与告警控制

- `cargo check --all-features` 是否无 warning？（deprecated/unused 要么修，要么显式 allow）
- 单元测试是否只验证“构建正确”，避免依赖真实网络？
- 文档示例是否可 `compile`（必要时 `no_run/ignore` 但要有理由）

## 3. 常见整改套路（建议）

- **入口收敛**：保留一个 canonical（例如 `DocsClient`），其余入口标 `deprecated` 并给迁移路径。
- **删除占位 API**：对外暴露的 builder 若无法工作，要么补齐实现，要么移除/隐藏。
- **范式统一**：先在一个子域（如 `wiki/v2`）试点，再逐步复制到同域其他模块。
- **端点统一**：明确“生产用 enum，测试用 const”（或反之），并把另外两套标记为 internal/test-only。
- **feature 对齐**：把 `mod` 粒度切到能明显减少编译体积的位置（以“用户开哪个 feature 就编进来什么”为目标）。

## 4. 使用方式（给用户的最短指令）

当用户说“审查 XXX 设计”时：
- 先确认范围与 breaking 约束（见“审查输入”）
- 按“输出模板”给出报告
- 只要用户同意，就按 P0→P1 顺序落地改造并跑 `cargo check/test`

## 5. References

- PR 审查报告模板：`references/design-review-report-template.md`
