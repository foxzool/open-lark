# Task 4: Define message types and models

## Summary
Created `crates/openlark-webhook/src/models.rs` with complete message type definitions and content structures.

## Deliverables
- ✅ `src/models.rs` created with 5 message types
- ✅ MsgType enum: text, post, image, file, interactive (card feature gated)
- ✅ Content structures: TextContent, PostContent, ImageContent, FileContent, InteractiveContent
- ✅ All structures use Serde Serialize/Deserialize
- ✅ InteractiveContent accepts serde_json::Value for card JSON
- ✅ Unit tests verify serialization produces correct JSON format
- ✅ Tests pass: 9 tests (11 with card feature)

## Implementation Details

### MsgType Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Text,
    Post,
    Image,
    File,
    #[cfg(feature = "card")]
    Interactive,
}
```

### Content Structures
- TextContent: { text: String }
- PostContent: { post: String } (rich text JSON string)
- ImageContent: { image_key: String }
- FileContent: { file_key: String }
- InteractiveContent: { card: serde_json::Value } (card feature only)

### Test Coverage
- Text content serialization ✅
- Post content serialization ✅
- Image content serialization ✅
- File content serialization ✅
- MsgType serialization ✅
- Interactive content serialization (with card feature) ✅
- MsgType interactive serialization (with card feature) ✅

## Key Decisions
1. Used `#[serde(rename_all = "snake_case")]` to match 飞书 webhook JSON format
2. InteractiveContent gated behind `card` feature flag
3. All content types have simple constructors for ergonomics
4. Tests verify JSON output matches expected format

## Verification
```bash
cargo test -p openlark-webhook --lib
# Result: 9 passed

cargo test -p openlark-webhook --lib --features "card"
# Result: 11 passed (includes interactive tests)
```

## Evidence
- Evidence file: `.sisyphus/evidence/task-04-model-serialization.txt`
- All serialization tests pass
- No diagnostics or warnings

## Status
✅ COMPLETE - Ready for Tasks 7-9 (message sending implementation)
