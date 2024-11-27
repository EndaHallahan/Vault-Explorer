use askama::Template;
use std::ops::Deref;

use vault_dweller::{ Tree, ItemType };

use crate::helpers::{traits::TreeBuilder, filters};
use crate::impl_treebuilder;
use crate::basetemplate::BaseTemplate;

// Note
#[derive(Template)]
#[template(path = "pages/note.html")]
pub struct NoteTemplate<'a> {
    pub _parent: &'a BaseTemplate<'a>,
    pub note: Note,
    pub tree_entry: Option<TreeEntry>,
    pub vault_name: String,
    pub side_nav_closed: bool,
}

impl_treebuilder!(NoteTemplate<'a>);

impl<'a> Deref for NoteTemplate<'a> {
    type Target = BaseTemplate<'a>;
    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[derive(Template)]
#[template(path = "components/tree_entry.html", escape = "none")]
pub struct TreeEntry {
    pub name: String,
    pub item: ItemType,
    pub vault_name: String,
    pub open_note_name: String,
    pub children: Vec<TreeEntry>,
}

pub struct Note {
    pub name: String,
    pub tags: Vec<String>,
    pub content: String,
}