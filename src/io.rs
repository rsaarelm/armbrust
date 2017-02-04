use stm32f030r8 as board;

pub fn putc(uart: *mut board::UsartLayout, c: char) {
    unsafe {
        (*uart).send(c as u16);
    }
}

pub fn puts(uart: *mut board::UsartLayout, s: &str) {
    for c in s.chars() {
        putc(uart, c);
    }
}

/// Print an integer.
pub fn puti(uart: *mut board::UsartLayout, mut i: i32) {
    if i < 0 {
        putc(uart, '-');
        i = -i;
    }
    if i == 0 {
        putc(uart, '0');
        return;
    }
    if i >= 10 {
        let prefix = i / 10;
        puti(uart, prefix);
        i -= prefix * 10;
    }

    debug_assert!(i < 10);

    putc(uart, ('0' as u8 + i as u8) as char);
}

pub fn getc(uart: *mut board::UsartLayout) -> char {
    unsafe { (*uart).recv() as u8 as char }
}
