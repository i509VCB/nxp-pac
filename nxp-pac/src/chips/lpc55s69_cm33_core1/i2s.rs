#[doc = "I2S interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s {
    ptr: *mut u8,
}
unsafe impl Send for I2s {}
unsafe impl Sync for I2s {}
impl I2s {
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
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "Status register for the primary channel pair."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c1cusize) as _) }
    }
    #[doc = "FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn fifocfg(self) -> crate::common::Reg<regs::Fifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "FIFO status register."]
    #[inline(always)]
    pub const fn fifostat(self) -> crate::common::Reg<regs::Fifostat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn fifotrig(self) -> crate::common::Reg<regs::Fifotrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn fifointenset(self) -> crate::common::Reg<regs::Fifointenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn fifointenclr(self) -> crate::common::Reg<regs::Fifointenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e14usize) as _) }
    }
    #[doc = "FIFO interrupt status register."]
    #[inline(always)]
    pub const fn fifointstat(self) -> crate::common::Reg<regs::Fifointstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e18usize) as _) }
    }
    #[doc = "FIFO write data."]
    #[inline(always)]
    pub const fn fifowr(self) -> crate::common::Reg<regs::Fifowr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize) as _) }
    }
    #[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fifowr48h(self) -> crate::common::Reg<regs::Fifowr48h, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e24usize) as _) }
    }
    #[doc = "FIFO read data."]
    #[inline(always)]
    pub const fn fiford(self) -> crate::common::Reg<regs::Fiford, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e30usize) as _) }
    }
    #[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fiford48h(self) -> crate::common::Reg<regs::Fiford48h, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e34usize) as _) }
    }
    #[doc = "FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn fifordnopop(self) -> crate::common::Reg<regs::Fifordnopop, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fiford48hnopop(
        self,
    ) -> crate::common::Reg<regs::Fiford48hnopop, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e44usize) as _) }
    }
    #[doc = "FIFO size register"]
    #[inline(always)]
    pub const fn fifosize(self) -> crate::common::Reg<regs::Fifosize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e48usize) as _) }
    }
    #[doc = "I2S Module identification"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
