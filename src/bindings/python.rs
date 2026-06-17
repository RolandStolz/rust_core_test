#[pyo3::pymodule]
pub mod cr_core {
    use crate::{Lanelet, Point, State};
    use pyo3::{prelude::*, pyclass, pymethods};

    #[pyclass(name = "Point")]
    #[derive(Clone)]
    struct PyPoint(Point);

    #[pymethods]
    impl PyPoint {
        #[new]
        fn new(x: f64, y: f64) -> Self {
            Self(Point::new(x, y))
        }

        #[getter]
        fn x(&self) -> f64 {
            self.0.x
        }

        #[setter]
        fn set_x(&mut self, val: f64) {
            self.0.x = val;
        }

        #[getter]
        fn y(&self) -> f64 {
            self.0.y
        }

        #[setter]
        fn set_y(&mut self, val: f64) {
            self.0.y = val;
        }
    }

    #[pyclass(name = "State")]
    #[derive(Clone)]
    struct PyState(State);

    #[pymethods]
    impl PyState {
        #[new]
        fn new(position: PyPoint, orientation: f64, velocity: f64, time: usize) -> Self {
            Self(State::new(position.0, orientation, velocity, time))
        }
    }

    #[pyclass(name = "Lanelet")]
    #[derive(Clone)]
    struct PyLanelet(Lanelet);

    #[pymethods]
    impl PyLanelet {
        #[new]
        fn new(left_bound: Vec<PyPoint>, right_bound: Vec<PyPoint>, id: usize) -> Self {
            let left_bound = left_bound.into_iter().map(|p| p.0).collect();
            let right_bound = right_bound.into_iter().map(|p| p.0).collect();
            Self(Lanelet::new(left_bound, right_bound, id))
        }
    }

    #[pyfunction]
    fn create_dummy_lanelet() -> PyLanelet {
        let left_bound = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)];
        let right_bound = vec![Point::new(0.0, 1.0), Point::new(1.0, 1.0)];
        PyLanelet(Lanelet::new(left_bound, right_bound, 1))
    }
}
