#![feature(lang_items, compiler_builtins_lib, asm)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ClockSystem {
    Dma1,
    Sram,
    Flitf,
    Crc,
    GpioA,
    GpioB,
    GpioC,
    GpioD,
    GpioF,
    Ts,
    Syscfg,
    Adc1,
    Tim1,
    Sp1,
    Usart1,
    Timi5,
    Timi6,
    Timi17,
    Dbcmcu,
    Timer2,
    Timer3,
    Timer6,
    Timer14,
    WindowWatchdog,
    Spi2,
    Usart2,
    I2c1,
    I2c2,
    Pwr,
    Dac,
    Cec,
}

#[repr(C)]
struct RccLayout {
    /// RCC clock control register,                                  Address offset: 0x00
    cr: u32,
    /// RCC clock configuration register,                            Address offset: 0x04
    cfgr: u32,
    /// RCC clock interrupt register,                                Address offset: 0x08
    cir: u32,
    /// RCC APB2 peripheral reset register,                          Address offset: 0x0C
    apb2rstr: u32,
    /// RCC APB1 peripheral reset register,                          Address offset: 0x10
    apb1rstr: u32,
    /// RCC AHB peripheral clock register,                           Address offset: 0x14
    ahbenr: u32,
    /// RCC APB2 peripheral clock enable register,                   Address offset: 0x18
    apb2enr: u32,
    /// RCC APB1 peripheral clock enable register,                   Address offset: 0x1C
    apb1enr: u32,
    /// RCC Backup domain control register,                          Address offset: 0x20
    bdcr: u32,
    /// RCC clock control & status register,                         Address offset: 0x24
    csr: u32,
    /// RCC AHB peripheral reset register,                           Address offset: 0x28
    ahbrstr: u32,
    /// RCC clock configuration register 2,                          Address offset: 0x2C
    cfgr2: u32,
    /// RCC clock configuration register 3,                          Address offset: 0x30
    cfgr3: u32,
    /// RCC clock control register 2,                                Address offset: 0x34
    cr2: u32,
}

impl RccLayout {
    pub fn start_clock(&mut self, sys: ClockSystem) {
        use ClockSystem::*;
        match sys {
            Dma1 => self.ahbenr |= 0x0000_0001,
            Sram => self.ahbenr |= 0x0000_0004,
            Flitf => self.ahbenr |= 0x0000_0010,
            Crc => self.ahbenr |= 0x0000_0040,
            GpioA => self.ahbenr |= 0x0002_0000,
            GpioB => self.ahbenr |= 0x0004_0000,
            GpioC => self.ahbenr |= 0x0008_0000,
            GpioD => self.ahbenr |= 0x0010_0000,
            GpioF => self.ahbenr |= 0x0040_0000,
            Ts => self.ahbenr |= 0x0100_0000,
            _ => {}
            // TODO, tag the rest to APB2ENR, APB1ENR registers.
            //
            // Syscfg,
            // Adc1,
            // Tim1,
            // Sp1,
            // Usart1,
            // Timi5,
            // Timi6,
            // Timi17,
            // Dbcmcu,
            // Timer2,
            // Timer3,
            // Timer6,
            // Timer14,
            // WindowWatchdog,
            // Spi2,
            // Usart2,
            // I2c1,
            // I2c2,
            // Pwr,
            // Dac,
            // Cec
            //
        }
    }
}

const RCC: *mut RccLayout = 0x4002_1000 as *mut RccLayout;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    unsafe {
        (*RCC).start_clock(ClockSystem::GpioA);
    }

    unsafe {
        (*GPIO).mode(Pin::P5, Mode::Output);
        (*GPIO).output_type(Pin::P5, OType::PushPull);
        (*GPIO).output_speed(Pin::P5, Speed::High);
        (*GPIO).set_pull_up_down(Pin::P5, Pup::Neither);
    }

    loop {
        unsafe {
            (*GPIO).set_bit(Pin::P5);
        }
        for _ in 0..200000 {
            nop();
        }
        unsafe {
            (*GPIO).unset_bit(Pin::P5);
        }
        for _ in 0..200000 {
            nop();
        }
    }
}

#[inline(never)]
fn nop() {
    unsafe {
        asm!("NOP");
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
