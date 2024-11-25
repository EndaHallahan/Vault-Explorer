mod route_handlers;
mod appstate;
mod basetemplate;
mod helpers;
mod search_indexer;

use axum::{
    routing::get,
    Router,
};
use std::{net::SocketAddr};
use tokio::signal;
use std::env;
use std::sync::Arc;
use std::fs;
use indexmap::{ IndexMap };
use tower_http::services::ServeDir;

use appstate::AppState;
use vault_dweller::{ VaultIndex, };
use route_handlers::{ root, note, search, api_search };
use search_indexer::build_search_index;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let vaults_dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/vaults");
    let vaults: IndexMap<String, VaultIndex> = get_vault_index_map(vaults_dir_path);
    let (search_reader, query_parser, schema) = build_search_index(&vaults).expect("Couldn't create search index!");
    let shared_state = Arc::new(AppState {
        vaults,
        search_reader,
        query_parser,
        schema,
    });

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(root::get))
        .route("/vault/:vault/note/:note", get(note::get))
        .route("/search", get(search::get))
        .route("/api/search", get(api_search::get))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server!");

    Ok(())   
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");

    /*#[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;

        println!("signal received, starting graceful shutdown");
    };

    #[cfg(not(unix))]
    let terminate = {
        std::future::pending::<()>();
        println!("signal received, starting graceful shutdown");
    };*/

    /*tokio::select! {
        _ = signal::ctrl_c => {},
        _ = terminate => {},
    }*/
}

fn get_vault_index_map(vault_dir_path: &str) -> IndexMap<String, VaultIndex> {
    let mut vaults: IndexMap<String, VaultIndex> = Default::default();
    let vault_dir_contents = fs::read_dir(vault_dir_path);
    match vault_dir_contents {
        Ok(entries) => {
            for entry in entries {
                let p = entry.unwrap().path();
                let name = p.file_name().unwrap().to_str().unwrap().to_owned();
                let vi = VaultIndex::new(Some(p.to_str().unwrap()), false);
                vaults.insert(name.clone(), vi.expect("Error creating vault index!"));
            }
        },
        Err(e) => {
            panic!("Error Getting vaults! {:?}", e);
        }
    }
    vaults
}
