#[doc = "GPT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt {
    ptr: *mut u8,
}
unsafe impl Send for Gpt {}
unsafe impl Sync for Gpt {}
impl Gpt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPT Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPT Prescaler Register"]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPT Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPT Interrupt Register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPT Output Compare Register 1"]
    #[inline(always)]
    pub const fn ocr1(self) -> crate::common::Reg<regs::Ocr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPT Output Compare Register 2"]
    #[inline(always)]
    pub const fn ocr2(self) -> crate::common::Reg<regs::Ocr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPT Output Compare Register 3"]
    #[inline(always)]
    pub const fn ocr3(self) -> crate::common::Reg<regs::Ocr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPT Input Capture Register 1"]
    #[inline(always)]
    pub const fn icr1(self) -> crate::common::Reg<regs::Icr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPT Input Capture Register 2"]
    #[inline(always)]
    pub const fn icr2(self) -> crate::common::Reg<regs::Icr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPT Counter Register"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;
pub mod vals;
