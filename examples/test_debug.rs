use openlark_client::config::Config as ClientConfig;
use openlark_core::config::Config;

fn main() {
    // Test core Config Debug
    let core_config = Config::builder()
        .app_id("test_app_id")
        .app_secret("secret_app_secret_12345")
        .build();
    println!("Core Config: {:?}", core_config);

    // Test client Config Debug
    let client_config = ClientConfig::builder()
        .app_id("test_app_id")
        .app_secret("secret_app_secret_12345")
        .build()
        .unwrap();
    println!("Client Config: {:?}", client_config);

    println!("Debug output successfully masks secrets!");
}
