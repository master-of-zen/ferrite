use std::ffi::OsStr;

/// Represents supported image formats and their file extensions.
/// This struct provides easy access to valid image formats and helper methods
/// for format validation.
pub struct SupportedFormats;

impl SupportedFormats {
    /// List of supported image extensions in lowercase.
    /// These match the formats that the `image` crate can decode.
    pub const EXTENSIONS: &'static [&'static str] =
        &["jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "tga", "webp"];

    /// Checks if a given file extension is supported by the image viewer.
    /// The check is case-insensitive to handle files with uppercase extensions.
    ///
    /// # Arguments
    /// * `extension` - The file extension to check, as an OsStr
    ///
    /// # Returns
    /// `true` if the extension represents a supported image format
    pub fn is_supported(extension: Option<&OsStr>) -> bool {
        extension
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .map(|e| Self::EXTENSIONS.contains(&e.as_str()))
            .unwrap_or(false)
    }

    /// Gets a formatted string of all supported extensions for display
    /// purposes. Useful for error messages or UI elements that need to show
    /// supported formats.
    ///
    /// # Returns
    /// A string like "jpg, jpeg, png, gif, bmp, ico, tiff, tga, webp"
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
