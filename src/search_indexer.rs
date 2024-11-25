use std::{ fs, env, path::PathBuf };
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, IndexReader, ReloadPolicy, error::TantivyError, directory::MmapDirectory};
use indexmap::{ IndexMap };

use vault_dweller::{ VaultIndex, NoteItem };

fn get_dir_path() -> PathBuf {
	let mut p = env::current_dir().unwrap();
	p.push("tantivy_index");
	p
}

pub fn build_search_index(vaults: &IndexMap<String, VaultIndex>) -> Result<(IndexReader, QueryParser, Schema), TantivyError> {
	let path = get_dir_path();
	if fs::exists(&path).is_err() || !fs::exists(&path).unwrap() {
		fs::create_dir(&path).expect("Couldn't create /tantivy_index directory!");
	}

	let mut schema_builder = Schema::builder();

	schema_builder.add_text_field("title", TEXT | STORED);
	schema_builder.add_text_field("vault", STRING | STORED);
	schema_builder.add_text_field("tag", STRING | STORED);
	schema_builder.add_text_field("body", TEXT | STORED);
	let schema = schema_builder.build();
	let directory = MmapDirectory::open(path).expect("Couldn't find index directory!");
	let index = Index::open_or_create(directory, schema.clone())?;

	let mut index_writer: IndexWriter = index.writer(50_000_000)?;

	let _clear_res = index_writer.delete_all_documents().unwrap();
    index_writer.commit()?;

	let title = schema.get_field("title").unwrap();
	let vault = schema.get_field("vault").unwrap();
	let tags = schema.get_field("tag").unwrap();
    let body = schema.get_field("body").unwrap();

    for e in vaults.values().enumerate() {
    	let (_, vi): (usize, &VaultIndex) = e;
    	let vault_name: &String = &vi.name;
    	for x in vi.notes.values().enumerate() {
    		let (_, note): (usize, &NoteItem) = x;
    		let mut note_doc = TantivyDocument::default();
    		
    		note_doc.add_text(title, &note.name);
    		note_doc.add_text(vault, vault_name);
    		for tag in &note.tags {
    			note_doc.add_text(tags, &tag);
    		}
    		note_doc.add_text(body, &note.get_contents().expect("Couldn't open note!"));
    		
    		index_writer.add_document(note_doc)?;
    	}
    }

    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let query_parser = QueryParser::for_index(&index, vec![title, body, vault]);

    Ok((reader, query_parser, schema))
}