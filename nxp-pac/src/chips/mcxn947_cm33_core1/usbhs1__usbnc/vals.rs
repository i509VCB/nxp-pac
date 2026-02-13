#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurDis {
    #[doc = "Enables"]
    OVRCRNT_DETCT_EN = 0x0,
    #[doc = "Disables"]
    OVRCRNT_DETCT_DIS = 0x01,
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
    ACTIVE_HI_OVRCRNT = 0x0,
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    ACTIVE_LOW_OVRCRNT = 0x01,
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
    ACTIVE_LO_PMIC = 0x0,
    #[doc = "PMIC Power Pin is High active."]
    ACTIVE_HI_PMIC = 0x01,
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
    #[doc = "vbus_valid"]
    VBUS_VALID = 0x0,
    #[doc = "sess_valid"]
    SESS_VALID_1 = 0x01,
    #[doc = "sess_valid"]
    SESS_VALID_2 = 0x02,
    #[doc = "sess_valid"]
    SESS_VALID_3 = 0x03,
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
    #[doc = "Interrupt Disabled"]
    INT_DIS = 0x0,
    #[doc = "Interrupt Enabled"]
    INT_EN = 0x01,
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
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0"]
    DPDM_WKUP_DIS = 0x0,
    #[doc = "DPDM changes wake-up to be enabled, it is for device only"]
    DPDM_WKUP_EN = 0x01,
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
    #[doc = "Disables"]
    WKUP_ID_DIS = 0x0,
    #[doc = "Enables"]
    WKUP_ID_EN = 0x01,
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
    INACTIVE = 0x0,
    #[doc = "Force wake-up"]
    FORCE_WKUP = 0x01,
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
    #[doc = "Disables"]
    SW_WKUP_DIS = 0x0,
    #[doc = "Enables"]
    SW_WKUP_EN = 0x01,
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
    #[doc = "Disables"]
    WKUP_VBUS_DIS = 0x0,
    #[doc = "Enables"]
    WKUP_VBUS_EN = 0x01,
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
