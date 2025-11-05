# 功能标志映射技术规范

## 概述

本文档定义了open-lark项目中Cargo功能标志与API URL路径的映射规范，确保架构的一致性和可维护性。

## 映射原则

### 1. 基本映射规则

**URL路径到功能标志的映射规则：**
```
/open-apis/{service}/{version}/{endpoint} → {service}功能标志
```

**示例：**
- `/open-apis/attendance/v1/user_task` → `attendance` 功能标志
- `/open-apis/contact/v3/users` → `contact` 功能标志
- `/open-apis/authen/v1/user_info` → `authen` 功能标志

### 2. 命名规范

**功能标志命名规范：**
- 使用小写字母
- 使用下划线分隔复合词
- 直接使用URL路径中的服务名称
- 特殊情况需要明确的映射规则

## 标准映射表

### 核心服务（高优先级）

| URL服务名 | 功能标志 | API数量 | 说明 |
|---------|---------|---------|------|
| attendance | attendance | 39 | 考勤管理 |
| contact | contact | 75 | 通讯录管理 |
| im | im | 71 | 即时通讯 |
| authen | authen | 6 | 用户认证 |
| auth | auth | 5 | 应用认证 |
| task | task | 75 | 任务管理 |
| approval | approval | 29 | 审批流程 |
| calendar | calendar | 44 | 日程管理 |

### 办公协作服务

| URL服务名 | 功能标志 | API数量 | 说明 |
|---------|---------|---------|------|
| sheets | sheets | 60 | 电子表格 |
| docx | docx | 19 | 文档处理 |
| drive | drive | 70 | 云盘存储 |
| wiki | wiki | 16 | 知识库 |
| board | board | 6 | 看板 |
| minutes | minutes | 4 | 会议纪要 |
| vc | vc | 56 | 视频会议 |

### 人力资源服务

| URL服务名 | 功能标志 | API数量 | 说明 |
|---------|---------|---------|------|
| corehr | corehr | 249 | 核心人力资源 |
| hire | hire | 182 | 招聘管理 |
| attendance | attendance | 39 | 考勤管理 |
| compensation | compensation | 21 | 薪酬管理 |
| payroll | payroll | 12 | 薪资计算 |
| performance | performance | 20 | 绩效管理 |
| okr | okr | 12 | 目标管理 |

### 企业管理服务

| URL服务名 | 功能标志 | API数量 | 说明 |
|---------|---------|---------|------|
| admin | admin | 14 | 管理后台 |
| tenant | tenant | 2 | 租户管理 |
| directory | directory | 21 | 组织架构 |
| application | application | 32 | 应用管理 |
| mdm | mdm | 4 | 移动设备管理 |
| security_and_compliance | security_and_compliance | 8 | 安全合规 |

### AI和智能服务

| URL服务名 | 功能标志 | API数量 | 说明 |
|---------|---------|---------|------|
| ai | ai | 0 | 人工智能 |
| lingo | lingo | 14 | 语言服务 |
| document_ai | document_ai | 18 | 文档AI |
| translation | translation | 2 | 翻译服务 |
| speech_to_text | speech_to_text | 2 | 语音转文字 |
| optical_char_recognition | optical_char_recognition | 1 | OCR识别 |

## 特殊映射规则

### 1. 认证服务统一

**URL服务名：** `authen`、`auth`
**功能标志：** `auth`
**说明：** 将用户认证和应用认证统一到`auth`功能标志下

### 2. 云文档服务统一

**URL服务名：** `docx`、`drive`
**功能标志：** `cloud-docs`
**说明：** 文档处理和云盘存储统一到`cloud-docs`功能标志下

### 3. 即时通讯服务

**URL服务名：** `im`、`message`
**功能标志：** `im`
**说明：** 即时通讯相关API统一到`im`功能标志下

## 共享数据模型架构

### common功能标志

**用途：** 提供跨服务共享的基础数据结构

**包含内容：**
- 基础用户信息（User、UserId等）
- 通用响应结构（BaseResponse等）
- 分页相关结构（PageToken、PageInfo等）
- 错误处理相关结构

### core功能标志

**用途：** 提供核心业务逻辑和工具函数

**包含内容：**
- HTTP客户端封装
- 配置管理
- 错误处理机制
- 日志记录工具

## 向后兼容性保证

### 1. 现有功能标志保持不变

所有现有的功能标志都将继续支持，不会进行破坏性更改。

### 2. 新功能标志按规范添加

新增的服务必须按照本规范命名和实现功能标志。

### 3. 渐进式迁移

对于需要调整的服务，采用渐进式迁移策略：
1. 新功能使用规范化的功能标志
2. 旧功能保持兼容
3. 在适当版本中统一功能标志

## 实施检查清单

### 功能标志配置检查

- [ ] 每个服务都有对应的功能标志
- [ ] 功能标志命名符合规范
- [ ] Cargo.toml中正确配置依赖关系

### 代码组织检查

- [ ] 服务模块按功能标志组织
- [ ] 共享数据模型放在common模块
- [ ] 条件编译指令正确使用

### 文档完整性检查

- [ ] 每个功能标志都有对应的文档
- [ ] API映射关系清晰明确
- [ ] 使用示例完整可用

## 工具和验证

### 1. 自动化审计工具

定期运行审计工具检查：
- API路径与功能标志的映射一致性
- 新API是否遵循规范
- 共享数据模型的正确使用

### 2. CI/CD集成

在CI流水线中添加检查：
- 功能标志配置验证
- 代码组织结构检查
- 文档完整性验证

### 3. 开发者工具

提供开发者工具：
- 功能标志使用指南
- 自动化代码生成工具
- 映射关系查询工具

## 维护和更新

### 规则更新流程

1. 发现新的映射需求
2. 评估影响范围
3. 更新规范文档
4. 更新相关工具和文档
5. 通知开发团队

### 版本兼容性

- 规范更新需要考虑向后兼容性
- 破坏性变更需要major版本更新
- 所有变更都需要有明确的迁移指南

---

**版本：** 1.0
**最后更新：** 2025-11-05
**维护者：** open-lark开发团队