use ndarray::ArrayViewMut1;
use numpy::PyArray1;
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

#[pyfunction]
/// Random coffee machine gives U(0,1) cups of coffee, how many times we need
/// to press the button?
pub fn ev_presses(x: f64, n_sims: usize, n_threads: usize) -> PyResult<f64> {
    let sims_per_thread = n_sims / n_threads;
    let total: f64 = (0..n_threads)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut s = 0.0;

            for _ in 0..sims_per_thread {
                s += draw_presses(x, &mut rng) as f64;
            }
            s
        })
        .sum();

    Ok(total / n_sims as f64)
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

    fn is_juttu(&self) -> bool {
        self.y
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
