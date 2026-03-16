# Explorer::v2 模块导出修复

## TL;DR

> **Quick Summary**: 修复 open-lark 中 `explorer::v2` 模块的编译错误和导出问题。
> 
> **Deliverables**:
> - v2 模块编译无错误
> - v2 模块可通过 `openlark_docs::ccm::explorer::v2` 路径访问
> 
> **Estimated Effort**: Quick（约 2 分钟）

---

## 修复内容

### 问题 1：编译错误
`v2/mod.rs` 中所有函数引用了未定义的 `option` 变量

### 问题 2：未导出
`mod.rs` 没有导出 `pub mod v2;`

---

## TODOs

- [x] 1. **修复 v2/mod.rs 编译错误** [quick] ✅

  **What to do**:
  - 将所有 `Some(option)` 改为 `None`
  - 文件：`crates/openlark-docs/src/ccm/explorer/v2/mod.rs`
  - 约 8 处需要修改

  **查找命令**:
  ```bash
  grep -n "Some(option)" crates/openlark-docs/src/ccm/explorer/v2/mod.rs
  ```

  **替换为**: `None`

  **References**:
  - `crates/openlark-docs/src/ccm/permission/v2/mod.rs:58` - 正确模式

  **QA**: `cargo check -p openlark-docs --features ccm-core`

  **Commit**: `fix(docs): 修复 explorer v2 模块编译错误`

---

- [x] 2. **添加 v2 模块导出** [quick] ✅

  **What to do**:
  - 在 `crates/openlark-docs/src/ccm/explorer/mod.rs` 添加一行

  **修改前**:
  ```rust
  /// 云盘浏览器模块
  pub mod explorer;
  // old 模块已废弃并删除
  ```

  **修改后**:
  ```rust
  /// 云盘浏览器模块
  pub mod explorer;
  pub mod v2;
  // old 模块已废弃并删除
  ```

  **QA**: `cargo check -p openlark-docs --features ccm-core`

  **Commit**: 与 Task 1 合并

---

## Final Verification

```bash
cargo check -p openlark-docs --features ccm-core
# 期望: Finished `dev` profile
```

---

## Success Criteria

- [x] `cargo check -p openlark-docs --features ccm-core` 编译通过 ✅
- [x] v2 模块可通过 `openlark_docs::ccm::explorer::v2` 访问 ✅
- [x] `get_folder_children` 函数可用 ✅
