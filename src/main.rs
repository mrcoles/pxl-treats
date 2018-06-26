extern crate pxl;

use pxl::*;

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
}

// ## Structs

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
    is_alive: bool,
    death_steps: i8
}

impl Target {
    fn new(_x: i32, _y: i32) -> Target {
        Target {
            x: _x,
            y: _x,
            is_alive: true,
            death_steps: 10
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
            target: Target::new(0, 0)
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

        self.target.x = 20;
        self.target.y = 20;
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

    fn is_collision(&self) -> bool {
        
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
