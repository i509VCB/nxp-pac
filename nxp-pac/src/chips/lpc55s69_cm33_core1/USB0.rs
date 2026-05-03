#[doc = "USB 2.0 Device Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB0 {
    ptr: *mut u8,
}
unsafe impl Send for USB0 {}
unsafe impl Sync for USB0 {}
impl USB0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Device Command/Status register."]
    #[inline(always)]
    pub const fn DEVCMDSTAT(self) -> crate::common::Reg<regs::DEVCMDSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB Info register."]
    #[inline(always)]
    pub const fn INFO(self) -> crate::common::Reg<regs::INFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB EP Command/Status List start address."]
    #[inline(always)]
    pub const fn EPLISTSTART(self) -> crate::common::Reg<regs::EPLISTSTART, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB Data buffer start address."]
    #[inline(always)]
    pub const fn DATABUFSTART(self) -> crate::common::Reg<regs::DATABUFSTART, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB Link Power Management register."]
    #[inline(always)]
    pub const fn LPM(self) -> crate::common::Reg<regs::LPM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB Endpoint skip."]
    #[inline(always)]
    pub const fn EPSKIP(self) -> crate::common::Reg<regs::EPSKIP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB Endpoint Buffer in use."]
    #[inline(always)]
    pub const fn EPINUSE(self) -> crate::common::Reg<regs::EPINUSE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB Endpoint Buffer Configuration register."]
    #[inline(always)]
    pub const fn EPBUFCFG(self) -> crate::common::Reg<regs::EPBUFCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB interrupt status register."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<regs::INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB set interrupt status register."]
    #[inline(always)]
    pub const fn INTSETSTAT(self) -> crate::common::Reg<regs::INTSETSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USB Endpoint toggle register."]
    #[inline(always)]
    pub const fn EPTOGGLE(self) -> crate::common::Reg<regs::EPTOGGLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
pub mod regs;
pub mod vals;
