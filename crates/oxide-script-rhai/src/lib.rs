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
    let mut engine = Engine::new();

    // Register standard math helpers
    engine.register_fn("sin", |x: f64| x.sin());
    engine.register_fn("cos", |x: f64| x.cos());
    engine.register_fn("tan", |x: f64| x.tan());
    engine.register_fn("sqrt", |x: f64| x.sqrt());
    engine.register_fn("abs", |x: f64| x.abs());
    engine.register_fn("pow", |x: f64, p: f64| x.powf(p));

    let mut scope = Scope::new();
    scope.push("PI", std::f64::consts::PI);
    scope.push("E", std::f64::consts::E);

    for (name, val) in variables {
        scope.push(*name, *val);
    }
    engine
        .eval_with_scope::<f64>(&mut scope, expr)
        .map_err(|e: Box<EvalAltResult>| RhaiError::Evaluation(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_parametric_dimensions() {
        let vars = [("width", 50.0), ("thickness", 3.0), ("hole_count", 4.0)];
        let res = eval_expression("width * 2.0 + thickness * hole_count", &vars).unwrap();
        assert_eq!(res, 112.0);

        let trig = eval_expression("sin(PI / 2.0) + sqrt(16.0)", &[]).unwrap();
        assert!((trig - 5.0).abs() < 1e-6);
    }
}
