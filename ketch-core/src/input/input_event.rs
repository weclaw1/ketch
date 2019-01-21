pub use winit::{Event, KeyboardInput, MouseScrollDelta, ElementState, MouseButton, AxisId, ButtonId, VirtualKeyCode};
pub use winit::dpi::LogicalPosition;
pub use winit::WindowEvent;
pub use winit::DeviceEvent;

// TODO: Make InputEvents simpler, dont use winit types inside enum variants, take events by reference when converting
/// Enum containing input events
pub enum InputEvent {
    KeyboardInput(KeyboardInput),
    CursorMoved (LogicalPosition),
    MouseMotion { delta: (f64, f64) },
    MouseWheel(MouseScrollDelta),
    MouseInput { button: MouseButton, state: ElementState },
    Motion { axis: AxisId, value: f64 },
    Button { button: ButtonId, state: ElementState },
}

/// Changes winit Events to InputEvents
pub fn to_input_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => Some(InputEvent::KeyboardInput(input)),
            WindowEvent::CursorMoved { position, .. } => Some(InputEvent::CursorMoved(position)),
            WindowEvent::MouseWheel { delta, .. } => Some(InputEvent::MouseWheel(delta)),
            WindowEvent::MouseInput { button, state, .. } => Some(InputEvent::MouseInput { button, state }),
            _ => None,
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => Some(InputEvent::MouseMotion { delta }),
            DeviceEvent::Motion { axis, value } => Some(InputEvent::Motion { axis, value }),
            DeviceEvent::Button { button, state } => Some(InputEvent::Button { button, state }),
            _ => None,
        },
        _ => None,
    }
}
