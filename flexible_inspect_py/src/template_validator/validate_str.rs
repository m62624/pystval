use super::*;
use crate::error::PyValidationErrorIterator;
use std::sync::Arc;

#[allow(clippy::type_complexity)]
#[pyclass(name = "TemplateValidator")]
pub struct PyTemplateValidator(Arc<TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>>);

#[pymethods]
impl PyTemplateValidator {
    #[new]
    pub fn new(cartridges: Vec<PyCartridge>) -> Self {
        Self(Arc::new(TemplateValidator::new(
            cartridges
                .into_iter()
                .map(|cartridge| cartridge.into())
                .collect(),
        )))
    }

    pub fn validate(&self, data: String) -> PyValidationErrorIterator {
        if let Err(value) = self.0.validate(data.into()) {
            return PyValidationErrorIterator::new(value.into_iter().collect());
        }
        PyValidationErrorIterator::new(vec![])
    }

    #[cfg(not(tarpaulin_include))]
    pub fn async_validate<'py>(&self, py: Python<'py>, data: String) -> PyResult<&'py PyAny> {
        let safety_self = Arc::clone(&self.0);
        let safety_data = Arc::from(data);
        pyo3_asyncio::async_std::future_into_py(py, async move {
            if let Err(value) = safety_self.async_validate(safety_data).await {
                return Ok(PyValidationErrorIterator::new(value.into_iter().collect()));
            }
            Ok(PyValidationErrorIterator::new(vec![]))
        })
    }
}
