#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register for DMA channel ."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control and status register for DMA channel ."]
    #[inline(always)]
    pub const fn ctlstat(self) -> crate::common::Reg<regs::Ctlstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Transfer configuration register for DMA channel ."]
    #[inline(always)]
    pub const fn xfercfg(self) -> crate::common::Reg<regs::Xfercfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "DMA controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SRAM address of the channel configuration table."]
    #[inline(always)]
    pub const fn srambase(self) -> crate::common::Reg<regs::Srambase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn enableset0(self) -> crate::common::Reg<regs::Enableset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn enableclr0(self) -> crate::common::Reg<regs::Enableclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn active0(self) -> crate::common::Reg<regs::Active0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn busy0(self) -> crate::common::Reg<regs::Busy0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn errint0(self) -> crate::common::Reg<regs::Errint0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn intenset0(self) -> crate::common::Reg<regs::Intenset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn intenclr0(self) -> crate::common::Reg<regs::Intenclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn inta0(self) -> crate::common::Reg<regs::Inta0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn intb0(self) -> crate::common::Reg<regs::Intb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn setvalid0(self) -> crate::common::Reg<regs::Setvalid0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn settrig0(self) -> crate::common::Reg<regs::Settrig0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn abort0(self) -> crate::common::Reg<regs::Abort0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 23usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
