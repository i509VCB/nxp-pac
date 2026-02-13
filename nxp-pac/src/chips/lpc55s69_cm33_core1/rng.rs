#[doc = "RNG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    #[inline(always)]
    pub const fn random_number(self) -> crate::common::Reg<regs::RandomNumber, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn counter_val(self) -> crate::common::Reg<regs::CounterVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn counter_cfg(self) -> crate::common::Reg<regs::CounterCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn online_test_cfg(
        self,
    ) -> crate::common::Reg<regs::OnlineTestCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn online_test_val(
        self,
    ) -> crate::common::Reg<regs::OnlineTestVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "IP identifier"]
    #[inline(always)]
    pub const fn moduleid(self) -> crate::common::Reg<regs::Moduleid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
