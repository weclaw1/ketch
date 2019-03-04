use ketch_core::settings::Settings;

#[derive(Clone)]
pub struct EditorState {
    pub run_game: bool,
    pub x_light_text_box_content: String,
    pub y_light_text_box_content: String,
    pub z_light_text_box_content: String,
}

impl EditorState {
    pub fn new() -> Self {
        EditorState {
            run_game: false,
            x_light_text_box_content: String::from("0.0"),
            y_light_text_box_content: String::from("0.0"),
            z_light_text_box_content: String::from("0.0"),
        }
    }
}

pub struct EditorInputState {
    pub mouse_delta_changed: bool,
    pub right_mouse_button_pressed: bool,
    pub camera_speed: f32,
    pub mouse_sensitivity: f32,
    pub mouse_delta: (f32, f32),
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl EditorInputState {
    pub fn new() -> Self {
        EditorInputState {
            mouse_delta_changed: false,
            right_mouse_button_pressed: false,
            camera_speed: 5.0,
            mouse_sensitivity: 0.2,
            mouse_delta: (0.0, 0.0),
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}