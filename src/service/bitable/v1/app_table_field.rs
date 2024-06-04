use serde::Deserialize;
use crate::core::config::Config;

pub struct AppTableFieldService {
    config: Config,
}

impl AppTableFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 字段的具体内容
#[derive(Debug, Deserialize)]
pub enum FieldValue {
    MultiLine(String),
    Barcode(String),
    Number(String),
    Currency(String),
    Progress(String),
    Rating(String),
    SingleOption(String),
    MultipleOption(Vec<String>),
    Date(u128),
    Checkbox(bool),
    Person(Person),
    Link(Link),
    Attachment(Attachment),
    OneWayLink(Vec<String>),
    TwoWayLink(Vec<String>),
    GroupChat(GroupChat),
    Location(Location),
    DateCreated(u128),
    LastModifiedDate(u128),
    CreatedBy(Person),
    ModifiedBy(Person),
    PhoneNumber(String),
    AutoSerial(String),
}

/// 人员
#[derive(Debug, Deserialize)]
pub struct Person {
    /// 人员名字
    pub name: String,
    /// ID
    pub id: String,
    /// 英文名字
    pub en_name: String,
    /// 邮箱
    pub email: String,
}

/// 群组
#[derive(Debug, Deserialize)]
pub struct GroupChat {
    /// 群组名
    pub name : String,
    /// 头像链接
    pub avatar_url: String,
    /// 群组id
    pub id: String
}

/// 超链接
#[derive(Debug, Deserialize)]
pub struct Link {
    /// 文本名称
    pub text: String,
    /// 超链接
    pub link: String,
}

/// 附件
#[derive(Debug, Deserialize)]
pub struct Attachment {
    /// 附件token
    pub file_token: String,
    /// 附件名称
    pub name: String,
    /// 附件的 mime 类型, 如: image/png
    pub r#type: String,
    /// 附件大小, 单位: 字节
    pub size: i32,
    /// 附件url
    pub url: String,
    /// 生成附件临时下载链接的url，需access token鉴权
    pub tmp_url: Option<String>,
}

/// 地理位置
#[derive(Debug, Deserialize)]
pub struct Location {
    /// 经纬度
    pub location: String,
    /// 省
    pub pname: String,
    /// 市
    pub cityname: String,
    /// 区
    pub adname: String,
    /// 详细地址
    pub address: String,
    /// 地名
    pub name: String,
    /// 完整地址
    pub full_address: String
}