use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

const MAX_COLOR_VALUE: i32 = 255;
const MAX_PPM_LINE_LENGTH: usize = 70;

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0., 0., 0.); width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        if let Some(index) = self.get_index(x, y) {
            self.pixels[index] = color;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        let in_bounds = 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32;

        if in_bounds {
            Some(x as usize + y as usize * self.width)
        } else {
            None
        }
    }

    pub fn to_ppm(&self) -> String {
        let ppm_header = format!("P3\n{} {}\n{}", self.width, self.height, MAX_COLOR_VALUE);

        let ppm_body: String = self
            .pixels
            .chunks(self.width)
            .map(process_row)
            .collect::<Vec<_>>()
            .join("\n");

        ppm_header + "\n" + &ppm_body + "\n"
    }
}

fn process_row(row: &[Color]) -> String {
    row.iter()
        .fold((0, String::new()), |accum, color| {
            process_pixel(accum, *color)
        })
        .1
}


fn process_pixel(
    (mut char_count, mut result_string): (usize, String),
    pixel: Color,
) -> (usize, String) {
    let scaled_pixel = pixel * (MAX_COLOR_VALUE as f64);

    let red = format_scaled_color(scaled_pixel.red);
    let green = format_scaled_color(scaled_pixel.green);
    let blue = format_scaled_color(scaled_pixel.blue);

    for component in [red, green, blue].iter() {
        if char_count + component.len() + 1 > MAX_PPM_LINE_LENGTH {
            result_string += "\n";
            char_count = 0;
        } else if char_count != 0 {
            result_string += " ";
            char_count += 1;
        }
        result_string += &component;
        char_count += component.len();
    }

    (char_count, result_string)
}

fn format_scaled_color(color_component: f64) -> String {
    format!(
        "{}",
        color_component.clamp(0., MAX_COLOR_VALUE as f64).round() as i16
    )
}
