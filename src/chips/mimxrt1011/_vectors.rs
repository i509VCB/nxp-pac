unsafe extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn DMA4();
    fn DMA5();
    fn DMA6();
    fn DMA7();
    fn DMA8();
    fn DMA9();
    fn DMA10();
    fn DMA11();
    fn DMA12();
    fn DMA13();
    fn DMA14();
    fn DMA15();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn PIT();
    fn USB_OTG1();
    fn FLEXSPI();
    fn FLEXRAM();
    fn LPI2C1();
    fn LPI2C2();
    fn GPT1();
    fn GPT2();
    fn LPSPI1();
    fn LPSPI2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn KPP();
    fn SRC();
    fn GPR_IRQ();
    fn CCM_1();
    fn CCM_2();
    fn EWM();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn RESERVED68();
    fn TRNG();
    fn RESERVED70();
    fn RESERVED71();
    fn SAI1();
    fn RTWDOG();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU();
    fn XBAR1_IRQ_0_1_2_3();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY();
    fn GPC();
    fn ADC1();
    fn FLEXIO1();
    fn DCDC();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO5_COMBINED_0_15();
    fn WDOG1();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_IRQ3();
    fn ADC_ETC_ERROR_IRQ();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 80] = [
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _handler: DMA4 },
    Vector { _handler: DMA5 },
    Vector { _handler: DMA6 },
    Vector { _handler: DMA7 },
    Vector { _handler: DMA8 },
    Vector { _handler: DMA9 },
    Vector { _handler: DMA10 },
    Vector { _handler: DMA11 },
    Vector { _handler: DMA12 },
    Vector { _handler: DMA13 },
    Vector { _handler: DMA14 },
    Vector { _handler: DMA15 },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: CTI0_ERROR,
    },
    Vector {
        _handler: CTI1_ERROR,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: PIT },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: FLEXSPI },
    Vector { _handler: FLEXRAM },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: KPP },
    Vector { _handler: SRC },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: EWM },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_WRAPPER,
    },
    Vector {
        _handler: SNVS_HP_WRAPPER_TZ,
    },
    Vector {
        _handler: SNVS_LP_WRAPPER,
    },
    Vector { _handler: CSU },
    Vector { _handler: DCP },
    Vector { _handler: DCP_VMI },
    Vector {
        _handler: RESERVED68,
    },
    Vector { _handler: TRNG },
    Vector {
        _handler: RESERVED70,
    },
    Vector {
        _handler: RESERVED71,
    },
    Vector { _handler: SAI1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector { _handler: PMU },
    Vector {
        _handler: XBAR1_IRQ_0_1_2_3,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY },
    Vector { _handler: GPC },
    Vector { _handler: ADC1 },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: DCDC },
    Vector {
        _handler: GPIO1_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO1_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO2_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector { _handler: WDOG1 },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_IRQ3,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
];
