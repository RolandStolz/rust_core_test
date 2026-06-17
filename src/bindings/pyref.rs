//! Reusable **consumer-side** glue (`pyref` feature).
//!
//! Any *separate* PyO3 extension that wants to borrow `cr_core` objects (produced by
//! the `python` extension) zero-copy depends on `cr_core` with `features = ["pyref"]`
//! and writes, e.g.:
//!
//! ```ignore
//! use cr_core::PointRef;
//! #[pyfunction]
//! fn f(p: PointRef) { println!("{} {}", p.x, p.y); }  // Deref -> &Point
//! ```
//!
//! No structs and no `FromPyObject` plumbing are re-defined downstream — they live
//! here, once. The borrow works across the module boundary via the producer's named
//! `PyCapsule` (a `#[pyclass]` type can't be shared across separately-compiled
//! extensions; the capsule pointer + name tag is what crosses the boundary).

use core::ffi::CStr;
use core::ops::{Deref, DerefMut};
use std::os::raw::c_void;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::{LANELET_CAPSULE, Lanelet, POINT_CAPSULE, Point, STATE_CAPSULE, State};

/// Pull the raw pointer out of `obj._cr_capsule()`, validating the capsule name
/// (the name is the cross-module type tag).
///
/// SAFETY of the returned pointer's *use*: it points into `obj`'s storage, which
/// PyO3 keeps alive for the duration of the call, so a borrow tied to `'py` is valid.
unsafe fn capsule_ptr(obj: pyo3::Borrowed<'_, '_, PyAny>, name: &'static CStr) -> PyResult<*mut c_void> {
    let cap = obj.call_method0("_cr_capsule")?;
    let ptr = unsafe { pyo3::ffi::PyCapsule_GetPointer(cap.as_ptr(), name.as_ptr()) };
    if ptr.is_null() {
        // GetPointer set an exception (e.g. wrong capsule name == wrong type).
        return Err(PyErr::take(obj.py()).unwrap_or_else(|| PyValueError::new_err("invalid cr_core capsule")));
    }
    Ok(ptr)
}

/// Generates an immutable + mutable zero-copy borrow type for a core struct, each
/// extractable as a `#[pyfunction]` argument and `Deref`-ing to the struct.
macro_rules! cr_ref {
    ($Ref:ident, $RefMut:ident, $T:ty, $cap:expr) => {
        #[doc = concat!("Zero-copy borrow of a `cr_core::", stringify!($T), "` inside a Python object.")]
        pub struct $Ref<'py>(&'py $T);

        impl<'py> FromPyObject<'_, 'py> for $Ref<'py> {
            type Error = PyErr;
            fn extract(obj: pyo3::Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
                let ptr = unsafe { capsule_ptr(obj, $cap)? } as *const $T;
                Ok(Self(unsafe { &*ptr }))
            }
        }
        impl Deref for $Ref<'_> {
            type Target = $T;
            fn deref(&self) -> &$T {
                self.0
            }
        }

        #[doc = concat!("Mutable zero-copy borrow of a `cr_core::", stringify!($T), "` (mutations are visible in Python).")]
        pub struct $RefMut<'py>(&'py mut $T);

        impl<'py> FromPyObject<'_, 'py> for $RefMut<'py> {
            type Error = PyErr;
            fn extract(obj: pyo3::Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
                let ptr = unsafe { capsule_ptr(obj, $cap)? } as *mut $T;
                Ok(Self(unsafe { &mut *ptr }))
            }
        }
        impl Deref for $RefMut<'_> {
            type Target = $T;
            fn deref(&self) -> &$T {
                self.0
            }
        }
        impl DerefMut for $RefMut<'_> {
            fn deref_mut(&mut self) -> &mut $T {
                self.0
            }
        }
    };
}

cr_ref!(PointRef, PointRefMut, Point, POINT_CAPSULE);
cr_ref!(StateRef, StateRefMut, State, STATE_CAPSULE);
cr_ref!(LaneletRef, LaneletRefMut, Lanelet, LANELET_CAPSULE);
