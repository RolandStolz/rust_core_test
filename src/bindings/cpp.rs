//! Canonical data structures + C++ interop, via [`cxx`].
//!
//! The structs declared inside the `#[cxx::bridge]` module below are the **single
//! source of truth**: cxx generates both the (repr-C, layout-stable) Rust definition
//! and a matching C++ struct. They are re-exported from the crate root (`crate::Point`
//! …), so PyO3 wrappers and the zero-copy interop use the *same* types — no parallel
//! definitions, no `From` conversions.
//!
//! Inherent methods (`Point::new` …) are written in normal `impl` blocks outside the
//! bridge. The `extern "C++"` block is gated behind the `cpp` feature so non-C++
//! builds (pyo3-only, `cargo test`, the `main` bin) don't reference a C++ symbol.

#[cxx::bridge(namespace = "cr_core")]
pub mod ffi {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Copy, Clone)]
    struct State {
        position: Point,
        orientation: f64,
        velocity: f64,
        time: usize,
    }

    #[derive(Debug, Clone)]
    struct Lanelet {
        left_bound: Vec<Point>,
        right_bound: Vec<Point>,
        id: usize,
    }

    extern "Rust" {
        /// Rust constructor exposed to C++ (matches the Python binding).
        fn create_dummy_lanelet() -> Lanelet;
    }

    #[cfg(feature = "cpp")]
    unsafe extern "C++" {
        include!("cr_core/demo.h");

        /// Example C++ consumer of the shared structs: averages every bound
        /// point of a `Lanelet`. Implemented in `cpp/src/demo.cpp`.
        fn lanelet_centroid(lanelet: &Lanelet) -> Point;
    }
}

impl ffi::Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl ffi::State {
    pub fn new(position: ffi::Point, orientation: f64, velocity: f64, time: usize) -> Self {
        Self {
            position,
            orientation,
            velocity,
            time,
        }
    }
}

impl ffi::Lanelet {
    pub fn new(left_bound: Vec<ffi::Point>, right_bound: Vec<ffi::Point>, id: usize) -> Self {
        Self {
            left_bound,
            right_bound,
            id,
        }
    }
}

fn create_dummy_lanelet() -> ffi::Lanelet {
    let left_bound = vec![ffi::Point::new(0.0, 0.0), ffi::Point::new(1.0, 0.0)];
    let right_bound = vec![ffi::Point::new(0.0, 1.0), ffi::Point::new(1.0, 1.0)];
    ffi::Lanelet::new(left_bound, right_bound, 1)
}

#[cfg(all(test, feature = "cpp"))]
mod tests {
    use super::{create_dummy_lanelet, ffi};

    #[test]
    fn cpp_can_read_shared_structs() {
        let lanelet = create_dummy_lanelet();
        // Centroid computed entirely in C++ from the shared struct's fields.
        let c = ffi::lanelet_centroid(&lanelet);
        assert_eq!(c.x, 0.5);
        assert_eq!(c.y, 0.5);
    }
}
