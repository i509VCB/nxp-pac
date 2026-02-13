unsafe extern "C" {
    fn WDT0();
    fn DMA0();
    fn GPIO_INTA();
    fn GPIO_INTB();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn SCT();
    fn CTIMER3();
    fn FLEXCOMM0();
    fn FLEXCOMM1();
    fn FLEXCOMM2();
    fn FLEXCOMM3();
    fn FLEXCOMM4();
    fn FLEXCOMM5();
    fn FLEXCOMM14();
    fn FLEXCOMM15();
    fn ADC();
    fn RESERVED39();
    fn ACMP();
    fn DMIC();
    fn RESERVED42();
    fn HYPERVISOR();
    fn SECUREVIOLATION();
    fn HWVAD0();
    fn ESPI();
    fn RNG();
    fn RTC();
    fn DSPWAKE();
    fn MU_A();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn CTIMER2();
    fn CTIMER4();
    fn OS_EVENT();
    fn FLEXSPI();
    fn FLEXCOMM6();
    fn FLEXCOMM7();
    fn USDHC0();
    fn USDHC1();
    fn SGPIO_INTA();
    fn SGPIO_INTB();
    fn I3C();
    fn USB();
    fn USB_WAKEUP();
    fn WDT1();
    fn USBPHY_DCD();
    fn DMA1();
    fn PUF();
    fn POWERQUAD();
    fn CASPER();
    fn PMC_PMIC();
    fn HASHCRYPT();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: WDT0 },
    Vector { _handler: DMA0 },
    Vector {
        _handler: GPIO_INTA,
    },
    Vector {
        _handler: GPIO_INTB,
    },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: SCT },
    Vector { _handler: CTIMER3 },
    Vector {
        _handler: FLEXCOMM0,
    },
    Vector {
        _handler: FLEXCOMM1,
    },
    Vector {
        _handler: FLEXCOMM2,
    },
    Vector {
        _handler: FLEXCOMM3,
    },
    Vector {
        _handler: FLEXCOMM4,
    },
    Vector {
        _handler: FLEXCOMM5,
    },
    Vector {
        _handler: FLEXCOMM14,
    },
    Vector {
        _handler: FLEXCOMM15,
    },
    Vector { _handler: ADC },
    Vector {
        _handler: RESERVED39,
    },
    Vector { _handler: ACMP },
    Vector { _handler: DMIC },
    Vector {
        _handler: RESERVED42,
    },
    Vector {
        _handler: HYPERVISOR,
    },
    Vector {
        _handler: SECUREVIOLATION,
    },
    Vector { _handler: HWVAD0 },
    Vector { _handler: ESPI },
    Vector { _handler: RNG },
    Vector { _handler: RTC },
    Vector { _handler: DSPWAKE },
    Vector { _handler: MU_A },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: OS_EVENT },
    Vector { _handler: FLEXSPI },
    Vector {
        _handler: FLEXCOMM6,
    },
    Vector {
        _handler: FLEXCOMM7,
    },
    Vector { _handler: USDHC0 },
    Vector { _handler: USDHC1 },
    Vector {
        _handler: SGPIO_INTA,
    },
    Vector {
        _handler: SGPIO_INTB,
    },
    Vector { _handler: I3C },
    Vector { _handler: USB },
    Vector {
        _handler: USB_WAKEUP,
    },
    Vector { _handler: WDT1 },
    Vector {
        _handler: USBPHY_DCD,
    },
    Vector { _handler: DMA1 },
    Vector { _handler: PUF },
    Vector {
        _handler: POWERQUAD,
    },
    Vector { _handler: CASPER },
    Vector { _handler: PMC_PMIC },
    Vector {
        _handler: HASHCRYPT,
    },
];
