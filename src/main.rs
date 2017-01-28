#![feature(lang_items, compiler_builtins_lib, asm)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

fn peek(addr: u32) -> u32 {
    unsafe { *(addr as usize as *const u32) }
}

fn poke(addr: u32, word: u32) {
    unsafe {
        let addr = addr as usize as *mut u32;
        *addr = word;
    }
}

#[inline(never)]
fn nop() {
    unsafe {
        asm!("NOP");
    }
}

const RCC: u32 = 0x4002_1000;
const GPIOA: u32 = 0x4800_0000;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    let ra = peek(RCC + 0x14);
    poke(RCC + 0x14, ra | 1<<17); // enable port A

    // magic stuff
    let mut ra = peek(GPIOA);
    ra &= !(3 << 10);
    ra |= 1<<10;
    poke(GPIOA, ra);

    let mut ra = peek(GPIOA + 0x4);
    ra &= !(1<<5);
    poke(GPIOA + 0x4, ra);

    let mut ra = peek(GPIOA + 0x8);
    ra |= 3<<10;
    poke(GPIOA + 0x8, ra);

    let mut ra = peek(GPIOA + 0xc);
    ra &= !(3<<10);
    poke(GPIOA + 0xc, ra);

    poke(GPIOA + 0x18, (1<<5) << 0);
    loop {
        poke(GPIOA + 0x18, (1<<5) << 0);
        for _ in 0..200000 { nop(); }
        poke(GPIOA + 0x18, (1<<5) << 16);
        for _ in 0..200000 { nop(); }
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
