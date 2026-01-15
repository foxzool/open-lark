# Drive File Operations

Cloud drive file management. 16 files, 5 subdirs, 90% code ratio.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Upload/download | `upload_all.rs`, `download.rs` | Chunked upload support |
| Copy/move | `copy.rs`, `move.rs` | File operations |
| Metadata | `metas.rs` | File info retrieval |
| Comments | `comment/` | File commenting system |

## CONVENTIONS
- Upload: prepare → part → finish workflow
- File token-based operations
- Async/await throughout
