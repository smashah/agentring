//! Agent Ring entrypoint. Loads config, then hands off to the platform tray app.
use agentring::config::Config;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/agentring"))
        .unwrap_or_else(|_| PathBuf::from(".agentring"));
    let _ = std::fs::create_dir_all(&dir);
    dir.join("config.toml")
}

fn main() {
    let path = config_path();
    let config = Config::load_or_default(&path).unwrap_or_else(|e| {
        eprintln!("agentring: config load failed ({e}), using defaults");
        Config::default_for_wx02()
    });
    // Persist defaults on first run so the user has a file to edit.
    if !path.exists() {
        if let Ok(toml) = config.to_toml() {
            let _ = std::fs::write(&path, toml);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Err(e) = agentring::app::run(config) {
            eprintln!("agentring: {e}");
            std::process::exit(1);
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = config;
        eprintln!("agentring: only macOS is wired in this build (M1); Windows is M3.");
    }
}
