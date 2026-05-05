#[doc = "PUF SRAM Control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PUF_SRAM_CTRL {
    ptr: *mut u8,
}
unsafe impl Send for PUF_SRAM_CTRL {}
unsafe impl Sync for PUF_SRAM_CTRL {}
impl PUF_SRAM_CTRL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Enable Clear Register."]
    #[inline(always)]
    pub const fn INT_CLR_ENABLE(
        self,
    ) -> crate::common::Reg<regs::INT_CLR_ENABLE, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Interrupt Enable Set Register."]
    #[inline(always)]
    pub const fn INT_SET_ENABLE(
        self,
    ) -> crate::common::Reg<regs::INT_SET_ENABLE, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn INT_STATUS(self) -> crate::common::Reg<regs::INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn INT_ENABLE(self) -> crate::common::Reg<regs::INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Interrupt Status Clear Register."]
    #[inline(always)]
    pub const fn INT_CLR_STATUS(
        self,
    ) -> crate::common::Reg<regs::INT_CLR_STATUS, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Interrupt Status set."]
    #[inline(always)]
    pub const fn INT_SET_STATUS(
        self,
    ) -> crate::common::Reg<regs::INT_SET_STATUS, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
}
pub mod regs;
