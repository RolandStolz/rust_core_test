//! PyO3 *consumer* module: a **separate** extension that borrows the producer's
//! Rust objects zero-copy.
//!
//! It does NOT redefine the structs — it depends on the `cr_core` crate and uses
//! the real `Point`/`State`/`Lanelet` types directly. A named `PyCapsule` carries
//! the pointer from a producer object across the module boundary; the capsule name
//! is validated by `PyCapsule_GetPointer` and acts as the type tag.

#[pyo3::pymodule]
pub mod rust_consumer {
    use core::ffi::CStr;
    use cr_core::{LANELET_CAPSULE, Lanelet, POINT_CAPSULE, Point, STATE_CAPSULE, State};
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use std::os::raw::c_void;

    /// Read the raw pointer out of `obj._cr_capsule()`, validating the capsule name.
    ///
    /// SAFETY of the returned pointer's *use*: it points into `obj`'s storage, which
    /// PyO3 keeps alive for the duration of the call, so a borrow tied to `'py` is
    /// valid. The producer guarantees the pointee is `#[repr(C)]`.
    unsafe fn capsule_ptr<'py>(
        obj: pyo3::Borrowed<'_, 'py, PyAny>,
        name: &'static CStr,
    ) -> PyResult<*mut c_void> {
        let cap = obj.call_method0("_cr_capsule")?;
        let ptr = unsafe { pyo3::ffi::PyCapsule_GetPointer(cap.as_ptr(), name.as_ptr()) };
        if ptr.is_null() {
            // GetPointer set an exception (e.g. wrong capsule name == wrong type).
            return Err(PyErr::take(obj.py())
                .unwrap_or_else(|| PyValueError::new_err("invalid cr_core capsule")));
        }
        Ok(ptr)
    }

    /// Zero-copy borrow of a producer `Point` (lifetime tied to the Python object).
    pub struct PointRef<'py>(&'py Point);
    impl<'py> FromPyObject<'_, 'py> for PointRef<'py> {
        type Error = PyErr;
        fn extract(obj: pyo3::Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
            let ptr = unsafe { capsule_ptr(obj, POINT_CAPSULE)? } as *const Point;
            Ok(Self(unsafe { &*ptr }))
        }
    }

    pub struct StateRef<'py>(&'py State);
    impl<'py> FromPyObject<'_, 'py> for StateRef<'py> {
        type Error = PyErr;
        fn extract(obj: pyo3::Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
            let ptr = unsafe { capsule_ptr(obj, STATE_CAPSULE)? } as *const State;
            Ok(Self(unsafe { &*ptr }))
        }
    }

    pub struct LaneletRef<'py>(&'py Lanelet);
    impl<'py> FromPyObject<'_, 'py> for LaneletRef<'py> {
        type Error = PyErr;
        fn extract(obj: pyo3::Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
            let ptr = unsafe { capsule_ptr(obj, LANELET_CAPSULE)? } as *const Lanelet;
            Ok(Self(unsafe { &*ptr }))
        }
    }

    /// Reads the borrowed `Point` in place; returns the address it dereferenced so
    /// the caller can prove it matches the producer object's address.
    #[pyfunction]
    fn print_point(p: PointRef<'_>) -> usize {
        let pt = p.0;
        println!("[rust_consumer] point = ({}, {}) @ {:p}", pt.x, pt.y, pt);
        pt as *const Point as usize
    }

    #[pyfunction]
    fn print_state(s: StateRef<'_>) -> usize {
        let st = s.0;
        println!(
            "[rust_consumer] state = pos=({}, {}), orientation={}, velocity={}, time={} @ {:p}",
            st.position.x, st.position.y, st.orientation, st.velocity, st.time, st
        );
        st as *const State as usize
    }

    #[pyfunction]
    fn print_lanelet(l: LaneletRef<'_>) -> usize {
        let ll = l.0;
        // Reads the Vec<Point> fields across the module boundary (sound here because
        // both extensions are built with the same toolchain against cr_core).
        print!("[rust_consumer] lanelet id={}, left=[", ll.id);
        for p in &ll.left_bound {
            print!("({},{}) ", p.x, p.y);
        }
        print!("], right=[");
        for p in &ll.right_bound {
            print!("({},{}) ", p.x, p.y);
        }
        println!("]");
        ll as *const Lanelet as usize
    }

    /// Mutates the producer's `Point` *through the shared pointer*. If this is truly
    /// zero-copy, the change is visible from Python afterwards.
    #[pyfunction]
    fn bump_point_x(obj: Bound<'_, PyAny>, dx: f64) -> PyResult<()> {
        let ptr = unsafe { capsule_ptr(obj.as_borrowed(), POINT_CAPSULE)? } as *mut Point;
        unsafe { (*ptr).x += dx };
        Ok(())
    }
}
