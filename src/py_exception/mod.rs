use super::*;
pub mod extra_collection;
pub use extra_collection as extra;
use pyo3::exceptions::PyException;
use pyo3::types;
use std::collections::HashMap;
mod unit_tests;
/// Создаем ошибку с переданными параметрами
pub fn init_error(obj: &PyObject, extra_hm: Option<HashMap<&str, &str>>) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        // dbg!(&extra_hm);
        // Создаем объект класса ошибки с переданными параметрами
        let extra = types::PyDict::new(py);
        if let Some(extra_hm) = extra_hm {
            for (key, value) in extra_hm {
                extra.set_item(key, value)?;
            }
        }
        let obj = obj.downcast::<types::PyType>(py)?;
        obj.setattr(EXTRA_FROM_CLASS_PY, extra)?;
        let obj = obj
            .downcast::<PyAny>()?
            .call(types::PyTuple::empty(py), Some(extra))?;
        // Создаем объект класса & Возвращаем ошибку
        Err(PyErr::new::<PyException, _>(obj.to_object(py)))
    })
}
