# OpenLark Feature Matrix

本文档描述 OpenLark 的 feature 组合测试策略和维护指南。

## 背景

OpenLark 采用模块化架构，提供 50+ 个 feature flags 支持按需编译。为确保跨模块组合在发布前后不会出现回归，我们将常用 feature 组合纳入 CI 自动化校验。

## 目标

- 覆盖 `no-default-features`、默认 features、常见业务组合和 `all-features`
- 为新增 feature 或 feature 重构提供回归保护
- 明确 feature 组合清单的维护策略

## 当前常见组合矩阵

| 组合 | 目的 | 当前回归信号 |
|------|------|--------------|
| `no-default-features` | 验证最小构建不依赖业务模块 | 根 crate `src/lib.rs` 的最小构建测试 + CI matrix |
| `auth,communication` | 最常见消息/通讯录接入 | 根 crate communication 入口测试 + CI matrix |
| `auth,docs-bitable` | Docs helper / README 主路径 | 根 crate docs-bitable helper 测试 + public examples compile-check |
| `auth,docs-drive` | Drive 上传/下载主路径 | 根 crate docs-drive helper 测试 + CI matrix |
| `essential` | 推荐业务组合（auth + communication + docs） | 根 crate `essential` 组合测试 + CI matrix |
| `enterprise` | 企业级主组合（essential + security + hr + workflow） | 根 crate `enterprise` 组合测试 + CI matrix |
| `webhook-full` | 自定义机器人卡片 + 签名组合 | 根 crate webhook-full 组合测试 + CI matrix |
| `communication,websocket` | 长连接消息能力 | CI matrix + public examples compile-check |

## 回归测试入口

本仓库当前有两层 feature 组合回归保护：

1. **根 crate 组合契约测试**
   - 位置：`src/lib.rs`
   - 作用：验证常见组合下的公开入口与关键 helper 类型是否可用
2. **CI feature matrix**
   - 位置：`.github/workflows/ci.yml`
   - 作用：对常见组合执行 `cargo build/test --lib`

本地快速回归命令：

```bash
just test-features-quick
```

完整组合回归命令：

```bash
just test-features
```

## 相关文件

- [ci.yml](../.github/workflows/ci.yml) - CI 配置，包含 feature 组合测试
- [feature-matrix.yml](../.github/workflows/feature-matrix.yml) - 高级 feature 矩阵测试
- [Cargo.toml](../Cargo.toml) - Feature 定义
- [src/lib.rs](../src/lib.rs) - 根 crate 的组合契约测试
- [justfile](../justfile) - 本地 feature 回归入口

## 参考

- Issue #47 - 添加 CI checks 用于常用 feature 组合
- [Cargo Features 文档](https://doc.rust-lang.org/cargo/reference/features.html)
- [cargo-hack 文档](https://github.com/taiki-e/cargo-hack)
