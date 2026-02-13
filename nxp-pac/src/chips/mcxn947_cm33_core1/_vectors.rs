unsafe extern "C" {
    fn OR();
    fn EDMA_0_CH0();
    fn EDMA_0_CH1();
    fn EDMA_0_CH2();
    fn EDMA_0_CH3();
    fn EDMA_0_CH4();
    fn EDMA_0_CH5();
    fn EDMA_0_CH6();
    fn EDMA_0_CH7();
    fn EDMA_0_CH8();
    fn EDMA_0_CH9();
    fn EDMA_0_CH10();
    fn EDMA_0_CH11();
    fn EDMA_0_CH12();
    fn EDMA_0_CH13();
    fn EDMA_0_CH14();
    fn EDMA_0_CH15();
    fn GPIO00();
    fn GPIO01();
    fn GPIO10();
    fn GPIO11();
    fn GPIO20();
    fn GPIO21();
    fn GPIO30();
    fn GPIO31();
    fn GPIO40();
    fn GPIO41();
    fn GPIO50();
    fn GPIO51();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn SCT0();
    fn CTIMER2();
    fn LP_FLEXCOMM0();
    fn LP_FLEXCOMM1();
    fn LP_FLEXCOMM2();
    fn LP_FLEXCOMM3();
    fn LP_FLEXCOMM4();
    fn LP_FLEXCOMM5();
    fn LP_FLEXCOMM6();
    fn LP_FLEXCOMM7();
    fn LP_FLEXCOMM8();
    fn LP_FLEXCOMM9();
    fn ADC0();
    fn ADC1();
    fn PINT0();
    fn PDM_EVENT();
    fn RESERVED65();
    fn USB0_FS();
    fn USB0_DCD();
    fn RTC();
    fn SMARTDMA();
    fn MAILBOX();
    fn CTIMER3();
    fn CTIMER4();
    fn OS_EVENT();
    fn FLEXSPI0();
    fn SAI0();
    fn SAI1();
    fn USDHC0();
    fn CAN0();
    fn CAN1();
    fn RESERVED80();
    fn RESERVED81();
    fn USB1_HS_PHY();
    fn USB1_HS();
    fn SEC_HYPERVISOR_CALL();
    fn RESERVED85();
    fn PLU();
    fn FREQME();
    fn SEC_VIO();
    fn ELS();
    fn PKC();
    fn PUF();
    fn PQ();
    fn EDMA_1_CH0();
    fn EDMA_1_CH1();
    fn EDMA_1_CH2();
    fn EDMA_1_CH3();
    fn EDMA_1_CH4();
    fn EDMA_1_CH5();
    fn EDMA_1_CH6();
    fn EDMA_1_CH7();
    fn EDMA_1_CH8();
    fn EDMA_1_CH9();
    fn EDMA_1_CH10();
    fn EDMA_1_CH11();
    fn EDMA_1_CH12();
    fn EDMA_1_CH13();
    fn EDMA_1_CH14();
    fn EDMA_1_CH15();
    fn CDOG0();
    fn CDOG1();
    fn I3C0();
    fn I3C1();
    fn NPU();
    fn GDET();
    fn VBAT0();
    fn EWM0();
    fn TSI_END_OF_SCAN();
    fn TSI_OUT_OF_SCAN();
    fn EMVSIM0();
    fn EMVSIM1();
    fn FLEXIO();
    fn DAC0();
    fn DAC1();
    fn DAC2();
    fn HSCMP0();
    fn HSCMP1();
    fn HSCMP2();
    fn FLEXPWM0_RELOAD_ERROR();
    fn FLEXPWM0_FAULT();
    fn FLEXPWM0_SUBMODULE0();
    fn FLEXPWM0_SUBMODULE1();
    fn FLEXPWM0_SUBMODULE2();
    fn FLEXPWM0_SUBMODULE3();
    fn FLEXPWM1_RELOAD_ERROR();
    fn FLEXPWM1_FAULT();
    fn FLEXPWM1_SUBMODULE0();
    fn FLEXPWM1_SUBMODULE1();
    fn FLEXPWM1_SUBMODULE2();
    fn FLEXPWM1_SUBMODULE3();
    fn QDC0_COMPARE();
    fn QDC0_HOME();
    fn QDC0_WDG_SAB();
    fn QDC0_IDX();
    fn QDC1_COMPARE();
    fn QDC1_HOME();
    fn QDC1_WDG_SAB();
    fn QDC1_IDX();
    fn ITRC0();
    fn BSP32();
    fn ELS_ERR();
    fn PKC_ERR();
    fn ERM_SINGLE_BIT_ERROR();
    fn ERM_MULTI_BIT_ERROR();
    fn FMU0();
    fn ETHERNET();
    fn ETHERNET_PMT();
    fn ETHERNET_MACLP();
    fn SINC_FILTER();
    fn LPTMR0();
    fn LPTMR1();
    fn SCG();
    fn SPC();
    fn WUU();
    fn PORT_EFT();
    fn ETB0();
    fn RESERVED166();
    fn RESERVED167();
    fn WWDT0();
    fn WWDT1();
    fn CMC0();
    fn CTI0();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 156] = [
    Vector { _handler: OR },
    Vector {
        _handler: EDMA_0_CH0,
    },
    Vector {
        _handler: EDMA_0_CH1,
    },
    Vector {
        _handler: EDMA_0_CH2,
    },
    Vector {
        _handler: EDMA_0_CH3,
    },
    Vector {
        _handler: EDMA_0_CH4,
    },
    Vector {
        _handler: EDMA_0_CH5,
    },
    Vector {
        _handler: EDMA_0_CH6,
    },
    Vector {
        _handler: EDMA_0_CH7,
    },
    Vector {
        _handler: EDMA_0_CH8,
    },
    Vector {
        _handler: EDMA_0_CH9,
    },
    Vector {
        _handler: EDMA_0_CH10,
    },
    Vector {
        _handler: EDMA_0_CH11,
    },
    Vector {
        _handler: EDMA_0_CH12,
    },
    Vector {
        _handler: EDMA_0_CH13,
    },
    Vector {
        _handler: EDMA_0_CH14,
    },
    Vector {
        _handler: EDMA_0_CH15,
    },
    Vector { _handler: GPIO00 },
    Vector { _handler: GPIO01 },
    Vector { _handler: GPIO10 },
    Vector { _handler: GPIO11 },
    Vector { _handler: GPIO20 },
    Vector { _handler: GPIO21 },
    Vector { _handler: GPIO30 },
    Vector { _handler: GPIO31 },
    Vector { _handler: GPIO40 },
    Vector { _handler: GPIO41 },
    Vector { _handler: GPIO50 },
    Vector { _handler: GPIO51 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: SCT0 },
    Vector { _handler: CTIMER2 },
    Vector {
        _handler: LP_FLEXCOMM0,
    },
    Vector {
        _handler: LP_FLEXCOMM1,
    },
    Vector {
        _handler: LP_FLEXCOMM2,
    },
    Vector {
        _handler: LP_FLEXCOMM3,
    },
    Vector {
        _handler: LP_FLEXCOMM4,
    },
    Vector {
        _handler: LP_FLEXCOMM5,
    },
    Vector {
        _handler: LP_FLEXCOMM6,
    },
    Vector {
        _handler: LP_FLEXCOMM7,
    },
    Vector {
        _handler: LP_FLEXCOMM8,
    },
    Vector {
        _handler: LP_FLEXCOMM9,
    },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: PINT0 },
    Vector {
        _handler: PDM_EVENT,
    },
    Vector {
        _handler: RESERVED65,
    },
    Vector { _handler: USB0_FS },
    Vector { _handler: USB0_DCD },
    Vector { _handler: RTC },
    Vector { _handler: SMARTDMA },
    Vector { _handler: MAILBOX },
    Vector { _handler: CTIMER3 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: OS_EVENT },
    Vector { _handler: FLEXSPI0 },
    Vector { _handler: SAI0 },
    Vector { _handler: SAI1 },
    Vector { _handler: USDHC0 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector {
        _handler: RESERVED80,
    },
    Vector {
        _handler: RESERVED81,
    },
    Vector {
        _handler: USB1_HS_PHY,
    },
    Vector { _handler: USB1_HS },
    Vector {
        _handler: SEC_HYPERVISOR_CALL,
    },
    Vector {
        _handler: RESERVED85,
    },
    Vector { _handler: PLU },
    Vector { _handler: FREQME },
    Vector { _handler: SEC_VIO },
    Vector { _handler: ELS },
    Vector { _handler: PKC },
    Vector { _handler: PUF },
    Vector { _handler: PQ },
    Vector {
        _handler: EDMA_1_CH0,
    },
    Vector {
        _handler: EDMA_1_CH1,
    },
    Vector {
        _handler: EDMA_1_CH2,
    },
    Vector {
        _handler: EDMA_1_CH3,
    },
    Vector {
        _handler: EDMA_1_CH4,
    },
    Vector {
        _handler: EDMA_1_CH5,
    },
    Vector {
        _handler: EDMA_1_CH6,
    },
    Vector {
        _handler: EDMA_1_CH7,
    },
    Vector {
        _handler: EDMA_1_CH8,
    },
    Vector {
        _handler: EDMA_1_CH9,
    },
    Vector {
        _handler: EDMA_1_CH10,
    },
    Vector {
        _handler: EDMA_1_CH11,
    },
    Vector {
        _handler: EDMA_1_CH12,
    },
    Vector {
        _handler: EDMA_1_CH13,
    },
    Vector {
        _handler: EDMA_1_CH14,
    },
    Vector {
        _handler: EDMA_1_CH15,
    },
    Vector { _handler: CDOG0 },
    Vector { _handler: CDOG1 },
    Vector { _handler: I3C0 },
    Vector { _handler: I3C1 },
    Vector { _handler: NPU },
    Vector { _handler: GDET },
    Vector { _handler: VBAT0 },
    Vector { _handler: EWM0 },
    Vector {
        _handler: TSI_END_OF_SCAN,
    },
    Vector {
        _handler: TSI_OUT_OF_SCAN,
    },
    Vector { _handler: EMVSIM0 },
    Vector { _handler: EMVSIM1 },
    Vector { _handler: FLEXIO },
    Vector { _handler: DAC0 },
    Vector { _handler: DAC1 },
    Vector { _handler: DAC2 },
    Vector { _handler: HSCMP0 },
    Vector { _handler: HSCMP1 },
    Vector { _handler: HSCMP2 },
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
        _handler: QDC0_COMPARE,
    },
    Vector {
        _handler: QDC0_HOME,
    },
    Vector {
        _handler: QDC0_WDG_SAB,
    },
    Vector { _handler: QDC0_IDX },
    Vector {
        _handler: QDC1_COMPARE,
    },
    Vector {
        _handler: QDC1_HOME,
    },
    Vector {
        _handler: QDC1_WDG_SAB,
    },
    Vector { _handler: QDC1_IDX },
    Vector { _handler: ITRC0 },
    Vector { _handler: BSP32 },
    Vector { _handler: ELS_ERR },
    Vector { _handler: PKC_ERR },
    Vector {
        _handler: ERM_SINGLE_BIT_ERROR,
    },
    Vector {
        _handler: ERM_MULTI_BIT_ERROR,
    },
    Vector { _handler: FMU0 },
    Vector { _handler: ETHERNET },
    Vector {
        _handler: ETHERNET_PMT,
    },
    Vector {
        _handler: ETHERNET_MACLP,
    },
    Vector {
        _handler: SINC_FILTER,
    },
    Vector { _handler: LPTMR0 },
    Vector { _handler: LPTMR1 },
    Vector { _handler: SCG },
    Vector { _handler: SPC },
    Vector { _handler: WUU },
    Vector { _handler: PORT_EFT },
    Vector { _handler: ETB0 },
    Vector {
        _handler: RESERVED166,
    },
    Vector {
        _handler: RESERVED167,
    },
    Vector { _handler: WWDT0 },
    Vector { _handler: WWDT1 },
    Vector { _handler: CMC0 },
    Vector { _handler: CTI0 },
];
