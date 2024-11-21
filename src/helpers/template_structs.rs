use askama::Template;

use vault_dweller::{ ItemType, };
use crate::helpers::{filters};

#[derive(Template)]
#[template(path = "components/tree_entry.html", escape = "none")]
pub struct TreeEntry {
    pub name: String,
    pub item: ItemType,
    pub vault_name: String,
    pub children: Vec<TreeEntry>,
}