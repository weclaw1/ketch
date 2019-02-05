use std::time::Duration;
use winit::dpi::PhysicalSize;

/// Stores engine settings.
pub struct Settings {
    window_title: String,
    initial_window_size: PhysicalSize,
    time_per_update: Duration,
    log_fps_frequency: Duration,
}

impl Settings {
    /// Creates new settings struct with given window title and screen size.
    pub fn new<S: Into<String>>(window_title: S, scr_width: f64, scr_height: f64) -> Self {
        Settings {
            window_title: window_title.into(),
            initial_window_size: PhysicalSize::new(scr_width, scr_height),
            time_per_update: Duration::from_millis(16),
            log_fps_frequency: Duration::from_secs(5),
            gui_editor: false,
        }
    }

    /// Returns initial window size.
    pub fn initial_window_size(&self) -> &PhysicalSize {
        &self.initial_window_size
    }

    /// Returns window title.
    pub fn window_title(&self) -> &str {
        &self.window_title
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
}