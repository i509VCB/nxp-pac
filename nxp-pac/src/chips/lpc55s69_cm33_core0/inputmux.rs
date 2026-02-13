#[doc = "Input multiplexing (INPUT MUX)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux {}
unsafe impl Sync for Inputmux {}
impl Inputmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input mux register for SCT0 input"]
    #[inline(always)]
    pub const fn sct0_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sct0Inmux, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER0 inputs"]
    #[inline(always)]
    pub const fn timer0captsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Timer0captsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER1 inputs"]
    #[inline(always)]
    pub const fn timer1captsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Timer1captsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Capture select registers for TIMER2 inputs"]
    #[inline(always)]
    pub const fn timer2captsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Timer2captsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Pin interrupt select register"]
    #[inline(always)]
    pub const fn pintsel(self, n: usize) -> crate::common::Reg<regs::Pintsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Trigger select register for DMA0 channel"]
    #[inline(always)]
    pub const fn dma0_itrig_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dma0ItrigInmux, crate::common::RW> {
        assert!(n < 23usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[doc = "DMA0 output trigger selection to become DMA0 trigger"]
    #[inline(always)]
    pub const fn dma0_otrig_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dma0OtrigInmux, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 4usize) as _)
        }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_ref(self) -> crate::common::Reg<regs::FreqmeasRef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement target clock"]
    #[inline(always)]
    pub const fn freqmeas_target(
        self,
    ) -> crate::common::Reg<regs::FreqmeasTarget, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select registers for TIMER3 inputs"]
    #[inline(always)]
    pub const fn timer3captsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Timer3captsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Capture select registers for TIMER4 inputs"]
    #[inline(always)]
    pub const fn timer4captsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Timer4captsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Pin interrupt secure select register"]
    #[inline(always)]
    pub const fn pintsecsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Pintsecsel, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger select register for DMA1 channel"]
    #[inline(always)]
    pub const fn dma1_itrig_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dma1ItrigInmux, crate::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "DMA1 output trigger selection to become DMA1 trigger"]
    #[inline(always)]
    pub const fn dma1_otrig_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dma1OtrigInmux, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "Enable DMA0 requests"]
    #[inline(always)]
    pub const fn dma0_req_ena(self) -> crate::common::Reg<regs::Dma0ReqEna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "Set one or several bits in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma0_req_ena_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0748usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma0_req_ena_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "Enable DMA1 requests"]
    #[inline(always)]
    pub const fn dma1_req_ena(self) -> crate::common::Reg<regs::Dma1ReqEna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "Set one or several bits in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma1_req_ena_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn dma1_req_ena_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "Enable DMA0 triggers"]
    #[inline(always)]
    pub const fn dma0_itrig_ena(self) -> crate::common::Reg<regs::Dma0ItrigEna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma0_itrig_ena_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ItrigEnaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma0_itrig_ena_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ItrigEnaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "Enable DMA1 triggers"]
    #[inline(always)]
    pub const fn dma1_itrig_ena(self) -> crate::common::Reg<regs::Dma1ItrigEna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma1_itrig_ena_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ItrigEnaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn dma1_itrig_ena_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ItrigEnaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
