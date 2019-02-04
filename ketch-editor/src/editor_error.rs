use conrod_core::text::font::Error;
use conrod_vulkano::RendererCreationError;
use derive_error::Error;

#[derive(Debug, Error)]
pub enum EditorCreationError {
    /// Couldn't create subpass for GUI editor!
    #[error(no_from, non_std)]
    SubpassCreationError,

    RendererCreationError(RendererCreationError),
    FontLoadError(Error),
}