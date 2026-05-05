#[doc = "USB1 High-speed Host Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBHSH {
    ptr: *mut u8,
}
unsafe impl Send for USBHSH {}
unsafe impl Sync for USBHSH {}
impl USBHSH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block."]
    #[inline(always)]
    pub const fn CAPLENGTH_CHIPID(
        self,
    ) -> crate::common::Reg<regs::CAPLENGTH_CHIPID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters."]
    #[inline(always)]
    pub const fn HCSPARAMS(self) -> crate::common::Reg<regs::HCSPARAMS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Frame Length Adjustment."]
    #[inline(always)]
    pub const fn FLADJ_FRINDEX(self) -> crate::common::Reg<regs::FLADJ_FRINDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Memory base address where ATL PTD0 is stored."]
    #[inline(always)]
    pub const fn ATLPTD(self) -> crate::common::Reg<regs::ATLPTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Memory base address where ISO PTD0 is stored."]
    #[inline(always)]
    pub const fn ISOPTD(self) -> crate::common::Reg<regs::ISOPTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Memory base address where INT PTD0 is stored."]
    #[inline(always)]
    pub const fn INTPTD(self) -> crate::common::Reg<regs::INTPTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Memory base address that indicates the start of the data payload buffers."]
    #[inline(always)]
    pub const fn DATAPAYLOAD(self) -> crate::common::Reg<regs::DATAPAYLOAD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB Command register."]
    #[inline(always)]
    pub const fn USBCMD(self) -> crate::common::Reg<regs::USBCMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB Interrupt Status register."]
    #[inline(always)]
    pub const fn USBSTS(self) -> crate::common::Reg<regs::USBSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB Interrupt Enable register."]
    #[inline(always)]
    pub const fn USBINTR(self) -> crate::common::Reg<regs::USBINTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Port Status and Control register."]
    #[inline(always)]
    pub const fn PORTSC1(self) -> crate::common::Reg<regs::PORTSC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Done map for each ATL PTD."]
    #[inline(always)]
    pub const fn ATLPTDD(self) -> crate::common::Reg<regs::ATLPTDD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Skip map for each ATL PTD."]
    #[inline(always)]
    pub const fn ATLPTDS(self) -> crate::common::Reg<regs::ATLPTDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Done map for each ISO PTD."]
    #[inline(always)]
    pub const fn ISOPTDD(self) -> crate::common::Reg<regs::ISOPTDD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Skip map for each ISO PTD."]
    #[inline(always)]
    pub const fn ISOPTDS(self) -> crate::common::Reg<regs::ISOPTDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Done map for each INT PTD."]
    #[inline(always)]
    pub const fn INTPTDD(self) -> crate::common::Reg<regs::INTPTDD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Skip map for each INT PTD."]
    #[inline(always)]
    pub const fn INTPTDS(self) -> crate::common::Reg<regs::INTPTDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Marks the last PTD in the list for ISO, INT and ATL."]
    #[inline(always)]
    pub const fn LASTPTD(self) -> crate::common::Reg<regs::LASTPTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Controls the port if it is attached to the host block or the device block."]
    #[inline(always)]
    pub const fn PORTMODE(self) -> crate::common::Reg<regs::PORTMODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs;
