use ketch_core::resource::AssetManager;
use crate::Editor;
use conrod_core::widget::id::Id;

pub enum EditorEvent {
    LightPositionChanged((f32, f32, f32)),
}

pub fn handle_light_position_changed(x: f32, y: f32, z: f32, asset_manager: &mut AssetManager) {
    if let Some(scene) = asset_manager.active_scene_mut() { 
        scene.set_light_position(x, y, z);
        if let Some(light_object) = scene.objects_mut().iter_mut().find(|x| x.light_source()) {
            light_object.set_position(x, y, z);
        }
    }
}
