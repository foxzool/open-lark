use serde::{Deserialize, Serialize};
/// 筛选条件,
#[derive(Debug, Clone)]
pub struct SheetFilterCondition {
    /// 筛选类型
    pub filter_type: String,
    /// 比较类型
    pub compare_type: Option<String>,
    /// 筛选参数
    pub expected: Vec<String>}
impl SheetFilterCondition {
    
}
#[derive(Debug, Clone)]
pub struct SheetFilterConditionBuilder {
    condition: SheetFilterCondition}
impl SheetFilterConditionBuilder {
    
}
