//! Oxide-3D Settings, user preferences, and standards configuration.

use directories::ProjectDirs;
use oxide_core::units::UnitSystem;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Error type for settings operations.
#[derive(Debug, Error)]
pub enum SettingsError {
    /// Failure determining standard directories.
    #[error("Could not determine config directory")]
    ConfigDirNotFound,

    /// I/O error reading or writing settings file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error parsing or serializing TOML configuration.
    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),

    /// Error deserializing TOML configuration.
    #[error("TOML deserialize error: {0}")]
    TomlDe(#[from] toml::de::Error),
}

/// Drafting and engineering standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftingStandard {
    /// International Organization for Standardization.
    Iso,
    /// American Society of Mechanical Engineers.
    Asme,
    /// Deutsches Institut für Normung.
    Din,
    /// Japanese Industrial Standards.
    Jis,
}

impl Default for DraftingStandard {
    fn default() -> Self {
        Self::Iso
    }
}

/// Comprehensive user and document settings for Oxide-3D.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideSettings {
    /// Active unit system.
    pub units: UnitSystem,
    /// Active drafting standard.
    pub standard: DraftingStandard,
    /// Dark mode toggle.
    pub dark_mode: bool,
    /// Autosave interval in seconds (0 = disabled).
    pub autosave_interval_secs: u32,
    /// Viewport grid size in active units.
    pub grid_size: f64,
    /// Snapping precision tolerance.
    pub snap_tolerance: f64,
    /// Default startup workbench identifier.
    pub default_workbench: String,
}

impl Default for OxideSettings {
    fn default() -> Self {
        Self {
            units: UnitSystem::MetricMm,
            standard: DraftingStandard::Iso,
            dark_mode: true,
            autosave_interval_secs: 180,
            grid_size: 10.0,
            snap_tolerance: 0.5,
            default_workbench: "Model".to_string(),
        }
    }
}

impl OxideSettings {
    /// Returns the standard path to the `settings.toml` configuration file.
    pub fn config_path() -> Result<PathBuf, SettingsError> {
        let proj_dirs = ProjectDirs::from("com", "oxide-3d", "Oxide-3D")
            .ok_or(SettingsError::ConfigDirNotFound)?;
        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("settings.toml"))
    }

    /// Load settings from disk or return default if file doesn't exist.
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Load settings from standard config path.
    pub fn load() -> Result<Self, SettingsError> {
        let path = Self::config_path()?;
        Self::load_from_path(&path)
    }

    /// Load settings from a specific path.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, SettingsError> {
        let content = fs::read_to_string(path)?;
        let settings: Self = toml::from_str(&content)?;
        Ok(settings)
    }

    /// Save current settings to standard config path.
    pub fn save(&self) -> Result<(), SettingsError> {
        let path = Self::config_path()?;
        self.save_to_path(&path)
    }

    /// Save current settings to a specific path.
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), SettingsError> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }
}
