/// Render pipeline orchestrator for PBR, Wireframe, and Hidden-Line shaders.
#[derive(Debug, Default)]
pub struct RenderPipelineManager {
    /// Wireframe enabled flag.
    pub wireframe_enabled: bool,
}

impl RenderPipelineManager {
    /// Create a new render pipeline manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
