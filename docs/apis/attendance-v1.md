# 飞书考勤开发规划

## 参考文档

[接入指南](https://open.feishu.cn/document/server-docs/attendance-v1/attendance-development-guidelines)

## 考勤班次

[创建班次](https://open.feishu.cn/document/server-docs/attendance-v1/shift/create)
[删除班次](https://open.feishu.cn/document/server-docs/attendance-v1/shift/delete)
[按 ID 查询班次](https://open.feishu.cn/document/server-docs/attendance-v1/shift/get)
[按名称查询班次](https://open.feishu.cn/document/server-docs/attendance-v1/shift/query)
[查询所有班次](https://open.feishu.cn/document/server-docs/attendance-v1/shift/list)

## 考勤排版

[创建或修改排班表](https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/batch_create)
[查询排班表](https://open.feishu.cn/document/server-docs/attendance-v1/user_daily_shift/query)
[创建或修改临时排班](https://open.feishu.cn/document/attendance-v1/user_daily_shift/batch_create_temp)

## 考勤管理

[查询考勤组下所有成员](https://open.feishu.cn/document/attendance-v1/group/list_user)
[创建或修改考勤组](https://open.feishu.cn/document/server-docs/attendance-v1/group/create)
[删除考勤组](https://open.feishu.cn/open-apis/attendance/v1/groups/:group_id)
[按 ID 查询考勤组](https://open.feishu.cn/document/server-docs/attendance-v1/group/get)
[按名称查询考勤组](https://open.feishu.cn/document/server-docs/attendance-v1/group/search)
[查询所有考勤组](https://open.feishu.cn/document/server-docs/attendance-v1/group/list)

## 考勤用户管理

[修改用户人脸识别信息](https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/modify)
[批量查询用户人脸识别信息](https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/query)
[上传用户人脸识别照片](https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/upload)
[下载用户人脸识别照片](https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/download)

## 考勤统计

[更新统计设置](https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/update)
[查询统计表头](https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query-2)
[查询统计设置](https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query)
[查询统计数据](https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query-3)

## 假勤审批

[获取审批数据](https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query)
[写入审批结果](https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/create)
[通知审批状态更新](https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/process)

## 考勤补卡

[通知补卡审批发起](https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/create)
[获取可补卡时间](https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query_user_allowed_remedys)
[获取补卡记录](https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query)

## 归档报表

[查询归档报表表头](https://open.feishu.cn/document/attendance-v1/archive_rule/user_stats_fields_query)
[写入归档报表结果](https://open.feishu.cn/document/attendance-v1/archive_rule/upload_report)
[删除归档报表行数据](https://open.feishu.cn/document/attendance-v1/archive_rule/del_report)
[查询所有归档规则](https://open.feishu.cn/document/attendance-v1/archive_rule/list)

## 打卡信息管理

[导入打卡流水](https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_create)
[查询打卡流水](https://open.feishu.cn/document/server-docs/attendance-v1/user_task/get)
[批量查询打卡流水](https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query-2)
[删除打卡流水](https://open.feishu.cn/document/attendance-v1/user_task/batch_del)
[查询打卡结果](https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query)

## 休假获取过期发放记录

[通过过期时间获取发放记录](https://open.feishu.cn/document/server-docs/attendance-v1/leave_employ_expire_record/get)

## 休假发放记录

[修改发放记录](https://open.feishu.cn/document/server-docs/attendance-v1/leave_accrual_record/patch)

## 事件

[打卡流水](https://open.feishu.cn/document/server-docs/attendance-v1/event/user-attendance-records-event)
[用户任务状态变更](https://open.feishu.cn/document/server-docs/attendance-v1/event/user-task-status-change-event)