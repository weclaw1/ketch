use std::error::Error;

use conrod_core::text::font;
use conrod_vulkano::RendererCreationError;

use quick_error::quick_error; 

quick_error! {
    #[derive(Debug)]
    pub enum EditorCreationError {
        SubpassCreationError {
            display("SubpassCreationError: couldn't create subpass for GUI editor")
        }
        RendererCreationError(err: RendererCreationError) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
        FontLoadError(err: font::Error) {
            from()
            display(x) -> ("{}: {}", x.description(), err)
            cause(err)
        }
    }
}
