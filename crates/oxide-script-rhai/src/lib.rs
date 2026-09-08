//! Oxide-3D Embedded Rhai Expression Engine for Parametric Dimensions & Formulas.

use rhai::{Engine, EvalAltResult, Scope};
use thiserror::Error;

/// Expression evaluation error.
#[derive(Debug, Error)]
pub enum RhaiError {
    /// Evaluation failure.
    #[error("Expression evaluation failed: {0}")]
    Evaluation(String),
}

/// Evaluates a parametric mathematical expression string with given variable scope.
pub fn eval_expression(expr: &str, variables: &[(&str, f64)]) -> Result<f64, RhaiError> {
    let engine = Engine::new();
    let mut scope = Scope::new();
    for (name, val) in variables {
        scope.push(*name, *val);
    }
    engine
        .eval_with_scope::<f64>(&mut scope, expr)
        .map_err(|e: Box<EvalAltResult>| RhaiError::Evaluation(e.to_string()))
}
