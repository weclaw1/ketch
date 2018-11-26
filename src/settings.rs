use winit::dpi::LogicalSize;

pub struct Settings {
    window_title: String,
    window_size: LogicalSize,
}

impl Settings {
    pub fn new(window_title: &str, scr_width: f64, scr_height: f64) -> Self {
        Settings {
            window_title: String::from(window_title),
            window_size: LogicalSize::new(scr_width, scr_height),
        }
    }

    pub fn set_window_size(&mut self, value: LogicalSize) {
        self.window_size = value;
    }

    pub fn window_size(&self) -> &LogicalSize {
        &self.window_size
    }

    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    pub fn set_window_title(&mut self, value: &str) {
        self.window_title = String::from(value);
    }
}