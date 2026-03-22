use crate::models::game::Game;
use home::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn save(games: &[Game]) -> std::io::Result<()> {
    if let Some(path) = get_config_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(games)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        fs::write(path, json)?;
    }
    Ok(())
}

fn get_config_path() -> Option<PathBuf> {
    home_dir().map(|path| path.join(".config").join("termite").join("config.json"))
}

pub fn load() -> Option<Vec<Game>> {
    let path = get_config_path()?;
    if !path.exists() {
        return None;
    }
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}
