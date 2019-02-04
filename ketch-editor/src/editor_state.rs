pub struct EditorState {
    pub x_text_box_content: String,
    pub y_text_box_content: String,
    pub z_text_box_content: String,
}

impl EditorState {
    pub fn new() -> Self {
        EditorState {
            x_text_box_content: String::from("0.0"),
            y_text_box_content: String::from("0.0"),
            z_text_box_content: String::from("0.0"),
        }
    }
}