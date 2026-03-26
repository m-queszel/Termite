use crate::models::game::{Game, InjectionStrategy, Mod};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use walkdir::WalkDir;

pub fn apply_mod(game: &Game, mod_index: usize) -> std::io::Result<()> {
    let mod_item = &game.mods[mod_index];

    let landing_zone = match mod_item.injection_method {
        InjectionStrategy::MergeFiles => &game.path,
        InjectionStrategy::TargetSubfolder | InjectionStrategy::AddAsFolder(_) => {
            game.mods_path.as_ref().unwrap_or(&game.path)
        }
    };

    match &mod_item.injection_method {
        InjectionStrategy::MergeFiles | InjectionStrategy::TargetSubfolder => {
            for entry in WalkDir::new(&mod_item.path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let source = entry.path();
                let relative = source
                    .strip_prefix(&mod_item.path)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                let target = landing_zone.join(relative);

                if source.is_dir() {
                    fs::create_dir_all(&target)?;
                } else if source.is_file() && !target.exists() {
                    symlink(source, &target)?;
                }
            }
        }

        InjectionStrategy::AddAsFolder(folder_name) => {
            let target = landing_zone.join(folder_name);
            if !target.exists() {
                symlink(&mod_item.path, &target)?;
            }
        }
    }
    Ok(())
}
pub fn remove_mod(game: &Game, mod_index: usize) -> std::io::Result<()> {
    let mod_item = &game.mods[mod_index];

    let landing_zone = match mod_item.injection_method {
        InjectionStrategy::MergeFiles => &game.path,
        InjectionStrategy::TargetSubfolder | InjectionStrategy::AddAsFolder(_) => {
            game.mods_path.as_ref().unwrap_or(&game.path)
        }
    };

    match &mod_item.injection_method {
        InjectionStrategy::MergeFiles | InjectionStrategy::TargetSubfolder => {
            for entry in WalkDir::new(&mod_item.path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let relative = entry
                    .path()
                    .strip_prefix(&mod_item.path)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                let target = landing_zone.join(relative);

                if target.is_symlink() {
                    fs::remove_file(target)?;
                }
            }
        }
        InjectionStrategy::AddAsFolder(folder_name) => {
            let target = landing_zone.join(folder_name);
            if target.is_symlink() {
                fs::remove_file(target)?;
            }
        }
    }
    Ok(())
}
