//! `cr_core` — the Rust core.
//!
//! The data structures `Point`/`State`/`Lanelet` are defined **once**, as cxx shared
//! structs in [`bindings::cpp`], and re-exported here. That single definition serves
//! every consumer: the PyO3 wrappers ([`bindings::python`]), the zero-copy interop
//! (`interop_test/`), and C++ (cxx generates a matching struct + header). No parallel
//! definitions, no conversions.

mod bindings;

pub use bindings::cpp::ffi::{Lanelet, Point, State};

/// `PyCapsule` names act as the cross-module **type tag** for the zero-copy handoff
/// in `interop_test`: `PyCapsule_GetPointer` validates the name and refuses a
/// mismatched capsule. CPython does not copy the name, so it must be `'static`;
/// producer (this crate built with `python`) and consumer (this crate as a plain
/// dependency) reference these same consts, guaranteeing the names match.
pub const POINT_CAPSULE: &core::ffi::CStr = c"cr_core.Point";
pub const STATE_CAPSULE: &core::ffi::CStr = c"cr_core.State";
pub const LANELET_CAPSULE: &core::ffi::CStr = c"cr_core.Lanelet";
