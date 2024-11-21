use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse},
    extract::{State},
};
use askama::Template;
use std::sync::Arc;
use std::ops::Deref;

use crate::appstate::AppState;
use crate::basetemplate::BaseTemplate;
use crate::helpers::filters;

// Root
#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub vault_list: Vec<&'a String>,
}
impl<'a> Deref for RootTemplate<'a> {
    type Target = BaseTemplate<'a>;
    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

pub async fn get(
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {

    let vault_list: Vec<&String> = state.vaults.keys().collect();

     let template = RootTemplate { 
        _parent: &BaseTemplate { 
            pagetitle: "Home",
            dark_mode: true,
        },
        vault_list,
    };
    let reply_html = template.render().unwrap();
    return (StatusCode::OK, Html(reply_html).into_response());
}