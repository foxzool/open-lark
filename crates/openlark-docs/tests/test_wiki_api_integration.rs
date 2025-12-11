//! Wiki API 集成测试
//!
//! 测试标准化后的Wiki API是否能正常工作

#[cfg(feature = "wiki")]
use openlark_core::config::Config;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::v1::node::search::SearchWikiParams;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::v2::service::WikiService;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::v2::space::create::CreateWikiSpaceParams;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::v2::space::list::ListWikiSpacesParams;

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

    // 测试空间列表请求构建器
    let _request = wiki.list_spaces();

    // 测试参数结构
    let params = ListWikiSpacesParams {
        site_type: Some(1),
        page_size: Some(20),
        page_token: None,
    };

    assert_eq!(params.site_type, Some(1));
    assert_eq!(params.page_size, Some(20));
    assert!(params.page_token.is_none());
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_space_create_params() {
    let params = CreateWikiSpaceParams {
        name: "测试知识库".to_string(),
        description: Some("测试描述".to_string()),
        space_type: Some("private".to_string()),
        icon: Some("📚".to_string()),
        domain: Some("test-wiki".to_string()),
    };

    assert_eq!(params.name, "测试知识库");
    assert_eq!(params.description, Some("测试描述".to_string()));
    assert_eq!(params.space_type, Some("private".to_string()));
    assert_eq!(params.icon, Some("📚".to_string()));
    assert_eq!(params.domain, Some("test-wiki".to_string()));
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_v1_search_params() {
    let params = SearchWikiParams {
        query: "搜索关键词".to_string(),
        space_ids: Some(vec!["space1".to_string(), "space2".to_string()]),
        node_type: Some("doc".to_string()),
        page_size: Some(10),
        page_token: Some("token123".to_string()),
    };

    assert_eq!(params.query, "搜索关键词");
    assert_eq!(
        params.space_ids,
        Some(vec!["space1".to_string(), "space2".to_string()])
    );
    assert_eq!(params.node_type, Some("doc".to_string()));
    assert_eq!(params.page_size, Some(10));
    assert_eq!(params.page_token, Some("token123".to_string()));
}

#[cfg(feature = "wiki")]
#[test]
fn test_wiki_service_all_methods() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let wiki = WikiService::new(config);

    // 测试所有方法都能正确创建请求构建器
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
