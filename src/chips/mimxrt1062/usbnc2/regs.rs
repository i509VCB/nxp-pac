#[doc = "USB OTG2 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg2Ctrl(pub u32);
impl UsbOtg2Ctrl {
    #[doc = "Disable OTG2 Overcurrent Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_dis(&self) -> super::vals::OverCurDis {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OverCurDis::from_bits(val as u8)
    }
    #[doc = "Disable OTG2 Overcurrent Detection"]
    #[inline(always)]
    pub const fn set_over_cur_dis(&mut self, val: super::vals::OverCurDis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "OTG2 Polarity of Overcurrent The polarity of OTG2 port overcurrent event"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_pol(&self) -> super::vals::OverCurPol {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OverCurPol::from_bits(val as u8)
    }
    #[doc = "OTG2 Polarity of Overcurrent The polarity of OTG2 port overcurrent event"]
    #[inline(always)]
    pub const fn set_over_cur_pol(&mut self, val: super::vals::OverCurPol) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "OTG2 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_pol(&self) -> super::vals::PwrPol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PwrPol::from_bits(val as u8)
    }
    #[doc = "OTG2 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    pub const fn set_pwr_pol(&mut self, val: super::vals::PwrPol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "OTG2 Wake-up Interrupt Enable This bit enables or disables the OTG2 wake-up interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> super::vals::Wie {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wie::from_bits(val as u8)
    }
    #[doc = "OTG2 Wake-up Interrupt Enable This bit enables or disables the OTG2 wake-up interrupt"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: super::vals::Wie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "OTG2 Software Wake-up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw_en(&self) -> super::vals::WkupSwEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::WkupSwEn::from_bits(val as u8)
    }
    #[doc = "OTG2 Software Wake-up Enable"]
    #[inline(always)]
    pub const fn set_wkup_sw_en(&mut self, val: super::vals::WkupSwEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "OTG2 Software Wake-up"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw(&self) -> super::vals::WkupSw {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::WkupSw::from_bits(val as u8)
    }
    #[doc = "OTG2 Software Wake-up"]
    #[inline(always)]
    pub const fn set_wkup_sw(&mut self, val: super::vals::WkupSw) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "OTG2 Wake-up on ID change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_id_en(&self) -> super::vals::WkupIdEn {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::WkupIdEn::from_bits(val as u8)
    }
    #[doc = "OTG2 Wake-up on ID change enable"]
    #[inline(always)]
    pub const fn set_wkup_id_en(&mut self, val: super::vals::WkupIdEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTG2 wake-up on VBUS change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_vbus_en(&self) -> super::vals::WkupVbusEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::WkupVbusEn::from_bits(val as u8)
    }
    #[doc = "OTG2 wake-up on VBUS change enable"]
    #[inline(always)]
    pub const fn set_wkup_vbus_en(&mut self, val: super::vals::WkupVbusEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up on DPDM change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_dpdm_en(&self) -> super::vals::WkupDpdmEn {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::WkupDpdmEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on DPDM change enable"]
    #[inline(always)]
    pub const fn set_wkup_dpdm_en(&mut self, val: super::vals::WkupDpdmEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "OTG2 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG port"]
    #[must_use]
    #[inline(always)]
    pub const fn wir(&self) -> super::vals::Wir {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Wir::from_bits(val as u8)
    }
    #[doc = "OTG2 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG port"]
    #[inline(always)]
    pub const fn set_wir(&mut self, val: super::vals::Wir) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbOtg2Ctrl {
    #[inline(always)]
    fn default() -> UsbOtg2Ctrl {
        UsbOtg2Ctrl(0)
    }
}
impl core::fmt::Debug for UsbOtg2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg2Ctrl")
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
impl defmt::Format for UsbOtg2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbOtg2Ctrl {{ over_cur_dis: {:?}, over_cur_pol: {:?}, pwr_pol: {:?}, wie: {:?}, wkup_sw_en: {:?}, wkup_sw: {:?}, wkup_id_en: {:?}, wkup_vbus_en: {:?}, wkup_dpdm_en: {:?}, wir: {:?} }}",
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
#[doc = "OTG2 UTMI PHY Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg2PhyCtrl0(pub u32);
impl UsbOtg2PhyCtrl0 {
    #[doc = "Indicating whether OTG2 UTMI PHY clock is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_clk_vld(&self) -> super::vals::UtmiClkVld {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::UtmiClkVld::from_bits(val as u8)
    }
    #[doc = "Indicating whether OTG2 UTMI PHY clock is valid"]
    #[inline(always)]
    pub const fn set_utmi_clk_vld(&mut self, val: super::vals::UtmiClkVld) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbOtg2PhyCtrl0 {
    #[inline(always)]
    fn default() -> UsbOtg2PhyCtrl0 {
        UsbOtg2PhyCtrl0(0)
    }
}
impl core::fmt::Debug for UsbOtg2PhyCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg2PhyCtrl0")
            .field("utmi_clk_vld", &self.utmi_clk_vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtg2PhyCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbOtg2PhyCtrl0 {{ utmi_clk_vld: {:?} }}",
            self.utmi_clk_vld()
        )
    }
}
