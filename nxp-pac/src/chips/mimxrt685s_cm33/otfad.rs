#[doc = "Array of registers: CTX_CTR%s, CTX_KEY%s, CTX_RGD_W0, CTX_RGD_W1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctx {
    ptr: *mut u8,
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
impl Ctx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AES Key Word"]
    #[inline(always)]
    pub const fn ctx_key(self, n: usize) -> crate::common::Reg<regs::CtxKey, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "AES Counter Word"]
    #[inline(always)]
    pub const fn ctx_ctr(self, n: usize) -> crate::common::Reg<regs::CtxCtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "AES Region Descriptor Word0"]
    #[inline(always)]
    pub const fn ctx_rgd_w0(self) -> crate::common::Reg<regs::CtxRgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "AES Region Descriptor Word1"]
    #[inline(always)]
    pub const fn ctx_rgd_w1(self) -> crate::common::Reg<regs::CtxRgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "OTFAD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otfad {
    ptr: *mut u8,
}
unsafe impl Send for Otfad {}
unsafe impl Sync for Otfad {}
impl Otfad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "Array of registers: CTX_CTR%s, CTX_KEY%s, CTX_RGD_W0, CTX_RGD_W1"]
    #[inline(always)]
    pub const fn ctx(self, n: usize) -> Ctx {
        assert!(n < 4usize);
        unsafe { Ctx::from_ptr(self.ptr.wrapping_add(0x0d00usize + n * 64usize) as _) }
    }
}
pub mod regs;
pub mod vals;
