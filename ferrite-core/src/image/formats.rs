use std::ffi::OsStr;

pub struct SupportedFormats;

impl SupportedFormats {
    pub const EXTENSIONS: &'static [&'static str] =
        &["jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "tga", "webp"];

    pub fn is_supported(extension: Option<&OsStr>) -> bool {
        extension
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .map(|e| Self::EXTENSIONS.contains(&e.as_str()))
            .unwrap_or(false)
    }

    pub fn supported_formats_string() -> String {
        Self::EXTENSIONS.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_extensions() {
        // Test valid extensions with different cases
        assert!(SupportedFormats::is_supported(Some(OsStr::new("jpg"))));
        assert!(SupportedFormats::is_supported(Some(OsStr::new("JPG"))));
        assert!(SupportedFormats::is_supported(Some(OsStr::new("PNG"))));

        // Test invalid extensions
        assert!(!SupportedFormats::is_supported(Some(OsStr::new("txt"))));
        assert!(!SupportedFormats::is_supported(Some(OsStr::new("doc"))));

        // Test None case
        assert!(!SupportedFormats::is_supported(None));
    }

    #[test]
    fn test_formats_string() {
        let formats = SupportedFormats::supported_formats_string();
        // Verify all supported formats are in the string
        for ext in SupportedFormats::EXTENSIONS {
            assert!(formats.contains(ext));
        }
    }
}
