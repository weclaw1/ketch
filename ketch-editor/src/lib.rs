use crate::editor_error::EditorCreationError;
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
use editor_state::EditorState;

use crate::widget_ids::Ids;
use conrod_core::Ui;


mod widget_ids;
mod editor_state;
mod editor_error;
mod gui;

pub struct Editor {
    ui: Ui,
    surface: Arc<Surface<Window>>,
    widget_ids: Ids,
    conrod_renderer: conrod_vulkano::Renderer,
    image_map: conrod_core::image::Map<conrod_vulkano::Image>,
    editor_state: EditorState,
}

impl Editor {
    pub fn new(renderer: &Renderer) -> Result<Self, EditorCreationError> {
        let surface = renderer.surface();
        let window_dimensions = ketch_core::renderer::get_window_dimensions(surface.window());

        let subpass = match vulkano::framebuffer::Subpass::from(renderer.render_pass(), 0) {
            Some(subpass) => subpass,
            None => return Err(EditorCreationError::SubpassCreationError),
        };

        let conrod_renderer = conrod_vulkano::Renderer::new(
            renderer.device(),
            subpass,
            renderer.queues().graphics_queue().family(),
            [window_dimensions.width as u32, window_dimensions.height as u32],
            ketch_core::renderer::get_window_dpi(surface.window()),
        )?;

        let mut ui = conrod_core::UiBuilder::new([window_dimensions.width, window_dimensions.height]).theme(Editor::theme()).build();
        let widget_ids = widget_ids::Ids::new(ui.widget_id_generator());
        let image_map = conrod_core::image::Map::new();
        ui.fonts.insert_from_file("ketch-editor/assets/fonts/NotoSans-Regular.ttf")?;

        Ok(
            Editor {
                ui,
                surface,
                widget_ids,
                conrod_renderer,
                image_map,

                editor_state: EditorState::new(),
            }
        )
    }

    pub fn theme() -> conrod_core::Theme {
        use conrod_core::position::{Align, Direction, Padding, Position, Relative};
        conrod_core::Theme {
            name: "Ketch Editor Theme".to_string(),
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
