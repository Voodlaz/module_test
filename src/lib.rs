use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
pub struct SomeClass {
    aa: usize
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn module_test(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SomeClass>()?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    Ok(())
}
