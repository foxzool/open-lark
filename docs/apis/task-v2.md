# 任务

[任务概述](https://open.feishu.cn/document/task-v2/overview)
[常见问题](https://open.feishu.cn/document/task-v2/faq)

## 任务

[任务功能概述](https://open.feishu.cn/document/task-v2/task/overview)
[创建任务](https://open.feishu.cn/document/task-v2/task/create)
[更新任务](https://open.feishu.cn/document/task-v2/task/patch)
[获取任务详情](https://open.feishu.cn/document/task-v2/task/get)
[删除任务](https://open.feishu.cn/document/task-v2/task/delete)
[添加任务成员](https://open.feishu.cn/document/task-v2/task/add_members)
[移除任务成员](https://open.feishu.cn/document/task-v2/task/remove_members)
[列取任务列表](https://open.feishu.cn/document/task-v2/task/list)
[列取任务所在清单](https://open.feishu.cn/document/task-v2/task/tasklists)
[任务加入清单](https://open.feishu.cn/document/task-v2/task/add_tasklist)
[任务移出清单](https://open.feishu.cn/document/task-v2/task/remove_tasklist)
[添加任务提醒](https://open.feishu.cn/document/task-v2/task/add_reminders)
[移除任务提醒](https://open.feishu.cn/document/task-v2/task/remove_reminders)
[添加依赖](https://open.feishu.cn/document/task-v2/task/add_dependencies)
[移除依赖](https://open.feishu.cn/document/task-v2/task/remove_dependencies)

### 开发者提示：在清单中管理任务

- 通过 `CreateTaskRequest` 的 `tasklist_guid` 字段，可以在任务创建时直接指定所属清单。
- 如需将已存在的任务加入新的清单，可使用 `TaskService::add_tasklist` 方法并提供 `AddTaskTasklistRequest`。

```rust,no_run
use open_lark::prelude::*;
use open_lark::service::task::v2::task::AddTaskTasklistRequest;

async fn attach_task_to_list(
    client: &LarkClient,
    task_guid: &str,
    tasklist_guid: String,
) -> SDKResult<()> {
    let request = AddTaskTasklistRequest {
        tasklist_guid,
        section_guid: None,
    };

    let response = client
        .task
        .task
        .add_tasklist(task_guid, request, None, None)
        .await?;

    if let Some(data) = response.data {
        println!("任务已加入清单: {:?}", data.tasklist.name);
    }

    Ok(())
}
```

## 子任务

[创建子任务](https://open.feishu.cn/document/task-v2/task-subtask/create)
[获取任务的子任务列表](https://open.feishu.cn/document/task-v2/task-subtask/list)

## 清单

[清单功能概述](https://open.feishu.cn/document/task-v2/tasklist/overview)
[创建清单](https://open.feishu.cn/document/task-v2/tasklist/create)
[获取清单详情](https://open.feishu.cn/document/task-v2/tasklist/get)
[更新清单](https://open.feishu.cn/document/task-v2/tasklist/patch)
[删除清单](https://open.feishu.cn/document/task-v2/tasklist/delete)
[添加清单成员](https://open.feishu.cn/document/task-v2/tasklist/add_members)
[移除清单成员](https://open.feishu.cn/document/task-v2/tasklist/remove_members)
[获取清单任务列表](https://open.feishu.cn/document/task-v2/tasklist/tasks)
[获取清单列表](https://open.feishu.cn/document/task-v2/tasklist/list)

## 清单动态订阅

[创建动态订阅](https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/create)
[获取动态订阅](https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/get)
[列取动态订阅](https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/list)
[更新动态订阅](https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/patch)
[删除动态订阅](https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/delete)

## 评论

[评论功能概述](https://open.feishu.cn/document/task-v2/comment/overview)
[创建评论](https://open.feishu.cn/document/task-v2/comment/create)
[获取评论详情](https://open.feishu.cn/document/task-v2/comment/get)
[更新评论](https://open.feishu.cn/document/task-v2/comment/patch)
[删除评论](https://open.feishu.cn/document/task-v2/comment/delete)
[获取评论列表](https://open.feishu.cn/document/task-v2/comment/list)

## 附件

[附件功能概述](https://open.feishu.cn/document/task-v2/attachment/attachment-feature-overview)
[上传附件](https://open.feishu.cn/document/task-v2/attachment/upload)
[列取附件](https://open.feishu.cn/document/task-v2/attachment/list)
[获取附件](https://open.feishu.cn/document/task-v2/attachment/get)
[删除附件](https://open.feishu.cn/document/task-v2/attachment/delete)

## 自定义分组

[自定义分组功能概述](https://open.feishu.cn/document/task-v2/section/section-feature-overview)
[创建自定义分组](https://open.feishu.cn/document/task-v2/section/create)
[获取自定义分组详情](https://open.feishu.cn/document/task-v2/section/get)
[更新自定义分组](https://open.feishu.cn/document/task-v2/section/patch)
[删除自定义分组](https://open.feishu.cn/document/task-v2/section/delete)
[获取自定义分组列表](https://open.feishu.cn/document/task-v2/section/list)
[获取自定义分组任务列表](https://open.feishu.cn/document/task-v2/section/tasks)

## 自定义字段

[自定义字段概述](https://open.feishu.cn/document/task-v2/custom_field/custom-field-overview)
[创建自定义字段](https://open.feishu.cn/document/task-v2/custom_field/create)
[获取自定义字段](https://open.feishu.cn/document/task-v2/custom_field/get)
[更新自定义字段](https://open.feishu.cn/document/task-v2/custom_field/patch)
[列取自定义字段](https://open.feishu.cn/document/task-v2/custom_field/list)
[将自定义字段加入资源](https://open.feishu.cn/document/task-v2/custom_field/add)
[将自定义字段移出资源](https://open.feishu.cn/document/task-v2/custom_field/remove)

## 自定义字段选项

[创建自定义任务选项](https://open.feishu.cn/document/task-v2/custom_field-option/create)
[更新自定义字段选项](https://open.feishu.cn/document/task-v2/custom_field-option/patch)
