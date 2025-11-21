# Feature 命名规范标准

> **文档版本**: 1.0
> **创建时间**: 2025-11-20
> **适用范围**: open-lark 项目所有 feature 定义

## 📋 命名规范概览

### 设计原则

1. **清晰性**: 名称应清晰表达功能含义
2. **一致性**: 同类型功能使用相同的命名模式
3. **简洁性**: 避免冗长，保持简洁易懂
4. **可扩展性**: 为未来功能扩展预留命名空间
5. **向后兼容**: 保持与现有 API 的兼容性

## 🏗️ 命名层级体系

### Level 1: 业务层次 (*-layer)

**命名模式**: `{业务领域}-layer`

**说明**: 代表不同层次的功能集合，满足不同用户群体的需求

**示例**:
```toml
# Level 1: 核心基础功能
core-layer = [
    "communication",  # IM、消息、联系人管理
    "docs",           # 云文档、表格、知识库
    "auth"            # 认证、授权、令牌管理
]

# Level 2: 专业协作功能
professional-layer = [
    "core-layer",
    "hr",             # 人力资源、考勤、招聘
    "ai",             # AI服务、智能分析
    "calendar"        # 日历、会议安排、日程管理
]

# Level 3: 企业级功能
enterprise-layer = [
    "professional-layer",
    "admin",          # 管理功能、行政服务
    "approval",       # 审批流程、工作流
    "helpdesk"        # 帮助台、客服管理、工单系统
]
```

### Level 2: 功能套件 (*-suite)

**命名模式**: `{业务场景}-suite`

**说明**: 针对特定业务场景的功能组合，向后兼容

**示例**:
```toml
# 通讯相关套件
im-suite = ["communication"]
contact-suite = ["communication"]
group-suite = ["communication"]

# 文档相关套件
docs-suite = ["docs"]
sheets-suite = ["docs"]
wiki-suite = ["docs"]

# 业务功能套件
hr-suite = ["hr"]
ai-suite = ["ai"]
admin-suite = ["admin"]
approval-suite = ["approval"]
helpdesk-suite = ["helpdesk"]
calendar-suite = ["calendar"]
```

### Level 3: 单个功能

**命名模式**: `{服务名称}`

**说明**: 直接使用服务模块名称，保持简洁

**示例**:
```toml
# 基础功能
auth = ["openlark-auth"]
communication = ["openlark-communication"]
docs = ["openlark-docs"]

# 专业功能
hr = ["openlark-hr"]
ai = ["openlark-ai"]
calendar = ["openlark-calendar"]

# 企业功能
admin = ["openlark-admin"]
approval = ["openlark-approval"]
helpdesk = ["openlark-helpdesk"]
```

### Level 4: 技术支持功能

**命名模式**: `{技术名称}`

**说明**: 独立于业务功能的技术支持

**示例**:
```toml
# WebSocket支持
websocket = ["openlark-protocol"]

# OpenTelemetry可观测性
otel = ["opentelemetry", "opentelemetry_sdk", "tracing-opentelemetry"]

# 异步支持
async = []
```

### Level 5: 客户端功能

**命名模式**: `client-{功能}`

**说明**: 客户端特定的功能组合

**示例**:
```toml
# 客户端核心功能
client-core = []

# 客户端业务集成
client-enterprise = ["client-core", "enterprise-layer"]
client-professional = ["client-core", "professional-layer"]
client-basic = ["client-core", "core-layer"]

# 技术支持
client-websocket = ["websocket"]
client-otel = ["otel"]

# 完整功能
client-complete = ["client-enterprise", "client-websocket", "client-otel"]
```

## 📊 命名映射表

### 向后兼容映射

| 旧名称 | 新名称 | 说明 |
|--------|--------|------|
| `docs-collaboration` | `core-layer` | 文档协作功能映射到核心层 |
| `communication-core` | `communication` | 直接映射到具体功能 |
| `professional-suite` | `professional-layer` | 保持原名但映射到新层 |
| `enterprise-suite` | `enterprise-layer` | 保持原名但映射到新层 |
| `full-suite` | `enterprise-layer` | 映射到最完整的层 |
| `cloud-docs` | `docs` | 传统的文档功能名 |

### Crate 映射关系

| Feature | 对应 Crate | 功能描述 |
|---------|------------|----------|
| `auth` | `openlark-auth` | 认证和授权 |
| `communication` | `openlark-communication` | IM消息和通讯 |
| `docs` | `openlark-docs` | 云文档和知识库 |
| `hr` | `openlark-hr` | 人力资源管理 |
| `ai` | `openlark-ai` | AI服务和智能分析 |
| `calendar` | `openlark-calendar` | 日历和会议管理 |
| `admin` | `openlark-admin` | 管理和行政功能 |
| `approval` | `openlark-approval` | 审批流程管理 |
| `helpdesk` | `openlark-helpdesk` | 帮助台和客服 |
| `websocket` | `openlark-protocol` | WebSocket协议支持 |

## 🔧 实施标准

### 定义顺序

1. **基础功能定义**: 单个功能的直接映射
2. **层次组合**: `-layer` 类型的功能组合
3. **场景组合**: `-suite` 类型的场景化组合
4. **兼容性别名**: 向后兼容的别名映射
5. **技术支持**: 独立的技术功能

### 依赖规范

- **单向依赖**: 高层依赖低层，禁止循环依赖
- **最小依赖**: 每层只依赖必要的下层功能
- **明确引用**: 避免 `*-suite` 之间的交叉引用

### 文档要求

- **功能描述**: 每个 feature 都必须有清晰的注释
- **使用示例**: 复杂功能组合需要提供使用示例
- **升级路径**: 明确各层之间的升级关系

## 📝 使用指南

### 用户选择建议

| 用户类型 | 推荐选择 | 包含功能 |
|----------|----------|----------|
| **基础用户** | `core-layer` | 沟通、文档、认证 |
| **专业用户** | `professional-layer` | 核心功能 + HR + AI + 日历 |
| **企业用户** | `enterprise-layer` | 专业功能 + 管理 + 审批 + 帮助台 |
| **完整用户** | `enterprise-layer` + 技术功能 | 所有功能 |

### 升级路径

```toml
# 从基础到专业
core-layer → professional-layer (增加: hr, ai, calendar)

# 从专业到企业
professional-layer → enterprise-layer (增加: admin, approval, helpdesk)

# 技术功能叠加
任何层 + websocket + otel = 完整功能
```

### 代码示例

```rust
// 使用基础功能
use open_lark::{core_layer, communication, docs, auth};

// 使用专业功能
use open_lark::{professional_layer, hr, ai, calendar};

// 使用企业功能
use open_lark::{enterprise_layer, admin, approval, helpdesk};

// 技术功能
use open_lark::{websocket, otel};

// 客户端使用
use open_lark_client::{client_basic, client_professional, client_enterprise};
```

## ⚠️ 注意事项

### 避免的命名模式

- ❌ **功能重复**: 避免不同名称表示相同功能
- ❌ **层级混乱**: 不要在 `-layer` 中直接引用 `-suite`
- ❌ **过度细分**: 避免创建过多的细粒度功能
- ❌ **命名冲突**: 避免与现有 crate 名称冲突

### 变更流程

1. **需求分析**: 明确功能需求和用户场景
2. **命名评审**: 按命名规范评审命名方案
3. **兼容性检查**: 确保向后兼容性
4. **文档更新**: 同步更新相关文档
5. **代码审查**: 通过代码审查流程
6. **发布验证**: 在测试环境验证变更

## 📚 相关文档

- [Feature 架构设计文档](../architecture/feature-layers.md)
- [依赖关系说明](../architecture/dependencies.md)
- [用户使用指南](../user-guide/feature-selection.md)
- [迁移指南](../migration/v0.14-to-v0.15.md)

---

## 🔄 更新历史

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| 1.0 | 2025-11-20 | 初始版本，建立命名规范标准 |

---

**文档维护**: 请在命名规范变更时及时更新本文档，确保规范的一致性和时效性。