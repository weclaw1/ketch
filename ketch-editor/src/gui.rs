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

impl Editor {
    pub fn gui(&mut self, asset_manager: &mut AssetManager) {
        let window_dimensions = ketch_core::renderer::get_window_dimensions(self.surface.window());
        let mut ui = self.ui.set_widgets();

        light_panel(&self.widget_ids, &mut ui, &mut self.editor_state, asset_manager);
    }
}

fn light_panel(ids: &Ids, ui: &mut conrod_core::UiCell, editor_state: &mut EditorState, asset_manager: &mut AssetManager) {
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

    for event in widget::TextBox::new(&editor_state.x_text_box_content).right_from(ids.x_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT])
                                            .set(ids.x_text_box, ui) 
    {
        match event {
            text_box::Event::Enter => {
                if let Some(scene) = asset_manager.active_scene_mut() {
                    match editor_state.x_text_box_content.parse::<f32>() {
                        Ok(val) => {
                            scene.set_light_position_x(val);
                            if let Some(light_object) = scene.objects_mut().iter_mut().find(|x| x.light_source()) {
                                light_object.set_position_x(val);
                            }
                        },
                        Err(err) => {
                            error!("Couldn't parse x light source text box: {}", err);
                            editor_state.x_text_box_content = scene.light_position_x().to_string();
                        },
                    }
                }
            },
            text_box::Event::Update(new_val) => {
                editor_state.x_text_box_content = new_val;
            }
        }
    }

    widget::Text::new("y:").mid_left_of(ids.light_panel_y_column)
                           .set(ids.y_light_label, ui);

    for event in widget::TextBox::new(&editor_state.y_text_box_content).right_from(ids.y_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT])
                                            .set(ids.y_text_box, ui) 
    {
        match event {
            text_box::Event::Enter => {
                if let Some(scene) = asset_manager.active_scene_mut() {
                    match editor_state.y_text_box_content.parse::<f32>() {
                        Ok(val) => {
                            scene.set_light_position_y(val);
                            if let Some(light_object) = scene.objects_mut().iter_mut().find(|x| x.light_source()) {
                                light_object.set_position_y(val);
                            }
                        },
                        Err(err) => {
                            error!("Couldn't parse y light source text box: {}", err);
                            editor_state.y_text_box_content = scene.light_position_y().to_string();
                        },
                    }
                }
            },
            text_box::Event::Update(new_val) => {
                editor_state.y_text_box_content = new_val;
            }
        }
    }

    widget::Text::new("z:").mid_left_of(ids.light_panel_z_column)
                           .set(ids.z_light_label, ui);

    for event in widget::TextBox::new(&editor_state.z_text_box_content).right_from(ids.z_light_label, WIDGET_DISTANCE)
                                            .wh([TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT])
                                            .set(ids.z_text_box, ui) 
    {
        match event {
            text_box::Event::Enter => {
                if let Some(scene) = asset_manager.active_scene_mut() {
                    match editor_state.z_text_box_content.parse::<f32>() {
                        Ok(val) => {
                            scene.set_light_position_z(val);
                            if let Some(light_object) = scene.objects_mut().iter_mut().find(|x| x.light_source()) {
                                light_object.set_position_z(val);
                            }
                        },
                        Err(err) => {
                            error!("Couldn't parse z light source text box: {}", err);
                            editor_state.z_text_box_content = scene.light_position_z().to_string();
                        },
                    }
                }
            },
            text_box::Event::Update(new_val) => {
                editor_state.z_text_box_content = new_val;
            }
        }
    }
}