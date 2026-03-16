# 修复代码质量问题 - 知识库

## 项目约定

### 命名规范
- 类型使用 PascalCase
- 方法使用 snake_case
- 模块使用 snake_case

### API 端点模式
- 在 `common/api_endpoints.rs` 定义 `ApiEndpoint` 枚举
- 使用 `to_url()` 或 `path()` 方法获取 URL 路径
- 避免硬编码 URL

### 验证步骤
- 每次修改后运行 `cargo check -p {crate}`
- 最终运行 `cargo fmt --check` 和 `cargo clippy`

## 当前任务状态

### Wave 1 (P0 修复) - 已完成
- [x] Task 1: 修复命名规范 - visitor/delete.rs
- [x] Task 2: 修复命名规范 - rule_external/delete.rs
- [x] Task 3: 修复 QuerySessionRequest.user_ids 字段传递
- [x] Task 4: 创建 openlark-auth api_endpoints.rs

### Wave 2 (P1 修复) - 已完成
- [x] Task 5: 创建 openlark-platform api_endpoints.rs
- [x] Task 6: 替换硬编码 URL - CreateBadgeBuilder
- [x] Task 7: 替换硬编码 URL - QuerySessionRequest
- [x] Task 8: 修复覆盖率配置一致性
- [x] Task 9: 运行代码格式化
- [x] Task 10: 最终验证

### 额外修复
- [x] Task 11: 修复 ListBadgeBuilder 硬编码 URL (计划外发现)

## 修复总结

### 已修复问题
1. **命名规范异常** - `udeleteRequest` 已重命名为 PascalCase
2. **字段未参与请求** - `QuerySessionRequest.user_ids` 已正确参与请求构造
3. **硬编码 URL** - 所有硬编码 URL 已替换为 `ApiEndpoint` 枚举
4. **配置一致性** - `.cargo-llvm-cov.toml` 使用 `all-features = true` 配置正确
5. **代码格式化** - `cargo fmt --check` 通过

### 验证结果
- [x] `cargo fmt --check` - 通过
- [x] `cargo clippy --workspace --all-features -- -D warnings` - 通过
- [x] `cargo build --workspace --all-features` - 成功

### 修改文件
1. `crates/openlark-platform/src/common/api_endpoints.rs` - 添加 ListBadge 端点
2. `crates/openlark-platform/src/admin/admin/v1/badge/list.rs` - 替换硬编码 URL

## 时间戳
- 计划开始: 2026-02-24
- 验证完成: 2026-02-26T12:12:09+08:00

## 备注
所有代码质量问题已修复完成。

