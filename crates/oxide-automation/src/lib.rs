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
    /// Create a new macro recorder.
    pub fn new() -> Self {
        Self::default()
    }

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

    /// Export recorded commands as an executable Python automation script.
    pub fn export_python_script(&self) -> String {
        let mut script = String::new();
        script.push_str("#!/usr/bin/env python3\n");
        script.push_str("# Auto-generated Oxide-3D Macro Automation Script\n");
        script.push_str("import oxide\n\n");
        script.push_str("app = oxide.Application.get_active()\n");
        script.push_str("doc = app.active_document()\n\n");

        for cmd in &self.recorded_commands {
            match cmd {
                OxideCommand::NewDocument => {
                    script.push_str("doc = app.new_document()\n");
                }
                OxideCommand::CreateExtrude { distance, .. } => {
                    script.push_str(&format!(
                        "doc.create_extrude(distance={:.4})\n",
                        distance
                    ));
                }
                OxideCommand::CreateFillet { radius, .. } => {
                    script.push_str(&format!(
                        "doc.create_fillet(radius={:.4})\n",
                        radius
                    ));
                }
                OxideCommand::SaveDocument { path } => {
                    if let Some(p) = path {
                        script.push_str(&format!(
                            "doc.save(path={:?})\n",
                            p.display().to_string()
                        ));
                    } else {
                        script.push_str("doc.save()\n");
                    }
                }
                OxideCommand::ExportStep { path } => {
                    script.push_str(&format!(
                        "doc.export_step(path={:?})\n",
                        path.display().to_string()
                    ));
                }
                OxideCommand::Undo => {
                    script.push_str("doc.undo()\n");
                }
                OxideCommand::Redo => {
                    script.push_str("doc.redo()\n");
                }
                _ => {
                    script.push_str(&format!("# Executed command: {:?}\n", cmd));
                }
            }
        }

        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::id::EntityKey;

    #[test]
    fn test_macro_recorder_and_python_export() {
        let mut recorder = MacroRecorder::new();
        recorder.start();

        recorder.record(OxideCommand::NewDocument);
        recorder.record(OxideCommand::CreateExtrude {
            profile: EntityKey::default(),
            distance: 25.4,
        });
        recorder.record(OxideCommand::CreateFillet {
            edges: vec![],
            radius: 2.5,
        });

        recorder.stop();
        assert_eq!(recorder.recorded_commands.len(), 3);

        let py_script = recorder.export_python_script();
        assert!(py_script.contains("app.new_document()"));
        assert!(py_script.contains("doc.create_extrude(distance=25.4000)"));
        assert!(py_script.contains("doc.create_fillet(radius=2.5000)"));
    }
}
