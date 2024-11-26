

use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use askama::Template;
use std::sync::Arc;
use std::ops::Deref;

use vault_dweller::{ Tree, };

use crate::appstate::AppState;
use crate::basetemplate::BaseTemplate;

use crate::helpers::{template_structs::TreeEntry, traits::TreeBuilder, markdown::parse_md};
use crate::impl_treebuilder;

// Note
#[derive(Template)]
#[template(path = "pages/note.html")]
struct NoteTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub note: Note,
    pub tree_entry: Option<TreeEntry>,
    pub vault_name: String,
}

impl_treebuilder!(NoteTemplate<'a>);

impl<'a> Deref for NoteTemplate<'a> {
    type Target = BaseTemplate<'a>;
    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

struct Note {
    name: String,
    tags: Vec<String>,
    content: String,
}

pub async fn get(
        State(state): State<Arc<AppState>>,
        Path((vault, note)): Path<(String, String)>
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