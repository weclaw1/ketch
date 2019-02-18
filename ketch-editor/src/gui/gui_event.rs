use crate::editor_state::EditorState;
use crate::editor_event::EditorEvent;
use conrod_core::widget::text_box;

use log::*;

pub fn light_text_box_event_execute(event: text_box::Event, 
                                    updated_text_box: &str,
                                    synced_editor_state: &EditorState, 
                                    current_editor_state: &mut EditorState) -> Option<EditorEvent> {
    match event {
        text_box::Event::Enter => {
            let x = match current_editor_state.x_light_text_box_content.parse() {
                Ok(x) => x,
                Err(err) => {
                    error!("Couldn't parse x light source text box: {}", err);
                    current_editor_state.x_light_text_box_content = synced_editor_state.x_light_text_box_content.clone();
                    current_editor_state.x_light_text_box_content.parse().unwrap()
                }
            };

            let y = match current_editor_state.y_light_text_box_content.parse() {
                Ok(y) => y,
                Err(err) => {
                    error!("Couldn't parse y light source text box: {}", err);
                    current_editor_state.y_light_text_box_content = synced_editor_state.y_light_text_box_content.clone();
                    current_editor_state.y_light_text_box_content.parse().unwrap()
                }
            };

            let z = match current_editor_state.z_light_text_box_content.parse() {
                Ok(z) => z,
                Err(err) => {
                    error!("Couldn't parse z light source text box: {}", err);
                    current_editor_state.z_light_text_box_content = synced_editor_state.z_light_text_box_content.clone();
                    current_editor_state.z_light_text_box_content.parse().unwrap()
                }
            };

            Some(EditorEvent::LightPositionChanged((x, y, z)))
        },
        text_box::Event::Update(new_val) => {
            match updated_text_box {
                "x" => current_editor_state.x_light_text_box_content = new_val,
                "y" => current_editor_state.y_light_text_box_content = new_val,
                "z" => current_editor_state.z_light_text_box_content = new_val,
                _   => panic!("updated_text_box field should have value x, y or z!")
            }
            None
        }
    }
}