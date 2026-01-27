//! 批量获取部门
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v1/department/batch_get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{BatchGetRequestBody, BatchGetResponse};

/// 批量获取部门请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID 列表（必填，最多 100 个）
    department_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_ids: Vec::new(),
        }
    }

    /// 设置部门 ID 列表（必填，最多 100 个）
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = department_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        if self.department_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "部门ID列表不能为空",
                "至少需要提供一个部门ID",
            ));
        }
        if self.department_ids.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "部门ID列表数量超限",
                "最多支持 100 个部门ID",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(&api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchGetRequestBody {
            department_ids: self.department_ids,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量获取部门响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
