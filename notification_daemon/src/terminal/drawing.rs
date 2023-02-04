use std::sync::Arc;
use tokio::sync::Mutex;

use crate::dbus::prep_notifications::Notification;

#[derive(Debug, Clone)]
pub struct NotificationBox {
    pub notifications: Arc<Mutex<Vec<Notification>>>,
}

impl NotificationBox {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(Mutex::new(vec![])),
        }
    }
}

pub mod methods {
    use console_engine::rect_style::BorderStyle;
    use console_engine::ConsoleEngine;
    use core::cmp::max;

    const HOR_BORDER_ALIGN: i32 = 2;
    const VER_BORDER_ALIGN: i32 = 0;
    const APP_NAME_SHIFT: i32 = 1;
    const TITLE_SHIFT: i32 = 3;
    const BODY_SHIFT: i32 = 5;

    pub fn print_app_name(engine: &mut ConsoleEngine, left_up: (i32, i32), app_name: &str) {
        engine.print(
            left_up.0 + HOR_BORDER_ALIGN,
            left_up.1 + VER_BORDER_ALIGN + APP_NAME_SHIFT,
            app_name,
        );
    }

    pub fn print_title(engine: &mut ConsoleEngine, left_up: (i32, i32), title: &str) {
        engine.print(
            left_up.0 + HOR_BORDER_ALIGN,
            left_up.1 + VER_BORDER_ALIGN + TITLE_SHIFT,
            title,
        );
    }

    pub fn print_body(engine: &mut ConsoleEngine, left_up: (i32, i32), body: &str) {
        let mut new_body = String::new();
        let mut count = 0;
            
        for l in body.replace("\n", " ").chars() {
            new_body.push(l);
            count += l.len_utf8();
            if count >= 29 {
                new_body.push('.');
                new_body.push('.');
                new_body.push('.');
                break;
            }
        }
        engine.print(
            left_up.0 + HOR_BORDER_ALIGN,
            left_up.1 + VER_BORDER_ALIGN + BODY_SHIFT,
            &new_body,
        );
    }

    pub fn draw_box_for_notification(
        engine: &mut ConsoleEngine,
        left_up: (i32, i32),
        longest_field: i32,
    ) {
        engine.rect_border(
            left_up.0,
            left_up.1,
            left_up.0 + HOR_BORDER_ALIGN * 2 + longest_field as i32,
            left_up.1 + VER_BORDER_ALIGN * 2 + 6,
            BorderStyle::new_light(),
        );
    }

    pub fn draw_frame(engine: &mut ConsoleEngine, width: i32, height: i32) {
        engine.rect_border(0, 0, width, height, BorderStyle::new_light());
    }

    pub fn get_longest_field(app_name: usize, title: usize, body: usize) -> i32 {
        let cur_max_field = max(app_name, max(title, body));
        if cur_max_field > 30 {
            30
        } else {
            cur_max_field as i32
        }
    }

    pub fn move_next_line(left_up: &mut (i32, i32), longest_field: &mut i32) {
        left_up.0 += *longest_field + HOR_BORDER_ALIGN * 2 + 2;
        left_up.1 = 1;
        *longest_field = 0;
    }
}
