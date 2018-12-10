use winit::dpi::PhysicalSize;

pub struct Settings {
    window_title: String,
    window_size: PhysicalSize,
    dpi: f64,
    near_plane: f32,
    far_plane: f32,
    grab_cursor: bool,
    hide_cursor: bool,
}

impl Settings {
    pub fn new<S: Into<String>>(window_title: S, scr_width: f64, scr_height: f64) -> Self {
        Settings {
            window_title: window_title.into(),
            window_size: PhysicalSize::new(scr_width, scr_height),
            dpi: 1.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            grab_cursor: false,
            hide_cursor: false,
        }
    }

    pub fn set_window_size(&mut self, value: PhysicalSize) {
        self.window_size = value;
    }

    pub fn window_size(&self) -> &PhysicalSize {
        &self.window_size
    }

    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    pub fn set_near_plane(&mut self, value: f32) {
        self.near_plane = value;
    }

    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }

    pub fn set_far_plane(&mut self, value: f32) {
        self.far_plane = value;
    }

    pub fn dpi(&self) -> f64 {
        self.dpi
    }

    pub fn set_dpi(&mut self, value: f64) {
        self.dpi = value;
    }

    pub fn set_grab_cursor(&mut self, value: bool) {
        self.grab_cursor = value;
    }

    pub fn grab_cursor(&self) -> bool {
        self.grab_cursor
    }

    pub fn set_hide_cursor(&mut self, value: bool) {
        self.hide_cursor = value;
    }

    pub fn hide_cursor(&self) -> bool {
        self.hide_cursor
    }
}