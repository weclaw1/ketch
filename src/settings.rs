pub struct Settings {
    scr_width: f64,
    scr_height: f64,
}

impl Settings {
    pub fn new(scr_width: f64, scr_height: f64) -> Self {
        Settings {
            scr_width: scr_width,
            scr_height: scr_height,
        }
    }

    pub fn scr_width(&self) -> f64 {
        self.scr_width()
    }

    pub fn set_scr_width(&mut self, value: f64) {
        self.scr_width = value;
    }

    pub fn scr_height(&self) -> f64 {
        self.scr_height()
    }

    pub fn set_scr_height(&mut self, value: f64) {
        self.scr_height = value;
    }
}