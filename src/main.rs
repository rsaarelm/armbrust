#![feature(lang_items, compiler_builtins_lib)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    loop {}
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
