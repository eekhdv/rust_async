use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

trait CoordGet<T> {
    fn get_selected_x(&self) -> i32;
    fn get_selected_y(&self) -> i32;
}

trait CoordSet<T> {
    fn set_selected_x(&mut self, x: i32);
    fn set_selected_y(&mut self, y: i32);
}

#[derive(Debug, PartialEq, Clone)]
enum Select {
    LeftUp,
    RightDown,
}

#[derive(Debug, PartialEq, Clone)]
struct Rect {
    left_up: (i32, i32),
    right_down: (i32, i32),
    selection: Select,
    fill: bool,
}

impl Rect {
    fn new() -> Self {
        Self { left_up: (4, 4), right_down: (18, 12), selection: Select::LeftUp, fill: false }
    }

    fn is_fill(&self) -> bool {
        self.fill
    }
}

impl CoordGet<Select> for Rect {
    fn get_selected_x(&self) -> i32 {
        if self.selection == Select::LeftUp { self.left_up.0 } else { self.right_down.0 }
    }

    fn get_selected_y(&self) -> i32 {
        if self.selection == Select::LeftUp { self.left_up.1 } else { self.right_down.1 }
    }
}

impl CoordSet<Select> for Rect {
    fn set_selected_x(&mut self, x: i32) {
        if self.selection == Select::LeftUp { self.left_up.0 = x; } else { self.right_down.0 = x; }
    }

    fn set_selected_y(&mut self, y: i32) {
        if self.selection == Select::LeftUp { self.left_up.1 = y } else { self.right_down.1 = y; }
    }
}

fn main() {
    // initializes a screen filling the terminal of at least 50x20 of size with a target of 3 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(50, 20, 5).unwrap();

    let mut rect = Rect::new();

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
                               // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen();
        if rect.is_fill() {
            engine.fill_rect(
                rect.left_up.0,
                rect.left_up.1,
                rect.right_down.0,
                rect.right_down.1,
                pixel::pxl('#'),
            );
        } else {
            engine.rect(
                rect.left_up.0,
                rect.left_up.1,
                rect.right_down.0,
                rect.right_down.1,
                pixel::pxl('#'),
            );
        }
        engine.print(0, 1, "Position: [2] [4] [6] [8] ; Change point: [5]   ");

        engine.print(
            0,
            0,
            format!("[S]hape: Rect, [F]ill : {}   ", rect.is_fill()).as_str(),
        );

        // display the configured coordinates and highlight the current one
        if engine.frame_count % 4 >= 2 {
            engine.set_pxl(rect.left_up.0, rect.left_up.1, pixel::pxl_fg('#', Color::Cyan));
            engine.set_pxl(rect.right_down.0, rect.right_down.1, pixel::pxl_fg('#', Color::Cyan));
            
            engine.set_pxl(
                rect.get_selected_x(),
                rect.get_selected_y(),
                pixel::pxl_fg('#', Color::Yellow),
            );
        }

        // handling coordinate displacement with a particular case for selection 1 of circle
        // because it's the range selection
        if (engine.is_key_held(KeyCode::Char('8')) || engine.is_key_pressed(KeyCode::Up))
            && rect.get_selected_y() > 0
            && (rect.selection == Select::LeftUp)
        {
            rect.set_selected_y(rect.get_selected_y() - 1);
        }
        if (engine.is_key_held(KeyCode::Char('6')) || engine.is_key_pressed(KeyCode::Right))
            && rect.get_selected_x() < engine.get_width() as i32 - 1
        {
            rect.set_selected_x(rect.get_selected_x() + 1);
        }
        if (engine.is_key_held(KeyCode::Char('2')) || engine.is_key_pressed(KeyCode::Down))
            && rect.get_selected_x() < engine.get_height() as i32 - 1
            && (rect.selection == Select::LeftUp)
        {
            rect.set_selected_y(rect.get_selected_y() + 1);
        }
        if (engine.is_key_held(KeyCode::Char('4')) || engine.is_key_pressed(KeyCode::Left))
            && rect.get_selected_x() > 0
        {
            rect.set_selected_x(rect.get_selected_x() - 1);
        }
        // switch between configured coordinates
        if engine.is_key_pressed(KeyCode::Char('5')) || engine.is_key_pressed(KeyCode::Char(' ')) {
            rect.selection = if rect.selection == Select::LeftUp { Select::RightDown } else { Select::LeftUp }
        }
        // toggle fill flag
        if engine.is_key_pressed(KeyCode::Char('f')) {
            rect.fill = !rect.is_fill();
        }

        engine.print(rect.left_up.0 + 1, rect.left_up.1 + (rect.right_down.1 - rect.left_up.1) / 2, "hello");
        engine.draw(); // draw the screen
    }
}
