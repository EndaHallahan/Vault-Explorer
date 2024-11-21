use axum::{
    http::{StatusCode},
    response::{Json, Html, Redirect, IntoResponse},
    extract::{State, Path},
};
use askama::Template;
use std::sync::Arc;
use std::ops::Deref;
use comrak::{markdown_to_html, Options};
use regex::{Captures, Regex};
use html_escape::encode_text;

use vault_dweller::{ VaultIndex, NoteItem, TreeNode, ItemType, Tree, QueryOutput};

use crate::appstate::AppState;
use crate::basetemplate::BaseTemplate;

// Any filter defined in the module `filters` is accessible in your template.
mod filters {
    // This filter does not have extra arguments
    pub fn pathify<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace(" ", "_").replace('/', "%2F"))
    }
}

#[derive(Template)]
#[template(path = "components/tree_entry.html", escape = "none")]
struct TreeEntry {
    name: String,
    item: ItemType,
    vault_name: String,
    children: Vec<TreeEntry>,
}

// Note
#[derive(Template)]
#[template(path = "pages/note.html")]
struct NoteTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub note: Note,
    pub tree_entry: Option<TreeEntry>,
}
impl<'a> NoteTemplate<'a> {
    pub fn build_tree(&mut self, tree: &Tree, vault_name: String) {
        self.tree_entry = Some(self.recurse_tree(0, tree, vault_name));
    }

    fn recurse_tree(&self, node_index: usize, tree: &Tree, vault_name: String) -> TreeEntry {
        let node = tree.get_node(node_index).expect("Couldn't get node!");
        let mut children: Vec<TreeEntry> = vec![];
        for child_index in &node.children {
            children.push(self.recurse_tree(*child_index, tree, vault_name.clone()));
        }
        TreeEntry {
            name: node.name.clone(),
            item: node.item.clone(),
            vault_name,
            children,
        }
    }
}
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

fn convert_links(in_md: String, vault_name: String) -> String {
    let re = Regex::new(r"\[\[([^\]]+?)(\|[^\]]+)?\]\]").unwrap();
    return re.replace_all(&in_md, |caps: &Captures| {
        let mut l_text = &caps[1];
        if caps.get(2) != None {
            l_text = &caps[2];
        }
        return format!(
            "[{}](/vault/{}/note/{})", 
            l_text.replacen('|', "", 1), 
            encode_text(&vault_name.replace(' ', "_").replace('/', "%2F")), 
            encode_text(&caps[1].replace(' ', "_").replace('/', "%2F"))
        );
    }).to_string();
}

fn convert_dataview(in_md: String, vault: &VaultIndex) -> String {
    let re = Regex::new(r"```dataview[\n\r]([\w\W]*?)[\n\r]```").unwrap();
     return re.replace_all(&in_md, |caps: &Captures| {
        let mut out_string: String = Default::default();
        let query_out = vault.query(&caps[1]);
        match query_out {
            QueryOutput::List(list) => {
                if list.len() == 0 {
                    out_string.push_str("\n> `Dataview: No results to show for list query.`\n");
                } else {
                    for li in list {
                        let mut name: String = Default::default();
                        let mut add_info: String = Default::default();
                        if let Some(note_name) = li.note_name {
                            name = format!(
                                "[{}](/vault/{}/note/{})",
                                note_name,
                                encode_text(&vault.name.replace(' ', "_").replace('/', "%2F")), 
                                encode_text(&note_name.replace(' ', "_").replace('/', "%2F"))
                            );
                        }
                        if let Some(additional_info) = li.additional_info {
                            add_info = format!(": {}", additional_info);
                        }
                        out_string.push_str(&format!("- {}{}\n", name, add_info));
                    }
                }
            },
            QueryOutput::Err(err_vec) => {
                for err in err_vec {
                    out_string.push_str(&format!("\n> `Dataview Error: {}`", err));
                }
                out_string.push_str("\n");
            },
            _ => todo!("Not implemented yet!"),
        }

        out_string
    }).to_string();
}

fn  markdown_options<'a>() -> Options<'a> {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.render.unsafe_ = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.underline = true;
    options.extension.spoiler = true;


    options
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
                content: markdown_to_html(&convert_dataview(convert_links(n.get_contents().expect("Couldn't get note contents!"), vault), &vi), &markdown_options()),
            };

            let mut template = NoteTemplate { 
                _parent: &BaseTemplate { 
                    pagetitle: &note.name.clone(),
                    dark_mode: true,
                },
                note,
                tree_entry: None,
            };
            template.build_tree(&vi.tree, vault_name);
            let reply_html = template.render().unwrap();
            return (StatusCode::OK, Html(reply_html).into_response());
        } else {
            return (StatusCode::NOT_FOUND, Html("<h1>404</h1>").into_response());
        }
    } else {
        return (StatusCode::NOT_FOUND, Html("<h1>404</h1>").into_response());
    }

     
}