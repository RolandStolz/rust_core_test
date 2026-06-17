
#[pyo3::pymodule]
pub mod cr_core {
    use pyo3::{prelude::*, pyclass};
    use crate::{Lanelet, Point, State};


    #[pyclass(name = "Point")]
    #[derive(Clone)]
    struct PyPoint(Point);

    #[pyclass(name = "State")]
    #[derive(Clone)]
    struct PyState(State);

    #[pyclass(name = "Lanelet")]
    #[derive(Clone)]
    struct PyLanelet(Lanelet);
}
