# OpenLark-HR 462 API Implementation Plan

## TL;DR

> **Implement ALL 462 stub APIs** in openlark-hr crate with complete type definitions,
> validation, and documentation following existing patterns.
>
> **Total APIs**: 462 stubs across 8 modules
> **Execution Strategy**: 5 waves, conservative parallelism (2-3 modules/wave)
> **Estimated Duration**: 40-50 hours of development work
> **Quality Standard**: Strict - complete types, validation, Chinese docs

---

## Wave Summary

| Wave | Modules | APIs | Effort | Parallelizable |
|------|---------|------|--------|----------------|
| Wave 1 | ehr + okr | 14 | ~3 hrs | ✅ Yes |
| Wave 2 | payroll + attendance | 34 | ~7 hrs | ✅ Yes |
| Wave 3 | performance + compensation | 42 | ~9 hrs | ✅ Yes |
| Wave 4 | hire | 177 | ~35 hrs | ⚠️ Internal parallel by resource |
| Wave 5 | feishu_people/corehr | 195 | ~40 hrs | ⚠️ Internal parallel by resource |

**Total Estimated Effort**: ~94 hours (can parallelize across waves for ~50hr wall-clock)

---

## Module Breakdown

| Module | APIs | % of Total | Complexity |
|--------|------|-----------|------------|
| feishu_people/corehr | 195 | 42% | High (v1 + v2 APIs) |
| hire | 177 | 38% | High (complex recruitment flows) |
| attendance | 22 | 5% | Medium (mostly complete) |
| performance | 21 | 5% | Medium (v1 + v2) |
| compensation | 21 | 5% | Medium (batch operations) |
| payroll | 12 | 3% | Low (list/query heavy) |
| okr | 12 | 3% | Low (CRUD operations) |
| ehr | 2 | <1% | Low |

---

## Implementation Patterns

### Pattern A: Simple List/Query API (15-20 min/API)
```rust
// GET/POST with minimal params, returns list
pub struct ListRequest { config: Config, page_size: Option<i32> }
// ~80 lines total
```

### Pattern B: Standard CRUD with Body (20-30 min/API)
```rust
// POST/PUT with request body, validation
pub struct CreateRequest { config: Config, field1: String, field2: Option<String> }
// ~100-120 lines total
```

### Pattern C: Complex Batch Operations (30-45 min/API)
```rust
// Batch operations with validation, complex response
pub struct BatchCreateRequest { config: Config, items: Vec<Item> }
// Requires models.rs with multiple structs
// ~140-200 lines total
```

### Pattern D: File Upload/Download (20-25 min/API)
```rust
// Special handling for binary data
// ~90-110 lines total
```

---

## Wave 1: EHR + OKR (14 APIs)

**Goal**: Build momentum with simplest modules
**Parallelism**: ✅ Both modules can run concurrently
**Estimated Effort**: 3 hours total

### EHR Module (2 APIs) - 20 minutes

| File | API Name | Pattern | Effort |
|------|----------|---------|--------|
| `ehr/ehr/v1/attachment/get.rs` | 下载附件 | B | 10 min |
| `ehr/ehr/v1/employee/list.rs` | 获取员工列表 | A | 10 min |

**Success Criteria**:
- [x] Both APIs compile without warnings
- [x] Request/Response structs have complete field definitions
- [x] Validation using `validate_required!` macro for mandatory fields
- [x] Chinese documentation comments present
- [x] Endpoints use `EhrApiV1` enum

**Verification**:
```bash
cd crates/openlark-hr
cargo check --features full-suite 2>&1 | grep -E "(ehr|error)"
just lint
```

### OKR Module (12 APIs) - 2.5 hours

| File | API Name | Pattern | Effort |
|------|----------|---------|--------|
| `okr/okr/v1/image/upload.rs` | 上传图片 | D | 15 min |
| `okr/okr/v1/okr/batch_get.rs` | 批量获取OKR | B | 20 min |
| `okr/okr/v1/period/create.rs` | 创建周期 | B | 15 min |
| `okr/okr/v1/period/list.rs` | 获取周期列表 | A | 10 min |
| `okr/okr/v1/period/patch.rs` | 更新周期 | B | 15 min |
| `okr/okr/v1/period_rule/list.rs` | 获取周期规则 | A | 10 min |
| `okr/okr/v1/progress_record/create.rs` | 创建进度记录 | B | 15 min |
| `okr/okr/v1/progress_record/delete.rs` | 删除进度记录 | B | 15 min |
| `okr/okr/v1/progress_record/get.rs` | 获取进度记录 | B | 15 min |
| `okr/okr/v1/progress_record/update.rs` | 更新进度记录 | B | 15 min |
| `okr/okr/v1/review/query.rs` | 获取复盘信息 | A | 10 min |
| `okr/okr/v1/user/okr/list.rs` | 获取用户OKR | A | 10 min |

**Shared Models Needed**:
- `okr/okr/v1/period/models.rs` - Period, PeriodRule structs
- `okr/okr/v1/progress_record/models.rs` - ProgressRecord struct
- `okr/okr/v1/okr/models.rs` - Okr, OkrObjective structs

**Success Criteria**:
- [x] All 12 APIs compile
- [x] 3 models.rs files created with complete types
- [x] Consistent field naming with snake_case
- [x] Optional fields use `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
- [x] Date fields use String with format documentation

**Verification**:
```bash
cargo check --features full-suite 2>&1 | grep -c "error"  # Should be 0
find crates/openlark-hr/src/okr -name "*.rs" -exec wc -l {} + | tail -1  # Count lines
```

---

## Wave 2: Payroll + Attendance (34 APIs)

**Goal**: Scale to medium-complexity modules
**Parallelism**: ✅ Both modules can run concurrently
**Estimated Effort**: 7 hours total

### Payroll Module (12 APIs) - 2.5 hours

| File | API Name | Pattern | Effort |
|------|----------|---------|--------|
| `payroll/payroll/v1/acct_item/list.rs` | 获取会计科目 | A | 10 min |
| `payroll/payroll/v1/cost_allocation_detail/list.rs` | 成本分摊明细 | A | 10 min |
| `payroll/payroll/v1/cost_allocation_plan/list.rs` | 成本分摊计划 | A | 10 min |
| `payroll/payroll/v1/cost_allocation_report/list.rs` | 成本分摊报表 | A | 10 min |
| `payroll/payroll/v1/datasource/list.rs` | 获取数据源 | A | 10 min |
| `payroll/payroll/v1/datasource_record/query.rs` | 查询数据源记录 | B | 15 min |
| `payroll/payroll/v1/datasource_record/save.rs` | 保存数据源记录 | B | 15 min |
| `payroll/payroll/v1/paygroup/list.rs` | 获取薪资组 | A | 10 min |
| `payroll/payroll/v1/payment_activity/archive.rs` | 归档发薪活动 | B | 15 min |
| `payroll/payroll/v1/payment_activity/list.rs` | 获取发薪活动 | A | 10 min |
| `payroll/payroll/v1/payment_activity_detail/list.rs` | 发薪活动明细 | A | 10 min |
| `payroll/payroll/v1/payment_detail/query.rs` | 查询发薪明细 | B | 15 min |

**Success Criteria**:
- [x] All 12 APIs compile
- [x] Consistent pagination pattern (page_token, page_size)
- [x] Batch save operations have proper validation
- [x] Archive operations handle optional parameters

### Attendance Module (22 APIs) - 4.5 hours

| File | API Name | Pattern | Effort |
|------|----------|---------|--------|
| `attendance/attendance/v1/approval_info/process.rs` | 审批状态处理 | B | 20 min |
| `attendance/attendance/v1/archive_rule/del_report.rs` | 删除归档报表 | B | 15 min |
| `attendance/attendance/v1/archive_rule/list.rs` | 获取归档规则 | A | 10 min |
| `attendance/attendance/v1/archive_rule/upload_report.rs` | 上传归档报表 | C | 30 min |
| `attendance/attendance/v1/archive_rule/user_stats_fields_query.rs` | 查询统计字段 | B | 20 min |
| `attendance/attendance/v1/file/download.rs` | 下载文件 | D | 15 min |
| `attendance/attendance/v1/file/upload.rs` | 上传文件 | D | 15 min |
| `attendance/attendance/v1/group/list_user.rs` | 获取考勤组成员 | B | 15 min |
| `attendance/attendance/v1/leave_accrual_record/patch.rs` | 修改发放记录 | B | 15 min |
| `attendance/attendance/v1/leave_employ_expire_record/get.rs` | 获取过期记录 | B | 15 min |
| `attendance/attendance/v1/user_approval/create.rs` | 创建审批 | B | 15 min |
| `attendance/attendance/v1/user_approval/query.rs` | 查询审批 | B | 15 min |
| `attendance/attendance/v1/user_daily_shift/batch_create.rs` | 批量创建排班 | C | 30 min |
| `attendance/attendance/v1/user_daily_shift/batch_create_temp.rs` | 批量创建临时排班 | C | 30 min |
| `attendance/attendance/v1/user_daily_shift/query.rs` | 查询排班 | B | 15 min |
| `attendance/attendance/v1/user_setting/modify.rs` | 修改用户设置 | B | 15 min |
| `attendance/attendance/v1/user_setting/query.rs` | 查询用户设置 | B | 15 min |
| `attendance/attendance/v1/user_stats_view/query.rs` | 查询统计视图 | B | 15 min |
| `attendance/attendance/v1/user_stats_view/update.rs` | 更新统计视图 | B | 15 min |
| `attendance/attendance/v1/user_task_remedy/create.rs` | 创建补卡 | B | 15 min |
| `attendance/attendance/v1/user_task_remedy/query.rs` | 查询补卡 | B | 15 min |
| `attendance/attendance/v1/user_task_remedy/query_user_allowed_remedys.rs` | 查询可补卡时间 | B | 15 min |

**Complex APIs Requiring Models**:
- `user_flow` (already complete - use as reference)
- `archive_rule/upload_report` - needs ArchiveReportUploadRequest/Response
- `user_daily_shift/batch_create*` - needs ShiftBatchCreateRequest

**Success Criteria**:
- [x] All 22 APIs compile
- [x] File upload/download follow existing binary patterns
- [x] Batch operations validate item limits (e.g., max 100 items)
- [x] Query APIs support pagination consistently
- [x] Error handling uses `validation_error` from openlark_core

---

## Wave 3: Performance + Compensation (42 APIs)

**Goal**: Handle v1/v2 version complexity and batch operations
**Parallelism**: ✅ Both modules can run concurrently
**Estimated Effort**: 9 hours total

### Performance Module (21 APIs) - 4 hours

| File | API Name | Version | Pattern | Effort |
|------|----------|---------|---------|--------|
| `performance/performance/v1/review_data/query.rs` | 查询评审数据 | v1 | A | 10 min |
| `performance/performance/v1/semester/list.rs` | 获取学期列表 | v1 | A | 10 min |
| `performance/performance/v1/stage_task/find_by_page.rs` | 分页查询任务 | v1 | B | 15 min |
| `performance/performance/v1/stage_task/find_by_user_list.rs` | 按用户查询任务 | v1 | B | 15 min |
| `performance/performance/v2/activity/query.rs` | 查询活动 | v2 | A | 10 min |
| `performance/performance/v2/additional_information/import.rs` | 导入补充信息 | v2 | C | 25 min |
| `performance/performance/v2/additional_information/query.rs` | 查询补充信息 | v2 | B | 15 min |
| `performance/performance/v2/additional_informations/batch/delete.rs` | 批量删除 | v2 | B | 15 min |
| `performance/performance/v2/indicator/query.rs` | 查询指标 | v2 | A | 10 min |
| `performance/performance/v2/metric_detail/import.rs` | 导入指标详情 | v2 | C | 25 min |
| `performance/performance/v2/metric_detail/query.rs` | 查询指标详情 | v2 | B | 15 min |
| `performance/performance/v2/metric_field/query.rs` | 查询指标字段 | v2 | A | 10 min |
| `performance/performance/v2/metric_lib/query.rs` | 查询指标库 | v2 | A | 10 min |
| `performance/performance/v2/metric_tag/list.rs` | 获取指标标签 | v2 | A | 10 min |
| `performance/performance/v2/metric_template/query.rs` | 查询指标模板 | v2 | A | 10 min |
| `performance/performance/v2/question/query.rs` | 查询问题 | v2 | A | 10 min |
| `performance/performance/v2/review_data/query.rs` | 查询评审数据 | v2 | A | 10 min |
| `performance/performance/v2/review_template/query.rs` | 查询评审模板 | v2 | A | 10 min |
| `performance/performance/v2/reviewee/query.rs` | 查询被评人 | v2 | B | 15 min |
| `performance/performance/v2/user_group_user_rel/write.rs` | 写入用户组关系 | v2 | B | 15 min |
| `performance/performance/v2/user_info/query.rs` | 查询用户信息 | v2 | A | 10 min |

**Key Complexity**: v1 vs v2 endpoint enums

### Compensation Management Module (21 APIs) - 5 hours

| File | API Name | Pattern | Effort |
|------|----------|---------|--------|
| `compensation_management/compensation/v1/archive/create.rs` | 创建薪资档案 | B | 15 min |
| `compensation_management/compensation/v1/archive/query.rs` | 查询薪资档案 | B | 15 min |
| `compensation_management/compensation/v1/change_reason/list.rs` | 获取变更原因 | A | 10 min |
| `compensation_management/compensation/v1/indicator/list.rs` | 获取指标列表 | A | 10 min |
| `compensation_management/compensation/v1/item/list.rs` | 获取薪资项 | A | 10 min |
| `compensation_management/compensation/v1/item_category/list.rs` | 获取薪资项分类 | A | 10 min |
| `compensation_management/compensation/v1/lump_sum_payment/batch_create.rs` | 批量创建一次性支付 | C | 30 min |
| `compensation_management/compensation/v1/lump_sum_payment/batch_remove.rs` | 批量删除 | C | 25 min |
| `compensation_management/compensation/v1/lump_sum_payment/batch_update.rs` | 批量更新 | C | 30 min |
| `compensation_management/compensation/v1/lump_sum_payment/query.rs` | 查询 | B | 15 min |
| `compensation_management/compensation/v1/lump_sum_payment/query_detail.rs` | 查询详情 | B | 15 min |
| `compensation_management/compensation/v1/plan/list.rs` | 获取薪资方案 | A | 10 min |
| `compensation_management/compensation/v1/recurring_payment/batch_create.rs` | 批量创建周期性支付 | C | 30 min |
| `compensation_management/compensation/v1/recurring_payment/batch_remove.rs` | 批量删除 | C | 25 min |
| `compensation_management/compensation/v1/recurring_payment/batch_update.rs` | 批量更新 | C | 30 min |
| `compensation_management/compensation/v1/recurring_payment/query.rs` | 查询 | B | 15 min |
| `compensation_management/compensation/v1/social_archive/query.rs` | 查询社保档案 | B | 15 min |
| `compensation_management/compensation/v1/social_archive_adjust_record/query.rs` | 查询调整记录 | B | 15 min |
| `compensation_management/compensation/v1/social_insurance/list.rs` | 获取社保列表 | A | 10 min |
| `compensation_management/compensation/v1/social_plan/list.rs` | 获取社保方案 | A | 10 min |
| `compensation_management/compensation/v1/social_plan/query.rs` | 查询社保方案 | B | 15 min |

**Key Complexity**: Many batch operations with validation

---

## Wave 4: Hire Module (177 APIs)

**Goal**: Tackle the largest module with internal parallelism
**Parallelism**: ⚠️ Group by resource type within hire module
**Estimated Effort**: 35 hours total

### Resource Groups for Parallel Execution

#### Group A: Talent Management (20 APIs) - 4 hours
- `talent/*` - 12 APIs
- `talent_folder/*` - 1 API
- `talent_pool/*` - 3 APIs
- `talent_tag/*` - 1 API
- `talent_blocklist/*` - 1 API
- `talent_operation_log/*` - 1 API

#### Group B: Application & Interview (25 APIs) - 5 hours
- `application/*` - 12 APIs (including sub-resources)
- `interview/*` - 3 APIs
- `interview_feedback_form/*` - 1 API
- `interview_record/*` - 3 APIs
- `interview_registration_schema/*` - 1 API
- `interview_round_type/*` - 1 API
- `interviewer/*` - 2 APIs
- `minutes/*` - 1 API
- `evaluation/*` - 2 APIs

#### Group C: Job Management (18 APIs) - 3.5 hours
- `job/*` - 13 APIs (including sub-resources)
- `job_function/*` - 1 API
- `job_process/*` - 1 API
- `job_publish_record/*` - 1 API
- `job_requirement/*` - 5 APIs
- `job_requirement_schema/*` - 1 API
- `job_schema/*` - 1 API
- `job_type/*` - 1 API

#### Group D: Offer & Employee (15 APIs) - 3 hours
- `offer/*` - 8 APIs
- `offer_application_form/*` - 2 APIs
- `offer_approval_template/*` - 1 API
- `offer_custom_field/*` - 1 API
- `offer_schema/*` - 1 API
- `employee/*` - 3 APIs

#### Group E: Referral & Website (20 APIs) - 3.5 hours
- `referral/*` - 2 APIs
- `referral_account/*` - 6 APIs
- `referral_website/*` - 2 APIs
- `website/*` - 1 API
- `website_channel/*` - 4 APIs
- `website_delivery/*` - 2 APIs
- `website_delivery_task/*` - 1 API
- `website_job_post/*` - 3 APIs
- `website_site_user/*` - 1 API

#### Group F: External Data & Eco (30 APIs) - 6 hours
- `external_application/*` - 4 APIs
- `external_background_check/*` - 4 APIs
- `external_interview/*` - 4 APIs
- `external_interview_assessment/*` - 3 APIs
- `external_offer/*` - 4 APIs
- `external_referral_reward/*` - 2 APIs
- `eco_background_check/*` - 3 APIs
- `eco_background_check_package/*` - 3 APIs
- `eco_background_check_custom_field/*` - 3 APIs
- `eco_account_custom_field/*` - 3 APIs

#### Group G: Agency & Background Check (10 APIs) - 2 hours
- `agency/*` - 7 APIs
- `background_check_order/*` - 2 APIs
- `tripartite_agreement/*` - 4 APIs

#### Group H: Other Resources (19 APIs) - 3 hours
- `advertisement/*` - 1 API
- `attachment/*` - 3 APIs
- `diversity_inclusion/*` - 1 API
- `ehr_import_task/*` - 1 API
- `exam/*` - 2 APIs
- `note/*` - 5 APIs
- `portal_apply_schema/*` - 1 API
- `questionnaire/*` - 1 API
- `registration_schema/*` - 1 API
- `resume_source/*` - 1 API
- `role/*` - 2 APIs
- `subject/*` - 1 API
- `termination_reason/*` - 1 API
- `test/*` - 1 API
- `todo/*` - 1 API
- `user_role/*` - 1 API

#### Group I: V2 APIs (20 APIs) - 4 hours
- `hire/v2/interview_record/*` - 2 APIs
- `hire/v2/talent/*` - 1 API
- Plus 17 other v2 APIs distributed across resources

---

## Wave 5: Feishu People / CoreHR Module (195 APIs)

**Goal**: Complete the largest module with systematic approach
**Parallelism**: ⚠️ Group by resource domain
**Estimated Effort**: 40 hours total

### Domain Groups for Parallel Execution

#### Domain A: Basic Data & References (25 APIs) - 4 hours
- `country_region/*` - 2 APIs
- `currency/*` - 2 APIs
- `national_id_type/*` - 5 APIs
- `subdivision/*` - 2 APIs
- `subregion/*` - 2 APIs
- `working_hours_type/*` - 5 APIs
- `common_data/*` - 2 APIs
- `basic_info/*` (v2) - 10 APIs

#### Domain B: Authorization & Security (10 APIs) - 2 hours
- `authorization/*` - 5 APIs
- `security_group/*` - 2 APIs
- `assigned_user/*` - 1 API
- `custom_field/*` - 3 APIs

#### Domain C: Person & Pre-Hire (V1) (10 APIs) - 2 hours
- `person/*` (v1) - 6 APIs
- `pre_hire/*` (v1) - 4 APIs

#### Domain D: Organization Structure (V2) - Department & Job (35 APIs) - 6 hours
- `department/*` (v2) - 3 APIs
- `job/*` (v2) - 5 APIs
- `job_family/*` (v2) - 3 APIs
- `job_grade/*` (v2) - 6 APIs
- `job_level/*` (v2) - 2 APIs
- `job_change/*` (v1+v2) - 3 APIs
- `cost_center/*` (v2) - 5 APIs
- `cost_center/version/*` (v2) - 3 APIs
- `location/*` (v2) - 6 APIs
- `location/address/*` (v2) - 3 APIs

#### Domain E: Employee Management (V2) (25 APIs) - 5 hours
- `employees/additional_job/*` - 4 APIs
- `employees/bp/*` - 1 API
- `employees/international_assignment/*` - 4 APIs
- `employees/job_data/*` - 2 APIs
- `person/*` (v2) - 2 APIs
- `pre_hire/*` (v2) - 13 APIs

#### Domain F: Process & Workflow (20 APIs) - 4 hours
- `process/*` (v2) - 7 APIs
- `process_revoke/*` - 1 API
- `process_withdraw/*` - 1 API
- `process/approver/*` - 1 API
- `process/extra/*` - 1 API
- `process/form_variable_data/*` - 1 API
- `process/transfer/*` - 1 API
- `approval_groups/*` (v2) - 4 APIs
- `approver/*` (v2) - 1 API
- `draft/*` (v2) - 1 API

#### Domain G: Position & Pathway (15 APIs) - 3 hours
- `position/*` (v2) - 7 APIs
- `pathway/*` (v2) - 6 APIs
- Custom org: `custom_org/*` (v2) - 7 APIs

#### Domain H: Offboarding & Probation (15 APIs) - 3 hours
- `offboarding/*` (v1) - 3 APIs
- `offboarding/*` (v2) - 3 APIs
- `probation/*` (v2) - 8 APIs
- `probation/assessment/*` (v2) - 3 APIs

#### Domain I: Cost & Workforce Planning (15 APIs) - 3 hours
- `cost_allocation/*` (v2) - 4 APIs
- `default_cost_center/*` (v2) - 4 APIs
- `workforce_plan/*` (v2) - 1 API
- `workforce_plan_detail/*` (v2) - 2 APIs
- `workforce_plan_detail_row/*` (v2) - 2 APIs

#### Domain J: Signature & Documents (15 APIs) - 3 hours
- `signature_file/*` (v2) - 5 APIs
- `signature_node/*` (v2) - 1 API
- `signature_template/*` (v2) - 1 API
- `signature_template_info_with_thumbnail/*` - 1 API
- `file/*` (v1) - 1 API
- `report_detail_row/*` (v2) - 2 APIs

#### Domain K: Leave & Benefits (10 APIs) - 2 hours
- `leave/*` (v1) - 4 APIs
- `leave_granting_record/*` (v1) - 2 APIs
- `compensation_standard/*` (v1) - 1 API
- Other leave-related APIs

#### Domain L: BP & Company (10 APIs) - 2 hours
- `bp/*` (v2) - 2 APIs
- `company/*` (v2) - 2 APIs
- `enum/*` (v2) - 1 API
- Other organizational APIs

---

## Success Criteria Per API

### Required Elements Checklist
- [x] **Request Struct**: All required fields as direct members, optional as `Option<T>`
- [x] **Builder Methods**: `new()`, setter for each field returning `Self`
- [x] **Execute Methods**: `execute()` and `execute_with_options()`
- [x] **Validation**: Use `validate_required!` for mandatory fields
- [x] **Response Struct**: Complete field definitions with proper types
- [x] **Endpoint**: Use enum from `common::api_endpoints`, never hardcode URLs
- [x] **Documentation**: Chinese comments explaining purpose and fields
- [x] **Models**: Separate `models.rs` for complex request/response bodies

### Code Quality Gates
- [x] `cargo check` passes with zero errors
- [x] `cargo clippy` passes with zero warnings
- [x] `cargo fmt` produces no changes
- [x] All `todo!()` macros removed from implemented files
- [x] No dead code warnings (use `#[allow(dead_code)]` where appropriate)

### Validation Requirements
- [x] Required string fields validated with `validate_required!`
- [x] Array length limits checked (e.g., max 100 items for batch)
- [x] Numeric ranges validated where applicable
- [x] Date format validation for string dates

### Testing Requirements
- [x] Compile-time type safety verified
- [x] Serialization/deserialization round-trips work
- [x] Endpoint URL generation correct

---

## Verification Commands

### Per-Wave Verification

```bash
# 1. Compile check
cargo check -p openlark-hr --all-features

# 2. Lint check
just lint
# Or: cargo clippy -p openlark-hr --all-features -- -D warnings

# 3. Format check
just fmt-check
# Or: cargo fmt -p openlark-hr -- --check

# 4. Count remaining stubs
grep -r "todo!" crates/openlark-hr/src --include="*.rs" | wc -l

# 5. Count completed APIs (lines > 70 typically means implemented)
find crates/openlark-hr/src -name "*.rs" -type f -exec wc -l {} \; | \
  awk '$1 > 70 {count++} END {print "Files with >70 lines:", count}'

# 6. Module-specific verification
cargo check -p openlark-hr --features hr-okr
cargo check -p openlark-hr --features hr-hire
cargo check -p openlark-hr --features hr-corehr
```

### Final Verification

```bash
# Full build
just build

# Run tests
just test

# Validate all stubs are gone
grep -r "todo!" crates/openlark-hr/src --include="*.rs"
# Should return nothing

# Count total API implementations
find crates/openlark-hr/src -name "*.rs" -type f ! -name "mod.rs" ! -name "models.rs" | wc -l

# Verify no TODO comments remain
grep -r "TODO:" crates/openlark-hr/src --include="*.rs" | wc -l
```

---

## Risk Mitigation

### Documentation Gaps
**Risk**: Official API docs incomplete or ambiguous field types
**Mitigation**:
1. Use `serde_json::Value` for truly unclear nested objects
2. Reference similar APIs in same module for type patterns
3. Mark unclear fields with `// NOTE: Type inferred from similar API`
4. Fetch docPath content: `python3 .claude/skills/openlark-api/scripts/fetch_docpath.py "<url>"`

### Type Complexity
**Risk**: Deeply nested request/response structures
**Mitigation**:
1. Flatten where possible using `#[serde(flatten)]`
2. Extract common sub-structs to models.rs
3. Use builder pattern for complex nested construction

### Batch Operation Limits
**Risk**: API has undocumented batch size limits
**Mitigation**:
1. Add client-side validation with reasonable limits (e.g., 100 items)
2. Document assumed limits in code comments
3. Return clear validation errors

### V1 vs V2 Endpoint Confusion
**Risk**: Mixing v1 and v2 endpoint enums
**Mitigation**:
1. Always check `common/api_endpoints.rs` for correct enum variant
2. Verify docPath URL matches enum URL
3. Use `HireApiV1` for v1, `HireApiV2` for v2

---

## Reference Files

### Complete Implementations (Use as Templates)
1. `attendance/attendance/v1/user_flow/batch_create.rs` - Pattern C (batch with validation)
2. `attendance/attendance/v1/user_flow/query.rs` - Pattern B (query with params)
3. `attendance/attendance/v1/user_flow/models.rs` - Complex models example

### Key Support Files
- `common/api_endpoints.rs` - All endpoint enums
- `common/service.rs` - Service trait patterns
- `common/models.rs` - Shared HR types
- `common/macros.rs` - Validation macros

### Documentation Resources
- `api_list_export.csv` - API metadata with docPath URLs
- `.claude/skills/openlark-api/` - API implementation skill
- Feishu Open API docs: https://open.feishu.cn/document/

---

## Execution Order Summary

```
Wave 1: ehr (2) + okr (12) = 14 APIs
  ↓ [Parallel]
Wave 2: payroll (12) + attendance (22) = 34 APIs
  ↓ [Parallel]
Wave 3: performance (21) + compensation (21) = 42 APIs
  ↓ [Sequential with internal parallel groups]
Wave 4: hire (177 APIs across 9 groups)
  ↓ [Sequential with internal parallel groups]
Wave 5: feishu_people/corehr (195 APIs across 12 domains)
  ↓
Final Verification & Integration
```

**Estimated Timeline**:
- Wave 1: 0.5 days
- Wave 2: 1 day
- Wave 3: 1.5 days
- Wave 4: 4-5 days
- Wave 5: 5-6 days
- **Total**: ~12-14 days with conservative parallelism

---

## Session ID

**Session**: `openlark-hr-462-apis`
**Created**: 2026-01-28
**Status**: Planning Complete - Ready for Execution
**Next Step**: Run `/start-work` to begin Wave 1 implementation
