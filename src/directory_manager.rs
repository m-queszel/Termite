use crate::models::game::Game;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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

pub fn list_steam_games(steam_root: PathBuf) -> Vec<Game> {
    let common_path = steam_root.join("steamapps").join("common");
    match fs::read_dir(&common_path) {
        Ok(entries) => entries
            .filter_map(|res| res.ok())
            .filter_map(|e| {
                let path = e.path();
                let name = path.file_name()?.to_str()?.to_owned();
                Some(Game {
                    name,
                    path,
                    mods_path: None,
                    mods: Vec::new(),
                })
            })
            .filter(|game| !game.name.contains("Proton") && !game.name.contains("Steam"))
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

pub fn grant_flatpak_permission(staging_path: &Path) -> std::io::Result<()> {
    Command::new("flatpak")
        .args([
            "override",
            "--user",
            &format!("--filesystem={}", staging_path.display()),
            "com.valvesoftware.Steam",
        ])
        .status()?;
    Ok(())
}

pub fn is_flatpak_game(game_path: &Path) -> bool {
    game_path
        .to_string_lossy()
        .contains(".var/app/com.valvesoftware.Steam")
}
