#[pyo3::pymodule]
pub mod cr_core {
    use crate::{LANELET_CAPSULE, Lanelet, POINT_CAPSULE, Point, STATE_CAPSULE, State};
    use core::ffi::CStr;
    use pyo3::{prelude::*, pyclass, pymethods};
    use std::os::raw::c_void;

    /// Wrap a raw pointer to an existing struct in a named capsule (no copy, no
    /// ownership). The name is the cross-module type tag; see [`crate::POINT_CAPSULE`].
    ///
    /// SAFETY: `ptr` must point at a live `T` that outlives every use of the capsule.
    /// Here it points into a pyclass's storage, which is stable and lives as long as
    /// the owning Python object.
    unsafe fn capsule_for<'py, T>(
        py: Python<'py>,
        ptr: *const T,
        name: &'static CStr,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw = unsafe { pyo3::ffi::PyCapsule_New(ptr as *mut c_void, name.as_ptr(), None) };
        unsafe { Bound::from_owned_ptr_or_err(py, raw) }
    }

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

        /// Address of the inner `Point` (for the zero-copy equality check).
        fn _addr(&self) -> usize {
            &self.0 as *const Point as usize
        }

        /// Named capsule over the inner `Point` — the zero-copy cross-module handoff.
        fn _cr_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            unsafe { capsule_for(py, &self.0 as *const Point, POINT_CAPSULE) }
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

        fn _addr(&self) -> usize {
            &self.0 as *const State as usize
        }

        fn _cr_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            unsafe { capsule_for(py, &self.0 as *const State, STATE_CAPSULE) }
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

        #[getter]
        fn id(&self) -> usize {
            self.0.id
        }

        #[setter]
        fn set_id(&mut self, val: usize) {
            self.0.id = val;
        }

        fn _addr(&self) -> usize {
            &self.0 as *const Lanelet as usize
        }

        fn _cr_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            unsafe { capsule_for(py, &self.0 as *const Lanelet, LANELET_CAPSULE) }
        }
    }

    #[pyfunction]
    fn create_dummy_lanelet() -> PyLanelet {
        let left_bound = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)];
        let right_bound = vec![Point::new(0.0, 1.0), Point::new(1.0, 1.0)];
        PyLanelet(Lanelet::new(left_bound, right_bound, 1))
    }
}
