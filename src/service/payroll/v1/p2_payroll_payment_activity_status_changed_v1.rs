use serde::{Deserialize, Serialize};

use crate::event::{context::EventContext, dispatcher::EventHandler};

/// 发薪活动状态变更事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentActivityStatusChangedData {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 变更前状态
    pub old_status: String,
    /// 变更后状态
    pub new_status: String,
    /// 状态变更时间戳（秒）
    pub changed_time: i64,
    /// 变更操作人
    pub operator: Option<OperatorInfo>,
    /// 变更原因
    pub reason: Option<String>,
    /// 薪资组ID
    pub paygroup_id: Option<String>,
    /// 发薪周期
    pub payment_period: Option<String>,
}

/// 操作人信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 用户类型
    pub user_type: Option<String>,
}

/// 发薪活动状态变更事件
#[derive(Debug, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityStatusChangedV1 {
    /// 事件通用信息
    #[serde(flatten)]
    pub context: EventContext,
    /// 事件业务数据
    pub event: PaymentActivityStatusChangedData,
}

/// 发薪活动状态变更事件处理器实现
pub struct P2PayrollPaymentActivityStatusChangedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityStatusChangedV1) -> anyhow::Result<()> + Send + Sync,
{
    pub(crate) f: F,
}

impl<F> P2PayrollPaymentActivityStatusChangedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityStatusChangedV1) -> anyhow::Result<()> + Send + Sync,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> EventHandler for P2PayrollPaymentActivityStatusChangedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityStatusChangedV1) -> anyhow::Result<()> + Send + Sync,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event = serde_json::from_slice::<P2PayrollPaymentActivityStatusChangedV1>(payload)?;
        (self.f)(event)
    }
}
