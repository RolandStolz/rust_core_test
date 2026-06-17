#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lanelet {
    pub left_bound: Vec<Point>,
    pub right_bound: Vec<Point>,
    pub id: usize,
}

impl Lanelet {
    pub fn new(left_bound: Vec<Point>, right_bound: Vec<Point>, id: usize) -> Self {
        Self {
            left_bound,
            right_bound,
            id,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct State {
    pub position: Point,
    pub orientation: f64,
    pub velocity: f64,
    pub time: usize,
}

impl State {
    pub fn new(position: Point, orientation: f64, velocity: f64, time: usize) -> Self {
        Self {
            position,
            orientation,
            velocity,
            time,
        }
    }
}

#[cfg(feature = "python")]
mod bindings;
