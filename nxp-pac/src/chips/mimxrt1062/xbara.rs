#[doc = "Crossbar Switch"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbara {
    ptr: *mut u8,
}
unsafe impl Send for Xbara {}
unsafe impl Sync for Xbara {}
impl Xbara {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crossbar B Select Register 0"]
    #[inline(always)]
    pub const fn sel0(self) -> crate::common::Reg<regs::Sel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Crossbar B Select Register 1"]
    #[inline(always)]
    pub const fn sel1(self) -> crate::common::Reg<regs::Sel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Crossbar B Select Register 2"]
    #[inline(always)]
    pub const fn sel2(self) -> crate::common::Reg<regs::Sel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Crossbar B Select Register 3"]
    #[inline(always)]
    pub const fn sel3(self) -> crate::common::Reg<regs::Sel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Crossbar B Select Register 4"]
    #[inline(always)]
    pub const fn sel4(self) -> crate::common::Reg<regs::Sel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Crossbar B Select Register 5"]
    #[inline(always)]
    pub const fn sel5(self) -> crate::common::Reg<regs::Sel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Crossbar B Select Register 6"]
    #[inline(always)]
    pub const fn sel6(self) -> crate::common::Reg<regs::Sel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Crossbar B Select Register 7"]
    #[inline(always)]
    pub const fn sel7(self) -> crate::common::Reg<regs::Sel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs;
