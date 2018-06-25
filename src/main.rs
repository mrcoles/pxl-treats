extern crate pxl;

use pxl::*;

const INCR: f32 = 0.1;
const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

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
        (512, 512)
    }

    fn render(&mut self, pixels: &mut [Pixel]) {
        for (i, pixel) in pixels.iter_mut().enumerate() {
            *pixel = Pixel {
                red: self.red,
                green: self.green,
                blue: (i as f32 * 0.01 + 1.0) % 1.0,
                alpha: 1.0
            };
        }

        self.draw_dot(pixels);
    }

    fn tick(&mut self, events: &[Event]) {
        let mut red_dir = 0;
        let mut green_dir = 0;

        for event in events {
            if let Event::Button { state: ButtonState::Pressed, button } = event {
                match button {
                    Button::Up => red_dir = 1,
                    Button::Down => red_dir = -1,
                    Button::Left => green_dir = -1,
                    Button::Right => green_dir = 1,
                    _ => {},
                };
            }
        }

        self.red = clampf(self.red + INCR * (red_dir as f32), 0.0, 1.0);
        self.green = clampf(self.green + INCR * (green_dir as f32), 0.0, 1.0);
    }
}

impl Daisy {
    fn draw_dot(&self, pixels: &mut [Pixel]) {
        let dot = self.dot;

        for dx in -5..5 {
            for dy in -5..5 {
                let i = index_of(dot.x + dx, dot.y + dy);
                pixels[i] = Pixel {
                    red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0
                }
            }
        }
    }
}

// ## helpers

fn clampf(val: f32, min: f32, max: f32) -> f32 {
    val.min(max).max(min)
}

fn index_of(x: i32, y: i32) -> usize {
    (y * WIDTH + x) as usize
}

// ## main

fn main() {
    pxl::run::<Daisy>();
}
