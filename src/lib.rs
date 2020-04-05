use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::{thread_rng, Rng};

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

fn draw_presses<R: Rng>(x: f64, rng: &mut R) -> usize {
    let mut s = 0.0;
    let mut n = 0;

    while s < x {
        n += 1;
        s += rng.gen::<f64>();
    }
    n
}

#[pyfunction]
/// Random coffee machine gives U(0,1) cups of coffee, how many times we need
/// to press the button?
pub fn ev_presses(x: f64, n_sims: usize) -> PyResult<f64> {
    let mut rng = thread_rng();
    let mut s = 0.0;

    for _ in 0..n_sims {
        s += draw_presses(x, &mut rng) as f64;
    }

    Ok(s / n_sims as f64)
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
    m.add_wrapped(wrap_pyfunction!(ev_presses))?;
    m.add_class::<Juttu>()?;

    Ok(())
}
