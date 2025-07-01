use std::env;

use open_lark::{
    prelude::*,
    service::ai::models::{
        FileRecognizeRequest, LanguageDetectRequest, SpeechFile, SpeechRecognizeRequest,
        TranslateRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = env::var("APP_ID")?;
    let app_secret = env::var("APP_SECRET")?;

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("开始演示 AI 能力接口...\n");

    // 1. 智能文档处理示例
    println!("1. 智能文档处理示例");
    demonstrate_document_ai(&client).await?;

    // 2. 光学字符识别示例
    println!("\n2. 光学字符识别示例");
    demonstrate_ocr(&client).await?;

    // 3. 语音识别示例
    println!("\n3. 语音识别示例");
    demonstrate_speech_recognition(&client).await?;

    // 4. 机器翻译示例
    println!("\n4. 机器翻译示例");
    demonstrate_translation(&client).await?;

    println!("\nAI 能力接口演示完成！");
    Ok(())
}

async fn demonstrate_document_ai(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("测试智能文档处理服务...");

    // 简历解析示例
    let resume_request = FileRecognizeRequest {
        file: "resume_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .parse_resume(resume_request, None)
        .await
    {
        Ok(response) => println!("✓ 简历解析成功: {:?}", response.data),
        Err(e) => println!("✗ 简历解析失败: {e:?}"),
    }

    // 身份证识别示例
    let id_card_request = FileRecognizeRequest {
        file: "id_card_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_id_card(id_card_request, None)
        .await
    {
        Ok(response) => println!("✓ 身份证识别成功: {:?}", response.data),
        Err(e) => println!("✗ 身份证识别失败: {e:?}"),
    }

    // 驾驶证识别示例
    let driving_license_request = FileRecognizeRequest {
        file: "driving_license_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_driving_license(driving_license_request, None)
        .await
    {
        Ok(response) => println!("✓ 驾驶证识别成功: {:?}", response.data),
        Err(e) => println!("✗ 驾驶证识别失败: {e:?}"),
    }

    // 银行卡识别示例
    let bank_card_request = FileRecognizeRequest {
        file: "bank_card_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_bank_card(bank_card_request, None)
        .await
    {
        Ok(response) => println!("✓ 银行卡识别成功: {:?}", response.data),
        Err(e) => println!("✗ 银行卡识别失败: {e:?}"),
    }

    // 营业执照识别示例
    let business_license_request = FileRecognizeRequest {
        file: "business_license_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_business_license(business_license_request, None)
        .await
    {
        Ok(response) => println!("✓ 营业执照识别成功: {:?}", response.data),
        Err(e) => println!("✗ 营业执照识别失败: {e:?}"),
    }

    // 增值税发票识别示例
    let vat_invoice_request = FileRecognizeRequest {
        file: "vat_invoice_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_vat_invoice(vat_invoice_request, None)
        .await
    {
        Ok(response) => println!("✓ 增值税发票识别成功: {:?}", response.data),
        Err(e) => println!("✗ 增值税发票识别失败: {e:?}"),
    }

    // 合同字段提取示例
    let contract_request = FileRecognizeRequest {
        file: "contract_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .extract_contract_fields(contract_request, None)
        .await
    {
        Ok(response) => println!("✓ 合同字段提取成功: {:?}", response.data),
        Err(e) => println!("✗ 合同字段提取失败: {e:?}"),
    }

    // 名片识别示例
    let business_card_request = FileRecognizeRequest {
        file: "business_card_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .document_ai
        .recognize_business_card(business_card_request, None)
        .await
    {
        Ok(response) => println!("✓ 名片识别成功: {:?}", response.data),
        Err(e) => println!("✗ 名片识别失败: {e:?}"),
    }

    Ok(())
}

async fn demonstrate_ocr(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("测试光学字符识别服务...");

    let ocr_request = FileRecognizeRequest {
        file: "image_file_token_or_base64".to_string(),
    };

    match client
        .ai
        .optical_char_recognition
        .basic_recognize(ocr_request, None)
        .await
    {
        Ok(response) => println!("✓ OCR文字识别成功: {:?}", response.data),
        Err(e) => println!("✗ OCR文字识别失败: {e:?}"),
    }

    Ok(())
}

async fn demonstrate_speech_recognition(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("测试语音识别服务...");

    // 语音文件识别示例
    let speech_request = SpeechRecognizeRequest {
        speech: SpeechFile {
            file_token: Some("speech_file_token".to_string()),
            content: None,
        },
        format: Some("wav".to_string()),
        sample_rate: Some(16000),
        language: Some("zh".to_string()),
    };

    match client
        .ai
        .speech_to_text
        .file_recognize(speech_request, None)
        .await
    {
        Ok(response) => println!("✓ 语音文件识别成功: {:?}", response.data),
        Err(e) => println!("✗ 语音文件识别失败: {e:?}"),
    }

    Ok(())
}

async fn demonstrate_translation(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("测试机器翻译服务...");

    // 语种检测示例
    let detect_request = LanguageDetectRequest {
        text: "Hello, how are you?".to_string(),
    };

    match client.ai.translation.detect(detect_request, None).await {
        Ok(response) => println!("✓ 语种检测成功: {:?}", response.data),
        Err(e) => println!("✗ 语种检测失败: {e:?}"),
    }

    // 文本翻译示例
    let translate_request = TranslateRequest {
        source_language: Some("en".to_string()),
        target_language: "zh".to_string(),
        text: "Hello, how are you?".to_string(),
    };

    match client
        .ai
        .translation
        .translate(translate_request, None)
        .await
    {
        Ok(response) => println!("✓ 文本翻译成功: {:?}", response.data),
        Err(e) => println!("✗ 文本翻译失败: {e:?}"),
    }

    Ok(())
}
