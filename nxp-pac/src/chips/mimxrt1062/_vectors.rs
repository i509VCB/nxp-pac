unsafe extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn CAN1();
    fn CAN2();
    fn FLEXRAM();
    fn KPP();
    fn TSC_DIG();
    fn GPR_IRQ();
    fn LCDIF();
    fn CSI();
    fn PXP();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn RESERVED68();
    fn TRNG();
    fn SJC();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU_EVENT();
    fn RESERVED78();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY1();
    fn USB_PHY2();
    fn ADC1();
    fn ADC2();
    fn DCDC();
    fn RESERVED86();
    fn GPIO10_COMBINED_0_31();
    fn GPIO1_INT0();
    fn GPIO1_INT1();
    fn GPIO1_INT2();
    fn GPIO1_INT3();
    fn GPIO1_INT4();
    fn GPIO1_INT5();
    fn GPIO1_INT6();
    fn GPIO1_INT7();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO2_COMBINED_16_31();
    fn GPIO3_COMBINED_0_15();
    fn GPIO3_COMBINED_16_31();
    fn GPIO4_COMBINED_0_15();
    fn GPIO4_COMBINED_16_31();
    fn GPIO5_COMBINED_0_15();
    fn GPIO5_COMBINED_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG();
    fn EWM();
    fn CCM_1();
    fn CCM_2();
    fn GPC();
    fn SRC();
    fn RESERVED115();
    fn GPT1();
    fn GPT2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn FLEXSPI2();
    fn FLEXSPI();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_TIMER();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn RESERVED143();
    fn RESERVED144();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
    fn ENET2();
    fn ENET2_1588_TIMER();
    fn CAN3();
    fn RESERVED171();
    fn FLEXIO3();
    fn GPIO6_7_8_9();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 158] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
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
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector { _handler: TSC_DIG },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: LCDIF },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
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
    Vector { _handler: SJC },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: PMU_EVENT,
    },
    Vector {
        _handler: RESERVED78,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY1 },
    Vector { _handler: USB_PHY2 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector {
        _handler: RESERVED86,
    },
    Vector {
        _handler: GPIO10_COMBINED_0_31,
    },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT7,
    },
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
        _handler: GPIO2_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO3_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO3_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO4_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO4_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: EWM },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: GPC },
    Vector { _handler: SRC },
    Vector {
        _handler: RESERVED115,
    },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: FLEXSPI2 },
    Vector { _handler: FLEXSPI },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_TIMER,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
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
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _handler: PIT },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector {
        _handler: RESERVED143,
    },
    Vector {
        _handler: RESERVED144,
    },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector { _handler: ENET2 },
    Vector {
        _handler: ENET2_1588_TIMER,
    },
    Vector { _handler: CAN3 },
    Vector {
        _handler: RESERVED171,
    },
    Vector { _handler: FLEXIO3 },
    Vector {
        _handler: GPIO6_7_8_9,
    },
];
