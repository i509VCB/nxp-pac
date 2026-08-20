#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USBNC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs1Usbnc {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs1Usbnc {}
unsafe impl Sync for Usbhs1Usbnc {}
impl Usbhs1Usbnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB OTG Control 1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::pac::common::Reg<Ctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB OTG Control 2."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::pac::common::Reg<Ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB Host HSIC Control."]
    #[inline(always)]
    pub const fn hsic_ctrl(self) -> crate::pac::common::Reg<HsicCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "USB OTG Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Disable Overcurrent Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_dis(&self) -> OverCurDis {
        let val = (self.0 >> 7usize) & 0x01;
        OverCurDis::from_bits(val as u8)
    }
    #[doc = "Disable Overcurrent Detection."]
    #[inline(always)]
    pub const fn set_over_cur_dis(&mut self, val: OverCurDis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Polarity of Overcurrent."]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_pol(&self) -> OverCurPol {
        let val = (self.0 >> 8usize) & 0x01;
        OverCurPol::from_bits(val as u8)
    }
    #[doc = "Polarity of Overcurrent."]
    #[inline(always)]
    pub const fn set_over_cur_pol(&mut self, val: OverCurPol) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Power Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_pol(&self) -> PwrPol {
        let val = (self.0 >> 9usize) & 0x01;
        PwrPol::from_bits(val as u8)
    }
    #[doc = "Power Polarity."]
    #[inline(always)]
    pub const fn set_pwr_pol(&mut self, val: PwrPol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> Wie {
        let val = (self.0 >> 10usize) & 0x01;
        Wie::from_bits(val as u8)
    }
    #[doc = "Wake-up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: Wie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Wake-up Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw_en(&self) -> WkupSwEn {
        let val = (self.0 >> 14usize) & 0x01;
        WkupSwEn::from_bits(val as u8)
    }
    #[doc = "Software Wake-up Enable."]
    #[inline(always)]
    pub const fn set_wkup_sw_en(&mut self, val: WkupSwEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software Wake-up."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw(&self) -> WkupSw {
        let val = (self.0 >> 15usize) & 0x01;
        WkupSw::from_bits(val as u8)
    }
    #[doc = "Software Wake-up."]
    #[inline(always)]
    pub const fn set_wkup_sw(&mut self, val: WkupSw) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-up on ID Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_id_en(&self) -> WkupIdEn {
        let val = (self.0 >> 16usize) & 0x01;
        WkupIdEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on ID Change Enable."]
    #[inline(always)]
    pub const fn set_wkup_id_en(&mut self, val: WkupIdEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up on VBUS Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_vbus_en(&self) -> WkupVbusEn {
        let val = (self.0 >> 17usize) & 0x01;
        WkupVbusEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on VBUS Change Enable."]
    #[inline(always)]
    pub const fn set_wkup_vbus_en(&mut self, val: WkupVbusEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up on DPDM Change Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_dpdm_en(&self) -> WkupDpdmEn {
        let val = (self.0 >> 29usize) & 0x01;
        WkupDpdmEn::from_bits(val as u8)
    }
    #[doc = "Wake-up on DPDM Change Enable."]
    #[inline(always)]
    pub const fn set_wkup_dpdm_en(&mut self, val: WkupDpdmEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-up Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn wir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Interrupt Request."]
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
#[doc = "USB OTG Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "VBUS Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> VbusSourceSel {
        let val = (self.0 >> 0usize) & 0x03;
        VbusSourceSel::from_bits(val as u8)
    }
    #[doc = "VBUS Source Select."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: VbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Auto Resume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn auturesume_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Resume Enable."]
    #[inline(always)]
    pub const fn set_auturesume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low Speed Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lowspeed_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low Speed Enable."]
    #[inline(always)]
    pub const fn set_lowspeed_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UTMI Clock Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Valid."]
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
#[doc = "USB Host HSIC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HsicCtrl(pub u32);
impl HsicCtrl {
    #[doc = "HSIC Clock ON."]
    #[must_use]
    #[inline(always)]
    pub const fn hsic_clk_on(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HSIC Clock ON."]
    #[inline(always)]
    pub const fn set_hsic_clk_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host HSIC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsic_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host HSIC Enable."]
    #[inline(always)]
    pub const fn set_hsic_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clock Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Valid."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurDis {
    #[doc = "Enables."]
    OvrcrntDetctEn = 0x0,
    #[doc = "Disables."]
    OvrcrntDetctDis = 0x01,
}
impl OverCurDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverCurDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverCurDis {
    #[inline(always)]
    fn from(val: u8) -> OverCurDis {
        OverCurDis::from_bits(val)
    }
}
impl From<OverCurDis> for u8 {
    #[inline(always)]
    fn from(val: OverCurDis) -> u8 {
        OverCurDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurPol {
    #[doc = "High active (high on this signal represents an overcurrent condition)."]
    ActiveHiOvrcrnt = 0x0,
    #[doc = "Low active (low on this signal represents an overcurrent condition)."]
    ActiveLowOvrcrnt = 0x01,
}
impl OverCurPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverCurPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverCurPol {
    #[inline(always)]
    fn from(val: u8) -> OverCurPol {
        OverCurPol::from_bits(val)
    }
}
impl From<OverCurPol> for u8 {
    #[inline(always)]
    fn from(val: OverCurPol) -> u8 {
        OverCurPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrPol {
    #[doc = "PMIC Power Pin is Low active."]
    ActiveLoPmic = 0x0,
    #[doc = "PMIC Power Pin is High active."]
    ActiveHiPmic = 0x01,
}
impl PwrPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrPol {
    #[inline(always)]
    fn from(val: u8) -> PwrPol {
        PwrPol::from_bits(val)
    }
}
impl From<PwrPol> for u8 {
    #[inline(always)]
    fn from(val: PwrPol) -> u8 {
        PwrPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSourceSel {
    #[doc = "vbus_valid."]
    VbusValid = 0x0,
    #[doc = "sess_valid."]
    SessValid1 = 0x01,
    #[doc = "sess_valid."]
    SessValid2 = 0x02,
    #[doc = "sess_valid."]
    SessValid3 = 0x03,
}
impl VbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> VbusSourceSel {
        VbusSourceSel::from_bits(val)
    }
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: VbusSourceSel) -> u8 {
        VbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wie {
    #[doc = "Interrupt Disabled."]
    IntDis = 0x0,
    #[doc = "Interrupt Enabled."]
    IntEn = 0x01,
}
impl Wie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wie {
    #[inline(always)]
    fn from(val: u8) -> Wie {
        Wie::from_bits(val)
    }
}
impl From<Wie> for u8 {
    #[inline(always)]
    fn from(val: Wie) -> u8 {
        Wie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupDpdmEn {
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    DpdmWkupDis = 0x0,
    #[doc = "DPDM changes wake-up to be enabled, it is for device only."]
    DpdmWkupEn = 0x01,
}
impl WkupDpdmEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupDpdmEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupDpdmEn {
    #[inline(always)]
    fn from(val: u8) -> WkupDpdmEn {
        WkupDpdmEn::from_bits(val)
    }
}
impl From<WkupDpdmEn> for u8 {
    #[inline(always)]
    fn from(val: WkupDpdmEn) -> u8 {
        WkupDpdmEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupIdEn {
    #[doc = "Disables."]
    WkupIdDis = 0x0,
    #[doc = "Enables."]
    WkupIdEn = 0x01,
}
impl WkupIdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupIdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupIdEn {
    #[inline(always)]
    fn from(val: u8) -> WkupIdEn {
        WkupIdEn::from_bits(val)
    }
}
impl From<WkupIdEn> for u8 {
    #[inline(always)]
    fn from(val: WkupIdEn) -> u8 {
        WkupIdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupSw {
    #[doc = "Inactive."]
    Inactive = 0x0,
    #[doc = "Force wake-up."]
    ForceWkup = 0x01,
}
impl WkupSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupSw {
    #[inline(always)]
    fn from(val: u8) -> WkupSw {
        WkupSw::from_bits(val)
    }
}
impl From<WkupSw> for u8 {
    #[inline(always)]
    fn from(val: WkupSw) -> u8 {
        WkupSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupSwEn {
    #[doc = "Disables."]
    SwWkupDis = 0x0,
    #[doc = "Enables."]
    SwWkupEn = 0x01,
}
impl WkupSwEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupSwEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupSwEn {
    #[inline(always)]
    fn from(val: u8) -> WkupSwEn {
        WkupSwEn::from_bits(val)
    }
}
impl From<WkupSwEn> for u8 {
    #[inline(always)]
    fn from(val: WkupSwEn) -> u8 {
        WkupSwEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupVbusEn {
    #[doc = "Disables."]
    WkupVbusDis = 0x0,
    #[doc = "Enables."]
    WkupVbusEn = 0x01,
}
impl WkupVbusEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupVbusEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupVbusEn {
    #[inline(always)]
    fn from(val: u8) -> WkupVbusEn {
        WkupVbusEn::from_bits(val)
    }
}
impl From<WkupVbusEn> for u8 {
    #[inline(always)]
    fn from(val: WkupVbusEn) -> u8 {
        WkupVbusEn::to_bits(val)
    }
}
