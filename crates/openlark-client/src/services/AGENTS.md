# Services Registry

Service registry central - 21 files, 90% code ratio.

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Service registration | `mod.rs` | All business services |
| Service traits | Individual service files | Builder patterns |

## CONVENTIONS
- Central registry for all 15+ services
- Each service has separate module
- Builder pattern for all operations
