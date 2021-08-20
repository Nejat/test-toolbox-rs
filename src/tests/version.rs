#[test]
fn readme_usage_version() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn readme_docs_link_version() {
    version_sync::assert_contains_regex!("README.md", "/test-toolbox/{version}/test_toolbox/");
}

#[test]
fn html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
