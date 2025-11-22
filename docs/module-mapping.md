# Open-Lark 模块映射文档

## 概述

本文档详细描述了从原始API数据到新模块化架构的映射关系，包括具体的API分布、迁移优先级和依赖关系。

## API数据总览

基于分析结果，总共有1,688个API，分布在以下业务领域：

| 总体统计 | 数量 | 占比 |
|---------|------|------|
| **总API数量** | **1,688** | **100%** |
| **业务模块数量** | **12** | - |
| **保留核心模块** | **3** | - |

## 详细模块映射

### 1. HR人力管理模块 (openlark-hr)

**模块标识**: `openlark-hr`
**API总数**: 484个 (28.7%)
**业务价值**: 高 (企业核心业务)

#### 子模块分布

| 子模块 | bizTag | API数量 | 主要功能 |
|--------|--------|---------|---------|
| 招聘管理 | hire | 182 | Offer管理、候选人跟踪、面试安排 |
| 核心HR | corehr | 144 | 员工管理、组织架构、基础HR |
| 人员管理 | feishu_people | 105 | 人员信息、档案管理 |
| 考勤管理 | attendance | 39 | 考勤记录、请假审批 |
| 薪酬管理 | payroll | 12 | 薪资计算、发放管理 |
| 电子HR | ehr | 2 | 电子化人力资源 |

#### 关键API示例

```rust
// 招聘管理 (hire)
offer_create_v1()           // 创建Offer
offer_update_v1()           // 更新Offer
candidate_list_v1()         // 获取候选人列表
interview_schedule_v1()     // 安排面试

// 核心HR (corehr)
employee_create_v1()        // 创建员工
employee_update_v1()        // 更新员工信息
department_list_v1()        // 获取部门列表

// 人员管理 (feishu_people)
profile_get_v1()           // 获取人员档案
emergency_contact_update_v1() // 更新紧急联系人

// 考勤管理 (attendance)
attendance_record_list_v1() // 获取考勤记录
leave_apply_v1()           // 申请请假

// 薪酬管理 (payroll)
salary_get_v1()            // 获取薪资信息
payslip_list_v1()          // 获取工资单
```

#### 目录结构设计

```
crates/openlark-hr/src/
├── lib.rs                  // 模块入口
├── models/                 // 共享模型
│   ├── mod.rs
│   ├── employee.rs         // 员工模型
│   ├── department.rs       // 部门模型
│   ├── offer.rs           // Offer模型
│   └── candidate.rs       // 候选人模型
├── hire/                  // 招聘管理 (182 APIs)
│   ├── v1/
│   │   ├── mod.rs
│   │   ├── offer.rs       // Offer相关API
│   │   ├── candidate.rs   // 候选人API
│   │   ├── interview.rs   // 面试API
│   │   └── onboarding.rs  // 入职API
│   └── models/
│       ├── offer.rs
│       └── candidate.rs
├── corehr/                // 核心HR (144 APIs)
│   ├── v1/
│   │   ├── mod.rs
│   │   ├── employee.rs    // 员工管理API
│   │   ├── department.rs  // 部门管理API
│   │   └── organization.rs // 组织架构API
│   └── models/
├── people/                // 人员管理 (105 APIs)
│   ├── v1/
│   │   ├── mod.rs
│   │   ├── profile.rs     // 人员档案API
│   │   └── contact.rs     // 联系方式API
│   └── models/
├── attendance/            // 考勤管理 (39 APIs)
│   ├── v1/
│   │   ├── mod.rs
│   │   ├── record.rs      // 考勤记录API
│   │   └── leave.rs       // 请假API
│   └── models/
└── payroll/               // 薪酬管理 (12 APIs)
    ├── v1/
    │   ├── mod.rs
    │   ├── salary.rs      // 薪资API
    │   └── payslip.rs     // 工资单API
    └── models/
```

### 2. 通讯协作模块 (openlark-communication)

**模块标识**: `openlark-communication`
**API总数**: 153个 (9.1%)
**业务价值**: 高 (高频使用功能)

#### 子模块分布

| 子模块 | bizTag | API数量 | 主要功能 |
|--------|--------|---------|---------|
| 联系人管理 | contact | 77 | 通讯录、用户信息、部门管理 |
| 即时消息 | im | 75 | 消息发送、接收、群聊管理 |
| 动态圈 | moments | 1 | 企业动态分享 |

#### 关键API示例

```rust
// 联系人管理 (contact)
user_create_v3()           // 创建用户
user_get_v3()             // 获取用户信息
user_list_v3()            // 获取用户列表
department_create_v3()     // 创建部门
department_list_v3()       // 获取部门列表

// 即时消息 (im)
message_send_v1()         // 发送消息
message_list_v1()         // 获取消息列表
chat_create_v1()          // 创建群聊
chat_member_add_v1()      // 添加群成员
```

#### 目录结构设计

```
crates/openlark-communication/src/
├── lib.rs
├── models/
├── contact/               // 联系人管理 (77 APIs)
│   ├── v1/
│   ├── v2/
│   ├── v3/
│   └── models/
├── im/                    // 即时消息 (75 APIs)
│   ├── v1/
│   └── models/
└── moments/               // 动态圈 (1 API)
    ├── v1/
    └── models/
```

### 3. 文档协作模块 (openlark-docs)

**模块标识**: `openlark-docs`
**API总数**: 254个 (15.0%)
**业务价值**: 高 (知识管理核心)

#### 子模块分布

| 子模块 | bizTag | API数量 | 主要功能 |
|--------|--------|---------|---------|
| 云文档 | ccm | 174 | 文档创建、编辑、共享、协作 |
| 基础服务 | base | 49 | 文件存储、基础操作 |
| 知识库 | baike | 27 | 企业知识库、Wiki管理 |
| 会议纪要 | minutes | 4 | 会议记录管理 |

#### 关键API示例

```rust
// 云文档 (ccm)
document_create_v1()       // 创建文档
document_read_v1()        // 读取文档
document_share_v1()       // 共享文档
document_comment_add_v1() // 添加评论

// 基础服务 (base)
file_upload_v1()          // 上传文件
file_download_v1()        // 下载文件
drive_space_list_v1()     // 获取云盘空间

// 知识库 (baike)
wiki_node_create_v2()     // 创建Wiki节点
wiki_space_list_v2()      // 获取Wiki空间
```

### 4. 任务审批模块 (openlark-workflow)

**模块标识**: `openlark-workflow`
**API总数**: 134个 (7.9%)
**业务价值**: 中高 (流程自动化)

#### 子模块分布

| 子模块 | bizTag | API数量 | 主要功能 |
|--------|--------|---------|---------|
| 任务管理 | task | 75 | 任务创建、分配、跟踪 |
| 审批流程 | approval | 53 | 审批定义、流程管理 |
| 看板管理 | board | 6 | 看板创建、任务卡片 |

#### 关键API示例

```rust
// 任务管理 (task)
task_create_v1()           // 创建任务
task_list_v1()            // 获取任务列表
task_complete_v1()        // 完成任务
task_assign_v1()          // 分配任务

// 审批流程 (approval)
approval_create_v4()      // 创建审批
approval_instance_list_v4() // 获取审批实例
approval_definition_list_v4() // 获取审批定义
```

### 5. 会议日程模块 (openlark-meeting)

**模块标识**: `openlark-meeting`
**API总数**: 117个 (6.9%)
**业务价值**: 中高 (协作核心)

#### 子模块分布

| 子模块 | bizTag | API数量 | 主要功能 |
|--------|--------|---------|---------|
| 日历管理 | calendar | 44 | 日程安排、会议创建 |
| 视频会议 | vc | 56 | 视频会议、会议室管理 |
| 会议室 | meeting_room | 17 | 会议室预订、管理 |

#### 关键API示例

```rust
// 日历管理 (calendar)
calendar_create_v4()      // 创建日历
calendar_event_create_v4() // 创建日程
calendar_event_list_v4()  // 获取日程列表

// 视频会议 (vc)
meeting_start_v1()        // 开始会议
meeting_join_v1()         // 加入会议
meeting_room_list_v1()    // 获取会议室列表
```

### 6. 其他模块概览

#### 邮件服务模块 (openlark-mail)
- **API数量**: 67个 (4.0%)
- **主要功能**: 邮件发送、接收、管理
- **关键API**: mail_send_v1(), mail_list_v1(), mail_group_create_v1()

#### 帮助台模块 (openlark-helpdesk)
- **API数量**: 50个 (3.0%)
- **主要功能**: 客服管理、工单系统
- **关键API**: ticket_create_v1(), ticket_list_v1(), agent_list_v1()

#### 应用管理模块 (openlark-platform)
- **API数量**: 86个 (5.1%)
- **主要功能**: 应用管理、平台工具、系统管理
- **关键API**: app_create_v1(), app_update_v1(), admin_user_list_v1()

#### AI智能模块 (openlark-ai)
- **API数量**: 23个 (1.4%)
- **主要功能**: AI服务、智能分析
- **关键API**: ai_chat_v1(), ai_summary_v1(), ai_translate_v1()

#### 安全认证模块 (openlark-security)
- **API数量**: 44个 (2.6%)
- **主要功能**: 身份认证、安全管理
- **关键API**: auth_login_v1(), auth_logout_v1(), security_audit_log_list_v1()

#### 数据分析模块 (openlark-analytics)
- **API数量**: 38个 (2.2%)
- **主要功能**: 搜索、报表、数据分析
- **关键API**: search_user_v1(), report_generate_v1(), directory_sync_v1()

#### 个人设置模块 (openlark-user)
- **API数量**: 21个 (1.2%)
- **主要功能**: 个人设置、用户偏好
- **关键API**: user_preference_get_v1(), user_preference_update_v1()

## 优先级和迁移策略

### 迁移优先级矩阵

| 模块 | 业务重要性 | API数量 | 复杂度 | 迁移优先级 |
|------|-----------|---------|--------|-----------|
| openlark-communication | 🔴 极高 | 153 | 🟡 中 | **P0 - 最高** |
| openlark-docs | 🔴 极高 | 254 | 🔴 高 | **P0 - 最高** |
| openlark-security | 🟡 高 | 44 | 🟢 低 | **P0 - 最高** |
| openlark-workflow | 🟡 高 | 134 | 🟡 中 | **P1 - 高** |
| openlark-meeting | 🟡 高 | 117 | 🟡 中 | **P1 - 高** |
| openlark-hr | 🟡 高 | 484 | 🔴 高 | **P2 - 中** |
| openlark-mail | 🟢 中 | 67 | 🟢 低 | **P2 - 中** |
| openlark-platform | 🟢 中 | 86 | 🟡 中 | **P2 - 中** |
| openlark-ai | 🟢 中 | 23 | 🟢 低 | **P3 - 低** |
| openlark-analytics | 🟢 中 | 38 | 🟡 中 | **P3 - 低** |
| openlark-helpdesk | 🟢 中 | 50 | 🟢 低 | **P3 - 低** |
| openlark-user | 🟢 低 | 21 | 🟢 低 | **P3 - 低** |

### 迁移阶段规划

#### Phase 1: 核心模块 (P0)
**目标**: 完成最高优先级模块迁移
**模块**: openlark-communication, openlark-docs, openlark-security
**预期时间**: 4-6周
**里程碑**:
- ✅ 完成模块设计和结构创建
- ✅ 实现核心API (前50%)
- ✅ 完成基础测试
- ✅ 提供示例代码

#### Phase 2: 业务流程模块 (P1)
**目标**: 完成业务流程相关模块
**模块**: openlark-workflow, openlark-meeting
**预期时间**: 2-3周

#### Phase 3: 企业功能模块 (P2)
**目标**: 完成企业级功能模块
**模块**: openlark-hr, openlark-mail, openlark-platform
**预期时间**: 4-6周
**里程碑**:
- HR模块实现 (484 APIs - 最大模块)
- 邮件和平台模块实现

#### Phase 4: 扩展功能模块 (P3)
**目标**: 完成扩展和高级功能
**模块**: openlark-ai, openlark-analytics, openlark-helpdesk, openlark-user
**预期时间**: 2-3周

## 依赖关系分析

### 模块依赖图

```mermaid
graph TD
    A["openlark-core"] --> B["openlark-client"]
    A --> C["openlark-hr"]
    A --> D["openlark-communication"]
    A --> E["openlark-docs"]
    A --> F["openlark-workflow"]
    A --> G["openlark-meeting"]
    A --> H["openlark-mail"]
    A --> I["openlark-helpdesk"]
    A --> J["openlark-platform"]
    A --> K["openlark-ai"]
    A --> L["openlark-security"]
    A --> M["openlark-analytics"]
    A --> N["openlark-user"]

    A --> O["openlark-protocol"]

    B --> C
    B --> D
    B --> E
    B --> F
    B --> G
    B --> H
    B --> I
    B --> J
    B --> K
    B --> L
    B --> M
    B --> N

    L --> C
    L --> D
    L --> E

    style A fill:#e1f5fe
    style O fill:#f3e5f5
    style B fill:#e8f5e8
    style L fill:#fff3e0
```

### 依赖关系说明

**核心依赖**:
- 所有业务模块依赖 `openlark-core`
- `openlark-security` 是 `openlark-hr`, `openlark-communication`, `openlark-docs` 的前置依赖

**功能依赖**:
- `openlark-client` 依赖所有业务模块
- `openlark-protocol` 为WebSocket功能提供支持

**循环依赖解决**:
- 通过接口抽象和依赖注入避免循环依赖
- 使用事件驱动架构减少直接依赖

## Feature配置策略

### 编译时间优化

```toml
[features]
# 默认配置 - 平衡功能和编译时间
default = ["communication", "docs", "security"]

# 快速开发 - 仅核心功能
dev = ["core", "security"]

# 完整功能 - 所有模块
full = [
    "communication", "docs", "hr", "workflow",
    "meeting", "ai", "security", "analytics",
    "mail", "helpdesk", "platform", "user"
]

# 业务场景配置
hr_suite = ["hr", "security", "communication"]
collaboration_suite = ["communication", "docs", "meeting", "workflow"]
enterprise_suite = ["hr", "security", "platform", "analytics"]
```

### 条件编译示例

```rust
// 在客户端中按需注册服务
#[cfg(feature = "hr")]
fn register_hr_services(client: &mut LarkClient) {
    client.register_service("hr", HRService::new(client.config.clone()));
}

#[cfg(feature = "communication")]
fn register_communication_services(client: &mut LarkClient) {
    client.register_service("communication", CommunicationService::new(client.config.clone()));
}
```

## 质量保证

### API覆盖率目标

| 模块 | 当前覆盖率 | 目标覆盖率 | 优先级 |
|------|-----------|-----------|--------|
| openlark-hr | 0% | 95% | P0 |
| openlark-communication | 0% | 95% | P0 |
| openlark-docs | 0% | 90% | P0 |
| openlark-security | 0% | 100% | P0 |
| openlark-workflow | 0% | 85% | P1 |
| openlark-meeting | 0% | 85% | P1 |
| 其他模块 | 0% | 80% | P2/P3 |

### 测试策略

1. **单元测试**: 每个API独立测试，目标覆盖率80%+
2. **集成测试**: 跨模块协作测试，覆盖关键业务流程
3. **端到端测试**: 完整业务场景测试，覆盖主要使用场景
4. **性能测试**: 关键API性能基准测试

### 验证工具

```python
# tools/module_validator.py
class ModuleValidator:
    def validate_api_coverage(self, module_name: str) -> float:
        """计算API覆盖率"""
        pass

    def validate_dependencies(self) -> bool:
        """验证依赖关系"""
        pass

    def validate_naming_conventions(self) -> List[str]:
        """验证命名规范"""
        pass
```

## 实施检查清单

### Phase 1 检查清单

- [ ] 完成核心模块设计和目录结构创建
- [ ] 实现openlark-hr模块前100个关键API
- [ ] 实现openlark-communication模块核心API
- [ ] 实现openlark-docs模块文档操作API
- [ ] 实现openlark-security模块认证API
- [ ] 创建基础测试用例
- [ ] 提供使用示例
- [ ] 更新文档

### 质量门禁

每个模块发布前必须通过：

1. **编译检查**: 零警告编译
2. **测试检查**: 单元测试覆盖率 > 80%
3. **文档检查**: 所有公共API有文档
4. **性能检查**: 关键API响应时间 < 500ms
5. **安全检查**: 通过安全扫描

---

本文档为Open-Lark模块化重构提供了详细的映射关系和实施指导，确保重构过程的有序进行和高质量交付。