use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn find_steam_root() -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    let paths = vec![
        format!("{}/.local/share/Steam", home),
        format!(
            "{}/.var/app/com.valvesoftware.Steam/.local/share/Steam",
            home
        ),
    ];

    paths
        .into_iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}

pub fn list_steam_games(steam_root: PathBuf) -> Vec<String> {
    let common_path = steam_root.join("steamapps").join("common");
    match fs::read_dir(common_path) {
        Ok(entries) => entries
            .filter_map(|res| res.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .filter_map(|p| p.file_name()?.to_str()?.to_owned().into())
            .filter(|name| !name.contains("Proton") && !name.contains("Steam"))
            .collect(),
        Err(_) => vec![],
    }
}

pub fn list_directory_contents(dir: &Path) -> Vec<String> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|res| res.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect(),
        Err(_) => vec!["<Could not read directory>".to_string()],
    }
}
