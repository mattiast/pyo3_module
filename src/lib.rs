use ndarray::ArrayViewMut1;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use rand::Rng;
use rand_pcg::Pcg64;
use rayon::prelude::*;

#[derive(Debug)]
pub enum MyError {
    TooLow,
}

impl From<MyError> for pyo3::PyErr {
    fn from(error: MyError) -> Self {
        pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", error))
    }
}

#[pyfunction]
/// add 5 to a nonnegative integer
pub fn add5(x: u32) -> Result<u32, MyError> {
    if x > 3 {
        Ok(x + 5)
    } else {
        Err(MyError::TooLow)
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
/// Replace values in an array by cumulative sum, in place
pub fn cumsum_inplace(x: &Bound<PyArray1<f64>>) {
    let mut y: ArrayViewMut1<f64> = unsafe { x.as_array_mut() };
    my_cumsum(&mut y);
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
pub fn ev_presses(x: f64, n_sims: usize) -> f64 {
    let seed = 5;

    let total: f64 = (0..n_sims)
        .into_par_iter()
        .fold_with(
            0.0,
            |acc, i| {
                let mut rng: Pcg64 = rand_seeder::Seeder::from((seed, i)).make_rng();
                let result = draw_presses(x, &mut rng) as f64;
                acc + result
            },
        )
        .sum();

    total / n_sims as f64
}

#[pyclass]
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

    /// Get square of x
    fn xsq(&self) -> i32 {
        self.x * self.x
    }

    #[getter]
    /// Tells whether y is true or not
    fn is_juttu(&self) -> bool {
        self.y
    }

    fn __repr__(&self) -> String {
        format!("Juttu({},{})", self.x, self.y)
    }
}

#[pyfunction]
#[pyo3(name = "kuus")]
fn add6(x: u32) -> u32 {
    x + 6
}

/// This is a submodule
#[pymodule]
fn subi(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add6))?;

    Ok(())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn sample_module(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add5))?;
    m.add_wrapped(wrap_pyfunction!(cumsum_inplace))?;
    m.add_wrapped(wrap_pyfunction!(ev_presses))?;
    m.add_class::<Juttu>()?;

    m.add_wrapped(wrap_pymodule!(subi))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{add6, ev_presses};

    #[test]
    fn test_foo() {
        assert_eq!(add6(4), 10);
    }

    #[test]
    fn test_ev() {
        let x = ev_presses(1.5, 400);
        assert!(x > 0.0);
    }
}
