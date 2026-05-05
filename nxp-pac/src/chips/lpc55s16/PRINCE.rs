#[doc = "PRINCE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE {
    ptr: *mut u8,
}
unsafe impl Send for PRINCE {}
unsafe impl Sync for PRINCE {}
impl PRINCE {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Encryption Enable register."]
    #[inline(always)]
    pub const fn ENC_ENABLE(self) -> crate::common::Reg<regs::ENC_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data Mask register, 32 Least Significant Bits."]
    #[inline(always)]
    pub const fn MASK_LSB(self) -> crate::common::Reg<regs::MASK_LSB, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data Mask register, 32 Most Significant Bits."]
    #[inline(always)]
    pub const fn MASK_MSB(self) -> crate::common::Reg<regs::MASK_MSB, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Lock register."]
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Initial Vector register for region 0, Least Significant Bits."]
    #[inline(always)]
    pub const fn IV_LSB0(self) -> crate::common::Reg<regs::IV_LSB0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Initial Vector register for region 0, Most Significant Bits."]
    #[inline(always)]
    pub const fn IV_MSB0(self) -> crate::common::Reg<regs::IV_MSB0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Base Address for region 0 register."]
    #[inline(always)]
    pub const fn BASE_ADDR0(self) -> crate::common::Reg<regs::BASE_ADDR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 0."]
    #[inline(always)]
    pub const fn SR_ENABLE0(self) -> crate::common::Reg<regs::SR_ENABLE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Initial Vector register for region 1, Least Significant Bits."]
    #[inline(always)]
    pub const fn IV_LSB1(self) -> crate::common::Reg<regs::IV_LSB1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Initial Vector register for region 1, Most Significant Bits."]
    #[inline(always)]
    pub const fn IV_MSB1(self) -> crate::common::Reg<regs::IV_MSB1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Base Address for region 1 register."]
    #[inline(always)]
    pub const fn BASE_ADDR1(self) -> crate::common::Reg<regs::BASE_ADDR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 1."]
    #[inline(always)]
    pub const fn SR_ENABLE1(self) -> crate::common::Reg<regs::SR_ENABLE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Initial Vector register for region 2, Least Significant Bits."]
    #[inline(always)]
    pub const fn IV_LSB2(self) -> crate::common::Reg<regs::IV_LSB2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Initial Vector register for region 2, Most Significant Bits."]
    #[inline(always)]
    pub const fn IV_MSB2(self) -> crate::common::Reg<regs::IV_MSB2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Base Address for region 2 register."]
    #[inline(always)]
    pub const fn BASE_ADDR2(self) -> crate::common::Reg<regs::BASE_ADDR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 2."]
    #[inline(always)]
    pub const fn SR_ENABLE2(self) -> crate::common::Reg<regs::SR_ENABLE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Error status register."]
    #[inline(always)]
    pub const fn ERR(self) -> crate::common::Reg<regs::ERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
}
pub mod regs;
