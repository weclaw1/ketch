use conrod_core::widget_ids;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // Title of lightsection.
        light_text,
        x_light_pos,
        x_text_box,
    }
}