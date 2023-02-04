pub enum DbusChannel {
    Notify { notification: Notification },
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rect {
    left_up: (i32, i32),
    right_down: (i32, i32),
}

impl Rect {
    pub fn new(left_up: (i32, i32), right_down: (i32, i32)) -> Self {
        Self {
            left_up: (left_up),
            right_down: (right_down),
        }
    }
}

#[derive(Debug)]
pub struct Notification {
    pub app_name: String,
    pub app_icon: String,
    pub title: String,
    pub body: String,
    pub expire_timeout: i32,
    pub window: Rect,
}
