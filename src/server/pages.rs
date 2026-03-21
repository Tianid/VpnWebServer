pub enum Page {
    Desktop,
    Mobile,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Page::Desktop => "resources/web_resources/html_pages/index_desktop.html",
            Page::Mobile  => "resources/web_resources/html_pages/index_mobile.html",
        }
    }
}
