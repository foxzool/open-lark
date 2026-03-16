# Draft: openlark-communication Refactor

## Requirements (Confirmed)
- Fix architectural inconsistencies in `crates/openlark-communication`
- Target Standard: `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs`

## Research Findings
- **Redundant Nesting**: `src/contact/contact`, `src/im/im`, `src/moments/moments` exist. Needs flattening.
- **Misplaced Module**: `aily` (AI Learning) is in `communication`, but `crates/openlark-ai` exists.
- **Fragmented Modules**: `contact_search` and `contact_user` are outside the main `contact` module.

## Open Questions for User
1. **Aily Migration**: Should I move `src/aily` to the `openlark-ai` crate?
2. **Contact Sub-modules**: Should `contact_search` and `contact_user` be merged into the main `contact` module?
3. **Compatibility**: Do you need backward compatibility (re-exports) for the old paths, or is a clean break preferred?
4. **Test Infrastructure**: Are there specific integration tests I should run to verify the refactor?
