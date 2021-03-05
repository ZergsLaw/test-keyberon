//! examples/destructure.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use lm3s6965::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
const APP: () = {
    struct Resources {
        // Some resources to work with
        #[init(0)]
        a: u32,
        #[init(0)]
        b: u32,
        #[init(0)]
        c: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::UART0);
        rtic::pend(Interrupt::UART1);
    }

    // Direct destructure
    #[task(binds = UART0, resources = [a, b, c])]
    fn uart0(cx: uart0::Context) {
        let a = cx.resources.a;
        let b = cx.resources.b;
        let c = cx.resources.c;

        hprintln!("UART0: a = {}, b = {}, c = {}", a, b, c).unwrap();
    }

    // De-structure-ing syntax
    #[task(binds = UART1, resources = [a, b, c])]
    fn uart1(cx: uart1::Context) {
        let uart1::Resources { a, b, c } = cx.resources;

        hprintln!("UART0: a = {}, b = {}, c = {}", a, b, c).unwrap();
    }
};
