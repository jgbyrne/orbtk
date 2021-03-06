/// This struct is used to define a thickness with `left`, `top`, `right` and `bottom`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Thickness {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Thickness {
    /// Creates a new thickness.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Thickness {
            left,
            top,
            right,
            bottom,
        }
    }
}
