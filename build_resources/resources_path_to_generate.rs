pub fn create_resource_paths() -> Vec<(String, String)> {
    // Directory paths
    let resources_path = "resources".to_string();
    let web_resources_path = format!("{}/web_resources", resources_path);
    let html_pages_path = format!("{}/html_pages", web_resources_path);
    let page_scripts_path = format!("{}/page_scripts", web_resources_path);

    // File paths
    let html_index_page_path = format!("{}/index.html", html_pages_path);
    let web_socket_js_path = format!("{}/WebSocket.js", page_scripts_path);

    vec![
        create_pair("RESOURCES_ROOT_PATH", resources_path),
        create_pair("WEB_RESOURCES_PATH", web_resources_path),
        create_pair("HTML_PAGES_PATH", html_pages_path),
        create_pair("PAGE_SCRIPTS_PATH", page_scripts_path),

        create_pair("HTML_INDEX_PAGE_PATH", html_index_page_path),
        create_pair("WEB_SOCKET_JS_PATH", web_socket_js_path),
    ]
}

fn create_pair(str1: &str, str2: String) -> (String, String) { (str1.to_string(), str2) }
