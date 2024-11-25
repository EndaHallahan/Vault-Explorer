use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct SearchResult {
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
    pub vault: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub message: Option<String>,
    pub results: Option<Vec<SearchResult>>,
}

pub fn respond_to_search(success: bool, message: Option<String>, results:Option<Vec<SearchResult>>) -> SearchResponse {
    SearchResponse {
        success,
        message,
        results,
    }
}