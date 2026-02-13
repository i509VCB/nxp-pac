#[doc = "Array of registers: CFG, CTLSTAT, XFERCFG"]
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
#[doc = "LPC-Next0 DMA controller"]
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
    #[doc = "Channel Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn enableset1(self) -> crate::common::Reg<regs::Enableset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn enableclr0(self) -> crate::common::Reg<regs::Enableclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn enableclr1(self) -> crate::common::Reg<regs::Enableclr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn active0(self) -> crate::common::Reg<regs::Active0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn active1(self) -> crate::common::Reg<regs::Active1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn busy0(self) -> crate::common::Reg<regs::Busy0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn busy1(self) -> crate::common::Reg<regs::Busy1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn errint0(self) -> crate::common::Reg<regs::Errint0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn errint1(self) -> crate::common::Reg<regs::Errint1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn intenset0(self) -> crate::common::Reg<regs::Intenset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn intenset1(self) -> crate::common::Reg<regs::Intenset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn intenclr0(self) -> crate::common::Reg<regs::Intenclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn intenclr1(self) -> crate::common::Reg<regs::Intenclr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn inta0(self) -> crate::common::Reg<regs::Inta0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn inta1(self) -> crate::common::Reg<regs::Inta1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn intb0(self) -> crate::common::Reg<regs::Intb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn intb1(self) -> crate::common::Reg<regs::Intb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn setvalid0(self) -> crate::common::Reg<regs::Setvalid0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn setvalid1(self) -> crate::common::Reg<regs::Setvalid1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn settrig0(self) -> crate::common::Reg<regs::Settrig0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn settrig1(self) -> crate::common::Reg<regs::Settrig1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn abort0(self) -> crate::common::Reg<regs::Abort0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn abort1(self) -> crate::common::Reg<regs::Abort1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Array of registers: CFG, CTLSTAT, XFERCFG"]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 33usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
