#![feature(lang_items, compiler_builtins_lib, asm)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pin {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Input,
    Output,
    Alternate,
    Analog,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OType {
    PushPull,
    OpenDrain,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pup {
    Neither,
    PullUp,
    PullDown,
}

#[repr(C)]
pub struct GpioLayout {
    /// Port mode register
    moder: u32,
    /// Output type register
    otyper: u32,
    /// Output speed register
    ospeedr: u32,
    /// Pull-up / pull-down register
    pupdr: u32,
    /// Input data register
    idr: u32,
    /// Output data register
    odr: u32,
    /// Bit set / reset register
    bsrr: u32,
    /// Alternate function low register
    lckr: u32,
    /// Alternate function high register
    afr: [u32; 2],
    /// Port bit reset register
    brr: u32,
}

#[inline(always)]
fn set_1bit(reg: &mut u32, n: u32, b: u32) {
    debug_assert!(n < 16);
    debug_assert!(b < 2);

    if b == 0 {
        *reg &= !(1 << n);
    } else {
        *reg |= b << n;
    }
}

#[inline(always)]
fn set_2bit(reg: &mut u32, n: u32, b: u32) {
    debug_assert!(n < 16);
    debug_assert!(b < 4);

    *reg &= !(3 << (n * 2));
    *reg |= b << (n * 2);
}

impl GpioLayout {
    pub fn mode(&mut self, p: Pin, m: Mode) {
        set_2bit(&mut self.moder, p as u32, m as u32);
    }

    pub fn output_type(&mut self, p: Pin, o: OType) {
        set_1bit(&mut self.otyper, p as u32, o as u32);
    }

    pub fn output_speed(&mut self, p: Pin, s: Speed) {
        set_2bit(&mut self.ospeedr, p as u32, s as u32);
    }

    pub fn set_pull_up_down(&mut self, p: Pin, u: Pup) {
        set_2bit(&mut self.ospeedr, p as u32, u as u32);
    }

    pub fn set_bit(&mut self, p: Pin) {
        self.bsrr = 1 << p as u32;
    }

    pub fn unset_bit(&mut self, p: Pin) {
        self.bsrr = 1 << (p as u32) + 16;
    }

    // TODO: Do the rest of the fields
    // See eg. http://hertaville.com/stm32f0-gpio-tutorial-part-1.html
    // for docs.
}



const GPIO: *mut GpioLayout = 0x4800_0000 as *mut GpioLayout;

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

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    let ra = peek(RCC + 0x14);
    poke(RCC + 0x14, ra | 1 << 17); // enable port A

    unsafe {
        (*GPIO).mode(Pin::P5, Mode::Output);
        (*GPIO).output_type(Pin::P5, OType::PushPull);
        (*GPIO).output_speed(Pin::P5, Speed::High);
        (*GPIO).set_pull_up_down(Pin::P5, Pup::Neither);
    }

    loop {
        unsafe { (*GPIO).set_bit(Pin::P5); }
        for _ in 0..200000 {
            nop();
        }
        unsafe { (*GPIO).unset_bit(Pin::P5); }
        for _ in 0..200000 {
            nop();
        }
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
