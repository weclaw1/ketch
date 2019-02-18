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