
# Termite

Termite is a TUI mod manager built for Linux environments. Termite uses symlink injections to manage mods and allows users to fully mod games from within the terminal.



## Installation

Termite is built with Rust. Ensure you have the Rust toolchain (https://rustup.rs/) installed before proceeding.

1. Clone the repository:

```bash
  git clone https://github.com/m-queszel/termite.git
  cd termite
```
2. Build and Run:
```bash
cargo run --release
```

## Navigation & Usage

Global Keys:

- j / k : Move Selection Down / Up
- Tab : Toggle focus between Games and Mods
- q : Quit Termite

Management:

- d : open Game Library (Select your game folder)
- Enter : Focus on the selected game
- m : Link a Mod Stagin Folder to the active game
- i : Manually set an Injection Path (The Mods/ folder)
- Space : Toggle (enable/disable) the selected Mod

File Explorer:

- l : Enter Directory
- h : Go back (Parent directory)
- s : Select current directory (used for selecing the injection path)
- Esc : Close the file Explorer

## Flatpak

If you use Steam via Flatpak, Termite will automatically detect that the game is sandboxed. When you link a mod folder, Termite will prompt you to grant the necessary filesystem permissions so the game can actually "see" your mods.
    
