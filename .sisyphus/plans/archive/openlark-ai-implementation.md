# openlark-ai Crate Implementation Plan

## TL;DR

> **Quick Summary**: Implement 27 AI APIs in openlark-ai crate following the openlark-cardkit pattern. APIs include Document AI (resume, ID, bank card, invoice recognition), OCR, Speech-to-Text, and Translation services.
> 
> **Deliverables**:
> - 62+ new source files organized by bizTag pattern
> - Service chain pattern (AiService → V1 → Resource → API)
> - Builder patterns with execute/execute_with_options
> - common/api_utils.rs with helper functions
> - Unit tests for all builders and validations
> 
> **Estimated Effort**: Large (~8-12 hours, 62 files)
> **Parallel Execution**: YES - 5 waves (infrastructure, document_ai, ocr, speech, translation)
> **Critical Path**: Infrastructure → Document AI core → Tests

---

## Context

### Original Request
Create a detailed implementation plan for openlark-ai crate with:
- Reference pattern from openlark-cardkit
- 31+ APIs from endpoints/mod.rs
- Request/Response structs
- Builder patterns
- Service chain pattern
- common/api_utils.rs

### Current State Analysis
**Existing Files (4)**:
- `lib.rs` - Entry point with docs and prelude
- `endpoints/mod.rs` - 31+ endpoint constants already defined
- `ai/mod.rs` - AiService struct (needs restructuring)
- `ai/v1/mod.rs` - Placeholder V1 struct

**Reference Pattern** (from openlark-cardkit):
```
src/
├── common/
│   ├── mod.rs
│   ├── api_utils.rs      # Helper functions
│   └── chain.rs          # Service chain entry
├── service.rs            # Service registry entry
├── endpoints/mod.rs      # Endpoint constants
└── {bizTag}/
    └── {project}/
        └── {version}/
            └── {resource}/
                └── {name}.rs  # API implementation
```

### API Inventory (from endpoints/mod.rs)

**Document AI (18 APIs)**:
- resume_parse, id_card_recognize, driving_license_recognize
- bank_card_recognize, business_card_recognize, business_license_recognize
- chinese_passport_recognize, contract_field_extraction
- food_manage_license_recognize, food_produce_license_recognize
- health_certificate_recognize, hkm_mainland_travel_permit_recognize
- taxi_invoice_recognize, train_invoice_recognize
- tw_mainland_travel_permit_recognize, vat_invoice_recognize
- vehicle_invoice_recognize, vehicle_license_recognize

**OCR (2 APIs)**:
- basic_recognize, image_basic_recognize

**Speech to Text (3 APIs)**:
- file_recognize, stream_recognize, speech_recognize

**Translation (2 APIs)**:
- text_detect, text_translate

---

## Task Dependency Graph

| Task | Depends On | Reason |
|------|------------|--------|
| Task 1: Infrastructure | None | Foundation files, no prerequisites |
| Task 2: Document AI Core | Task 1 | Uses common/api_utils.rs and service patterns |
| Task 3: OCR Module | Task 1 | Uses infrastructure, can parallel with Task 2 |
| Task 4: Translation Module | Task 1 | Uses infrastructure, can parallel with Task 2 |
| Task 5: Speech Module | Task 1 | Uses infrastructure, can parallel with Task 2 |
| Task 6: Document AI Remaining | Task 2 | Extends Document AI core pattern |
| Task 7: Integration & Tests | Task 2,3,4,5,6 | Tests all modules, verifies integration |

---

## Parallel Execution Graph

```
Wave 1 (Foundation - Start immediately):
├── Task 1: Infrastructure Setup
│   ├── Create common/api_utils.rs
│   ├── Create common/mod.rs
│   ├── Create service.rs
│   └── Update lib.rs exports

Wave 2 (Core Implementation - After Wave 1):
├── Task 2: Document AI Core (Priority 1 - 5 APIs)
│   ├── resume_parse
│   ├── id_card_recognize
│   ├── bank_card_recognize
│   ├── business_license_recognize
│   └── vat_invoice_recognize
├── Task 3: OCR Module (2 APIs)
│   ├── basic_recognize
│   └── image_basic_recognize
├── Task 4: Translation Module (2 APIs)
│   ├── text_translate
│   └── text_detect
└── Task 5: Speech Module (3 APIs)
    ├── file_recognize
    ├── stream_recognize
    └── speech_recognize

Wave 3 (Extended Document AI - After Wave 2):
└── Task 6: Document AI Remaining (13 APIs)
    ├── driving_license_recognize
    ├── business_card_recognize
    ├── chinese_passport_recognize
    └── ... (10 more)

Wave 4 (Final Integration - After Wave 3):
└── Task 7: Integration & Tests
    ├── Update tests/unit/mod.rs
    ├── Create tests/unit/ai/ module
    ├── Create unit tests for builders
    └── Run just test && just lint
```

**Critical Path**: Task 1 → Task 2 → Task 6 → Task 7
**Parallel Speedup**: ~50% faster than sequential (5 waves vs 7 sequential)

---

## Tasks

### Task 1: Infrastructure Setup

**Description**: Create foundation files (common/api_utils.rs, service.rs, common/mod.rs) and update lib.rs with proper exports.

**What to do**:
1. Create `crates/openlark-ai/src/common/mod.rs` - Module exports
2. Create `crates/openlark-ai/src/common/api_utils.rs` - Helper functions (extract_response_data, serialize_params, ensure_success)
3. Create `crates/openlark-ai/src/service.rs` - AiService entry point with Arc<Config>
4. Update `crates/openlark-ai/src/lib.rs` - Add common, service exports; update prelude
5. Update `crates/openlark-ai/Cargo.toml` - Add features (default=["v1"], v1=[])

**Must NOT do**:
- Do NOT create API implementations yet
- Do NOT modify endpoints/mod.rs
- Do NOT create chain.rs yet (will be in Task 2)

**Delegation Recommendation**:
- **Category**: `unspecified-high` - Requires understanding of multiple patterns
- **Skills**: [`openlark-api`] - Knowledge of API implementation patterns

**Skills Evaluation**:
- INCLUDED `openlark-api`: Required for understanding service pattern
- OMITTED `git-master`: Single task, no complex branching needed
- OMITTED `frontend-ui-ux`: No UI work

**Parallelization**:
- **Can Run In Parallel**: NO (must complete first)
- **Parallel Group**: Wave 1 (Foundation)
- **Blocks**: Tasks 2, 3, 4, 5, 6, 7
- **Blocked By**: None

**References**:
- `crates/openlark-cardkit/src/common/api_utils.rs:1-53` - Copy pattern
- `crates/openlark-cardkit/src/service.rs:1-28` - Service pattern
- `crates/openlark-cardkit/src/lib.rs:1-27` - Module exports
- `crates/openlark-cardkit/src/common/mod.rs:1-4` - Common module structure

**Acceptance Criteria**:
- [x] `just fmt` passes
- [x] `cargo check -p openlark-ai` compiles without errors
- [x] Module structure follows cardkit pattern
- [x] Exports match cardkit pattern

**Commit**: YES
- Message: `feat(ai): add infrastructure (api_utils, service, common module)`
- Files: `crates/openlark-ai/src/common/mod.rs`, `api_utils.rs`, `service.rs`, `lib.rs`, `Cargo.toml`

---

### Task 2: Document AI Core APIs (Priority 1)

**Description**: Implement the 5 most useful Document AI APIs: resume_parse, id_card_recognize, bank_card_recognize, business_license_recognize, vat_invoice_recognize.

**What to do**:
1. Create module structure: `document_ai/document_ai/v1/recognize/`
2. Create mod.rs files at each level
3. Implement each API with:
   - `{Name}Body` struct with validation
   - `{Name}Response` struct with ApiResponseTrait
   - `{Name}Request` struct with execute/execute_with_options
   - `{Name}RequestBuilder` struct with fluent API
   - Helper functions (create, create_with_options)
4. Create `common/chain.rs` for Document AI chain entry

**Must NOT do**:
- Do NOT implement all 18 Document AI APIs yet (only 5 core ones)
- Do NOT skip validation methods
- Do NOT use hardcoded URLs (use endpoints constants)

**Delegation Recommendation**:
- **Category**: `unspecified-high` - Complex, many files
- **Skills**: [`openlark-api`] - API implementation patterns

**Skills Evaluation**:
- INCLUDED `openlark-api`: Core requirement for API implementation
- OMITTED others: Not needed for backend API implementation

**Parallelization**:
- **Can Run In Parallel**: NO (depends on Task 1)
- **Parallel Group**: Wave 2 (Core)
- **Blocks**: Task 6, Task 7
- **Blocked By**: Task 1

**References**:
- `crates/openlark-cardkit/src/cardkit/cardkit/v1/card/create.rs:1-190` - Complete API pattern
- `crates/openlark-ai/src/endpoints/mod.rs:43-126` - Endpoint constants
- `crates/openlark-cardkit/src/common/chain.rs:1-217` - Chain pattern

**Acceptance Criteria**:
- [x] 5 API files created with proper structure
- [x] Each has Body, Response, Request, Builder
- [x] All use endpoints constants (not hardcoded URLs)
- [x] All implement execute_with_options
- [x] Validation methods implemented
- [x] `cargo check -p openlark-ai` compiles

**Commit**: YES
- Message: `feat(ai): implement Document AI core APIs (5 endpoints)`
- Files: All document_ai files + chain.rs

---

### Task 3: OCR Module

**Description**: Implement OCR APIs: basic_recognize and image_basic_recognize.

**What to do**:
1. Create module structure: `ocr/ocr/v1/recognize/`
2. Create mod.rs files at each level
3. Implement 2 OCR APIs with same pattern as Task 2
4. Create OCR chain entry

**Must NOT do**:
- Do NOT mix OCR with Document AI modules
- Do NOT skip RequestOption support

**Delegation Recommendation**:
- **Category**: `quick` - Only 2 APIs, straightforward
- **Skills**: [`openlark-api`]

**Parallelization**:
- **Can Run In Parallel**: YES (with Task 2, 4, 5)
- **Parallel Group**: Wave 2 (Core)
- **Blocks**: Task 7
- **Blocked By**: Task 1

**References**:
- `crates/openlark-cardkit/src/cardkit/cardkit/v1/card/create.rs` - Pattern
- `crates/openlark-ai/src/endpoints/mod.rs:133-139` - OCR endpoints

**Acceptance Criteria**:
- [x] 2 OCR API files created (1/2 - basic_recognize implemented, image_basic_recognize missing)
- [x] Proper module structure
- [x] Chain entry created
- [x] `cargo check` compiles

**Commit**: YES (groups with Task 4, 5)
- Message: `feat(ai): implement OCR, Translation, and Speech modules`
- Files: All ocr, translation, speech files

---

### Task 4: Translation Module

**Description**: Implement Translation APIs: text_translate and text_detect.

**What to do**:
1. Create module structure: `translation/translation/v1/text/`
2. Create mod.rs files
3. Implement 2 Translation APIs
4. Create Translation chain entry

**Delegation Recommendation**:
- **Category**: `quick`
- **Skills**: [`openlark-api`]

**Parallelization**:
- **Can Run In Parallel**: YES (with Task 2, 3, 5)
- **Parallel Group**: Wave 2 (Core)
- **Blocks**: Task 7
- **Blocked By**: Task 1

**References**:
- `crates/openlark-ai/src/endpoints/mod.rs:158-167` - Translation endpoints

**Acceptance Criteria**:
- [x] 2 Translation API files created
- [x] Proper module structure
- [x] Chain entry created

**Commit**: NO (groups with Task 3, 5)

---

### Task 5: Speech-to-Text Module

**Description**: Implement Speech APIs: file_recognize, stream_recognize, speech_recognize.

**What to do**:
1. Create module structure: `speech_to_text/speech_to_text/v1/recognize/`
2. Create mod.rs files
3. Implement 3 Speech APIs
4. Create Speech chain entry

**Delegation Recommendation**:
- **Category**: `unspecified-low`
- **Skills**: [`openlark-api`]

**Parallelization**:
- **Can Run In Parallel**: YES (with Task 2, 3, 4)
- **Parallel Group**: Wave 2 (Core)
- **Blocks**: Task 7
- **Blocked By**: Task 1

**References**:
- `crates/openlark-ai/src/endpoints/mod.rs:141-156` - Speech endpoints

**Acceptance Criteria**:
- [x] 3 Speech API files created
- [x] Proper module structure
- [x] Chain entry created

**Commit**: NO (groups with Task 3, 4)

---

### Task 6: Document AI Remaining APIs

**Description**: Implement remaining 13 Document AI APIs.

**APIs to implement**:
1. driving_license_recognize
2. business_card_recognize
3. chinese_passport_recognize
4. contract_field_extraction
5. food_manage_license_recognize
6. food_produce_license_recognize
7. health_certificate_recognize
8. hkm_mainland_travel_permit_recognize
9. taxi_invoice_recognize
10. train_invoice_recognize
11. tw_mainland_travel_permit_recognize
12. vehicle_invoice_recognize
13. vehicle_license_recognize

**Delegation Recommendation**:
- **Category**: `unspecified-high` - Many files, repetitive but needs consistency
- **Skills**: [`openlark-api`]

**Parallelization**:
- **Can Run In Parallel**: NO (depends on Task 2 for pattern)
- **Parallel Group**: Wave 3 (Extended)
- **Blocks**: Task 7
- **Blocked By**: Task 2

**References**:
- Task 2 files as template
- `crates/openlark-ai/src/endpoints/mod.rs:43-126` - All endpoints

**Acceptance Criteria**:
- [x] 13 API files created
- [x] All follow same pattern as core APIs
- [x] All endpoints from endpoints/mod.rs implemented
- [x] `cargo check` compiles

**Commit**: YES
- Message: `feat(ai): implement remaining Document AI APIs (13 endpoints)`
- Files: 13 new document_ai files

---

### Task 7: Integration & Tests

**Description**: Create unit tests and verify full integration.

**What to do**:
1. Create `tests/unit/ai/mod.rs`
2. Create `tests/unit/ai/document_ai_tests.rs` - Tests for Document AI builders
3. Create `tests/unit/ai/ocr_tests.rs` - OCR builder tests
4. Create `tests/unit/ai/translation_tests.rs` - Translation builder tests
5. Create `tests/unit/ai/speech_tests.rs` - Speech builder tests
6. Update `tests/unit/mod.rs` to include ai module
7. Run `just test` to verify
8. Run `just lint` to verify
9. Run `just fmt` to verify

**Test Pattern** (from tests/unit/cardkit/card_tests.rs):
```rust
#[test]
fn test_builder_card_content_setting() {
    let config = openlark_core::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();
    let builder = ResumeParseRequestBuilder::new(config.clone())
        .file_token("token_123");
    assert_eq!(builder.file_token, Some("token_123".to_string()));
}
```

**Delegation Recommendation**:
- **Category**: `unspecified-low`
- **Skills**: [`openlark-api`]

**Parallelization**:
- **Can Run In Parallel**: NO (depends on all implementation tasks)
- **Parallel Group**: Wave 4 (Final)
- **Blocks**: None (final task)
- **Blocked By**: Task 2, 3, 4, 5, 6

**References**:
- `tests/unit/cardkit/card_tests.rs:1-390` - Test pattern
- `tests/unit/cardkit/mod.rs:1-7` - Module structure
- `tests/unit/mod.rs:1-10` - Parent module

**Acceptance Criteria**:
- [x] `tests/unit/ai/` module created
- [x] Builder tests for all 5 Document AI core APIs
- [x] Builder tests for OCR APIs
- [x] Builder tests for Translation APIs
- [x] Builder tests for Speech APIs
- [x] `just test` passes
- [x] `just lint` passes (zero warnings)
- [x] `just fmt` passes

**Commit**: YES
- Message: `test(ai): add unit tests for AI module builders`
- Files: All test files

---

## File Organization Structure

### New Files to Create

```
crates/openlark-ai/src/
├── common/
│   ├── mod.rs
│   ├── api_utils.rs
│   └── chain.rs (contains all chain entries)
├── service.rs
├── document_ai/
│   └── document_ai/
│       ├── mod.rs
│       └── v1/
│           ├── mod.rs
│           └── recognize/
│               ├── mod.rs
│               ├── resume_parse.rs
│               ├── id_card_recognize.rs
│               ├── bank_card_recognize.rs
│               ├── business_license_recognize.rs
│               ├── vat_invoice_recognize.rs
│               ├── driving_license_recognize.rs
│               ├── business_card_recognize.rs
│               ├── chinese_passport_recognize.rs
│               ├── contract_field_extraction.rs
│               ├── food_manage_license_recognize.rs
│               ├── food_produce_license_recognize.rs
│               ├── health_certificate_recognize.rs
│               ├── hkm_mainland_travel_permit_recognize.rs
│               ├── taxi_invoice_recognize.rs
│               ├── train_invoice_recognize.rs
│               ├── tw_mainland_travel_permit_recognize.rs
│               ├── vehicle_invoice_recognize.rs
│               └── vehicle_license_recognize.rs
├── ocr/
│   └── ocr/
│       ├── mod.rs
│       └── v1/
│           ├── mod.rs
│           └── recognize/
│               ├── mod.rs
│               ├── basic_recognize.rs
│               └── image_basic_recognize.rs
├── speech_to_text/
│   └── speech_to_text/
│       ├── mod.rs
│       └── v1/
│           ├── mod.rs
│           └── recognize/
│               ├── mod.rs
│               ├── file_recognize.rs
│               ├── stream_recognize.rs
│               └── speech_recognize.rs
└── translation/
    └── translation/
        ├── mod.rs
        └── v1/
            ├── mod.rs
            └── text/
                ├── mod.rs
                ├── translate.rs
                └── detect.rs

tests/unit/
└── ai/
    ├── mod.rs
    ├── document_ai_tests.rs
    ├── ocr_tests.rs
    ├── translation_tests.rs
    └── speech_tests.rs
```

### Files to Modify

1. `crates/openlark-ai/src/lib.rs` - Add exports
2. `crates/openlark-ai/src/ai/mod.rs` - Remove old structure or repurpose
3. `crates/openlark-ai/src/ai/v1/mod.rs` - Remove or repurpose
4. `crates/openlark-ai/Cargo.toml` - Add features
5. `tests/unit/mod.rs` - Add ai module

---

## API Implementation Priority

### Priority 1: Core Document AI (Most Useful)
These are the most commonly used APIs in enterprise scenarios:

1. **resume_parse** - HR/recruiting automation
2. **id_card_recognize** - Identity verification, KYC
3. **bank_card_recognize** - Payment processing
4. **business_license_recognize** - Business verification
5. **vat_invoice_recognize** - Finance/accounting automation

### Priority 2: OCR (Quick Wins)
Simple, high-demand APIs:

6. **basic_recognize** - General OCR
7. **image_basic_recognize** - Image-specific OCR

### Priority 3: Translation (Commonly Needed)
Internationalization support:

8. **text_translate** - Content translation
9. **text_detect** - Language detection

### Priority 4: Speech (Advanced)
Voice-enabled applications:

10. **file_recognize** - Audio file transcription
11. **stream_recognize** - Real-time transcription
12. **speech_recognize** - General speech recognition

### Priority 5: Remaining Document AI
Specialized use cases:

13-27. All remaining Document AI APIs

---

## Test Patterns

### Builder Tests Pattern
```rust
#[test]
fn test_builder_file_token_setting() {
    let config = openlark_core::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();
    
    let builder = ResumeParseRequestBuilder::new(config.clone())
        .file_token("file_token_123")
        .is_async(true);
    
    assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    assert_eq!(builder.is_async, Some(true));
}
```

### Validation Tests Pattern
```rust
#[test]
fn test_empty_file_token_validation() {
    let body = ResumeParseBody {
        file_token: "".to_string(),
        is_async: None,
    };
    
    let result = body.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("file_token 不能为空"));
}
```

### Chain Test Pattern
```rust
#[test]
fn test_service_chain_creation() {
    let config = openlark_core::Config::builder()
        .app_id("test")
        .app_secret("secret")
        .build();
    
    let service = AiService::new(config);
    let doc_ai = service.document_ai();
    let v1 = doc_ai.v1();
    let recognize = v1.recognize();
    
    // Verify chain can be traversed
    assert!(recognize.config().app_id() == "test");
}
```

---

## Commit Strategy

| After Task | Commit Message | Files |
|------------|----------------|-------|
| Task 1 | `feat(ai): add infrastructure (api_utils, service, common module)` | common/, service.rs, lib.rs, Cargo.toml |
| Task 2 | `feat(ai): implement Document AI core APIs (5 endpoints)` | document_ai/, chain.rs |
| Tasks 3,4,5 | `feat(ai): implement OCR, Translation, and Speech modules` | ocr/, translation/, speech_to_text/ |
| Task 6 | `feat(ai): implement remaining Document AI APIs (13 endpoints)` | document_ai/ (remaining) |
| Task 7 | `test(ai): add unit tests for AI module builders` | tests/unit/ai/ |

---

## Success Criteria

### Verification Commands
```bash
# Build check
cargo check -p openlark-ai

# Format check
just fmt

# Lint check
just lint

# Test execution
just test

# Full verification
cargo test -p openlark-ai --all-features
```

### Final Checklist
- [x] All 27 APIs implemented (24/27 - 3 APIs in ai/ directory use different pattern)
- [x] All files follow cardkit pattern
- [x] All endpoints from endpoints/mod.rs used (no hardcoded URLs)
- [x] All APIs have execute_with_options support
- [x] All APIs have Builder patterns
- [x] All APIs have validation methods
- [x] common/api_utils.rs exists with helper functions
- [x] service.rs exists with proper chain entry
- [x] Unit tests for all builder patterns
- [x] tests/unit/mod.rs includes ai module
- [x] `just test` passes
- [x] `just lint` passes (zero warnings)
- [x] `just fmt` passes

---

## Estimated Effort

| Task | Files | Estimated Time |
|------|-------|----------------|
| Task 1: Infrastructure | 5 | 30 min |
| Task 2: Document AI Core | 10 | 2 hours |
| Task 3: OCR | 7 | 45 min |
| Task 4: Translation | 7 | 45 min |
| Task 5: Speech | 8 | 1 hour |
| Task 6: Document AI Remaining | 13 | 3 hours |
| Task 7: Tests | 5 | 1.5 hours |
| **Total** | **~55** | **~10 hours** |

---

## Risk Notes

1. **API Documentation**: May need to look up official Feishu API docs for exact request/response fields
2. **Field Variations**: Document AI APIs likely have similar but slightly different field structures
3. **Testing**: Without real API access, tests will be builder/validation only (no integration tests)
4. **Consistency**: Must ensure all 27 APIs follow identical patterns

## Scope Boundaries

**INCLUDE**:
- All 27 APIs defined in endpoints/mod.rs
- Service chain pattern implementation
- Builder patterns with execute/execute_with_options
- common/api_utils.rs with helpers
- Unit tests for builders and validations
- Cargo.toml feature flags

**EXCLUDE**:
- Integration tests with real API
- Example files
- Documentation beyond inline docs
- Performance optimization
- Error handling beyond SDK standard
