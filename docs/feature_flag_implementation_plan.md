# 功能标志实施计划

## 项目概述

基于zen consensus的分析结果，实施按URL路径映射Cargo功能标志的系统性优化项目。

**现状总结：**
- 总计1551个API，分布在62个服务中
- 37个服务已有对应功能标志（匹配率60%）
- 25个服务需要新增功能标志
- 发现多个命名不匹配的情况

## 分阶段实施计划

### 第一阶段：关键服务优先（第1周）

#### 高优先级服务（立即实施）

这些服务使用频率高，影响范围大：

| 服务 | 现状 | 优先级 | 预计工作量 |
|------|------|--------|------------|
| **authen** → **auth** | 不匹配 | 高 | 2小时 |
| **sheets** | 缺失 | 高 | 1天 |
| **bitable** | 缺失 | 高 | 1天 |
| **docx** → **cloud-docs** | 不匹配 | 高 | 2小时 |
| **drive** → **cloud-docs** | 不匹配 | 高 | 2小时 |

#### 实施任务

1. **authen统一到auth**
   - 修改现有authen相关代码
   - 更新功能标志依赖
   - 添加别名兼容性

2. **新增sheets功能标志**
   - 创建sheets服务模块
   - 实现核心API
   - 添加到主客户端

3. **新增bitable功能标志**
   - 创建bitable服务模块
   - 实现多维表格API
   - 集成到项目架构

4. **云文档服务统一**
   - docx和drive统一使用cloud-docs
   - 重新组织相关代码
   - 更新文档和示例

### 第二阶段：中频服务扩展（第2周）

#### 中优先级服务

| 服务 | 现状 | 优先级 | 预计工作量 |
|------|------|--------|------------|
| **translation** | 缺失 | 中 | 4小时 |
| **speech_to_text** | 缺失 | 中 | 4小时 |
| **apaas** | 缺失 | 中 | 1天 |
| **aily** | 已有 | 中 | 优化 |
| **personal_settings** → **personal-settings** | 不匹配 | 中 | 2小时 |

#### 实施任务

1. **新增翻译和语音服务**
   - translation功能标志
   - speech_to_text功能标志
   - 基础API实现

2. **应用平台服务**
   - apaas功能标志
   - 应用管理相关API

3. **现有服务优化**
   - aily服务完善
   - personal_settings命名修正

### 第三阶段：专项服务完善（第3周）

#### 低优先级但重要的服务

| 服务 | 现状 | 优先级 | 预计工作量 |
|------|------|--------|------------|
| **passport** | 已有 | 低 | 优化 |
| **ephemeral** | 缺失 | 低 | 4小时 |
| **optical_char_recognition** | 缺失 | 低 | 4小时 |
| **face_verify** | 缺失 | 低 | 4小时 |
| **trust_party** | 已有 | 低 | 优化 |

## 详细实施方案

### 1. authen统一到auth实施方案

#### 当前状态
```rust
// 现有代码可能使用authen
#[cfg(feature = "authen")]
pub use crate::service::authen::AuthenService;

// 而auth功能标志已存在
#[cfg(feature = "auth")]
pub use crate::service::auth::AuthService;
```

#### 实施步骤

1. **添加别名兼容性**
```rust
// 在service/mod.rs中添加
#[cfg(feature = "authen")]
pub use crate::service::auth::AuthService as AuthenService;

// 或者在authen模块中重导出
#[cfg(feature = "auth")]
pub use super::auth::AuthService;
```

2. **更新客户端**
```rust
// 在LarkClient中
#[cfg(feature = "auth")]
pub auth: AuthService,

// 添加别名支持
#[cfg(feature = "authen")]
pub authen: AuthenService,
```

3. **Cargo.toml更新**
```toml
# 保持向后兼容
authen = ["auth"]

# 在文档中说明推荐使用auth
[features]
default = ["auth"]
```

### 2. 新增sheets功能标志实施方案

#### 目录结构创建
```
src/service/sheets/
├── mod.rs
├── models/
│   └── mod.rs
└── v3/
    ├── mod.rs
    └── spreadsheet.rs
```

#### 核心实现
```rust
// src/service/sheets/mod.rs
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Sheets服务模块
//!
//! 提供飞书多维表格相关的API功能

use crate::core::config::Config;

/// Sheets服务
#[derive(Debug, Clone)]
pub struct SheetsService {
    pub config: Config,
    pub v3: v3::SheetsServiceV3,
}

impl SheetsService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v3: v3::SheetsServiceV3::new(config),
        }
    }
}

pub mod v3;
```

### 3. 云文档服务统一实施方案

#### 当前问题
- docx和drive使用不同的服务名
- 都属于云文档功能范畴
- 需要统一到cloud-docs功能标志

#### 实施方案

1. **创建统一服务模块**
```rust
// src/service/cloud_docs/mod.rs
pub struct CloudDocsService {
    pub config: Config,
    pub docx: docx::DocxServiceV1,
    pub drive: drive::DriveServiceV1,
}
```

2. **API路径重映射**
```rust
// 处理 /open-apis/docx/* 的API
impl DocxServiceV1 {
    // docx相关实现
}

// 处理 /open-apis/drive/* 的API
impl DriveServiceV1 {
    // drive相关实现
}
```

3. **客户端集成**
```rust
// src/client/mod.rs
#[cfg(feature = "cloud-docs")]
pub cloud_docs: CloudDocsService,
```

## 共享数据模型设计

### common功能标志结构

```rust
// src/common/mod.rs
pub mod models;

pub use models::*;

// src/common/models/mod.rs
//! 跨服务共享的数据模型

use serde::{Deserialize, Serialize};

/// 基础用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseUser {
    pub user_id: String,
    pub name: String,
    pub en_name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub avatar_url: Option<String>,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub page_token: Option<String>,
    pub page_size: i32,
    pub total: i32,
    pub has_more: bool,
}

/// API响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
```

## 自动化验证工具

### API映射检查工具

```rust
// tools/feature_flag_validator.rs
use std::collections::HashMap;

pub struct FeatureFlagValidator {
    api_mappings: HashMap<String, String>,
}

impl FeatureFlagValidator {
    pub fn new() -> Self {
        // 从CSV文件加载API映射
        let mappings = load_api_mappings();
        Self { api_mappings: mappings }
    }

    pub fn validate_mapping(&self, api_path: &str, feature_flag: &str) -> bool {
        if let Some(service) = extract_service_name(api_path) {
            // 检查映射是否符合规范
            match service.as_str() {
                "authen" => feature_flag == "auth",
                "docx" | "drive" => feature_flag == "cloud-docs",
                _ => feature_flag == service,
            }
        } else {
            false
        }
    }
}
```

### CI/CD集成

```yaml
# .github/workflows/feature-flag-validation.yml
name: Feature Flag Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Validate Feature Flags
        run: |
          cargo run --bin feature_flag_validator
          echo "Feature flag validation passed"
```

## 风险缓解措施

### 1. 向后兼容性保证

- 所有现有功能标志保持不变
- 新增功能标志遵循规范
- 提供迁移指南和工具

### 2. 渐进式实施

- 分批次实施，控制风险
- 每批次都有完整的测试验证
- 提供回滚方案

### 3. 文档和培训

- 完整的技术文档
- 开发者使用指南
- 最佳实践分享

## 成功标准

### 技术指标
- [ ] 100%的API按规范映射到功能标志
- [ ] 所有新API遵循命名规范
- [ ] CI/CD自动验证通过率100%

### 质量指标
- [ ] 0个破坏性变更
- [ ] 所有变更有对应文档
- [ ] 开发者反馈积极

### 用户体验指标
- [ ] 新用户学习成本降低50%
- [ ] API查找时间缩短60%
- [ ] 错误率降低90%

## 时间安排

| 阶段 | 时间 | 主要任务 | 交付物 |
|------|------|----------|--------|
| 第1阶段 | 第1周 | 高优先级服务 | auth统一、sheets、bitable、云文档统一 |
| 第2阶段 | 第2周 | 中优先级服务 | 翻译语音、apaas、服务优化 |
| 第3阶段 | 第3周 | 低优先级服务 | 专项服务、工具完善 |
| 第4阶段 | 第4周 | 文档和培训 | 完整文档、使用指南、团队培训 |

## 团队协作

### 分工安排

- **架构师**：制定规范，设计共享模型
- **核心开发者**：实施重构，工具开发
- **文档工程师**：文档编写，示例创建
- **测试工程师**：测试验证，质量保证

### 沟通机制

- 每日进度同步会议
- 每周技术评审
- 阶段完成总结会议
- 最终项目复盘

---

**版本：** 1.0
**项目经理：** open-lark开发团队
**预计完成时间：** 2025-12-03