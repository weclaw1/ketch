use crate::editor_event::EditorEvent::LightPositionChanged;
use ketch_core::resource::AssetManager;
use crate::Editor;
use conrod_core::widget::id::Id;

pub enum EditorEvent {
    LightPositionChanged((f32, f32, f32)),
}

impl EditorEvent {
    pub fn execute(self, asset_manager: &mut AssetManager) {
        match self {
            LightPositionChanged((x, y, z)) => EditorEvent::handle_light_position_changed(x, y, z, asset_manager),
        }
    }

    fn handle_light_position_changed(x: f32, y: f32, z: f32, asset_manager: &mut AssetManager) {
        if let Some(scene) = asset_manager.active_scene_mut() { 
            scene.set_light_position(x, y, z);
            if let Some(light_object) = scene.objects_mut().iter_mut().find(|x| x.light_source()) {
                light_object.set_position(x, y, z);
            }
        }
    }
}


