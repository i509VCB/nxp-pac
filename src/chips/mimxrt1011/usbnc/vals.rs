#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurDis {
    #[doc = "Enables overcurrent detection"]
    OVER_CUR_DIS_0 = 0x0,
    #[doc = "Disables overcurrent detection"]
    OVER_CUR_DIS_1 = 0x01,
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
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_0 = 0x0,
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_1 = 0x01,
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
    PWR_POL_0 = 0x0,
    #[doc = "PMIC Power Pin is High active."]
    PWR_POL_1 = 0x01,
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
pub enum UtmiClkVld {
    #[doc = "Invalid"]
    UTMI_CLK_VLD_0 = 0x0,
    #[doc = "Valid"]
    UTMI_CLK_VLD_1 = 0x01,
}
impl UtmiClkVld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtmiClkVld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtmiClkVld {
    #[inline(always)]
    fn from(val: u8) -> UtmiClkVld {
        UtmiClkVld::from_bits(val)
    }
}
impl From<UtmiClkVld> for u8 {
    #[inline(always)]
    fn from(val: UtmiClkVld) -> u8 {
        UtmiClkVld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wie {
    #[doc = "Interrupt Disabled"]
    WIE_0 = 0x0,
    #[doc = "Interrupt Enabled"]
    WIE_1 = 0x01,
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
pub enum Wir {
    #[doc = "No wake-up interrupt request received"]
    WIR_0 = 0x0,
    #[doc = "Wake-up Interrupt Request received"]
    WIR_1 = 0x01,
}
impl Wir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wir {
    #[inline(always)]
    fn from(val: u8) -> Wir {
        Wir::from_bits(val)
    }
}
impl From<Wir> for u8 {
    #[inline(always)]
    fn from(val: Wir) -> u8 {
        Wir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupDpdmEn {
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    WKUP_DPDM_EN_0 = 0x0,
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    WKUP_DPDM_EN_1 = 0x01,
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
    #[doc = "Disable"]
    WKUP_ID_EN_0 = 0x0,
    #[doc = "Enable"]
    WKUP_ID_EN_1 = 0x01,
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
    #[doc = "Inactive"]
    WKUP_SW_0 = 0x0,
    #[doc = "Force wake-up"]
    WKUP_SW_1 = 0x01,
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
    #[doc = "Disable"]
    WKUP_SW_EN_0 = 0x0,
    #[doc = "Enable"]
    WKUP_SW_EN_1 = 0x01,
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
    #[doc = "Disable"]
    WKUP_VBUS_EN_0 = 0x0,
    #[doc = "Enable"]
    WKUP_VBUS_EN_1 = 0x01,
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
