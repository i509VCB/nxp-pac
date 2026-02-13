#[doc = "DCP register reference index"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcp {
    ptr: *mut u8,
}
unsafe impl Send for Dcp {}
unsafe impl Sync for Dcp {}
impl Dcp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_set(self) -> crate::common::Reg<regs::StatSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_clr(self) -> crate::common::Reg<regs::StatClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_tog(self) -> crate::common::Reg<regs::StatTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl(self) -> crate::common::Reg<regs::Channelctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_set(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_clr(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_tog(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "DCP capability 0 register"]
    #[inline(always)]
    pub const fn capability0(self) -> crate::common::Reg<regs::Capability0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DCP capability 1 register"]
    #[inline(always)]
    pub const fn capability1(self) -> crate::common::Reg<regs::Capability1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "DCP context buffer pointer"]
    #[inline(always)]
    pub const fn context(self) -> crate::common::Reg<regs::Context, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "DCP key index"]
    #[inline(always)]
    pub const fn key(self) -> crate::common::Reg<regs::Key, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DCP key data"]
    #[inline(always)]
    pub const fn keydata(self) -> crate::common::Reg<regs::Keydata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "DCP work packet 0 status register"]
    #[inline(always)]
    pub const fn packet0(self) -> crate::common::Reg<regs::Packet0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "DCP work packet 1 status register"]
    #[inline(always)]
    pub const fn packet1(self) -> crate::common::Reg<regs::Packet1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "DCP work packet 2 status register"]
    #[inline(always)]
    pub const fn packet2(self) -> crate::common::Reg<regs::Packet2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "DCP work packet 3 status register"]
    #[inline(always)]
    pub const fn packet3(self) -> crate::common::Reg<regs::Packet3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "DCP work packet 4 status register"]
    #[inline(always)]
    pub const fn packet4(self) -> crate::common::Reg<regs::Packet4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "DCP work packet 5 status register"]
    #[inline(always)]
    pub const fn packet5(self) -> crate::common::Reg<regs::Packet5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "DCP work packet 6 status register"]
    #[inline(always)]
    pub const fn packet6(self) -> crate::common::Reg<regs::Packet6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "DCP channel 0 command pointer address register"]
    #[inline(always)]
    pub const fn ch0cmdptr(self) -> crate::common::Reg<regs::Ch0cmdptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "DCP channel 0 semaphore register"]
    #[inline(always)]
    pub const fn ch0sema(self) -> crate::common::Reg<regs::Ch0sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat(self) -> crate::common::Reg<regs::Ch0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_set(self) -> crate::common::Reg<regs::Ch0statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_clr(self) -> crate::common::Reg<regs::Ch0statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_tog(self) -> crate::common::Reg<regs::Ch0statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts(self) -> crate::common::Reg<regs::Ch0opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_set(self) -> crate::common::Reg<regs::Ch0optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_clr(self) -> crate::common::Reg<regs::Ch0optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_tog(self) -> crate::common::Reg<regs::Ch0optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "DCP channel 1 command pointer address register"]
    #[inline(always)]
    pub const fn ch1cmdptr(self) -> crate::common::Reg<regs::Ch1cmdptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "DCP channel 1 semaphore register"]
    #[inline(always)]
    pub const fn ch1sema(self) -> crate::common::Reg<regs::Ch1sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat(self) -> crate::common::Reg<regs::Ch1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_set(self) -> crate::common::Reg<regs::Ch1statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_clr(self) -> crate::common::Reg<regs::Ch1statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_tog(self) -> crate::common::Reg<regs::Ch1statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts(self) -> crate::common::Reg<regs::Ch1opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_set(self) -> crate::common::Reg<regs::Ch1optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_clr(self) -> crate::common::Reg<regs::Ch1optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_tog(self) -> crate::common::Reg<regs::Ch1optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "DCP channel 2 command pointer address register"]
    #[inline(always)]
    pub const fn ch2cmdptr(self) -> crate::common::Reg<regs::Ch2cmdptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "DCP channel 2 semaphore register"]
    #[inline(always)]
    pub const fn ch2sema(self) -> crate::common::Reg<regs::Ch2sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat(self) -> crate::common::Reg<regs::Ch2stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_set(self) -> crate::common::Reg<regs::Ch2statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_clr(self) -> crate::common::Reg<regs::Ch2statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_tog(self) -> crate::common::Reg<regs::Ch2statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts(self) -> crate::common::Reg<regs::Ch2opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_set(self) -> crate::common::Reg<regs::Ch2optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_clr(self) -> crate::common::Reg<regs::Ch2optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_tog(self) -> crate::common::Reg<regs::Ch2optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "DCP channel 3 command pointer address register"]
    #[inline(always)]
    pub const fn ch3cmdptr(self) -> crate::common::Reg<regs::Ch3cmdptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "DCP channel 3 semaphore register"]
    #[inline(always)]
    pub const fn ch3sema(self) -> crate::common::Reg<regs::Ch3sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat(self) -> crate::common::Reg<regs::Ch3stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_set(self) -> crate::common::Reg<regs::Ch3statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_clr(self) -> crate::common::Reg<regs::Ch3statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_tog(self) -> crate::common::Reg<regs::Ch3statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts(self) -> crate::common::Reg<regs::Ch3opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_set(self) -> crate::common::Reg<regs::Ch3optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_clr(self) -> crate::common::Reg<regs::Ch3optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_tog(self) -> crate::common::Reg<regs::Ch3optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "DCP debug select register"]
    #[inline(always)]
    pub const fn dbgselect(self) -> crate::common::Reg<regs::Dbgselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "DCP debug data register"]
    #[inline(always)]
    pub const fn dbgdata(self) -> crate::common::Reg<regs::Dbgdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "DCP page table register"]
    #[inline(always)]
    pub const fn pagetable(self) -> crate::common::Reg<regs::Pagetable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "DCP version register"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
}
pub mod regs;
pub mod vals;
