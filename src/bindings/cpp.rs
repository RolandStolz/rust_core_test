//! C++ interop via [`cxx`].
//!
//! The structs declared inside the `#[cxx::bridge]` module below are *shared*
//! types: `cxx` generates both the Rust definition and a matching C++ struct, so
//! a `cr_core::Point` / `State` / `Lanelet` can be constructed and read on either
//! side and passed across the boundary by value.
//!
//! They mirror the plain-Rust core types in [`crate`]; `From` impls convert
//! between the two so the core logic stays free of any FFI concerns (the same way
//! [`super::python`] wraps them for PyO3).

#[cxx::bridge(namespace = "cr_core")]
pub mod ffi {
    /// Mirror of [`crate::Point`].
    #[derive(Clone, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    /// Mirror of [`crate::State`].
    #[derive(Clone, Debug)]
    struct State {
        position: Point,
        orientation: f64,
        velocity: f64,
        time: usize,
    }

    /// Mirror of [`crate::Lanelet`].
    #[derive(Clone, Debug)]
    struct Lanelet {
        left_bound: Vec<Point>,
        right_bound: Vec<Point>,
        id: usize,
    }

    extern "Rust" {
        /// Rust constructor exposed to C++ (matches the Python binding).
        fn create_dummy_lanelet() -> Lanelet;
    }

    unsafe extern "C++" {
        include!("cr_core/demo.h");

        /// Example C++ consumer of the shared structs: averages every bound
        /// point of a `Lanelet`. Implemented in `cpp/demo.cc`.
        fn lanelet_centroid(lanelet: &Lanelet) -> Point;
    }
}

use crate::{Lanelet, Point, State};

impl From<Point> for ffi::Point {
    fn from(p: Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<ffi::Point> for Point {
    fn from(p: ffi::Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<State> for ffi::State {
    fn from(s: State) -> Self {
        Self {
            position: s.position.into(),
            orientation: s.orientation,
            velocity: s.velocity,
            time: s.time,
        }
    }
}

impl From<ffi::State> for State {
    fn from(s: ffi::State) -> Self {
        Self {
            position: s.position.into(),
            orientation: s.orientation,
            velocity: s.velocity,
            time: s.time,
        }
    }
}

impl From<Lanelet> for ffi::Lanelet {
    fn from(l: Lanelet) -> Self {
        Self {
            left_bound: l.left_bound.into_iter().map(Into::into).collect(),
            right_bound: l.right_bound.into_iter().map(Into::into).collect(),
            id: l.id,
        }
    }
}

impl From<ffi::Lanelet> for Lanelet {
    fn from(l: ffi::Lanelet) -> Self {
        Self {
            left_bound: l.left_bound.into_iter().map(Into::into).collect(),
            right_bound: l.right_bound.into_iter().map(Into::into).collect(),
            id: l.id,
        }
    }
}

fn create_dummy_lanelet() -> ffi::Lanelet {
    let left_bound = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)];
    let right_bound = vec![Point::new(0.0, 1.0), Point::new(1.0, 1.0)];
    Lanelet::new(left_bound, right_bound, 1).into()
}

#[cfg(test)]
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
