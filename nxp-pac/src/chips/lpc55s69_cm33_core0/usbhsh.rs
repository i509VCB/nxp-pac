#[doc = "USB1 High-speed Host Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsh {
    ptr: *mut u8,
}
unsafe impl Send for Usbhsh {}
unsafe impl Sync for Usbhsh {}
impl Usbhsh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    #[inline(always)]
    pub const fn caplength_chipid(
        self,
    ) -> crate::common::Reg<regs::CaplengthChipid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters"]
    #[inline(always)]
    pub const fn hcsparams(self) -> crate::common::Reg<regs::Hcsparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Frame Length Adjustment"]
    #[inline(always)]
    pub const fn fladj_frindex(self) -> crate::common::Reg<regs::FladjFrindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Memory base address where ATL PTD0 is stored"]
    #[inline(always)]
    pub const fn atlptd(self) -> crate::common::Reg<regs::Atlptd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Memory base address where ISO PTD0 is stored"]
    #[inline(always)]
    pub const fn isoptd(self) -> crate::common::Reg<regs::Isoptd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Memory base address where INT PTD0 is stored"]
    #[inline(always)]
    pub const fn intptd(self) -> crate::common::Reg<regs::Intptd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Memory base address that indicates the start of the data payload buffers"]
    #[inline(always)]
    pub const fn datapayload(self) -> crate::common::Reg<regs::Datapayload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB Command register"]
    #[inline(always)]
    pub const fn usbcmd(self) -> crate::common::Reg<regs::Usbcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB Interrupt Status register"]
    #[inline(always)]
    pub const fn usbsts(self) -> crate::common::Reg<regs::Usbsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB Interrupt Enable register"]
    #[inline(always)]
    pub const fn usbintr(self) -> crate::common::Reg<regs::Usbintr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Port Status and Control register"]
    #[inline(always)]
    pub const fn portsc1(self) -> crate::common::Reg<regs::Portsc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Done map for each ATL PTD"]
    #[inline(always)]
    pub const fn atlptdd(self) -> crate::common::Reg<regs::Atlptdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Skip map for each ATL PTD"]
    #[inline(always)]
    pub const fn atlptds(self) -> crate::common::Reg<regs::Atlptds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Done map for each ISO PTD"]
    #[inline(always)]
    pub const fn isoptdd(self) -> crate::common::Reg<regs::Isoptdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Skip map for each ISO PTD"]
    #[inline(always)]
    pub const fn isoptds(self) -> crate::common::Reg<regs::Isoptds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Done map for each INT PTD"]
    #[inline(always)]
    pub const fn intptdd(self) -> crate::common::Reg<regs::Intptdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Skip map for each INT PTD"]
    #[inline(always)]
    pub const fn intptds(self) -> crate::common::Reg<regs::Intptds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
    #[inline(always)]
    pub const fn lastptd(self) -> crate::common::Reg<regs::Lastptd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Controls the port if it is attached to the host block or the device block"]
    #[inline(always)]
    pub const fn portmode(self) -> crate::common::Reg<regs::Portmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs;
