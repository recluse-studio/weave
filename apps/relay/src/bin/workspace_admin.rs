use std::env;

use anyhow::{Result, bail};
use drive_sync::{audit_workspace, rebuild_cache};
use workspace::{WorkspaceRepository, find_fixture_root};

fn main() -> Result<()> {
    let root = env::var("WEAVE_WORKSPACE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or(find_fixture_root()?);
    let repository = WorkspaceRepository::new(root);
    let command = env::args().nth(1).unwrap_or_else(|| "audit".to_string());

    let payload = match command.as_str() {
        "audit" => serde_json::to_string_pretty(&audit_workspace(&repository)?)?,
        "rebuild" => serde_json::to_string_pretty(&rebuild_cache(&repository)?)?,
        other => bail!("unknown workspace admin command: {other}"),
    };

    println!("{payload}");
    Ok(())
}
