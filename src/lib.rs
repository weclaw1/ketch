mod renderer;
mod settings;
mod resource;
mod input;

use self::renderer::{Renderer};
use self::settings::Settings;
use self::input::{InputSystem, InputMapping};


/// A struct representing the top level of this library.
/// It provides access to all the subsystems that can be used.
pub struct Smml<T: InputMapping> {
    renderer: Renderer,
    //resource_manager: AssetManager,
    input_system: InputSystem<T>,
    settings: Settings,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
