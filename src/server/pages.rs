pub enum Page {
    Index,
    // Add new variants here as new HTML pages are created
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Page::Index => "resources/web_resources/html_pages/index.html",
        }
    }
}
