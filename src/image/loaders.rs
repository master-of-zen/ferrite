use image::DynamicImage;
use std::path::Path;
use tracing::{debug, warn};

pub enum ImageLoader {
    Standard,
    Heif,
}

impl ImageLoader {
    pub fn from_path(path: &Path) -> Self {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            match extension.to_lowercase().as_str() {
                "heic" | "heif" => Self::Heif,
                _ => Self::Standard,
            }
        } else {
            Self::Standard
        }
    }

    pub fn load_from_memory(&self, data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::Standard => {
                image::load_from_memory(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::Heif => {
                self.load_heif_from_memory(data)
            }
        }
    }

    fn load_heif_from_memory(&self, data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
        use libheif_rs::{HeifContext, ColorSpace, RgbChroma, LibHeif};

        debug!("Loading HEIF image from memory");

        let lib_heif = LibHeif::new();
        let ctx = HeifContext::read_from_bytes(data)?;
        let handle = ctx.primary_image_handle()?;
        
        let width = handle.width();
        let height = handle.height();
        
        debug!("HEIF image dimensions: {}x{}", width, height);

        // Decode the image using the correct v2.2 API
        let image = lib_heif.decode(
            &handle,
            ColorSpace::Rgb(RgbChroma::Rgb),
            None,
        )?;
        
        // Get the RGB plane data
        let planes = image.planes();
        let interleaved = planes.interleaved.ok_or("Failed to get interleaved plane")?;
        
        // Create RGB buffer with proper stride handling
        let mut rgb_buffer = Vec::with_capacity((width * height * 3) as usize);
        let stride = interleaved.stride;
        let plane_data = interleaved.data;
        
        for y in 0..height {
            let row_start = (y * stride as u32) as usize;
            let pixel_data_size = (width * 3) as usize;
            
            if row_start + pixel_data_size <= plane_data.len() {
                rgb_buffer.extend_from_slice(&plane_data[row_start..row_start + pixel_data_size]);
            } else {
                warn!("HEIF: Row data extends beyond buffer, truncating");
                break;
            }
        }

        let img_buffer = image::RgbImage::from_raw(width, height, rgb_buffer)
            .ok_or("Failed to create RGB image buffer")?;

        Ok(DynamicImage::ImageRgb8(img_buffer))
    }
}