use pyo3::exceptions::PyOverflowError;
use pyo3::prelude::*;

/// Fibonacci module
#[pymodule]
mod fibonacci {
    use super::*;

    /// Iterator generating the n-th Fibonacci number
    #[pyclass]
    pub struct Fibonacci {
        curr: usize,
        pub next: Option<usize>,
    }

    #[pymethods]
    impl Fibonacci {
        #[new]
        #[pyo3(signature = (start=0))]
        pub fn new(start: usize) -> PyResult<Self> {
            let mut elem = Fibonacci {
                curr: 0,
                next: Some(1),
            };

            for _ in 0..start {
                elem.next();
            }

            match elem.next {
                Some(_) => Ok(elem),
                None => Err(PyOverflowError::new_err("Integer overflow")),
            }
        }

        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<usize> {
            match slf.next() {
                Some(val) => Ok(val),
                None => Err(PyOverflowError::new_err("Integer overflow")),
            }
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

#[cfg(test)]
mod tests {
    use super::fibonacci::Fibonacci;
    use super::*;

    #[test]
    fn test_fibonacci() -> Result<(), PyErr> {
        assert!(Fibonacci::new(0)?.next.unwrap() == 1);
        assert!(Fibonacci::new(1)?.next.unwrap() == 1);
        assert!(Fibonacci::new(2)?.next.unwrap() == 2);
        assert!(Fibonacci::new(3)?.next.unwrap() == 3);
        assert!(Fibonacci::new(4)?.next.unwrap() == 5);
        Ok(())
    }
}
