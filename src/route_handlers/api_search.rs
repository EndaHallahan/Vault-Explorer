use axum::{
    http::{StatusCode},
    response::{Json, IntoResponse},
    extract::{State, Query,},
};
use std::sync::Arc;
use serde::{Deserialize};
use tantivy::schema::*;
use tantivy::collector::TopDocs;
use tantivy::snippet::{Snippet, SnippetGenerator};
use unicode_segmentation::UnicodeSegmentation;

use crate::appstate::AppState;
use crate::helpers::ajax_responses::{ respond_to_search, SearchResult };
use crate::helpers::markdown::strip_md;
use crate::helpers::tags::condense_tags;

#[derive(Deserialize, Debug)]
pub struct SearchData {
    query: String,
}

fn highlight(snippet: Snippet) -> String {
    let mut result = String::new();
    let mut start_from = 0;

    for fragment_range in snippet.highlighted() {
        result.push_str(&snippet.fragment()[start_from..fragment_range.start]);
        result.push_str("<b>");
        result.push_str(&snippet.fragment()[fragment_range.clone()]);
        result.push_str("</b>");
        start_from = fragment_range.end;
    }

    result.push_str(&snippet.fragment()[start_from..]);

    result
}


pub async fn get(
        State(state): State<Arc<AppState>>,
        payload: Query<SearchData>
    ) -> impl IntoResponse {

     // Empty fields
    if payload.query.is_empty() {
        //println!("Name field empty!");
        let response = respond_to_search(false, Some("Empty search body!".to_string()), None);
        return (StatusCode::BAD_REQUEST, Json(response).into_response());
    }

    if payload.query.len() > 1000 {
        //println!("Name field too long!");
        let response = respond_to_search(false, Some("Search body was too long!".to_string()), None);
        return (StatusCode::BAD_REQUEST, Json(response).into_response());
    }

    let searcher = state.search_reader.searcher();

    let title = state.schema.get_field("title").unwrap();
    let vault = state.schema.get_field("vault").unwrap();
    let tag = state.schema.get_field("tag").unwrap();
    let body = state.schema.get_field("body").unwrap();


    if let Ok(query) = state.query_parser.parse_query(&payload.query) {
        let snippet_generator = SnippetGenerator::create(&searcher, &*query, body).expect("Couldn't create snippit generator!");
        if let Ok(top_docs) = searcher.search(&query, &TopDocs::with_limit(50)) {
            let mut results: Vec<SearchResult> = vec![];
            for (_score, doc_address) in top_docs {
                let retrieved_doc: TantivyDocument = searcher.doc(doc_address).expect("Couldn't retrieve document from index!");
                
                let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);

                let mut doc_vault_name: String = Default::default();
                let mut doc_title: String = Default::default();
                let mut doc_body: String = Default::default();
                let mut doc_tags: Vec<String> = vec![];
                if let Some(OwnedValue::Str(vn)) = retrieved_doc.get_first(vault) {
                    doc_vault_name = vn.to_string();
                };
                if let Some(OwnedValue::Str(t)) = retrieved_doc.get_first(title) {
                    doc_title = t.to_string();
                };
                
                if snippet.is_empty() {
                    if let Some(OwnedValue::Str(b)) = retrieved_doc.get_first(body) {
                        let mut g = UnicodeSegmentation::graphemes(b.as_str(), true).collect::<Vec<&str>>();
                        g.truncate(150);
                        doc_body = g.join("");
                        doc_body.push_str("...");
                    };
                } else {
                    doc_body = format!("...{}...", highlight(snippet));
                }

                retrieved_doc.get_all(tag).for_each(|ot| {
                    if let OwnedValue::Str(tag) = ot {
                        doc_tags.push(tag.to_string());
                    }
                });

                if let Some(_vi) =  state.vaults.get(&doc_vault_name) {
                    
                    let new_result = SearchResult {
                        title: doc_title,
                        tags: condense_tags(doc_tags),
                        body: strip_md(doc_body),
                        vault: doc_vault_name,
                    };
                    results.push(new_result);
                } else {
                    let response = respond_to_search(false, Some("Something went wrong on our end! Could not find vault in search result.".to_string()), None);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(response).into_response());
                };
            }
            let response = respond_to_search(true, None, Some(results));
            return (StatusCode::OK, Json(response).into_response());
        } else {
            let response = respond_to_search(false, Some("Search body was malformed!".to_string()), None);
            return (StatusCode::BAD_REQUEST, Json(response).into_response());
        }
            
    } else {
        let response = respond_to_search(false, Some("Something went wrong on our end!".to_string()), None);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response).into_response());
    }
}

    
