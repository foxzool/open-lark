use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::service::bitable::v1::Record;

pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增记录
    pub async fn create(
        &self,
        request: CreateAppTableRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppTableRecordResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records",
            app_token = request.app_token,
            table_id = request.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 新增多条记录
    pub async fn batch_create(
        &self,
        request: BatchCreateAppTableRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateAppTableRecordResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create",
            app_token = request.app_token,
            table_id = request.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 查询记录
    pub async fn search(
        &self,
        request: SearchAppTableRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchAppTableRecordResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search",
            app_token = request.app_token,
            table_id = request.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新记录
    pub async fn update(
        &self,
        request: UpdateAppTableRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAppTableRecordResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}",
            app_token = request.app_token,
            table_id = request.table_id,
            record_id = request.record_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 新增记录
#[derive(Debug, Serialize, Default)]
pub struct CreateAppTableRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    ///
    /// 示例值："open_id"
    ///
    /// 可选值有：
    ///
    /// open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。了解更多：如何获取 Open ID
    /// union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// 示例值："fe599b60-450f-46ff-b2ef-9f6675625b97"
    #[serde(skip)]
    client_token: Option<String>,
    /// 是否忽略一致性读写检查，默认为 false，即在进行读写操作时，系统将确保读取到的数据和写入的数据是一致的。可选值：
    ///
    /// * true：忽略读写一致性检查，提高性能，但可能会导致某些节点的数据不同步，出现暂时不一致
    /// * false：开启读写一致性检查，确保数据在读写过程中一致
    #[serde(skip)]
    ignore_consistency_check: Option<bool>,
    ///
    /// 要新增的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。
    ///
    /// 注意：
    ///
    /// 该接口支持的字段类型及其描述如下所示：
    ///
    /// 文本：原值展示，不支持 markdown 语法
    /// 数字：填写数字格式的值
    /// 单选：填写选项值，对于新的选项值，将会创建一个新的选项
    /// 多选：填写多个选项值，对于新的选项值，将会创建一个新的选项。如果填写多个相同的新选项值，将会创建多个相同的选项
    /// 日期：填写毫秒级时间戳
    /// 复选框：填写 true 或 false
    /// 条码
    /// 人员：填写用户的open_id、union_id 或 user_id，类型需要与 user_id_type 指定的类型一致
    /// 电话号码：填写文本内容
    /// 超链接：参考以下示例，text 为文本值，link 为 URL 链接
    /// 附件：填写附件 token，需要先调用上传素材或分片上传素材接口将附件上传至该多维表格中
    /// 单向关联：填写被关联表的记录 ID
    /// 双向关联：填写被关联表的记录 ID
    /// 地理位置：填写经纬度坐标
    /// 不同类型字段的数据结构请参考多维表格记录数据结构。
    ///
    /// 示例值：{"人员": [{"id": "ou_2910013f1e6456f16a0ce75ede9abcef"}]
    ///
    #[serde(flatten)]
    fields: Record,
}

impl CreateAppTableRecordRequest {
    pub fn builder() -> AppTableRecordCreateRequestBuilder {
        AppTableRecordCreateRequestBuilder::default()
    }
}

impl CreateAppTableRecordRequest {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            ..Default::default()
        }
    }

    pub async fn create(
        &self,
        config: &Config,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppTableRecordResponse>> {
        let mut api_req = self.api_request.clone();
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records",
            app_token = self.app_token,
            table_id = self.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Default)]
pub struct AppTableRecordCreateRequestBuilder {
    request: CreateAppTableRecordRequest,
}

impl AppTableRecordCreateRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表ID
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 用户 ID 类型
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// 示例值："fe599b60-450f-46ff-b2ef-9f667562
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 是否忽略一致性读写检查，默认为 false，即在进行读写操作时，系统将确保读取到的数据和写入的数据是一致的。可选值：
    /// * true：忽略读写一致性检查，提高性能，但可能会导致某些节点的数据不同步，出现暂时不一致
    /// * false：开启读写一致性检查，确保数据在读写过程中一致
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.request.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    /// 要新增的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。
    /// 注意：
    /// 该接口支持的字段类型及其描述如下所示：
    /// 文本：原值展示，不支持 markdown 语法
    /// 数字：填写数字格式的值
    /// 单选：填写选项值，对于新的选项值，将会创建一个新的选项
    /// 多选：填写多个选项值，对于新的选项值，将会创建一个新的选项。如果填写多个相同的新选项值，将会创建多个相同的选项
    /// 日期：填写毫秒级时间戳
    /// 复选框：填写 true 或 false
    /// 条码
    /// 人员：填写用户的open_id、union_id 或 user_id，类型需要与 user_id_type 指定的类型一致
    /// 电话号码：填写文本内容
    /// 超链接：参考以下示例，text 为文本值，link 为 URL 链接
    /// 附件：填写附件 token，需要先调用上传素材或分片上传素材接口将附件上传至该多维表格中
    /// 单向关联：填写被关联表的记录 ID
    /// 双向关联：填写被关联表的记录 ID
    /// 地理位置：填写经纬度坐标
    /// 不同类型字段的数据结构请参考多维表格记录数据结构。
    /// 示例值：{"人员": [{"id": "ou_2910013f1e6456f16a0ce75ede9abcef"}]
    pub fn fields(mut self, fields: Record) -> Self {
        self.request.fields = fields;
        self
    }

    /// 添加记录
    pub fn add_field(mut self, field_name: impl ToString, value: Value) -> Self {
        self.request
            .fields
            .fields
            .insert(field_name.to_string(), value);
        self
    }

    pub fn build(mut self) -> CreateAppTableRecordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAppTableRecordResponse {
    pub record: Record,
}

impl ApiResponseTrait for CreateAppTableRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增多条记录
#[derive(Debug, Serialize, Default)]
pub struct BatchCreateAppTableRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    ///
    /// 示例值："open_id"
    ///
    /// 可选值有：
    ///
    /// open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。了解更多：如何获取 Open ID
    /// union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// 示例值："fe599b60-450f-46ff-b2ef-9f6675625b97"
    #[serde(skip)]
    client_token: Option<String>,
    /// 是否忽略一致性读写检查，默认为 false，即在进行读写操作时，系统将确保读取到的数据和写入的数据是一致的。可选值：
    ///
    /// * true：忽略读写一致性检查，提高性能，但可能会导致某些节点的数据不同步，出现暂时不一致
    /// * false：开启读写一致性检查，确保数据在读写过程中一致
    #[serde(skip)]
    ignore_consistency_check: Option<bool>,
    ///
    /// 要新增的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。
    ///
    /// 注意：
    ///
    /// 该接口支持的字段类型及其描述如下所示：
    ///
    /// 文本：原值展示，不支持 markdown 语法
    /// 数字：填写数字格式的值
    /// 单选：填写选项值，对于新的选项值，将会创建一个新的选项
    /// 多选：填写多个选项值，对于新的选项值，将会创建一个新的选项。如果填写多个相同的新选项值，将会创建多个相同的选项
    /// 日期：填写毫秒级时间戳
    /// 复选框：填写 true 或 false
    /// 条码
    /// 人员：填写用户的open_id、union_id 或 user_id，类型需要与 user_id_type 指定的类型一致
    /// 电话号码：填写文本内容
    /// 超链接：参考以下示例，text 为文本值，link 为 URL 链接
    /// 附件：填写附件 token，需要先调用上传素材或分片上传素材接口将附件上传至该多维表格中
    /// 单向关联：填写被关联表的记录 ID
    /// 双向关联：填写被关联表的记录 ID
    /// 地理位置：填写经纬度坐标
    /// 不同类型字段的数据结构请参考多维表格记录数据结构。
    ///
    /// 示例值：{"人员": [{"id": "ou_2910013f1e6456f16a0ce75ede9abcef"}]
    ///
    records: Vec<Record>,
}

impl BatchCreateAppTableRecordRequest {
    pub fn builder() -> AppTableRecordBatchCreateRequestBuilder {
        AppTableRecordBatchCreateRequestBuilder::default()
    }
}

impl BatchCreateAppTableRecordRequest {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            ..Default::default()
        }
    }

    pub async fn create(
        &self,
        config: &Config,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateAppTableRecordResponse>> {
        let mut api_req = self.api_request.clone();
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records",
            app_token = self.app_token,
            table_id = self.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Default)]
pub struct AppTableRecordBatchCreateRequestBuilder {
    request: BatchCreateAppTableRecordRequest,
}

impl AppTableRecordBatchCreateRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表ID
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 用户 ID 类型
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// 示例值："fe599b60-450f-46ff-b2ef-9f667562
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 是否忽略一致性读写检查，默认为 false，即在进行读写操作时，系统将确保读取到的数据和写入的数据是一致的。可选值：
    /// * true：忽略读写一致性检查，提高性能，但可能会导致某些节点的数据不同步，出现暂时不一致
    /// * false：开启读写一致性检查，确保数据在读写过程中一致
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.request.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    /// 要新增的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。
    /// 注意：
    /// 该接口支持的字段类型及其描述如下所示：
    /// 文本：原值展示，不支持 markdown 语法
    /// 数字：填写数字格式的值
    /// 单选：填写选项值，对于新的选项值，将会创建一个新的选项
    /// 多选：填写多个选项值，对于新的选项值，将会创建一个新的选项。如果填写多个相同的新选项值，将会创建多个相同的选项
    /// 日期：填写毫秒级时间戳
    /// 复选框：填写 true 或 false
    /// 条码
    /// 人员：填写用户的open_id、union_id 或 user_id，类型需要与 user_id_type 指定的类型一致
    /// 电话号码：填写文本内容
    /// 超链接：参考以下示例，text 为文本值，link 为 URL 链接
    /// 附件：填写附件 token，需要先调用上传素材或分片上传素材接口将附件上传至该多维表格中
    /// 单向关联：填写被关联表的记录 ID
    /// 双向关联：填写被关联表的记录 ID
    /// 地理位置：填写经纬度坐标
    /// 不同类型字段的数据结构请参考多维表格记录数据结构。
    /// 示例值：{"人员": [{"id": "ou_2910013f1e6456f16a0ce75ede9abcef"}]
    pub fn records(mut self, fields: Vec<Record>) -> Self {
        self.request.records = fields;
        self
    }

    pub fn build(mut self) -> BatchCreateAppTableRecordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct BatchCreateAppTableRecordResponse {
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchCreateAppTableRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Default)]
pub struct SearchAppTableRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 表ID
    table_id: String,
    /// 视图的唯一标识符，获取指定视图下的记录view_id 参数说明
    ///
    /// 注意：当 filter 参数 或 sort
    /// 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的view_id 会被忽略。
    ///
    /// 示例值："vewqhz51lk"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 50 字符
    view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    ///
    /// 示例值：["字段1","字段2"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 200
    field_names: Option<Vec<String>>,
    /// 排序条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 100
    sort: Option<SearchSort>,
    /// 筛选条件
    filter: Option<SearchFilterInfo>,
    /// 控制是否返回自动计算的字段, true 表示返回
    ///
    /// 示例值：false
    automatic: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct SearchSort {
    /// 字段名称
    ///
    /// 示例值："多行文本"
    ///
    /// 数据校验规则：
    /// - 长度范围：0 字符 ～ 1000 字符
    pub field_name: Option<String>,
    /// 是否倒序排序
    ///
    /// 默认值：false
    pub desc: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct SearchFilterInfo {
    /// 条件逻辑连接词
    ///
    /// 示例值："and"
    ///
    /// 可选值有：
    ///
    /// and：满足全部条件
    /// or：满足任一条件
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 10 字符
    pub conjunction: Option<String>,
    /// 筛选条件集合
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 50
    pub conditions: Option<Vec<SearchCondition>>,
}

/// 筛选条件
#[derive(Debug, Serialize, Default)]
pub struct SearchCondition {
    /// 筛选条件的左值，值为字段的名称
    ///
    /// 示例值："字段1"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 1000 字符
    pub field_name: String,
    /// 条件运算符
    ///
    /// 示例值："is"
    ///
    /// 可选值有：
    ///
    /// is：等于
    /// isNot：不等于
    /// contains：包含
    /// doesNotContain：不包含
    /// isEmpty：为空
    /// isNotEmpty：不为空
    /// isGreater：大于
    /// isGreaterEqual：大于等于
    /// isLess：小于
    /// isLessEqual：小于等于
    /// like：LIKE 运算符。暂未支持
    /// in：IN 运算符。暂未支持
    pub operator: String,
    /// 目标值
    ///
    /// 目标值填写指南
    ///
    /// 示例值：["文本内容"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 10
    pub value: Option<Vec<String>>,
}

impl SearchAppTableRecordRequest {
    pub fn builder() -> AppTableRecordSearchRequestBuilder {
        AppTableRecordSearchRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AppTableRecordSearchRequestBuilder {
    request: SearchAppTableRecordRequest,
}

impl AppTableRecordSearchRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表ID
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 用户 ID 类型
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("user_id_type".to_string(), user_id_type.to_string());
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
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_request
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    /// 视图的唯一标识符，获取指定视图下的记录view_id 参数说明
    ///
    /// 注意：当 filter 参数 或 sort
    /// 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的view_id 会被忽略。
    ///
    /// 示例值："vewqhz51lk"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 50 字符
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = Some(view_id.to_string());
        self
    }

    /// 字段名称，用于指定本次查询返回记录中包含的字段
    ///
    /// 示例值：["字段1","字段2"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 200
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    /// 排序条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 100
    pub fn sort(mut self, sort: SearchSort) -> Self {
        self.request.sort = Some(sort);
        self
    }

    /// 筛选条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 50
    pub fn filter(mut self, filter: SearchFilterInfo) -> Self {
        self.request.filter = Some(filter);
        self
    }

    /// 控制是否返回自动计算的字段, true 表示返回
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(mut self) -> SearchAppTableRecordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchAppTableRecordResponse {
    pub items: Vec<Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for SearchAppTableRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateAppTableRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符
    #[serde(skip)]
    record_id: String,
    /// 用户 ID 类型
    ///
    /// 示例值："open_id"
    ///
    /// 可选值有：
    ///
    /// open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。了解更多：如何获取 Open ID
    /// union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。了解更多：如何获取 User ID？
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// 示例值："fe599b60-450f-46ff-b2ef-9f6675625b97"
    #[serde(skip)]
    client_token: Option<String>,

    /// 要更新的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。
    fields: Value,
}

impl UpdateAppTableRecordRequest {
    pub fn builder() -> AppTableRecordUpdateRequestBuilder {
        AppTableRecordUpdateRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AppTableRecordUpdateRequestBuilder {
    request: UpdateAppTableRecordRequest,
}

impl AppTableRecordUpdateRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表ID
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 记录的唯一标识符
    pub fn record_id(mut self, record_id: impl ToString) -> Self {
        self.request.record_id = record_id.to_string();
        self
    }

    /// fields
    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    /// build
    pub fn build(mut self) -> UpdateAppTableRecordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppTableRecordResponse {
    pub record: Value,
}

impl ApiResponseTrait for UpdateAppTableRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
