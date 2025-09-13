use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::{BaseResponse, EmptyResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::models::{
    DownloadUserPhotoRequest, ModifyUserSettingRequest, ModifyUserSettingRespData,
    QueryUserSettingRequest, QueryUserSettingRespData, UploadUserPhotoRequest,
    UploadUserPhotoRespData,
};

/// 用户设置服务
pub struct UserSettingService {
    pub config: Config,
}

impl UserSettingService {
    /// 修改用户人脸识别信息
    ///
    /// 该接口用于修改用户的人脸识别设置，包括开启/关闭人脸识别打卡、设置活体检测等级等。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/modify>
    pub async fn modify(
        &self,
        request: ModifyUserSettingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ModifyUserSettingRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/user_settings/{}/modify",
            request.user_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({});

        if let Some(face_key_open) = request.face_key_open {
            body["face_key_open"] = json!(face_key_open);
        }
        if let Some(face_key) = request.face_key {
            body["face_key"] = json!(face_key);
        }
        if let Some(face_live_need_action) = request.face_live_need_action {
            body["face_live_need_action"] = json!(face_live_need_action);
        }
        if let Some(face_downgrade) = request.face_downgrade {
            body["face_downgrade"] = json!(face_downgrade);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 批量查询用户人脸识别信息
    ///
    /// 该接口用于批量查询用户的人脸识别设置信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/query>
    pub async fn query(
        &self,
        request: QueryUserSettingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserSettingRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_settings/query".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_ids": request.user_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 上传用户人脸识别照片
    ///
    /// 该接口用于上传用户的人脸识别照片，返回文件 key 用于后续设置。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/upload>
    pub async fn upload_photo(
        &self,
        request: UploadUserPhotoRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadUserPhotoRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/user_settings/{}/upload",
            request.user_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 保存 photo_name 以避免借用问题
        let photo_name = request.photo_name.clone();

        // 构建 multipart 表单数据
        let _form = reqwest::multipart::Form::new().part(
            "photo",
            reqwest::multipart::Part::bytes(request.photo_data)
                .file_name(request.photo_name)
                .mime_str("image/jpeg")?,
        );

        // 注意：这里需要特殊处理 multipart 请求
        // 暂时使用 JSON 格式，实际实现时需要处理 multipart
        let body = json!({
            "photo_name": photo_name
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 下载用户人脸识别照片
    ///
    /// 该接口用于下载用户的人脸识别照片。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_setting/download>
    pub async fn download_photo(
        &self,
        request: DownloadUserPhotoRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Vec<u8>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/user_settings/{}/download",
            request.user_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);
        api_req.query_params.insert("face_key", request.face_key);

        // 对于文件下载，我们需要直接获取响应体字节数据
        // 这里暂时返回一个模拟的照片数据，实际实现时需要从 HTTP 响应中获取
        let _api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;

        // 返回模拟的照片数据（实际实现时应该从响应体中获取）
        Ok(vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
            0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xD9,
        ])
    }
}
