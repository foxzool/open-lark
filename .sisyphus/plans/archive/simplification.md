# Plan 3: 简化过度设计 (Issue E + F)

## TL;DR

> **Quick Summary**: 简化 ServiceRegistry 为轻量元数据查询，清理 root crate 过多的第三方依赖重导出。
> 
> **Deliverables**:
> - 简化后的 ServiceRegistry（删除 factory/resolver/bootstrap 复杂度）
> - 精简后的 root crate 依赖（仅保留 API 表面需要的重导出）
> 
> **Estimated Effort**: Short
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Task 1 → Task 2 → Task 4
> **前置依赖**: 无（独立于 Plan 1/2）

---

## Context

### Original Request
ServiceRegistry 有 5 个文件但实际服务访问全靠编译时 struct 字段。Root crate 重导出 23 个第三方依赖。

### 关键发现
- registry/ 目录：bootstrap.rs, dependency_resolver.rs, feature_flags.rs, mod.rs, service_factory.rs
- 公开 API 仅 `has_service()` 和 `list_services()`
- Root Cargo.toml 第 186-208 行重导出 anyhow/chrono/serde/tokio 等 23 个依赖
- 这些依赖锁定用户版本，升级即 breaking change

---

## Work Objectives

### Core Objective
减少不必要的复杂度和依赖暴露，让 SDK 更精简。

### Definition of Done
- [x] registry/ 目录文件数 ≤ 2
- [x] root crate 非 optional 第三方依赖数 ≤ 5
- [x] `cargo check --workspace --all-features` 通过
- [x] `cargo test --workspace` 全部通过

### Must Have
- `client.registry().has_service("docs")` 仍可用
- `client.registry().list_services()` 仍可用
- 用户通过 `open-lark` crate 仍能访问核心功能

### Must NOT Have (Guardrails)
- ❌ 不删除用户 API 表面需要的重导出
- ❌ 不修改业务 crate 内部实现
- ❌ 不改变 Client struct 的公开字段

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION**

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after
- **Framework**: cargo test

---

## Execution Strategy

```
Wave 1 (Parallel — 审计 + 简化):
├── Task 1: 审计 ServiceRegistry 实际使用 [quick]
├── Task 2: 审计 root crate 重导出的实际使用 [quick]

Wave 2 (After Wave 1 — 实施):
├── Task 3: 简化 ServiceRegistry [unspecified-high]
├── Task 4: 精简 root crate 依赖 [unspecified-high]

Wave 3 (验证):
├── Task 5: 全工作区验证 [quick]

Wave FINAL:
├── Task F1: Plan compliance audit [oracle]
├── Task F2: Code quality review [unspecified-high]
├── Task F3: Scope fidelity check [deep]

Critical Path: Task 1 → Task 3 → Task 5 → F1-F3
Max Concurrent: 2
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 3 |
| 2 | — | 4 |
| 3 | 1 | 5 |
| 4 | 2 | 5 |
| 5 | 3, 4 | F1-F3 |

---

## TODOs

- [x] 1. 审计 ServiceRegistry 实际使用
  **What to do**:
  - 查找 `DefaultServiceRegistry`, `ServiceRegistry`, `ServiceEntry`, `ServiceMetadata` 的所有外部引用
  - 记录 `register_compiled_services()` 的调用点
  - 记录 `has_service()` 和 `list_services()` 的调用点（包括 examples/tests）
  - 评估 `dependency_resolver.rs` 和 `service_factory.rs` 是否有外部使用
  - 输出到 `.sisyphus/evidence/task-1-registry-audit.md`
  **Must NOT do**: 不修改任何代码
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 2)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 3
  - **Blocked By**: None
  **References**:
  - `crates/openlark-client/src/registry/` — 5 个文件
  - `crates/openlark-client/src/client.rs:149` — `register_compiled_services()` 调用
  - `crates/openlark-client/src/lib.rs:274-276` — ServiceRegistry 公开导出
  **Acceptance Criteria**:
  - [ ] 审计清单文件存在
  - [ ] 清单包含每个 registry 文件的使用/未使用状态
  **QA Scenarios:**
  ```
  Scenario: 审计完整性
    Tool: Bash (grep)
    Steps:
      1. grep -rn 'ServiceRegistry\|has_service\|list_services\|ServiceEntry' crates/ src/ examples/ --include="*.rs" -l
      2. 对比清单
    Expected Result: 清单覆盖所有引用
    Evidence: .sisyphus/evidence/task-1-registry-audit.md
  ```
  **Commit**: NO

- [x] 2. 审计 root crate 重导出的实际使用
  **What to do**:
  - 检查 `Cargo.toml` 第 186-208 行的 23 个重导出依赖
  - 对每个依赖，搜索其类型是否出现在 SDK 的公开 API 签名中（函数参数/返回值/pub struct 字段）
  - 分类为：API 表面需要（保留）vs 仅内部使用（移除重导出）
  - 输出到 `.sisyphus/evidence/task-2-reexport-audit.md`
  **Must NOT do**: 不修改任何代码
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 1)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 4
  - **Blocked By**: None
  **References**:
  - `Cargo.toml:186-208` — 23 个重导出依赖
  - `src/lib.rs` — root crate 公开 API
  **Acceptance Criteria**:
  - [ ] 审计清单文件存在
  - [ ] 每个依赖标注为「保留」或「移除」及理由
  **QA Scenarios:**
  ```
  Scenario: 审计完整性
    Tool: Bash (grep)
    Steps:
      1. 统计 Cargo.toml 中非 optional 第三方依赖数
      2. 对比清单中的条目数
    Expected Result: 数量一致
    Evidence: .sisyphus/evidence/task-2-reexport-audit.md
  ```
  **Commit**: NO
- [x] 3. 简化 ServiceRegistry
  **What to do**:
  - 根据 Task 1 审计结果，删除未使用的文件（预计 dependency_resolver.rs, service_factory.rs）
  - 将 bootstrap.rs 中 `register_compiled_services()` 逻辑内联到 mod.rs 或 client.rs
  - 简化 `DefaultServiceRegistry` 为轻量 struct（仅 Vec<ServiceMetadata>）
  - 保留 `has_service()` 和 `list_services()` 公开 API
  - 删除 `feature_flags.rs` 如果其逻辑可内联
  **Must NOT do**:
  - 不删除 `has_service()` / `list_services()` 公开 API
  - 不修改 Client struct 的公开字段
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: [`openlark-code-standards`]
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 4)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5
  - **Blocked By**: Task 1
  **References**:
  - `crates/openlark-client/src/registry/` — 5 个文件
  - `crates/openlark-client/src/client.rs:149` — register_compiled_services() 调用
  - `.sisyphus/evidence/task-1-registry-audit.md` — Task 1 产出
  **Acceptance Criteria**:
  - [ ] registry/ 目录 ≤ 2 个文件
  - [ ] `has_service()` 和 `list_services()` 仍可用
  - [ ] `cargo test -p openlark-client` 通过
  **QA Scenarios:**
  ```
  Scenario: Registry 简化验证
    Tool: Bash
    Steps:
      1. ls crates/openlark-client/src/registry/ | wc -l
      2. cargo test -p openlark-client -- registry --nocapture
    Expected Result: ≤2 文件，测试通过
    Evidence: .sisyphus/evidence/task-3-registry-slim.txt
  ```
  **Commit**: YES
  - Message: `refactor(client): 简化 ServiceRegistry 为轻量元数据查询`
  - Files: `crates/openlark-client/src/registry/*`
  - Pre-commit: `cargo test -p openlark-client`
- [x] 4. 精简 root crate 依赖
  **What to do**:
  - 根据 Task 2 审计结果，将「仅内部使用」的依赖从 root Cargo.toml 移除或改为 optional
  - 预计保留（API 表面需要）：`serde`, `serde_json`, `tokio`, `reqwest`, `chrono`
  - 预计移除重导出：`anyhow`, `async-trait`, `base64`, `futures-util`, `hmac`, `log`, `rand`, `serde_repr`, `sha2`, `urlencoding`, `strum`, `strum_macros`, `thiserror`, `url`, `uuid`, `lark-websocket-protobuf`, `tracing`, `tracing-subscriber`
  - 移除 examples 专用依赖从主依赖区（`colored`, `clap` 应仅在 `[dev-dependencies]` 或 example 级别）
  - 更新 `src/lib.rs` 移除对应的 `pub use`（如有）
  **Must NOT do**:
  - 不删除 API 表面需要的重导出
  - 不修改子 crate 的依赖
  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 3)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 5
  - **Blocked By**: Task 2
  **References**:
  - `Cargo.toml:186-208` — 当前 23 个重导出依赖
  - `Cargo.toml:211-212` — colored/clap examples 依赖
  - `.sisyphus/evidence/task-2-reexport-audit.md` — Task 2 产出
  **Acceptance Criteria**:
  - [ ] root crate 非 optional 第三方依赖 ≤ 5
  - [ ] `cargo check --all-features` 通过
  - [ ] examples 仍可编译
  **QA Scenarios:**
  ```
  Scenario: 依赖精简验证
    Tool: Bash
    Steps:
      1. cargo check --all-features
      2. cargo check --example websocket_echo_bot --features "communication,websocket" 2>&1 || echo "SKIP if no example"
    Expected Result: 编译通过
    Evidence: .sisyphus/evidence/task-4-deps-slim.txt
  ```
  **Commit**: YES
  - Message: `refactor(root): 精简第三方依赖重导出`
  - Files: `Cargo.toml`, `src/lib.rs`
  - Pre-commit: `cargo check --all-features`
- [x] 5. 全工作区验证
  **What to do**:
  - 运行全工作区编译、测试、clippy
  - 验证 examples 仍可编译
  - 更新 AGENTS.md 中 registry 相关描述（如有变更）
  **Must NOT do**: 不修改生产代码
  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Blocks**: F1-F3
  - **Blocked By**: Task 3, Task 4
  **References**:
  - `crates/openlark-client/AGENTS.md` — registry 相关描述
  **Acceptance Criteria**:
  - [ ] `cargo check --workspace --all-features` 通过
  - [ ] `cargo test --workspace` 全部通过
  - [ ] `cargo clippy --workspace --all-features` 零警告
  **QA Scenarios:**
  ```
  Scenario: 全工作区验证
    Tool: Bash
    Steps:
      1. cargo check --workspace --all-features
      2. cargo test --workspace 2>&1 | tail -5
      3. cargo clippy --workspace --all-features -- -D warnings 2>&1 | tail -3
    Expected Result: 全部通过
    Evidence: .sisyphus/evidence/task-5-workspace-verify.txt
  ```
  **Commit**: YES
  - Message: `docs: 更新 AGENTS.md 反映 registry 简化`
  - Pre-commit: `cargo test --workspace`
## Final Verification Wave (MANDATORY)

- [x] F1. **Plan Compliance Audit** — `oracle`
  Output: `Must Have [N/N] | Must NOT Have [N/N] | VERDICT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N/N] | VERDICT`

- [x] F3. **Scope Fidelity Check** — `deep`
  Output: `Tasks [N/N compliant] | VERDICT`

---

## Commit Strategy

- **Wave 2a**: `refactor(client): 简化 ServiceRegistry 为轻量元数据查询`
- **Wave 2b**: `refactor(root): 精简第三方依赖重导出，仅保留 API 表面需要的类型`
- **Wave 3**: `test: 全工作区编译验证`

---

## Success Criteria

### Verification Commands
```bash
cargo check --workspace --all-features  # Expected: pass
cargo test --workspace                   # Expected: all pass
ls crates/openlark-client/src/registry/  # Expected: ≤2 files
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
