#[doc = "FLASH_CFPA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_CFPA {
    ptr: *mut u8,
}
unsafe impl Send for FLASH_CFPA {}
unsafe impl Sync for FLASH_CFPA {}
impl FLASH_CFPA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn HEADER(self) -> crate::common::Reg<regs::HEADER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn VERSION(self) -> crate::common::Reg<regs::VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Secure firmware version (Monotonic counter)."]
    #[inline(always)]
    pub const fn S_FW_Version(self) -> crate::common::Reg<regs::S_FW_Version, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Non-Secure firmware version (Monotonic counter)."]
    #[inline(always)]
    pub const fn NS_FW_Version(self) -> crate::common::Reg<regs::NS_FW_Version, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Image key revocation ID (Monotonic counter)."]
    #[inline(always)]
    pub const fn IMAGE_KEY_REVOKE(
        self,
    ) -> crate::common::Reg<regs::IMAGE_KEY_REVOKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ROTKH_REVOKE(self) -> crate::common::Reg<regs::ROTKH_REVOKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn VENDOR_USAGE(self) -> crate::common::Reg<regs::VENDOR_USAGE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn DCFG_CC_SOCU_PIN(
        self,
    ) -> crate::common::Reg<regs::DCFG_CC_SOCU_PIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn DCFG_CC_SOCU_DFLT(
        self,
    ) -> crate::common::Reg<regs::DCFG_CC_SOCU_DFLT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
    #[inline(always)]
    pub const fn ENABLE_FA_MODE(
        self,
    ) -> crate::common::Reg<regs::ENABLE_FA_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
    #[inline(always)]
    pub const fn CMPA_PROG_IN_PROGRESS(
        self,
    ) -> crate::common::Reg<regs::CMPA_PROG_IN_PROGRESS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION0_IV_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION0_IV_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION1_IV_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION1_IV_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_HEADER0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_HEADER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_HEADER1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_HEADER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY0(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY1(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY2(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY3(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY4(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY5(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY6(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY7(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY8(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY9(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY10(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE12(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_BODY11(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_BODY11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn PRINCE_REGION2_IV_CODE13(
        self,
    ) -> crate::common::Reg<regs::PRINCE_REGION2_IV_CODE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Customer Defined (Programable through ROM API)."]
    #[inline(always)]
    pub const fn CUSTOMER_DEFINED(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CUSTOMER_DEFINED, crate::common::RW> {
        assert!(n < 56usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]."]
    #[inline(always)]
    pub const fn SHA256_DIGEST(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHA256_DIGEST, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
