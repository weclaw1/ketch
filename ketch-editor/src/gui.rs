use crate::editor_event::EditorEvent;
use crate::editor_state::EditorState;
use crate::widget_ids::Ids;
use ketch_core::resource::AssetManager;
use conrod_core::position::Positionable;
use conrod_core::widget::Widget;
use conrod_core::widget::text_box;
use conrod_core::position::Sizeable;
use conrod_core::position::Position;
use conrod_core::position::Dimension;
use conrod_core::position::Align;
use conrod_core::position::Relative;
use conrod_core::position::Place;
use conrod_core::color;
use conrod_core::Labelable;
use conrod_core::widget;

use crate::Editor;

use log::*;

mod gui_event;

use gui_event::light_text_box_event_execute;

impl Editor {
    pub fn update_gui(&mut self) {
        let window_dimensions = ketch_core::renderer::get_window_dimensions(self.surface.window());
        let mut ui = self.ui.set_widgets();

        light_panel(&self.widget_ids, &mut ui, &self.synced_editor_state, &mut self.current_editor_state, &mut self.pending_editor_events);
    }
}

fn light_panel(ids: &Ids, ui: &mut conrod_core::UiCell, 
               synced_editor_state: &EditorState, current_editor_state: &mut EditorState,
               pending_editor_events: &mut Vec<EditorEvent>) {
    const PANEL_TITLE: &str = "Light";
    const PANEL_WIDTH: f64 = 300.0;
    const PANEL_HEIGHT: f64 = 150.0;

    const COLUMN_PADDING: f64 = 10.0;
    const WIDGET_DISTANCE: f64 = 10.0;

    const TEXT_BOX_WIDTH: f64 = 50.0;
    const TEXT_BOX_HEIGHT: f64 = 25.0;

    let floating = widget::Canvas::new().floating(true).top_left()
                                        .w_h(PANEL_WIDTH, PANEL_HEIGHT)
                                        .title_bar(PANEL_TITLE);

    floating.flow_right(&[
        (ids.light_panel_x_column, widget::Canvas::new().pad(COLUMN_PADDING)),
        (ids.light_panel_y_column, widget::Canvas::new().pad(COLUMN_PADDING)),
        (ids.light_panel_z_column, widget::Canvas::new().pad(COLUMN_PADDING)),
    ]).set(ids.light_panel_canvas, ui);

    widget::Text::new("x:").mid_left_of(ids.light_panel_x_column)
                           .set(ids.x_light_label, ui);

    let x_light_text_box = widget::TextBox::new(&current_editor_state.x_light_text_box_content).right_from(ids.x_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT]);

    pending_editor_events.extend(x_light_text_box.set(ids.x_light_text_box, ui).into_iter()
                                                 .filter_map(|event| light_text_box_event_execute(event, "x", synced_editor_state, current_editor_state)));

    widget::Text::new("y:").mid_left_of(ids.light_panel_y_column)
                           .set(ids.y_light_label, ui);

    let y_light_text_box = widget::TextBox::new(&current_editor_state.y_light_text_box_content).right_from(ids.y_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT]);

    pending_editor_events.extend(y_light_text_box.set(ids.y_light_text_box, ui).into_iter()
                                                 .filter_map(|event| light_text_box_event_execute(event, "y", synced_editor_state, current_editor_state)));

    widget::Text::new("z:").mid_left_of(ids.light_panel_z_column)
                           .set(ids.z_light_label, ui);

    let z_light_text_box = widget::TextBox::new(&current_editor_state.z_light_text_box_content).right_from(ids.z_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT]);

    pending_editor_events.extend(z_light_text_box.set(ids.z_light_text_box, ui).into_iter()
                                                 .filter_map(|event| light_text_box_event_execute(event, "z", synced_editor_state, current_editor_state)));

}