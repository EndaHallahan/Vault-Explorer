use axum::{
    http::{StatusCode},
    response::{Json, IntoResponse},
    extract::{State, Query,},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tantivy::schema::*;
use tantivy::collector::TopDocs;

use crate::appstate::AppState;
use crate::helpers::ajax_responses::{ respond_to_search, SearchResult };
use crate::helpers::markdown::parse_md;

#[derive(Deserialize, Debug)]
pub struct SearchData {
    query: String,
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

    if let Ok(query) = state.query_parser.parse_query(&payload.query) {
        if let Ok(top_docs) = searcher.search(&query, &TopDocs::with_limit(50)) {
            let mut results: Vec<SearchResult> = vec![];
            for (_score, doc_address) in top_docs {
                let retrieved_doc: TantivyDocument = searcher.doc(doc_address).expect("Couldn't retrieve document from index!");
                
                let title = state.schema.get_field("title").unwrap();
                let vault = state.schema.get_field("vault").unwrap();
                let tags = state.schema.get_field("tags").unwrap();
                let body = state.schema.get_field("body").unwrap();

                let mut doc_vault_name: String = Default::default();
                let mut doc_title: String = Default::default();
                let mut doc_body: String = Default::default();
                if let OwnedValue::Str(vn) = retrieved_doc.get_first(vault).unwrap() {
                    doc_vault_name = vn.to_string();
                };
                if let OwnedValue::Str(t) = retrieved_doc.get_first(title).unwrap() {
                    doc_title = t.to_string();
                };
                if let OwnedValue::Str(b) = retrieved_doc.get_first(body).unwrap() {
                    doc_body = b.to_string();
                };

                if let Some(vi) =  state.vaults.get(&doc_vault_name) {
                    
                    let new_result = SearchResult {
                        title: doc_title,
                        body: parse_md(doc_body, &vi),
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

    
