# OpenLark Feature Matrix

本文档描述 OpenLark 的 feature 组合测试策略和维护指南。

## 背景

OpenLark 采用模块化架构，提供 50+ 个 feature flags 支持按需编译。为确保跨模块组合在发布前后不会出现回归，我们将常用 feature 组合纳入 CI 自动化校验。

## 目标

- 覆盖 `no-default-features`、默认 features、常见业务组合和 `all-features`
- 为新增 feature 或 feature 重构提供回归保护
- 明确 feature 组合清单的维护策略

## 相关文件

- [ci.yml](../.github/workflows/ci.yml) - CI 配置，包含 feature 组合测试
- [feature-matrix.yml](../.github/workflows/feature-matrix.yml) - 高级 feature 矩阵测试
- [Cargo.toml](../Cargo.toml) - Feature 定义

## 参考

- Issue #47 - 添加 CI checks 用于常用 feature 组合
- [Cargo Features 文档](https://doc.rust-lang.org/cargo/reference/features.html)
- [cargo-hack 文档](https://github.com/taiki-e/cargo-hack)
