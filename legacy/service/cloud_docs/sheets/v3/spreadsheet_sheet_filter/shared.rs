use serde::{Deserialize, Serialize};

/// 筛选条件
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SheetFilterCondition {
    /// 筛选类型
    pub filter_type: String,
    /// 比较类型
    pub compare_type: Option<String>,
    /// 筛选参数
    pub expected: Vec<String>,
}

impl SheetFilterCondition {
    pub fn builder() -> SheetFilterConditionBuilder {
        SheetFilterConditionBuilder::default()
    }
}

#[derive(Default)]
pub struct SheetFilterConditionBuilder {
    condition: SheetFilterCondition,
}

impl SheetFilterConditionBuilder {
    /// 筛选类型
    pub fn filter_type(mut self, filter_type: impl ToString) -> Self {
        self.condition.filter_type = filter_type.to_string();
        self
    }

    /// 比较类型
    pub fn compare_type(mut self, compare_type: impl ToString) -> Self {
        self.condition.compare_type = Some(compare_type.to_string());
        self
    }

    /// 筛选参数
    pub fn expected(mut self, expected: Vec<String>) -> Self {
        self.condition.expected = expected;
        self
    }

    pub fn build(self) -> SheetFilterCondition {
        self.condition
    }
}
