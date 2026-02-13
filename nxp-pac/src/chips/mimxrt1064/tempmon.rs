#[doc = "Temperature Monitor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempmon {
    ptr: *mut u8,
}
unsafe impl Send for Tempmon {}
unsafe impl Sync for Tempmon {}
impl Tempmon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0(self) -> crate::common::Reg<regs::Tempsense0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1(self) -> crate::common::Reg<regs::Tempsense1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2(self) -> crate::common::Reg<regs::Tempsense2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
