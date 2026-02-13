unsafe extern "C" {
    fn WDT_BOD();
    fn DMA0();
    fn GINT0();
    fn GINT1();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn SCT0();
    fn CTIMER3();
    fn FLEXCOMM0();
    fn FLEXCOMM1();
    fn FLEXCOMM2();
    fn FLEXCOMM3();
    fn FLEXCOMM4();
    fn FLEXCOMM5();
    fn FLEXCOMM6();
    fn FLEXCOMM7();
    fn ADC0();
    fn ACMP();
    fn USB0_NEEDCLK();
    fn USB0();
    fn RTC();
    fn MAILBOX();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn CTIMER2();
    fn CTIMER4();
    fn OS_EVENT();
    fn SDIO();
    fn USB1_PHY();
    fn USB1();
    fn USB1_NEEDCLK();
    fn SEC_HYPERVISOR_CALL();
    fn SEC_GPIO_INT0_IRQ0();
    fn SEC_GPIO_INT0_IRQ1();
    fn PLU();
    fn SEC_VIO();
    fn HASHCRYPT();
    fn CASER();
    fn PUF();
    fn PQ();
    fn DMA1();
    fn FLEXCOMM8();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: WDT_BOD },
    Vector { _handler: DMA0 },
    Vector { _handler: GINT0 },
    Vector { _handler: GINT1 },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: SCT0 },
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
        _handler: FLEXCOMM6,
    },
    Vector {
        _handler: FLEXCOMM7,
    },
    Vector { _handler: ADC0 },
    Vector { _reserved: 0 },
    Vector { _handler: ACMP },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB0_NEEDCLK,
    },
    Vector { _handler: USB0 },
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _handler: MAILBOX },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: OS_EVENT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDIO },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USB1_PHY },
    Vector { _handler: USB1 },
    Vector {
        _handler: USB1_NEEDCLK,
    },
    Vector {
        _handler: SEC_HYPERVISOR_CALL,
    },
    Vector {
        _handler: SEC_GPIO_INT0_IRQ0,
    },
    Vector {
        _handler: SEC_GPIO_INT0_IRQ1,
    },
    Vector { _handler: PLU },
    Vector { _handler: SEC_VIO },
    Vector {
        _handler: HASHCRYPT,
    },
    Vector { _handler: CASER },
    Vector { _handler: PUF },
    Vector { _handler: PQ },
    Vector { _handler: DMA1 },
    Vector {
        _handler: FLEXCOMM8,
    },
];
