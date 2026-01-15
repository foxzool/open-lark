# Core Infrastructure

Core infra: HTTP, config, error handling, validation. 17 files, 11 subdirs, 90% code ratio.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Error codes | `error/` | 8 files, 30+ codes |
| Validation | `validation/` | 13 files |
| HTTP client | `http.rs` | Request/response handling |
| Config | `config/` | App/tenant config |

## CONVENTIONS
- Shared Config for multi-client scenarios
- Arc<Config> for thread-safety
- Strict error type system
- No as any / @ts-ignore / @ts-expect-error
