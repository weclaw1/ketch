#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12;
#[cfg(not(any(feature = "vulkan", feature = "dx12", feature = "metal", feature = "gl")))]
extern crate gfx_backend_empty;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan;

extern crate winit;
extern crate gfx_hal as hal;
extern crate glsl_to_spirv;

pub mod render;

use render::{Renderer};

/// A struct representing the top level of this library.
/// It provides access to all the subsystems that can be used.
pub struct Smml {
    renderer: Renderer,
    resource_manager: AssetManager,
    input_system: InputSystem,
    settings: Settings,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
