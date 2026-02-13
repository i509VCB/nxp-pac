#[doc = "AND/OR/INVERT module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi {
    ptr: *mut u8,
}
unsafe impl Send for Aoi {}
unsafe impl Sync for Aoi {}
impl Aoi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    #[inline(always)]
    pub const fn bfcrt01(self, n: usize) -> crate::common::Reg<regs::Bfcrt01, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    #[inline(always)]
    pub const fn bfcrt23(self, n: usize) -> crate::common::Reg<regs::Bfcrt23, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
