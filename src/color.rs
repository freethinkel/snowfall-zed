use std::str::FromStr;

use palette::{
    rgb::{Rgb, Rgba},
    Clamp, MixAssign, Srgb, Srgba,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    value: Srgba<u8>,
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let hex = if hex.len() == 7 {
            format!("{}ff", hex)
        } else {
            hex.to_string()
        };

        let value = Srgba::from_str(&hex).unwrap();

        return Self { value };
    }

    pub fn with_opacity(&self, amount: f32) -> Self {
        let mut color: Srgba<u8> = Srgba::from(self.value);
        let alpha = (amount * 255.0) as u8;
        color.alpha = alpha;

        return Self { value: color };
    }

    pub fn mix(&self, other: Color, amount: f32) -> Self {
        // Convert self and other colors to Rgba<f32> for mixing
        let mut new_color = self.value.into_format().clone();
        new_color.mix_assign(other.value.into_format(), amount);
        // let new_color = self.value.into_format().mix_assign(other, amount * 255);

        return Self {
            value: new_color.into(),
        };
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TO HEX
        let color: Srgba<u8> = self.value;

        let hex = match color.alpha {
            255 => format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue),
            _ => format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                color.red, color.green, color.blue, color.alpha
            ),
        };

        serializer.serialize_str(&hex)
    }
}
