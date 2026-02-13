#[doc = "Config security level register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csl(pub u32);
impl Csl {
    #[doc = "Secure user read access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn sur_s2(&self) -> super::vals::SurS2 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SurS2::from_bits(val as u8)
    }
    #[doc = "Secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn set_sur_s2(&mut self, val: super::vals::SurS2) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure supervisor read access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn ssr_s2(&self) -> super::vals::SsrS2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SsrS2::from_bits(val as u8)
    }
    #[doc = "Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn set_ssr_s2(&mut self, val: super::vals::SsrS2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Non-secure user read access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nur_s2(&self) -> super::vals::NurS2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::NurS2::from_bits(val as u8)
    }
    #[doc = "Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn set_nur_s2(&mut self, val: super::vals::NurS2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Non-secure supervisor read access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nsr_s2(&self) -> super::vals::NsrS2 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::NsrS2::from_bits(val as u8)
    }
    #[doc = "Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn set_nsr_s2(&mut self, val: super::vals::NsrS2) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure user write access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn suw_s2(&self) -> super::vals::SuwS2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SuwS2::from_bits(val as u8)
    }
    #[doc = "Secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn set_suw_s2(&mut self, val: super::vals::SuwS2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure supervisor write access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn ssw_s2(&self) -> super::vals::SswS2 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SswS2::from_bits(val as u8)
    }
    #[doc = "Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn set_ssw_s2(&mut self, val: super::vals::SswS2) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure user write access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw_s2(&self) -> super::vals::NuwS2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::NuwS2::from_bits(val as u8)
    }
    #[doc = "Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn set_nuw_s2(&mut self, val: super::vals::NuwS2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Non-secure supervisor write access control for the second slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nsw_s2(&self) -> super::vals::NswS2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::NswS2::from_bits(val as u8)
    }
    #[doc = "Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn set_nsw_s2(&mut self, val: super::vals::NswS2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "The lock bit corresponding to the second slave. It is written by the secure software."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s2(&self) -> super::vals::LockS2 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LockS2::from_bits(val as u8)
    }
    #[doc = "The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn set_lock_s2(&mut self, val: super::vals::LockS2) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure user read access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn sur_s1(&self) -> super::vals::SurS1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SurS1::from_bits(val as u8)
    }
    #[doc = "Secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn set_sur_s1(&mut self, val: super::vals::SurS1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure supervisor read access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn ssr_s1(&self) -> super::vals::SsrS1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SsrS1::from_bits(val as u8)
    }
    #[doc = "Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn set_ssr_s1(&mut self, val: super::vals::SsrS1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Non-secure user read access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nur_s1(&self) -> super::vals::NurS1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::NurS1::from_bits(val as u8)
    }
    #[doc = "Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn set_nur_s1(&mut self, val: super::vals::NurS1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Non-secure supervisor read access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nsr_s1(&self) -> super::vals::NsrS1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::NsrS1::from_bits(val as u8)
    }
    #[doc = "Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn set_nsr_s1(&mut self, val: super::vals::NsrS1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure user write access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn suw_s1(&self) -> super::vals::SuwS1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::SuwS1::from_bits(val as u8)
    }
    #[doc = "Secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn set_suw_s1(&mut self, val: super::vals::SuwS1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure supervisor write access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn ssw_s1(&self) -> super::vals::SswS1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SswS1::from_bits(val as u8)
    }
    #[doc = "Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn set_ssw_s1(&mut self, val: super::vals::SswS1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Non-secure user write access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw_s1(&self) -> super::vals::NuwS1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::NuwS1::from_bits(val as u8)
    }
    #[doc = "Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn set_nuw_s1(&mut self, val: super::vals::NuwS1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Non-secure supervisor write access control for the first slave"]
    #[must_use]
    #[inline(always)]
    pub const fn nsw_s1(&self) -> super::vals::NswS1 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::NswS1::from_bits(val as u8)
    }
    #[doc = "Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn set_nsw_s1(&mut self, val: super::vals::NswS1) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "The lock bit corresponding to the first slave. It is written by the secure software."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s1(&self) -> super::vals::LockS1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::LockS1::from_bits(val as u8)
    }
    #[doc = "The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn set_lock_s1(&mut self, val: super::vals::LockS1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Csl {
    #[inline(always)]
    fn default() -> Csl {
        Csl(0)
    }
}
impl core::fmt::Debug for Csl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csl")
            .field("sur_s2", &self.sur_s2())
            .field("ssr_s2", &self.ssr_s2())
            .field("nur_s2", &self.nur_s2())
            .field("nsr_s2", &self.nsr_s2())
            .field("suw_s2", &self.suw_s2())
            .field("ssw_s2", &self.ssw_s2())
            .field("nuw_s2", &self.nuw_s2())
            .field("nsw_s2", &self.nsw_s2())
            .field("lock_s2", &self.lock_s2())
            .field("sur_s1", &self.sur_s1())
            .field("ssr_s1", &self.ssr_s1())
            .field("nur_s1", &self.nur_s1())
            .field("nsr_s1", &self.nsr_s1())
            .field("suw_s1", &self.suw_s1())
            .field("ssw_s1", &self.ssw_s1())
            .field("nuw_s1", &self.nuw_s1())
            .field("nsw_s1", &self.nsw_s1())
            .field("lock_s1", &self.lock_s1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csl {{ sur_s2: {:?}, ssr_s2: {:?}, nur_s2: {:?}, nsr_s2: {:?}, suw_s2: {:?}, ssw_s2: {:?}, nuw_s2: {:?}, nsw_s2: {:?}, lock_s2: {:?}, sur_s1: {:?}, ssr_s1: {:?}, nur_s1: {:?}, nsr_s1: {:?}, suw_s1: {:?}, ssw_s1: {:?}, nuw_s1: {:?}, nsw_s1: {:?}, lock_s1: {:?} }}",
            self.sur_s2(),
            self.ssr_s2(),
            self.nur_s2(),
            self.nsr_s2(),
            self.suw_s2(),
            self.ssw_s2(),
            self.nuw_s2(),
            self.nsw_s2(),
            self.lock_s2(),
            self.sur_s1(),
            self.ssr_s1(),
            self.nur_s1(),
            self.nsr_s1(),
            self.suw_s1(),
            self.ssw_s1(),
            self.nuw_s1(),
            self.nsw_s1(),
            self.lock_s1()
        )
    }
}
#[doc = "HP0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hp0(pub u32);
impl Hp0 {
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the eDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_dma(&self) -> super::vals::HpDma {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::HpDma::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the eDMA"]
    #[inline(always)]
    pub const fn set_hp_dma(&mut self, val: super::vals::HpDma) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dma(&self) -> super::vals::Hp0LDma {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hp0LDma::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: super::vals::Hp0LDma) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_lcdif(&self) -> super::vals::HpLcdif {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HpLcdif::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the LCDIF"]
    #[inline(always)]
    pub const fn set_hp_lcdif(&mut self, val: super::vals::HpLcdif) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> super::vals::Hp0LLcdif {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hp0LLcdif::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: super::vals::Hp0LLcdif) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the CSI"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_csi(&self) -> super::vals::HpCsi {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::HpCsi::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the CSI"]
    #[inline(always)]
    pub const fn set_hp_csi(&mut self, val: super::vals::HpCsi) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[must_use]
    #[inline(always)]
    pub const fn l_csi(&self) -> super::vals::Hp0LCsi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hp0LCsi::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: super::vals::Hp0LCsi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the PXP"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_pxp(&self) -> super::vals::HpPxp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::HpPxp::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the PXP"]
    #[inline(always)]
    pub const fn set_hp_pxp(&mut self, val: super::vals::HpPxp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_pxp(&self) -> super::vals::Hp0LPxp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Hp0LPxp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: super::vals::Hp0LPxp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the DCP"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_dcp(&self) -> super::vals::HpDcp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::HpDcp::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the DCP"]
    #[inline(always)]
    pub const fn set_hp_dcp(&mut self, val: super::vals::HpDcp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dcp(&self) -> super::vals::Hp0LDcp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hp0LDcp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: super::vals::Hp0LDcp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_enet(&self) -> super::vals::HpEnet {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::HpEnet::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET"]
    #[inline(always)]
    pub const fn set_hp_enet(&mut self, val: super::vals::HpEnet) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet(&self) -> super::vals::Hp0LEnet {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Hp0LEnet::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: super::vals::Hp0LEnet) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC1"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_usdhc1(&self) -> super::vals::HpUsdhc1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HpUsdhc1::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC1"]
    #[inline(always)]
    pub const fn set_hp_usdhc1(&mut self, val: super::vals::HpUsdhc1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> super::vals::Hp0LUsdhc1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Hp0LUsdhc1::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: super::vals::Hp0LUsdhc1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC2"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_usdhc2(&self) -> super::vals::HpUsdhc2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HpUsdhc2::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC2"]
    #[inline(always)]
    pub const fn set_hp_usdhc2(&mut self, val: super::vals::HpUsdhc2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> super::vals::Hp0LUsdhc2 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Hp0LUsdhc2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: super::vals::Hp0LUsdhc2) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the TPSMP"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_tpsmp(&self) -> super::vals::HpTpsmp {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::HpTpsmp::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the TPSMP"]
    #[inline(always)]
    pub const fn set_hp_tpsmp(&mut self, val: super::vals::HpTpsmp) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> super::vals::Hp0LTpsmp {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Hp0LTpsmp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: super::vals::Hp0LTpsmp) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USB"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_usb(&self) -> super::vals::HpUsb {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::HpUsb::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USB"]
    #[inline(always)]
    pub const fn set_hp_usb(&mut self, val: super::vals::HpUsb) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usb(&self) -> super::vals::Hp0LUsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Hp0LUsb::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: super::vals::Hp0LUsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET2"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_enet2(&self) -> super::vals::HpEnet2 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::HpEnet2::from_bits(val as u8)
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET2"]
    #[inline(always)]
    pub const fn set_hp_enet2(&mut self, val: super::vals::HpEnet2) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet2(&self) -> super::vals::Hp0LEnet2 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Hp0LEnet2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[inline(always)]
    pub const fn set_l_enet2(&mut self, val: super::vals::Hp0LEnet2) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Hp0 {
    #[inline(always)]
    fn default() -> Hp0 {
        Hp0(0)
    }
}
impl core::fmt::Debug for Hp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hp0")
            .field("hp_dma", &self.hp_dma())
            .field("l_dma", &self.l_dma())
            .field("hp_lcdif", &self.hp_lcdif())
            .field("l_lcdif", &self.l_lcdif())
            .field("hp_csi", &self.hp_csi())
            .field("l_csi", &self.l_csi())
            .field("hp_pxp", &self.hp_pxp())
            .field("l_pxp", &self.l_pxp())
            .field("hp_dcp", &self.hp_dcp())
            .field("l_dcp", &self.l_dcp())
            .field("hp_enet", &self.hp_enet())
            .field("l_enet", &self.l_enet())
            .field("hp_usdhc1", &self.hp_usdhc1())
            .field("l_usdhc1", &self.l_usdhc1())
            .field("hp_usdhc2", &self.hp_usdhc2())
            .field("l_usdhc2", &self.l_usdhc2())
            .field("hp_tpsmp", &self.hp_tpsmp())
            .field("l_tpsmp", &self.l_tpsmp())
            .field("hp_usb", &self.hp_usb())
            .field("l_usb", &self.l_usb())
            .field("hp_enet2", &self.hp_enet2())
            .field("l_enet2", &self.l_enet2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hp0 {{ hp_dma: {:?}, l_dma: {:?}, hp_lcdif: {:?}, l_lcdif: {:?}, hp_csi: {:?}, l_csi: {:?}, hp_pxp: {:?}, l_pxp: {:?}, hp_dcp: {:?}, l_dcp: {:?}, hp_enet: {:?}, l_enet: {:?}, hp_usdhc1: {:?}, l_usdhc1: {:?}, hp_usdhc2: {:?}, l_usdhc2: {:?}, hp_tpsmp: {:?}, l_tpsmp: {:?}, hp_usb: {:?}, l_usb: {:?}, hp_enet2: {:?}, l_enet2: {:?} }}",
            self.hp_dma(),
            self.l_dma(),
            self.hp_lcdif(),
            self.l_lcdif(),
            self.hp_csi(),
            self.l_csi(),
            self.hp_pxp(),
            self.l_pxp(),
            self.hp_dcp(),
            self.l_dcp(),
            self.hp_enet(),
            self.l_enet(),
            self.hp_usdhc1(),
            self.l_usdhc1(),
            self.hp_usdhc2(),
            self.l_usdhc2(),
            self.hp_tpsmp(),
            self.l_tpsmp(),
            self.hp_usb(),
            self.l_usb(),
            self.hp_enet2(),
            self.l_enet2()
        )
    }
}
#[doc = "HPCONTROL0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpcontrol0(pub u32);
impl Hpcontrol0 {
    #[doc = "Indicates the privilege/user mode for the eDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_dma(&self) -> super::vals::HpcDma {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::HpcDma::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the eDMA"]
    #[inline(always)]
    pub const fn set_hpc_dma(&mut self, val: super::vals::HpcDma) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dma(&self) -> super::vals::Hpcontrol0LDma {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hpcontrol0LDma::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: super::vals::Hpcontrol0LDma) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the privilege/user mode for the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_lcdif(&self) -> super::vals::HpcLcdif {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HpcLcdif::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the LCDIF"]
    #[inline(always)]
    pub const fn set_hpc_lcdif(&mut self, val: super::vals::HpcLcdif) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> super::vals::Hpcontrol0LLcdif {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hpcontrol0LLcdif::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: super::vals::Hpcontrol0LLcdif) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the privilege/user mode for the CSI"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_csi(&self) -> super::vals::HpcCsi {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::HpcCsi::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the CSI"]
    #[inline(always)]
    pub const fn set_hpc_csi(&mut self, val: super::vals::HpcCsi) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[must_use]
    #[inline(always)]
    pub const fn l_csi(&self) -> super::vals::Hpcontrol0LCsi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hpcontrol0LCsi::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: super::vals::Hpcontrol0LCsi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates the privilege/user mode for the PXP"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_pxp(&self) -> super::vals::HpcPxp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::HpcPxp::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the PXP"]
    #[inline(always)]
    pub const fn set_hpc_pxp(&mut self, val: super::vals::HpcPxp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_pxp(&self) -> super::vals::Hpcontrol0LPxp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Hpcontrol0LPxp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: super::vals::Hpcontrol0LPxp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates the privilege/user mode for the DCP"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_dcp(&self) -> super::vals::HpcDcp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::HpcDcp::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the DCP"]
    #[inline(always)]
    pub const fn set_hpc_dcp(&mut self, val: super::vals::HpcDcp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dcp(&self) -> super::vals::Hpcontrol0LDcp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hpcontrol0LDcp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: super::vals::Hpcontrol0LDcp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates the privilege/user mode for the ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_enet(&self) -> super::vals::HpcEnet {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::HpcEnet::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the ENET"]
    #[inline(always)]
    pub const fn set_hpc_enet(&mut self, val: super::vals::HpcEnet) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet(&self) -> super::vals::Hpcontrol0LEnet {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Hpcontrol0LEnet::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: super::vals::Hpcontrol0LEnet) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Indicates the privilege/user mode for the USDHC1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_usdhc1(&self) -> super::vals::HpcUsdhc1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HpcUsdhc1::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the USDHC1"]
    #[inline(always)]
    pub const fn set_hpc_usdhc1(&mut self, val: super::vals::HpcUsdhc1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> super::vals::Hpcontrol0LUsdhc1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Hpcontrol0LUsdhc1::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: super::vals::Hpcontrol0LUsdhc1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Indicates the privilege/user mode for the USDHC2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_usdhc2(&self) -> super::vals::HpcUsdhc2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HpcUsdhc2::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the USDHC2"]
    #[inline(always)]
    pub const fn set_hpc_usdhc2(&mut self, val: super::vals::HpcUsdhc2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2."]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> super::vals::Hpcontrol0LUsdhc2 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Hpcontrol0LUsdhc2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2."]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: super::vals::Hpcontrol0LUsdhc2) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Indicates the privilege/user mode for the TPSMP"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_tpsmp(&self) -> super::vals::HpcTpsmp {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::HpcTpsmp::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the TPSMP"]
    #[inline(always)]
    pub const fn set_hpc_tpsmp(&mut self, val: super::vals::HpcTpsmp) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP."]
    #[must_use]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> super::vals::Hpcontrol0LTpsmp {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Hpcontrol0LTpsmp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP."]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: super::vals::Hpcontrol0LTpsmp) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Indicates the privilege/user mode for the USB"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_usb(&self) -> super::vals::HpcUsb {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::HpcUsb::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the USB"]
    #[inline(always)]
    pub const fn set_hpc_usb(&mut self, val: super::vals::HpcUsb) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB."]
    #[must_use]
    #[inline(always)]
    pub const fn l_usb(&self) -> super::vals::Hpcontrol0LUsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Hpcontrol0LUsb::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USB."]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: super::vals::Hpcontrol0LUsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates the privilege/user mode for the ENET2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpc_enet2(&self) -> super::vals::HpcEnet2 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::HpcEnet2::from_bits(val as u8)
    }
    #[doc = "Indicates the privilege/user mode for the ENET2"]
    #[inline(always)]
    pub const fn set_hpc_enet2(&mut self, val: super::vals::HpcEnet2) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet2(&self) -> super::vals::Hpcontrol0LEnet2 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Hpcontrol0LEnet2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[inline(always)]
    pub const fn set_l_enet2(&mut self, val: super::vals::Hpcontrol0LEnet2) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Hpcontrol0 {
    #[inline(always)]
    fn default() -> Hpcontrol0 {
        Hpcontrol0(0)
    }
}
impl core::fmt::Debug for Hpcontrol0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpcontrol0")
            .field("hpc_dma", &self.hpc_dma())
            .field("l_dma", &self.l_dma())
            .field("hpc_lcdif", &self.hpc_lcdif())
            .field("l_lcdif", &self.l_lcdif())
            .field("hpc_csi", &self.hpc_csi())
            .field("l_csi", &self.l_csi())
            .field("hpc_pxp", &self.hpc_pxp())
            .field("l_pxp", &self.l_pxp())
            .field("hpc_dcp", &self.hpc_dcp())
            .field("l_dcp", &self.l_dcp())
            .field("hpc_enet", &self.hpc_enet())
            .field("l_enet", &self.l_enet())
            .field("hpc_usdhc1", &self.hpc_usdhc1())
            .field("l_usdhc1", &self.l_usdhc1())
            .field("hpc_usdhc2", &self.hpc_usdhc2())
            .field("l_usdhc2", &self.l_usdhc2())
            .field("hpc_tpsmp", &self.hpc_tpsmp())
            .field("l_tpsmp", &self.l_tpsmp())
            .field("hpc_usb", &self.hpc_usb())
            .field("l_usb", &self.l_usb())
            .field("hpc_enet2", &self.hpc_enet2())
            .field("l_enet2", &self.l_enet2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpcontrol0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpcontrol0 {{ hpc_dma: {:?}, l_dma: {:?}, hpc_lcdif: {:?}, l_lcdif: {:?}, hpc_csi: {:?}, l_csi: {:?}, hpc_pxp: {:?}, l_pxp: {:?}, hpc_dcp: {:?}, l_dcp: {:?}, hpc_enet: {:?}, l_enet: {:?}, hpc_usdhc1: {:?}, l_usdhc1: {:?}, hpc_usdhc2: {:?}, l_usdhc2: {:?}, hpc_tpsmp: {:?}, l_tpsmp: {:?}, hpc_usb: {:?}, l_usb: {:?}, hpc_enet2: {:?}, l_enet2: {:?} }}",
            self.hpc_dma(),
            self.l_dma(),
            self.hpc_lcdif(),
            self.l_lcdif(),
            self.hpc_csi(),
            self.l_csi(),
            self.hpc_pxp(),
            self.l_pxp(),
            self.hpc_dcp(),
            self.l_dcp(),
            self.hpc_enet(),
            self.l_enet(),
            self.hpc_usdhc1(),
            self.l_usdhc1(),
            self.hpc_usdhc2(),
            self.l_usdhc2(),
            self.hpc_tpsmp(),
            self.l_tpsmp(),
            self.hpc_usb(),
            self.l_usb(),
            self.hpc_enet2(),
            self.l_enet2()
        )
    }
}
#[doc = "Secure access register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sa(pub u32);
impl Sa {
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_dma(&self) -> super::vals::NsaDma {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::NsaDma::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_dma(&mut self, val: super::vals::NsaDma) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dma(&self) -> super::vals::SaLDma {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SaLDma::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: super::vals::SaLDma) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_lcdif(&self) -> super::vals::NsaLcdif {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::NsaLcdif::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_lcdif(&mut self, val: super::vals::NsaLcdif) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> super::vals::SaLLcdif {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SaLLcdif::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: super::vals::SaLLcdif) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_csi(&self) -> super::vals::NsaCsi {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::NsaCsi::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_csi(&mut self, val: super::vals::NsaCsi) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[must_use]
    #[inline(always)]
    pub const fn l_csi(&self) -> super::vals::SaLCsi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SaLCsi::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: super::vals::SaLCsi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Non-Secure Access Policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_pxp(&self) -> super::vals::NsaPxp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::NsaPxp::from_bits(val as u8)
    }
    #[doc = "Non-Secure Access Policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_pxp(&mut self, val: super::vals::NsaPxp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_pxp(&self) -> super::vals::SaLPxp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SaLPxp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: super::vals::SaLPxp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_dcp(&self) -> super::vals::NsaDcp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::NsaDcp::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_dcp(&mut self, val: super::vals::NsaDcp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_dcp(&self) -> super::vals::SaLDcp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SaLDcp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: super::vals::SaLDcp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_enet(&self) -> super::vals::NsaEnet {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::NsaEnet::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_enet(&mut self, val: super::vals::NsaEnet) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet(&self) -> super::vals::SaLEnet {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SaLEnet::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: super::vals::SaLEnet) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_usdhc1(&self) -> super::vals::NsaUsdhc1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::NsaUsdhc1::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usdhc1(&mut self, val: super::vals::NsaUsdhc1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> super::vals::SaLUsdhc1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SaLUsdhc1::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: super::vals::SaLUsdhc1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_usdhc2(&self) -> super::vals::NsaUsdhc2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::NsaUsdhc2::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usdhc2(&mut self, val: super::vals::NsaUsdhc2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> super::vals::SaLUsdhc2 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::SaLUsdhc2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: super::vals::SaLUsdhc2) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_tpsmp(&self) -> super::vals::NsaTpsmp {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::NsaTpsmp::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_tpsmp(&mut self, val: super::vals::NsaTpsmp) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[must_use]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> super::vals::SaLTpsmp {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SaLTpsmp::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: super::vals::SaLTpsmp) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_usb(&self) -> super::vals::NsaUsb {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::NsaUsb::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usb(&mut self, val: super::vals::NsaUsb) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[must_use]
    #[inline(always)]
    pub const fn l_usb(&self) -> super::vals::SaLUsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SaLUsb::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: super::vals::SaLUsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nsa_enet2(&self) -> super::vals::NsaEnet2 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::NsaEnet2::from_bits(val as u8)
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_enet2(&mut self, val: super::vals::NsaEnet2) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[must_use]
    #[inline(always)]
    pub const fn l_enet2(&self) -> super::vals::SaLEnet2 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SaLEnet2::from_bits(val as u8)
    }
    #[doc = "Lock bit set by the TZ software for the ENET2"]
    #[inline(always)]
    pub const fn set_l_enet2(&mut self, val: super::vals::SaLEnet2) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Sa {
    #[inline(always)]
    fn default() -> Sa {
        Sa(0)
    }
}
impl core::fmt::Debug for Sa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sa")
            .field("nsa_dma", &self.nsa_dma())
            .field("l_dma", &self.l_dma())
            .field("nsa_lcdif", &self.nsa_lcdif())
            .field("l_lcdif", &self.l_lcdif())
            .field("nsa_csi", &self.nsa_csi())
            .field("l_csi", &self.l_csi())
            .field("nsa_pxp", &self.nsa_pxp())
            .field("l_pxp", &self.l_pxp())
            .field("nsa_dcp", &self.nsa_dcp())
            .field("l_dcp", &self.l_dcp())
            .field("nsa_enet", &self.nsa_enet())
            .field("l_enet", &self.l_enet())
            .field("nsa_usdhc1", &self.nsa_usdhc1())
            .field("l_usdhc1", &self.l_usdhc1())
            .field("nsa_usdhc2", &self.nsa_usdhc2())
            .field("l_usdhc2", &self.l_usdhc2())
            .field("nsa_tpsmp", &self.nsa_tpsmp())
            .field("l_tpsmp", &self.l_tpsmp())
            .field("nsa_usb", &self.nsa_usb())
            .field("l_usb", &self.l_usb())
            .field("nsa_enet2", &self.nsa_enet2())
            .field("l_enet2", &self.l_enet2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sa {{ nsa_dma: {:?}, l_dma: {:?}, nsa_lcdif: {:?}, l_lcdif: {:?}, nsa_csi: {:?}, l_csi: {:?}, nsa_pxp: {:?}, l_pxp: {:?}, nsa_dcp: {:?}, l_dcp: {:?}, nsa_enet: {:?}, l_enet: {:?}, nsa_usdhc1: {:?}, l_usdhc1: {:?}, nsa_usdhc2: {:?}, l_usdhc2: {:?}, nsa_tpsmp: {:?}, l_tpsmp: {:?}, nsa_usb: {:?}, l_usb: {:?}, nsa_enet2: {:?}, l_enet2: {:?} }}",
            self.nsa_dma(),
            self.l_dma(),
            self.nsa_lcdif(),
            self.l_lcdif(),
            self.nsa_csi(),
            self.l_csi(),
            self.nsa_pxp(),
            self.l_pxp(),
            self.nsa_dcp(),
            self.l_dcp(),
            self.nsa_enet(),
            self.l_enet(),
            self.nsa_usdhc1(),
            self.l_usdhc1(),
            self.nsa_usdhc2(),
            self.l_usdhc2(),
            self.nsa_tpsmp(),
            self.l_tpsmp(),
            self.nsa_usb(),
            self.l_usb(),
            self.nsa_enet2(),
            self.l_enet2()
        )
    }
}
