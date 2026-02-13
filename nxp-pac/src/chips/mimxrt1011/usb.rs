#[doc = "USB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Identification register"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Hardware General"]
    #[inline(always)]
    pub const fn hwgeneral(self) -> crate::common::Reg<regs::Hwgeneral, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Host Hardware Parameters"]
    #[inline(always)]
    pub const fn hwhost(self) -> crate::common::Reg<regs::Hwhost, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Device Hardware Parameters"]
    #[inline(always)]
    pub const fn hwdevice(self) -> crate::common::Reg<regs::Hwdevice, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Buffer Hardware Parameters"]
    #[inline(always)]
    pub const fn hwtxbuf(self) -> crate::common::Reg<regs::Hwtxbuf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RX Buffer Hardware Parameters"]
    #[inline(always)]
    pub const fn hwrxbuf(self) -> crate::common::Reg<regs::Hwrxbuf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Load"]
    #[inline(always)]
    pub const fn gptimer0ld(self) -> crate::common::Reg<regs::Gptimer0ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Controller"]
    #[inline(always)]
    pub const fn gptimer0ctrl(self) -> crate::common::Reg<regs::Gptimer0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Load"]
    #[inline(always)]
    pub const fn gptimer1ld(self) -> crate::common::Reg<regs::Gptimer1ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Controller"]
    #[inline(always)]
    pub const fn gptimer1ctrl(self) -> crate::common::Reg<regs::Gptimer1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "System Bus Config"]
    #[inline(always)]
    pub const fn sbuscfg(self) -> crate::common::Reg<regs::Sbuscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Capability Registers Length"]
    #[inline(always)]
    pub const fn caplength(self) -> crate::common::Reg<regs::Caplength, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Host Controller Interface Version"]
    #[inline(always)]
    pub const fn hciversion(self) -> crate::common::Reg<regs::Hciversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters"]
    #[inline(always)]
    pub const fn hcsparams(self) -> crate::common::Reg<regs::Hcsparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Host Controller Capability Parameters"]
    #[inline(always)]
    pub const fn hccparams(self) -> crate::common::Reg<regs::Hccparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Device Controller Interface Version"]
    #[inline(always)]
    pub const fn dciversion(self) -> crate::common::Reg<regs::Dciversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Device Controller Capability Parameters"]
    #[inline(always)]
    pub const fn dccparams(self) -> crate::common::Reg<regs::Dccparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB Command Register"]
    #[inline(always)]
    pub const fn usbcmd(self) -> crate::common::Reg<regs::Usbcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(self) -> crate::common::Reg<regs::Usbsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn usbintr(self) -> crate::common::Reg<regs::Usbintr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "USB Frame Index"]
    #[inline(always)]
    pub const fn frindex(self) -> crate::common::Reg<regs::Frindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Device Address"]
    #[inline(always)]
    pub const fn deviceaddr(self) -> crate::common::Reg<regs::Deviceaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Frame List Base Address"]
    #[inline(always)]
    pub const fn periodiclistbase(
        self,
    ) -> crate::common::Reg<regs::Periodiclistbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Next Asynch. Address"]
    #[inline(always)]
    pub const fn asynclistaddr(self) -> crate::common::Reg<regs::Asynclistaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Endpoint List Address"]
    #[inline(always)]
    pub const fn endptlistaddr(self) -> crate::common::Reg<regs::Endptlistaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Programmable Burst Size"]
    #[inline(always)]
    pub const fn burstsize(self) -> crate::common::Reg<regs::Burstsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "TX FIFO Fill Tuning"]
    #[inline(always)]
    pub const fn txfilltuning(self) -> crate::common::Reg<regs::Txfilltuning, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Endpoint NAK"]
    #[inline(always)]
    pub const fn endptnak(self) -> crate::common::Reg<regs::Endptnak, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Endpoint NAK Enable"]
    #[inline(always)]
    pub const fn endptnaken(self) -> crate::common::Reg<regs::Endptnaken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Configure Flag Register"]
    #[inline(always)]
    pub const fn configflag(self) -> crate::common::Reg<regs::Configflag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Port Status & Control"]
    #[inline(always)]
    pub const fn portsc1(self) -> crate::common::Reg<regs::Portsc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "On-The-Go Status & control"]
    #[inline(always)]
    pub const fn otgsc(self) -> crate::common::Reg<regs::Otgsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "USB Device Mode"]
    #[inline(always)]
    pub const fn usbmode(self) -> crate::common::Reg<regs::Usbmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Endpoint Setup Status"]
    #[inline(always)]
    pub const fn endptsetupstat(
        self,
    ) -> crate::common::Reg<regs::Endptsetupstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Endpoint Prime"]
    #[inline(always)]
    pub const fn endptprime(self) -> crate::common::Reg<regs::Endptprime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Endpoint Flush"]
    #[inline(always)]
    pub const fn endptflush(self) -> crate::common::Reg<regs::Endptflush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Endpoint Status"]
    #[inline(always)]
    pub const fn endptstat(self) -> crate::common::Reg<regs::Endptstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Endpoint Complete"]
    #[inline(always)]
    pub const fn endptcomplete(self) -> crate::common::Reg<regs::Endptcomplete, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Endpoint Control0"]
    #[inline(always)]
    pub const fn endptctrl0(self) -> crate::common::Reg<regs::Endptctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Endpoint Control 1"]
    #[inline(always)]
    pub const fn endptctrl1(self) -> crate::common::Reg<regs::Endptctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Endpoint Control 2"]
    #[inline(always)]
    pub const fn endptctrl2(self) -> crate::common::Reg<regs::Endptctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Endpoint Control 3"]
    #[inline(always)]
    pub const fn endptctrl3(self) -> crate::common::Reg<regs::Endptctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Endpoint Control 4"]
    #[inline(always)]
    pub const fn endptctrl4(self) -> crate::common::Reg<regs::Endptctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Endpoint Control 5"]
    #[inline(always)]
    pub const fn endptctrl5(self) -> crate::common::Reg<regs::Endptctrl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Endpoint Control 6"]
    #[inline(always)]
    pub const fn endptctrl6(self) -> crate::common::Reg<regs::Endptctrl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Endpoint Control 7"]
    #[inline(always)]
    pub const fn endptctrl7(self) -> crate::common::Reg<regs::Endptctrl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
