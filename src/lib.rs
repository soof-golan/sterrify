use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn to_string(a: PyObject) -> PyResult<String> {
    Ok((a).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn sterrify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_string, m)?)?;
    Ok(())
}