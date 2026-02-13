unsafe extern "C" {
    fn RESERVED16();
    fn CMC();
    fn DMA_CH0();
    fn DMA_CH1();
    fn DMA_CH2();
    fn DMA_CH3();
    fn DMA_CH4();
    fn DMA_CH5();
    fn DMA_CH6();
    fn DMA_CH7();
    fn ERM0_SINGLE_BIT();
    fn ERM0_MULTI_BIT();
    fn FMU0();
    fn GLIKEY0();
    fn MBC0();
    fn SCG0();
    fn SPC0();
    fn TDET();
    fn WUU0();
    fn CAN0();
    fn CAN1();
    fn FLEXIO();
    fn I3C0();
    fn LPI2C0();
    fn LPI2C1();
    fn LPSPI0();
    fn LPSPI1();
    fn LPUART0();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn USB0();
    fn CDOG0();
    fn CTIMER0();
    fn CTIMER1();
    fn CTIMER2();
    fn CTIMER3();
    fn CTIMER4();
    fn FLEXPWM0_RELOAD_ERROR();
    fn FLEXPWM0_FAULT();
    fn FLEXPWM0_SUBMODULE0();
    fn FLEXPWM0_SUBMODULE1();
    fn FLEXPWM0_SUBMODULE2();
    fn FLEXPWM0_SUBMODULE3();
    fn EQDC0_COMPARE();
    fn EQDC0_HOME();
    fn EQDC0_WATCHDOG();
    fn EQDC0_INDEX();
    fn FREQME0();
    fn LPTMR0();
    fn OS_EVENT();
    fn WAKETIMER0();
    fn UTICK0();
    fn WWDT0();
    fn ADC0();
    fn ADC1();
    fn CMP0();
    fn CMP1();
    fn CMP2();
    fn DAC0();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn LPI2C2();
    fn LPI2C3();
    fn FLEXPWM1_RELOAD_ERROR();
    fn FLEXPWM1_FAULT();
    fn FLEXPWM1_SUBMODULE0();
    fn FLEXPWM1_SUBMODULE1();
    fn FLEXPWM1_SUBMODULE2();
    fn FLEXPWM1_SUBMODULE3();
    fn EQDC1_COMPARE();
    fn EQDC1_HOME();
    fn EQDC1_WATCHDOG();
    fn EQDC1_INDEX();
    fn LPUART5();
    fn MAU();
    fn SMARTDMA();
    fn CDOG1();
    fn PKC();
    fn SGI();
    fn TRNG0();
    fn ADC2();
    fn ADC3();
    fn RTC();
    fn RTC_1HZ();
    fn SLCD();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 122] = [
    Vector {
        _handler: RESERVED16,
    },
    Vector { _handler: CMC },
    Vector { _handler: DMA_CH0 },
    Vector { _handler: DMA_CH1 },
    Vector { _handler: DMA_CH2 },
    Vector { _handler: DMA_CH3 },
    Vector { _handler: DMA_CH4 },
    Vector { _handler: DMA_CH5 },
    Vector { _handler: DMA_CH6 },
    Vector { _handler: DMA_CH7 },
    Vector {
        _handler: ERM0_SINGLE_BIT,
    },
    Vector {
        _handler: ERM0_MULTI_BIT,
    },
    Vector { _handler: FMU0 },
    Vector { _handler: GLIKEY0 },
    Vector { _handler: MBC0 },
    Vector { _handler: SCG0 },
    Vector { _handler: SPC0 },
    Vector { _handler: TDET },
    Vector { _handler: WUU0 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FLEXIO },
    Vector { _handler: I3C0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPI2C0 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPSPI0 },
    Vector { _handler: LPSPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: LPUART0 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: USB0 },
    Vector { _reserved: 0 },
    Vector { _handler: CDOG0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER3 },
    Vector { _handler: CTIMER4 },
    Vector {
        _handler: FLEXPWM0_RELOAD_ERROR,
    },
    Vector {
        _handler: FLEXPWM0_FAULT,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE0,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE1,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE2,
    },
    Vector {
        _handler: FLEXPWM0_SUBMODULE3,
    },
    Vector {
        _handler: EQDC0_COMPARE,
    },
    Vector {
        _handler: EQDC0_HOME,
    },
    Vector {
        _handler: EQDC0_WATCHDOG,
    },
    Vector {
        _handler: EQDC0_INDEX,
    },
    Vector { _handler: FREQME0 },
    Vector { _handler: LPTMR0 },
    Vector { _reserved: 0 },
    Vector { _handler: OS_EVENT },
    Vector {
        _handler: WAKETIMER0,
    },
    Vector { _handler: UTICK0 },
    Vector { _handler: WWDT0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: CMP0 },
    Vector { _handler: CMP1 },
    Vector { _handler: CMP2 },
    Vector { _handler: DAC0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector { _reserved: 0 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector {
        _handler: FLEXPWM1_RELOAD_ERROR,
    },
    Vector {
        _handler: FLEXPWM1_FAULT,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE0,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE1,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE2,
    },
    Vector {
        _handler: FLEXPWM1_SUBMODULE3,
    },
    Vector {
        _handler: EQDC1_COMPARE,
    },
    Vector {
        _handler: EQDC1_HOME,
    },
    Vector {
        _handler: EQDC1_WATCHDOG,
    },
    Vector {
        _handler: EQDC1_INDEX,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPUART5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: MAU },
    Vector { _handler: SMARTDMA },
    Vector { _handler: CDOG1 },
    Vector { _handler: PKC },
    Vector { _handler: SGI },
    Vector { _reserved: 0 },
    Vector { _handler: TRNG0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC2 },
    Vector { _handler: ADC3 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _handler: RTC_1HZ },
    Vector { _handler: SLCD },
];
