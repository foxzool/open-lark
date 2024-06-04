use reqwest::Method;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

pub struct AppTableFieldService {
    config: Config,
}

impl AppTableFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn list(
        &self,
        request: ListAppTableFieldRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAppTableFieldResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields",
            app_token = request.app_token,
            table_id = request.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
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
    pub name: String,
    /// 头像链接
    pub avatar_url: String,
    /// 群组id
    pub id: String,
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
    pub full_address: String,
}

/// 列出字段请求
#[derive(Debug, Default)]
pub struct ListAppTableFieldRequest {
    api_request: ApiRequest,
    app_token: String,
    table_id: String,
}

impl ListAppTableFieldRequest {
    pub fn builder() -> ListAppTableFieldRequestBuilder {
        ListAppTableFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListAppTableFieldRequestBuilder {
    request: ListAppTableFieldRequest,
}

impl ListAppTableFieldRequestBuilder {
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 视图ID
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("view_id".to_string(), view_id.to_string());
        self
    }

    /// 控制字段描述（多行文本格式）数据的返回格式, true 表示以数组富文本形式返回
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request.api_request.query_params.insert(
            "text_field_as_array".to_string(),
            text_field_as_array.to_string(),
        );
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    ///
    /// 示例值：10
    ///
    /// 默认值：20
    ///
    /// 数据校验规则：
    ///
    /// 最大值：100
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_request
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    pub fn build(self) -> ListAppTableFieldRequest {
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct ListAppTableFieldResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: String,
    /// 总数
    pub total: i32,
    /// 字段信息
    pub items: Vec<AppTableField>,
}

impl ApiResponseTrait for ListAppTableFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 字段信息
#[derive(Debug, Deserialize)]
pub struct AppTableField {
    /// 多维表格字段名
    ///
    /// 请注意：
    ///
    /// 名称中的首尾空格将会被去除。
    pub field_name: String,
    /// 多维表格字段类型
    // 可选值有：
    /// - 1：多行文本
    /// - 2：数字
    /// - 3：单选
    /// - 4：多选
    /// - 5：日期
    /// - 7：复选框
    /// - 11：人员
    /// - 13：电话号码
    /// - 15：超链接
    /// - 17：附件
    /// - 18：关联
    /// - 20：公式
    /// - 21：双向关联
    /// - 22：地理位置
    /// - 23：群组
    /// - 1001：创建时间
    /// - 1002：最后更新时间
    /// - 1003：创建人
    /// - 1004：修改人
    /// - 1005：自动编号
    pub r#type: FieldType,
    /// 字段属性
    pub property: Option<AppTableFieldProperty>,
    /// 字段的描述
    pub description: Option<AppTableFieldDescription>,
    /// 是否是索引列
    pub is_primary: bool,
    /// 多维表格字段 id
    pub field_id: String,
    /// 字段在界面上的展示类型，例如进度字段是数字的一种展示形态
    ///
    /// 可选值有：
    ///
    /// - Text：多行文本
    /// - Barcode：条码
    /// - Number：数字
    /// - Progress：进度
    /// - Currency：货币
    /// - Rating：评分
    /// - SingleSelect：单选
    /// - MultiSelect：多选
    /// - DateTime：日期
    /// - Checkbox：复选框
    /// - User：人员
    /// - GroupChat：群组
    /// - Phone：电话号码
    /// - Url：超链接
    /// - Attachment：附件
    /// - SingleLink：单向关联
    /// - Formula：公式
    /// - DuplexLink：双向关联
    /// - Location：地理位置
    /// - CreatedTime：创建时间
    /// - ModifiedTime：最后更新时间
    /// - CreatedUser：创建人
    /// - ModifiedUser：修改人
    /// - AutoNumber：自动编号
    pub ui_type: UiType,
    /// 是否是隐藏字段
    pub is_hidden: Option<bool>,
}

// 字段属性
#[derive(Debug, Deserialize)]
pub struct AppTableFieldProperty {
    /// 单选、多选字段的选项信息
    pub options: Option<Vec<AppTableFieldPropertyOption>>,
    /// 数字、公式字段的显示格式
    pub formatter: Option<String>,
    /// 日期、创建时间、最后更新时间字段的显示格式
    pub date_formater: Option<String>,
    /// 日期字段中新纪录自动填写创建时间
    pub auto_fill: Option<bool>,
    /// 人员字段中允许添加多个成员，单向关联、双向关联中允许添加多个记录
    pub multiple: Option<bool>,
    /// 单向关联、双向关联字段中关联的数据表的id
    pub table_id: Option<String>,
    /// 单向关联、双向关联字段中关联的数据表的名字
    pub table_name: Option<String>,
    /// 双向关联字段中关联的数据表中对应的双向关联字段的名字
    pub back_field_name: Option<String>,
    /// 自动编号类型
    pub auto_serial: Option<AppTableFieldPropertyAutoSerial>,
    /// 地理位置输入方式
    pub location: Option<AppTableFieldPropertyLocation>,
    /// 公式字段的表达式
    pub formula_expression: Option<String>,
    /// 字段支持的编辑模式
    pub allowed_edit_modes: Option<AppTableFieldPropertyAllowedEditModes>,
    /// 进度、评分等字段的数据范围最小值
    pub min: Option<f32>,
    /// 进度、评分等字段的数据范围最大值
    pub max: Option<f32>,
    /// 进度等字段是否支持自定义范围
    pub range_customize: Option<bool>,
    /// 货币币种
    pub currency_code: Option<String>,
    /// 评分字段的相关设置
    pub rating: Option<AppTableFieldPropertyRating>,
}

/// 单选、多选字段的选项信息
#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyOption {
    /// 选项名
    pub name: String,
    /// 选项 ID，创建时不允许指定 ID
    pub id: String,
    /// 选项颜色
    pub color: i32,
}

/// 自动编号类型
#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyAutoSerial {
    /// 自动编号类型
    ///
    /// 可选值有：
    ///
    /// - custom：自定义编号
    /// - auto_increment_number：自增数字
    pub r#type: String,
    pub options: Vec<AppTableFieldPropertyAutoSerialOption>,
}

/// 自动编号规则列表
#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyAutoSerialOption {
    /// 自动编号的可选规则项类型
    ///
    /// 可选值有：
    ///
    /// system_number：自增数字位,value范围1-9
    /// fixed_text：固定字符，最大长度：20
    /// created_time：创建时间，支持格式 "yyyyMMdd"、"yyyyMM"、"yyyy"、"MMdd"、"MM"、"dd"
    pub r#type: String,
    /// 与自动编号的可选规则项类型相对应的取值
    pub value: String,
}

/// 地理位置输入方式

#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyLocation {
    /// 地理位置输入限制
    ///
    /// 可选值有：
    ///
    /// - only_mobile：只允许移动端上传
    /// - not_limit：无限制
    pub input_type: String,
}

/// 字段支持的编辑模式
#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyAllowedEditModes {
    /// 是否允许手动录入
    pub manual: bool,
    /// 是否允许移动端录入
    pub scan: bool,
}

/// 评分字段的相关设置
#[derive(Debug, Deserialize)]
pub struct AppTableFieldPropertyRating {
    /// 评分字段的符号展示
    pub symbol: String,
}

/// 字段的描述
#[derive(Debug, Deserialize)]
pub struct AppTableFieldDescription {
    /// 是否禁止同步，如果为true，表示禁止同步该描述内容到表单的问题描述注意：
    /// 该字段在列出字段时不生效。只在新增、修改字段时生效。
    pub disable_sync: bool,
    /// 字段描述内容
    pub text: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum UiType {
    Text,
    /// 多行文本
    Barcode,
    /// 条码
    Number,
    /// 数字
    Progress,
    /// 进度
    Currency,
    /// 货币
    Rating,
    /// 评分
    SingleSelect,
    /// 单选
    MultiSelect,
    /// 多选
    DateTime,
    /// 日期
    Checkbox,
    /// 复选框
    User,
    /// 人员
    GroupChat,
    /// 群组
    Phone,
    /// 电话号码
    Url,
    /// 超链接
    Attachment,
    /// 附件
    SingleLink,
    /// 单向关联
    Formula,
    /// 公式
    DuplexLink,
    /// 双向关联
    Location,
    /// 地理位置
    CreatedTime,
    /// 创建时间
    ModifiedTime,
    /// 最后更新时间
    CreatedUser,
    /// 创建人
    ModifiedUser,
    /// 修改人
    /// 自动编号
    AutoNumber,
}

#[derive(Debug, Deserialize_repr, PartialEq)]
#[repr(u16)]
pub enum FieldType {
    /// - 1：多行文本
    Text = 1,
    /// - 2：数字
    Number = 2,
    /// - 3：单选
    SingleSelect = 3,
    /// - 4：多选
    MultiSelect = 4,
    /// - 5：日期
    DateTime = 5,
    /// - 7：复选框
    Checkbox = 7,
    /// - 11：人员
    User = 11,
    /// - 13：电话号码
    PhoneNumber = 13,
    /// - 15：超链接
    Url = 15,
    /// - 17：附件
    Attachment = 17,
    /// - 18：关联
    Link = 18,
    /// - 20：公式
    Formula = 20,
    /// - 21：双向关联
    DuplexLink = 21,
    /// - 22：地理位置
    Location = 22,
    /// - 23：群组
    GroupChat = 23,
    /// - 1001：创建时间
    CreatedTime = 1001,
    /// - 1002：最后更新时间
    ModifiedTime = 1002,
    /// - 1003：创建人
    CreatedUser = 1003,
    /// - 1004：修改人
    ModifiedUser = 1004,
    /// - 1005：自动编号
    AutoSerial = 1005,
}
