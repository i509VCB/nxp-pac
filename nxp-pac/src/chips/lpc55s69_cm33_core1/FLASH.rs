#[doc = "FLASH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH {
    ptr: *mut u8,
}
unsafe impl Send for FLASH {}
unsafe impl Sync for FLASH {}
impl FLASH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "command register."]
    #[inline(always)]
    pub const fn CMD(self) -> crate::common::Reg<regs::CMD, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "event register."]
    #[inline(always)]
    pub const fn EVENT(self) -> crate::common::Reg<regs::EVENT, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "start (or only) address for next flash command."]
    #[inline(always)]
    pub const fn STARTA(self) -> crate::common::Reg<regs::STARTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "end address for next flash command, if command operates on address ranges."]
    #[inline(always)]
    pub const fn STOPA(self) -> crate::common::Reg<regs::STOPA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
    #[inline(always)]
    pub const fn DATAW(self, n: usize) -> crate::common::Reg<regs::DATAW, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Clear interrupt enable bits."]
    #[inline(always)]
    pub const fn INT_CLR_ENABLE(
        self,
    ) -> crate::common::Reg<regs::INT_CLR_ENABLE, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "Set interrupt enable bits."]
    #[inline(always)]
    pub const fn INT_SET_ENABLE(
        self,
    ) -> crate::common::Reg<regs::INT_SET_ENABLE, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "Interrupt status bits."]
    #[inline(always)]
    pub const fn INT_STATUS(self) -> crate::common::Reg<regs::INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Interrupt enable bits."]
    #[inline(always)]
    pub const fn INT_ENABLE(self) -> crate::common::Reg<regs::INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Clear interrupt status bits."]
    #[inline(always)]
    pub const fn INT_CLR_STATUS(
        self,
    ) -> crate::common::Reg<regs::INT_CLR_STATUS, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Set interrupt status bits."]
    #[inline(always)]
    pub const fn INT_SET_STATUS(
        self,
    ) -> crate::common::Reg<regs::INT_SET_STATUS, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Controller+Memory module identification."]
    #[inline(always)]
    pub const fn MODULE_ID(self) -> crate::common::Reg<regs::MODULE_ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
