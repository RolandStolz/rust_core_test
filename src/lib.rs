#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lanelet {
    pub left_bound: Vec<Point>,
    pub right_bound: Vec<Point>,
    pub id: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct State {
    pub position: Point,
    pub orientation: f64,
    pub velocity: f64,
    pub time: usize,
}


#[cfg(feature = "python")]
mod bindings;
