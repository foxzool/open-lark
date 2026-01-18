//! Wiki API 集成测试
//!
//! 测试标准化后的 Wiki API 是否能正常工作。

#[cfg(feature = "wiki")]
use openlark_core::config::Config;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::v2::WikiService;

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_service_creation() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);
    assert_eq!(wiki.config().app_id(), "test_app_id");
    assert_eq!(wiki.config().app_secret(), "test_app_secret");
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_space_list_request_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);

    let _request = wiki.list_spaces().page_size(20);
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_space_create_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);

    let _request = wiki
        .create_space()
        .name("测试知识库")
        .description("测试描述");
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_v1_search_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);

    let _request = wiki
        .search_wiki()
        .query("搜索关键词")
        .space_id("space1")
        .page_size(10)
        .page_token("token123");
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_service_all_methods() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);

    let _list_request = wiki.list_spaces();
    let _get_request = wiki.get_space("test_space_id");
    let _create_request = wiki.create_space();
    let _update_request = wiki.update_space_setting("test_space_id");
    let _get_node_request = wiki.get_space_node();
    let _list_members_request = wiki.list_space_members("test_space_id");
    let _create_member_request = wiki.create_space_member("test_space_id");
    let _delete_member_request = wiki.delete_space_member("test_space_id");
    let _list_nodes_request = wiki.list_space_nodes("test_space_id");
    let _create_node_request = wiki.create_space_node("test_space_id");
    let _move_node_request = wiki.move_space_node("test_space_id", "node_token");
    let _copy_node_request = wiki.copy_space_node("test_space_id", "node_token");
    let _update_title_request = wiki.update_space_node_title("test_space_id", "node_token");
    let _move_docs_request = wiki.move_docs_to_wiki("test_space_id");
    let _get_task_request = wiki.get_task("task_id");
    let _search_request = wiki.search_wiki();
}
