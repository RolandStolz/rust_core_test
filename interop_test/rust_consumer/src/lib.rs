//! Example downstream consumer: a **separate** PyO3 extension that borrows `cr_core`
//! objects zero-copy.
//!
//! Note what's NOT here: no struct definitions, no `FromPyObject`/capsule plumbing.
//! Those live once in `cr_core` (the `pyref` feature). This crate just depends on
//! `cr_core` and writes its functions against the provided `…Ref` types — exactly
//! what any other downstream project would do.

#[pyo3::pymodule]
pub mod rust_consumer {
    use cr_core::{LaneletRef, Point, PointRef, PointRefMut, StateRef};
    use pyo3::prelude::*;

    // The `…Ref` types `Deref` to the core struct, so fields are read directly.
    // Each fn returns the address it dereferenced, so the caller can prove it
    // matches the producer object's address (zero-copy).

    #[pyfunction]
    fn print_point(p: PointRef<'_>) -> usize {
        println!("[rust_consumer] point = ({}, {})", p.x, p.y);
        &*p as *const Point as usize
    }

    #[pyfunction]
    fn print_state(s: StateRef<'_>) -> usize {
        println!(
            "[rust_consumer] state = pos=({}, {}), orientation={}, velocity={}, time={}",
            s.position.x, s.position.y, s.orientation, s.velocity, s.time
        );
        &*s as *const _ as usize
    }

    #[pyfunction]
    fn print_lanelet(l: LaneletRef<'_>) -> usize {
        print!("[rust_consumer] lanelet id={}, left=[", l.id);
        for p in &l.left_bound {
            print!("({},{}) ", p.x, p.y);
        }
        print!("], right=[");
        for p in &l.right_bound {
            print!("({},{}) ", p.x, p.y);
        }
        println!("]");
        &*l as *const _ as usize
    }

    /// Mutates the producer's `Point` through the shared borrow — visible in Python.
    #[pyfunction]
    fn bump_point_x(mut p: PointRefMut<'_>, dx: f64) {
        p.x += dx;
    }
}
