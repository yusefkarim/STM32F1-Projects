#![deny(unsafe_code)]
#![no_std]
use cortex_m::asm::nop;
use stm32f1::stm32f103::{FLASH, RCC};


pub fn system_clock_init(flash: &FLASH, rcc: &RCC) {
    // To read data from FLASH memory, the correct number of wait states must
    // be set, two wait states, if 48 MHz < SYSCLK ≤ 72 MHz
    flash.acr.write(|w| w.latency().ws2());

    // Enable the External High Speed oscillator (HSE)
    rcc.cr.modify(|_, w| w.csson().on().hseon().on().hsebyp().not_bypassed());
    while rcc.cr.read().hserdy().is_not_ready() { nop(); }

    // Select HSE as clock source for PLL, then configure PLL as 72 MHZ
    // f = (HSE / HSE_DIV) * PLL Multiplier = (8 MHz / 1) * 9 = 72 MHz
    rcc.cfgr.modify(|_, w| w.pllxtpre().div1()
                            .pllsrc().hse_div_prediv()
                            .pllmul().mul9());

    // Enable the Phase Lock Loop (PLL)
    rcc.cr.modify(|_, w| w.pllon().on());
    while rcc.cr.read().pllrdy().is_not_ready() { nop(); }

    // AHB will run at 72 MHz, APB1 will run at 36 MHz,  APB2 will run at 72 MHz
    rcc.cfgr.modify(|_, w| w.hpre().div1().ppre1().div2().ppre2().div1());

    // Select PLL as system clock
    rcc.cfgr.modify(|_, w| w.sw().pll());
    // Wait until System Clock has switched to PLL
    while !rcc.cfgr.read().sws().is_pll() { nop(); }
}
