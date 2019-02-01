use ketch_core::resource::AssetManager;
use vulkano::swapchain::Surface;
use vulkano::device::Queue;
use std::sync::Arc;
use ketch_core::input::input_event::Event;
use winit::Window;
use ketch_core::input::input_event::InputEvent;
use conrod_vulkano::Image;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use ketch_core::renderer::Renderer;
use conrod_core::render::Primitives;
use conrod_core::position::Positionable;
use conrod_core::widget::Widget;
use conrod_core::widget::text_box;
use conrod_core::position::Sizeable;
use conrod_core::position::Position;
use conrod_core::position::Dimension;
use conrod_core::position::Align;
use conrod_core::position::Relative;
use conrod_core::position::Place;
use crate::widget_ids::Ids;
use conrod_core::Ui;
use conrod_core::widget;

mod widget_ids;

pub struct Editor {
    ui: Ui,
    surface: Arc<Surface<Window>>,
    widget_ids: Ids,
    conrod_renderer: conrod_vulkano::Renderer,
    image_map: conrod_core::image::Map<conrod_vulkano::Image>,

    x_text_box_content: String,
}

impl Editor {
    pub fn new(renderer: &Renderer) -> Self {
        let surface = renderer.surface();
        let window_dimensions = ketch_core::renderer::get_window_dimensions(surface.window());

        let subpass = vulkano::framebuffer::Subpass::from(renderer.render_pass(), 0).expect("Couldn't create subpass for GUI!");
        let conrod_renderer = conrod_vulkano::Renderer::new(
            renderer.device(),
            subpass,
            renderer.queues().graphics_queue().family(),
            [window_dimensions.width as u32, window_dimensions.height as u32],
            ketch_core::renderer::get_window_dpi(surface.window()),
        ).unwrap();

        let mut ui = conrod_core::UiBuilder::new([window_dimensions.width, window_dimensions.height]).theme(Editor::theme()).build();
        let widget_ids = widget_ids::Ids::new(ui.widget_id_generator());
        let image_map = conrod_core::image::Map::new();
        ui.fonts.insert_from_file("ketch-editor/assets/fonts/NotoSans-Regular.ttf").unwrap();

        Editor {
            ui,
            surface,
            widget_ids,
            conrod_renderer,
            image_map,

            x_text_box_content: String::from("0.0"),
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

    pub fn gui(&mut self, asset_manager: &mut AssetManager) {
        const MARGIN: conrod_core::Scalar = 30.0;

        let window_dimensions = ketch_core::renderer::get_window_dimensions(self.surface.window());
        let mut ui = self.ui.set_widgets();

        widget::Canvas::new().x_position(Position::Relative(Relative::Align(Align::Start), None))
                             .x_dimension(Dimension::Absolute(window_dimensions.width / 3.0))
                             .pad(MARGIN)
                             .scroll_kids_vertically()
                             .set(self.widget_ids.canvas, &mut ui);

        widget::Text::new("Light").mid_top_of(self.widget_ids.canvas).set(self.widget_ids.light_text, &mut ui);

        widget::Text::new("x:").y_position_relative(Relative::Scalar(-20.0))
                               .x_place_on(self.widget_ids.canvas, Place::Start(None))
                               .set(self.widget_ids.x_light_pos, &mut ui);

        for event in widget::TextBox::new(&self.x_text_box_content).right_from(self.widget_ids.x_light_pos, 20.0)
                                                .y_place_on(self.widget_ids.x_light_pos, Place::End(None))
                                                .wh([60.0, 30.0])
                                                .set(self.widget_ids.x_text_box, &mut ui) 
        {
            match event {
                text_box::Event::Enter => {
                    if let Some(scene) = asset_manager.active_scene_mut() {
                        let position_x: Result<f32, _> = self.x_text_box_content.parse();
                        match position_x {
                            Ok(val) => scene.set_light_position_x(val),
                            Err(err) => (),
                        }
                    }
                },
                text_box::Event::Update(new_val) => {
                    self.x_text_box_content = new_val;
                }
            }
        }

    }

    pub fn draw_ui(&self) -> Primitives {
        self.ui.draw()
    }

    pub fn conrod_renderer_mut(&mut self) -> &mut conrod_vulkano::Renderer {
        &mut self.conrod_renderer
    }

    pub fn image_map(&self) -> &conrod_core::image::Map<Image> {
        &self.image_map
    }

    pub fn handle_input(&mut self, window: &Window, input_events: Vec<Event>, asset_manager: &mut AssetManager) {
        input_events.into_iter().filter_map(|event| conrod_winit::convert_event(event, window))
                                .for_each(|event| self.ui.handle_event(event));
        if self.ui.global_input().events().next().is_some() {
            self.gui(asset_manager);
        }
    }

    pub fn add_glyph_commands(&mut self, mut command_buffer_builder: AutoCommandBufferBuilder) -> AutoCommandBufferBuilder {
        let primitives = self.ui.draw();
        let window_dimensions = ketch_core::renderer::get_window_dimensions(self.surface.window());
        let dpi = ketch_core::renderer::get_window_dpi(self.surface.window());

        let viewport = [0.0, 0.0, window_dimensions.width as f32, window_dimensions.height as f32];
        let mut cmds = self.conrod_renderer.fill(&self.image_map, viewport, dpi, primitives).unwrap();

        for cmd in cmds.commands.drain(..) {
            let buffer = cmds.glyph_cpu_buffer_pool.chunk(cmd.data.iter().cloned()).unwrap();
            command_buffer_builder = command_buffer_builder.copy_buffer_to_image_dimensions(
                buffer,
                cmds.glyph_cache_texture.clone(),
                [cmd.offset[0], cmd.offset[1], 0],
                [cmd.size[0], cmd.size[1], 1],
                0,
                1,
                0
            ).expect("Failed to submit command for caching glyph");
        }

        command_buffer_builder
    }

    pub fn add_draw_commands(&mut self, queue: Arc<Queue>, mut command_buffer_builder: AutoCommandBufferBuilder) -> AutoCommandBufferBuilder {
        let window_dimensions = ketch_core::renderer::get_window_dimensions(self.surface.window());

        let viewport = [0.0, 0.0, window_dimensions.width as f32, window_dimensions.height as f32];

        let draw_cmds = self.conrod_renderer.draw(
            queue,
            &self.image_map,
            viewport,
        ).unwrap();
        for cmd in draw_cmds {
            let conrod_vulkano::DrawCommand {
                graphics_pipeline,
                dynamic_state,
                vertex_buffer,
                descriptor_set,
            } = cmd;
            command_buffer_builder = command_buffer_builder
                .draw(
                    graphics_pipeline,
                    &dynamic_state,
                    vec![vertex_buffer],
                    descriptor_set,
                    (),
                )
                .expect("failed to submit draw command");
        }
        command_buffer_builder
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
