use open_lark::{
    prelude::*,
    service::human_authentication::{
        models::{AuthenticationStatus, CropParameters, IdType, ImageType},
        FaceImageCropRequest, FaceImageUploadRequest, IdentityCreateRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 实名认证服务示例 ===\n");

    // 1. 录入身份信息
    println!("1. 录入身份信息");
    let identity_request = IdentityCreateRequest {
        name: "张三".to_string(),
        id_number: "110101199001010001".to_string(),
        id_type: Some(IdType::IdCard),
    };

    match client
        .human_authentication
        .create_identity(identity_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("✅ 成功录入身份信息: ID={}", data.identity_id);
                println!("   创建时间: {}", data.created_at);

                let identity_id = data.identity_id;

                // 2. 上传人脸基准图片
                println!("\n2. 上传人脸基准图片");

                // 注意：这里使用示例的 base64 数据，实际使用时应该是真实的图片数据
                let sample_image_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

                let upload_request = FaceImageUploadRequest {
                    identity_id: identity_id.clone(),
                    image_content: sample_image_base64.to_string(),
                    image_type: ImageType::Png,
                };

                match client
                    .human_authentication
                    .upload_face_image(upload_request, None)
                    .await
                {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            println!("✅ 成功上传人脸图片: ID={}", data.image_id);
                            println!("   上传时间: {}", data.uploaded_at);

                            let image_id = data.image_id;

                            // 3. 裁剪人脸图片
                            println!("\n3. 裁剪人脸图片");
                            let crop_request = FaceImageCropRequest {
                                image_id: image_id.clone(),
                                crop_params: Some(CropParameters {
                                    x: 100,
                                    y: 100,
                                    width: 200,
                                    height: 200,
                                }),
                            };

                            match client
                                .human_authentication
                                .crop_face_image(crop_request, None)
                                .await
                            {
                                Ok(response) => {
                                    if let Some(data) = response.data {
                                        println!(
                                            "✅ 成功裁剪人脸图片: ID={}",
                                            data.cropped_image_id
                                        );
                                        println!("   裁剪时间: {}", data.cropped_at);

                                        if let Some(face) = data.detected_face {
                                            println!(
                                                "   检测到人脸区域: ({}, {}) {}x{}",
                                                face.x, face.y, face.width, face.height
                                            );
                                            println!("   检测置信度: {:.2}", face.confidence);
                                        }
                                    }
                                }
                                Err(e) => println!("❌ 裁剪人脸图片失败: {e:?}"),
                            }

                            // 4. 查询认证结果
                            println!("\n4. 查询人脸认证结果");
                            match client
                                .human_authentication
                                .query_authentication_result(&identity_id, None)
                                .await
                            {
                                Ok(response) => {
                                    if let Some(data) = response.data {
                                        println!("✅ 查询认证结果成功:");
                                        println!("   身份记录ID: {}", data.identity_id);
                                        println!("   认证状态: {:?}", data.status);

                                        match data.status {
                                            AuthenticationStatus::Success => {
                                                if let Some(score) = data.confidence_score {
                                                    println!("   置信度分数: {score:.2}");
                                                }
                                                if let Some(completed_at) = data.completed_at {
                                                    println!("   完成时间: {completed_at}");
                                                }
                                            }
                                            AuthenticationStatus::Failed => {
                                                if let Some(error) = data.error_message {
                                                    println!("   失败原因: {error}");
                                                }
                                            }
                                            AuthenticationStatus::Processing => {
                                                println!("   正在处理中，请稍后再次查询");
                                            }
                                            AuthenticationStatus::Pending => {
                                                println!("   等待处理中");
                                            }
                                            AuthenticationStatus::Timeout => {
                                                println!("   处理超时");
                                            }
                                        }
                                    }
                                }
                                Err(e) => println!("❌ 查询认证结果失败: {e:?}"),
                            }
                        }
                    }
                    Err(e) => println!("❌ 上传人脸图片失败: {e:?}"),
                }
            }
        }
        Err(e) => println!("❌ 录入身份信息失败: {e:?}"),
    }

    println!("\n=== 实名认证服务示例完成 ===");
    println!("注意:");
    println!("1. 本示例使用测试数据，实际使用时请替换为真实的身份信息和图片");
    println!("2. 请确保遵循相关法律法规和隐私保护要求");
    println!("3. 敏感数据传输和存储需要采取适当的安全措施");

    Ok(())
}
