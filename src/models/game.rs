use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub path: PathBuf,
    pub mods_path: Option<PathBuf>,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    pub name: String,
    pub path: PathBuf,
    pub enabled: bool,
    pub injection_method: InjectionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjectionStrategy {
    MergeFiles,
    AddAsFolder(PathBuf),
    TargetSubfolder,
}
