//! Agent Ring entrypoint. Loads config, then hands off to the platform tray app.
use agentring::config::Config;
use std::path::PathBuf;

fn config_path() -> Result<PathBuf, String> {
    let dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/agentring"))
        .unwrap_or_else(|_| PathBuf::from(".agentring"));
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("could not create config directory {}: {e}", dir.display()))?;
    Ok(dir.join("config.toml"))
}

fn run() -> Result<(), String> {
    let path = config_path()?;
    let existed = path
        .try_exists()
        .map_err(|e| format!("could not inspect config {}: {e}", path.display()))?;
    let config = Config::load_or_default(&path)
        .map_err(|e| format!("could not load config {}: {e}", path.display()))?;
    // Persist defaults on first run so the user has a file to edit.
    if !existed {
        let toml = config
            .to_toml()
            .map_err(|e| format!("could not serialize default config: {e}"))?;
        std::fs::write(&path, toml)
            .map_err(|e| format!("could not write config {}: {e}", path.display()))?;
    }

    #[cfg(target_os = "macos")]
    {
        agentring::app::run(config)?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = config;
        eprintln!("agentring: only macOS is wired in this build (M1); Windows is M3.");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("agentring: {e}");
        std::process::exit(1);
    }
}
