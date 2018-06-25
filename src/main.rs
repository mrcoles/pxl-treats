extern crate pxl;

use pxl::*;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

const INCR_DOT: i32 = WIDTH / 50;
const INCR_COLOR: f32 = 0.05;

#[derive(Copy, Clone)]
struct Dot {
    x: i32,
    y: i32
}

struct Daisy {
    red: f32,
    green: f32,
    dot: Dot
}

impl Program for Daisy {
    fn new() -> Daisy {
        Daisy {
            red: 0.5,
            green: 0.5,
            dot: Dot { x: WIDTH / 2, y: HEIGHT / 2 }
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (WIDTH as usize, HEIGHT as usize)
    }

    fn render(&mut self, pixels: &mut [Pixel]) {
        for (i, pixel) in pixels.iter_mut().enumerate() {
            *pixel = Pixel {
                red: self.red,
                green: self.green,
                blue: (i as f32 * 0.05 + 1.0) % 1.0,
                alpha: 1.0
            };
        }

        self.draw_dot(pixels);
    }

    fn tick(&mut self, events: &[Event]) {
        let mut red_dir = 0;
        let mut green_dir = 0;
        let mut dot_dx = 0;
        let mut dot_dy = 0;

        for event in events {
            if let Event::Button { state: ButtonState::Pressed, button } = event {
                match button {
                    Button::Up => {
                        red_dir = 1;
                        dot_dy = -1;
                    }
                    Button::Down => {
                        red_dir = -1;
                        dot_dy = 1;
                    }
                    Button::Left => {
                        green_dir = -1;
                        dot_dx = -1;
                    }
                    Button::Right => {
                        green_dir = 1;
                        dot_dx = 1;
                    }
                    _ => {},
                };
            }
        }

        self.dot.x = (self.dot.x + dot_dx * INCR_DOT).max(0).min(WIDTH - 1);
        self.dot.y = (self.dot.y + dot_dy * INCR_DOT).max(0).min(HEIGHT - 1);

        self.red = clampf(self.red + INCR_COLOR * (red_dir as f32), 0.0, 1.0);
        self.green = clampf(self.green + INCR_COLOR * (green_dir as f32), 0.0, 1.0);
    }
}

impl Daisy {
    fn draw_dot(&self, pixels: &mut [Pixel]) {
        let dot = self.dot;
        let color = Pixel { red: 1.0 - self.red, green: 1.0 - self.green, blue: 0.5, alpha: 1.0 };

        for dx in -5..=5 {
            for dy in -5..=5 {
                if let Some(i) = index_of(dot.x + dx, dot.y + dy) {
                    pixels[i] = color;
                }
            }
        }
    }
}

// ## helpers

fn clampf(val: f32, min: f32, max: f32) -> f32 {
    val.min(max).max(min)
}

fn index_of(x: i32, y: i32) -> Option<usize> {
    if x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT {
        None
    } else {
        let index = y * WIDTH + x;
        Some(index as usize)
    }
}

// ## main

fn main() {
    pxl::run::<Daisy>();
}
