use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new_black(); width * height],
        }
    }

    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        &self.pixels[self.get_pixel_index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color.clone();
    }

    pub fn set_center(&mut self, x: usize, y: usize, color: &Color) {
        let index = self.get_pixel_index(x + self.width / 2, self.height / 2 - y);
        self.pixels[index] = color.clone();
    }

    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn for_each<F>(&self, func: F)
    where
        F: Fn(usize, usize),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                func(x, y);
            }
        }
    }

    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut body = String::new();
        let mut line_len = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let rgb = self.get(x, y).rgb();
                let red = rgb.red.to_string();
                let green = rgb.green.to_string();
                let blue = rgb.blue.to_string();

                if line_len + red.len() >= 70 {
                    let str = format!("{}{}", '\n', red);
                    body.push_str(&str);
                    line_len = red.len();
                } else {
                    let str: String = if x == 0 {
                        red
                    } else {
                        format!("{}{}", ' ', red)
                    };

                    body.push_str(&str);
                    line_len = line_len + str.len();
                }

                if line_len + green.len() >= 70 {
                    let str = format!("{}{}", '\n', green);
                    body.push_str(&str);
                    line_len = green.len();
                } else {
                    let str = format!("{}{}", ' ', green);
                    body.push_str(&str);
                    line_len = line_len + str.len();
                }

                if line_len + blue.len() >= 70 {
                    let str = format!("{}{}", '\n', blue);
                    body.push_str(&str);
                    line_len = blue.len();
                } else {
                    let str = format!("{}{}", ' ', blue);
                    body.push_str(&str);
                    line_len = line_len + str.len();
                }

                if x == self.width - 1 {
                    body.push('\n');
                    line_len = 0;
                }
            }
        }

        header + &body
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, Color};

    #[test]
    fn creating_a_canvas() {
        let canvas = Canvas::new(10, 20);
        let black = Color::new_black();

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        canvas.for_each(|x, y| assert_eq!(canvas.get(x, y), &black));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.set(2, 3, &red);

        assert_eq!(c.get(2, 3), &red)
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("5 3"));
        assert_eq!(lines.next(), Some("255"));
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.set(0, 0, &c1);
        c.set(2, 1, &c2);
        c.set(4, 2, &c3);

        let ppm = c.to_ppm();
        let mut lines = ppm.lines().skip(3);

        assert_eq!(lines.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let color = Color::new(1.0, 0.8, 0.6);
        let canvas = Canvas::new_with_color(10, 2, color);

        let ppm = canvas.to_ppm();
        let mut lines = ppm.lines().skip(3);

        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204")
        );
        assert_eq!(
            lines.next(),
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204")
        );
        assert_eq!(
            lines.next(),
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.ends_with('\n'));
    }
}
