//! Core types used throughout the configuration system.
//!
//! This module provides type-safe wrappers around primitive types to ensure
//! configuration values are valid and consistent.

use crate::error::{ConfigError, Result};
use serde::{Deserialize, Serialize};

/// Represents an RGBA color with validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    /// Creates a new color from RGBA components
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    /// Creates a color from a hexadecimal string (e.g., "#FF0000FF")
    pub fn from_hex(hex: &str) -> Result<Self> {
        if !hex.starts_with('#') || hex.len() != 9 {
            return Err(ConfigError::ColorError(
                "Invalid hex color format. Expected '#RRGGBBAA'".to_string(),
            ));
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| {
            ConfigError::ColorError("Invalid red component".to_string())
        })?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| {
            ConfigError::ColorError("Invalid green component".to_string())
        })?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| {
            ConfigError::ColorError("Invalid blue component".to_string())
        })?;
        let a = u8::from_str_radix(&hex[7..9], 16).map_err(|_| {
            ConfigError::ColorError("Invalid alpha component".to_string())
        })?;

        Ok(Self::new(r, g, b, a))
    }

    /// Converts the color to a hexadecimal string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

/// Represents a 2D vector with validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    /// Creates a new vector with validation
    pub fn new(x: f64, y: f64) -> Result<Self> {
        if x.is_finite() && y.is_finite() {
            Ok(Self {
                x,
                y,
            })
        } else {
            Err(ConfigError::ValidationError(
                "Vector components must be finite numbers".to_string(),
            ))
        }
    }

    /// Gets the x component
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Gets the y component
    pub fn y(&self) -> f64 {
        self.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Default for Position {
    fn default() -> Self {
        Position::TopRight
    }
}
/// Represents a mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

/// Re-export eframe types for consistency
pub use eframe::egui::{Color32, Key};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let color = ColorRGBA::new(255, 128, 0, 255);
        assert_eq!(color.to_hex(), "#FF8000FF");

        let parsed = ColorRGBA::from_hex("#FF8000FF").unwrap();
        assert_eq!(color, parsed);
    }

    #[test]
    fn test_vector_validation() {
        assert!(Vector2D::new(1.0, 2.0).is_ok());
        assert!(Vector2D::new(f64::INFINITY, 2.0).is_err());
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "&str", into = "String")]
pub struct SerializableKey(Key);

impl From<&str> for SerializableKey {
    fn from(s: &str) -> Self {
        // Implement key string parsing
        Self(match s {
            "Equals" => Key::Equals,
            "Plus" => Key::Plus,
            "Minus" => Key::Minus,
            "W" => Key::W,
            "S" => Key::S,
            "F" => Key::F,
            "Q" => Key::Q,
            "Num0" => Key::Num0,
            _ => Key::Equals, // Default
        })
    }
}

impl From<SerializableKey> for String {
    fn from(key: SerializableKey) -> Self {
        match key.0 {
            Key::Equals => "Equals",
            Key::Plus => "Plus",
            Key::Minus => "Minus",
            Key::W => "W",
            Key::S => "S",
            Key::F => "F",
            Key::Num0 => "Num0",
            _ => "Equal",
        }
        .to_string()
    }
}
