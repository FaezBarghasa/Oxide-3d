//! AutoCAD-Style Interactive Command Prompt & Command Aliases.

use serde::{Deserialize, Serialize};

/// Command Prompt Execution Result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandPromptResult {
    Executed { command: String, primary: String },
    Prompting(String),
    Unknown(String),
    Empty,
}

/// AutoCAD-Style Command Prompt Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPromptModel {
    pub input_text: String,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub prompt_message: String,
    pub active_command: Option<String>,
}

impl Default for CommandPromptModel {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            history: Vec::new(),
            history_index: None,
            prompt_message: "Type a command (e.g. LINE, CIRCLE, EXT, M, C)...".to_string(),
            active_command: None,
        }
    }
}

impl CommandPromptModel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve command aliases (AutoCAD & OpenCADStudio standard).
    #[must_use]
    pub fn resolve_alias(input: &str) -> &'static str {
        match input.trim().to_uppercase().as_str() {
            "L" | "LINE" => "LINE",
            "PL" | "PLINE" | "POLYLINE" => "PLINE",
            "C" | "CIRCLE" => "CIRCLE",
            "A" | "ARC" => "ARC",
            "REC" | "RECTANG" | "RECTANGLE" => "RECTANGLE",
            "POL" | "POLYGON" => "POLYGON",
            "EL" | "ELLIPSE" => "ELLIPSE",
            "SPL" | "SPLINE" => "SPLINE",
            "H" | "HATCH" => "HATCH",
            "T" | "MT" | "MTEXT" | "TEXT" => "MTEXT",
            "M" | "MOVE" => "MOVE",
            "CO" | "CP" | "COPY" => "COPY",
            "RO" | "ROTATE" => "ROTATE",
            "SC" | "SCALE" => "SCALE",
            "TR" | "TRIM" => "TRIM",
            "EX" | "EXTEND" => "EXTEND",
            "F" | "FILLET" => "FILLET",
            "CHA" | "CHAMFER" => "CHAMFER",
            "O" | "OFFSET" => "OFFSET",
            "MI" | "MIRROR" => "MIRROR",
            "AR" | "ARRAY" => "ARRAY",
            "X" | "EXPLODE" => "EXPLODE",
            "E" | "ERASE" | "DELETE" => "ERASE",
            "Z" | "ZOOM" => "ZOOM",
            "P" | "PAN" => "PAN",
            "LA" | "LAYER" => "LAYER",
            "B" | "BLOCK" => "BLOCK",
            "I" | "INSERT" => "INSERT",
            "DIM" | "DIMENSION" => "DIMENSION",
            "EXT" | "EXTRUDE" => "EXTRUDE",
            "REV" | "REVOLVE" => "REVOLVE",
            "SW" | "SWEEP" => "SWEEP",
            "LOFT" => "LOFT",
            "UNI" | "UNION" => "UNION",
            "SU" | "SUBTRACT" => "SUBTRACT",
            "IN" | "INTERSECT" => "INTERSECT",
            _ => "UNKNOWN",
        }
    }

    /// Set input text.
    pub fn set_input(&mut self, text: &str) {
        self.input_text = text.to_string();
    }

    /// Submit input and execute.
    pub fn submit(&mut self) -> CommandPromptResult {
        let trimmed = self.input_text.trim().to_string();
        self.input_text.clear();
        if trimmed.is_empty() {
            return CommandPromptResult::Empty;
        }

        self.history.push(trimmed.clone());
        let canonical = Self::resolve_alias(&trimmed);
        if canonical == "UNKNOWN" {
            self.prompt_message = format!("Unknown command \"{trimmed}\". Type ? for help.");
            CommandPromptResult::Unknown(trimmed)
        } else {
            self.active_command = Some(canonical.to_string());
            self.prompt_message = format!("{canonical}: Specify first point or [Options]:");
            CommandPromptResult::Executed {
                command: trimmed,
                primary: canonical.to_string(),
            }
        }
    }
}
