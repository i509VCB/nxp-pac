#[doc = "AOI"]
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
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0"]
    #[inline(always)]
    pub const fn bfcrt010(self) -> crate::common::Reg<regs::Bfcrt010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0"]
    #[inline(always)]
    pub const fn bfcrt230(self) -> crate::common::Reg<regs::Bfcrt230, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1"]
    #[inline(always)]
    pub const fn bfcrt011(self) -> crate::common::Reg<regs::Bfcrt011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1"]
    #[inline(always)]
    pub const fn bfcrt231(self) -> crate::common::Reg<regs::Bfcrt231, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2"]
    #[inline(always)]
    pub const fn bfcrt012(self) -> crate::common::Reg<regs::Bfcrt012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2"]
    #[inline(always)]
    pub const fn bfcrt232(self) -> crate::common::Reg<regs::Bfcrt232, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3"]
    #[inline(always)]
    pub const fn bfcrt013(self) -> crate::common::Reg<regs::Bfcrt013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3"]
    #[inline(always)]
    pub const fn bfcrt233(self) -> crate::common::Reg<regs::Bfcrt233, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
