use vault_dweller::{ Tree, };
use crate::helpers::shared_templates::TreeEntry;

pub trait TreeBuilder {  
    fn build_tree(&mut self, tree: &Tree, vault_name: String, open_note_name: String);
    fn recurse_tree(&self, node_index: usize, tree: &Tree, vault_name: String, open_note_name: String) -> TreeEntry;
}

#[macro_export] 
macro_rules! impl_treebuilder {
    ($type:ty) => {
        impl <'a> TreeBuilder for $type {
            fn build_tree(&mut self, tree: &Tree, vault_name: String, open_note_name: String) {
                self.tree_entry = Some(self.recurse_tree(0, tree, vault_name, open_note_name));
            }

            fn recurse_tree(&self, node_index: usize, tree: &Tree, vault_name: String, open_note_name: String) -> TreeEntry {
                let node = tree.get_node(node_index).expect("Couldn't get node!");
                let mut children: Vec<TreeEntry> = vec![];
                for child_index in &node.children {
                    children.push(self.recurse_tree(*child_index, tree, vault_name.clone(), open_note_name.clone()));
                }
                TreeEntry {
                    name: node.name.clone(),
                    item: node.item.clone(),
                    vault_name,
                    open_note_name,
                    children,
                }
            }
        }
    }
}