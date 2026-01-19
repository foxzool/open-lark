---
name: openlark-validation-style
description: OpenLark Rust SDK 的 feature-crate 参数校验规范（必填校验）。当需要统一/评审 `validate()` 写法，或用户询问 `openlark_core::validate_required`（函数）与 `openlark_core::validate_required!`（宏）的区别、是否需要宏、空白字符串处理等问题时使用。
allowed-tools:
  - Read
  - Grep
---

# OpenLark Validation Style

## 目标

在各 feature crate 的请求/Builder `validate(&self) -> SDKResult<()>` 中统一：

- 必填字段校验写法（减少样板代码）
- 空白字符串是否视为缺失（避免不同 crate 行为漂移）
- 失败时返回的错误类型与消息风格

## 规则

### 1) 默认：在 `validate()` 内使用 `openlark_core::validate_required!`（宏）快速失败

适用场景：必填校验失败就应该立即返回 `Err(...)`。

注意：该宏内部用 `.is_empty()` 判空，不会自动 `trim()`。

字符串字段一律传入 `.trim()` 结果，保证空白字符串也视为缺失：

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required!(self.app_token.trim(), "app_token 不能为空");
    openlark_core::validate_required!(self.table_id.trim(), "table_id 不能为空");
    Ok(())
}
```

非字符串容器（如 `Vec<T>`）可直接传字段（按“长度是否为 0”判空）：

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required!(self.items, "items 不能为空");
    Ok(())
}
```

### 2) 需要自定义控制流（不立刻 return / 聚合多条错误）时，使用 `openlark_core::validation::validate_required`（函数）

适用场景：

- 需要做多条校验后统一返回（例如收集多个错误）
- 需要明确的 `trim()` 语义（函数内部 `trim()` 后判空）

注意：函数只返回 `bool`，你需要自己决定错误消息与返回时机：

```rust
fn validate(&self) -> openlark_core::SDKResult<()> {
    if !openlark_core::validation::validate_required(&self.name, "name") {
        return Err(openlark_core::CoreError::validation_msg("name 不能为空"));
    }
    Ok(())
}
```

### 3) 禁止在 feature crate 内重复定义 `validate_required!`（或同语义宏）

统一复用 `openlark_core::validate_required!`，避免各 crate 的判空规则/错误类型不一致。

## 速查

- “校验失败立即返回” → `openlark_core::validate_required!(field[.trim()], "...")`
- “空白也算空” → 字符串传 `.trim()`，或使用函数 `openlark_core::validation::validate_required`
- “多条校验汇总再返回” → 函数 `validate_required` + 自定义聚合逻辑
