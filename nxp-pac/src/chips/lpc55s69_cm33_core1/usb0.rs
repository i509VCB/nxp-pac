#[doc = "USB 2.0 Device Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0 {
    ptr: *mut u8,
}
unsafe impl Send for Usb0 {}
unsafe impl Sync for Usb0 {}
impl Usb0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Device Command/Status register"]
    #[inline(always)]
    pub const fn devcmdstat(self) -> crate::common::Reg<regs::Devcmdstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB Info register"]
    #[inline(always)]
    pub const fn info(self) -> crate::common::Reg<regs::Info, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB EP Command/Status List start address"]
    #[inline(always)]
    pub const fn epliststart(self) -> crate::common::Reg<regs::Epliststart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB Data buffer start address"]
    #[inline(always)]
    pub const fn databufstart(self) -> crate::common::Reg<regs::Databufstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB Link Power Management register"]
    #[inline(always)]
    pub const fn lpm(self) -> crate::common::Reg<regs::Lpm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB Endpoint skip"]
    #[inline(always)]
    pub const fn epskip(self) -> crate::common::Reg<regs::Epskip, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB Endpoint Buffer in use"]
    #[inline(always)]
    pub const fn epinuse(self) -> crate::common::Reg<regs::Epinuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB Endpoint Buffer Configuration register"]
    #[inline(always)]
    pub const fn epbufcfg(self) -> crate::common::Reg<regs::Epbufcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB interrupt status register"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB interrupt enable register"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB set interrupt status register"]
    #[inline(always)]
    pub const fn intsetstat(self) -> crate::common::Reg<regs::Intsetstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USB Endpoint toggle register"]
    #[inline(always)]
    pub const fn eptoggle(self) -> crate::common::Reg<regs::Eptoggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
pub mod regs;
pub mod vals;
