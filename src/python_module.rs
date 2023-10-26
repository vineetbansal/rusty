#![cfg(feature = "extension-module")]
use pyo3::prelude::*;

use crate::{myrustmodule};

#[pyfunction]
fn py_factorial(a: i64) -> PyResult<i64> {
    Ok(myrustmodule::factorial(a))
}

#[pyclass]
struct PyBedReader {
    x: u8
}

#[pymethods]
impl PyBedReader {
    #[new]
    pub fn new() -> PyBedReader { PyBedReader {x: 42} }

    pub fn foo(&self) -> u8 {
        &self.x + 1
    }
}

#[pymodule]
fn rusty(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_factorial, m)?)?;
    m.add_class::<PyBedReader>()?;
    Ok(())
}
