//! Oxide-3D Settings, user preferences, and standards configuration.

use oxide_core::units::UnitSystem;
use serde::{Deserialize, Serialize};

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
}

impl Default for OxideSettings {
    fn default() -> Self {
        Self {
            units: UnitSystem::MetricMm,
            standard: DraftingStandard::Iso,
            dark_mode: true,
            autosave_interval_secs: 180,
        }
    }
}
