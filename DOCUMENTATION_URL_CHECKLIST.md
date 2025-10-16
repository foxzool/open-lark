# 飞书API文档URL更新清单

## 📊 总体进度跟踪

### 当前状态
- **总API数量**: 4,304个
- **有文档URL的API**: 134个 (3.1%)
- **缺少文档URL的API**: 4,170个 (96.9%)
- **目标**: 100%覆盖率

### 覆盖率里程碑
- [ ] 25% 覆盖率 (约1,076个API) - 第2周目标
- [ ] 50% 覆盖率 (约2,152个API) - 第4周目标
- [ ] 80% 覆盖率 (约3,443个API) - 第6周目标
- [ ] 100% 覆盖率 (4,304个API) - 最终目标

---

## 🎯 第一阶段：核心优先服务 (第1-2周)

### 1. cloud_docs 服务 (v1) - 最高优先级
**API数量**: 2,412 | **当前覆盖**: 63个 (2.6%) | **缺少**: 2,349个

#### 核心模块清单
- [ ] **file模块** (约500个API)
  - [ ] `get_file_meta` ✅ (已有)
  - [ ] `get_file_statistics` ✅ (已有)
  - [ ] `list_file_view_records` ✅ (已有)
  - [ ] `create_file` ✅ (已有)
  - [ ] `copy_file` ✅ (已有)
  - [ ] `delete_file` ✅ (已有)
  - [ ] `create_file_shortcut` ✅ (已有)
  - [ ] `search_files` ✅ (已有)
  - [ ] 其余492个API需要添加文档URL

- [ ] **folder模块** (约300个API)
  - [ ] 所有folder相关API需要文档URL

- [ ] **media模块** (约400个API)
  - [ ] 所有media相关API需要文档URL

- [ ] **docx模块** (约600个API)
  - [ ] `document/create` ✅ (已有)
  - [ ] `document/get` ✅ (已有)
  - [ ] `document/raw_content` ✅ (已有)
  - [ ] `document/list` ✅ (已有)
  - [ ] `document_block/create` ✅ (已有)
  - [ ] `document_block/get` ✅ (已有)
  - [ ] `document_block/patch` ✅ (已有)
  - [ ] `document_block/batch_update` ✅ (已有)
  - [ ] `document_block/batch_delete` ✅ (已有)
  - [ ] `document_block/get-2` ✅ (已有)
  - [ ] 其余590个API需要文档URL

- [ ] **bitable模块** (约400个API)
  - [ ] 所有bitable相关API需要文档URL

- [ ] **其他模块** (约200个API)
  - [ ] comments, events等模块需要文档URL

#### 验收标准
- [ ] 所有公共API方法都有文档URL
- [ ] URL格式: `https://open.feishu.cn/document/server-docs/drive-v1/{category}/{action}`
- [ ] 通过检测工具验证覆盖率达到95%+

### 2. contact 服务 (v3) - 高优先级
**API数量**: 101 | **当前覆盖**: 0个 (0%) | **缺少**: 101个

#### 用户管理模块
- [ ] `user/create` - 需要添加文档URL
- [ ] `user/patch` - 需要添加文档URL
- [ ] `user/update_user_id` - 需要添加文档URL
- [ ] `user/get` - 需要添加文档URL
- [ ] `user/batch` - 需要添加文档URL
- [ ] `user/find_by_department` - 需要添加文档URL
- [ ] `user/batch_get_id` - 需要添加文档URL
- [ ] `user/search` - 需要添加文档URL
- [ ] `user/delete` - 需要添加文档URL
- [ ] `user/resurrect` - 需要添加文档URL
- [ ] `user/list` - 需要添加文档URL

#### 部门管理模块
- [ ] `department/create` - 需要添加文档URL
- [ ] `department/patch` - 需要添加文档URL
- [ ] `department/delete` - 需要添加文档URL
- [ ] `department/get` - 需要添加文档URL
- [ ] `department/list` - 需要添加文档URL
- [ ] 其他department相关API

#### 其他模块
- [ ] `group`相关API
- [ ] `job_family`, `job_level`, `job_title`等API
- [ ] `functional_role`相关API
- [ ] `custom_attr`, `scope`, `work_city`等API

#### 验收标准
- [ ] URL格式: `https://open.feishu.cn/document/server-docs/contact-v3/{category}/{action}`
- [ ] 100%覆盖所有公共API方法

### 3. im 服务 (v1) - 高优先级
**API数量**: 139 | **当前覆盖**: 7个 (5%) | **缺少**: 132个

#### 消息模块
- [ ] `message/send` ✅ (已有)
- [ ] `message/list` ✅ (已有)
- [ ] `message/receive` ✅ (已有)
- [ ] 其余消息相关API需要文档URL

#### 聊天模块
- [ ] `chat/list` ✅ (已有)
- [ ] 其余chat相关API需要文档URL

#### 其他模块
- [ ] `message_reaction`, `message_card`等模块
- [ ] `image`, `file`, `url_preview`, `buzz_messages`等
- [ ] `pin`相关API

#### 验收标准
- [ ] URL格式: `https://open.feishu.cn/document/server-docs/im-v1/{category}/{action}`
- [ ] 覆盖率达到95%+

---

## 🔧 第二阶段：业务关键服务 (第3-4周)

### 4. hire 服务 (v1) - 中高优先级
**API数量**: 210 | **当前覆盖**: 0个 (0%) | **缺少**: 210个

- [ ] 所有招聘相关API需要文档URL
- [ ] 重点：候选人和面试管理

### 5. directory 服务 (v1) - 中高优先级
**API数量**: 136 | **当前覆盖**: 15个 (11%) | **缺少**: 121个

- [ ] 员工管理相关API
- [ ] 部门管理相关API
- [ ] URL格式: `https://open.feishu.cn/document/directory-v1/{category}/{action}`

### 6. approval 服务 (v4) - 中高优先级
**API数量**: 45 | **当前覆盖**: 0个 (0%) | **缺少**: 45个

- [ ] 审批实例、任务、评论相关API
- [ ] URL格式: `https://open.feishu.cn/document/server-docs/approval-v4/{category}/{action}`

---

## 🎯 第三阶段：完善覆盖 (第5-6周)

### 7. 其他服务模块

#### attendance 服务 (v1) - ✅ 参考模型
**API数量**: 82 | **当前覆盖**: 39个 (47.6%) | **缺少**: 43个
- [ ] 已经是良好的参考模型，覆盖率较高
- [ ] 补充剩余43个API的文档URL

#### 其他需要处理的服务：
- [ ] **task** (v2) - 61个API，0%覆盖
- [ ] **ai** (v1) - 33个API，0%覆盖
- [ ] **calendar** (v4) - 40个API，7.5%覆盖
- [ ] **search** (v1/v2) - 39个API，2.9%覆盖
- [ ] **mail** (v1) - 53个API，0%覆盖
- [ ] **cardkit** (v1) - 38个API，13.2%覆盖
- [ ] **其他30+个服务模块**

---

## 🔍 质量检查清单

### 每个服务模块完成后检查：
- [ ] 运行检测工具：`cargo run --bin doc_url_detector`
- [ ] 确认覆盖率提升到目标水平
- [ ] 所有URL格式正确且可访问
- [ ] 文档描述清晰准确
- [ ] 代码编译无错误
- [ ] 运行相关测试通过

### 全局质量标准：
- [ ] URL标准化格式一致性
- [ ] 中文描述准确性和完整性
- [ ] 英文URL映射正确性
- [ ] 代码风格和格式统一
- [ ] 无破坏性变更

---

## 📋 工具和脚本

### 可用工具：
1. **检测工具**: `cargo run --bin doc_url_detector`
2. **格式化工具**: 标准化URL生成
3. **验证工具**: URL有效性检查
4. **批量处理脚本**: 按服务模块自动化处理

### 文档URL模板：
```rust
/// [API功能描述]
///
/// [详细功能说明]
///
/// <https://open.feishu.cn/document/server-docs/{service}-{version}/{category}/{action}>
pub async fn {method_name}(
    &self,
    request: {RequestType},
    option: Option<RequestOption>,
) -> SDKResult<{ResponseType}> {
    // 实现
}
```

---

## 📊 进度统计

### 已完成的服务模块：
- [ ] attendance (v1) - 目标: 100%覆盖
- [ ] cloud_docs (v1) - 目标: 95%+覆盖
- [ ] contact (v3) - 目标: 100%覆盖
- [ ] im (v1) - 目标: 95%+覆盖
- [ ] hire (v1) - 目标: 90%+覆盖
- [ ] directory (v1) - 目标: 95%+覆盖
- [ ] approval (v4) - 目标: 90%+覆盖
- [ ] 其他服务模块...

### 每日更新记录：
- **日期**:
- **完成的服务**:
- **新增文档URL数量**:
- **当前总覆盖率**:
- **遇到的问题和解决方案**:

---

## 🚀 开始执行

准备就绪，可以从cloud_docs服务的file模块开始实施！