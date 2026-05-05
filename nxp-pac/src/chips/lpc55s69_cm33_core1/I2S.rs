#[doc = "I2S interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2S {
    ptr: *mut u8,
}
unsafe impl Send for I2S {}
unsafe impl Sync for I2S {}
impl I2S {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub const fn CFG1(self) -> crate::common::Reg<regs::CFG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub const fn CFG2(self) -> crate::common::Reg<regs::CFG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "Status register for the primary channel pair."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub const fn DIV(self) -> crate::common::Reg<regs::DIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c1cusize) as _) }
    }
    #[doc = "FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn FIFOCFG(self) -> crate::common::Reg<regs::FIFOCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "FIFO status register."]
    #[inline(always)]
    pub const fn FIFOSTAT(self) -> crate::common::Reg<regs::FIFOSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn FIFOTRIG(self) -> crate::common::Reg<regs::FIFOTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn FIFOINTENSET(self) -> crate::common::Reg<regs::FIFOINTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn FIFOINTENCLR(self) -> crate::common::Reg<regs::FIFOINTENCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e14usize) as _) }
    }
    #[doc = "FIFO interrupt status register."]
    #[inline(always)]
    pub const fn FIFOINTSTAT(self) -> crate::common::Reg<regs::FIFOINTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e18usize) as _) }
    }
    #[doc = "FIFO write data."]
    #[inline(always)]
    pub const fn FIFOWR(self) -> crate::common::Reg<regs::FIFOWR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize) as _) }
    }
    #[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn FIFOWR48H(self) -> crate::common::Reg<regs::FIFOWR48H, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e24usize) as _) }
    }
    #[doc = "FIFO read data."]
    #[inline(always)]
    pub const fn FIFORD(self) -> crate::common::Reg<regs::FIFORD, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e30usize) as _) }
    }
    #[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn FIFORD48H(self) -> crate::common::Reg<regs::FIFORD48H, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e34usize) as _) }
    }
    #[doc = "FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn FIFORDNOPOP(self) -> crate::common::Reg<regs::FIFORDNOPOP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn FIFORD48HNOPOP(
        self,
    ) -> crate::common::Reg<regs::FIFORD48HNOPOP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e44usize) as _) }
    }
    #[doc = "FIFO size register."]
    #[inline(always)]
    pub const fn FIFOSIZE(self) -> crate::common::Reg<regs::FIFOSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e48usize) as _) }
    }
    #[doc = "I2S Module identification."]
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
