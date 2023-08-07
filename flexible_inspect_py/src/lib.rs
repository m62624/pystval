mod cartridges;
mod error;
mod rules;
mod template_validator;
pub use cartridges::{cartridges_bytes::PyCartridgeBytes, cartridges_str::PyCartridge};
use flexible_inspect_rs::prelude::*;
use pyo3::prelude::*;
pub use rules::PyMatchRequeriment;
pub use rules::{rule_bytes::PyRuleBytes, rule_str::PyRule};
pub use template_validator::validate_bytes::PyTemplateValidatorBytes;
pub use template_validator::validate_str::PyTemplateValidator;

#[cfg(not(tarpaulin_include))]
#[pymodule]
pub fn flexible_inspect_py(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
    py_module.add_class::<PyMatchRequeriment>()?;
    py_module.add_class::<PyRule>()?;
    py_module.add_class::<PyRuleBytes>()?;
    py_module.add_class::<PyCartridge>()?;
    py_module.add_class::<PyCartridgeBytes>()?;
    py_module.add_class::<PyTemplateValidator>()?;
    py_module.add_class::<PyTemplateValidatorBytes>()?;
    Ok(())
}
