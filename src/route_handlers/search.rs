use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse},
    extract::{State, Query,},
};
use askama::Template;
use std::sync::Arc;
use std::ops::Deref;
use serde::{Deserialize};

use crate::appstate::AppState;
use crate::basetemplate::BaseTemplate;

// Note
#[derive(Template)]
#[template(path = "pages/search.html")]
struct SearchTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub search_data: SearchData,
}

impl<'a> Deref for SearchTemplate<'a> {
    type Target = BaseTemplate<'a>;
    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchData {
    query: Option<String>,
}

pub async fn get(
        State(_state): State<Arc<AppState>>,
        Query(payload): Query<SearchData>
    ) -> impl IntoResponse {

    let template = SearchTemplate { 
        _parent: &BaseTemplate { 
            pagetitle: "Search",
            dark_mode: true,
        },
        search_data: payload,
    };

    let reply_html = template.render().unwrap();
    return (StatusCode::OK, Html(reply_html).into_response());
     
}