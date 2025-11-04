# OpenLark 代码迁移策略

## 迁移总览

本文档详细描述了从单体 crate 架构迁移到多 crate 架构的具体策略、步骤和风险控制措施。

## 迁移原则

### 1. 渐进式迁移
- 一次只迁移一个功能模块
- 每步都要确保项目可编译和测试通过
- 避免大爆炸式的重构

### 2. 向后兼容性
- 保持所有公共 API 不变
- 现有用户代码无需修改
- Feature flags 系统继续工作

### 3. 风险最小化
- 每个阶段都有回滚计划
- 充分的测试覆盖
- 详细的变更记录

## 迁移阶段规划

### 阶段 0: 准备工作 (1天)

#### 目标
建立新的基础架构，为后续迁移做准备。

#### 具体任务
1. **备份当前代码**
   ```bash
   git checkout -b refactor/multi-crate
   git add -A
   git commit -m "backup: current single-crate state before refactoring"
   ```

2. **创建基础目录结构**
   ```bash
   mkdir -p crates/{open-lark-core,open-lark-communication,open-lark-collaboration,open-lark-hr-suite,open-lark-ai-platform,open-lark-enterprise,open-lark-integrations,open-lark-extensions}
   ```

3. **设置 workspace 配置**
   - 创建根级别的 Cargo.toml (virtual manifest)
   - 配置 workspace members
   - 设置共享依赖版本

#### 验证标准
- [ ] Workspace 配置正确
- [ ] 所有现有目录保持不变
- [ ] 项目仍可正常编译

### 阶段 1: 核心基础设施迁移 (2天)

#### 目标
建立 `open-lark-core` crate，包含所有核心基础设施。

#### 迁移范围
- `src/core/` → `crates/open-lark-core/src/core/`
- `src/client/` → `crates/open-lark-core/src/client/`
- 共享的错误处理、配置、工具函数

#### 具体步骤

**Day 1: 创建 open-lark-core**

1. **创建 crate 基础结构**
   ```bash
   # 创建 Cargo.toml
   cat > crates/open-lark-core/Cargo.toml << 'EOF'
   [package]
   name = "open-lark-core"
   version.workspace = true
   edition.workspace = true

   [dependencies]
   tokio = { workspace = true }
   serde = { workspace = true }
   # ... 其他核心依赖
   EOF

   # 创建 lib.rs
   touch crates/open-lark-core/src/lib.rs
   ```

2. **迁移核心模块**
   - 复制 `src/core/` 内容到 `crates/open-lark-core/src/core/`
   - 复制 `src/client/` 内容到 `crates/open-lark-core/src/client/`
   - 更新内部模块引用

3. **更新主项目依赖**
   - 在根 Cargo.toml 中添加对 `open-lark-core` 的依赖
   - 更新 `src/lib.rs` 中的引用

**Day 2: 集成和测试**

1. **修复编译错误**
   - 更新模块路径
   - 解决依赖问题
   - 调整导出语句

2. **运行测试**
   ```bash
   cargo test --package open-lark-core
   cargo test  # 运行全部测试确保没有破坏
   ```

3. **性能基准测试**
   ```bash
   cargo build --release --features "im"
   # 记录编译时间和包大小
   ```

#### 验证标准
- [ ] `open-lark-core` crate 独立编译成功
- [ ] 所有现有测试通过
- [ ] 功能完整性验证通过
- [ ] 性能基准建立

### 阶段 2: 通讯服务迁移 (2天)

#### 目标
迁移 `im`, `contact`, `group`, `search` 服务到 `open-lark-communication` crate。

#### 迁移范围
- `src/service/im/` → `crates/open-lark-communication/src/services/im/`
- `src/service/contact/` → `crates/open-lark-communication/src/services/contact/`
- `src/service/group/` → `crates/open-lark-communication/src/services/group/`
- `src/service/search/` → `crates/open-lark-communication/src/services/search/`

#### 具体步骤

**Day 1: 创建和迁移**

1. **创建 communication crate**
   ```bash
   cat > crates/open-lark-communication/Cargo.toml << 'EOF'
   [package]
   name = "open-lark-communication"
   version.workspace = true
   edition.workspace = true

   [dependencies]
   open-lark-core = { workspace = true }
   tokio = { workspace = true }
   serde = { workspace = true }
   EOF
   ```

2. **迁移服务代码**
   - 复制相关服务目录
   - 更新模块引用路径
   - 调整依赖声明

3. **更新客户端接口**
   - 修改 `src/client/mod.rs` 中的服务注册
   - 确保新的 crate 被正确引用

**Day 2: 集成和验证**

1. **更新 feature flags**
   ```toml
   # 在根 Cargo.toml 中
   communication = ["open-lark-communication"]
   im = ["communication"]  # legacy mapping
   contact = ["communication"]
   ```

2. **运行集成测试**
   ```bash
   cargo test --features "communication"
   cargo test --features "im"  # 测试 legacy feature
   ```

3. **示例验证**
   ```bash
   cargo run --example im_v1_demo --features "im"
   cargo run --example contact_v3_role_management --features "contact"
   ```

#### 验证标准
- [ ] Communication crate 编译成功
- [ ] 所有通讯相关示例正常运行
- [ ] Legacy feature flags 继续工作
- [ ] 包大小优化效果初步显现

### 阶段 3: 协作服务迁移 (2天)

#### 目标
迁移 `cloud-docs`, `calendar`, `approval`, `task`, `minutes` 到 `open-lark-collaboration` crate。

#### 迁移范围
- `src/service/cloud_docs/`
- `src/service/calendar/`
- `src/service/approval/`
- `src/service/task/`
- `src/service/minutes/`

#### 具体步骤

**Day 1: 创建和迁移**
1. 创建 `open-lark-collaboration` crate
2. 迁移协作相关服务代码
3. 更新依赖关系

**Day 2: 集成和验证**
1. 更新 feature flags 映射
2. 运行协作相关测试
3. 验证示例代码

#### 验证标准
- [ ] Collaboration crate 编译成功
- [ ] 所有协作相关功能正常
- [ ] 新的 feature system 工作
- [ ] 性能测试显示改进

### 阶段 4: HR 服务迁移 (2天)

#### 目标
迁移所有 HR 相关服务到 `open-lark-hr-suite` crate。

#### 迁移范围
- `src/service/hire/`
- `src/service/corehr/`
- `src/service/ehr/`
- `src/service/payroll/`
- `src/service/attendance/`
- `src/service/performance/`
- `src/service/okr/`

#### 验证标准
- [ ] HR suite crate 编译成功
- [ ] 所有 HR 相关功能正常
- [ ] 性能优化效果显著

### 阶段 5: 高级服务迁移 (2天)

#### 目标
迁移 AI、企业管理和集成服务。

#### 迁移范围
- **AI Platform**: `ai`, `aily`, `lingo`, `analytics`
- **Enterprise**: `admin`, `tenant`, `directory`, `security_and_compliance`
- **Integrations**: `mail`, `vc`, `bot`, `application`, `cardkit`
- **Extensions**: 剩余的小型服务

#### 验证标准
- [ ] 所有剩余服务迁移完成
- [ ] 整体功能完整性验证
- [ ] 全面性能测试

### 阶段 6: 统一接口和向后兼容 (1天)

#### 目标
创建统一的对外的 API 接口，确保向后兼容性。

#### 具体任务

1. **创建统一包 (可选)**
   ```rust
   // crates/open-lark/src/lib.rs
   pub use open_lark_core::*;
   pub use open_lark communication::*;
   // ... 其他 crate
   ```

2. **更新客户端生成逻辑**
   ```rust
   // 根据启用的 features 动态构建客户端
   #[cfg(feature = "communication")]
   pub use open_lark_communication::LarkCommunicationClient;

   #[cfg(feature = "collaboration")]
   pub use open_lark_collaboration::LarkCollaborationClient;
   ```

3. **文档更新**
   - 更新 README 中的使用说明
   - 创建迁移指南
   - 更新示例代码

#### 验证标准
- [ ] 现有用户代码无需修改即可编译
- [ ] 所有 feature flags 正常工作
- [ ] 文档完整且准确

## 测试策略

### 1. 单元测试
- 每个 crate 独立测试
- 确保内部逻辑正确性

### 2. 集成测试
- 跨 crate 的功能测试
- API 兼容性测试

### 3. 性能测试
- 编译时间对比
- 包大小测量
- 运行时性能验证

### 4. 兼容性测试
- 现有示例代码验证
- Legacy feature flags 测试
- 用户代码兼容性验证

## 风险控制

### 1. 回滚计划
- 每个阶段完成后创建 git tag
- 保留原始代码分支
- 快速回滚方案

### 2. 质量保证
- 代码审查
- 自动化测试
- 性能基准监控

### 3. 用户沟通
- 迁移进度透明化
- 详细的变更日志
- 用户支持文档

## 预期时间线

| 阶段 | 时间 | 主要成果 |
|------|------|----------|
| 准备工作 | 1天 | 基础架构建立 |
| 核心基础设施 | 2天 | open-lark-core crate |
| 通讯服务 | 2天 | open-lark-communication crate |
| 协作服务 | 2天 | open-lark-collaboration crate |
| HR 服务 | 2天 | open-lark-hr-suite crate |
| 高级服务 | 2天 | 剩余所有服务 |
| 统一接口 | 1天 | 向后兼容性保证 |
| **总计** | **12天** | **完整的多 crate 架构** |

## 成功指标

### 技术指标
- [ ] 编译时间减少 60-80%
- [ ] 包大小优化 30-70%
- [ ] 所有测试通过
- [ ] 零功能回归

### 用户体验指标
- [ ] 现有代码无需修改
- [ ] 文档完整清晰
- [ ] 示例代码正常运行
- [ ] 社区反馈积极

这个迁移策略确保了在获得技术收益的同时，最大程度地降低了对用户的影响。