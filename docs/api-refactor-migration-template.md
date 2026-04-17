# API Refactor Migration Guide Template

本模板用于**重构引入 API 迁移**时的专门 migration guide。

与 `docs/api-compatibility-note-template.md` 的区别：

- compatibility note 更偏 release note / 简版兼容性摘要
- 本模板更适合较大 refactor 的完整迁移说明

## 模板

````md
# <Refactor Title> Migration Guide

## 1. 背景

- 这次重构解决了什么问题
- 为什么旧结构需要收敛或替换

## 2. 受影响范围

- Surface: [entrypoint | typed api | helper | feature | re-export]
- Affected path(s): `...`
- Change type: [breaking | deprecated | compatible extension]

## 3. 谁需要关注

- 哪类调用方需要迁移
- 哪类调用方不受影响

## 4. Before / After

### Before
```rust
// old usage
```

### After
```rust
// new usage
```

## 5. 替代方案与决策理由

- 推荐替代路径：`...`
- 如果存在次优替代方案，也应说明何时使用它

## 6. 迁移步骤

1. 更新依赖 / feature / import path
2. 替换旧 API 调用
3. 根据需要调整行为参数 / 默认值
4. 重新运行示例 / 测试 / compile-check

## 7. 兼容期与移除计划

- Deprecated in: `x.y.z`
- Earliest removal: `...`
- Support policy reference: `docs/DEPRECATED_API_SUPPORT_POLICY.md`

## 8. 常见问题 / 风险点

- 迁移时最容易踩到的差异
- 默认值、分页、错误语义、返回类型是否变化

## 9. 验证建议

- 推荐运行的命令
- 推荐检查的行为或示例
````

## 使用说明

- 如果只是一个小型兼容性说明，优先用 `docs/api-compatibility-note-template.md`
- 如果涉及入口重构、helper 语义调整、typed API 路径变化等多步迁移，优先用本模板
