#[cfg(test)]
mod unit_tests;
//=====================================================================
use pyo3::exceptions::{self, PyException};
use pyo3::prelude::*;
use pyo3::types;
//=====================================================================
use lazy_static::lazy_static;
use std::collections::HashMap;
//=====================================================================
mod captures;
mod custom_error;
mod exception_container;
mod rule;
mod template_validator;
mod validate;
// ============================= CONST ================================

// имя модуля для `Python`
pub const MODULE_NAME: &str = "pystval";
// имя атрибута, где хранится само сообщение и **extra переменные из класса Python
pub const MESSAGE_WITH_EXTRA_FROM_CLASS_PY: &str = "message";
// имя атрибута, где хранится регулярные выражения из класса Python
pub const RULES_FROM_CLASS_PY: &str = "rules";
// имя атрибута, где хранится **extra переменные для заполнения из класса Python
pub const EXTRA_FROM_CLASS_PY: &str = "extra";
// имя класса ошибки (базовый шаблон) для `Python`
pub const BASE_ERROR: &str = "PystvalError";

// =====================================================================

// Импортируем всё необходимое в `Python`
#[cfg(not(tarpaulin_include))]
mod export {
    use super::*;
    #[pymodule]
    pub fn pystval(_py: Python<'_>, py_module: &PyModule) -> PyResult<()> {
        // py_module.add_class::<rule::Rule>()?;
        // py_module.add_class::<rule::MatchRequirement>()?;
        Ok(())
    }
}
