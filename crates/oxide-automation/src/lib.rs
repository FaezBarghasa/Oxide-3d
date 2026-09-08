//! Oxide-3D Automation, Macro Recorder, and Command Logging.

use oxide_core::command::OxideCommand;
use serde::{Deserialize, Serialize};

/// Recorder that captures commands for macro generation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MacroRecorder {
    /// Recorded command stream.
    pub recorded_commands: Vec<OxideCommand>,
    /// Recording active flag.
    pub is_recording: bool,
}

impl MacroRecorder {
    /// Start recording commands.
    pub fn start(&mut self) {
        self.recorded_commands.clear();
        self.is_recording = true;
    }

    /// Stop recording commands.
    pub fn stop(&mut self) {
        self.is_recording = false;
    }

    /// Record a dispatched command.
    pub fn record(&mut self, cmd: OxideCommand) {
        if self.is_recording {
            self.recorded_commands.push(cmd);
        }
    }
}
