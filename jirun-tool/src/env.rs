use std::path::PathBuf;

pub fn try_load_dotenv() {
    for path in env_locations() {
        if path.exists() {
            dotenvy::from_path(path).ok();
            break;
        }
    }
}

fn env_locations() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join(".env"));
    }
    if let Some(home_config) = dirs::config_dir() {
        paths.push(home_config.join("jirun").join(".env"));
    }
    paths
}
