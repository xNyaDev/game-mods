use documented::DocumentedFields;
use serde::{Deserialize, Serialize};
use crate::{DocumentedStruct, TypedFields};

impl DocumentedStruct for Config {
    fn get_doc_string(location: &str, field: &str) -> Option<&'static str> {
        match location {
            "" => Config::get_field_docs(field).ok(),
            "loading" => Loading::get_field_docs(field).ok(),
            "logging" => Logging::get_field_docs(field).ok(),
            _ => None,
        }
    }

    fn get_type_string(location: &str, field: &str) -> Option<&'static str> {
        match location {
            "" => Config::get_field_type(field),
            "loading" => Loading::get_field_type(field),
            "logging" => Logging::get_field_type(field),
            _ => None,
        }
    }
}

#[derive(Default, DocumentedFields, Serialize, Deserialize, TypedFields)]
pub struct Config {
    /// Options related to loading DLLs
    pub loading: Loading,
    /// Options related to logging
    pub logging: Logging,
}

#[derive(DocumentedFields, Serialize, Deserialize, TypedFields)]
pub struct Loading {
    /// A list of files to load. Unix shell style patterns are supported
    pub load_paths: Vec<String>,
    /// Some loaders change the directory while loading. For example if the DLL is `mods/mod.dll`,
    /// the loader will change directory into `mods` while loading it, then back. Some mods rely on
    /// that and will fail loading if the correct directory isn't set.
    ///
    /// Enable this option to switch to the behaviour described above
    pub change_workdir: bool,
    /// If enabled, create a thread and load all DLLs from inside that thread.
    /// If disabled, everything is loaded in DllMain.
    ///
    /// Original DLL always gets loaded in DllMain.
    pub delayed_load: bool,
    /// While trying to load the original library, skip any functions that were not found.
    /// Will cause a crash if the game tries to use that function as it causes a `jmp 0`.
    ///
    /// Sometimes needed if the modloader DLL was built on Windows natively and is being used in Wine.
    /// Do not enable unless you get an error saying to do so
    pub skip_missing_symbols: bool,
}

#[derive(Default, DocumentedFields, Serialize, Deserialize, TypedFields)]
pub struct Logging {
    /// Enable logging what is being loaded
    pub enable_logging: bool,
    /// Create a new console window if there isn't one
    pub alloc_console: bool,
    /// Force no colors in console output
    ///
    /// Needed on Wine - it detects the console window as supporting colors while it doesn't
    /// and the ANSI color codes are not processed, just printed as-is.
    pub disable_colors: bool,
}

impl Default for Loading {
    fn default() -> Self {
        Loading {
            load_paths: vec!["xnya_*.dll".to_string()],
            change_workdir: false,
            delayed_load: false,
            skip_missing_symbols: false,
        }
    }
}
