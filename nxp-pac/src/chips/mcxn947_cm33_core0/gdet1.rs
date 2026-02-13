#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdet1 {
    ptr: *mut u8,
}
unsafe impl Send for Gdet1 {}
unsafe impl Sync for Gdet1 {}
impl Gdet1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GDET Configuration 0 Register"]
    #[inline(always)]
    pub const fn gdet_conf_0(self) -> crate::common::Reg<regs::GdetConf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GDET Configuration 1 Register"]
    #[inline(always)]
    pub const fn gdet_conf_1(self) -> crate::common::Reg<regs::GdetConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GDET Enable Register"]
    #[inline(always)]
    pub const fn gdet_enable1(self) -> crate::common::Reg<regs::GdetEnable1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GDET Configuration 2 Register"]
    #[inline(always)]
    pub const fn gdet_conf_2(self) -> crate::common::Reg<regs::GdetConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GDET Configuration 3 Register"]
    #[inline(always)]
    pub const fn gdet_conf_3(self) -> crate::common::Reg<regs::GdetConf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GDET Configuration 4 Register"]
    #[inline(always)]
    pub const fn gdet_conf_4(self) -> crate::common::Reg<regs::GdetConf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GDET Configuration 5 Register"]
    #[inline(always)]
    pub const fn gdet_conf_5(self) -> crate::common::Reg<regs::GdetConf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GDET Reset Register"]
    #[inline(always)]
    pub const fn gdet_reset(self) -> crate::common::Reg<regs::GdetReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "GDET Test Register"]
    #[inline(always)]
    pub const fn gdet_test(self) -> crate::common::Reg<regs::GdetTest, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "GDET Delay Control Register"]
    #[inline(always)]
    pub const fn gdet_dly_ctrl(self) -> crate::common::Reg<regs::GdetDlyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
    }
}
pub mod regs;
