#[doc = "PORT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port {
    ptr: *mut u8,
}
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
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
    #[doc = "Global Pin Control Low"]
    #[inline(always)]
    pub const fn gpclr(self) -> crate::common::Reg<regs::Gpclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Global Pin Control High"]
    #[inline(always)]
    pub const fn gpchr(self) -> crate::common::Reg<regs::Gpchr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Calibration 0"]
    #[inline(always)]
    pub const fn calib0(self) -> crate::common::Reg<regs::Calib0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Calibration 1"]
    #[inline(always)]
    pub const fn calib1(self) -> crate::common::Reg<regs::Calib1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pin Control 0"]
    #[inline(always)]
    pub const fn pcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
