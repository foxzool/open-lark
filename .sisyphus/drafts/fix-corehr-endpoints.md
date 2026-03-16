# 修复 CoreHR API 端点定义

## 问题
需要在 `api_endpoints.rs` 中添加 3 个缺失的端点定义，并修复相关文件中的语法错误。

## 任务

### 1. 添加枚举变体（已完成）
在 `FeishuPeopleApiV1` 枚举中添加：
- `ProcessFormVariableDataGet`
- `TransferReasonQuery`
- `TransferTypeQuery`

### 2. 添加 match 实现（已完成）
在 `to_url()` match 语句中添加对应的 URL 生成逻辑。

### 3. 修复语法错误（待执行）
修复以下文件中的 `u0026self` 为 `&self`：
- `form_variable_data/get.rs`
- `transfer_reason/query.rs`
- `transfer_type/query.rs`

## 执行命令

```bash
# 修复转义字符
sed -i 's/u0026self/\u0026self/g' crates/openlark-hr/src/feishu_people/corehr/v1/process/form_variable_data/get.rs
crates/openlark-hr/src/feishu_people/corehr/v1/transfer_reason/query.rs
crates/openlark-hr/src/feishu_people/corehr/v1/transfer_type/query.rs

# 验证编译
cargo check --all-features -p openlark-hr
```

## 验证
- [ ] `cargo check --all-features -p openlark-hr` 编译通过
- [ ] `cargo test --all-features -p openlark-hr` 测试通过
