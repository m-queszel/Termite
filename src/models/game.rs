use std::path::PathBuf;

#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub path: PathBuf,
    pub mods_path: Option<PathBuf>,
    pub mods: Vec<Mod>,
}

#[derive(Debug)]
pub struct Mod {
    pub name: String,
    pub path: PathBuf,
    pub enabled: bool,
}
