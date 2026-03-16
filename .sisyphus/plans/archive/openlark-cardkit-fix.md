# OpenLark CardKit Issues Fix Work Plan

## Context

### Issues Summary
1. **Empty macros.rs**: Only contains a comment line, no actual macros
2. **Insufficient test coverage**: Only endpoint tests exist; missing API, validation, and integration tests
3. **Repeated validation code**: Same `card_id.trim().is_empty()` pattern duplicated in 9 files
4. **missing_docs allow**: Blanket `#![allow(missing_docs)]` suppresses doc requirements

### Current State Analysis
- **Crate**: `openlark-cardkit`
- **APIs**: 25+ CardKit APIs
- **Pattern**: Builder pattern for API requests
- **Validation**: Uses `openlark_core::error::validation_error`
- **Validation utilities**: `openlark_core::validation::validate_required` already exists

### Files Affected by Validation Duplication
| File | Validations |
|------|-------------|
| `card/settings.rs` | card_id |
| `card/update.rs` | card_id |
| `card/batch_update.rs` | card_id |
| `card/id_convert.rs` | source_id_type, target_id_type, card_ids |
| `card/element/content.rs` | card_id, element_id |
| `card/element/create.rs` | card_id (x2) |
| `card/element/update.rs` | card_id, element_id |
| `card/element/delete.rs` | card_id, element_id |
| `card/element/patch.rs` | card_id, element_id |

---

## Task Dependency Graph

| Task ID | Task Name | Depends On | Reason |
|---------|-----------|------------|--------|
| 1 | Delete/Fix macros.rs | None | Independent decision task |
| 2 | Create validation utilities module | None | Independent foundation task |
| 3 | Replace validation code with utilities | Task 2 | Requires utilities module to exist |
| 4 | Add API call tests | None | Can be done in parallel with validation refactor |
| 5 | Add validation tests | Task 3 | Tests the refactored validation |
| 6 | Add documentation | Tasks 2,3 | Docs for new utilities and public APIs |
| 7 | Remove missing_docs allow | Task 6 | Only after docs are added |

---

## Parallel Execution Graph

### Wave 1: Foundation (Start Immediately)
```
├── Task 1: Handle macros.rs (delete or implement)
│   └── Decision: Delete file (no macros used)
│
├── Task 2: Create validation utilities module
│   └── Create: src/common/validation.rs
│   └── Export in: src/common/mod.rs
│
└── Task 4: Add API call tests (independent)
    └── Create: tests/unit/cardkit/
```

### Wave 2: Refactor & Test (After Wave 1)
```
├── Task 3: Replace validation code with utilities
│   └── Depends: Task 2 (validation utilities exist)
│   └── Files: 9 API files
│   └── Pattern: Replace inline validation with utility calls
│
└── Task 5: Add validation tests
    └── Depends: Task 3 (validation logic refactored)
    └── Test: validation utilities edge cases
```

### Wave 3: Documentation & Cleanup (After Wave 2)
```
└── Task 6: Add documentation
    └── Depends: Tasks 2,3 (utilities exist and integrated)
    └── Add docs to: All public types and functions
    └── Remove: #![allow(missing_docs)]
```

**Critical Path**: Task 2 → Task 3 → Task 5 → Task 6
**Parallel Speedup**: ~35% faster than sequential

---

## Tasks

### Task 1: Handle Empty macros.rs File

**Description**: Decide whether to delete the empty macros.rs file or add useful macros

**Investigation Results**:
- File: `crates/openlark-cardkit/src/macros.rs`
- Content: Only 2 lines (comment + empty line)
- Usage: Referenced in `lib.rs` with `#[macro_use] mod macros;`
- Search result: No macros from this module are used anywhere in the crate

**Decision**: **DELETE the file**
- No macros defined
- No macros used
- Other crates don't rely on cardkit macros

**Acceptance Criteria**:
- [x] Delete `crates/openlark-cardkit/src/macros.rs`
- [x] Remove `#[macro_use] mod macros;` from `lib.rs`
- [x] Run `cargo build --package openlark-cardkit` → SUCCESS
- [x] Run `cargo test --package openlark-cardkit` → All tests pass

**Delegation Recommendation**:
- **Category**: `quick` - Simple file deletion and cleanup
- **Skills**: [`git-master`] - For atomic commit
- **Reasoning**: This is a trivial cleanup task requiring no complex logic

---

### Task 2: Create Validation Utilities Module

**Description**: Create a shared validation utilities module to eliminate code duplication

**Current Pattern (Duplicated)**:
```rust
if body.card_id.trim().is_empty() {
    return Err(openlark_core::error::validation_error(
        "card_id 不能为空",
        "card_id 不能为空",
    ));
}
```

**Solution**: Create `crates/openlark-cardkit/src/common/validation.rs`

**Implementation Requirements**:
```rust
//! CardKit 共享验证工具
//!
//! 提供卡片实体和组件相关的通用验证函数。

use openlark_core::error;

/// 验证卡片 ID 是否有效
/// 
/// # 参数
/// - `card_id`: 卡片 ID
/// 
/// # 返回
/// - `Ok(())`: 验证通过
/// - `Err`: 验证失败，返回 CoreError
pub fn validate_card_id(card_id: &str) -> Result<(), error::CoreError> {
    if card_id.trim().is_empty() {
        Err(error::validation_error(
            "card_id 不能为空",
            "卡片 ID 不能为空或仅包含空白字符",
        ))
    } else {
        Ok(())
    }
}

/// 验证组件 ID 是否有效
/// 
/// # 参数
/// - `element_id`: 组件 ID
/// 
/// # 返回
/// - `Ok(())`: 验证通过
/// - `Err`: 验证失败，返回 CoreError
pub fn validate_element_id(element_id: &str) -> Result<(), error::CoreError> {
    if element_id.trim().is_empty() {
        Err(error::validation_error(
            "element_id 不能为空",
            "组件 ID 不能为空或仅包含空白字符",
        ))
    } else {
        Ok(())
    }
}

/// 验证 ID 类型是否有效
/// 
/// # 参数
/// - `id_type`: ID 类型字符串
/// - `field_name`: 字段名称（用于错误信息）
/// 
/// # 返回
/// - `Ok(())`: 验证通过
/// - `Err`: 验证失败，返回 CoreError
pub fn validate_id_type(id_type: &str, field_name: &str) -> Result<(), error::CoreError> {
    if id_type.trim().is_empty() {
        Err(error::validation_error(
            format!("{} 不能为空", field_name),
            format!("{} 不能为空或仅包含空白字符", field_name),
        ))
    } else {
        Ok(())
    }
}

/// 验证 ID 列表是否非空
/// 
/// # 参数
/// - `ids`: ID 列表
/// - `field_name`: 字段名称（用于错误信息）
/// 
/// # 返回
/// - `Ok(())`: 验证通过
/// - `Err`: 验证失败，返回 CoreError
pub fn validate_id_list(ids: &[String], field_name: &str) -> Result<(), error::CoreError> {
    if ids.is_empty() {
        Err(error::validation_error(
            format!("{} 不能为空", field_name),
            format!("{} 必须包含至少一个 ID", field_name),
        ))
    } else {
        Ok(())
    }
}
```

**Integration**:
- [x] Create file: `src/common/validation.rs`
- [x] Add to `src/common/mod.rs`: `pub mod validation;`
- [x] Add re-export in `src/common/mod.rs`: `pub use validation::{validate_card_id, validate_element_id, validate_id_type, validate_id_list};`

**Acceptance Criteria**:
- [x] Module compiles without errors
- [x] All 4 validation functions are exported
- [x] Functions return `Result<(), CoreError>`
- [x] Error messages are in Chinese (consistent with codebase)
- [x] Unit tests added for each validation function

**Delegation Recommendation**:
- **Category**: `unspecified-low` - Moderate effort, straightforward implementation
- **Skills**: [] - No special skills needed, standard Rust
- **Reasoning**: Pure Rust code creation following existing patterns

---

### Task 3: Replace Inline Validation with Utilities

**Description**: Replace all inline `trim().is_empty()` validation with utility function calls

**Files to Modify**:

#### 3.1: `src/cardkit/cardkit/v1/card/settings.rs`
**Current**:
```rust
if body.card_id.trim().is_empty() {
    return Err(openlark_core::error::validation_error(
        "card_id 不能为空",
        "card_id 不能为空",
    ));
}
```
**Change to**:
```rust
use crate::common::validation::validate_card_id;
// ...
validate_card_id(&body.card_id)?;
```

#### 3.2: `src/cardkit/cardkit/v1/card/update.rs`
- Replace card_id validation (line 92-97)
- Add import: `use crate::common::validation::validate_card_id;`

#### 3.3: `src/cardkit/cardkit/v1/card/batch_update.rs`
- Replace card_id validation (line 76-82)
- Add import

#### 3.4: `src/cardkit/cardkit/v1/card/id_convert.rs`
- Replace source_id_type validation (line 71-76)
- Replace target_id_type validation (line 77-82)
- Replace card_ids validation (line 83-88)
- Use `validate_id_type` and `validate_id_list`
- **Note**: Has 2 versions (body struct vs builder), update both

#### 3.5: `src/cardkit/cardkit/v1/card/element/content.rs`
- Replace card_id validation (line 73-78)
- Replace element_id validation (line 79-84)
- Use `validate_card_id` and `validate_element_id`

#### 3.6: `src/cardkit/cardkit/v1/card/element/create.rs`
- Replace card_id validation in builder (line 61-66)
- Replace card_id validation in standalone function (line 135-140)

#### 3.7: `src/cardkit/cardkit/v1/card/element/update.rs`
- Replace card_id validation (line 73-78)
- Replace element_id validation (line 79-84)

#### 3.8: `src/cardkit/cardkit/v1/card/element/delete.rs`
- Replace card_id validation (line 64-69)
- Replace element_id validation (line 70-75)

#### 3.9: `src/cardkit/cardkit/v1/card/element/patch.rs`
- Replace card_id validation (line 67-72)
- Replace element_id validation (line 73-78)

**Pattern for Each File**:
1. Add import at top: `use crate::common::validation::{validate_card_id, validate_element_id};`
2. Find `if ...trim().is_empty()` block
3. Replace with appropriate validation function call
4. Use `?` operator to propagate error

**Acceptance Criteria**:
- [x] All 9 files updated
- [x] No more `trim().is_empty()` patterns in cardkit (except validation module)
- [x] All imports added correctly
- [x] `cargo build --package openlark-cardkit` succeeds
- [x] `cargo test --package openlark-cardkit` passes
- [x] `cargo clippy --package openlark-cardkit` shows no warnings

**Delegation Recommendation**:
- **Category**: `unspecified-low` - Repetitive but careful work
- **Skills**: [] - Standard refactoring
- **Reasoning**: Mechanical refactoring following a pattern, requires precision

---

### Task 4: Add API Call Tests

**Description**: Create comprehensive tests for CardKit API calls

**Test File**: `tests/unit/cardkit/mod.rs` and individual test files

**Test Structure**:
```
tests/unit/cardkit/
├── mod.rs              # Test module entry
├── card_tests.rs       # Card entity API tests
└── element_tests.rs    # Card element API tests
```

**Test Coverage Requirements**:

#### 4.1: Card Entity Tests (`card_tests.rs`)
- [x] `test_create_card_request_builder` - Test builder pattern
- [x] `test_create_card_validation` - Test card_content validation
- [x] `test_update_card_request_builder` - Test update builder
- [x] `test_update_card_card_id_validation` - Test card_id validation
- [x] `test_batch_update_card_validation` - Test batch_update validation
- [x] `test_settings_card_validation` - Test settings card_id validation
- [x] `test_id_convert_validation` - Test id_convert parameter validation

#### 4.2: Card Element Tests (`element_tests.rs`)
- [x] `test_create_element_validation` - Test element create validation
- [x] `test_update_element_validation` - Test element update validation
- [x] `test_delete_element_validation` - Test element delete validation
- [x] `test_patch_element_validation` - Test element patch validation
- [x] `test_content_element_validation` - Test content element validation

**Test Pattern Example**:
```rust
#[test]
fn test_validate_card_id_empty() {
    use openlark_cardkit::common::validation::validate_card_id;
    
    let result = validate_card_id("");
    assert!(result.is_err());
    
    let result = validate_card_id("   ");
    assert!(result.is_err());
}

#[test]
fn test_validate_card_id_valid() {
    use openlark_cardkit::common::validation::validate_card_id;
    
    let result = validate_card_id("card_123");
    assert!(result.is_ok());
    
    let result = validate_card_id("  card_123  ");
    assert!(result.is_ok()); // trim should handle this
}
```

**Mock Server Tests** (if infrastructure exists):
- Use `wiremock` for HTTP mocking (check other test files)
- Test successful API calls
- Test error responses
- Test request body serialization

**Acceptance Criteria**:
- [x] Test files created in `tests/unit/cardkit/`
- [x] `mod.rs` exports all test modules
- [x] At least 12 test cases covering all validation scenarios
- [x] All tests pass: `cargo test --package openlark-cardkit`
- [x] Tests follow existing patterns from `tests/unit/im/`

**Delegation Recommendation**:
- **Category**: `unspecified-low` - Moderate effort test writing
- **Skills**: [] - Standard Rust testing
- **Reasoning**: Following existing test patterns, no special domain knowledge needed

---

### Task 5: Add Validation Tests

**Description**: Add specific tests for the new validation utilities

**Test File**: `crates/openlark-cardkit/src/common/validation.rs` (inline tests)

**Test Cases Required**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_card_id_empty() {
        assert!(validate_card_id("").is_err());
        assert!(validate_card_id("   ").is_err());
        assert!(validate_card_id("\t\n").is_err());
    }

    #[test]
    fn test_validate_card_id_valid() {
        assert!(validate_card_id("card_123").is_ok());
        assert!(validate_card_id("  card_123  ").is_ok());
        assert!(validate_card_id("card-abc-123").is_ok());
    }

    #[test]
    fn test_validate_element_id_empty() {
        assert!(validate_element_id("").is_err());
        assert!(validate_element_id("   ").is_err());
    }

    #[test]
    fn test_validate_element_id_valid() {
        assert!(validate_element_id("elem_456").is_ok());
        assert!(validate_element_id("element_abc").is_ok());
    }

    #[test]
    fn test_validate_id_type_empty() {
        assert!(validate_id_type("", "source_id_type").is_err());
        assert!(validate_id_type("   ", "target_id_type").is_err());
    }

    #[test]
    fn test_validate_id_type_valid() {
        assert!(validate_id_type("card_id", "field").is_ok());
        assert!(validate_id_type("card_open_id", "field").is_ok());
    }

    #[test]
    fn test_validate_id_list_empty() {
        assert!(validate_id_list(&[], "card_ids").is_err());
    }

    #[test]
    fn test_validate_id_list_valid() {
        assert!(validate_id_list(&["id1".to_string()], "card_ids").is_ok());
        assert!(validate_id_list(&["id1".to_string(), "id2".to_string()], "card_ids").is_ok());
    }

    #[test]
    fn test_error_messages() {
        let err = validate_card_id("").unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("card_id"));
        assert!(err_msg.contains("不能为空"));
    }
}
```

**Acceptance Criteria**:
- [x] All 4 validation functions have tests
- [x] Edge cases covered (empty, whitespace, valid)
- [x] Error message content verified
- [x] Tests pass: `cargo test --package openlark-cardkit validation`

**Delegation Recommendation**:
- **Category**: `quick` - Straightforward test cases
- **Skills**: [] - Standard unit testing
- **Reasoning**: Simple test writing with clear expected outcomes

---

### Task 6: Add Documentation and Remove missing_docs Allow

**Description**: Add comprehensive documentation to all public types and remove the blanket `missing_docs` allow

**Files to Document**:

#### 6.1: `src/lib.rs`
- Add module-level documentation
- Document re-exports
- Remove `#![allow(missing_docs)]`

#### 6.2: `src/common/validation.rs` (new file)
- Module documentation (already provided in Task 2)
- Document each public function

#### 6.3: `src/common/mod.rs`
- Document the common module
- Document re-exports

#### 6.4: `src/endpoints/mod.rs`
- Document endpoint constants
- Document helper functions

#### 6.5: `src/service.rs`
- Document CardkitService
- Document public methods

#### 6.6: `src/common/chain.rs`
- Document CardkitClient
- Document chain methods

#### 6.7: `src/cardkit/cardkit/v1/card/*.rs`
- Document all public structs and their fields
- Document all public functions
- Document request builders

#### 6.8: `src/cardkit/cardkit/v1/card/element/*.rs`
- Document all public structs and their fields
- Document all public functions
- Document request builders

**Documentation Pattern**:
```rust
/// 创建卡片实体请求体
/// 
/// 用于创建新的卡片实体，包含卡片内容、类型、模板等信息。
/// 
/// # 示例
/// ```
/// use openlark_cardkit::cardkit::cardkit::v1::card::create::CreateCardBody;
/// 
/// let body = CreateCardBody {
///     card_content: serde_json::json!({"type": "div", "text": "Hello"}),
///     card_type: Some("interactive".to_string()),
///     template_id: None,
///     temp: None,
///     temp_expire_time: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardBody {
    /// 卡片内容，必须是一个有效的 JSON 对象
    pub card_content: serde_json::Value,
    /// 卡片类型，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    // ... etc
}
```

**Acceptance Criteria**:
- [x] `#![allow(missing_docs)]` removed from `lib.rs`
- [x] `cargo doc --package openlark-cardkit` builds without warnings
- [x] All public items have documentation
- [x] All doc comments in Chinese (consistent with codebase)
- [x] Examples included for main structs and functions

**Delegation Recommendation**:
- **Category**: `unspecified-low` - Repetitive documentation work
- **Skills**: [] - Standard Rust doc comments
- **Reasoning**: Following existing documentation patterns in the codebase

---

## Commit Strategy

| After Task | Commit Message | Files | Verification |
|------------|----------------|-------|--------------|
| 1 | `🔧 refactor: 删除空的 macros.rs 文件` | lib.rs (remove import), macros.rs (deleted) | build passes |
| 2 | `✨ feat: 添加 CardKit 共享验证工具模块` | common/validation.rs, common/mod.rs | tests pass |
| 3 | `♻️ refactor: 使用共享验证工具替换重复代码` | 9 API files | clippy clean |
| 4 | `✅ test: 添加 CardKit API 调用测试` | tests/unit/cardkit/** | all tests pass |
| 5 | `✅ test: 添加验证工具单元测试` | common/validation.rs | validation tests pass |
| 6 | `📝 docs: 添加 CardKit 模块文档` | All public modules | doc build passes |

---

## Success Criteria

### Build Verification
```bash
# All builds must succeed
cargo build --package openlark-cardkit
cargo test --package openlark-cardkit
cargo clippy --package openlark-cardkit -- -D warnings
cargo doc --package openlark-cardkit --no-deps
```

### Coverage Verification
- [x] `macros.rs` deleted (Issue 1)
- [x] 12+ new test cases added (Issue 2)
- [x] Validation utilities module created (Issue 3)
- [x] Zero `trim().is_empty()` patterns remaining in API files (Issue 3)
- [x] `#![allow(missing_docs)]` removed (Issue 4)
- [x] Zero doc warnings (Issue 4)

### Final Verification Command
```bash
just test && just lint && just fmt-check
```

---

## Appendix: File Reference

### Source Files to Modify
```
crates/openlark-cardkit/src/
├── lib.rs                              # Remove macros import, remove missing_docs allow
├── macros.rs                           # DELETE
├── common/
│   ├── mod.rs                          # Add validation module export
│   └── validation.rs                   # CREATE - validation utilities
├── endpoints/
│   └── mod.rs                          # Add documentation
├── service.rs                          # Add documentation
├── common/
│   └── chain.rs                        # Add documentation
└── cardkit/cardkit/v1/card/
    ├── settings.rs                     # Use validation utilities
    ├── update.rs                       # Use validation utilities
    ├── batch_update.rs                 # Use validation utilities
    ├── id_convert.rs                   # Use validation utilities
    └── element/
        ├── content.rs                  # Use validation utilities
        ├── create.rs                   # Use validation utilities
        ├── update.rs                   # Use validation utilities
        ├── delete.rs                   # Use validation utilities
        └── patch.rs                    # Use validation utilities
```

### Test Files to Create
```
tests/unit/cardkit/
├── mod.rs                              # CREATE
├── card_tests.rs                       # CREATE
└── element_tests.rs                    # CREATE
```

**Total Changes**:
- 1 file deleted
- 4 files created
- 11 files modified
- ~300 lines of code added (validation + tests)
