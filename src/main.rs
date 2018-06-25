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
            red: 1.0,
            green: 0.0,
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
            if let Event::Button {
                state: ButtonState::Pressed,
                button: Button::Up
            } = event {
                // up
                red_dir = 1;
            } else if let Event::Button {
                state: ButtonState::Pressed,
                button: Button::Down
            } = event {
                // down
                red_dir = -1;
            } else if let Event::Button {
                state: ButtonState::Pressed,
                button: Button::Right
            } = event {
                // up
                green_dir = 1;
            } else if let Event::Button {
                state: ButtonState::Pressed,
                button: Button::Left
            } = event {
                // down
                green_dir = -1;
            }
        }

        self.red += INCR * (red_dir as f32);
        self.green += INCR * (green_dir as f32);
    }
}

// helpers

// TODO - clamp



//

fn main() {
    pxl::run::<Daisy>();
}
