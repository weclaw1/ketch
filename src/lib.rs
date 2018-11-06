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
