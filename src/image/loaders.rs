use image::DynamicImage;
use std::path::Path;
use tracing::{debug, warn};

pub enum ImageLoader {
    Standard,
    Heif,
    Jxl,
}

impl ImageLoader {
    pub fn from_path(path: &Path) -> Self {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            match extension.to_lowercase().as_str() {
                "heic" | "heif" => Self::Heif,
                "jxl" => Self::Jxl,
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
            Self::Jxl => {
                self.load_jxl_from_memory(data)
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

    fn load_jxl_from_memory(&self, data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
        use jpegxl_rs::{decoder_builder, decode::Pixels};

        debug!("Loading JXL image from memory");

        let decoder = decoder_builder()
            .build()
            .map_err(|e| format!("Failed to create JXL decoder: {}", e))?;

        let (info, pixels) = decoder.decode(data)
            .map_err(|e| format!("Failed to decode JXL image: {}", e))?;

        debug!("JXL image dimensions: {}x{}, channels: {}, has_alpha: {}", 
               info.width, info.height, info.num_color_channels, info.has_alpha_channel);

        // Convert pixels to Vec<u8> handling different pixel formats
        let pixel_data: Vec<u8> = match pixels {
            Pixels::Uint8(data) => data,
            Pixels::Uint16(data) => {
                // Convert u16 to u8 by scaling down
                data.into_iter().map(|x| (x >> 8) as u8).collect()
            },
            Pixels::Float(data) => {
                // Convert f32 to u8
                data.into_iter().map(|x| (x * 255.0).clamp(0.0, 255.0) as u8).collect()
            },
            Pixels::Float16(data) => {
                // Convert f16 to u8
                data.into_iter().map(|x| (x.to_f32() * 255.0).clamp(0.0, 255.0) as u8).collect()
            },
        };

        // Determine actual number of channels (color + alpha)
        let total_channels = info.num_color_channels + if info.has_alpha_channel { 1 } else { 0 };

        match total_channels {
            1 => {
                let img_buffer = image::GrayImage::from_raw(info.width, info.height, pixel_data)
                    .ok_or("Failed to create grayscale image buffer from JXL data")?;
                Ok(DynamicImage::ImageLuma8(img_buffer))
            }
            2 => {
                let img_buffer = image::GrayAlphaImage::from_raw(info.width, info.height, pixel_data)
                    .ok_or("Failed to create grayscale+alpha image buffer from JXL data")?;
                Ok(DynamicImage::ImageLumaA8(img_buffer))
            }
            3 => {
                let img_buffer = image::RgbImage::from_raw(info.width, info.height, pixel_data)
                    .ok_or("Failed to create RGB image buffer from JXL data")?;
                Ok(DynamicImage::ImageRgb8(img_buffer))
            }
            4 => {
                let img_buffer = image::RgbaImage::from_raw(info.width, info.height, pixel_data)
                    .ok_or("Failed to create RGBA image buffer from JXL data")?;
                Ok(DynamicImage::ImageRgba8(img_buffer))
            }
            _ => {
                Err(format!("Unsupported JXL channel count: {}", total_channels).into())
            }
        }
    }
}