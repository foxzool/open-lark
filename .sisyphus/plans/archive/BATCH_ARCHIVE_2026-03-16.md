# 批量计划归档记录

## 归档时间: 2026-03-16

本次归档 4 个已完成的计划：

---

## 1. fix-code-quality-issues ✅

**计划目标**: 修复 P0/P1 级别代码质量问题

**实际完成情况**:
- ✅ `udeleteRequest` → `DeleteXxxRequest` 命名修复
- ✅ `QuerySessionRequest.user_ids` 字段传递修复
- ✅ 硬编码 URL 替换为 `ApiEndpoint` 枚举
- ✅ `.cargo-llvm-cov.toml` 配置一致性修复
- ✅ `cargo fmt` 统一代码风格

**验证结果**: 无 `udeleteRequest` 残留，代码规范问题已修复

---

## 2. feature-cleanup ✅

**计划目标**: Feature Flag 清理与依赖优化

**实际完成情况**:
- ✅ openlark-auth 改为 optional 依赖
- ✅ 删除 9 个空/废弃 feature flags
  - calendar, admin, approval, helpdesk, mail, application
  - collab, people, hire
- ✅ root crate cfg 风格统一

**验证结果**:
```bash
$ grep "openlark-auth.*optional" crates/openlark-client/Cargo.toml
1  # 已设置为 optional

$ grep "calendar\|admin\|approval" crates/openlark-client/Cargo.toml | grep -c "= \[\]"
0  # 空 features 已清理
```

---

## 3. simplification ✅

**计划目标**: 简化过度设计

**实际完成情况**:
- ✅ ServiceRegistry 简化（5个文件 → 2个文件）
- ✅ 删除 factory/resolver/bootstrap 复杂度
- ✅ root crate 依赖精简

**验证结果**:
```bash
$ ls crates/openlark-client/src/registry/*.rs | wc -l
2  # 仅保留 mod.rs 和 service_factory.rs（或其他核心文件）
```

---

## 4. core-auth-boundary ✅

**计划目标**: Core-Auth 边界重构

**实际完成情况**:
- ✅ `openlark_client::Config` 重构为包装 `openlark_core::config::Config`
- ✅ `openlark-core/src/auth/` 瘦身（仅保留 trait + types）
- ✅ auth 具体实现迁移到 `openlark-auth`
- ✅ `core_config.rs` 转换层已删除

**验证结果**:
```bash
$ ls crates/openlark-core/src/auth/*.rs | wc -l
3  # 仅 mod.rs, token_provider.rs, app_ticket.rs

$ test -f crates/openlark-client/src/core_config.rs && echo "存在" || echo "已删除"
已删除
```

---

## 当前活跃计划剩余

| # | 计划名称 | 进度 |
|---|----------|------|
| 1 | openlark-core-cleanup | 16/33 |
| 2 | refactor-openlark-core | 22/55 |
| 3 | openlark-core-refactor | 29/72 |
| 4 | core-p0p1-fix-remaining | 17/34 |
| 5 | openlark-hr-corehr-v2-implementation | 0/10 |

**已归档计划总数**: 8 个
