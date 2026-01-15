# PROJECT KNOWLEDGE BASE

**Generated:** 2026-01-15 01:26:55
**Commit:** unknown
**Branch:** unknown

## OVERVIEW

Enterprise-grade Rust SDK for Feishu (Lark) APIs with 1,134+ endpoints across 22 specialized crates. Workspace architecture with business-domain organization.

## STRUCTURE

```
crates/
├── openlark-core/        # Core infra: HTTP, config, error handling
├── openlark-protocol/     # WebSocket protocol definitions
├── openlark-client/       # High-level client + service registry
├── openlark-auth/         # Authentication services
├── openlark-communication/ # IM, contacts, groups (300+ APIs)
├── openlark-docs/         # Cloud docs, spreadsheets, wiki (254 APIs)
├── openlark-hr/           # HR: attendance, hiring, CoreHR (153 APIs)
├── openlark-ai/           # AI services & assistants
├── openlark-meeting/       # Video conferencing (75 APIs)
├── openlark-mail/         # Email services (30 APIs)
├── openlark-helpdesk/      # Helpdesk services (47 APIs)
├── openlark-cardkit/       # CardKit components (25 APIs)
├── openlark-security/      # Security services
├── openlark-application/   # Application management
├── protobuf/              # Generated protobuf bindings
└── openlark-admin/        # Admin functions
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add new API | `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs` | Follow path strictly |
| Builder pattern usage | `crates/openlark-client/src/services/` | 21 files, service definitions |
| API endpoints | `crates/openlark-docs/src/common/api_endpoints.rs` | Enum-based mapping |
| Error handling | `crates/openlark-core/src/error/` | 8 files, comprehensive error codes |
| Validation logic | `crates/openlark-core/src/validation/` | 13 files |
| IM message ops | `crates/openlark-communication/src/im/im/v1/message/` | 16 files |
| Drive file ops | `crates/openlark-docs/src/ccm/drive/v1/file/` | 16 files |
| Bitable records | `crates/openlark-docs/src/base/bitable/v1/app/table/record/` | 12 files |
| Unit tests | `tests/unit/` | Feature-organized (auth, im, cloud_docs, etc.) |
| Integration tests | `tests/integration/` | End-to-end workflows |
| Property tests | `tests/property/` | Proptest-based robustness |

## CONVENTIONS

### File Organization
- **API Path**: `src/{bizTag}/{project}/{version}/{resource}/{name}.rs`
- **Example**: `src/im/im/v1/message/create.rs`
- **Module**: `mod.rs` in each directory
- **Prelude**: `prelude.rs` for re-exports in major crates

### Naming
- **Modules/Functions**: snake_case
- **Types**: PascalCase
- **Features**: kebab-case (e.g., `cloud-docs`, `communication-core`)

### API Implementation
- Use `Error`, `Client`, `Request`, `Response`, `end_points` from architecture
- Reference `endpoints` constants via `ApiEndpoint` enum
- Never hardcode API URLs
- Builder pattern: `create_xxx_builder() → .execute()`

### Language
- **Comments**: Chinese throughout
- **Commits**: emoji + Chinese messages (e.g., `🐛 fix: 修复 WebSocket 解析问题`)
- **Documentation**: Chinese docs with Rust examples

## ANTI-PATTERNS (THIS PROJECT)

### Deprecated Items
- Old `CoreError` type → use `CoreError` replacement
- Old field types → use `RecordFieldValue` directly
- Old enum variants in docs models

### Forbidden Practices
- **Manual URL construction** - Use `ApiEndpoint` enum constants only
- **Bypassing ServiceRegistry** - Always go through service registry
- **Hardcoded endpoints in lib.rs** - Centralize in `endpoints/` modules
- **Ignoring conditional compilation** - Respect `cfg(feature = "...")` gates
- **Mixing versions** - Don't put v3 logic in v2 directories

### Test Requirements
- All code must have tests
- Tests must not panic under any circumstances
- Property tests for robustness (WebSocket, JSON, HTTP)
- Integration tests for end-to-end workflows

## UNIQUE STYLES

### Enterprise-Grade CI/CD
- 9 specialized GitHub workflows (unusually complex for Rust)
- Custom API consistency tools (`enhanced_api_checker`, `api_compatibility_tester`)
- Feature matrix testing with `cargo-hack`
- Security audits + supply chain analysis
- Multi-stage release with dual publishing (GitHub + crates.io)

### Strict Quality Gates
- Clippy: cognitive complexity 50, type complexity 500 (relaxed for 214k LOC)
- Code coverage: 65% total, 60% per crate minimum
- Dependency security: only approved open-source licenses (deny.toml)
- Zero warnings enforced in CI

### Feature-Based Architecture
- 15+ crates organized by business domain, not technical layer
- Root `src/lib.rs` conditionally re-exports all crates based on features
- 50+ fine-grained feature flags for conditional compilation
- Examples in `examples/` organized by feature (basic/, api/, core/)

### Development Workflow
- Primary task runner: `just` (15+ commands)
- Justfile includes automated release processes
- Build: `just build` → `cargo build --all-features`
- Test: `just test` → `cargo test --all-features`
- Release: `just release 0.x.y` triggers CI

## COMMANDS

```bash
# Development
just build           # Build all features
just test            # Run unit + doc tests
just lint            # Clippy zero-warnings check
just fmt / just fmt-check  # Format/check format
just docs            # Generate local docs
just check-all       # Pre-release checks
just release 0.x.y    # Tag + trigger CI release

# Cargo equivalents
cargo build --all-features
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt
```

## NOTES

### Project Scale
- 1,354 total files (excluding target/)
- 214,660 lines of code
- 100 files >500 lines (complexity hotspots)
- Max depth: 15 directories
- 15 workspace crates

### Key Architectural Decisions
- Root workspace has `src/` directory (unusual for workspaces)
- Re-exports in root `src/lib.rs` for unified interface
- No binary targets (`main.rs`) - pure library SDK
- Tools in `tools/` are Python scripts, not Rust binaries

### Testing Strategy
- Feature-organized unit tests (tests/unit/{auth, im, cloud_docs, ...})
- Property tests for robustness (tests/property/)
- Integration tests for workflows (tests/integration/)
- Inline tests in crates alongside implementation
- Real API integration with credentials when available

### Existing Knowledge Base
- Root: `./AGENTS.md` (59 lines)
- Communication: `crates/openlark-communication/AGENTS.md` (39 lines)
- Docs: `crates/openlark-docs/AGENTS.md` (39 lines)
- Client: `crates/openlark-client/CLAUDE.md`
- Core: `crates/openlark-core/CLAUDE.md`
