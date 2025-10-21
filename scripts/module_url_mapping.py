#!/usr/bin/env python3
"""
模块URL映射数据
从官方API数据提取的模块基础URL映射
"""

# 模块URL映射 - 用于为没有文档URL的模块添加文档
MODULE_URL_MAPPING = {
    "acs": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1",
        "api_count": 11,
        "apis": [
            {"method": "GET", "name": "获取单个用户信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user/get"},
            {"method": "GET", "name": "获取用户列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user/list"},
            {"method": "PUT", "name": "上传人脸图片", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user-face/update"},
            {"method": "GET", "name": "下载人脸图片", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user-face/get"},
            {"method": "POST", "name": "设备绑定权限组", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/device_bind"},
            # ... 还有 6 个API
        ]
    },
    "admin": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 13,
        "apis": [
            {"method": "GET", "name": "获取部门维度的用户活跃和功能使用数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_dept_stat/list"},
            {"method": "GET", "name": "获取用户维度的用户活跃和功能使用数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_user_stat/list"},
            {"method": "POST", "name": "创建勋章", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/create"},
            {"method": "PUT", "name": "修改勋章信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/update"},
            {"method": "POST", "name": "上传勋章图片", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create"},
            # ... 还有 8 个API
        ]
    },
    "ai": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 22,
        "apis": [
            {"method": "POST", "name": "识别文件中的简历信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/resume/parse"},
            {"method": "POST", "name": "识别文件中的机动车发票", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_invoice/recognize"},
            {"method": "POST", "name": "识别文件中的健康证", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/health_certificate/recognize"},
            {"method": "POST", "name": "识别文件中的港澳居民来往内地通行证", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/hkm_mainland_travel_permit/recognize"},
            {"method": "POST", "name": "识别文件中的台湾居民来往大陆通行证", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/tw_mainland_travel_permit/recognize"},
            # ... 还有 17 个API
        ]
    },
    "aily": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1",
        "api_count": 21,
        "apis": [
            {"method": "POST", "name": "创建会话", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/create"},
            {"method": "PUT", "name": "更新会话", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/update"},
            {"method": "GET", "name": "获取会话", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/get"},
            {"method": "DELETE", "name": "删除会话", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/delete"},
            {"method": "POST", "name": "发送 Aily 消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-aily_message/create"},
            # ... 还有 16 个API
        ]
    },
    "application": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6",
        "api_count": 35,
        "apis": [
            {"method": "PUT", "name": "转移应用所有者", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-owner/update"},
            {"method": "PUT", "name": "更新应用协作者", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-collaborators/update"},
            {"method": "GET", "name": "获取应用协作者列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-collaborators/get"},
            {"method": "GET", "name": "获取应用信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get"},
            {"method": "GET", "name": "获取应用版本信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/get"},
            # ... 还有 30 个API
        ]
    },
    "approval": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 48,
        "apis": [
            {"method": "POST", "name": "创建审批定义", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create"},
            {"method": "GET", "name": "查看指定审批定义", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get"},
            {"method": "POST", "name": "创建审批实例", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create"},
            {"method": "POST", "name": "撤回审批实例", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cancel"},
            {"method": "POST", "name": "抄送审批实例", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cc"},
            # ... 还有 43 个API
        ]
    },
    "attendance": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 37,
        "apis": [
            {"method": "POST", "name": "创建班次", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create"},
            {"method": "DELETE", "name": "删除班次", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete"},
            {"method": "GET", "name": "按 ID 查询班次", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/get"},
            {"method": "POST", "name": "按名称查询班次", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query"},
            {"method": "GET", "name": "查询所有班次", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/list"},
            # ... 还有 32 个API
        ]
    },
    "authentication": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 13,
        "apis": [
            {"method": "GET", "name": "获取用户信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get"},
            {"method": "POST", "name": "批量获取脱敏的用户登录信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/passport-v1/session/query"},
            {"method": "POST", "name": "退出登录", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/passport-v1/session/logout"},
            {"method": "POST", "name": "自建应用获取 tenant_access_token", "url": "https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal"},
            {"method": "POST", "name": "自建应用获取 app_access_token", "url": "https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal"},
            # ... 还有 8 个API
        ]
    },
    "calendar": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 26,
        "apis": [
            {"method": "POST", "name": "查询主日历信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary"},
            {"method": "POST", "name": "批量获取主日历信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primarys"},
            {"method": "POST", "name": "批量查询日历信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/mget"},
            {"method": "POST", "name": "查询主日历日程忙闲信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/list"},
            {"method": "POST", "name": "批量查询主日历日程忙闲信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/batch"},
            # ... 还有 21 个API
        ]
    },
    "cardkit": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1",
        "api_count": 10,
        "apis": [
            {"method": "POST", "name": "创建卡片实体", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/create"},
            {"method": "PATCH", "name": "更新卡片实体配置", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/settings"},
            {"method": "POST", "name": "局部更新卡片实体", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/batch_update"},
            {"method": "PUT", "name": "全量更新卡片实体", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/update"},
            {"method": "POST", "name": "新增组件", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/create"},
            # ... 还有 5 个API
        ]
    },
    "contact": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 74,
        "apis": [
            {"method": "POST", "name": "创建用户", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create"},
            {"method": "PATCH", "name": "修改用户部分信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/patch"},
            {"method": "PATCH", "name": "更新用户 ID", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update_user_id"},
            {"method": "GET", "name": "获取单个用户信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get"},
            {"method": "GET", "name": "批量获取用户信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch"},
            # ... 还有 69 个API
        ]
    },
    "corehr": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2",
        "api_count": 143,
        "apis": [
            {"method": "POST", "name": "查询枚举信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/enum/search"},
            {"method": "POST", "name": "查询国家/地区信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search"},
            {"method": "POST", "name": "查询省份/主要行政区信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region_subdivision/search"},
            {"method": "POST", "name": "查询城市信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-city/search"},
            {"method": "POST", "name": "查询区/县信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-district/search"},
            # ... 还有 138 个API
        ]
    },
    "directory": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1",
        "api_count": 16,
        "apis": [
            {"method": "DELETE", "name": "离职员工", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/delete"},
            {"method": "POST", "name": "恢复离职员工", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/resurrect"},
            {"method": "PATCH", "name": "更新在职员工为待离职", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/to_be_resigned"},
            {"method": "PATCH", "name": "更新待离职成员为在职", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/regular"},
            {"method": "POST", "name": "创建部门", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/create"},
            # ... 还有 11 个API
        ]
    },
    "ehr": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 1,
        "apis": [
            {"method": "GET", "name": "批量获取员工花名册信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/ehr/ehr-v1/employee/list"},
        ]
    },
    "helpdesk": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1",
        "api_count": 50,
        "apis": [
            {"method": "PATCH", "name": "更新客服信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/patch"},
            {"method": "GET", "name": "获取客服邮箱", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/agent_email"},
            {"method": "POST", "name": "创建客服工作日程", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_schedule/create"},
            {"method": "DELETE", "name": "删除客服工作日程", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/delete"},
            {"method": "PATCH", "name": "更新客服工作日程", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/patch"},
            # ... 还有 45 个API
        ]
    },
    "hire": {
        "base_url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1",
        "api_count": 180,
        "apis": [
            {"method": "GET", "name": "获取申请表模板列表", "url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/portal_apply_schema/list"},
            {"method": "POST", "name": "查询地点列表", "url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/location/query"},
            {"method": "GET", "name": "获取地址列表", "url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/location/list"},
            {"method": "GET", "name": "获取角色详情", "url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/role/get"},
            {"method": "GET", "name": "获取角色列表", "url": "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/role/list"},
            # ... 还有 175 个API
        ]
    },
    "im": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 74,
        "apis": [
            {"method": "POST", "name": "发送消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create"},
            {"method": "POST", "name": "回复消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply"},
            {"method": "PUT", "name": "编辑消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update"},
            {"method": "POST", "name": "转发消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward"},
            {"method": "POST", "name": "合并转发消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/merge_forward"},
            # ... 还有 69 个API
        ]
    },
    "mail": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 67,
        "apis": [
            {"method": "POST", "name": "创建邮箱文件夹", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/create"},
            {"method": "DELETE", "name": "删除邮箱文件夹", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/delete"},
            {"method": "PATCH", "name": "修改邮箱文件夹", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/patch"},
            {"method": "GET", "name": "列出邮箱文件夹", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/list"},
            {"method": "GET", "name": "获取邮件卡片的邮件列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/get_by_card"},
            # ... 还有 62 个API
        ]
    },
    "mdm": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3",
        "api_count": 4,
        "apis": [
            {"method": "GET", "name": "根据主数据编码批量查询国家/地区", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/batch_country_region/get"},
            {"method": "GET", "name": "分页批量查询国家/地区", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/list"},
            {"method": "POST", "name": "用户数据维度绑定", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v1/user_auth_data_relation/bind"},
            {"method": "POST", "name": "用户数据维度解绑", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v1/user_auth_data_relation/unbind"},
        ]
    },
    "minutes": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1",
        "api_count": 4,
        "apis": [
            {"method": "GET", "name": "下载妙记音视频文件", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get"},
            {"method": "GET", "name": "导出妙记文字记录", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-transcript/get"},
            {"method": "GET", "name": "获取妙记统计数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-statistics/get"},
            {"method": "GET", "name": "获取妙记信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/get"},
        ]
    },
    "moments": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/moments-v1",
        "api_count": 1,
        "apis": [
            {"method": "GET", "name": "查询帖子信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/moments-v1/post/get"},
        ]
    },
    "okr": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 12,
        "apis": [
            {"method": "POST", "name": "创建 OKR 周期", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/create"},
            {"method": "PATCH", "name": "修改 OKR 周期状态", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/patch"},
            {"method": "GET", "name": "获取 OKR 周期列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/list"},
            {"method": "GET", "name": "获取 OKR 周期规则", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period_rule/list"},
            {"method": "GET", "name": "获取用户的 OKR 列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/user-okr/list"},
            # ... 还有 7 个API
        ]
    },
    "payroll": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1",
        "api_count": 12,
        "apis": [
            {"method": "GET", "name": "批量查询算薪项", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/acct_item/list"},
            {"method": "GET", "name": "获取薪资组基本信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/paygroup/list"},
            {"method": "GET", "name": "获取外部数据源配置信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource/list"},
            {"method": "POST", "name": "创建 / 更新外部算薪数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource_record/save"},
            {"method": "POST", "name": "批量查询外部算薪数据记录", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource_record/query"},
            # ... 还有 7 个API
        ]
    },
    "performance": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2",
        "api_count": 20,
        "apis": [
            {"method": "GET", "name": "获取周期列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/semester/list"},
            {"method": "POST", "name": "获取项目列表", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/activity/query"},
            {"method": "POST", "name": "批量查询补充信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_information/query"},
            {"method": "POST", "name": "批量导入补充信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_information/import"},
            {"method": "DELETE", "name": "批量删除补充信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_informations-batch/delete"},
            # ... 还有 15 个API
        ]
    },
    "report": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 3,
        "apis": [
            {"method": "GET", "name": "查询规则", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/rule/query"},
            {"method": "POST", "name": "移除规则看板", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/rule-view/remove"},
            {"method": "POST", "name": "查询任务", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/task/query"},
        ]
    },
    "search": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2",
        "api_count": 14,
        "apis": [
            {"method": "POST", "name": "搜索消息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/message/create"},
            {"method": "POST", "name": "搜索应用", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/app/create"},
            {"method": "POST", "name": "创建数据源", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/create"},
            {"method": "DELETE", "name": "删除数据源", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/delete"},
            {"method": "PATCH", "name": "修改数据源", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/patch"},
            # ... 还有 9 个API
        ]
    },
    "task": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2",
        "api_count": 71,
        "apis": [
            {"method": "POST", "name": "创建任务", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/create"},
            {"method": "PATCH", "name": "更新任务", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/patch"},
            {"method": "GET", "name": "获取任务详情", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/get"},
            {"method": "DELETE", "name": "删除任务", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/delete"},
            {"method": "POST", "name": "添加任务成员", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_members"},
            # ... 还有 66 个API
        ]
    },
    "tenant": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2",
        "api_count": 2,
        "apis": [
            {"method": "GET", "name": "获取企业席位信息接口", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query"},
            {"method": "GET", "name": "获取企业信息", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant/query"},
        ]
    },
    "vc": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
        "api_count": 55,
        "apis": [
            {"method": "POST", "name": "预约会议", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply"},
            {"method": "DELETE", "name": "删除预约", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/delete"},
            {"method": "PUT", "name": "更新预约", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/update"},
            {"method": "GET", "name": "获取预约", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get"},
            {"method": "GET", "name": "获取活跃会议", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get_active_meeting"},
            # ... 还有 50 个API
        ]
    },
    "workplace": {
        "base_url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1",
        "api_count": 3,
        "apis": [
            {"method": "POST", "name": "获取工作台访问数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/workplace_access_data/search"},
            {"method": "POST", "name": "获取定制工作台访问数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/custom_workplace_access_data/search"},
            {"method": "POST", "name": "获取定制工作台小组件访问数据", "url": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/workplace_block_access_data/search"},
        ]
    },
}

# 缺少官方数据的已实现模块
MODULES_WITHOUT_OFFICIAL_DATA = [
    "apass",
    "bot",
    "cloud_docs",
    "elearning",
    "group",
    "human_authentication",
    "lingo",
    "personal_settings",
    "security_and_compliance",
    "tenant_tag",
    "trust_party",
    "verification",
]
