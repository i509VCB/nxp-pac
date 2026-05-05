#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHANNEL {
    ptr: *mut u8,
}
unsafe impl Send for CHANNEL {}
unsafe impl Sync for CHANNEL {}
impl CHANNEL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register for DMA channel."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control and status register for DMA channel."]
    #[inline(always)]
    pub const fn CTLSTAT(self) -> crate::common::Reg<regs::CTLSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Transfer configuration register for DMA channel."]
    #[inline(always)]
    pub const fn XFERCFG(self) -> crate::common::Reg<regs::XFERCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "DMA controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA {
    ptr: *mut u8,
}
unsafe impl Send for DMA {}
unsafe impl Sync for DMA {}
impl DMA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA control."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt status."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SRAM address of the channel configuration table."]
    #[inline(always)]
    pub const fn SRAMBASE(self) -> crate::common::Reg<regs::SRAMBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn ENABLESET0(self) -> crate::common::Reg<regs::ENABLESET0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn ENABLECLR0(self) -> crate::common::Reg<regs::ENABLECLR0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn ACTIVE0(self) -> crate::common::Reg<regs::ACTIVE0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn BUSY0(self) -> crate::common::Reg<regs::BUSY0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn ERRINT0(self) -> crate::common::Reg<regs::ERRINT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn INTENSET0(self) -> crate::common::Reg<regs::INTENSET0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn INTENCLR0(self) -> crate::common::Reg<regs::INTENCLR0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn INTA0(self) -> crate::common::Reg<regs::INTA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn INTB0(self) -> crate::common::Reg<regs::INTB0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn SETVALID0(self) -> crate::common::Reg<regs::SETVALID0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn SETTRIG0(self) -> crate::common::Reg<regs::SETTRIG0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn ABORT0(self) -> crate::common::Reg<regs::ABORT0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn CHANNEL(self, n: usize) -> CHANNEL {
        assert!(n < 23usize);
        unsafe { CHANNEL::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
