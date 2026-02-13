#[doc = "SEMA42"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sema420 {
    ptr: *mut u8,
}
unsafe impl Send for Sema420 {}
unsafe impl Sync for Sema420 {}
impl Sema420 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate3(self) -> crate::common::Reg<regs::Gate3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate2(self) -> crate::common::Reg<regs::Gate2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate1(self) -> crate::common::Reg<regs::Gate1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate0(self) -> crate::common::Reg<regs::Gate0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate7(self) -> crate::common::Reg<regs::Gate7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate6(self) -> crate::common::Reg<regs::Gate6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate5(self) -> crate::common::Reg<regs::Gate5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate4(self) -> crate::common::Reg<regs::Gate4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate11(self) -> crate::common::Reg<regs::Gate11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate10(self) -> crate::common::Reg<regs::Gate10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate9(self) -> crate::common::Reg<regs::Gate9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate8(self) -> crate::common::Reg<regs::Gate8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate15(self) -> crate::common::Reg<regs::Gate15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate14(self) -> crate::common::Reg<regs::Gate14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate13(self) -> crate::common::Reg<regs::Gate13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate12(self) -> crate::common::Reg<regs::Gate12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "Reset Gate Read"]
    #[inline(always)]
    pub const fn rstgt_r(self) -> crate::common::Reg<regs::RstgtR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Reset Gate Write"]
    #[inline(always)]
    pub const fn rstgt_w(self) -> crate::common::Reg<regs::RstgtW, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
}
pub mod regs;
pub mod vals;
