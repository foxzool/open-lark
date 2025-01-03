use crate::service::bitable::v1::Person;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub fields: HashMap<String, Value>,
    /// 记录Id
    pub record_id: String,
    /// 创建人
    pub created_by: Option<Person>,
    /// 创建时间
    pub created_time: Option<u128>,
    /// 修改人
    pub last_modified_by: Option<Person>,
    /// 最近更新时间
    pub last_modified_time: Option<u128>,
}
