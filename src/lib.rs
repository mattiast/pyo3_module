use eyre::{eyre, Result};
use ndarray::ArrayViewMut1;
use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{class::basic::PyObjectProtocol, wrap_pymodule};
use rand::Rng;

use rand_core::SeedableRng;
use rand_pcg::Pcg64;
use rayon::prelude::*;

#[pyfunction]
#[text_signature = "(x)"]
/// add 5 to a nonnegative integer
pub fn add5(x: u32) -> Result<u32> {
    if x > 3 {
        Ok(x + 5)
    } else {
        Err(eyre!("Too low number"))
    }
}

fn my_cumsum(x: &mut ArrayViewMut1<f64>) {
    let mut s: f64 = 0.0;
    for xi in x.iter_mut() {
        s += *xi;
        *xi = s;
    }
}

#[pyfunction]
#[text_signature = "(x)"]
/// Replace values in an array by cumulative sum, in place
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
#[text_signature = "(x, n_sims)"]
pub fn ev_presses(x: f64, n_sims: usize) -> f64 {
    let mut rng = Pcg64::seed_from_u64(5);

    let seeds: Vec<<Pcg64 as SeedableRng>::Seed> = (0..n_sims).map(|_| rng.gen()).collect();
    let total: f64 = seeds
        .into_par_iter()
        .map(|seed| {
            let mut rng = Pcg64::from_seed(seed);
            draw_presses(x, &mut rng) as f64
        })
        .sum();

    total / n_sims as f64
}

#[pyclass]
#[text_signature = "(x, y)"]
/// Class representing a thing
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

    #[text_signature = "($self)"]
    /// Get square of x
    fn xsq(&self) -> i32 {
        self.x * self.x
    }

    #[getter]
    /// Tells whether y is true or not
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

#[pyfunction]
#[text_signature = "(x)"]
fn add6(x: u32) -> u32 {
    x + 6
}

/// This is a submodule
#[pymodule]
fn subi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add6))?;

    Ok(())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sample_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add5))?;
    m.add_wrapped(wrap_pyfunction!(cumsum_inplace))?;
    m.add_wrapped(wrap_pyfunction!(ev_presses))?;
    m.add_class::<Juttu>()?;

    m.add_wrapped(wrap_pymodule!(subi))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::add6;

    #[test]
    fn test_foo() {
        assert_eq!(add6(4), 10);
    }
}
