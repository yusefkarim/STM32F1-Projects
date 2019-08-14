#include "stm32f103xb.h"

void SysTick_Handler(void) {
    GPIOC->ODR ^= GPIO_ODR_ODR13;
}


int main(void) {

    /* ----- GPIO PB13 Setup ----- */
	RCC->APB2ENR |= RCC_APB2ENR_IOPBEN;
	RCC->APB2ENR |= RCC_APB2ENR_AFIOEN;
	// Set mode of pin as output, alternate function push-pull (mode 10)
	GPIOB->CRH &= ~GPIO_CRH_MODE13;
	GPIOB->CRH |= GPIO_CRH_MODE13_0;
    GPIOB->CRH &= ~GPIO_CRH_CNF13;
    GPIOB->CRH |= GPIO_CRH_CNF13_1;
	// Choose PB13 through the Event Control Register
	AFIO->EVCR &= ~AFIO_EVCR_PIN;
	AFIO->EVCR |= (AFIO_EVCR_PORT_PB | AFIO_EVCR_PIN_PX13);
	// Ensure default mapping, which has PB13 mapped to CH1N
	AFIO->MAPR &= ~AFIO_MAPR_TIM1_REMAP;


    /* ----- TIM1 CH1 Setup ----- */
    // Enable TIM1 clock
    RCC->APB2ENR |= RCC_APB2ENR_TIM1EN;
    // Set count direction as up-counting
    TIM1->CR1 &= ~TIM_CR1_DIR;
    // Clock prescalaer (16 bit value, max 65,535)
    TIM1->PSC = 4000 - 1;
    // Auto-realod value, for up counting goes from 0->ARR
    TIM1->ARR = 100 - 1;
    // Capture/compare register can be any value 0 < CCR < ARR
    TIM1->CCR1 = 50;
    // Main output enable (MOE): 0 = Disable, 1 = Enable
	TIM1->BDTR |= TIM_BDTR_MOE;
	// Clear output compare mode bits of channel 1
	TIM1->CCMR1 &= ~TIM_CCMR1_OC1M;
	// Select toggle mode (0011)
	TIM1->CCMR1 |= TIM_CCMR1_OC1M_0 | TIM_CCMR1_OC1M_1;
	// Select output polarity: 0 = active high, 1 = active low
	TIM1->CCER &= ~TIM_CCER_CC1NP;
	// Enable output for channel 1 complementary output
	TIM1->CCER |= TIM_CCER_CC1NE;
	// Enable TIM1
	TIM1->CR1 |= TIM_CR1_CEN;


    /* ----- GPIO PC13 Setup ----- */
    RCC->APB2ENR |= RCC_APB2ENR_IOPCEN;
    // Set output mode as 01, for low speed output mode
    GPIOC->CRH &= ~GPIO_CRH_MODE13;
    GPIOC->CRH |= GPIO_CRH_MODE13_0;
    // Clearing CNF bits configures pins as push-pull output
    GPIOC->CRH &= ~GPIO_CRH_CNF13;
    // Turn of the LED initially (The User LED is HIGH when PC13 is LOW)
    GPIOC->ODR |= GPIO_ODR_ODR13;

    /* ----- Enabling SysTick with interrupt ----- */
    // Disable SysTick IRQ and SysTick counter
    SysTick->CTRL = 0;                            
    // Set reload register
    SysTick->LOAD = 20000 - 1;
    // Set interrupt priority of SysTick as least urgent
    // (highest priority number). This function is defined in core_cm3.h
    NVIC_SetPriority(SysTick_IRQn, (1<<__NVIC_PRIO_BITS) - 1);
    // Reset the SysTick counter value (by writing to it)         
    SysTick->VAL = 0;
    // Select processor clock
    // 1 = processor clock, 0 = external clock
    SysTick->CTRL |= SysTick_CTRL_CLKSOURCE_Msk;
    // Enables SysTick exception request            
    // 1 = counting down to zero asserts the SysTick exception request
    // 0 = counting down to zero does not assert the SysTick exception request
    SysTick->CTRL |= SysTick_CTRL_TICKINT_Msk;                            
    // Enable SysTick timer                       
    SysTick->CTRL |= SysTick_CTRL_ENABLE_Msk;

    while(1) { __NOP(); }
}
