# Hire generated stub audit

更新时间：2026-04-17

## 背景

`#106` 处理的是 `crates/openlark-hr/src/hire/hire/` 下由生成骨架遗留的 shipped-source TODO/FIXME。

在本轮清理前，`tools/audit_todos.py` 对 Hire 目录统计出：

- **243** 条 TODO/FIXME 注释
- 分布在 **177** 个源码文件中

这些注释主要有两类：

1. **空请求骨架**
   - `// TODO: 添加请求字段`
   - `// TODO: 初始化字段`
   - `// TODO: 添加字段 setter 方法`
2. **响应 schema 占位**
   - `/// TODO: 根据官方文档添加具体字段`

## 本轮处理

### 1. 把 shipped-source TODO 迁移为显式占位说明

为了避免“源码里看起来还没做完，但 issue / 文档里又没有追踪”的双重失真，本轮把 Hire 目录中的 TODO 注释统一替换为更明确的占位说明：

- 空请求骨架改成“当前尚未建模请求字段；补齐 schema 前保持零字段请求”
- `Value` 响应占位改成“当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构”

这样做的目标不是“假装 schema 已完成”，而是把 debt 从源码噪音迁移到可追踪的 issue / 文档层。

### 2. 固化剩余 schema debt 的结构统计

清理后，Hire 目录里不再保留 TODO/FIXME 标记。随着 #112 / #113 持续推进，当前剩余的 schema debt 为：

- **0 个文件**：零字段请求骨架已全部消除
- **1 个接口**：`talent_object/query` 已确认是官方无参请求，不再视为骨架
- **50 个文件**：响应仍然是 `Value` 直透，需要后续 typed 化

## 分类结论

### A. 零字段请求骨架（已完成）

`#111` 已完成以下收敛：

- 21 个接口已补齐 query/body/path builder 字段
- `crates/openlark-hr/src/hire/hire/v1/talent_object/query.rs` 已确认官方本身无请求参数，因此保留为显式无参请求
- 新增定向验证：`crates/openlark-hr/tests/hire_request_modeling_tests.rs`

已不再存在“零字段请求骨架仍待建模”的剩余项。

### B. `Value` 响应直透（当前 50 files）

这批接口已经具备 endpoint/execute 入口，但公共返回类型仍是 `pub data: Value`。这意味着：

- 运行时可用
- Rust 层 contract 仍弱
- 字段级兼容性无法通过类型系统表达

优先建议聚焦：

- offer / note / job / interview / referral / website / attachment
- v2 `interview_record` / `talent`
- 外部招聘 / 外部面试 / 背调相关路径

跟踪 issue：**#113 Continue replacing remaining Hire Value pass-through responses**

`#113` 现在是 Hire schema debt 的主跟踪入口。

## 决策

本轮 `#106` 的定位是：

- **完成审计**
- **清理 shipped-source TODO 噪音**
- **把剩余 schema debt 转移到明确 issue 跟踪**

而不是在一个 issue 里一次性为 177 个接口补完所有 typed schema。

## 退出标准

`#106` 可以关闭的条件：

- Hire shipped source 中不再保留骨架式 TODO/FIXME
- 零字段请求骨架已在 `#111` 中完成收敛
- `Value` 响应直透已有独立跟踪（#113）
- 审计结论已固化到仓库文档


## #112 首批 typed response 收敛结果

`#112` 已完成第一批高价值 Hire response typed 化，覆盖：

- role / subject / website / talent_folder / talent_tag / termination_reason / todo
- location / interviewer / evaluation / evaluation_task / exam_marking_task
- minutes / referral.search / referral_account.create / referral_account.reconciliation
- interview_record.list（v1 / v2）

同时新增：

- `crates/openlark-hr/src/hire/hire/common_models.rs`
- `crates/openlark-hr/tests/hire_response_contract_tests.rs`

当前剩余 `pub data: Value` 直透响应数：**159**

后续统一由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪。


## #113 继续收敛进展

本轮继续完成第二批 Hire typed response 收敛，新增覆盖：

- user_role.list
- referral_website.job_post.list
- talent_pool.search
- interview_record.attachment.get
- talent_object.query
- referral_account.get_account_assets
- v2 talent.get

并顺手补齐了：

- `referral_account/get_account_assets` 的真实查询参数透传
- `v2/talent/get` 的 `user_id_type` 查询参数透传

当前剩余 `pub data: Value` 直透响应数：**138**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪。

## #113 第三、四批收敛进展

在前两批基础上，`#113` 又继续完成了：

- note.create / get / list / patch / delete
- attachment.create / get / preview
- offer.create / list / update / offer_status / intern_offer_status
- offer_application_form.list
- offer.get
- offer_application_form.get
- offer_schema.get

同步补强：

- `crates/openlark-hr/tests/hire_tests.rs`
- `crates/openlark-hr/tests/hire_response_contract_tests.rs`

当前剩余 `pub data: Value` 直透响应数：**135**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 application / job / external_* 等剩余高价值路径。

## #113 第五批收敛进展

本轮继续完成 application 主路径的 typed response 收敛：

- application.create
- application.get
- application.get_detail
- application.list
- application.interview.list

同时复用到 `common_models.rs` 的通用结构包括：

- `ApplicationJobInfo`
- `ApplicationTalentInfo`
- `ApplicationOfferInfo`
- `ApplicationSummary`
- `ApplicationInterviewRecord`

当前剩余 `pub data: Value` 直透响应数：**130**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 job / external_* / agency 等剩余高价值路径。

## #113 第六批收敛进展

本轮继续完成 job 主路径的 typed response 收敛：

- job.combined_create
- job.get
- job.get_detail
- job.combined_update
- job.list
- job.open
- job.close
- job.config
- job.recruiter
- job.update_config

同时新增并复用的通用结构包括：

- `JobRecruiterRecord`
- `JobSummary`
- `JobConfigInfo`

当前剩余 `pub data: Value` 直透响应数：**120**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 external_* / agency / interview / website 余下高价值路径。

## #113 第七批收敛进展

本轮继续完成 external application / external offer 主路径的 typed response 收敛：

- external_application.create / delete / list / update
- external_offer.create / delete / batch_query / update

同时新增并复用的通用结构包括：

- `ExternalApplicationSummary`
- `ExternalOfferSummary`

当前剩余 `pub data: Value` 直透响应数：**112**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 agency / website / talent / job_requirement 等剩余高价值路径。

## #113 第八批收敛进展

本轮继续完成 website 主路径的 typed response 收敛：

- website.channel.create / delete / list / update
- website.delivery.create_by_attachment / create_by_resume
- website.delivery_task.get
- website.job_post.get / list / search
- website.site_user.create

同时新增并复用的通用结构包括：

- `WebsiteChannelSummary`
- `WebsiteDeliveryTaskResult`
- `WebsiteJobPostSummary`
- `WebsiteSiteUserSummary`

当前剩余 `pub data: Value` 直透响应数：**101**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 agency / talent / job_requirement / application 剩余 mutation 等路径。

## #113 第九批收敛进展

本轮继续完成 agency 主路径的 typed response 收敛：

- agency.batch_query
- agency.get
- agency.get_agency_account
- agency.operate_agency_account
- agency.protect
- agency.protect_search
- agency.query

同时新增并复用的通用结构包括：

- `AgencySummary`
- `AgencyAccountSummary`
- `AgencyProtectionSummary`

当前剩余 `pub data: Value` 直透响应数：**94**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 talent / job_requirement / application 剩余 mutation / external_background_check / external_interview 等路径。

## #113 第十批收敛进展

本轮继续完成 talent / job_requirement 主路径的 typed response 收敛：

- talent.add_to_folder
- talent.remove_to_folder
- talent.onboard_status
- talent.tag
- talent.external_info.create / update
- job_requirement.create / delete / list / list_by_id / update

同时新增并复用的通用结构包括：

- `TalentExternalInfoRecord`
- `JobRequirementSummary`

当前剩余 `pub data: Value` 直透响应数：**83**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 application 剩余 mutation / external_background_check / external_interview / tripartite_agreement 等路径。

## #113 第十一批收敛进展

本轮继续完成 application mutation 主路径的 typed response 收敛：

- application.cancel_onboard
- application.offer
- application.recover
- application.terminate
- application.transfer_onboard
- application.transfer_stage

同时新增并复用的通用结构包括：

- `ApplicationOperationResult`

当前剩余 `pub data: Value` 直透响应数：**77**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 external_background_check / external_interview / tripartite_agreement / 其余 low-volume 域。

## #113 第十二批收敛进展

本轮继续完成 external_background_check / external_interview / tripartite_agreement 主路径的 typed response 收敛：

- external_background_check.batch_query / create / delete / update
- external_interview.batch_query / create / delete / update
- tripartite_agreement.create / delete / list / update

同时新增并复用的通用结构包括：

- `ExternalBackgroundCheckSummary`
- `ExternalInterviewSummary`
- `TripartiteAgreementSummary`

当前剩余 `pub data: Value` 直透响应数：**65**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清剩余 low-volume 业务域。

## #113 第十三批收敛进展

本轮继续完成低频生态域 typed response 收敛：

- eco_account_custom_field.create / batch_update / batch_delete
- eco_background_check_custom_field.create / batch_update / batch_delete
- eco_background_check_package.create / batch_update / batch_delete
- eco_exam_paper.create / batch_update / batch_delete
- eco_background_check.cancel / update_progress / update_result

同时新增并复用的通用结构包括：

- `EcoCustomFieldSummary`
- `EcoBackgroundCheckPackageSummary`
- `EcoExamPaperSummary`
- `EcoOperationResult`

当前剩余 `pub data: Value` 直透响应数：**50**。

后续仍由：**#113 Continue replacing remaining Hire Value pass-through responses** 跟踪，并优先继续清 employee / referral_account / interview / background_check_order / external_referral_reward / talent_pool / external_interview_assessment 等低频余项。
