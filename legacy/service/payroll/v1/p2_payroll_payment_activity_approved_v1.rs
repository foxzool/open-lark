use serde::{Deserialize, Serialize};

use crate::event::{context::EventContext, dispatcher::EventHandler};

/// 发薪活动封存事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentActivityApprovedData {
    /// 发薪活动ID
    pub payment_activity_id: String,
    /// 封存时间戳（秒）
    pub approved_time: i64,
    /// 封存操作人
    pub operator: Option<OperatorInfo>,
    /// 封存原因
    pub reason: Option<String>,
    /// 薪资组ID
    pub paygroup_id: Option<String>,
    /// 发薪周期
    pub payment_period: Option<String>,
    /// 发薪总金额
    pub total_amount: Option<PaymentAmount>,
    /// 涉及员工数量
    pub employee_count: Option<u32>,
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

/// 发薪金额信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentAmount {
    /// 金额
    pub amount: String,
    /// 货币类型
    pub currency: String,
}

/// 发薪活动封存事件
#[derive(Debug, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityApprovedV1 {
    /// 事件通用信息
    #[serde(flatten)]
    pub context: EventContext,
    /// 事件业务数据
    pub event: PaymentActivityApprovedData,
}

/// 发薪活动封存事件处理器实现
pub struct P2PayrollPaymentActivityApprovedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityApprovedV1) -> anyhow::Result<()> + Send + Sync,
{
    pub(crate) f: F,
}

impl<F> P2PayrollPaymentActivityApprovedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityApprovedV1) -> anyhow::Result<()> + Send + Sync,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> EventHandler for P2PayrollPaymentActivityApprovedV1ProcessorImpl<F>
where
    F: Fn(P2PayrollPaymentActivityApprovedV1) -> anyhow::Result<()> + Send + Sync,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event = serde_json::from_slice::<P2PayrollPaymentActivityApprovedV1>(payload)?;
        (self.f)(event)
    }
}
