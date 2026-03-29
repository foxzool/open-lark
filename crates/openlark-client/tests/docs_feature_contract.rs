//! `openlark-client/docs` feature 的最小对外契约检查。
//!
//! 该 feature 至少需要保证 `client.docs.ccm` 可用。

#![cfg(feature = "docs")]

use openlark_client::Client;

#[test]
fn docs_feature_exposes_ccm_entry() {
    let client = Client::builder()
        .app_id("test_app")
        .app_secret("test_secret")
        .build()
        .expect("client should build with docs feature");

    assert_eq!(client.docs.ccm.config().app_id(), "test_app");
}
