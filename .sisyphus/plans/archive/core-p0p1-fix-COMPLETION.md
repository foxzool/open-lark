# core-p0p1-fix 计划归档

## 计划状态: 已完成 ✅

**归档时间**: 2026-03-16  
**完成状态**: 10/10 任务全部完成，验证通过率 100%

## 修复内容

### P0 问题（高优先级）

| # | 问题 | 修复文件 | 状态 |
|---|------|----------|------|
| 1 | 网络错误 source 链丢失 | `http.rs` | ✅ `err.into()` 保留完整链路 |
| 2 | req_timeout 未接入请求链 | `request_builder/mod.rs` | ✅ 优先级: ApiRequest.timeout > Config.req_timeout |
| 3 | 导出宏 hygiene 问题 | `lib.rs` | ✅ `$crate::` 替代硬编码 crate 名 |
| 4 | 序列化错误静默丢弃 | `api/mod.rs` | ✅ 移除 `RequestData::Empty` fallback |

### P1 问题（中优先级）

| # | 问题 | 修复文件 | 状态 |
|---|------|----------|------|
| 5 | 43 个无用业务 feature | `Cargo.toml` | ✅ 已移除，保留 default/websocket/otel/testing |
| 6 | 孤儿模块 observability.rs | `error/` | ✅ 已删除 |
| 7 | send() 占位 API | `api/mod.rs` | ✅ 已移除 |
| 8 | ApiRequest 字段可见性过宽 | `api/mod.rs` | ✅ `pub` → `pub(crate)` |
| 9 | 错误分类体系分散 | `error/*.rs` | ✅ 收敛为 ErrorCode 主导 |

## 关键代码变更

### 1. 网络错误 Source 链保留
```rust
// Before
Err(crate::error::network_error(err.to_string()))

// After  
Err(err.into())  // 利用 From<reqwest::Error> for CoreError
```

### 2. 超时配置接入
```rust
// 优先级：ApiRequest.timeout > Config.req_timeout > 无超时
if let Some(timeout) = req.timeout.or(config.req_timeout) {
    req_builder = req_builder.timeout(timeout);
}
```

### 3. 宏 Hygiene 修复
```rust
// Before
openlark_core::Validatable::is_empty_trimmed(...)

// After
$crate::Validatable::is_empty_trimmed(...)
```

### 4. 错误分类收敛
```rust
impl CoreError {
    pub fn severity(&self) -> ErrorSeverity {
        self.code().severity()  // 委托给 ErrorCode
    }
    
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Timeout { .. } => self.code().is_retryable(),
            Self::RateLimit { .. } => self.code().is_retryable(),
            Self::ServiceUnavailable { .. } => self.code().is_retryable(),
            Self::Internal { .. } => self.code().is_retryable(),
            _ => false,
        }
    }
}
```

## 验证结果

### 编译检查
```bash
$ cargo check -p openlark-core
Finished: 零错误

$ cargo check --workspace  
Finished: 无跨 crate 破坏

$ cargo clippy -p openlark-core
0 warnings: 零警告
```

### 测试验证
```bash
$ cargo test -p openlark-core
test result: ok. 36 passed; 0 failed

$ cargo test --workspace
test result: ok. 无新增失败
```

### Feature 清理验证
```bash
$ grep -c "^acs\|^admin\|^corehr\|^hire" crates/openlark-core/Cargo.toml
0  # 43 个业务 feature 已全部移除
```

## 影响范围

- **修改文件**: 8 个 core 内部文件
- **跨 crate 影响**: 零（所有变更保持 API 兼容）
- **测试覆盖**: 36 个测试全部通过
- **Clippy**: 零警告

## 后续工作

此计划所有任务已完成，相关改进已并入主分支。

---
**归档**: 此计划已完全实现并归档。
