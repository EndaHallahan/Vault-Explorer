

use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use askama::Template;
use std::sync::Arc;

use crate::appstate::AppState;
use crate::helpers::{
    traits::TreeBuilder,
    markdown::parse_md,
    shared_templates::{Note, NoteTemplate},
};
use crate::basetemplate::BaseTemplate;

pub async fn get(
        State(state): State<Arc<AppState>>,
        Path((vault, note)): Path<(String, String)>,
    ) -> impl IntoResponse {

    let vault_name = vault.replace('_', " ");
    let note_name = note.replace('_', " ");

    if let Some(vi) =  state.vaults.get(&vault_name) {
        let vault_name = vi.name.clone();
        if let Some(n) = vi.get_note(&note_name) {
            let note = Note {
                name: n.name.clone(),
                tags: n.tags.clone(),
                content: parse_md(n.get_contents().expect("Couldn't get note contents!"), &vi),
            };

            let mut template = NoteTemplate { 
                _parent: &BaseTemplate { 
                    pagetitle: &note.name.clone(),
                    dark_mode: true,
                },
                note,
                tree_entry: None,
                vault_name: vault_name.clone(),
            };

            template.build_tree(&vi.tree, vault_name, note_name);
            let reply_html = template.render().unwrap();
            return (StatusCode::OK, Html(reply_html).into_response());
        } else {
            return (StatusCode::NOT_FOUND, Html("<h1>404</h1>").into_response());
        }
    } else {
        return (StatusCode::NOT_FOUND, Html("<h1>404</h1>").into_response());
    }

     
}