#[doc = "USB OTG Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Disable Overcurrent Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_dis(&self) -> super::vals::OverCurDis {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OverCurDis::from_bits(val as u8)
    }
    #[doc = "Disable Overcurrent Detection"]
    #[inline(always)]
    pub const fn set_over_cur_dis(&mut self, val: super::vals::OverCurDis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Polarity of Overcurrent"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_pol(&self) -> super::vals::OverCurPol {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OverCurPol::from_bits(val as u8)
    }
    #[doc = "Polarity of Overcurrent"]
    #[inline(always)]
    pub const fn set_over_cur_pol(&mut self, val: super::vals::OverCurPol) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Power Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_pol(&self) -> super::vals::PwrPol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PwrPol::from_bits(val as u8)
    }
    #[doc = "Power Polarity"]
    #[inline(always)]
    pub const fn set_pwr_pol(&mut self, val: super::vals::PwrPol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> super::vals::Wie {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wie::from_bits(val as u8)
    }
    #[doc = "Wake-up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: super::vals::Wie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Wake-up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw_en(&self) -> super::vals::WkupSwEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::WkupSwEn::from_bits(val as u8)
    }
    #[doc = "Software Wake-up Enable"]
    #[inline(always)]
    pub const fn set_wkup_sw_en(&mut self, val: super::vals::WkupSwEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software Wake-up"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw(&self) -> super::vals::WkupSw {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::WkupSw::from_bits(val as u8)
    }
    #[doc = "Software Wake-up"]
    #[inline(always)]
    pub const fn set_wkup_sw(&mut self, val: super::vals::WkupSw) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-up on ID Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_id_en(&self) -> super::vals::WkupIdEn {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::WkupIdEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on ID Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_id_en(&mut self, val: super::vals::WkupIdEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up on VBUS Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_vbus_en(&self) -> super::vals::WkupVbusEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::WkupVbusEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on VBUS Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_vbus_en(&mut self, val: super::vals::WkupVbusEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up on DPDM Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_dpdm_en(&self) -> super::vals::WkupDpdmEn {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::WkupDpdmEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on DPDM Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_dpdm_en(&mut self, val: super::vals::WkupDpdmEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-up Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn wir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Interrupt Request"]
    #[inline(always)]
    pub const fn set_wir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("over_cur_dis", &self.over_cur_dis())
            .field("over_cur_pol", &self.over_cur_pol())
            .field("pwr_pol", &self.pwr_pol())
            .field("wie", &self.wie())
            .field("wkup_sw_en", &self.wkup_sw_en())
            .field("wkup_sw", &self.wkup_sw())
            .field("wkup_id_en", &self.wkup_id_en())
            .field("wkup_vbus_en", &self.wkup_vbus_en())
            .field("wkup_dpdm_en", &self.wkup_dpdm_en())
            .field("wir", &self.wir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ over_cur_dis: {:?}, over_cur_pol: {:?}, pwr_pol: {:?}, wie: {:?}, wkup_sw_en: {:?}, wkup_sw: {:?}, wkup_id_en: {:?}, wkup_vbus_en: {:?}, wkup_dpdm_en: {:?}, wir: {=bool:?} }}",
            self.over_cur_dis(),
            self.over_cur_pol(),
            self.pwr_pol(),
            self.wie(),
            self.wkup_sw_en(),
            self.wkup_sw(),
            self.wkup_id_en(),
            self.wkup_vbus_en(),
            self.wkup_dpdm_en(),
            self.wir()
        )
    }
}
#[doc = "USB OTG Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "VBUS Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::VbusSourceSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::VbusSourceSel::from_bits(val as u8)
    }
    #[doc = "VBUS Source Select"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::VbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Auto Resume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn auturesume_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Resume Enable"]
    #[inline(always)]
    pub const fn set_auturesume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low Speed Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lowspeed_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low Speed Enable"]
    #[inline(always)]
    pub const fn set_lowspeed_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UTMI Clock Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Valid"]
    #[inline(always)]
    pub const fn set_utmi_clk_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("auturesume_en", &self.auturesume_en())
            .field("lowspeed_en", &self.lowspeed_en())
            .field("utmi_clk_vld", &self.utmi_clk_vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ vbus_source_sel: {:?}, auturesume_en: {=bool:?}, lowspeed_en: {=bool:?}, utmi_clk_vld: {=bool:?} }}",
            self.vbus_source_sel(),
            self.auturesume_en(),
            self.lowspeed_en(),
            self.utmi_clk_vld()
        )
    }
}
#[doc = "USB Host HSIC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HsicCtrl(pub u32);
impl HsicCtrl {
    #[doc = "HSIC Clock ON"]
    #[must_use]
    #[inline(always)]
    pub const fn hsic_clk_on(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HSIC Clock ON"]
    #[inline(always)]
    pub const fn set_hsic_clk_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host HSIC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsic_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host HSIC Enable"]
    #[inline(always)]
    pub const fn set_hsic_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clock Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Valid"]
    #[inline(always)]
    pub const fn set_clk_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HsicCtrl {
    #[inline(always)]
    fn default() -> HsicCtrl {
        HsicCtrl(0)
    }
}
impl core::fmt::Debug for HsicCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HsicCtrl")
            .field("hsic_clk_on", &self.hsic_clk_on())
            .field("hsic_en", &self.hsic_en())
            .field("clk_vld", &self.clk_vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HsicCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HsicCtrl {{ hsic_clk_on: {=bool:?}, hsic_en: {=bool:?}, clk_vld: {=bool:?} }}",
            self.hsic_clk_on(),
            self.hsic_en(),
            self.clk_vld()
        )
    }
}
