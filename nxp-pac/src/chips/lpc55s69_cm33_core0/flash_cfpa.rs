#[doc = "FLASH_CFPA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashCfpa {
    ptr: *mut u8,
}
unsafe impl Send for FlashCfpa {}
unsafe impl Sync for FlashCfpa {}
impl FlashCfpa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn header(self) -> crate::common::Reg<regs::Header, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Secure firmware version (Monotonic counter)"]
    #[inline(always)]
    pub const fn s_fw_version(self) -> crate::common::Reg<regs::SFwVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Non-Secure firmware version (Monotonic counter)"]
    #[inline(always)]
    pub const fn ns_fw_version(self) -> crate::common::Reg<regs::NsFwVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Image key revocation ID (Monotonic counter)"]
    #[inline(always)]
    pub const fn image_key_revoke(
        self,
    ) -> crate::common::Reg<regs::ImageKeyRevoke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn rotkh_revoke(self) -> crate::common::Reg<regs::RotkhRevoke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn vendor_usage(self) -> crate::common::Reg<regs::VendorUsage, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn dcfg_cc_socu_pin(
        self,
    ) -> crate::common::Reg<regs::DcfgCcSocuPin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
    #[inline(always)]
    pub const fn dcfg_cc_socu_dflt(
        self,
    ) -> crate::common::Reg<regs::DcfgCcSocuDflt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
    #[inline(always)]
    pub const fn enable_fa_mode(self) -> crate::common::Reg<regs::EnableFaMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
    #[inline(always)]
    pub const fn cmpa_prog_in_progress(
        self,
    ) -> crate::common::Reg<regs::CmpaProgInProgress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region0_iv_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion0IvCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region1_iv_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion1IvCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_header0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvHeader0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_header1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvHeader1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body0(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body1(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body2(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body3(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body4(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body5(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body6(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body7(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body8(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body9(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body10(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code12(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_body11(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvBody11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn prince_region2_iv_code13(
        self,
    ) -> crate::common::Reg<regs::PrinceRegion2IvCode13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Customer Defined (Programable through ROM API)"]
    #[inline(always)]
    pub const fn customer_defined(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CustomerDefined, crate::common::RW> {
        assert!(n < 56usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sha256Digest, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
