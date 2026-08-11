unsafe extern "C" {
    fn RESERVED16();
    fn CMC();
    fn DMA0_CH0();
    fn DMA0_CH1();
    fn DMA0_CH2();
    fn DMA0_CH3();
    fn DMA0_CH4();
    fn DMA0_CH5();
    fn DMA0_CH6();
    fn DMA0_CH7();
    fn ERM0_SINGLE_BIT();
    fn ERM0_MULTI_BIT();
    fn FMU0();
    fn GLIKEY0();
    fn MBC0();
    fn SCG0();
    fn SPC0();
    fn VBAT0();
    fn WUU0();
    fn CAN0();
    fn RESERVED36();
    fn RESERVED37();
    fn RESERVED38();
    fn FLEXIO();
    fn I3C0();
    fn RESERVED41();
    fn LPI2C0();
    fn LPI2C1();
    fn LPSPI0();
    fn LPSPI1();
    fn RESERVED46();
    fn LPUART0();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn USB0();
    fn RESERVED53();
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
    fn RESERVED65();
    fn QDC0_COMPARE();
    fn QDC0_HOME();
    fn QDC0_WATCHDOG();
    fn QDC0_INDEX();
    fn FREQME0();
    fn LPTMR0();
    fn RESERVED72();
    fn OS_EVENT();
    fn WAKETIMER0();
    fn UTICK0();
    fn WWDT0();
    fn RESERVED77();
    fn ADC0();
    fn ADC1();
    fn CMP0();
    fn CMP1();
    fn RESERVED82();
    fn DAC0();
    fn RESERVED84();
    fn RESERVED85();
    fn RESERVED86();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn RESERVED92();
    fn LPI2C2();
    fn LPI2C3();
    fn FLEXPWM1_RELOAD_ERROR();
    fn FLEXPWM1_FAULT();
    fn FLEXPWM1_SUBMODULE0();
    fn FLEXPWM1_SUBMODULE1();
    fn FLEXPWM1_SUBMODULE2();
    fn RESERVED100();
    fn QDC1_COMPARE();
    fn QDC1_HOME();
    fn QDC1_WATCHDOG();
    fn QDC1_INDEX();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 89] = [
    Vector {
        _handler: RESERVED16,
    },
    Vector { _handler: CMC },
    Vector { _handler: DMA0_CH0 },
    Vector { _handler: DMA0_CH1 },
    Vector { _handler: DMA0_CH2 },
    Vector { _handler: DMA0_CH3 },
    Vector { _handler: DMA0_CH4 },
    Vector { _handler: DMA0_CH5 },
    Vector { _handler: DMA0_CH6 },
    Vector { _handler: DMA0_CH7 },
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
    Vector { _handler: VBAT0 },
    Vector { _handler: WUU0 },
    Vector { _handler: CAN0 },
    Vector {
        _handler: RESERVED36,
    },
    Vector {
        _handler: RESERVED37,
    },
    Vector {
        _handler: RESERVED38,
    },
    Vector { _handler: FLEXIO },
    Vector { _handler: I3C0 },
    Vector {
        _handler: RESERVED41,
    },
    Vector { _handler: LPI2C0 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPSPI0 },
    Vector { _handler: LPSPI1 },
    Vector {
        _handler: RESERVED46,
    },
    Vector { _handler: LPUART0 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: USB0 },
    Vector {
        _handler: RESERVED53,
    },
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
        _handler: RESERVED65,
    },
    Vector {
        _handler: QDC0_COMPARE,
    },
    Vector {
        _handler: QDC0_HOME,
    },
    Vector {
        _handler: QDC0_WATCHDOG,
    },
    Vector {
        _handler: QDC0_INDEX,
    },
    Vector { _handler: FREQME0 },
    Vector { _handler: LPTMR0 },
    Vector {
        _handler: RESERVED72,
    },
    Vector { _handler: OS_EVENT },
    Vector {
        _handler: WAKETIMER0,
    },
    Vector { _handler: UTICK0 },
    Vector { _handler: WWDT0 },
    Vector {
        _handler: RESERVED77,
    },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: CMP0 },
    Vector { _handler: CMP1 },
    Vector {
        _handler: RESERVED82,
    },
    Vector { _handler: DAC0 },
    Vector {
        _handler: RESERVED84,
    },
    Vector {
        _handler: RESERVED85,
    },
    Vector {
        _handler: RESERVED86,
    },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector {
        _handler: RESERVED92,
    },
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
        _handler: RESERVED100,
    },
    Vector {
        _handler: QDC1_COMPARE,
    },
    Vector {
        _handler: QDC1_HOME,
    },
    Vector {
        _handler: QDC1_WATCHDOG,
    },
    Vector {
        _handler: QDC1_INDEX,
    },
];
