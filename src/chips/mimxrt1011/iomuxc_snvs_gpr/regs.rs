#[doc = "GPR3 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr3(pub u32);
impl Gpr3 {
    #[doc = "Set to enable LPSR mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpsr_mode_enable(&self) -> super::vals::LpsrModeEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpsrModeEnable::from_bits(val as u8)
    }
    #[doc = "Set to enable LPSR mode."]
    #[inline(always)]
    pub const fn set_lpsr_mode_enable(&mut self, val: super::vals::LpsrModeEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DCDC captured status clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_status_capt_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC captured status clear"]
    #[inline(always)]
    pub const fn set_dcdc_status_capt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "POR_B pad control"]
    #[must_use]
    #[inline(always)]
    pub const fn por_pull_type(&self) -> super::vals::PorPullType {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PorPullType::from_bits(val as u8)
    }
    #[doc = "POR_B pad control"]
    #[inline(always)]
    pub const fn set_por_pull_type(&mut self, val: super::vals::PorPullType) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DCDC_IN low voltage detect."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_in_low_vol(&self) -> super::vals::DcdcInLowVol {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DcdcInLowVol::from_bits(val as u8)
    }
    #[doc = "DCDC_IN low voltage detect."]
    #[inline(always)]
    pub const fn set_dcdc_in_low_vol(&mut self, val: super::vals::DcdcInLowVol) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DCDC output over current alert"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_over_cur(&self) -> super::vals::DcdcOverCur {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DcdcOverCur::from_bits(val as u8)
    }
    #[doc = "DCDC output over current alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_cur(&mut self, val: super::vals::DcdcOverCur) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DCDC output over voltage alert"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_over_vol(&self) -> super::vals::DcdcOverVol {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DcdcOverVol::from_bits(val as u8)
    }
    #[doc = "DCDC output over voltage alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_vol(&mut self, val: super::vals::DcdcOverVol) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DCDC status OK"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_sts_dc_ok(&self) -> super::vals::DcdcStsDcOk {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DcdcStsDcOk::from_bits(val as u8)
    }
    #[doc = "DCDC status OK"]
    #[inline(always)]
    pub const fn set_dcdc_sts_dc_ok(&mut self, val: super::vals::DcdcStsDcOk) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Gpr3 {
    #[inline(always)]
    fn default() -> Gpr3 {
        Gpr3(0)
    }
}
impl core::fmt::Debug for Gpr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr3")
            .field("lpsr_mode_enable", &self.lpsr_mode_enable())
            .field("dcdc_status_capt_clr", &self.dcdc_status_capt_clr())
            .field("por_pull_type", &self.por_pull_type())
            .field("dcdc_in_low_vol", &self.dcdc_in_low_vol())
            .field("dcdc_over_cur", &self.dcdc_over_cur())
            .field("dcdc_over_vol", &self.dcdc_over_vol())
            .field("dcdc_sts_dc_ok", &self.dcdc_sts_dc_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr3 {{ lpsr_mode_enable: {:?}, dcdc_status_capt_clr: {=bool:?}, por_pull_type: {:?}, dcdc_in_low_vol: {:?}, dcdc_over_cur: {:?}, dcdc_over_vol: {:?}, dcdc_sts_dc_ok: {:?} }}",
            self.lpsr_mode_enable(),
            self.dcdc_status_capt_clr(),
            self.por_pull_type(),
            self.dcdc_in_low_vol(),
            self.dcdc_over_cur(),
            self.dcdc_over_vol(),
            self.dcdc_sts_dc_ok()
        )
    }
}
