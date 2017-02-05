extern crate fixray;
extern crate termion;

use fixray::*;

struct TermDriver;

impl Driver for TermDriver {
    fn screen_size(&self) -> (u32, u32) {
        (64, 64)
    }

    fn draw_screen<F>(&self, pixel_f: F)
        where F: Fn(u32, u32) -> Color
    {
        let (w, h) = self.screen_size();
        let h = h / 2;
        for line in 0..h {
            for x in 0..w {
                let back = pixel_f(x, line * 2);
                let fore = pixel_f(x, line * 2 + 1);

                use termion::color;

                match back {
                    Color::Black => print!("{}", color::Bg(color::Black)),
                    Color::Red => print!("{}", color::Bg(color::Red)),
                    Color::Green => print!("{}", color::Bg(color::Green)),
                    Color::Yellow => print!("{}", color::Bg(color::Yellow)),
                    Color::Blue => print!("{}", color::Bg(color::Blue)),
                    Color::Magenta => print!("{}", color::Bg(color::Magenta)),
                    Color::Cyan => print!("{}", color::Bg(color::Cyan)),
                    Color::White => print!("{}", color::Bg(color::White)),
                };

                match fore {
                    Color::Black => print!("{}", color::Fg(color::Black)),
                    Color::Red => print!("{}", color::Fg(color::Red)),
                    Color::Green => print!("{}", color::Fg(color::Green)),
                    Color::Yellow => print!("{}", color::Fg(color::Yellow)),
                    Color::Blue => print!("{}", color::Fg(color::Blue)),
                    Color::Magenta => print!("{}", color::Fg(color::Magenta)),
                    Color::Cyan => print!("{}", color::Fg(color::Cyan)),
                    Color::White => print!("{}", color::Fg(color::White)),
                };
                print!("▄");
            }
            println!("{}", termion::style::Reset);
        }
    }
}

fn main() {
    use Color::*;
    let scene = Scene
        + Object::new(sphere_fn(v3(0, 10, 2), FP(3)), m(Material::Surface(Red, Red, Black)))
        + Object::new(plane_fn(v3(0, 0, 1), FP(0)), m(Material::Checkerboard(Green, White)))
        ;

    let frustum = Frustum {
        origin: v3(0, 0, 3),
        dir: v3(1, 2, 0).normalized(),
        up: v3(0, 0, 1),
    };


    TermDriver.draw_screen(|x, y| {
        trace(&scene, &frustum.ray(x, y))
        /*
        match (x + y) % 4 {
            0 => Color::Red,
            1 => Color::Yellow,
            2 => Color::Green,
            3 => Color::Blue,
            _ => Color::White,
        }
        */
    });
}
