#[doc = "Array of registers: VMAPCTX_WD%s, BIVCTX_WD%s"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxValidIvArray {
    ptr: *mut u8,
}
unsafe impl Send for CtxValidIvArray {}
unsafe impl Sync for CtxValidIvArray {}
impl CtxValidIvArray {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bitmap of Valid Control for Memory Context n"]
    #[inline(always)]
    pub const fn vmapctx_wd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::VmapctxWd, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Block Initial Vector for Memory Context n"]
    #[inline(always)]
    pub const fn bivctx_wd(self, n: usize) -> crate::common::Reg<regs::BivctxWd, crate::common::W> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
}
#[doc = "NPX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npx0 {
    ptr: *mut u8,
}
unsafe impl Send for Npx0 {}
unsafe impl Sync for Npx0 {}
impl Npx0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NPX Control Register"]
    #[inline(always)]
    pub const fn npxcr(self) -> crate::common::Reg<regs::Npxcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "NPX Status Register"]
    #[inline(always)]
    pub const fn npxsr(self) -> crate::common::Reg<regs::Npxsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Cache Obfuscation Mask"]
    #[inline(always)]
    pub const fn cacmsk(self) -> crate::common::Reg<regs::Cacmsk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data Remap"]
    #[inline(always)]
    pub const fn remap(self) -> crate::common::Reg<regs::Remap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Array of registers: VMAPCTX_WD%s, BIVCTX_WD%s"]
    #[inline(always)]
    pub const fn ctx_valid_iv_array(self, n: usize) -> CtxValidIvArray {
        assert!(n < 4usize);
        unsafe { CtxValidIvArray::from_ptr(self.ptr.wrapping_add(0x40usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
