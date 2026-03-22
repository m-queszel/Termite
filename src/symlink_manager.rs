use crate::models::game::{Game, InjectionStrategy, Mod};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use walkdir::WalkDir;

pub fn apply_mod(game: &Game, mod_index: usize) -> std::io::Result<()> {
    let mod_item = &game.mods[mod_index];
    if let InjectionStrategy::MergeFiles = mod_item.injection_method {
        for entry in WalkDir::new(&mod_item.path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let source = entry.path();
            let relative_path = source
                .strip_prefix(&mod_item.path)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let target = game.path.join(relative_path);

            if source.is_dir() {
                fs::create_dir_all(&target)?;
            } else if source.is_file() && !target.exists() {
                symlink(source, &target)?;
            }
        }
    }
    Ok(())
}
pub fn remove_mod(game: &Game, mod_index: usize) -> std::io::Result<()> {
    let mod_item = &game.mods[mod_index];
    if let InjectionStrategy::MergeFiles = mod_item.injection_method {
        for entry in WalkDir::new(&mod_item.path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let source = entry.path();
            let relative_path = source
                .strip_prefix(&mod_item.path)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let target = game.path.join(relative_path);

            if target.is_symlink() {
                fs::remove_file(target)?;
            }
        }
    }
    Ok(())
}
