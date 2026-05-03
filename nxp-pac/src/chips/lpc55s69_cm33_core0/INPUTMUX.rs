#[doc = "Input multiplexing (INPUT MUX)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INPUTMUX {
    ptr: *mut u8,
}
unsafe impl Send for INPUTMUX {}
unsafe impl Sync for INPUTMUX {}
impl INPUTMUX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input mux register for SCT0 input."]
    #[inline(always)]
    pub const fn SCT0_INMUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SCT0_INMUX, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER0 inputs."]
    #[inline(always)]
    pub const fn TIMER0CAPTSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TIMER0CAPTSEL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER1 inputs."]
    #[inline(always)]
    pub const fn TIMER1CAPTSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TIMER1CAPTSEL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER2 inputs."]
    #[inline(always)]
    pub const fn TIMER2CAPTSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TIMER2CAPTSEL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Pin interrupt select register."]
    #[inline(always)]
    pub const fn PINTSEL(self, n: usize) -> crate::common::Reg<regs::PINTSEL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Trigger select register for DMA0 channel."]
    #[inline(always)]
    pub const fn DMA0_ITRIG_INMUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DMA0_ITRIG_INMUX, crate::common::RW> {
        assert!(n < 23usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[doc = "DMA0 output trigger selection to become DMA0 trigger."]
    #[inline(always)]
    pub const fn DMA0_OTRIG_INMUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DMA0_OTRIG_INMUX, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 4usize) as _)
        }
    }
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn FREQMEAS_REF(self) -> crate::common::Reg<regs::FREQMEAS_REF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement target clock."]
    #[inline(always)]
    pub const fn FREQMEAS_TARGET(
        self,
    ) -> crate::common::Reg<regs::FREQMEAS_TARGET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select registers for TIMER3 inputs."]
    #[inline(always)]
    pub const fn TIMER3CAPTSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TIMER3CAPTSEL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Capture select registers for TIMER4 inputs."]
    #[inline(always)]
    pub const fn TIMER4CAPTSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TIMER4CAPTSEL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Pin interrupt secure select register."]
    #[inline(always)]
    pub const fn PINTSECSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PINTSECSEL, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger select register for DMA1 channel."]
    #[inline(always)]
    pub const fn DMA1_ITRIG_INMUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DMA1_ITRIG_INMUX, crate::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "DMA1 output trigger selection to become DMA1 trigger."]
    #[inline(always)]
    pub const fn DMA1_OTRIG_INMUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DMA1_OTRIG_INMUX, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "Enable DMA0 requests."]
    #[inline(always)]
    pub const fn DMA0_REQ_ENA(self) -> crate::common::Reg<regs::DMA0_REQ_ENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "Set one or several bits in DMA0_REQ_ENA register."]
    #[inline(always)]
    pub const fn DMA0_REQ_ENA_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENA_SET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0748usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA0_REQ_ENA register."]
    #[inline(always)]
    pub const fn DMA0_REQ_ENA_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENA_CLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "Enable DMA1 requests."]
    #[inline(always)]
    pub const fn DMA1_REQ_ENA(self) -> crate::common::Reg<regs::DMA1_REQ_ENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "Set one or several bits in DMA1_REQ_ENA register."]
    #[inline(always)]
    pub const fn DMA1_REQ_ENA_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENA_SET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA1_REQ_ENA register."]
    #[inline(always)]
    pub const fn DMA1_REQ_ENA_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENA_CLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "Enable DMA0 triggers."]
    #[inline(always)]
    pub const fn DMA0_ITRIG_ENA(
        self,
    ) -> crate::common::Reg<regs::DMA0_ITRIG_ENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "Set one or several bits in DMA0_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn DMA0_ITRIG_ENA_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_ITRIG_ENA_SET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA0_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn DMA0_ITRIG_ENA_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_ITRIG_ENA_CLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "Enable DMA1 triggers."]
    #[inline(always)]
    pub const fn DMA1_ITRIG_ENA(
        self,
    ) -> crate::common::Reg<regs::DMA1_ITRIG_ENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "Set one or several bits in DMA1_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn DMA1_ITRIG_ENA_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_ITRIG_ENA_SET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA1_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn DMA1_ITRIG_ENA_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_ITRIG_ENA_CLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
