//! Combo overlay support: reads a `combos.yaml` file next to the executable
//! (or the current working directory) and exposes the list of combos to be
//! rendered as arcs/labels on top of the keyboard layout.
//!
//! YAML format (positions are indices into the physical-layout key list,
//! matching what `keymap-drawer` uses):
//!
//! ```yaml
//! combos:
//!   - positions: [1, 11]
//!     label: "@"
//!   - positions: [16, 17, 18]
//!     label: "Enter"
//! ```

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct Combo {
    pub positions: Vec<usize>,
    pub label: String,
    /// Optional list of layer indices the combo is active on. Empty = always.
    #[serde(default)]
    pub layers: Vec<usize>,
}

#[derive(Deserialize, Debug, Default)]
struct CombosFile {
    #[serde(default)]
    combos: Vec<Combo>,
}

fn find_combos_file() -> Option<PathBuf> {
    // 1. Next to the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("combos.yaml");
            if p.exists() {
                return Some(p);
            }
        }
    }
    // 2. Current working directory
    let cwd = PathBuf::from("combos.yaml");
    if cwd.exists() {
        return Some(cwd);
    }
    None
}

pub fn load_combos() -> Vec<Combo> {
    let Some(path) = find_combos_file() else {
        return Vec::new();
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("keypeek: failed to read {}: {}", path.display(), e);
            return Vec::new();
        }
    };
    let parsed: CombosFile = match serde_yaml::from_str(&text) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("keypeek: failed to parse {}: {}", path.display(), e);
            return Vec::new();
        }
    };
    eprintln!(
        "keypeek: loaded {} combos from {}",
        parsed.combos.len(),
        path.display()
    );
    parsed.combos
}
