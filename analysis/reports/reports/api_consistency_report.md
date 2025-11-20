# API设计一致性检查报告

生成时间: 2025-11-08 10:11:41 UTC

## 📊 总体统计

- 检查的服务文件数: 668
- 平均一致性得分: 21%
- 发现的问题总数: 3748

## 🔍 服务详细分析

### feishu_people 服务

#### unknown - src/service/feishu_people/core/v1/positions.rs
**一致性得分**: 79%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_by_department`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_sequences`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_statistics`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_holders`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_positions_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_positions_by_department_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_positions_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_position_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_position_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_position_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_sequences_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_statistics_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_position_holders_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `position_ids`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `position_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `position_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/feishu_people/core/v1/persons.rs
**一致性得分**: 54%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_by_department`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_avatar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_avatar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_basic_info`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_person_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_persons_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_persons_by_department_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_persons_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_person_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_person_status_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_person_avatar_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_person_avatar_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_person_basic_info_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `person_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `image_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/feishu_people/core/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/core/v1/departments.rs
**一致性得分**: 79%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_sub_departments`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_root_departments`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_members`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_statistics`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_department_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_departments_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_sub_departments_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_root_departments_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_departments_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_department_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_department_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_department_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_department_members_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_department_statistics_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_ids`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/feishu_people/core/v1/companies.rs
**一致性得分**: 79%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_statistics`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization_structure`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_subsidiaries`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_company_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_companies_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_companies_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_company_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_company_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_company_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_company_statistics_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization_structure_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_subsidiaries_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `company_ids`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `company_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `company_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `max_depth`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/feishu_people/core/v1/contracts.rs
**一致性得分**: 13%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_by_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `terminate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `renew`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_expiring_contracts`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contract_data`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_data`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reason`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `termination_date`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new_end_date`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `renewal_terms`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `department_id`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_contract_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_contracts_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_contracts_by_user_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_contracts_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_contract_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_contract_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `terminate_contract_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `renew_contract_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_expiring_contracts_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_contract_statistics_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'user_id_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'user_id_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'contract_data' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'update_data' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'reason' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'termination_date' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new_end_date' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'renewal_terms' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'department_id' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_contract_builder' 缺少Builder模式实现
   💡 建议为 'create_contract_builder' 创建对应的Builder结构体

#### unknown - src/service/feishu_people/core/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/leaves/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/leaves/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/authorizations/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/authorizations/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/basic_data/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/feishu_people/basic_data/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### payroll 服务

#### v1 - src/service/payroll/v1/calculation.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `calculate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_calculate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `simulate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_calculation_history`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'calculate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_calculate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'simulate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_calculation_history' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/payroll/v1/paygroup.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_paygroups`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_salary_rules`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_salary_rules`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_employees_to_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove_employees_from_paygroup`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_paygroup_employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_paygroup_statistics`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_paygroup' 缺少Builder模式实现
   💡 建议为 'create_paygroup' 创建对应的Builder结构体
🔴 方法 'create_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_paygroups' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_salary_rules' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_salary_rules' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_employees_to_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove_employees_from_paygroup' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_paygroup_employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_paygroup_statistics' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/payroll/v1/report.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `generate_monthly_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `generate_annual_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `generate_employee_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_reports`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `export_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `distribute_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_salary_overview`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'generate_monthly_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'generate_annual_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'generate_employee_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_reports' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'export_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'distribute_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_salary_overview' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/payroll/v1/payment.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_payment_activity`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_payment_activity`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_payment_activities`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_payment_activity`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `cancel_payment_activity`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_payment_details`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_get_payment_details`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `retry_payment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute_payment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_payment_execution_status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `stop_payment_execution`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_payment_activity' 缺少Builder模式实现
   💡 建议为 'create_payment_activity' 创建对应的Builder结构体
🔴 方法 'create_payment_activity' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_payment_activity' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_payment_activities' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_payment_activity' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'cancel_payment_activity' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_payment_details' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_get_payment_details' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'retry_payment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute_payment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_payment_execution_status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'stop_payment_execution' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/payroll/v1/acct_item.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_acct_item`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_acct_item`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_acct_item`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_acct_item`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_acct_items`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_acct_item_formula`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_acct_item_formula`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `test_acct_item_formula`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_acct_item_template`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_acct_item_templates`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_acct_item_usage_stats`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_import_acct_items`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_acct_item' 缺少Builder模式实现
   💡 建议为 'create_acct_item' 创建对应的Builder结构体
🔴 方法 'create_acct_item' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_acct_item' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_acct_item' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_acct_item' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_acct_items' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_acct_item_formula' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_acct_item_formula' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'test_acct_item_formula' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_acct_item_template' 缺少Builder模式实现
   💡 建议为 'create_acct_item_template' 创建对应的Builder结构体
🔴 方法 'create_acct_item_template' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_acct_item_templates' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_acct_item_usage_stats' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_import_acct_items' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/payroll/v1/datasource.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_datasource`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_datasource`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_datasource`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_datasource`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_datasources`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sync_data`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_sync_history`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `test_datasource_connection`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_data_quality_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `clean_data`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_datasource_stats`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_datasource' 缺少Builder模式实现
   💡 建议为 'create_datasource' 创建对应的Builder结构体
🔴 方法 'create_datasource' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_datasource' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_datasource' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_datasource' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_datasources' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sync_data' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_sync_history' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'test_datasource_connection' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_data_quality_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'clean_data' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_datasource_stats' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/payroll/payment_activity/mod.rs
**一致性得分**: 70%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_payment_activities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_payment_activity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `archive_payment_activity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_payment_activity`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/payroll/payment_detail/mod.rs
**一致性得分**: 70%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_payment_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_payment_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_payment_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `export_payment_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/payroll/paygroup/mod.rs
**一致性得分**: 66%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_paygroups`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `activate_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `deactivate_paygroup`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_paygroup' 缺少Builder模式实现
   💡 建议为 'create_paygroup' 创建对应的Builder结构体

### approval 服务

#### unknown - src/service/approval/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/file/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `upload`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/instance.rs
**一致性得分**: 62%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `withdraw`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `urge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_my_approval_stats`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_department_approval_stats`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v4 - src/service/approval/v4/message/mod.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `send`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/task.rs
**一致性得分**: 68%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `process`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `approve`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reject`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `transfer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rollback`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_approver`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_task_operations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `urge_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/search/mod.rs
**一致性得分**: 77%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_instances`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_tasks`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_cc`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_approval_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_user_tasks`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_instances_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_tasks_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_cc_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_approval_id_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_user_tasks_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `approval_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `instance_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `approval_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `instance_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `task_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `approval_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `instance_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `approval_name`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `approval_code`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `task_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### v4 - src/service/approval/v4/external_task/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'list' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/instance_comment/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/external_instance/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `check`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/approval/v4/approval.rs
**一致性得分**: 58%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_templates`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_from_template`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_approval_permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_approval_permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_from_template' 缺少Builder模式实现
   💡 建议为 'create_from_template' 创建对应的Builder结构体

#### v4 - src/service/approval/v4/external_approval/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### moments 服务

#### unknown - src/service/moments/post/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_post`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/moments/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/moments/events/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `dispatch_event`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'dispatch_event' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'w' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### mdm 服务

#### unknown - src/service/mdm/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/mdm/country_region/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/mdm/user_auth_data_relation/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `bind`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unbind`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### security_and_compliance 服务

#### v1 - src/service/security_and_compliance/v1/compliance_management.rs
**一致性得分**: 68%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_compliance_overview`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `standards`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `include_risk_assessment`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `include_recommendations`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `evaluation_period`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_compliance_overview_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_compliance_overview_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/security_and_compliance/v1/security_monitoring.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_real_time_security_events`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_security_posture_analysis`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_anomaly_detection_results`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_anomaly_detection_results_mock`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_attack_chain_analysis`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attack_chain_analysis_mock`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `limit`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `severity_filter`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `event_type_filter`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_filter`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `resource_type_filter`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `analysis_types`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `start_time`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `end_time`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `entity_types`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `confidence_threshold`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `severity_levels`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `attack_chain_id`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `include_indicators`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `include_ttp`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_real_time_security_events_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_security_posture_analysis_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_anomaly_detection_results_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_attack_chain_analysis_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'get_anomaly_detection_results_mock' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_attack_chain_analysis_mock' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'start_time' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'end_time' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'analysis_types' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'start_time' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'end_time' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'entity_types' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'confidence_threshold' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'severity_levels' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'attack_chain_id' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'include_indicators' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'include_ttp' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_real_time_security_events_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_security_posture_analysis_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_anomaly_detection_results_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_attack_chain_analysis_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/security_and_compliance/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/security_and_compliance/v1/access_control.rs
**一致性得分**: 24%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_access_permissions_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_access_permissions`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_access_policy`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_access_policy_mock`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_user_permission_summary`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_user_permission_summary_mock`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `resource_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `resource_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `permission_levels`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `policy_name`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `policy_type`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `description`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `target_resources`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `target_subjects`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `permission_level`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `user_id`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `resource_types`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_access_permissions_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `create_access_policy_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档
- `get_user_permission_summary_builder`: ✅ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_access_permissions_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'create_access_policy_mock' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_user_permission_summary_mock' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'policy_name' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'policy_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'description' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'target_resources' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'target_subjects' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'permission_level' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'user_id' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'resource_types' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_access_permissions_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'create_access_policy_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get_user_permission_summary_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/security_and_compliance/v1/risk_assessment.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_risk_assessment_results`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_risk_matrix`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_risk_monitoring_dashboard`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_risk_assessment_results' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_risk_matrix' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_risk_monitoring_dashboard' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/security_and_compliance/v1/audit_trail.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `search_audit_logs`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_audit_log_details`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `generate_audit_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'search_audit_logs' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_audit_log_details' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'generate_audit_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/security_and_compliance/v1/security_policy.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_security_policies`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_security_policy`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `evaluate_policy_compliance`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_policy_enforcement_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_security_policies' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_security_policy' 缺少Builder模式实现
   💡 建议为 'create_security_policy' 创建对应的Builder结构体
🔴 方法 'create_security_policy' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'evaluate_policy_compliance' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_policy_enforcement_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/security_and_compliance/audit_log/mod.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `audit_data_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/security_and_compliance/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/security_and_compliance/openapi_log/mod.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### cloud_docs 服务

#### v1 - src/service/cloud_docs/v1/assistant.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cloud_docs/v1/board.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cloud_docs/v1/drive.rs
**一致性得分**: 51%

**API方法分析**:
- `is_completed`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_success`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_failed`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_progress_percentage`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_progress_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `export_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_task_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_task_status_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_file`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_file_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_export_task`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_export_task_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `task_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_tokens`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `export_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `quality`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_comments`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `watermark`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_export_task_builder' 缺少Builder模式实现
   💡 建议为 'create_export_task_builder' 创建对应的Builder结构体

#### v1 - src/service/cloud_docs/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/cloud_docs/v1/wiki.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cloud_docs/v1/comments.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/assistant/v1/subscription/create.rs
**一致性得分**: 40%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_doc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_slide`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_whiteboard`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_mindnote`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `basic_subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `high_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `urgent_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `notification_interval`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `auto_renew`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_tag`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `extra`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_notification`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_notification_interval`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_notification_interval_hours`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_high_frequency`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_auto_renew`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_tags`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_tag`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type_enum`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_time_formatted`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `full_summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_time_formatted' 缺少Builder模式实现
   💡 建议为 'create_time_formatted' 创建对应的Builder结构体
🟡 方法 'create_subscription' 缺少Builder模式实现
   💡 建议为 'create_subscription' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/assistant/v1/subscription/patch.rs
**一致性得分**: 23%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `activate`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `pause`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `resume`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `notification`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `notification_interval`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `quick_notification`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `standard_notification`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `slow_notification`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `high_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `low_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `urgent_priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `auto_renew`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_tag`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `remove_tag`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `clear_tags`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `extra`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `safe_pause`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `quick_activate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `eco_activate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch_subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'activate' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/assistant/v1/subscription/get.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/assistant/v1/subscription/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_subscribe_doc`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_subscribe_sheet`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_subscribe_slide`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_subscribe_whiteboard`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_subscribe_mindnote`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `premium_subscribe_doc`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `urgent_subscribe_sheet`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `activate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pause`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `cancel`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quick_activate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `eco_activate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `safe_pause`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_subscribed`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_subscribe`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_activate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_pause`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_subscribe_doc' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_subscribe_sheet' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_subscribe_slide' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_subscribe_whiteboard' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_subscribe_mindnote' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'premium_subscribe_doc' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'urgent_subscribe_sheet' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'activate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pause' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'cancel' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quick_activate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'eco_activate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'safe_pause' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_subscribed' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_subscribe' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_activate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_pause' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/assistant/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/assistant/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/comments/list.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `whole_comments_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `partial_comments_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_whole`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `solved_comments_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsolved_comments_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_solved`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_solved_info`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_pagination`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_whole`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_quote`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reply_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `first_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `last_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_solved_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_whole_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_update_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_solved_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_extra`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_update_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `plain_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contains_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `plain_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `len`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_element`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_run`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `plain_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_text_run`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `italic`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `underline`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_italic`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_underline`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `solved_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsolved_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `whole_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `partial_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `first_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `last_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_comment_by_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comments_by_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_desc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_asc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_comments`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/list_replies.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reply_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_more_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `next_page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `first_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `last_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_desc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_asc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replies_by_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replies_containing_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/create.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `content`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `styled_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bold_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `italic_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `underline_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_whole`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_solved`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_whole_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_styled_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_italic`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_underline`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_strikethrough`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_comment' 缺少Builder模式实现
   💡 建议为 'create_comment' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/comments/patch.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `solve`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsolve`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_solved`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_solved_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_solved_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/get.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `plain_text_content`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contains_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reply_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_more_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `next_page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_resolved`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_whole_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_quote`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_solved_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_update_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/mod.rs
**一致性得分**: 67%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_replies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/comments/update_reply.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reply_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `plain_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contains_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_update_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/batch_query.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_comment_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `success_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `failed_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_all_successful`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `success_rate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `failed_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_comment_by_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `solved_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsolved_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `whole_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `partial_comment_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_desc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sorted_by_create_time_asc`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comments_by_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_query_comments`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/comments/delete_reply.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_docx_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reply_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_open_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_user_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_union_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_delete_time`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `lifetime_ms`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `lifetime_seconds`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/batch_get.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_get_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/batch_create.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_create_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/create.rs
**一致性得分**: 26%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_record' 缺少Builder模式实现
   💡 建议为 'create_record' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/delete.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/update.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/batch_delete.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_delete_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/batch_update.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_update_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/app_table_record/search.rs
**一致性得分**: 6%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `search_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `or`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `not_equals`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `contains`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `not_contains`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `is_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `is_not_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `greater_than`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `less_than`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'or' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'not_equals' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/form/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_form_questions`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/form/patch.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch_form_question`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/form/mod.rs
**一致性得分**: 56%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app/copy.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app/update.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app/get.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/list.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/batch_create.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_create_role_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/delete.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete_role_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/batch_delete.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_delete_role_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role_member/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/app_table/list.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/batch_create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/create.rs
**一致性得分**: 4%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_default_view_name`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `number`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `single_select`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `multi_select`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `date`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_default_view_name' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'number' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/patch.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/delete.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/batch_delete.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/app_dashboard/list.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_dashboards`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_dashboards_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_dashboards' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_dashboards_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/bitable/v1/app_dashboard/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/bitable/v1/app_workflow/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_workflows`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_workflow/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/list.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/create.rs
**一致性得分**: 10%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `grid_view`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `kanban_view`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `gallery_view`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `gantt_view`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_view_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_property`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'grid_view' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/patch.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/delete.rs
**一致性得分**: 20%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete_view`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'delete_view' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/get.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_view/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_field/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_field/create.rs
**一致性得分**: 26%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_field' 缺少Builder模式实现
   💡 建议为 'create_field' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/bitable/v1/app_table_field/update.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_table_field/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/v1/app_role/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_app_roles`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role/delete.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete_app_role`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role/update.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update_app_role`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/bitable/v1/app_role/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/bitable/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new_from_shared' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/spreadsheet_sheet/operate_sheets.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/spreadsheet_sheet/update_sheet_properties.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/sheet_row_col/delete_dimension_range.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/sheet_row_col/update_dimension_range.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/sheet_row_col/add_dimension_range.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/sheet_row_col/insert_dimension_range.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/append_data.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/split_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/write_image.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/merge_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/batch_set_cell_style.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/set_cell_style.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/prepend_data.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/write_data_to_a_single_range.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/reading_multiple_range.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/write_data_to_multi_ranges.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v2/data_operation/reading_a_single_range.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet/create.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet/patch.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet/get.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/sheets/v3/protect_range/create.rs
**一致性得分**: 11%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `row_range`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `column_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'row_range' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/protect_range/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/protect_range/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/protect_range/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/protect_range/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/create.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/float_image/query.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/float_image/create.rs
**一致性得分**: 3%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_name`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_offset`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `square`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_name' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_offset' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'square' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/float_image/patch.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/float_image/delete.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/float_image/get.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/float_image/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet/query.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/query.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `equal`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `not_equal`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `contains`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `greater_than`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `less_than`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'equal' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'equal' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'not_equal' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'contains' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'greater_than' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'less_than' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/insert_rows_or_columns.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/add_rows_or_columns.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/update_rows_or_columns.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/delete_rows_or_columns.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/sheet_row_col/move_dimension.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_validation/query.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_validation/create.rs
**一致性得分**: 12%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `number_range`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `text_length`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_input_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_error_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_strict`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'number_range' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_validation/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_validation/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_validation/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/query.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/patch.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/condition_format/create.rs
**一致性得分**: 1%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `greater_than`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `less_than`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `equal_to`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `text_contains`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `duplicate_values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `blank_values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `text_color`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `font_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_text_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_italic`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_underline`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_strikethrough`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'greater_than' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'text_color' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/condition_format/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/condition_format/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/condition_format/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/condition_format/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/reading_multiple_ranges.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/append_data.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/find_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/split_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/merge_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/batch_set_cell_style.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/set_cell_style.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/prepend_data.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/reading_single_range.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reading_single_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_cell_value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/write_data_to_multiple_ranges.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_data_to_multiple_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `total_cell_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_value_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_string_matrix`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_mixed_values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `total_cells`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_successful`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/write_images.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `with_size`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'with_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'with_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/sheets/v3/data_operation/replace_cells.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/board/v1/whiteboard.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_thumbnail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_format`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_width`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_height`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_format' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_width' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_height' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/board/v1/whiteboard_node/list.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `whiteboard_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `small_page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `medium_page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `large_page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_drawing_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_content_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_container_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `category`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_font_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_stroke_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `style_summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `area`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `center`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `overlaps_with`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `distance_to`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bounds_description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_connections`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `content_summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_length`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_locked`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_visible`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_content`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `has_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_time_formatted`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `complexity_score`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `node_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `pagination_info`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_whiteboard_nodes`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/board/mod.rs
**一致性得分**: 62%

**API方法分析**:
- `new_from_shared`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_nodes`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/wiki/v2/space_node/copy.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `copy_space_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/wiki/v2/space_node/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_space_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/wiki/v2/space_node/create.rs
**一致性得分**: 46%

**API方法分析**:
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `space_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_doc_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sheet_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_mindnote_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_bitable_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `obj_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_node_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_origin_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_shortcut_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `node_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_space_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_space_node' 缺少Builder模式实现
   💡 建议为 'create_space_node' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/wiki/v2/space_node/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `r`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `copy`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'r' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'copy' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/wiki/v2/space_setting/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'update' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/wiki/v2/mod.rs
**一致性得分**: 100%

#### unknown - src/service/cloud_docs/wiki/v2/task/get.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/wiki/v2/task/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `move_docs_to_wiki`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'move_docs_to_wiki' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'move_docs_to_wiki' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/wiki/v2/space_member/create.rs
**一致性得分**: 26%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_space_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_space_member' 缺少Builder模式实现
   💡 建议为 'create_space_member' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/wiki/v2/space_member/mod.rs
**一致性得分**: 47%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'list' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/wiki/v2/space/create.rs
**一致性得分**: 26%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_space`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_space' 缺少Builder模式实现
   💡 建议为 'create_space' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/wiki/v2/space/get.rs
**一致性得分**: 58%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `space_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_space_info`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `space_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/cloud_docs/wiki/v2/space/mod.rs
**一致性得分**: 52%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_space_info_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_space_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_space_builder' 缺少Builder模式实现
   💡 建议为 'create_space_builder' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/wiki/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1/like.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_file_likes`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1/folder.rs
**一致性得分**: 44%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_root_folder_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_files`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_folder_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_folder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_folder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_folder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_folder_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_folder_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_folder_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_folder_meta_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_files_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `order_by`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `direction`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_folder' 缺少Builder模式实现
   💡 建议为 'create_folder' 创建对应的Builder结构体
🟡 方法 'create_folder_builder' 缺少Builder模式实现
   💡 建议为 'create_folder_builder' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/drive/v1/file_version.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_email`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_avatar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `confirm`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `deleted_version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `deleted_at`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_success`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `current_version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_current_version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `version_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `created_at`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `modified_at`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `creator`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `modifier`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `version_number`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_current`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `preview_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `thumbnail_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formatted_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'created_at' 缺少Builder模式实现
   💡 建议为 'created_at' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/drive/v1/subscription.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `confirm`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `subscription_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'confirm' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'subscription_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/drive/v1/media.rs
**一致性得分**: 10%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `upload_all_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_prepare`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_part`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_finish`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `download`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_tmp_download_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1/event.rs
**一致性得分**: 16%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `subscribe_file_events`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_file_subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsubscribe_file_events`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1/files.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_upload_file_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_download_file_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `download_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `checksum`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parent_node`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `checksum`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `auto_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `calculate_adler32_checksum`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate_file_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_async_task_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_async_task_status_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_shortcut`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_shortcut_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_delete_file_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `copy_file`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `copy_file_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_upload_file_builder' 缺少Builder模式实现
   💡 建议为 'create_upload_file_builder' 创建对应的Builder结构体
🟡 方法 'create_download_file_builder' 缺少Builder模式实现
   💡 建议为 'create_download_file_builder' 创建对应的Builder结构体
🟡 方法 'create_shortcut' 缺少Builder模式实现
   💡 建议为 'create_shortcut' 创建对应的Builder结构体
🟡 方法 'create_shortcut_builder' 缺少Builder模式实现
   💡 建议为 'create_shortcut_builder' 创建对应的Builder结构体
🟡 方法 'create_delete_file_builder' 缺少Builder模式实现
   💡 建议为 'create_delete_file_builder' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/drive/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_import_task_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_export_task_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_file_version_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_file_version_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_file_subscription_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_file_subscription_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_import_task_builder' 缺少Builder模式实现
   💡 建议为 'create_import_task_builder' 创建对应的Builder结构体
🔴 方法 'create_import_task_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_export_task_builder' 缺少Builder模式实现
   💡 建议为 'create_export_task_builder' 创建对应的Builder结构体
🔴 方法 'create_export_task_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_file_version_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_file_version_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_file_subscription_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_file_subscription_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/drive/v1/export_task.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `with_options`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `export_format`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `options`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `task_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `progress`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `created_at`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `export_url`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_completed`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_failed`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'with_options' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'export_format' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'options' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'task_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'progress' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'created_at' 缺少Builder模式实现
   💡 建议为 'created_at' 创建对应的Builder结构体
🔴 方法 'created_at' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'export_url' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_completed' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_failed' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/drive/v1/import_task.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_url`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_folder_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `overwrite`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `task_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `error_message`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `created_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `completed_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_completed`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_failed`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_processing`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_pending`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_url' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_folder_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'overwrite' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'task_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'error_message' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'created_time' 缺少Builder模式实现
   💡 建议为 'created_time' 创建对应的Builder结构体
🔴 方法 'created_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'completed_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_completed' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_failed' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_processing' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_pending' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/drive/v1/permissions.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1/file.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_file_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_file_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_file_view_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `copy_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_file_shortcut`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_files`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_prepare`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_part`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_finish`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_import_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_import_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_count`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_offset`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_owner_ids`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_block_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_checksum`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_file' 缺少Builder模式实现
   💡 建议为 'create_file' 创建对应的Builder结构体
🟡 方法 'create_file_shortcut' 缺少Builder模式实现
   💡 建议为 'create_file_shortcut' 创建对应的Builder结构体
🟡 方法 'create_import_task' 缺少Builder模式实现
   💡 建议为 'create_import_task' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_count' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_offset' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_owner_ids' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_block_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_checksum' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v1.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/v2/explorer.rs
**一致性得分**: 16%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `root_folder_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_meta`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_folder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_folder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_folder_iter`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `next`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_folder' 缺少Builder模式实现
   💡 建议为 'create_folder' 创建对应的Builder结构体
🟢 方法 'list_folder_iter' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/drive/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/drive/v2.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/public_v2/patch.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch_permission_public_v2`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/public_v2/get.rs
**一致性得分**: 29%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_permission_public_v2`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/member/list.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_permission_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `group_by_member_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/member/batch_create.rs
**一致性得分**: 22%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_create_permission_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `can_edit`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `can_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_owner`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'can_edit' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/member/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_permission_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_permission_member' 缺少Builder模式实现
   💡 建议为 'create_permission_member' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/permission/mod.rs
**一致性得分**: 63%

**API方法分析**:
- `new_from_shared`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `transfer_owner`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `auth_permission`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_permission_public`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch_permission_public`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_password`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_password`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_password`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_permission_public_v2`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch_permission_public_v2`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_member' 缺少Builder模式实现
   💡 建议为 'create_member' 创建对应的Builder结构体
🟡 方法 'create_password' 缺少Builder模式实现
   💡 建议为 'create_password' 创建对应的Builder结构体

#### unknown - src/service/cloud_docs/docx/v1/document_block.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_children`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_index`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `with_page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute_with_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_index' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'with_page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/cloud_docs/docx/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/docx/v1/document_block_descendant.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_block_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_index`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_children`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_elements`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_descendant_block`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `block_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `index`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `children`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_descendant_block_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_block_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_index' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_children' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_elements' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'create_descendant_block' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'block_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'index' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'children' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_descendant_block_builder' 缺少Builder模式实现
   💡 建议为 'create_descendant_block_builder' 创建对应的Builder结构体
🔴 方法 'create_descendant_block_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/docx/v1/document.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_folder_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_raw_content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_blocks`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `convert_to_docx`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_document_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_folder_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_raw_content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_blocks' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'convert_to_docx' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_document_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cloud_docs/docx/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new_from_shared`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new_from_shared' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new_from_shared' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### elearning 服务

#### unknown - src/service/elearning/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/elearning/course_registration/mod.rs
**一致性得分**: 29%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### authentication 服务

#### v1 - src/service/authentication/v1/auth.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/authentication/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### tenant_tag 服务

#### unknown - src/service/tenant_tag/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/tenant_tag/tag_binding/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/tenant_tag/tag/mod.rs
**一致性得分**: 100%

### human_authentication 服务

#### unknown - src/service/human_authentication/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### wiki 服务

#### v2 - src/service/wiki/v2/space_node.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `space_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `node_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_space_node_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'space_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'node_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_space_node_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/wiki/v2/task.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_task_result`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `task_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_task_result_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_task_result' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'task_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_task_result_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/wiki/v2/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/wiki/v2/space_member.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `space_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_space_member_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'space_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_space_member_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/wiki/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### im 服务

#### v1 - src/service/im/v1/pin/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `as_str`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_pin_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_pin_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_pin_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'as_str' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'as_str' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_pin_builder' 缺少Builder模式实现
   💡 建议为 'create_pin_builder' 创建对应的Builder结构体
🔴 方法 'create_pin_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_pin_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_pin_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/im/v1/buzz_messages/mod.rs
**一致性得分**: 44%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `urgent_app`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `urgent_sms`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `urgent_phone`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'urgent_app' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/file/mod.rs
**一致性得分**: 100%

#### v1 - src/service/im/v1/message_card/mod.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delay_update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `send_visible`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_visible`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'patch' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message_service.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/url_preview/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_update`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'batch_update' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message/types.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message/send.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message/list.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `with_pagination`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'w' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'w' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'with_pagination' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message/builders.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'w' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `forward`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `upload_image`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `download_file`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `download_image`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pin_message`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `unpin_message`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `list_pins`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_send_message`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `image_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_data`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `msg_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `as_str`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `image_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `file_data`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `from_file_path`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_receive_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `msg_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pin_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `unpin_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_pins_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `download_image_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `download_file_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_send_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `msg_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `quote`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `thread_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `receive_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `forward_message_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `upload_image_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'forward' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'upload_image' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'download_file' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'download_image' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pin_message' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'unpin_message' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_pins' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_send_message' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'image_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_data' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'msg_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'as_str' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'as_str' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'image_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'file_data' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'from_file_path' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_receive_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'msg_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pin_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'unpin_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_pins_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'download_image_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'download_file_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_send_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'msg_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'quote' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'thread_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'receive_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'forward_message_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'upload_image_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/im/v1/image/mod.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/message_reaction/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `container_id_type`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `text_line`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `at_user`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `msg_type`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `add_text`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `receive_id_type`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `request_body`: ✅ Builder, ❌ StandardResponse, ❌ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `msg_type`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `msg_type_string`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `receive_id`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `uuid_string`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'container_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'container_id_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'text_line' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'text_line' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'at_user' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'at_user' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'msg_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'msg_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'content' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'add_text' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'add_text' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'execute' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'receive_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'receive_id_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'request_body' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'request_body' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'execute' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'content' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'msg_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'msg_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'msg_type_string' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'msg_type_string' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'receive_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'receive_id' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'uuid' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'uuid_string' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'uuid_string' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'build' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/im/v1/chats.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement_block_content`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `avatar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `chat_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `join_permission`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `share_permission`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `management_mode`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `avatar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `chat_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `join_permission`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `share_permission`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `management_mode`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `uuid`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_chat_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_chat_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement_block_content_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement_block_content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'avatar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'chat_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'join_permission' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'share_permission' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'management_mode' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'avatar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'chat_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'join_permission' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'share_permission' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'management_mode' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'uuid' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_chat_builder' 缺少Builder模式实现
   💡 建议为 'create_chat_builder' 创建对应的Builder结构体
🔴 方法 'create_chat_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_chat_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement_block_content_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/im/v1/batch_message/mod.rs
**一致性得分**: 36%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `send`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_progress`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `read_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'send' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/im/v2/mod.rs
**一致性得分**: 100%

#### unknown - src/service/im/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### meeting_room 服务

#### unknown - src/service/meeting_room/buildings/default/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/buildings/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/schedules/default/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/schedules/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/rooms/default/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/meeting_room/rooms/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### apaas 服务

#### v1 - src/service/apaas/v1/apps.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_apps_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_apps_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/apaas/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### aily 服务

#### unknown - src/service/aily/skill/mod.rs
**一致性得分**: 38%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `start_skill`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_skill`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_skills`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/aily/message/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_messages`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_message' 缺少Builder模式实现
   💡 建议为 'create_message' 创建对应的Builder结构体

#### unknown - src/service/aily/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/aily/knowledge/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `ask_data_knowledge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_file`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_data_knowledge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_data_knowledge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_data_knowledge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_data_knowledge`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_data_knowledge_categories`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_data_knowledge' 缺少Builder模式实现
   💡 建议为 'create_data_knowledge' 创建对应的Builder结构体

#### unknown - src/service/aily/run/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_run`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_run`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_runs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_run`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_run' 缺少Builder模式实现
   💡 建议为 'create_run' 创建对应的Builder结构体

#### unknown - src/service/aily/session/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_session`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_session`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_session`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_session`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_session' 缺少Builder模式实现
   💡 建议为 'create_session' 创建对应的Builder结构体

### minutes 服务

#### v1 - src/service/minutes/v1/minute/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/minutes/v1/statistics/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/minutes/v1/transcript/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/minutes/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/minutes/v1/media/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'get' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/minutes/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### search 服务

#### v1 - src/service/search/v1/user.rs
**一致性得分**: 80%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_user_iter`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_user_with_validated_pagination`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `search_user_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `with_pagination`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `next`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### v1 - src/service/search/v1/mod.rs
**一致性得分**: 5%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `config`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'config' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/search/v2/data_source/mod.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v2 - src/service/search/v2/suite_search/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `doc_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `owner_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_by`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `doc_type_chinese`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `formatted_create_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `formatted_update_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `result_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_more_results`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `next_page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `group_by_doc_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `group_by_owner`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search_suite_object`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `doc_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `doc`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sheet`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `slide`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `bitable`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `owner_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_by_create_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_by_update_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_by_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `ascending`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `descending`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search_suite_object_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'doc_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'owner_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_by' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_direction' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'doc_type_chinese' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'formatted_create_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'formatted_update_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'result_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_more_results' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'next_page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'group_by_doc_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'group_by_owner' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search_suite_object' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'doc_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'doc' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sheet' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'slide' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'bitable' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'owner_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_by_create_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_by_update_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_by_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'ascending' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'descending' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search_suite_object_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/search/v2/schema/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v2 - src/service/search/v2/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `w`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'w' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/search/v2/data_item/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/search/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### apass 服务

#### unknown - src/service/apass/environment_variable/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_environment_variables`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_environment_variable`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/audit_log/mod.rs
**一致性得分**: 35%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_audit_logs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_audit_log`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_data_change_logs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_data_change_log_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_audit_events`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/function/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `invoke_function`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/object/mod.rs
**一致性得分**: 28%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `oql_query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_query_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_delete_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_record' 缺少Builder模式实现
   💡 建议为 'create_record' 创建对应的Builder结构体

#### unknown - src/service/apass/permission/mod.rs
**一致性得分**: 50%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_remove_role_member_authorization`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_role_member_authorization`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_role_member`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_remove_record_permission_member_authorization`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_record_permission_member_authorization`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/seat/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_seat_assignment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_seat_activity`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/apass/flow/mod.rs
**一致性得分**: 41%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `execute_flow`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_user_tasks`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `agree_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reject_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `transfer_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_assignee_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cc_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `expedite_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_user_task_rollback_points`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rollback_user_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_user_task_chat_group`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_user_task_chat_group' 缺少Builder模式实现
   💡 建议为 'create_user_task_chat_group' 创建对应的Builder结构体

### analytics 服务

#### v1 - src/service/analytics/v1/monitoring.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_real_time_monitoring`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_alert_history`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_alert_rule`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_sla_monitoring`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_monitoring_dashboard`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_monitoring_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_real_time_monitoring' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_alert_history' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_alert_rule' 缺少Builder模式实现
   💡 建议为 'create_alert_rule' 创建对应的Builder结构体
🔴 方法 'create_alert_rule' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_sla_monitoring' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_monitoring_dashboard' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_monitoring_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/analytics/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/analytics/v1/app_analytics.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_app_usage_statistics`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_feature_usage_analysis`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_app_performance_analysis`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_app_integration_analysis`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_app_roi_analysis`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_app_usage_statistics' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_feature_usage_analysis' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_app_performance_analysis' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_app_integration_analysis' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_app_roi_analysis' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/analytics/v1/insights.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_intelligent_insights`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_predictive_analysis`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `detect_anomalies`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_decision_recommendations`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_intelligent_insights' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_predictive_analysis' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'detect_anomalies' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_decision_recommendations' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/analytics/v1/reports.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_custom_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_reports`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `generate_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_report_data`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `preview_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `export_report`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_export_history`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_report_template`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_report_templates`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_report_schedule`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_report_schedule`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_report_permissions`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_report_permissions`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_custom_report' 缺少Builder模式实现
   💡 建议为 'create_custom_report' 创建对应的Builder结构体
🔴 方法 'create_custom_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_reports' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'generate_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_report_data' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'preview_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'export_report' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_export_history' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_report_template' 缺少Builder模式实现
   💡 建议为 'create_report_template' 创建对应的Builder结构体
🔴 方法 'create_report_template' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_report_templates' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_report_schedule' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_report_schedule' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_report_permissions' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_report_permissions' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/analytics/v1/overview.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_enterprise_overview`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_activity_overview`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_app_usage_overview`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_key_business_metrics`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_data_quality_overview`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_enterprise_overview' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_activity_overview' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_app_usage_overview' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_key_business_metrics' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_data_quality_overview' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/analytics/v1/user_analytics.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_user_behavior_patterns`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_journey`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_segmentation`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_churn_prediction`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_value_assessment`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_user_behavior_patterns' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_journey' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_segmentation' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_churn_prediction' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_value_assessment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/analytics/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### address_book 服务

#### v1 - src/service/address_book/v1/mod.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### unknown - src/service/address_book/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### report 服务

#### unknown - src/service/report/rule_view/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `remove`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/report/rule/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/report/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/report/task/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### application 服务

#### v6 - src/service/application/v6/app_badge/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `set`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'set' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/appstore_paid_info/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `check_user_access`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_tenant_paid_plans`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_order_info`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'check_user_access' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/application_feedback/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'update' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/admin/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_installed_apps`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_user_available_apps`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contacts_range_configuration`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_contacts_range_configuration`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_app_availability`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `check_white_black_list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_app_availability`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `enable_disable_app`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_app_admins`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_app_admin_permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `verify_app_admin`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'list_installed_apps' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/scope/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `apply`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'apply' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/app_usage/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `department_overview`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `message_push_overview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `overview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'department_overview' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v6 - src/service/application/v6/application/mod.rs
**一致性得分**: 36%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `transfer_owner`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update_collaborators`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_collaborators`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_version`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_versions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `contacts_range_suggest`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `underaudit_list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_audit_status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_group`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'transfer_owner' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/application/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### directory 服务

#### v1 - src/service/directory/v1/department/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/department/patch.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `en_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `parent_department_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `leader_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `order`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'name' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/department/delete.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `department_id_type`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'department_id_type' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/department/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mget_department_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `filter_department_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mget_department_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'filter_department_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/department/mget.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `department_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/department/filter.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_department_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_field`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `department_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_more`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_department_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_field' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_direction' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_more' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/department/search.rs
**一致性得分**: 23%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/mod.rs
**一致性得分**: 100%

#### v1 - src/service/directory/v1/employee/resurrect.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `restore_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `restore_reason`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `restore_remark`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `work_location`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_success`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `restore_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `operation_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_info`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_department_info`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'restore_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'restore_reason' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'restore_remark' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'work_location' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_success' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'restore_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'operation_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_info' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_department_info' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_no`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `en_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `work_location`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_level`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `join_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_no`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `en_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `work_location`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_level`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `join_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_no' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'en_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'work_location' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_level' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'join_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'create_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_no' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'en_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'work_location' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_level' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'join_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_employee_builder' 缺少Builder模式实现
   💡 建议为 'create_employee_builder' 创建对应的Builder结构体
🔴 方法 'create_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/patch.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_no`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `en_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `work_location`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_level`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_updates`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_no`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `en_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `work_location`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_level`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_no' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'en_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'work_location' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_level' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_updates' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_no' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'en_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'work_location' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_level' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leave_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leave_reason`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leave_remark`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_success`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leave_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `operation_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leave_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leave_reason' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leave_remark' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_success' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leave_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'operation_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/regular.rs
**一致性得分**: 0%

**API方法分析**:
- `delete_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `regular_employee`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `regular_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resurrect_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resurrect_employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `mget_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mget_employee`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `filter_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `filter_employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `to_be_resigned_employee_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `to_be_resigned_employee`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `updated_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'delete_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'regular_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'regular_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resurrect_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resurrect_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mget_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mget_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'filter_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'filter_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'to_be_resigned_employee_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'to_be_resigned_employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'updated_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/employee/mget.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_active`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_deleted`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_resigned`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `en_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `gender`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `avatar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `join_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_success`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_failures`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `failed_items`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `total`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `success_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `failed_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `success_rate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `error_code`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `error_message`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_active' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_deleted' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_resigned' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'en_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'gender' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'avatar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'join_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_time' 缺少Builder模式实现
   💡 建议为 'create_time' 创建对应的Builder结构体
🔴 方法 'create_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_success' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_failures' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'failed_items' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'total' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'success_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'failed_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'success_rate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'error_code' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'error_message' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/filter.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_field`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status_description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_active`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_number`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `hire_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_more`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_count`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `has_employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `filter_by_status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `filter_by_department`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `active_employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resigned_employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pending_employees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_field`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_field' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_direction' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status_description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_active' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_number' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'hire_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_time' 缺少Builder模式实现
   💡 建议为 'create_time' 创建对应的Builder结构体
🔴 方法 'create_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_more' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_count' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'has_employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'filter_by_status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'filter_by_department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'active_employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resigned_employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pending_employees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_field' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_direction' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/directory/v1/employee/search.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/directory/v1/employee/to_be_resigned.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_reason`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_remark`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new_with_details`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_to_be_resigned`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `operation_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_reason`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_reason_description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_remark`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `formatted_resign_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_reason`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `resign_remark`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_reason' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_remark' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new_with_details' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_to_be_resigned' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'status' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'operation_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_reason' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_reason_description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_remark' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'formatted_resign_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_reason' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'resign_remark' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/directory/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### group 服务

#### v1 - src/service/group/v1/chat_tab/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/group/v1/chat/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/group/v1/chat_menu_tree/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/group/v1/chat_member/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/group/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `w`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'w' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/group/v1/chat_announcement/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/group/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### task 服务

#### v1 - src/service/task/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_tasks`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_task' 缺少Builder模式实现
   💡 建议为 'create_task' 创建对应的Builder结构体
🔴 方法 'create_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_tasks' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/task/v2/task_subtask/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/custom_field/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/section/mod.rs
**一致性得分**: 28%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `tasks`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/comment/mod.rs
**一致性得分**: 100%

#### v2 - src/service/task/v2/attachment/mod.rs
**一致性得分**: 100%

#### v2 - src/service/task/v2/tasklist_activity_subscription/mod.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/custom_field_option/mod.rs
**一致性得分**: 27%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/tasklist/mod.rs
**一致性得分**: 25%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `tasks`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/task/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_tasks`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_tasklist`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_tasklist`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_tasklist`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_tasklist`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_tasklists`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_task_members`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove_task_members`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_tasklist_members`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove_tasklist_members`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_comment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_comments`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `upload_attachment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_attachments`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_custom_field`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_custom_fields`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_task' 缺少Builder模式实现
   💡 建议为 'create_task' 创建对应的Builder结构体
🔴 方法 'create_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_tasks' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_tasklist' 缺少Builder模式实现
   💡 建议为 'create_tasklist' 创建对应的Builder结构体
🔴 方法 'create_tasklist' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_tasklist' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_tasklist' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_tasklist' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_tasklists' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_task_members' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove_task_members' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_tasklist_members' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove_tasklist_members' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_comment' 缺少Builder模式实现
   💡 建议为 'create_comment' 创建对应的Builder结构体
🔴 方法 'create_comment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_comments' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'upload_attachment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_attachments' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_custom_field' 缺少Builder模式实现
   💡 建议为 'create_custom_field' 创建对应的Builder结构体
🔴 方法 'create_custom_field' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_custom_fields' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/task/v2/task/mod.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_reminders`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_tasklist`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_reminders`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_dependencies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_dependencies`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/task/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### docx 服务

#### v1 - src/service/docx/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/docx/v1/document.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement_block_content`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `get_announcement_block_content_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `document_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_document_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_document_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'folder_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement_block_content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_announcement_block_content_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'folder_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'document_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_document_builder' 缺少Builder模式实现
   💡 建议为 'create_document_builder' 创建对应的Builder结构体
🔴 方法 'create_document_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_document_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/docx/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### performance 服务

#### v1 - src/service/performance/v1/results.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `open_results`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `confirm_result`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'open_results' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'confirm_result' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/performance/v1/activities.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `start`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pause`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `finish`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `cancel`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_participants`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_participants`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove_participants`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_progress`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'start' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pause' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'finish' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'cancel' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_participants' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_participants' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove_participants' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_progress' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/performance/v1/reviews.rs
**一致性得分**: 1%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `submit`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'submit' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/performance/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/performance/v1/templates.rs
**一致性得分**: 1%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/performance/v1/cycles.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `start`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `pause`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `finish`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_statistics`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_activities`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'start' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'pause' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'finish' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_statistics' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_activities' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/performance/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/performance/stage_task/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `find_tasks_by_user_list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_tasks_by_page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/performance/metric_detail/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_metric_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `import_metric_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/performance/review_config/mod.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_semesters`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_activities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_additional_information`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `import_additional_information`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_additional_information`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_user_group_members`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_reviewees`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_review_templates`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_review_items`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_tag_question_configs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_metrics`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_metric_templates`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_metric_fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_metric_tags`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/performance/review_data/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_results`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_details`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### lingo 服务

#### unknown - src/service/lingo/classification/mod.rs
**一致性得分**: 21%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/lingo/draft/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update_draft`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/lingo/file/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `upload_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `download_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/lingo/entity/mod.rs
**一致性得分**: 25%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_entity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_entity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_entity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_entity`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_entities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_entities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_entities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `highlight_entities`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_entity' 缺少Builder模式实现
   💡 建议为 'create_entity' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/lingo/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/lingo/repo/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_repos`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### ai_embedding 服务

#### v1 - src/service/ai_embedding/v1/text_embedding.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `text_embedding`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `batch_text_embedding`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `multimodal_embedding`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'text_embedding' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'text_embedding' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'batch_text_embedding' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'multimodal_embedding' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/ai_embedding/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### attendance 服务

#### v1 - src/service/attendance/v1/user_task.rs
**一致性得分**: 56%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_del`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_result`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/leave_accrual_record.rs
**一致性得分**: 23%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/leave_employ_expire_record.rs
**一致性得分**: 21%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/mod_old.rs
**一致性得分**: 100%

#### v1 - src/service/attendance/v1/user_stats_data.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_data`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/user_task_remedy.rs
**一致性得分**: 48%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/user_approval.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `process`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/attendance/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_user_task`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `query_user_tasks`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_shift`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_shifts`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_stats`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_user_task' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'query_user_tasks' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_shift' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_shifts' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_stats' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/attendance/v1/archive_rule.rs
**一致性得分**: 23%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/user_setting.rs
**一致性得分**: 60%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_photo`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `download_photo`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/user_daily_shift.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/attendance/v1/group.rs
**一致性得分**: 59%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/attendance/v1/shift.rs
**一致性得分**: 22%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_size' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'page_token' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/attendance/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### verification 服务

#### v1 - src/service/verification/v1/mod.rs
**一致性得分**: 100%

#### unknown - src/service/verification/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### base 服务

#### unknown - src/service/base/bitable/mod.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_app`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `copy_app`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_app`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_app`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_table`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_tables`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_table`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_app' 缺少Builder模式实现
   💡 建议为 'create_app' 创建对应的Builder结构体
🟡 方法 'create_table' 缺少Builder模式实现
   💡 建议为 'create_table' 创建对应的Builder结构体
🟡 方法 'create_record' 缺少Builder模式实现
   💡 建议为 'create_record' 创建对应的Builder结构体

#### unknown - src/service/base/models/mod.rs
**一致性得分**: 100%

#### unknown - src/service/base/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### acs 服务

#### unknown - src/service/acs/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### contact 服务

#### unknown - src/service/contact/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v3 - src/service/contact/v3/group_member.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_add`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `simplelist`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_remove`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_group_member_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_group_members_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_add' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'simplelist' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_remove' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_group_member_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_group_members_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/job_family.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_job_family_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_job_families_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_job_family_builder' 缺少Builder模式实现
   💡 建议为 'create_job_family_builder' 创建对应的Builder结构体
🔴 方法 'create_job_family_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_job_families_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/job_level.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_family_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `job_family_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_job_level_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_job_levels_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_family_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'job_family_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_job_level_builder' 缺少Builder模式实现
   💡 建议为 'create_job_level_builder' 创建对应的Builder结构体
🔴 方法 'create_job_level_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_job_levels_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/functional_role.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `role_name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_functional_role_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_functional_roles_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'role_name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_functional_role_builder' 缺少Builder模式实现
   💡 建议为 'create_functional_role_builder' 创建对应的Builder结构体
🔴 方法 'create_functional_role_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_functional_roles_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/functional_role_member.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_scopes`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_role_member_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_role_members_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_scopes' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_role_member_builder' 缺少Builder模式实现
   💡 建议为 'create_role_member_builder' 创建对应的Builder结构体
🔴 方法 'create_role_member_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_role_members_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/job_title.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_job_titles_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_job_titles_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/user.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_by_department`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `restore`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `email`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `mobile`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `employee_no`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `gender`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_user_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search_user_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_by_department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'restore' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'email' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'mobile' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'position' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'employee_no' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'gender' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'query' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_user_builder' 缺少Builder模式实现
   💡 建议为 'create_user_builder' 创建对应的Builder结构体
🔴 方法 'create_user_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search_user_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/custom_attr.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_custom_attrs_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_custom_attrs_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/scope.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_authority`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_authority`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `scope_details`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_scope_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_scope_authority_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_scope_authority_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_authority' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_authority' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'scope_details' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_scope_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_scope_authority_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_scope_authority_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/work_city.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_work_cities_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_work_cities_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/department.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_sub_department_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_id_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_department_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `leader_user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_department_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `search_department_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_sub_department_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_id_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_department_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'leader_user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'query' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_department_builder' 缺少Builder模式实现
   💡 建议为 'create_department_builder' 创建对应的Builder结构体
🔴 方法 'create_department_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'search_department_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/unit.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `bind_department`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `unbind_department`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_departments`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `unit_code`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_unit_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_unit_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_unit_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_units_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'bind_department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'unbind_department' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_departments' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'unit_code' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_unit_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_unit_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_unit_builder' 缺少Builder模式实现
   💡 建议为 'create_unit_builder' 创建对应的Builder结构体
🔴 方法 'create_unit_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_units_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/employee_type_enum.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_employee_type_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_employee_types_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_employee_type_builder' 缺少Builder模式实现
   💡 建议为 'create_employee_type_builder' 创建对应的Builder结构体
🔴 方法 'create_employee_type_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_employee_types_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/contact/v3/group.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `simple_list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_groups`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `get_detail`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `group_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `group_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `member_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `department_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `include_members`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_group_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_group_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_groups_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_groups_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_group_detail_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'simple_list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_groups' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_detail' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'group_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'name' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'group_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'member_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'department_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'include_members' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_group_builder' 缺少Builder模式实现
   💡 建议为 'create_group_builder' 创建对应的Builder结构体
🔴 方法 'create_group_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_group_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_groups_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_groups_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_group_detail_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### helpdesk 服务

#### v1 - src/service/helpdesk/v1/category/mod.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/notification/mod.rs
**一致性得分**: 36%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `preview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `submit_approve`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_approve`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute_send`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_send`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/faq/mod.rs
**一致性得分**: 34%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `faq_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/helpdesk/v1/agent_skill/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/agent/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `agent_email`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/mod.rs
**一致性得分**: 100%

#### v1 - src/service/helpdesk/v1/ticket/mod.rs
**一致性得分**: 54%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `assignee_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `creator_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_mask`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_update_fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_tickets`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_ticket`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_ticket`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `list_tickets_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_ticket_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_ticket_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `assignee_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `creator_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_direction`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ticket_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `priority`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `assignee`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_mask`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v1 - src/service/helpdesk/v1/agent_schedule/mod.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/agent_skill_rule/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `operator_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/ticket_customized_field/mod.rs
**一致性得分**: 35%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/helpdesk/v1/event/mod.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `subscribe`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsubscribe`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/helpdesk/v1/ticket_message/mod.rs
**一致性得分**: 28%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_group_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_group_message' 缺少Builder模式实现
   💡 建议为 'create_group_message' 创建对应的Builder结构体

#### unknown - src/service/helpdesk/models.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/helpdesk/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### event 服务

#### v1 - src/service/event/v1/mod.rs
**一致性得分**: 21%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_outbound_ip`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_outbound_ip_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/event/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### vc 服务

#### v1 - src/service/vc/v1/room/mod.rs
**一致性得分**: 28%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/vc/v1/recording/mod.rs
**一致性得分**: 36%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `start`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `stop`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_permission`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'start' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/vc/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/vc/v1/meeting/mod.rs
**一致性得分**: 100%

#### v1 - src/service/vc/v1/reserve/mod.rs
**一致性得分**: 100%

#### unknown - src/service/vc/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### corehr 服务

#### unknown - src/service/corehr/organization/mod.rs
**一致性得分**: 25%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_department`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_departments`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_department_tree`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_company`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_companies`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_department' 缺少Builder模式实现
   💡 建议为 'create_department' 创建对应的Builder结构体
🟡 方法 'create_company' 缺少Builder模式实现
   💡 建议为 'create_company' 创建对应的Builder结构体

#### unknown - src/service/corehr/lifecycle/mod.rs
**一致性得分**: 19%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_pre_hire`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_pre_hire`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_job_change`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_job_change`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_offboarding`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_offboarding`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_pre_hire' 缺少Builder模式实现
   💡 建议为 'create_pre_hire' 创建对应的Builder结构体
🟡 方法 'create_job_change' 缺少Builder模式实现
   💡 建议为 'create_job_change' 创建对应的Builder结构体
🟡 方法 'create_offboarding' 缺少Builder模式实现
   💡 建议为 'create_offboarding' 创建对应的Builder结构体

#### unknown - src/service/corehr/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/corehr/basic_info/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `search_enum`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_country_region`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_nationality`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `convert_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/corehr/employee/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/corehr/job_management/mod.rs
**一致性得分**: 10%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_job_family`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_job_families`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_job_level`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_job_levels`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_job_grade`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_job_grades`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_job`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_jobs`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_job_family' 缺少Builder模式实现
   💡 建议为 'create_job_family' 创建对应的Builder结构体
🟡 方法 'create_job_level' 缺少Builder模式实现
   💡 建议为 'create_job_level' 创建对应的Builder结构体
🟡 方法 'create_job_grade' 缺少Builder模式实现
   💡 建议为 'create_job_grade' 创建对应的Builder结构体
🟡 方法 'create_job' 缺少Builder模式实现
   💡 建议为 'create_job' 创建对应的Builder结构体

### personal_settings 服务

#### v1 - src/service/personal_settings/v1/system_status/mod.rs
**一致性得分**: 42%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_open`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_close`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/personal_settings/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/personal_settings/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### calendar 服务

#### unknown - src/service/calendar/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/meeting_chat/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/meeting_chat/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/meeting_chat/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/calendar_event/list.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_time_min`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_time_max`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_query`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_sort_by`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_sort_order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build_query_string`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `time_min`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `time_max`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_by`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `sort_order`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_time_min' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_time_max' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_query' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_sort_by' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_sort_order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build_query_string' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'page_size' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'time_min' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'time_max' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'query' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_by' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'sort_order' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_summary`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_start_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_end_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_is_all_day`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `start_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `end_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_all_day`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_summary' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_start_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_end_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_is_all_day' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'summary' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'start_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'end_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_all_day' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/patch.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_summary`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_start_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_end_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_is_all_day`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_color`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `start_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `end_time`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `is_all_day`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `color`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_summary' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_start_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_end_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_is_all_day' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_color' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'summary' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'start_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'end_time' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'is_all_day' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'color' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/get.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_calendar_event_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_calendar_events_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_calendar_event_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_calendar_event_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_calendar_event_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `reply`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `reply_calendar_event_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_calendar_event_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_calendar_events_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_calendar_event_builder' 缺少Builder模式实现
   💡 建议为 'create_calendar_event_builder' 创建对应的Builder结构体
🔴 方法 'create_calendar_event_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_calendar_event_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_calendar_event_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'reply' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'reply_calendar_event_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar_event/reply.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_comment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_send_notifications`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `set_user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `comment`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `send_notifications`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_comment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_send_notifications' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'set_user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'comment' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'send_notifications' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/calendar/list.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/calendar/create.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/calendar/get.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/calendar/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/attendee/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/meeting_minute/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_calendar_event`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_calendar_event`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_calendar_event`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_calendar_event`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_calendar_events`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_primary_calendar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_calendars`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_user_free_busy`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_users_free_busy`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_event_attendees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `add_event_attendees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `remove_event_attendees`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_meeting_rooms`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `book_meeting_room`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `subscribe_calendar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `unsubscribe_calendar`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_calendar_subscriptions`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_calendar_event' 缺少Builder模式实现
   💡 建议为 'create_calendar_event' 创建对应的Builder结构体
🔴 方法 'create_calendar_event' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_calendar_event' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_calendar_event' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_calendar_event' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_calendar_events' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_primary_calendar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_calendars' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_user_free_busy' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_users_free_busy' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_event_attendees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'add_event_attendees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'remove_event_attendees' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_meeting_rooms' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'book_meeting_room' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'subscribe_calendar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'unsubscribe_calendar' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_calendar_subscriptions' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v4 - src/service/calendar/v4/timeoff_event/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/calendar_acl/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/meeting_room_event/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/setting/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v4 - src/service/calendar/v4/exchange_binding/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### sheets 服务

#### v2 - src/service/sheets/v2/single_write.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `service_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `service_version`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_range_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_single_value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_single_row`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/batch_read.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `read_multiple_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `read_single_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `read_ranges_from_vec`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges_from_string`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build_and_validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/values_single_write.rs
**一致性得分**: 56%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_parse_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_request_body`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `write_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_csv`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_hashmap`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_parse_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/sheet_cells.rs
**一致性得分**: 50%

**API方法分析**:
- `text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `boolean`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formula`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_blank`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_formula`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_string`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_string`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_string`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_valid`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_cell`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build_and_validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/merge_cells.rs
**一致性得分**: 57%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_cells`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `unmerge_cells`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_rows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_columns`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/sheet_management.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_request`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_requests`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_sheets`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_sheet_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_request`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_requests`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/sheets_batch_update.rs
**一致性得分**: 53%

**API方法分析**:
- `add_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `duplicate_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `grid_properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_request`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_requests`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_request_body`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_colored_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_hidden_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_sheet`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `duplicate_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_request`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `grid_properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/values_append.rs
**一致性得分**: 52%

**API方法分析**:
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_row`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_rows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_hashmap`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_csv`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_array`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `append`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `append_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `append_from_hashmap_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `append_from_csv_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_row`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_rows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_hashmap`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_csv`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_array`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v2 - src/service/sheets/v2/values_batch_write.rs
**一致性得分**: 55%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_parse_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_request_body`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_write`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_write_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_csv_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_hashmap_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_input_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_parse_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `include_values_in_response`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `response_date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/batch_write.rs
**一致性得分**: 52%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `row_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_ranges_from_vec`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `total_cell_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_multiple_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_single_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_ranges_from_vec`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges_from_tuples`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_time_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build_and_validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `total_cell_count`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `clear_ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/dimension_operations.rs
**一致性得分**: 58%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `inherit_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `inherit_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `frozen`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `pixel_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `insert_dimension_range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_dimension_range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `add_dimension_range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_dimension_range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `add_rows_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_columns_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_rows_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_columns_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `insert_rows_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `insert_columns_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_rows_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_columns_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `inherit_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `inherit_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `hidden`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `frozen`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `pixel_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `column_width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/values_prepend.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_formula`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `overwrite`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `clear_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_request_body`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_csv`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `prepend`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `prepend_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_array`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_hashmap`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_csv`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `as_formula`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `overwrite`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `clear_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/style_operations.rs
**一致性得分**: 41%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `font_family`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `font_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `italic`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `strikethrough`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `underline`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `foreground_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `top`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bottom`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `left`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `right`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rgb`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rgba`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hex`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `black`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `white`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `red`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `green`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `blue`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `borders`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `horizontal_alignment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `vertical_alignment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `wrap_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `style_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_styles`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_styles`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_update_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_styles`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'white' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'red' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'green' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'blue' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/sheets/v2/image_write.rs
**一致性得分**: 53%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `z_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `calculate_absolute_position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_height`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `area`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `alt_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `absolute_position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `summary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `simple`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_offset`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `image_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `at_cell`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_dimensions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `alt_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `z_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build_unchecked`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `write_images_batch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_image_info`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/batch_read_ranges.rs
**一致性得分**: 56%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formula_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build_query_params`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_read`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_read_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `read_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `value_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `date_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formula_render_option`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v2 - src/service/sheets/v2/data_validation.rs
**一致性得分**: 52%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value1`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value2`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `dropdown_source`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `prompt_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `error_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allow_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `strict_mode`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_data_validation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_data_validation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `dropdown_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number_range_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `prompt_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `error_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allow_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `strict_mode`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `operator`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value1`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value2`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `prompt_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `error_message`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### unknown - src/service/sheets/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/sheets/v3/sheet_protection.rs
**一致性得分**: 42%

**API方法分析**:
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `start_row_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `end_row_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `start_column_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `end_column_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `condition_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parameters`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_parameter`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `protection_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `warning_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_editor`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_protection_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `protection_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `protection_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ranges`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `warning_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `protection_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_protection_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_editor`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `protection_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `warning_only`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_protection_builder' 缺少Builder模式实现
   💡 建议为 'create_protection_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/filter_views.rs
**一致性得分**: 49%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `condition_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `ignore_case`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `filter_view_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_in_toolbar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_filter_view`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_filter_view`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_filter_view`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_filter_view`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_filter_view_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_filter_view_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_in_toolbar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_in_toolbar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `clear_conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_filter_view_builder' 缺少Builder模式实现
   💡 建议为 'create_filter_view_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/data_filter.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_equals`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_contains`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number_between`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `is_not_empty`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `with_sort_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `filter_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `conditions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_data_filter`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_data_filter`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_data_filter_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_data_filter_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v3 - src/service/sheets/v3/spreadsheet_create.rs
**一致性得分**: 42%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheets`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `time_zone`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `locale`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_path`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_request_body`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_spreadsheet_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_colored_sheet`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `time_zone`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `locale`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_path`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_property`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_spreadsheet_builder' 缺少Builder模式实现
   💡 建议为 'create_spreadsheet_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/charts.rs
**一致性得分**: 40%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `font_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `bold`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `min`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `max`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_gridlines`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `legend`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `x_axis`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `y_axis`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `border_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sub_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `chart_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_series`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `series`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sub_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `style`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `chart_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_chart`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_chart`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_chart_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_chart_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_chart' 缺少Builder模式实现
   💡 建议为 'create_chart' 创建对应的Builder结构体
🟡 方法 'create_chart_builder' 缺少Builder模式实现
   💡 建议为 'create_chart_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/conditional_format.rs
**一致性得分**: 50%

**API方法分析**:
- `rgb`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rgba`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `hex`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `red`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `green`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `blue`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `yellow`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `orange`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `purple`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `gray`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `light_gray`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `white`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `black`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_value`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `direction`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `two_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_point`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_arrows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_traffic_lights`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `background_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `text_color`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `data_bar`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `two_color_scale`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_color_scale`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_arrows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `three_traffic_lights`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `single_condition`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `formula`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rule`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `conditional_format_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_conditional_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_conditional_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `set_conditional_format_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_conditional_format_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v3 - src/service/sheets/v3/spreadsheet.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_spreadsheet_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `folder_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_spreadsheet_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_spreadsheet_builder' 缺少Builder模式实现
   💡 建议为 'create_spreadsheet_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/sheets/v3/comments.rs
**一致性得分**: 41%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_mention`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_user_mention`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `avatar_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell_reference`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_reply`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell_reference`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cell_reference`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `comment_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_comments`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_comment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_comment_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_comments_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_comment_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_comment' 缺少Builder模式实现
   💡 建议为 'create_comment' 创建对应的Builder结构体
🟡 方法 'create_comment_builder' 缺少Builder模式实现
   💡 建议为 'create_comment_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/move_dimension.rs
**一致性得分**: 52%

**API方法分析**:
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `dimension`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `rows`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `columns`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `source_start_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `source_end_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `source_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `destination_index`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `move_dimension`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `move_rows_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `move_columns_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `from_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `to_position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v3 - src/service/sheets/v3/pivot_tables.rs
**一致性得分**: 41%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_subtotals`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sort_order`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `custom_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number_format`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `custom_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `values`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `multiple_selections`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_row_grand_totals`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_column_grand_totals`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_row_headers`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `show_column_headers`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `merge_labels`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_row_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_column_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_value_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_filter_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `layout`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `source_range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_row_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_column_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_value_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_filter_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `layout`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `pivot_table_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_pivot_table`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_pivot_table`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_pivot_table_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_pivot_table_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_pivot_table' 缺少Builder模式实现
   💡 建议为 'create_pivot_table' 创建对应的Builder结构体
🟡 方法 'create_pivot_table_builder' 缺少Builder模式实现
   💡 建议为 'create_pivot_table_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/macros.rs
**一致性得分**: 42%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allow_file_access`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allow_network_access`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allow_system_calls`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `allowed_sheets`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `forbidden_operations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `string`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `number`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `boolean`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `array`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `author`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `permissions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `async_execution`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `macro_name`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_parameter`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `parameters`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `async_execution`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `macro_script`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execution_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute_macro`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_macro`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_macro_status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute_macro_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_macro_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_macro_status_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create_macro' 缺少Builder模式实现
   💡 建议为 'create_macro' 创建对应的Builder结构体
🟡 方法 'create_macro_builder' 缺少Builder模式实现
   💡 建议为 'create_macro_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/float_images.rs
**一致性得分**: 41%

**API方法分析**:
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `height`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_x`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_y`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `height`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_x`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_y`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `width`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `height`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_x`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset_y`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `offset`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_field`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_all`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_float_image_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `float_image_ids`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟡 方法 'create_builder' 缺少Builder模式实现
   💡 建议为 'create_builder' 创建对应的Builder结构体

#### v3 - src/service/sheets/v3/sheet.rs
**一致性得分**: 59%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_sheets`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_sheet`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_sheets_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_sheet_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_cells`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `find_cells_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `case_sensitive`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_whole_word`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `case_sensitive`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_whole_word`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ✅ StandardResponse, ✅ 文档

#### v3 - src/service/sheets/v3/find_replace.rs
**一致性得分**: 51%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `case_sensitive`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_whole_word`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `look_in_formulas`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `case_sensitive`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_whole_word`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `look_in_formulas`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `confirm_replacements`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replace_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `spreadsheet_token`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `sheet_id`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replace_text`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `match_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range_type`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `range`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replace_options`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replace`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `find_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `replace_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

### auth 服务

#### v1 - src/service/auth/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `user_info`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `oidc_access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `oidc_refresh_access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_auth_code`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `refresh_access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'user_info' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'oidc_access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'oidc_refresh_access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_auth_code' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'refresh_access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/auth/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v3 - src/service/auth/v3/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `tenant_access_token_internal`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `app_access_token_internal`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `app_ticket_resend`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `app_access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `tenant_access_token`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'tenant_access_token_internal' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'app_access_token_internal' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'app_ticket_resend' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'app_access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'tenant_access_token' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

### bot 服务

#### unknown - src/service/bot/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v3 - src/service/bot/v3/info/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v3 - src/service/bot/v3/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### app_engine 服务

#### unknown - src/service/app_engine/audit_log/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/audit_log/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/permissions/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/permissions/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/apps/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/apps/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/seat_management/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/app_engine/seat_management/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### cardkit 服务

#### v1 - src/service/cardkit/v1/card/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `card_json`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_card`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `card_json`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_card_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'card_json' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'create_card' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'card_json' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_card_builder' 缺少Builder模式实现
   💡 建议为 'create_card_builder' 创建对应的Builder结构体
🔴 方法 'create_card_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `card_json`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_card`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `title`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `description`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `card_json`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_card_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'card_json' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_card' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'title' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'description' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'card_json' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_card_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card/batch_update.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/cardkit/v1/card/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card/settings.rs
**一致性得分**: 17%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/cardkit/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card_element/create.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_card_element`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_card_element_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'create_card_element' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_card_element_builder' 缺少Builder模式实现
   💡 建议为 'create_card_element_builder' 创建对应的Builder结构体
🔴 方法 'create_card_element_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card_element/patch.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `element_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `build_patch_body`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_card_element`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `element_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `patch_card_element_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'element_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'build_patch_body' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_card_element' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'element_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'patch_card_element_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card_element/delete.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_card_element`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_card_element_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_card_element' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_card_element_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card_element/update.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `element_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `validate`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_card_element`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `element_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `content`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `parent_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_card_element_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'element_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'validate' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_card_element' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'element_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'content' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'parent_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_card_element_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/cardkit/v1/card_element/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/cardkit/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### mail 服务

#### v1 - src/service/mail/v1/user_mailbox_alias/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/public_mailbox_alias/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/contact/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/attachment/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `download_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/folder/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体

#### v1 - src/service/mail/v1/message/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `send`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_by_card`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/mail_group_alias/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/rule/mod.rs
**一致性得分**: 35%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reorder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/mail_group/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `patch`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🟢 方法 'create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/mod.rs
**一致性得分**: 100%

#### v1 - src/service/mail/v1/mail_group_member/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/public_mailbox/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/address/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/public_mailbox_member/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/mail_group_manager/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_create`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `batch_delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'batch_create' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/mail_group_permission_member/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v1 - src/service/mail/v1/event/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `subscribe`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `subscription`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unsubscribe`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'subscribe' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/mail/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### hire 服务

#### unknown - src/service/hire/attachment/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_upload_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_attachment_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_attachments`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_attachment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_attachment`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_download_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_preview_url`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_download`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_delete`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_attachment_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_upload_task' 缺少Builder模式实现
   💡 建议为 'create_upload_task' 创建对应的Builder结构体

#### unknown - src/service/hire/ecological_docking/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/ecological_docking/background_check/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_packages`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_order`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_order_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_orders`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_order`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_report`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_orders`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_order' 缺少Builder模式实现
   💡 建议为 'create_order' 创建对应的Builder结构体

#### unknown - src/service/hire/ecological_docking/exam/mod.rs
**一致性得分**: 45%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_papers`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `arrange_exam`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_record_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `submit_exam`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_exam`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reschedule_exam`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_exam_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/candidate_management/offer/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_offer_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_offers`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `send_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `withdraw_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_onboarding`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_onboardings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_onboarding_progress`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_offer' 缺少Builder模式实现
   💡 建议为 'create_offer' 创建对应的Builder结构体
🟡 方法 'create_onboarding' 缺少Builder模式实现
   💡 建议为 'create_onboarding' 创建对应的Builder结构体

#### unknown - src/service/hire/candidate_management/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/candidate_management/interview/mod.rs
**一致性得分**: 40%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_interview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_interview_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_interviews`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `arrange_interview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `submit_interview_evaluation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_interview_evaluations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `cancel_interview`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reschedule_interview`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_interview' 缺少Builder模式实现
   💡 建议为 'create_interview' 创建对应的Builder结构体

#### unknown - src/service/hire/candidate_management/application/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_application`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_application_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_applications`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `advance_application`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reject_application`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_application_interviews`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_application_offer`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_application_evaluation`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_application' 缺少Builder模式实现
   💡 建议为 'create_application' 创建对应的Builder结构体
🟡 方法 'create_offer' 缺少Builder模式实现
   💡 建议为 'create_offer' 创建对应的Builder结构体

#### unknown - src/service/hire/candidate_management/talent_pool/mod.rs
**一致性得分**: 40%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_pool`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_pool_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_pools`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_pool_talents`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_talent_to_pool`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `remove_talent_from_pool`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_pool`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_pool`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_pool' 缺少Builder模式实现
   💡 建议为 'create_pool' 创建对应的Builder结构体

#### unknown - src/service/hire/candidate_management/talent/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_talent`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_talent_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_talents`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_talent`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_talent`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_talent_application_history`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_import_talents`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_talent_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_talent_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_import_talents_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `w`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_talents_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_create_talents_with_builder`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_talent' 缺少Builder模式实现
   💡 建议为 'create_talent' 创建对应的Builder结构体
🟡 方法 'create_talent_with_builder' 缺少Builder模式实现
   💡 建议为 'create_talent_with_builder' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/offer_settings/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_settings_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_settings' 缺少Builder模式实现
   💡 建议为 'create_settings' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/job_process/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_process`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_process_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_processes`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_process`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_process`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_process' 缺少Builder模式实现
   💡 建议为 'create_process' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/job_requirement/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_requirement`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_requirement_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_requirements`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_requirement`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_requirement`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_requirement' 缺少Builder模式实现
   💡 建议为 'create_requirement' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/auth/mod.rs
**一致性得分**: 38%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_role_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_roles`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_user_roles`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/recruitment_config/location/mod.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_locations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_locations`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/recruitment_config/subject/mod.rs
**一致性得分**: 43%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_subject`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_subject_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_subjects`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_subject`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_subject`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `enable_subject`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `disable_subject`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_subject' 缺少Builder模式实现
   💡 建议为 'create_subject' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/interview_settings/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_settings_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_settings' 缺少Builder模式实现
   💡 建议为 'create_settings' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/recruitment_config/job/mod.rs
**一致性得分**: 42%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_job`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_job`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_job_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_jobs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `close_job`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `open_job`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_job' 缺少Builder模式实现
   💡 建议为 'create_job' 创建对应的Builder结构体

#### unknown - src/service/hire/recruitment_config/application.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_talent_tags`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_registration_forms`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/get_candidates/external_system/mod.rs
**一致性得分**: 35%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_system_config`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_system_configs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_sync_task`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_sync_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `import_external_candidates`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_external_candidates`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `convert_external_candidate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `test_system_connection`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_system_config' 缺少Builder模式实现
   💡 建议为 'create_system_config' 创建对应的Builder结构体
🟡 方法 'create_sync_task' 缺少Builder模式实现
   💡 建议为 'create_sync_task' 创建对应的Builder结构体

#### unknown - src/service/hire/get_candidates/referral/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_referral`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_referral_detail`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_referrals`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `register_referral_account`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_referral_account`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `grant_referral_reward`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_reward_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_reward_settings`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_referral' 缺少Builder模式实现
   💡 建议为 'create_referral' 创建对应的Builder结构体
🟡 方法 'create_reward_settings' 缺少Builder模式实现
   💡 建议为 'create_reward_settings' 创建对应的Builder结构体

#### unknown - src/service/hire/get_candidates/agency/mod.rs
**一致性得分**: 35%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_agency`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_agencies`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_recommendation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_recommendations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `add_consultant`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_consultants`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `confirm_recommendation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `reject_recommendation`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_agency' 缺少Builder模式实现
   💡 建议为 'create_agency' 创建对应的Builder结构体
🟡 方法 'create_recommendation' 缺少Builder模式实现
   💡 建议为 'create_recommendation' 创建对应的Builder结构体

#### unknown - src/service/hire/get_candidates/website/mod.rs
**一致性得分**: 45%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_website_jobs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `publish_job_to_website`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `unpublish_job_from_website`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_website_applications`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_website_configuration`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_website_configuration`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `convert_website_application`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_website_job_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/get_candidates/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/hire/referral_account/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_account`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_accounts`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_balance`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_income_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `apply_withdrawal`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_withdrawal_records`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `approve_withdrawal`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `enable_account`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `disable_account`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_referral_statistics`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_account' 缺少Builder模式实现
   💡 建议为 'create_account' 创建对应的Builder结构体

### tenant 服务

#### v2 - src/service/tenant/v2/tenant/mod.rs
**一致性得分**: 100%

#### v2 - src/service/tenant/v2/tenant_product_assign_info/mod.rs
**一致性得分**: 31%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new_from_shared`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new_from_shared' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### v2 - src/service/tenant/v2/mod.rs
**一致性得分**: 100%

#### unknown - src/service/tenant/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### baike 服务

#### unknown - src/service/baike/lingo/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/baike/lingo/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/baike/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/baike/dictionary/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/baike/dictionary/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### passport 服务

#### v1 - src/service/passport/v1/mod.rs
**一致性得分**: 15%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### v1 - src/service/passport/v1/sessions/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `logout`: ✅ Builder, ❌ StandardResponse, ✅ 文档
- `logout_builder`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_ids`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `user_id_type`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `execute`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'logout' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'logout_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_ids' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'user_id_type' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'execute' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/passport/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/passport/sessions/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### ccm 服务

#### unknown - src/service/ccm/sheets/v2/range.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/sheets/v2/style.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/sheets/v2/spreadsheet.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_metadata`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_all_sheets`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_properties`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_metadata' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_all_sheets' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_properties' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create' 缺少Builder模式实现
   💡 建议为 'create' 创建对应的Builder结构体
🔴 方法 'create' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/ccm/sheets/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/sheets/v2/worksheet.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/sheets/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/sheets/v3/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/export_tasks/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/wiki/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/wiki/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/wiki/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/doc/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/doc/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v1/view_record.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v1/meta.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v1/statistics.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v1/file.rs
**一致性得分**: 100%

#### unknown - src/service/ccm/drive/explorer/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/v2/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/drive/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/permission/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/v1/comment.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/v1/block.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/v1/document.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ccm/docx/documents/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### admin 服务

#### unknown - src/service/admin/password/mod.rs
**一致性得分**: 100%

#### unknown - src/service/admin/badge_grant/mod.rs
**一致性得分**: 100%

#### unknown - src/service/admin/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/admin/data_report/mod.rs
**一致性得分**: 100%

#### unknown - src/service/admin/badge/mod.rs
**一致性得分**: 100%

### okr 服务

#### v1 - src/service/okr/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_period`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_period`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_period`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_periods`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_okr`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_okr`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `update_okr`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `delete_okr`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_user_okrs`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `batch_get_okrs`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_progress_record`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `list_progress_records`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `create_review`: ❌ Builder, ❌ StandardResponse, ✅ 文档
- `get_review`: ❌ Builder, ❌ StandardResponse, ✅ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_period' 缺少Builder模式实现
   💡 建议为 'create_period' 创建对应的Builder结构体
🔴 方法 'create_period' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_period' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_period' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_periods' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_okr' 缺少Builder模式实现
   💡 建议为 'create_okr' 创建对应的Builder结构体
🔴 方法 'create_okr' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_okr' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'update_okr' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'delete_okr' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_user_okrs' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'batch_get_okrs' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_progress_record' 缺少Builder模式实现
   💡 建议为 'create_progress_record' 创建对应的Builder结构体
🔴 方法 'create_progress_record' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'list_progress_records' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟡 方法 'create_review' 缺少Builder模式实现
   💡 建议为 'create_review' 创建对应的Builder结构体
🔴 方法 'create_review' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🔴 方法 'get_review' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理

#### unknown - src/service/okr/progress_record/mod.rs
**一致性得分**: 30%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_progress_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_progress_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_progress_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_progress_record`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `upload_progress_image`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_progress_record' 缺少Builder模式实现
   💡 建议为 'create_progress_record' 创建对应的Builder结构体

#### unknown - src/service/okr/period/mod.rs
**一致性得分**: 33%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_period`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_period_status`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_periods`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_period' 缺少Builder模式实现
   💡 建议为 'create_period' 创建对应的Builder结构体

#### unknown - src/service/okr/period_rule/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_period_rules`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/okr/okr/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_user_okrs`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_okrs`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/okr/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/okr/review/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `query_reviews`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### ehr 服务

#### v1 - src/service/ehr/v1/attendance.rs
**一致性得分**: 78%

**API方法分析**:
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_records`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_attendance_records`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_checkin_record`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_checkin_record`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_checkin_record`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_statistics`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_exceptions`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `process_attendance_exception`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_report`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_records_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `batch_get_attendance_records_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_checkin_record_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `update_checkin_record_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `delete_checkin_record_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_statistics_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_exceptions_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `process_attendance_exception_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_report_builder`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `attendance_status`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `employee_ids`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `checkin_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `checkin_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `employee_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `statistics_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `employee_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `exception_types`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_size`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `page_token`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `process_data`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `report_type`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `employee_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `department_id`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `start_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `end_date`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `format`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `build`: ✅ Builder, ✅ StandardResponse, ✅ 文档

#### v1 - src/service/ehr/v1/mod.rs
**一致性得分**: 46%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_employee`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_employee`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_employee`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `query_employees`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_employee`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_department`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_departments`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_position`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_positions`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `create_salary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_employee_salary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `adjust_salary`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_attendance_records`: ✅ Builder, ✅ StandardResponse, ✅ 文档
- `create_performance_evaluation`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_employee_performance`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_employee' 缺少Builder模式实现
   💡 建议为 'create_employee' 创建对应的Builder结构体
🟡 方法 'create_department' 缺少Builder模式实现
   💡 建议为 'create_department' 创建对应的Builder结构体
🟡 方法 'create_position' 缺少Builder模式实现
   💡 建议为 'create_position' 创建对应的Builder结构体
🟡 方法 'create_salary' 缺少Builder模式实现
   💡 建议为 'create_salary' 创建对应的Builder结构体
🟡 方法 'create_performance_evaluation' 缺少Builder模式实现
   💡 建议为 'create_performance_evaluation' 创建对应的Builder结构体

#### v1 - src/service/ehr/v1/leave.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `adjust_leave_balance_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `approve_leave_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `cancel_leave_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_leave_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `create_leave_rule_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `delete_leave_rule_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_leave_rules_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_leave_statistics_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `get_pending_approvals_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `query_leave_balance_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `query_leave_records_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `update_leave_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `update_leave_rule_builder`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'adjust_leave_balance_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'adjust_leave_balance_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'approve_leave_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'approve_leave_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'cancel_leave_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'cancel_leave_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_leave_builder' 缺少Builder模式实现
   💡 建议为 'create_leave_builder' 创建对应的Builder结构体
🔴 方法 'create_leave_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'create_leave_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_leave_rule_builder' 缺少Builder模式实现
   💡 建议为 'create_leave_rule_builder' 创建对应的Builder结构体
🔴 方法 'create_leave_rule_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'create_leave_rule_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'delete_leave_rule_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'delete_leave_rule_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_leave_rules_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'get_leave_rules_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_leave_statistics_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'get_leave_statistics_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'get_pending_approvals_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'get_pending_approvals_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'query_leave_balance_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'query_leave_balance_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'query_leave_records_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'query_leave_records_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'update_leave_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'update_leave_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'update_leave_rule_builder' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'update_leave_rule_builder' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ehr/attachment/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `download_attachment`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ehr/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ehr/employee/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_employees`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### ai 服务

#### unknown - src/service/ai/speech_to_text/mod.rs
**一致性得分**: 39%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `file_recognize`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `stream_recognize`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ai/optical_char_recognition/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `basic_recognize`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ai/translation/mod.rs
**一致性得分**: 100%

#### unknown - src/service/ai/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/ai/document_ai/mod.rs
**一致性得分**: 37%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `parse_resume`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_id_card`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_driving_license`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_bank_card`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_business_license`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_vat_invoice`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `extract_contract_fields`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_business_card`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_vehicle_invoice`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_health_certificate`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_hkm_mainland_travel_permit`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_tw_mainland_travel_permit`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_chinese_passport`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_vehicle_license`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_train_invoice`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_taxi_invoice`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_food_produce_license`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `recognize_food_manage_license`: ❌ Builder, ✅ StandardResponse, ✅ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### trust_party 服务

#### unknown - src/service/trust_party/collaboration_organization/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `list_organizations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization_structure`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization_user`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_organization_department`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_shared_member_scope`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `admin_list_organizations`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/trust_party/searchable_visible_rules/mod.rs
**一致性得分**: 32%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `create_rule`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `update_rule`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_rules`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `delete_rule`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟡 方法 'create_rule' 缺少Builder模式实现
   💡 建议为 'create_rule' 创建对应的Builder结构体
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/trust_party/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/trust_party/applications/v1/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/trust_party/applications/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

### workplace 服务

#### unknown - src/service/workplace/mod.rs
**一致性得分**: 0%

**API方法分析**:
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ❌ StandardResponse, ❌ 文档

**发现的问题**:
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🔴 方法 'new' 未使用统一的StandardResponse错误处理
   💡 建议使用 .into_result() 方法进行统一错误处理
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/workplace/workplace_access_data/mod.rs
**一致性得分**: 22%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `search`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_custom`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `search_custom_widget`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

#### unknown - src/service/workplace/app_recommend/mod.rs
**一致性得分**: 22%

**API方法分析**:
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `get_favourite_apps`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `get_recommended_apps`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `list_recommend_rules`: ❌ Builder, ✅ StandardResponse, ✅ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档
- `new`: ❌ Builder, ✅ StandardResponse, ❌ 文档

**发现的问题**:
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数
🟢 方法 'new' 缺少文档注释
   💡 建议添加 /// 文档注释描述方法功能和参数

## 🚀 改进建议

### 高优先级
1. **统一错误处理**: 为所有API方法实现StandardResponse模式
2. **补充Builder模式**: 为create类型的方法添加Builder支持

### 中优先级
1. **完善文档**: 为所有公开API添加详细的文档注释
2. **命名规范**: 确保所有API遵循一致的命名约定

### 低优先级
1. **代码风格**: 统一代码格式和结构
2. **性能优化**: 识别和优化潜在的性能瓶颈

