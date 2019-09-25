#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32f1::stm32f103;


#[entry]
fn main() -> ! {
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let rcc = board_peripherals.RCC;
    let gpiob = board_peripherals.GPIOB;
    // let afio = board_peripherals.AFIO;
    // unsafe { afio.mapr.write(|w| w.tim1_remap().bits(0)); }
    let tim1 = board_peripherals.TIM1;

    /* GPIO Port B configuration */
    // Enable PB GPIO clock
    rcc.apb2enr.write(|w| w.iopben().enabled());
    gpiob.crh.write(|w| w.mode13().output50().cnf13().alt_push_pull()
                         .mode14().output50().cnf14().alt_push_pull()
                         .mode15().output50().cnf15().alt_push_pull());

    /* TIM1 Configuration */
    // Enable TIM1 clock
    rcc.apb2enr.write(|w| w.tim1en().enabled());
    // Up-counting
    tim1.cr1.write(|w| w.dir().up());
    // Clock prescalaer (16 bit value, max 65,535)
    tim1.psc.write(|w| w.psc().bits(4000 - 1));
    // Auto-realod value, for up counting goes from 0->ARR
    tim1.arr.write(|w| w.arr().bits(100 - 1));
    // PWM Mode 1 output on channel 1, 2, 3
    // Output channel 1, 2, 3 preload enabled
    tim1.ccmr1_output().write(|w| w.oc1m().pwm_mode1()
                                   .oc1pe().enabled()
                                   .oc2m().pwm_mode1()
                                   .oc2pe().enabled());
    tim1.ccmr2_output().write(|w| w.oc3m().pwm_mode1()
                                   .oc3pe().enabled());
    // Enable complementary output of channel 1, 2, 3
    tim1.ccer.write(|w| w.cc1ne().set_bit()
                         .cc1p().clear_bit()
                         .cc2ne().set_bit()
                         .cc2p().clear_bit()
                         .cc3ne().set_bit()
                         .cc3p().clear_bit());
    // Output Compare register for channel 1, 2, 3 (Controls duty cycle)
    tim1.ccr1.write(|w| w.ccr().bits(50));
    tim1.ccr2.write(|w| w.ccr().bits(25));
    tim1.ccr3.write(|w| w.ccr().bits(25));
    // Main output enable
    tim1.bdtr.write(|w| w.moe().enabled());
    // Enable counter
    tim1.cr1.write(|w| w.cen().enabled());

    loop {
        nop();
    }
}
