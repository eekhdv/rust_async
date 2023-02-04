pub struct ScreenDimensions {
    pub width: u32,
    pub height: u32,
}

impl ScreenDimensions {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}
