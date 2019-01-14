use image::DynamicImage;
use image::open;
use conrod_core::render::Primitives;
use conrod_core::position::Positionable;
use conrod_core::widget::Widget;
use conrod_core::position::Sizeable;
use conrod_core::position::Dimension;
use crate::widget_ids::Ids;
use conrod_core::Ui;
use conrod_core::widget;
use ketch_engine::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;

mod widget_ids;

pub struct Editor {
    ui: Ui,
    widget_ids: Ids,
    image_map: conrod_core::image::Map<DynamicImage>,
}

impl Editor {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        let (window_width, window_height) = {
            let settings = settings.borrow();
            (settings.window_size().width, settings.window_size().height)
        };
        let mut ui = conrod_core::UiBuilder::new([window_width, window_height]).theme(Editor::theme()).build();
        let widget_ids = widget_ids::Ids::new(ui.widget_id_generator());
        let image_map = conrod_core::image::Map::new();
        ui.fonts.insert_from_file("ketch-editor/assets/fonts/NotoSans-Regular.ttf").unwrap();

        Editor {
            ui,
            widget_ids,
            image_map,
        }
    }

    pub fn theme() -> conrod_core::Theme {
        use conrod_core::position::{Align, Direction, Padding, Position, Relative};
        conrod_core::Theme {
            name: "Demo Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: conrod_core::color::DARK_CHARCOAL,
            shape_color: conrod_core::color::LIGHT_CHARCOAL,
            border_color: conrod_core::color::BLACK,
            border_width: 0.0,
            label_color: conrod_core::color::WHITE,
            font_id: None,
            font_size_large: 26,
            font_size_medium: 18,
            font_size_small: 12,
            widget_styling: conrod_core::theme::StyleMap::default(),
            mouse_drag_threshold: 0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }
    }

    pub fn gui(&mut self) {
        const MARGIN: conrod_core::Scalar = 30.0;

        let window_width = self.ui.win_w;
        let mut ui = self.ui.set_widgets();

        widget::Canvas::new().x_dimension(Dimension::Absolute(window_width / 3.0))
                             .pad(MARGIN)
                             .scroll_kids_vertically()
                             .set(self.widget_ids.canvas, &mut ui);

        widget::Text::new("Ala ma kota").mid_top_of(self.widget_ids.canvas).set(self.widget_ids.test_text, &mut ui);
    }

    pub fn draw_ui(&self) -> Primitives {
        self.ui.draw()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
