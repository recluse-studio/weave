use std::{env, net::SocketAddr, thread};

use anyhow::Result;
use workspace::{WorkspaceRepository, find_fixture_root};

fn main() {
    if let Err(error) = run() {
        eprintln!("WEAVE desktop shell failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let workspace_root = env::var("WEAVE_WORKSPACE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or(find_fixture_root()?);
    let address: SocketAddr = env::var("WEAVE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8787".to_string())
        .parse()?;

    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("desktop runtime");
        runtime.block_on(async move {
            let repository = WorkspaceRepository::new(workspace_root);
            let app = api::app(repository);
            let listener = tokio::net::TcpListener::bind(address)
                .await
                .expect("desktop relay listener");
            axum::serve(listener, app)
                .await
                .expect("desktop relay serve");
        });
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("desktop shell run");

    Ok(())
}
