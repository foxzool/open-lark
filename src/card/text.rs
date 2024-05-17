use serde::{Deserialize, Serialize};

/// 自定义字号
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomTextSize {
    ///  在无法差异化配置字号的旧版飞书客户端上，生效的字号属性。选填。
    pub default: String,
    /// 桌面端的字号。
    pub pc: String,
    /// 移动端的字号。
    pub mobile: String,
}
