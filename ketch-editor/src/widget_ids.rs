use conrod_core::widget_ids;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        light_panel_canvas,
        light_panel_x_column,
        light_panel_y_column,
        light_panel_z_column,
        x_light_label,
        x_light_text_box,
        y_light_label,
        y_light_text_box,
        z_light_label,
        z_light_text_box,
        run_button,
    }
}