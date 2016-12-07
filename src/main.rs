#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    write("Hello, world!\n");

    loop {}
}

pub fn write(text: &str) {
    const UART0: u32 = 0x4000C000;

    for c in text.chars() {
        unsafe { *(UART0 as *mut u32) = c as u32 }
    }
}

mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
