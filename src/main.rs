#![feature(lang_items, compiler_builtins_lib, asm)]
#![feature(core_intrinsics)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

use core::intrinsics::{volatile_load, volatile_store};

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

pub const CLOCK_FREQ_HZ: u32 = 8_000_000;

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

// TODO: These should encode the GPIO id too, A, B, C etc...

pub const USER_LED: Pin = Pin::P5;
pub const USART1_TX: Pin = Pin::P9;
pub const USART1_RX: Pin = Pin::P10;
pub const USART2_TX: Pin = Pin::P2;
pub const USART2_RX: Pin = Pin::P3;

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

    pub fn set(&mut self, p: Pin) {
        self.bsrr = 1 << p as u32;
    }

    pub fn unset(&mut self, p: Pin) {
        self.bsrr = 1 << (p as u32) + 16;
    }

    // TODO: What's the value here?
    pub fn alternate_function(&mut self, p: Pin, n: u32) {
        debug_assert!(n < 16);
        let p = p as u32;
        if p < 8 {
            self.afr[0] &= !(15 << (p * 4));
            self.afr[0] |= n << (p * 4);
        } else {
            let p = p - 8;
            self.afr[1] &= !(15 << (p * 4));
            self.afr[1] |= n << (p * 4);
        }
    }

    // TODO: Do the rest of the fields
    // See eg. http://hertaville.com/stm32f0-gpio-tutorial-part-1.html
    // for docs.
}

const GPIOA: *mut GpioLayout = 0x4800_0000 as *mut GpioLayout;

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
pub struct RccLayout {
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
    /// Start clock for a system.
    pub fn start(&mut self, sys: ClockSystem) {
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
            // Syscfg,
            // Adc1,
            // Tim1,
            // Sp1,
            Usart1 => self.apb2enr |= 0x0000_4000,
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
            Usart2 => self.apb1enr |= 0x0002_0000,
            // I2c1,
            // I2c2,
            // Pwr,
            // Dac,
            // Cec
            _ => {} // TODO: Fill in the rest of the constants and remove this.
        }
    }

    /// Reset clock for a system.
    pub fn reset(&mut self, sys: ClockSystem) {
        use ClockSystem::*;
        match sys {
            GpioA => self.ahbrstr |= 0x0002_0000,
            GpioB => self.ahbrstr |= 0x0004_0000,
            GpioC => self.ahbrstr |= 0x0008_0000,
            GpioD => self.ahbrstr |= 0x0001_0000,
            GpioF => self.ahbrstr |= 0x0004_0000,
            Ts => self.ahbrstr |= 0x0010_0000,
            Usart1 => self.apb2rstr |= 0x0000_4000,
            Usart2 => self.apb1rstr |= 0x0002_0000,
            // TODO: The rest
            _ => {} // The catch-all is needed, this does not cover all items `start` does.
        }
    }
}

const RCC: *mut RccLayout = 0x4002_1000 as *mut RccLayout;


#[repr(C)]
pub struct UsartLayout {
    cr1: u32,
    cr2: u32,
    cr3: u32,
    // Baud rate
    brr: u16,
    _reserved1: u16,
    gtpr: u16,
    _reserved2: u16,
    rtor: u32,
    rqr: u16,
    _reserved3: u16,
    isr: u32,
    icr: u32,
    rdr: u16,
    _reserved4: u16,
    tdr: u16,
    _reserved5: u16,
}

impl UsartLayout {
    pub fn send(&mut self, c: u16) {
        unsafe {
            loop {
                if volatile_load(&self.isr) & (1<<7) != 0 { break; }
            }
        }

        self.tdr = c;
    }

    pub fn recv(&self) -> u16 {
        unsafe {
            loop {
                if volatile_load(&self.isr) & (1<<5) != 0 { break; }
            }
        }

        self.rdr
    }

    pub fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.send(c as u16);
        }
    }
}

const USART1: *mut UsartLayout = 0x4001_3800 as *mut UsartLayout;
const USART2: *mut UsartLayout = 0x4000_4400 as *mut UsartLayout;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    unsafe {
        (*RCC).start(ClockSystem::GpioA);
        (*RCC).start(ClockSystem::Usart1);

        // TODO: Pinout config API that merges the settings for different outputs to one mask so
        // that there are fewer GPIO writes.

        (*GPIOA).mode(USER_LED, Mode::Output);
        (*GPIOA).output_type(USER_LED, OType::PushPull);
        (*GPIOA).output_speed(USER_LED, Speed::High);
        (*GPIOA).set_pull_up_down(USER_LED, Pup::Neither);

        (*GPIOA).mode(USART1_TX, Mode::Alternate);
        (*GPIOA).mode(USART1_RX, Mode::Alternate);
        (*GPIOA).output_type(USART1_TX, OType::PushPull);
        (*GPIOA).output_type(USART1_RX, OType::PushPull);
        (*GPIOA).output_speed(USART1_TX, Speed::High);
        (*GPIOA).output_speed(USART1_RX, Speed::High);
        (*GPIOA).set_pull_up_down(USART1_TX, Pup::Neither);
        (*GPIOA).set_pull_up_down(USART1_RX, Pup::Neither);
        (*GPIOA).alternate_function(USART1_TX, 1);
        (*GPIOA).alternate_function(USART1_RX, 1);

        (*RCC).reset(ClockSystem::Usart1);

        (*USART1).brr = (CLOCK_FREQ / 9600) as u16;
        (*USART1).cr1 = 0b111;

        //(*USART1).print("\x1B[m\x1B[2J");
        loop {
            //(*USART1).print("test");
            (*GPIOA).set(USER_LED);
            busy_wait(200000);
            (*GPIOA).unset(USER_LED);
            busy_wait(200000);
        }
    }
}

fn busy_wait(count: usize) {
    for _ in 0..count {
        nop();
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
