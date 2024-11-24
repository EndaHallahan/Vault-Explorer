use vault_dweller::{ VaultIndex };
use indexmap::{ IndexMap };
use tantivy::{ IndexReader, schema::Schema, query::QueryParser, };

//#[derive(Debug)]
pub struct AppState {
    pub vaults: IndexMap<String, VaultIndex>,
    pub search_reader: IndexReader,
    pub query_parser: QueryParser,
    pub schema: Schema
}