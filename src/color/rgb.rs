use super::Color;

#[derive(Debug, Clone)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Get a reference to the rgb's red.
    pub fn red(&self) -> u8 {
        self.red
    }

    /// Get a reference to the rgb's green.
    pub fn green(&self) -> u8 {
        self.green
    }

    /// Get a reference to the rgb's blue.
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

fn color_value_to_rgb(value: f64) -> u8 {
    (Color::clamp(value) * 255.0).round() as u8
}

impl From<&Color> for RGB {
    fn from(color: &Color) -> Self {
        RGB::new(
            color_value_to_rgb(color.red()),
            color_value_to_rgb(color.green()),
            color_value_to_rgb(color.blue()),
        )
    }
}
