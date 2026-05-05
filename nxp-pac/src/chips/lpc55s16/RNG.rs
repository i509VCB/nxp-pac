#[doc = "RNG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RNG {
    ptr: *mut u8,
}
unsafe impl Send for RNG {}
unsafe impl Sync for RNG {}
impl RNG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub const fn RANDOM_NUMBER(self) -> crate::common::Reg<regs::RANDOM_NUMBER, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub const fn ENCRYPTED_NUMBER(
        self,
    ) -> crate::common::Reg<regs::ENCRYPTED_NUMBER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn COUNTER_VAL(self) -> crate::common::Reg<regs::COUNTER_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn COUNTER_CFG(self) -> crate::common::Reg<regs::COUNTER_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ONLINE_TEST_CFG(
        self,
    ) -> crate::common::Reg<regs::ONLINE_TEST_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ONLINE_TEST_VAL(
        self,
    ) -> crate::common::Reg<regs::ONLINE_TEST_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ENTROPY_INJECT(
        self,
    ) -> crate::common::Reg<regs::ENTROPY_INJECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn MISC_CFG(self) -> crate::common::Reg<regs::MISC_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Powerdown mode (standard but certainly useless here)."]
    #[inline(always)]
    pub const fn POWERDOWN(self) -> crate::common::Reg<regs::POWERDOWN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "IP identifier."]
    #[inline(always)]
    pub const fn MODULEID(self) -> crate::common::Reg<regs::MODULEID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
