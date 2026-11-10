use std::{env::current_exe, fs};

use ardeck::SwitchKind;
use serde::{Deserialize, Serialize};

static MAPPING_FILE_NAME: &'static str = "mapping.ron";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingPreset {
    preset_name: String,
    mappinglist: Vec<Mapping>,
}

impl MappingPreset {
    pub fn preset_name(&self) -> &str {
        &self.preset_name
    }

    pub fn mappinglist(&self) -> &[Mapping] {
        &self.mappinglist
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    switch_kind: SwitchKind,
    switch_id: u8,
    press_key: char,
}

impl Mapping {
    pub fn switch_kind(&self) -> SwitchKind {
        self.switch_kind
    }

    pub fn switch_id(&self) -> u8 {
        self.switch_id
    }

    pub fn press_key(&self) -> char {
        self.press_key
    }
}

pub fn load_config() -> Option<Vec<MappingPreset>> {
    let current_exe = match current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Critical Error: Failed to get the exe location: {e}");
            panic!();
        }
    };

    let mapping_file = current_exe
        .parent()
        .expect("Failed to get executable directory")
        .join(MAPPING_FILE_NAME);

    let config = match fs::read_to_string(&mapping_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {MAPPING_FILE_NAME}: {e}");
            return None;
        }
    };

    let mapping_presets: Vec<MappingPreset> = match ron::from_str(&config) {
        Ok(presets) => presets,
        Err(e) => {
            eprintln!("Failed to parse {MAPPING_FILE_NAME}: {e}");
            panic!();
        }
    };

    Some(mapping_presets)
}
