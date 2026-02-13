#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Udf {
    ptr: *mut u8,
}
unsafe impl Send for Udf {}
unsafe impl Sync for Udf {}
impl Udf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn udf_ctrl(self) -> crate::common::Reg<regs::UdfCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn udf_status(self) -> crate::common::Reg<regs::UdfStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data In Register"]
    #[inline(always)]
    pub const fn udf_wr_data(self) -> crate::common::Reg<regs::UdfWrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data Out Register"]
    #[inline(always)]
    pub const fn udf_rd_data(self) -> crate::common::Reg<regs::UdfRdData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
