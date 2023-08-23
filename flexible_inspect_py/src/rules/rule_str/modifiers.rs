use super::*;

#[pymethods]
impl PyRule {
    pub fn extend(&mut self, nested_rules: Vec<PyRule>) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        let nested_rules = nested_rules
            .into_iter()
            .map(|rule| rule.try_into())
            .collect::<PyResult<Vec<Rule>>>()?;
        mem_self.0 = mem_self.0.map(|rule| rule.extend(nested_rules));
        Ok(mem_self)
    }

    pub fn counter_is_equal(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.counter_is_equal(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.counter_more_than(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.counter_less_than(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }

    pub fn all_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.all_r_for_any_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }

    pub fn any_r_for_all_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.any_r_for_all_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }

    pub fn any_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRule = self.try_into()?;
        mem_self.0 = Some(
            mem_self
                .0
                .map(|rule| rule.any_r_for_any_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(mem_self)
    }
}
