use serde::{Deserialize, Serialize};;

use crate::event::{context::EventHeader, dispatcher::EventHandler};

/// 考勤用户任务状态变更事件 (user.task_status_change_event)
#[derive(Debug, Serialize, Deserialize)]
pub struct P2AttendanceUserTaskStatusChangeV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2AttendanceUserTaskStatusChangeV1Data,
}

pub(crate) struct P2AttendanceUserTaskStatusChangeV1ProcessorImpl<F>
where
    F: Fn(P2AttendanceUserTaskStatusChangeV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2AttendanceUserTaskStatusChangeV1ProcessorImpl<F>
where
    F: Fn(P2AttendanceUserTaskStatusChangeV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2AttendanceUserTaskStatusChangeV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2AttendanceUserTaskStatusChangeV1ProcessorImpl<F>
where
    F: Fn(P2AttendanceUserTaskStatusChangeV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2AttendanceUserTaskStatusChangeV1ProcessorImpl { f }
    }
}

/// 考勤用户任务状态变更事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2AttendanceUserTaskStatusChangeV1Data {
    /// 用户信息
    pub user_id: AttendanceUserId,
    /// 状态变更信息
    pub task_status_change: AttendanceTaskStatusChange,
    /// 租户key
    pub tenant_key: String,
}

/// 考勤事件中的用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceUserId {
    /// 用户的 union id
    pub union_id: String,
    /// 用户的 user id
    pub user_id: String,
    /// 用户的 open id
    pub open_id: String,
    /// 用户的 employee id
    pub employee_id: Option<String>,
}

/// 考勤任务状态变更信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceTaskStatusChange {
    /// 任务ID
    pub task_id: String,
    /// 用户ID
    pub user_id: String,
    /// 员工ID
    pub employee_id: Option<String>,
    /// 考勤组ID
    pub group_id: String,
    /// 班次ID
    pub shift_id: String,
    /// 记录日期，格式：yyyy-MM-dd
    pub record_date: String,
    /// 班次名称
    pub shift_name: String,
    /// 原状态，1：正常，2：早退，3：迟到，4：严重迟到，5：缺卡，6：无效，7：无班次，8：休息
    pub old_status: i32,
    /// 新状态，1：正常，2：早退，3：迟到，4：严重迟到，5：缺卡，6：无效，7：无班次，8：休息
    pub new_status: i32,
    /// 状态变更原因
    pub change_reason: String,
    /// 变更类型，1：管理员修改，2：补卡，3：审批通过，4：系统自动调整
    pub change_type: i32,
    /// 变更操作人ID
    pub operator_id: Option<String>,
    /// 变更时间戳（毫秒）
    pub change_time: String,
    /// 变更备注
    pub change_comment: Option<String>,
    /// 相关的打卡记录
    pub check_records: Option<Vec<AttendanceCheckRecord>>,
}

/// 打卡记录信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceCheckRecord {
    /// 打卡记录ID
    pub record_id: String,
    /// 打卡时间，格式：yyyy-MM-dd HH:mm:ss
    pub check_time: String,
    /// 打卡类型，1：上班打卡，2：下班打卡
    pub check_type: i32,
    /// 打卡结果，1：正常，2：早退，3：迟到，4：严重迟到，5：缺卡，6：无效
    pub check_result: i32,
    /// 位置信息
    pub location: Option<AttendanceLocation>,
    /// 是否外勤打卡
    pub is_field: bool,
    /// 是否补卡
    pub is_remedy: bool,
    /// 打卡备注
    pub comment: Option<String>,
}

/// 打卡位置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceLocation {
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 位置名称
    pub address: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use crate::event::context::EventContext;

    #[test]
    fn test_decode_attendance_status_change_event() {
        let event_data = json!({
            "schema": "2.0",
            "header": {
                "event_id": "7db4fd0bb90cfa6127e3aaa446d39b39",
                "token": "",
                "create_time": "1719211482721",
                "event_type": "attendance.user_task.status_change_v1",
                "tenant_key": "tenant_key",
                "app_id": "app_id"
            },
            "event": {
                "user_id": {
                    "open_id": "ou_b434284f852b1531071855b16d19f40b",
                    "union_id": "on_526dbf7b9ef1fda341aecb79a2481662",
                    "user_id": "aa5cf9d7",
                    "employee_id": "emp_001"
                },
                "task_status_change": {
                    "task_id": "task_123456",
                    "user_id": "aa5cf9d7",
                    "employee_id": "emp_001",
                    "group_id": "group_001",
                    "shift_id": "shift_001",
                    "record_date": "2024-06-20",
                    "shift_name": "标准班次",
                    "old_status": 3,
                    "new_status": 1,
                    "change_reason": "补卡申请通过",
                    "change_type": 3,
                    "operator_id": "admin_001",
                    "change_time": "1719211482485",
                    "change_comment": "员工提交补卡申请，审批通过",
                    "check_records": [
                        {
                            "record_id": "record_001",
                            "check_time": "2024-06-20 09:30:00",
                            "check_type": 1,
                            "check_result": 1,
                            "location": {
                                "latitude": 39.908822,
                                "longitude": 116.397128,
                                "address": "北京市朝阳区"
                            },
                            "is_field": false,
                            "is_remedy": true,
                            "comment": "补卡"
                        }
                    ]
                },
                "tenant_key": "133195a24e8f575d"
            }
        });

        let event_context: EventContext = serde_json::from_value(event_data).unwrap();

        // println!("{:#?}", event_context);
        assert_eq!(event_context.schema, Some("2.0".to_string()));
    }
}
