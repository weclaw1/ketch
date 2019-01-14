use std::time::Duration;
use winit::dpi::PhysicalSize;

/// Stores engine settings.
pub struct Settings {
    window_title: String,
    window_size: PhysicalSize,
    time_per_update: Duration,
    log_fps_frequency: Duration,
    dpi: f64,
    near_plane: f32,
    far_plane: f32,
    grab_cursor: bool,
    hide_cursor: bool,
    gui_editor: bool,
}

impl Settings {
    /// Creates new settings struct with given window title and screen size.
    pub fn new<S: Into<String>>(window_title: S, scr_width: f64, scr_height: f64) -> Self {
        Settings {
            window_title: window_title.into(),
            window_size: PhysicalSize::new(scr_width, scr_height),
            time_per_update: Duration::from_millis(16),
            log_fps_frequency: Duration::from_secs(5),
            dpi: 1.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            grab_cursor: false,
            hide_cursor: false,
            gui_editor: false,
        }
    }

    /// Changes window size.
    pub fn set_window_size(&mut self, value: PhysicalSize) {
        self.window_size = value;
    }

    /// Returns current window size.
    pub fn window_size(&self) -> &PhysicalSize {
        &self.window_size
    }

    /// Returns window title.
    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    /// Returns the near plane used by the camera perspective-view frustum.
    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    /// Sets the near plane used by the camera perspective-view frustum.
    pub fn set_near_plane(&mut self, value: f32) {
        self.near_plane = value;
    }

    /// Returns the far plane used by the camera perspective-view frustum.
    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }

    /// Sets the far plane used by the camera perspective-view frustum.
    pub fn set_far_plane(&mut self, value: f32) {
        self.far_plane = value;
    }

    /// Returns the screen dpi.
    pub fn dpi(&self) -> f64 {
        self.dpi
    }

    /// Sets the current screen dpi.
    pub fn set_dpi(&mut self, value: f64) {
        self.dpi = value;
    }

    /// Can be used to grab the cursor in the window.
    pub fn set_grab_cursor(&mut self, value: bool) {
        self.grab_cursor = value;
    }

    /// Checks if cursor is currently grabbed in the window.
    pub fn grab_cursor(&self) -> bool {
        self.grab_cursor
    }

    /// Can be used to hide cursor.
    pub fn set_hide_cursor(&mut self, value: bool) {
        self.hide_cursor = value;
    }

    /// Checks if cursor is currently hidden.
    pub fn hide_cursor(&self) -> bool {
        self.hide_cursor
    }

    /// Sets time step between game updates.
    pub fn set_time_per_update(&mut self, value: Duration) {
        self.time_per_update = value;
    }

    /// Returns time step between game updates.
    pub fn time_per_update(&self) -> Duration {
        self.time_per_update
    }

    /// Sets duration between game fps logs.
    pub fn set_log_fps_frequency(&mut self, value: Duration) {
        self.log_fps_frequency = value;
    }

    /// Returns duration between game fps logs.
    pub fn log_fps_frequency(&self) -> Duration {
        self.log_fps_frequency
    }

    pub fn set_gui_editor(&mut self, value: bool) {
        self.gui_editor = value;
    }

    pub fn gui_editor(&self) -> bool {
        self.gui_editor
    }
}