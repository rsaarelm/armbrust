extern crate fixray;
extern crate termion;

use termion::color::*;
use fixray::Driver;

struct TermDriver;

fn print_color(b: fixray::Color, f: fixray::Color) {
    match b {
        fixray::Color::Black => print!("{}", Bg(Black)),
        fixray::Color::Red => print!("{}", Bg(Red)),
        fixray::Color::Green => print!("{}", Bg(Green)),
        fixray::Color::Yellow => print!("{}", Bg(Yellow)),
        fixray::Color::Blue => print!("{}", Bg(Blue)),
        fixray::Color::Magenta => print!("{}", Bg(Magenta)),
        fixray::Color::Cyan => print!("{}", Bg(Cyan)),
        fixray::Color::White => print!("{}", Bg(White)),
    };

    match f {
        fixray::Color::Black => print!("{}", Fg(Black)),
        fixray::Color::Red => print!("{}", Fg(Red)),
        fixray::Color::Green => print!("{}", Fg(Green)),
        fixray::Color::Yellow => print!("{}", Fg(Yellow)),
        fixray::Color::Blue => print!("{}", Fg(Blue)),
        fixray::Color::Magenta => print!("{}", Fg(Magenta)),
        fixray::Color::Cyan => print!("{}", Fg(Cyan)),
        fixray::Color::White => print!("{}", Fg(White)),
    };
}

impl fixray::Driver for TermDriver {
    fn screen_size(&self) -> (u32, u32) { (64, 64) }

    fn draw_screen<F>(&self, pixel_f: F) where F: Fn(u32, u32) -> fixray::Color {
        let (w, h) = self.screen_size();
        let h = h / 2;
        for line in 0..h {
            for x in 0..w {
                let back = pixel_f(x, line * 2);
                let fore = pixel_f(x, line * 2 + 1);
                print_color(back, fore);
                print!("▄");
            }
            println!("{}", termion::style::Reset);
        }
    }
}

fn main() {
    use fixray::Color;
    TermDriver.draw_screen(|x, y| {
        match (x + y) % 4 {
            0 => Color::Red,
            1 => Color::Yellow,
            2 => Color::Green,
            3 => Color::Blue,
            _ => Color::White,
        }
    });
}
