use crate::service::bitable::v1::Person;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Record {
    pub fields: HashMap<String, Value>,
    /// 记录Id
    pub record_id: Option<String>,
    /// 创建人
    pub created_by: Option<Person>,
    /// 创建时间
    pub created_time: Option<u128>,
    /// 修改人
    pub last_modified_by: Option<Person>,
    /// 最近更新时间
    pub last_modified_time: Option<u128>,
}

impl Record {
    pub fn from_json(json: Value) -> Self {
        let obj = json.as_object().expect("json must be a object");
        let mut fields = HashMap::new();
        for (k, v) in obj.iter() {
            fields.insert(k.clone(), v.clone());
        }
        Record {
            fields,
            record_id: None,
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        }
    }
}

/// 一些帮助函数
impl Record {
    /// 获取文本
    pub fn get_text(&self, key: &str) -> Option<String> {
        let value = self.fields.get(key)?;
        let array = value.as_array()?;
        let text = array[0].get("text")?.as_str()?;
        Some(text.to_string())
    }

    /// 获取数字
    pub fn get_number(&self, key: &str) -> Option<f64> {
        let value = self.fields.get(key)?;
        let number = value.as_f64()?;
        Some(number)
    }
}
