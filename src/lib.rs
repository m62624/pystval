//! `Pystval` - `Rust` библиотека для `Python`. Выполняет валидацию данных (строки) с помощью регулярных выражений

mod check;
mod constant;
mod convert;
mod init;
mod unit_tests;

use constant::*;
use pyo3::gc::{PyTraverseError, PyVisit};
use pyo3::{prelude::*, types, ToPyObject};
use std::{collections::HashMap, str};

// Используем разные виды regex для различной сложности выражений
//=============================
use fancy_regex;
use regex;
//=============================

/// Перечечисление, где даны варианты действия при положительном результате регулярных выражений
#[pyclass]
#[derive(Debug, Clone)]
pub enum IfFound {
    DoNothing,
    RaiseError,
}

/// Структура для хранения ошибок и статуса
#[derive(Debug, Clone)]
pub struct ShotStatus {
    error: PyObject,
    status: IfFound,
}

impl ShotStatus {
    pub fn new(error: PyObject, status: IfFound) -> Self {
        Self { error, status }
    }
}

/// Структруа для хранения ошибок и регулярных выражений\
/// Является шаблоном для создание *уникальных* валидаторов
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    // хранит все ошибки ( KEY: `ID` и VALUE: `PyError` )
    python_classes: HashMap<usize, ShotStatus>,
    // хранит default regex ( KEY: `regex (string)` и VALUE: `ID` )
    all_simple_rules: HashMap<String, usize>,
    // хранит fancy regex ( KEY: `regex (string)` и VALUE: `ID` )
    all_hard_rules: HashMap<String, usize>,
    // Собираем все default regex и *одним проходом* проверяем все регулярки
    selected_simple_rules: regex::RegexSet,
}

// Реализация методов для TemplateValidator которые будут доступны в `Python`
#[pymethods]
impl TemplateValidator {
    /// Создаем экземлпяр с заданными параметрами проверки\
    /// Принимает `PyList [class,class,class]`\
    /// **Может принимать сразу `class` без экземпляра**
    #[new]
    pub fn __new__(flags: PyObject) -> PyResult<Self> {
        Ok(Python::with_gil(|py| -> PyResult<Self> {
            let mut python_classes: HashMap<usize, ShotStatus> = HashMap::new();
            let mut all_simple_rules: HashMap<String, usize> = HashMap::new();
            let mut all_hard_rules: HashMap<String, usize> = HashMap::new();
            let mut selected_simple_rules: Vec<String> = Vec::new();
            init::data_unpackaging(
                py,
                flags,
                &mut python_classes,
                &mut all_simple_rules,
                &mut all_hard_rules,
                &mut selected_simple_rules,
            )?;
            Ok(Self {
                python_classes,
                all_simple_rules,
                all_hard_rules,
                selected_simple_rules: regex::RegexSet::new(selected_simple_rules).unwrap(),
            })
        })?)
    }

    //================== (РАБОТАЕТ ТОЛЬКО С `C API` (CPYTHON))==================
    /*
    Метод `__traverse__` используется для рекурсивного обхода объекта и уведомления `Python` о всех вложенных объектах, которые должны быть добавлены в механизм управления памятью `Python`. В этой реализации метод `__traverse__` проходится по всем значениям в хэш-таблице `python_classes` и вызывает `visit.call()` для каждого объекта типа `PyObject` в свойстве `error` объекта типа `ShotStatus`. Это гарантирует, что объекты типа `PyObject` не будут освобождены Python'ом до тех пор, пока они не перестанут использоваться в `Rust`.
     */
    #[cfg(not(tarpaulin_include))]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for shot_status in self.python_classes.values() {
            // Поскольку ShotStatus не является PyClass, мы вызываем visit.call() напрямую
            visit.call(&shot_status.error)?;
        }
        Ok(())
    }

    /*
    Метод `__clear__` используется для освобождения памяти, занятой объектом. В этой реализации метод `__clear__` очищает хэш-таблицу `python_classes`, что приводит к уменьшению счетчика ссылок на каждый объект типа PyObject в свойстве error объекта типа `ShotStatus`. Если количество ссылок на объект достигнет нуля, он будет автоматически удален Python'ом.
     */
    #[cfg(not(tarpaulin_include))]
    fn __clear__(&mut self) {
        // Удаляем все значения из HashMap, тем самым снижая счетчик ссылок на PyObject
        self.python_classes.clear();
    }
    //================== (РАБОТАЕТ ТОЛЬКО С `C API` (CPYTHON))==================
}

// Реализация методов для TemplateValidator которые доступны в Rust
impl TemplateValidator {}

// Импортируем всё необходимое в `Python`
#[cfg(not(tarpaulin_include))]
mod export {
    use super::*;
    #[pymodule]
    fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_class::<TemplateValidator>()?;
        m.add_class::<IfFound>()?;
        Ok(())
    }
}
