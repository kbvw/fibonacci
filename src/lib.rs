use pyo3::prelude::*;

/// Fibonacci module
#[pymodule]
mod fibonacci {
    use super::*;

    /// Iterator generating the n-th Fibonacci number
    #[pyclass]
    struct Fibonacci {
        curr: usize,
        next: Option<usize>,
    }

    #[pymethods]
    impl Fibonacci {
        #[new]
        fn new() -> Self {
            Fibonacci {
                curr: 0,
                next: Some(1),
            }
        }

        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<usize> {
            slf.next()
        }
    }

    impl Iterator for Fibonacci {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            match self.next {
                Some(val) => {
                    let next = self.curr.checked_add(val);

                    self.curr = val;
                    self.next = next;

                    Some(self.curr)
                }
                None => None,
            }
        }
    }
}
