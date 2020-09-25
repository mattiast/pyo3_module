use ndarray::ArrayViewMut1;
use numpy::PyArray1;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

#[pyfunction]
/// add 5 to a nonnegative integer
pub fn add5(x: u32) -> PyResult<u32> {
    Ok(x + 5)
}

fn my_cumsum(x: &mut ArrayViewMut1<f64>) {
    let mut s: f64 = 0.0;
    for xi in x.iter_mut() {
        s += *xi;
        *xi = s;
    }
}

#[pyfunction]
pub fn cumsum_inplace(x: &PyArray1<f64>) {
    let mut x: ArrayViewMut1<f64> = unsafe { x.as_array_mut() };
    my_cumsum(&mut x);
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

/// Random coffee machine gives U(0,1) cups of coffee, how many times we need
/// to press the button?
#[pyfunction]
pub fn ev_presses(x: f64, n_sims: usize) -> PyResult<f64> {
    let total: f64 = (0..n_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            draw_presses(x, &mut rng) as f64
        })
        .sum();

    Ok(total / n_sims as f64)
}

#[pyclass]
struct Juttu {
    #[pyo3(get)]
    x: i32,
    y: bool,
}

#[pymethods]
impl Juttu {
    #[new]
    fn new(x: i32, y: bool) -> Self {
        Juttu { x, y }
    }

    fn xsq(&self) -> PyResult<i32> {
        Ok(self.x * self.x)
    }

    #[getter]
    fn is_juttu(&self) -> bool {
        self.y
    }
}

#[pyproto]
impl PyObjectProtocol for Juttu {
    fn __repr__(&self) -> String {
        format!("Juttu({},{})", self.x, self.y)
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sample_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add5))?;
    m.add_wrapped(wrap_pyfunction!(cumsum_inplace))?;
    m.add_wrapped(wrap_pyfunction!(ev_presses))?;
    m.add_class::<Juttu>()?;

    Ok(())
}
