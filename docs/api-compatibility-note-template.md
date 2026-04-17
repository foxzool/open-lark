# API Compatibility Note Template

本模板用于 future release note / migration guide 中的兼容性说明，和
`docs/PUBLIC_API_STABILITY_POLICY.md` 保持一致。

如果迁移场景是较大的 refactor，而不是简短 release note，请改用：

- `docs/api-refactor-migration-template.md`

## Release Notes 模板

```md
## Compatibility Notes

- Surface: [entrypoint | typed api | helper | feature | re-export]
- Change type: [breaking | deprecated | compatible extension]
- Affected path / feature: `...`
- User action required: ...
- Migration summary: ...
```

## Migration Guide 模板

````md
## API Compatibility Migration

### Before
```rust
// old path / old feature / old helper
```

### After
```rust
// new path / new feature / new helper
```

### Why
- ...

### Removal timeline
- Deprecated in: ...
- Earliest removal: ...
````
