#![allow(dead_code)]
#![allow(dead_code)]
#![allow(dead_code)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
/// 字段的具体内容,
#[derive(Clone)]
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
    AutoSerial(String)}
/// 人员,
#[derive(Clone)]
pub struct Person {
    /// 人员名字
    pub name: String,
    /// ID
    pub id: String,
    /// 英文名字
    pub en_name: String,
    /// 邮箱
    pub email: String,
/// 群组,
#[derive(Clone)]
pub struct GroupChat {
/// 群组名,
    pub name: String,
    /// 头像链接
    pub avatar_url: String,
    /// 群组id
    pub id: String,
/// 超链接,
#[derive(Clone)]
pub struct Link {
    /// 文本名称
    pub text: String,
    /// 超链接
    pub link: String,
/// 附件,
#[derive(Clone)]
pub struct Attachment {
/// 附件token,
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
    pub tmp_url: Option<String>}
/// 地理位置,
#[derive(Clone)]
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
    pub full_address: String,
/// 字段信息,
#[derive(Clone)]
pub struct AppTableField {
    /// 多维表格字段名
    pub field_name: String,
    /// 多维表格字段类型
    pub r#type: FieldType,
    /// 字段属性
    pub property: Option<AppTableFieldProperty>,
    /// 字段的描述
    pub description: Option<AppTableFieldDescription>,
    /// 是否是索引列
    pub is_primary: bool,
    /// 多维表格字段 id
    pub field_id: String,
    /// 字段在界面上的展示类型
    pub ui_type: UiType,
    /// 是否是隐藏字段
    pub is_hidden: Option<bool>}
// 字段属性,
#[derive(Clone)]
pub struct AppTableFieldProperty {
    /// 单选、多选字段的选项信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<AppTableFieldPropertyOption>>,
    /// 数字、公式字段的显示格式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<String>,
    /// 日期、创建时间、最后更新时间字段的显示格式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<String>,
    /// 日期字段中新纪录自动填写创建时间,
#[serde(skip_serializing_if = "Option::is_none")]
    pub auto_fill: Option<bool>,
    /// 人员字段中允许添加多个成员，单向关联、双向关联中允许添加多个记录,
#[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    /// 单向关联、双向关联字段中关联的数据表的id,
#[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 单向关联、双向关联字段中关联的数据表的名字,
#[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 双向关联字段中关联的数据表中对应的双向关联字段的名字,
#[serde(skip_serializing_if = "Option::is_none")]
    pub back_field_name: Option<String>,
    /// 自动编号类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub auto_serial: Option<AppTableFieldPropertyAutoSerial>,
    /// 地理位置输入方式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AppTableFieldPropertyLocation>,
    /// 公式字段的表达式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub formula_expression: Option<String>,
    /// 字段支持的编辑模式,
#[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_edit_modes: Option<AppTableFieldPropertyAllowedEditModes>,
    /// 进度、评分等字段的数据范围最小值,
#[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f32>,
    /// 进度、评分等字段的数据范围最大值,
#[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f32>,
    /// 进度等字段是否支持自定义范围,
#[serde(skip_serializing_if = "Option::is_none")]
    pub range_customize: Option<bool>,
    /// 货币币种,
#[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// 评分字段的相关设置,
#[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<AppTableFieldPropertyRating>}
/// 单选、多选字段的选项信息,
#[derive(Clone)]
pub struct AppTableFieldPropertyOption {
    /// 选项名
    pub name: String,
    /// 选项 ID，创建时不允许指定 ID,
#[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 选项颜色,
#[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>}
/// 自动编号类型,
#[derive(Clone)]
pub struct AppTableFieldPropertyAutoSerial {
    /// 自动编号类型: custom 或 auto_increment_number
    pub r#type: String,
    pub options: Vec<AppTableFieldPropertyAutoSerialOption>}
/// 自动编号规则列表,
#[derive(Clone)]
pub struct AppTableFieldPropertyAutoSerialOption {
    /// 自动编号的可选规则项类型
    pub r#type: String,
    /// 与自动编号的可选规则项类型相对应的取值
    pub value: String,
/// 地理位置输入方式,
#[derive(Clone)]
pub struct AppTableFieldPropertyLocation {
    /// 地理位置输入限制: only_mobile 或 not_limit
    pub input_type: String,
/// 字段支持的编辑模式,
#[derive(Clone)]
pub struct AppTableFieldPropertyAllowedEditModes {
    /// 是否允许手动录入
    pub manual: bool,
    /// 是否允许移动端录入
    pub scan: bool,
/// 评分字段的相关设置,
#[derive(Clone)]
pub struct AppTableFieldPropertyRating {
    /// 评分字段的符号展示
    pub symbol: String,
/// 字段的描述,
#[derive(Clone)]
pub struct AppTableFieldDescription {
    /// 是否禁止同步,
#[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sync: Option<bool>,
    /// 字段描述内容
    pub text: String,

#[derive(Clone)]
pub enum UiType {
    Text,
    Barcode,
    Number,
    Progress,
    Currency,
    Rating,
    SingleSelect,
    MultiSelect,
    DateTime,
    Checkbox,
    User,
    GroupChat,
    Phone,
    Url,
    Attachment,
    SingleLink,
    Formula,
    DuplexLink,
    Location,
    CreatedTime,
    ModifiedTime,
    CreatedUser,
    ModifiedUser,
    AutoNumber,
    Stage,
    Lookup,
    Button}

#[derive(Clone)]
#[repr(u16)]
pub enum FieldType {
#[default]
    /// 多行文本
    Text = 1,
    /// 数字
    Number = 2,
    /// 单选
    SingleSelect = 3,
    /// 多选
    MultiSelect = 4,
    /// 日期
    DateTime = 5,
    /// 复选框
    Checkbox = 7,
    /// 人员
    User = 11,
    /// 电话号码
    PhoneNumber = 13,
    /// 超链接
    Url = 15,
    /// 附件
    Attachment = 17,
    /// 关联
    Link = 18,
    /// 查找引用
    Lookup = 19,
    /// 公式
    Formula = 20,
    /// 双向关联
    DuplexLink = 21,
    /// 地理位置
    Location = 22,
    /// 群组
    GroupChat = 23,
    /// 流程
    Stage = 24,
    /// 创建时间
    CreatedTime = 1001,
    /// 最后更新时间
    ModifiedTime = 1002,
    /// 创建人
    CreatedUser = 1003,
    /// 修改人
    ModifiedUser = 1004,
    /// 自动编号
    AutoSerial = 1005,
    /// 按钮
    Button = 3001}
impl AppTableFieldProperty {
    
}
/// 创建数字字段属性,
    pub fn number(formatter: Option<String>) -> Self {
Self {
            formatter,
            ..Self::text()}
    }
/// 创建单选字段属性,
    pub fn single_select(options: Vec<AppTableFieldPropertyOption>) -> Self {
Self {
            options: Some(options),
            ..Self::text()}
    }
/// 创建多选字段属性,
    pub fn multi_select(options: Vec<AppTableFieldPropertyOption>) -> Self {
Self {
            options: Some(options),
            ..Self::text()}
    }
/// 创建日期字段属性,
    pub fn date(date_formatter: Option<String>, auto_fill: Option<bool>) -> Self {
Self {
            date_formatter,
            auto_fill,
            ..Self::text()}
    }
/// 创建人员字段属性,
    pub fn user(multiple: Option<bool>) -> Self {
Self {
            multiple,
            ..Self::text()}
    }
/// 创建进度字段属性,
    pub fn progress(min: f32, max: f32, range_customize: bool) -> Self {
Self {
            min: Some(min),
            max: Some(max),
            range_customize: Some(range_customize),
            formatter: Some("0".to_string()),
            ..Self::text()}
    }
/// 创建货币字段属性,
    pub fn currency(currency_code: String) -> Self {
Self {
            currency_code: Some(currency_code),
            formatter: Some("0.00".to_string()),
            ..Self::text()}
    }
/// 创建评分字段属性,
    pub fn rating(symbol: String, max: f32) -> Self {
Self {
            rating: Some(AppTableFieldPropertyRating { symbol }),
            max: Some(max),
            ..Self::text(),
}
}
