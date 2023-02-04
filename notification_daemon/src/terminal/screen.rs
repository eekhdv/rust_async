pub struct ScreenDimensions {
    pub width: i32,
    pub height: i32,
}

impl ScreenDimensions {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}
