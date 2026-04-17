# Changelog Compatibility Categories

本文档定义 OpenLark 后续版本在 changelog / release note 中使用的兼容性分类结构。

目标：让用户能快速识别 **typed API、helper、breaking changes** 的影响范围，而不必逐段猜测升级风险。

## 1. 推荐分类结构

从 `0.15.x` 开始，版本条目中的兼容性相关内容建议按以下顺序出现：

```md
## [x.y.z] - YYYY-MM-DD

### Compatibility

#### Typed APIs
- ...

#### Helpers & Convenience Methods
- ...

#### Breaking Changes
- ...

#### Deprecations
- ...

#### Migration Notes
- ...

### Added
- ...

### Changed
- ...

### Fixed
- ...
```

说明：

- `Compatibility` 是**影响升级判断**的优先分类块，应放在常规 `Added / Changed / Fixed` 之前
- 不是每次发布都必须填满所有子分类；没有内容时可省略空小节
- 如果一个变更同时属于多个类别，优先放到最能帮助升级判断的位置

## 2. 每类内容的归档规则

### Typed APIs

用于记录：

- 新增的 typed endpoint / model / builder
- typed API 的兼容扩展
- typed API 的 semver 风险说明

不用于记录：

- 仅内部生成脚本重构
- 与最终用户无关的 schema/生成流程调整

### Helpers & Convenience Methods

用于记录：

- 新增 helper / convenience method
- helper 默认值、分页、歧义处理、返回语义变化
- helper 行为兼容性说明

### Breaking Changes

用于记录：

- 所有需要用户改代码或改升级策略的变化
- 即使函数签名没变，但业务结果会变化的行为调整

规则：

- 这里必须写**受影响 surface**
- 必须说明用户动作
- 如有替代路径，必须同时链接到 migration note

### Deprecations

用于记录：

- 新加的 `#[deprecated]`
- 兼容入口收敛
- support window / earliest removal 信息

### Migration Notes

用于记录：

- before / after 示例
- 替代路径
- 与 `docs/api-compatibility-note-template.md` 或 refactor migration template 的对应关系

## 3. 写作规则

每条 changelog compatibility 项建议至少包含：

1. **surface**：typed API / helper / feature / entrypoint / re-export
2. **change type**：breaking / deprecated / compatible extension
3. **user action**：是否需要迁移

示例：

```md
#### Breaking Changes
- typed API: `TaskListRequest` 将 `page_size` 默认值从 20 调整为 100；如果依赖旧分页窗口，请显式设置 `page_size`
```

## 4. 与 release notes 的关系

- changelog 是长期归档面
- release notes 是发布时的阅读面

因此：

- changelog 应保持分类稳定
- release notes 可以在不改变分类语义的前提下进一步摘要或重排

## 5. 当前模板入口

后续版本发布时，建议直接复用：

- `CHANGELOG.md` 中 `Unreleased` 的 compatibility 分类骨架
- `docs/api-compatibility-note-template.md`
- `docs/api-compatibility-release-checklist.md`
