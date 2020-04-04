use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// add 5 to a nonnegative integer
pub fn add5(x: u32) -> PyResult<u32> {
    Ok(x + 5)
}

#[pyfunction]
pub fn cumsum_inplace(x: &PyArrayDyn<f64>) -> PyResult<()> {
    let mut x = x.as_array_mut();

    let mut s: f64 = 0.0;
    for xi in x.iter_mut() {
        s += *xi;
        *xi = s;
    }

    Ok(())
}

#[pyclass]
struct Juttu {
    x: i32,
    y: bool,
}

#[pymethods]
impl Juttu {
    #[new]
    fn new(x: i32, y: bool) -> Self {
        let juttu = Juttu { x, y };
        juttu
    }

    fn xsq(&self) -> PyResult<i32> {
        Ok(self.x * self.x)
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sample_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add5))?;
    m.add_wrapped(wrap_pyfunction!(cumsum_inplace))?;
    m.add_class::<Juttu>()?;

    Ok(())
}
