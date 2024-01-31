use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn yellow() -> Color {
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.0,
        }
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}
impl Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl From<cgmath::Vector3<f32>> for Color {
    fn from(value: cgmath::Vector3<f32>) -> Self {
        Self {
            r: value.x.into(),
            g: value.y.into(),
            b: value.z.into(),
        }
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        data.resize(width * height, Color::black());
        Self {
            width,
            height,
            data,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x]
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Color {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.data[y * self.width + x]
    }
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[x + y * self.height] = color;
    }

    pub fn print_ansi_rgb_spaces(&self) -> String {
        let mut result = String::new();
        // ESC[48;2;r;g;bm
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                let quantize = |val: f32| f32::round(val.clamp(0.0, 1.0) * 256.0) as i32;
                let red = quantize(c.r);
                let green = quantize(c.g);
                let blue = quantize(c.b);
                result.push_str(&format!("\x1b[48;2;{red};{green};{blue}m "));
            }
            result.push_str("\x1b[0m\n")
        }
        result
    }
    pub fn print_ansi_rgb_halfblock(&self) -> String {
        let mut result = String::new();
        let quantize = |val: f32| (f32::round(val * 256.0) as i32).clamp(0, 255);
        // ESC[48;2;r;g;bm
        result.push_str("\x1b[0m");
        for yhalf in 0..self.height / 2 {
            let y1 = yhalf * 2;
            let y2 = yhalf * 2 + 1;
            for x in 0..self.width {
                {
                    let c = self.get(x, y1);
                    let red = quantize(c.r);
                    let green = quantize(c.g);
                    let blue = quantize(c.b);
                    result.push_str(&format!("\x1b[38;2;{red};{green};{blue}m"));
                }
                {
                    let c = self.get(x, y2);
                    let red = quantize(c.r);
                    let green = quantize(c.g);
                    let blue = quantize(c.b);
                    result.push_str(&format!("\x1b[48;2;{red};{green};{blue}m"));
                }
                result.push('▀');
            }
            result.push_str("\x1b[0m\n")
        }

        if self.height % 2 == 1 {
            for x in 0..self.width {
                let c = self.get(x, self.height - 1);
                let red = quantize(c.r);
                let green = quantize(c.g);
                let blue = quantize(c.b);
                result.push_str(&format!("\x1b[38;2;{red};{green};{blue}m"));

                result.push('▀');
            }
            result.push_str("\x1b[0m\n")
        }

        result
    }
}
