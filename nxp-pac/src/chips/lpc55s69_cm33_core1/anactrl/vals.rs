#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashPwrdwn {
    #[doc = "Flash is not in power down mode."]
    PWRUP = 0x0,
    #[doc = "Flash is in power down mode."]
    PWRDWN = 0x01,
}
impl FlashPwrdwn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashPwrdwn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashPwrdwn {
    #[inline(always)]
    fn from(val: u8) -> FlashPwrdwn {
        FlashPwrdwn::from_bits(val)
    }
}
impl From<FlashPwrdwn> for u8 {
    #[inline(always)]
    fn from(val: FlashPwrdwn) -> u8 {
        FlashPwrdwn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro192mTrimSrc {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE = 0x0,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL = 0x01,
}
impl Fro192mTrimSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro192mTrimSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro192mTrimSrc {
    #[inline(always)]
    fn from(val: u8) -> Fro192mTrimSrc {
        Fro192mTrimSrc::from_bits(val)
    }
}
impl From<Fro192mTrimSrc> for u8 {
    #[inline(always)]
    fn from(val: Fro192mTrimSrc) -> u8 {
        Fro192mTrimSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Highz {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE = 0x0,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE = 0x01,
}
impl Highz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Highz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Highz {
    #[inline(always)]
    fn from(val: u8) -> Highz {
        Highz::from_bits(val)
    }
}
impl From<Highz> for u8 {
    #[inline(always)]
    fn from(val: Highz) -> u8 {
        Highz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo0CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo0CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo0CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo0CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo0CtrlFs {
        Ringo0CtrlFs::from_bits(val)
    }
}
impl From<Ringo0CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo0CtrlFs) -> u8 {
        Ringo0CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo0CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo0CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo0CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo0CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo0CtrlPd {
        Ringo0CtrlPd::from_bits(val)
    }
}
impl From<Ringo0CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo0CtrlPd) -> u8 {
        Ringo0CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo1CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlFs {
        Ringo1CtrlFs::from_bits(val)
    }
}
impl From<Ringo1CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlFs) -> u8 {
        Ringo1CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo1CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlPd {
        Ringo1CtrlPd::from_bits(val)
    }
}
impl From<Ringo1CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlPd) -> u8 {
        Ringo1CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlS {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Ringo1CtrlS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlS {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlS {
        Ringo1CtrlS::from_bits(val)
    }
}
impl From<Ringo1CtrlS> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlS) -> u8 {
        Ringo1CtrlS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo2CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlFs {
        Ringo2CtrlFs::from_bits(val)
    }
}
impl From<Ringo2CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlFs) -> u8 {
        Ringo2CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo2CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlPd {
        Ringo2CtrlPd::from_bits(val)
    }
}
impl From<Ringo2CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlPd) -> u8 {
        Ringo2CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlS {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Ringo2CtrlS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlS {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlS {
        Ringo2CtrlS::from_bits(val)
    }
}
impl From<Ringo2CtrlS> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlS) -> u8 {
        Ringo2CtrlS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sl {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Sl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sl {
    #[inline(always)]
    fn from(val: u8) -> Sl {
        Sl::from_bits(val)
    }
}
impl From<Sl> for u8 {
    #[inline(always)]
    fn from(val: Sl) -> u8 {
        Sl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwnSwp {
    #[doc = "Normal mode."]
    NORMAL = 0x0,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR = 0x01,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR = 0x02,
    #[doc = "Don't use."]
    FORBIDDEN = 0x03,
}
impl SwnSwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwnSwp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwnSwp {
    #[inline(always)]
    fn from(val: u8) -> SwnSwp {
        SwnSwp::from_bits(val)
    }
}
impl From<SwnSwp> for u8 {
    #[inline(always)]
    fn from(val: SwnSwp) -> u8 {
        SwnSwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout {
    #[doc = "0.750 V."]
    V_0P750 = 0x0,
    #[doc = "0.775 V."]
    V_0P775 = 0x01,
    #[doc = "0.800 V."]
    V_0P800 = 0x02,
    #[doc = "0.825 V."]
    V_0P825 = 0x03,
    #[doc = "0.850 V."]
    V_0P850 = 0x04,
    #[doc = "0.875 V."]
    V_0P875 = 0x05,
    #[doc = "0.900 V."]
    V_0P900 = 0x06,
    #[doc = "0.925 V."]
    V_0P925 = 0x07,
}
impl Vout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout {
    #[inline(always)]
    fn from(val: u8) -> Vout {
        Vout::from_bits(val)
    }
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(val: Vout) -> u8 {
        Vout::to_bits(val)
    }
}
