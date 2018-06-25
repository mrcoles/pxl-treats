extern crate pxl;

use pxl::*;

const INCR: f32 = 0.1;

struct Daisy {
    red: f32,
    green: f32,
}

impl Program for Daisy {
    fn new() -> Daisy {
        Daisy {
            red: 0.5,
            green: 0.5,
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (512, 512)
    }

    fn render(&mut self, pixels: &mut [Pixel]) {
        for (i, pixel) in pixels.iter_mut().enumerate() {
            *pixel = Pixel{
                red: self.red,
                green: self.green,
                blue: (i as f32 * 0.01 + 1.0) % 1.0,
                alpha: 1.0
            };
        }
    }

    fn tick(&mut self, events: &[Event]) {
        let mut red_dir = 0;
        let mut green_dir = 0;

        for event in events {
            if let Event::Button { state: ButtonState::Pressed, button } = event {
                match button {
                    Button::Down => red_dir = -1,
                    Button::Up => red_dir = 1,
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

// ## helpers

fn clampf(val: f32, min: f32, max: f32) -> f32 {
    val.min(max).max(min)
}

// ## main

fn main() {
    pxl::run::<Daisy>();
}
