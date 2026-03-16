use std::{env, net::SocketAddr};

use tracing::info;
use tracing_subscriber::EnvFilter;
use workspace::{WorkspaceRepository, find_fixture_root};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("relay=info,tower_http=info")),
        )
        .init();

    let root = env::var("WEAVE_WORKSPACE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or(find_fixture_root()?);
    let address: SocketAddr = env::var("WEAVE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8787".to_string())
        .parse()?;

    let repository = WorkspaceRepository::new(root.clone());
    let app = api::app(repository);
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!(workspace_root = %root.display(), %address, "relay listening");
    axum::serve(listener, app).await?;
    Ok(())
}
