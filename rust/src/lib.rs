use pyo3::prelude::*;
pub mod myrustmodule;

#[pyfunction]
fn py_factorial(a: i64) -> PyResult<i64> {
    Ok(myrustmodule::factorial(a))
}

#[pymodule]
fn rusty(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_factorial, m)?)?;
    Ok(())
}
