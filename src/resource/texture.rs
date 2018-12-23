use vulkano::device::Device;
use std::sync::Arc;
use vulkano::device::Queue;
use vulkano::image::{ImmutableImage, Dimensions};
use vulkano::sampler::{Sampler, SamplerAddressMode, Filter, MipmapMode};
use vulkano::format::Format;
use image::RgbaImage;
use std::path::Path;

/// Image applied on mesh
pub struct Texture {
    name: String,

    image_buffer: Arc<ImmutableImage<Format>>,
    sampler: Arc<Sampler>,
}

impl Texture {
    /// Loads texture from image file.
    pub fn load<S: Into<String>, P: AsRef<Path>>(name: S, image_path: P, upload_queue: Arc<Queue>, device: Arc<Device>) -> Self {
        let loaded_image = match image::open(image_path) {
            Ok(image) => image,
            Err(e) => panic!("Couldn't load image: {}", e),
        };
        let image_rgba = loaded_image.to_rgba();

        Texture::new(name, image_rgba, upload_queue, device)
    }

    /// Creates new texture from loaded image.
    pub fn new<S: Into<String>>(name: S, image: RgbaImage, upload_queue: Arc<Queue>, device: Arc<Device>) -> Self {
        let (image_buffer, _future) = ImmutableImage::from_iter(
            image.clone().into_raw().into_iter(),
            Dimensions::Dim2d { width: image.width(), height: image.height() },
            Format::R8G8B8A8Srgb,
            upload_queue,
        ).expect("Failed to create image buffer");

        let sampler = Sampler::new(
                        device, 
                        Filter::Linear, 
                        Filter::Linear,
                        MipmapMode::Nearest, 
                        SamplerAddressMode::Repeat, 
                        SamplerAddressMode::Repeat,
                        SamplerAddressMode::Repeat, 
                        0.0, 
                        1.0, 
                        0.0, 
                        1.0
                     ).expect("Failed to create a sampler");

        Texture {
            name: name.into(),
            image_buffer,
            sampler,
        }
    }

    /// Returns name of this texture.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns image buffer used by this texture.
    pub fn image_buffer(&self) -> Arc<ImmutableImage<vulkano::format::Format>>
    {
        self.image_buffer.clone()
    }

    /// Returns sampler used by this texture.
    pub fn sampler(&self) -> Arc<vulkano::sampler::Sampler>{
        self.sampler.clone()
    }
}