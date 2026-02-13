#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdet0 {
    ptr: *mut u8,
}
unsafe impl Send for Gdet0 {}
unsafe impl Sync for Gdet0 {}
impl Gdet0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GDET configuration 0 register"]
    #[inline(always)]
    pub const fn gdet_conf_0(self) -> crate::common::Reg<regs::GdetConf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GDET configuration 1 register"]
    #[inline(always)]
    pub const fn gdet_conf_1(self) -> crate::common::Reg<regs::GdetConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GDET enable register"]
    #[inline(always)]
    pub const fn gdet_enable1(self) -> crate::common::Reg<regs::GdetEnable1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GDET configuration 2 register"]
    #[inline(always)]
    pub const fn gdet_conf_2(self) -> crate::common::Reg<regs::GdetConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GDET configuration 3 register"]
    #[inline(always)]
    pub const fn gdet_conf_3(self) -> crate::common::Reg<regs::GdetConf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GDET configuration 4 register"]
    #[inline(always)]
    pub const fn gdet_conf_4(self) -> crate::common::Reg<regs::GdetConf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GDET configuration 5 register"]
    #[inline(always)]
    pub const fn gdet_conf_5(self) -> crate::common::Reg<regs::GdetConf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GDET reset register"]
    #[inline(always)]
    pub const fn gdet_reset(self) -> crate::common::Reg<regs::GdetReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "GDET test Register"]
    #[inline(always)]
    pub const fn gdet_test(self) -> crate::common::Reg<regs::GdetTest, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "GDET delay control register"]
    #[inline(always)]
    pub const fn gdet_dly_ctrl(self) -> crate::common::Reg<regs::GdetDlyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
    }
}
pub mod regs;
