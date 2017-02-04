use math::{fp, FP};
use vga;
use vga::Color::*;

pub fn draw(x: u32, y: u32) -> vga::Color {
    let x = (fp(x as i32) - fp(48)) / fp(13) / fp(2);
    let y = (fp(y as i32) - fp(32)) / fp(16) / fp(2);
    let m = mandelbrot(x, y);

    if m == 0 {
        White
    } else if m < 10 {
        Blue
    } else if m < 20 {
        Magenta
    } else if m < 30 {
        Red
    } else if m < 40 {
        Green
    } else if m < 50 {
        Yellow
    } else if m < 99 {
        Cyan
    } else {
        Black
    }
}

fn mandelbrot(cx: FP, cy: FP) -> usize {
    const ITER: usize = 100;
    let mut x = fp(0);
    let mut y = fp(0);

    for i in 0..ITER {
        let x2 = x * x - y * y + cx;
        y = fp(2) * x * y + cy;
        x = x2;

        if x * x + y * y > fp(4) {
            return i;
        }
    }

    return ITER;
}
