//! Oxide-3D WASM Sandboxed Plugin Host (Wasmtime & WASI).

use thiserror::Error;
use wasmtime::Engine;

/// Plugin host errors.
#[derive(Debug, Error)]
pub enum PluginError {
    /// Engine initialization failure.
    #[error("Failed to initialize WASM engine: {0}")]
    Engine(String),

    /// Plugin load error.
    #[error("Plugin load failure: {0}")]
    Load(String),
}

/// WASM Plugin host manager.
pub struct WasmPluginHost {
    /// Wasmtime runtime engine.
    pub engine: Engine,
}

impl std::fmt::Debug for WasmPluginHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmPluginHost").finish_non_exhaustive()
    }
}

impl Default for WasmPluginHost {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create default WasmPluginHost"))
    }
}

impl WasmPluginHost {
    /// Initialize a new WASM plugin host engine.
    pub fn new() -> Result<Self, PluginError> {
        let engine = Engine::default();
        Ok(Self { engine })
    }
}
