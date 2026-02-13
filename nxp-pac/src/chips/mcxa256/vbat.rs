#[doc = "VBAT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbat {
    ptr: *mut u8,
}
unsafe impl Send for Vbat {}
unsafe impl Sync for Vbat {}
impl Vbat {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FRO16K Control A"]
    #[inline(always)]
    pub const fn froctla(self) -> crate::common::Reg<regs::Froctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FRO16K Lock A"]
    #[inline(always)]
    pub const fn frolcka(self) -> crate::common::Reg<regs::Frolcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FRO16K Clock Enable"]
    #[inline(always)]
    pub const fn froclke(self) -> crate::common::Reg<regs::Froclke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Array of registers: WAKEUPA"]
    #[inline(always)]
    pub const fn wakeup(self, n: usize) -> Wakeup {
        assert!(n < 2usize);
        unsafe { Wakeup::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 8usize) as _) }
    }
    #[doc = "Wakeup Lock A"]
    #[inline(always)]
    pub const fn waklcka(self) -> crate::common::Reg<regs::Waklcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
}
#[doc = "Array of registers: WAKEUPA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeup {
    ptr: *mut u8,
}
unsafe impl Send for Wakeup {}
unsafe impl Sync for Wakeup {}
impl Wakeup {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wakeup 0 Register A"]
    #[inline(always)]
    pub const fn wakeupa(self) -> crate::common::Reg<regs::Wakeupa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
