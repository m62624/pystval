use super::*;

/// Проверка конструктора
#[cfg(test)]
mod fn_new {
    use super::*;

    #[test]
    fn new_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\d", MatchRequirement::MustBeFound)?);
        Ok(())
    }

    #[test]
    fn new_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w", MatchRequirement::MustNotBefound)?);
        Ok(())
    }

    #[test]
    fn new_t_2() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?);
        Ok(())
    }

    #[test]
    fn new_t_3() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustNotBefound)?);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn new_e_0() {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"(?P<invalid)", MatchRequirement::MustNotBefound).unwrap());
    }
}

/// Проверка расширения `Rule`
#[cfg(test)]
mod fn_extend {
    use super::*;

    #[test]
    fn extend_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?;
            let extended_rule = rule.extend_t(
                py,
                vec![
                    Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?,
                ],
            )?;
            dbg!(extended_rule);
            Ok(())
        })
    }

    #[test]
    fn extend_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?;
            let extended_rule = rule.extend_t(
                py,
                vec![Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?],
            )?;
            dbg!(extended_rule);
            Ok(())
        })
    }

    #[test]
    #[should_panic]
    fn extend_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound).unwrap();
            rule.extend(
                py,
                types::PyList::new(
                    py,
                    vec![
                        Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"\w", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        types::PyType::new::<mock_obj::CustomClassError>(py).into(),
                    ],
                )
                .into_py(py),
            )
            .unwrap();
        });
    }
}
#[cfg(test)]
mod fn_get_selected_rules {
    use super::*;

    #[test]
    fn get_selected_rules_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?;
            let extended_rule = rule.extend_t(
                py,
                vec![
                    Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?,
                ],
            )?;
            let sb = extended_rule.get_op_subrules().as_ref().unwrap();
            assert_eq!(
                Rule::get_selected_rules(
                    sb.get_default_rgx_set(),
                    sb.get_default_rgx_vec(),
                    "[qwe] cxa a:D"
                )
                .unwrap()
                .len(),
                2
            );
            Ok(())
        })
    }
}
