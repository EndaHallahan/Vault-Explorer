use vault_dweller::{ VaultIndex };
use indexmap::{ IndexMap };

//#[derive(Debug)]
pub struct AppState {
    pub vaults: IndexMap<String, VaultIndex>,
}