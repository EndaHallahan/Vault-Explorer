use askama::Template;

// Base
#[derive(Template)]
#[template(path = "pages/base.html")]
pub struct BaseTemplate<'a> {
    pub pagetitle: &'a str,
    pub dark_mode: bool,
}