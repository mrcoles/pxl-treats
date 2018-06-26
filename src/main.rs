extern crate pxl;
extern crate rand;

use pxl::*;
use rand::Rng;

// ## Constants

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;
const DOT_WIDTH: i32 = 11;
const DOT_HEIGHT: i32 = 11;

const INCR_DOT: i32 = WIDTH / 50;
const INCR_COLOR: f32 = 0.05;

// const BLACK: Pixel = Pixel { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
const WHITE: Pixel = Pixel { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };

// ## Traits

trait Renderable {
    /// centered x-position
    fn get_x(&self) -> i32;
    /// centered y-position
    fn get_y(&self) -> i32;
    /// width of element
    fn get_width(&self) -> i32;
    /// height of element
    fn get_height(&self) -> i32;

    fn get_bounds(&self) -> Bounds {
        Bounds {
            left: self.get_x() - self.get_width() / 2,
            right: self.get_x() + self.get_width() / 2,
            top: self.get_y() - self.get_height() / 2,
            bottom: self.get_y() + self.get_height() / 2,
        }
    }

    fn is_touching(&self, r: &Renderable) -> bool {
        self.get_bounds().is_touching(r.get_bounds())
    }
}

// ## Structs

struct Bounds { left: i32, right: i32, top: i32, bottom: i32 }

impl Bounds {
    fn is_touching(&self, b: Bounds) -> bool {
        let bad_x = self.left > b.right || self.right < b.left;
        let bad_y = self.top > b.bottom || self.bottom < b.top;
        !bad_x && !bad_y
    }
}

// Dot

#[derive(Copy, Clone)]
struct Dot {
    x: i32,
    y: i32
}

impl Renderable for Dot {
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
    fn get_width(&self) -> i32 {
        DOT_WIDTH
    }
    fn get_height(&self) -> i32 {
        DOT_HEIGHT
    }
}

// Target

#[derive(Copy, Clone)]
struct Target {
    x: i32,
    y: i32,
}

impl Target {
    fn new() -> Target {
        let x = rand::thread_rng().gen_range(5, WIDTH - 5);
        let y = rand::thread_rng().gen_range(5, HEIGHT - 5);

        Target {
            x: x,
            y: y,
        }
    }
}

impl Renderable for Target {
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
    fn get_width(&self) -> i32 {
        DOT_WIDTH
    }
    fn get_height(&self) -> i32 {
        DOT_HEIGHT
    }
}

// Daisy

struct Daisy {
    red: f32,
    green: f32,
    dot: Dot,
    target: Target
}

impl Program for Daisy {
    fn new() -> Daisy {
        Daisy {
            red: 0.5,
            green: 0.5,
            dot: Dot { x: WIDTH / 2, y: HEIGHT / 2 },
            target: Target::new()
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

        let color = Pixel {
            red: 1.0 - self.red,
            green: 1.0 - self.green,
            blue: 0.5, alpha: 1.0,
        };
        self.draw(pixels, color, &self.dot);

        self.draw(pixels, WHITE, &self.target);
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

        if self.dot.is_touching(&self.target) {
            self.target = Target::new();
        }
    }
}

impl Daisy {
    fn draw(&self, pixels: &mut [Pixel], color: Pixel, r: &Renderable) {
        let width = r.get_width();
        let height = r.get_height();
        let x = r.get_x();
        let y = r.get_y();

        let x_start = x - width / 2;
        let x_end = x + width / 2;

        let y_start = y - height / 2;
        let y_end = y + height / 2;

        for tx in x_start..=x_end {
            for ty in y_start..=y_end {
                if let Some(i) = index_of(tx, ty) {
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
