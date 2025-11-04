# OpenLark 新目录结构规划

## 目标结构

基于架构重构分析，新的目录结构将采用扁平化 workspace 布局，符合 Rust 社区最佳实践。

```
open-lark-workspace/
├── Cargo.toml                    # 根 workspace 配置 (virtual manifest)
├── README.md                     # 项目总览和快速开始
├── CHANGELOG.md                  # 变更日志
├── LICENSE                       # 许可证
├── .gitignore                    # Git 忽略规则
├── justfile                      # 构建脚本
├── clippy.toml                   # Clippy 配置
├── rustfmt.toml                  # 代码格式化配置
│
├── crates/                       # 子 crate 目录
│   ├── open-lark-core/           # 核心基础设施
│   ├── open-lark-communication/  # 通讯核心服务
│   ├── open-lark-collaboration/  # 协作办公服务
│   ├── open-lark-hr-suite/       # 人力资源管理
│   ├── open-lark-ai-platform/    # AI 和智能服务
│   ├── open-lark-enterprise/     # 企业级服务
│   ├── open-lark-integrations/   # 第三方集成和工具
│   ├── open-lark-extensions/     # 扩展和专用服务
│   └── lark-websocket-protobuf/  # WebSocket protobuf (已存在)
│
├── examples/                     # 示例代码
│   ├── basic/                    # 基础示例
│   ├── api/                      # API 使用示例
│   └── tracing/                  # 追踪和监控示例
│
├── tools/                        # 开发工具
│   ├── api_consistency_checker.rs
│   ├── enhanced_api_checker.rs
│   └── api_compatibility_tester.rs
│
├── benches/                      # 性能基准测试
│   ├── json_parsing_bench.rs
│   ├── websocket_frame_bench.rs
│   └── token_performance_bench.rs
│
├── tests/                        # 集成测试
│   └── integration/
│
└── docs/                         # 文档
    ├── architecture/             # 架构文档
    ├── migration/                # 迁移指南
    └── api/                      # API 文档
```

## 各 Crate 详细结构

### open-lark-core
```
crates/open-lark-core/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                    # 库入口，重新导出核心组件
│   ├── client/                   # HTTP 客户端
│   │   ├── mod.rs
│   │   ├── config.rs             # 客户端配置
│   │   └── http_client.rs        # HTTP 客户端实现
│   ├── auth/                     # 认证和令牌管理
│   │   ├── mod.rs
│   │   ├── token_manager.rs      # 令牌管理器
│   │   └── types.rs              # 认证相关类型
│   ├── error/                    # 错误处理
│   │   ├── mod.rs
│   │   ├── sdk_error.rs          # SDK 错误类型
│   │   └── result.rs             # 结果类型
│   ├── config/                   # 配置管理
│   │   ├── mod.rs
│   │   └── environment.rs        # 环境配置
│   └── utils/                    # 通用工具
│       ├── mod.rs
│       ├── macros.rs             # 实用宏
│       └── helpers.rs            # 辅助函数
└── tests/
    └── integration/
```

### open-lark-communication
```
crates/open-lark-communication/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                    # 重新导出所有通讯服务
│   ├── services/                 # 服务实现
│   │   ├── mod.rs
│   │   ├── im/                   # 即时通讯
│   │   │   ├── mod.rs
│   │   │   ├── v1/
│   │   │   └── v2/
│   │   ├── contact/              # 联系人管理
│   │   │   ├── mod.rs
│   │   │   └── v3/
│   │   ├── group/                # 群组管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   └── search/               # 搜索服务
│   │       ├── mod.rs
│   │       └── v2/
│   └── models/                   # 数据模型
│       └── mod.rs
└── tests/
```

### open-lark-collaboration
```
crates/open-lark-collaboration/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── cloud_docs/           # 云文档
│   │   │   ├── mod.rs
│   │   │   ├── drive/
│   │   │   ├── wiki/
│   │   │   └── sheet/
│   │   ├── calendar/             # 日历管理
│   │   │   ├── mod.rs
│   │   │   └── v4/
│   │   ├── approval/             # 审批流程
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── task/                 # 任务管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   └── minutes/              # 会议纪要
│   │       ├── mod.rs
│   │       └── v1/
│   └── models/
│       └── mod.rs
└── tests/
```

### open-lark-hr-suite
```
crates/open-lark-hr-suite/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── hire/                 # 招聘管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── corehr/               # 核心 HR
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── ehr/                  # 电子健康记录
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── payroll/              # 薪资管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── attendance/           # 考勤管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   ├── performance/          # 绩效管理
│   │   │   ├── mod.rs
│   │   │   └── v1/
│   │   └── okr/                  # OKR 管理
│   │       ├── mod.rs
│   │       └── v1/
│   └── models/
│       └── mod.rs
└── tests/
```

## 根 Cargo.toml 配置

```toml
[workspace]
resolver = "2"
members = [
    "crates/open-lark-core",
    "crates/open-lark-communication",
    "crates/open-lark-collaboration",
    "crates/open-lark-hr-suite",
    "crates/open-lark-ai-platform",
    "crates/open-lark-enterprise",
    "crates/open-lark-integrations",
    "crates/open-lark-extensions",
    "crates/lark-websocket-protobuf"
]

[workspace.package]
version = "0.15.0"
edition = "2021"
authors = ["ZoOL <zhooul@gmail.com>"]
license = "Apache-2.0"
repository = "https://github.com/foxzool/open-lark"
homepage = "https://github.com/foxzool/open-lark"
documentation = "https://docs.rs/open-lark"

[workspace.dependencies]
# 内部依赖
open-lark-core = { path = "crates/open-lark-core", version = "0.15.0" }
open-lark-communication = { path = "crates/open-lark-communication", version = "0.15.0", optional = true }
open-lark-collaboration = { path = "crates/open-lark-collaboration", version = "0.15.0", optional = true }
open-lark-hr-suite = { path = "crates/open-lark-hr-suite", version = "0.15.0", optional = true }
open-lark-ai-platform = { path = "crates/open-lark-ai-platform", version = "0.15.0", optional = true }
open-lark-enterprise = { path = "crates/open-lark-enterprise", version = "0.15.0", optional = true }
open-lark-integrations = { path = "crates/open-lark-integrations", version = "0.15.0", optional = true }
open-lark-extensions = { path = "crates/open-lark-extensions", version = "0.15.0", optional = true }

# 外部依赖 (共享版本管理)
tokio = { version = "1.38", features = ["rt", "rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12.7", default-features = false, features = ["json", "multipart", "rustls-tls"] }
chrono = { version = "0.4.38", features = ["serde"] }
anyhow = "1.0.86"
async-trait = "0.1.83"
thiserror = "2.0"
tracing = "0.1"
uuid = { version = "1.8.0", features = ["v4"] }
```

## 向后兼容的统一包

为了保持向后兼容性，创建一个 `open-lark` 包作为统一入口：

```toml
# crates/open-lark/Cargo.toml (如果需要的话)
[package]
name = "open-lark"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Enterprise-grade Lark/Feishu Open API SDK with comprehensive Chinese documentation and advanced error handling"
documentation = "https://docs.rs/open-lark"
repository.workspace = true
homepage.workspace = true

[features]
default = ["communication"]
communication = ["open-lark-communication"]
collaboration = ["communication", "open-lark-collaboration"]
hr-suite = ["communication", "open-lark-hr-suite"]
ai-platform = ["communication", "open-lark-ai-platform"]
enterprise = ["communication", "open-lark-enterprise"]
complete = ["communication", "collaboration", "hr-suite", "ai-platform", "enterprise"]

# Legacy feature mapping (向后兼容)
im = ["communication"]
cloud-docs = ["collaboration"]
contact = ["communication"]
group = ["communication"]
authentication = ["communication"]
search = ["communication"]
hire = ["hr-suite"]
attendance = ["hr-suite"]
approval = ["collaboration"]
calendar = ["collaboration"]
ai = ["ai-platform"]
# ... 其他 legacy features

[dependencies]
open-lark-core = { workspace = true }
open-lark-communication = { workspace = true, optional = true }
open-lark-collaboration = { workspace = true, optional = true }
open-lark-hr-suite = { workspace = true, optional = true }
open-lark-ai-platform = { workspace = true, optional = true }
open-lark-enterprise = { workspace = true, optional = true }
```

## 迁移优势

### 1. 扁平化结构
- 符合 Rust 社区最佳实践
- 避免深层嵌套的复杂性
- 便于导航和维护

### 2. 清晰的职责分离
- 每个 crate 有明确的功能边界
- 减少不必要的依赖传递
- 支持独立开发和测试

### 3. 向后兼容性
- 保持现有的公共 API
- 提供平滑的迁移路径
- Legacy features 自动映射到新的 crate 系统

### 4. 构建优化
- 支持并行编译
- 更细粒度的编译缓存
- 显著的编译时间改善

### 5. 包大小优化
- 按需链接未使用的 crate
- 减少最终二进制大小
- 提升用户体验

## 实施步骤

1. **创建新的目录结构**
2. **建立 workspace 配置**
3. **逐个创建和迁移 crate**
4. **更新构建脚本和 CI 配置**
5. **迁移示例和文档**
6. **验证功能完整性**

这个新的目录结构将为 open-lark 项目的长期发展提供坚实的技术基础。