#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl AdcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcRst {
    #[inline(always)]
    fn from(val: u8) -> AdcRst {
        AdcRst::from_bits(val)
    }
}
impl From<AdcRst> for u8 {
    #[inline(always)]
    fn from(val: AdcRst) -> u8 {
        AdcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl AdcclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivHalt {
        AdcclkdivHalt::from_bits(val)
    }
}
impl From<AdcclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivHalt) -> u8 {
        AdcclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AdcclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivReqflag {
        AdcclkdivReqflag::from_bits(val)
    }
}
impl From<AdcclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivReqflag) -> u8 {
        AdcclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl AdcclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivReset {
        AdcclkdivReset::from_bits(val)
    }
}
impl From<AdcclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivReset) -> u8 {
        AdcclkdivReset::to_bits(val)
    }
}
#[doc = "ADC clock source select"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkselSel {
    #[doc = "Main clk."]
    mainclk = 0x0,
    #[doc = "PLL0 clk."]
    pll0 = 0x01,
    #[doc = "FRO 96 MHZ clk."]
    fro96 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "No clk."]
    none = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl AdcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> AdcclkselSel {
        AdcclkselSel::from_bits(val)
    }
}
impl From<AdcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: AdcclkselSel) -> u8 {
        AdcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl AhbclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivHalt {
        AhbclkdivHalt::from_bits(val)
    }
}
impl From<AhbclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivHalt) -> u8 {
        AhbclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AhbclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivReqflag {
        AhbclkdivReqflag::from_bits(val)
    }
}
impl From<AhbclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivReqflag) -> u8 {
        AhbclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl AhbclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivReset {
        AhbclkdivReset::from_bits(val)
    }
}
impl From<AhbclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivReset) -> u8 {
        AhbclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnalogCtrlRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl AnalogCtrlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnalogCtrlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnalogCtrlRst {
    #[inline(always)]
    fn from(val: u8) -> AnalogCtrlRst {
        AnalogCtrlRst::from_bits(val)
    }
}
impl From<AnalogCtrlRst> for u8 {
    #[inline(always)]
    fn from(val: AnalogCtrlRst) -> u8 {
        AnalogCtrlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApFsDevNeedclk {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl ApFsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApFsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApFsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApFsDevNeedclk {
        ApFsDevNeedclk::from_bits(val)
    }
}
impl From<ApFsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApFsDevNeedclk) -> u8 {
        ApFsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApFsHostNeedclk {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl ApFsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApFsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApFsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApFsHostNeedclk {
        ApFsHostNeedclk::from_bits(val)
    }
}
impl From<ApFsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApFsHostNeedclk) -> u8 {
        ApFsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApHsDevNeedclk {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl ApHsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApHsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApHsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApHsDevNeedclk {
        ApHsDevNeedclk::from_bits(val)
    }
}
impl From<ApHsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApHsDevNeedclk) -> u8 {
        ApHsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApHsHostNeedclk {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl ApHsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApHsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApHsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApHsHostNeedclk {
        ApHsHostNeedclk::from_bits(val)
    }
}
impl From<ApHsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApHsHostNeedclk) -> u8 {
        ApHsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CasperRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CasperRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CasperRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CasperRst {
    #[inline(always)]
    fn from(val: u8) -> CasperRst {
        CasperRst::from_bits(val)
    }
}
impl From<CasperRst> for u8 {
    #[inline(always)]
    fn from(val: CasperRst) -> u8 {
        CasperRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CclkDrvPhase {
    #[doc = "0 degree shift."]
    ENUM_0_DEG = 0x0,
    #[doc = "90 degree shift."]
    ENUM_90_DEG = 0x01,
    #[doc = "180 degree shift."]
    ENUM_180_DEG = 0x02,
    #[doc = "270 degree shift."]
    ENUM_270_DEG = 0x03,
}
impl CclkDrvPhase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CclkDrvPhase {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CclkDrvPhase {
    #[inline(always)]
    fn from(val: u8) -> CclkDrvPhase {
        CclkDrvPhase::from_bits(val)
    }
}
impl From<CclkDrvPhase> for u8 {
    #[inline(always)]
    fn from(val: CclkDrvPhase) -> u8 {
        CclkDrvPhase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CclkSamplePhase {
    #[doc = "0 degree shift."]
    ENUM_0_DEG = 0x0,
    #[doc = "90 degree shift."]
    ENUM_90_DEG = 0x01,
    #[doc = "180 degree shift."]
    ENUM_180_DEG = 0x02,
    #[doc = "270 degree shift."]
    ENUM_270_DEG = 0x03,
}
impl CclkSamplePhase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CclkSamplePhase {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CclkSamplePhase {
    #[inline(always)]
    fn from(val: u8) -> CclkSamplePhase {
        CclkSamplePhase::from_bits(val)
    }
}
impl From<CclkSamplePhase> for u8 {
    #[inline(always)]
    fn from(val: CclkSamplePhase) -> u8 {
        CclkSamplePhase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl ClkoutdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivHalt {
        ClkoutdivHalt::from_bits(val)
    }
}
impl From<ClkoutdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivHalt) -> u8 {
        ClkoutdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl ClkoutdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReqflag {
        ClkoutdivReqflag::from_bits(val)
    }
}
impl From<ClkoutdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReqflag) -> u8 {
        ClkoutdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl ClkoutdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReset {
        ClkoutdivReset::from_bits(val)
    }
}
impl From<ClkoutdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReset) -> u8 {
        ClkoutdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clockgenupdatelockout(u32);
impl Clockgenupdatelockout {
    #[doc = "all hardware clock configruration are freeze."]
    pub const FREEZE: Self = Self(0x0);
    #[doc = "update all clock configuration."]
    pub const ENABLE: Self = Self(0x01);
}
impl Clockgenupdatelockout {
    pub const fn from_bits(val: u32) -> Clockgenupdatelockout {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clockgenupdatelockout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FREEZE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockgenupdatelockout {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FREEZE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clockgenupdatelockout {
    #[inline(always)]
    fn from(val: u32) -> Clockgenupdatelockout {
        Clockgenupdatelockout::from_bits(val)
    }
}
impl From<Clockgenupdatelockout> for u32 {
    #[inline(always)]
    fn from(val: Clockgenupdatelockout) -> u32 {
        Clockgenupdatelockout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CompRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CompRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompRst {
    #[inline(always)]
    fn from(val: u8) -> CompRst {
        CompRst::from_bits(val)
    }
}
impl From<CompRst> for u8 {
    #[inline(always)]
    fn from(val: CompRst) -> u8 {
        CompRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "the CPU is not in lockup."]
    AWAKE = 0x0,
    #[doc = "the CPU is in lockup."]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "the CPU is not sleeping."]
    AWAKE = 0x0,
    #[doc = "the CPU is sleeping."]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1lockup {
    #[doc = "the CPU is not in lockup."]
    AWAKE = 0x0,
    #[doc = "the CPU is in lockup."]
    SLEEPING = 0x01,
}
impl Cpu1lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu1lockup {
        Cpu1lockup::from_bits(val)
    }
}
impl From<Cpu1lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu1lockup) -> u8 {
        Cpu1lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1rsten {
    #[doc = "The CPU1 is not being reset."]
    RELEASED = 0x0,
    #[doc = "The CPU1 is being reset."]
    ASSERTED = 0x01,
}
impl Cpu1rsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1rsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1rsten {
    #[inline(always)]
    fn from(val: u8) -> Cpu1rsten {
        Cpu1rsten::from_bits(val)
    }
}
impl From<Cpu1rsten> for u8 {
    #[inline(always)]
    fn from(val: Cpu1rsten) -> u8 {
        Cpu1rsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1sleeping {
    #[doc = "the CPU is not sleeping."]
    AWAKE = 0x0,
    #[doc = "the CPU is sleeping."]
    SLEEPING = 0x01,
}
impl Cpu1sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu1sleeping {
        Cpu1sleeping::from_bits(val)
    }
}
impl From<Cpu1sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu1sleeping) -> u8 {
        Cpu1sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcgenRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CrcgenRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcgenRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcgenRst {
    #[inline(always)]
    fn from(val: u8) -> CrcgenRst {
        CrcgenRst::from_bits(val)
    }
}
impl From<CrcgenRst> for u8 {
    #[inline(always)]
    fn from(val: CrcgenRst) -> u8 {
        CrcgenRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel0Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel0Sel {
        Ctimerclksel0Sel::from_bits(val)
    }
}
impl From<Ctimerclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel0Sel) -> u8 {
        Ctimerclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel1Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel1Sel {
        Ctimerclksel1Sel::from_bits(val)
    }
}
impl From<Ctimerclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel1Sel) -> u8 {
        Ctimerclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel2Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel2Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel2Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel2Sel {
        Ctimerclksel2Sel::from_bits(val)
    }
}
impl From<Ctimerclksel2Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel2Sel) -> u8 {
        Ctimerclksel2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel3Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel3Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel3Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel3Sel {
        Ctimerclksel3Sel::from_bits(val)
    }
}
impl From<Ctimerclksel3Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel3Sel) -> u8 {
        Ctimerclksel3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel4Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel4Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel4Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel4Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel4Sel {
        Ctimerclksel4Sel::from_bits(val)
    }
}
impl From<Ctimerclksel4Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel4Sel) -> u8 {
        Ctimerclksel4Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datacfg {
    #[doc = "Data accesses from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all data accesses."]
    ONEBUF = 0x01,
    #[doc = "All buffers can be used for data accesses."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl Datacfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datacfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datacfg {
    #[inline(always)]
    fn from(val: u8) -> Datacfg {
        Datacfg::from_bits(val)
    }
}
impl From<Datacfg> for u8 {
    #[inline(always)]
    fn from(val: Datacfg) -> u8 {
        Datacfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Dbgen {
        DebugFeaturesCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Dbgen) -> u8 {
        DebugFeaturesCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Niden {
        DebugFeaturesCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Niden) -> u8 {
        DebugFeaturesCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        DebugFeaturesDpCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Dbgen) -> u8 {
        DebugFeaturesDpCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Niden {
        DebugFeaturesDpCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Niden) -> u8 {
        DebugFeaturesDpCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Dma0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rst {
        Dma0Rst::from_bits(val)
    }
}
impl From<Dma0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rst) -> u8 {
        Dma0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Dma1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rst {
        Dma1Rst::from_bits(val)
    }
}
impl From<Dma1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rst) -> u8 {
        Dma1Rst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enableupdate(u16);
impl Enableupdate {
    #[doc = "Bit Fields 0 - 15 of this register are not updated"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Bit Fields 0 - 15 of this register are updated"]
    pub const ENABLE: Self = Self(0xc0de);
}
impl Enableupdate {
    pub const fn from_bits(val: u16) -> Enableupdate {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Enableupdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0xc0de => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enableupdate {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0xc0de => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Enableupdate {
    #[inline(always)]
    fn from(val: u16) -> Enableupdate {
        Enableupdate::from_bits(val)
    }
}
impl From<Enableupdate> for u16 {
    #[inline(always)]
    fn from(val: Enableupdate) -> u16 {
        Enableupdate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcRst {
    #[inline(always)]
    fn from(val: u8) -> FcRst {
        FcRst::from_bits(val)
    }
}
impl From<FcRst> for u8 {
    #[inline(always)]
    fn from(val: FcRst) -> u8 {
        FcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fetchcfg {
    #[doc = "Instruction fetches from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all instruction fetches."]
    ONEBUF = 0x01,
    #[doc = "All buffers may be used for instruction fetches."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fetchcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fetchcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fetchcfg {
    #[inline(always)]
    fn from(val: u8) -> Fetchcfg {
        Fetchcfg::from_bits(val)
    }
}
impl From<Fetchcfg> for u8 {
    #[inline(always)]
    fn from(val: Fetchcfg) -> u8 {
        Fetchcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FlashRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashRst {
    #[inline(always)]
    fn from(val: u8) -> FlashRst {
        FlashRst::from_bits(val)
    }
}
impl From<FlashRst> for u8 {
    #[inline(always)]
    fn from(val: FlashRst) -> u8 {
        FlashRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashtim {
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0 = 0x0,
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1 = 0x01,
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2 = 0x02,
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3 = 0x03,
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4 = 0x04,
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5 = 0x05,
    #[doc = "7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    FLASHTIM6 = 0x06,
    #[doc = "8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    FLASHTIM7 = 0x07,
    #[doc = "9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    FLASHTIM8 = 0x08,
    #[doc = "10 system clocks flash access time (for system clock rates up to 115 MHz)."]
    FLASHTIM9 = 0x09,
    #[doc = "11 system clocks flash access time (for system clock rates up to 130 MHz)."]
    FLASHTIM10 = 0x0a,
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    FLASHTIM11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Flashtim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashtim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashtim {
    #[inline(always)]
    fn from(val: u8) -> Flashtim {
        Flashtim::from_bits(val)
    }
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(val: Flashtim) -> u8 {
        Flashtim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FmcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmcRst {
    #[inline(always)]
    fn from(val: u8) -> FmcRst {
        FmcRst::from_bits(val)
    }
}
impl From<FmcRst> for u8 {
    #[inline(always)]
    fn from(val: FmcRst) -> u8 {
        FmcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FrohfdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReqflag {
        FrohfdivReqflag::from_bits(val)
    }
}
impl From<FrohfdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReqflag) -> u8 {
        FrohfdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FrohfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReset {
        FrohfdivReset::from_bits(val)
    }
}
impl From<FrohfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReset) -> u8 {
        FrohfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GintRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GintRst {
    #[inline(always)]
    fn from(val: u8) -> GintRst {
        GintRst::from_bits(val)
    }
}
impl From<GintRst> for u8 {
    #[inline(always)]
    fn from(val: GintRst) -> u8 {
        GintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Rst {
        Gpio0Rst::from_bits(val)
    }
}
impl From<Gpio0Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Rst) -> u8 {
        Gpio0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Rst {
        Gpio1Rst::from_bits(val)
    }
}
impl From<Gpio1Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Rst) -> u8 {
        Gpio1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Rst {
        Gpio2Rst::from_bits(val)
    }
}
impl From<Gpio2Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Rst) -> u8 {
        Gpio2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Rst {
        Gpio3Rst::from_bits(val)
    }
}
impl From<Gpio3Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Rst) -> u8 {
        Gpio3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSecIntRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GpioSecIntRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSecIntRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSecIntRst {
    #[inline(always)]
    fn from(val: u8) -> GpioSecIntRst {
        GpioSecIntRst::from_bits(val)
    }
}
impl From<GpioSecIntRst> for u8 {
    #[inline(always)]
    fn from(val: GpioSecIntRst) -> u8 {
        GpioSecIntRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSecRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GpioSecRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSecRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSecRst {
    #[inline(always)]
    fn from(val: u8) -> GpioSecRst {
        GpioSecRst::from_bits(val)
    }
}
impl From<GpioSecRst> for u8 {
    #[inline(always)]
    fn from(val: GpioSecRst) -> u8 {
        GpioSecRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashAesRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HashAesRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashAesRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashAesRst {
    #[inline(always)]
    fn from(val: u8) -> HashAesRst {
        HashAesRst::from_bits(val)
    }
}
impl From<HashAesRst> for u8 {
    #[inline(always)]
    fn from(val: HashAesRst) -> u8 {
        HashAesRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsDevWakeupN {
    #[doc = "Forces USB1_PHY to wake-up."]
    FORCE_WUP = 0x0,
    #[doc = "Normal USB1_PHY behavior."]
    NORMAL_WUP = 0x01,
}
impl HsDevWakeupN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsDevWakeupN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsDevWakeupN {
    #[inline(always)]
    fn from(val: u8) -> HsDevWakeupN {
        HsDevWakeupN::from_bits(val)
    }
}
impl From<HsDevWakeupN> for u8 {
    #[inline(always)]
    fn from(val: HsDevWakeupN) -> u8 {
        HsDevWakeupN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsLspiRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HsLspiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsLspiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsLspiRst {
    #[inline(always)]
    fn from(val: u8) -> HsLspiRst {
        HsLspiRst::from_bits(val)
    }
}
impl From<HsLspiRst> for u8 {
    #[inline(always)]
    fn from(val: HsLspiRst) -> u8 {
        HsLspiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HslspiclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl HslspiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HslspiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HslspiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> HslspiclkselSel {
        HslspiclkselSel::from_bits(val)
    }
}
impl From<HslspiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: HslspiclkselSel) -> u8 {
        HslspiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntClear {
    #[doc = "No effect."]
    NONE = 0x0,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR = 0x01,
}
impl IntClear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntClear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntClear {
    #[inline(always)]
    fn from(val: u8) -> IntClear {
        IntClear::from_bits(val)
    }
}
impl From<IntClear> for u8 {
    #[inline(always)]
    fn from(val: IntClear) -> u8 {
        IntClear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrl {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE = 0x0,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE = 0x01,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING = 0x02,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH = 0x03,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING = 0x04,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW = 0x05,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH = 0x06,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2 = 0x07,
}
impl IntCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrl {
    #[inline(always)]
    fn from(val: u8) -> IntCtrl {
        IntCtrl::from_bits(val)
    }
}
impl From<IntCtrl> for u8 {
    #[inline(always)]
    fn from(val: IntCtrl) -> u8 {
        IntCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntEnable {
    #[doc = "interrupt disable."]
    INT_DISABLE = 0x0,
    #[doc = "interrupt enable."]
    INT_ENABLE = 0x01,
}
impl IntEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntEnable {
    #[inline(always)]
    fn from(val: u8) -> IntEnable {
        IntEnable::from_bits(val)
    }
}
impl From<IntEnable> for u8 {
    #[inline(always)]
    fn from(val: IntEnable) -> u8 {
        IntEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntSource {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT = 0x0,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT = 0x01,
}
impl IntSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntSource {
    #[inline(always)]
    fn from(val: u8) -> IntSource {
        IntSource::from_bits(val)
    }
}
impl From<IntSource> for u8 {
    #[inline(always)]
    fn from(val: IntSource) -> u8 {
        IntSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatus {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl IntStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatus {
    #[inline(always)]
    fn from(val: u8) -> IntStatus {
        IntStatus::from_bits(val)
    }
}
impl From<IntStatus> for u8 {
    #[inline(always)]
    fn from(val: IntStatus) -> u8 {
        IntStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoconRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl IoconRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IoconRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IoconRst {
    #[inline(always)]
    fn from(val: u8) -> IoconRst {
        IoconRst::from_bits(val)
    }
}
impl From<IoconRst> for u8 {
    #[inline(always)]
    fn from(val: IoconRst) -> u8 {
        IoconRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "1010: Enable write access to all 6 registers."]
    ENABLE = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MailboxRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MailboxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MailboxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MailboxRst {
    #[inline(always)]
    fn from(val: u8) -> MailboxRst {
        MailboxRst::from_bits(val)
    }
}
impl From<MailboxRst> for u8 {
    #[inline(always)]
    fn from(val: MailboxRst) -> u8 {
        MailboxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselaSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MainclkselaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselaSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselaSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselaSel {
        MainclkselaSel::from_bits(val)
    }
}
impl From<MainclkselaSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselaSel) -> u8 {
        MainclkselaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselbSel {
    #[doc = "Main Clock A."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "PLL1 clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MainclkselbSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselbSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselbSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselbSel {
        MainclkselbSel::from_bits(val)
    }
}
impl From<MainclkselbSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselbSel) -> u8 {
        MainclkselbSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Map {
    #[doc = "Vector Table in ROM."]
    ROM0 = 0x0,
    #[doc = "Vector Table in RAM."]
    RAM1 = 0x01,
    #[doc = "Vector Table in Flash."]
    FLASH0 = 0x02,
    #[doc = "Vector Table in Flash."]
    FLASH1 = 0x03,
}
impl Map {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Map {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Map {
    #[inline(always)]
    fn from(val: u8) -> Map {
        Map::from_bits(val)
    }
}
impl From<Map> for u8 {
    #[inline(always)]
    fn from(val: Map) -> u8 {
        Map::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkclkselSel {
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl MclkclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MclkclkselSel {
        MclkclkselSel::from_bits(val)
    }
}
impl From<MclkclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MclkclkselSel) -> u8 {
        MclkclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl MclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MclkdivHalt {
        MclkdivHalt::from_bits(val)
    }
}
impl From<MclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MclkdivHalt) -> u8 {
        MclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl MclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> MclkdivReqflag {
        MclkdivReqflag::from_bits(val)
    }
}
impl From<MclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: MclkdivReqflag) -> u8 {
        MclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl MclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MclkdivReset {
        MclkdivReset::from_bits(val)
    }
}
impl From<MclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MclkdivReset) -> u8 {
        MclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkio {
    #[doc = "input mode."]
    INPUT = 0x0,
    #[doc = "output mode."]
    OUTPUT = 0x01,
}
impl Mclkio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkio {
    #[inline(always)]
    fn from(val: u8) -> Mclkio {
        Mclkio::from_bits(val)
    }
}
impl From<Mclkio> for u8 {
    #[inline(always)]
    fn from(val: Mclkio) -> u8 {
        Mclkio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MrtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRst {
    #[inline(always)]
    fn from(val: u8) -> MrtRst {
        MrtRst::from_bits(val)
    }
}
impl From<MrtRst> for u8 {
    #[inline(always)]
    fn from(val: MrtRst) -> u8 {
        MrtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MuxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRst {
    #[inline(always)]
    fn from(val: u8) -> MuxRst {
        MuxRst::from_bits(val)
    }
}
impl From<MuxRst> for u8 {
    #[inline(always)]
    fn from(val: MuxRst) -> u8 {
        MuxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl OstimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerRst {
    #[inline(always)]
    fn from(val: u8) -> OstimerRst {
        OstimerRst::from_bits(val)
    }
}
impl From<OstimerRst> for u8 {
    #[inline(always)]
    fn from(val: OstimerRst) -> u8 {
        OstimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhaseActive {
    #[doc = "Bypassed."]
    BYPASSED = 0x0,
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT = 0x01,
}
impl PhaseActive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhaseActive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhaseActive {
    #[inline(always)]
    fn from(val: u8) -> PhaseActive {
        PhaseActive::from_bits(val)
    }
}
impl From<PhaseActive> for u8 {
    #[inline(always)]
    fn from(val: PhaseActive) -> u8 {
        PhaseActive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRst {
    #[inline(always)]
    fn from(val: u8) -> PintRst {
        PintRst::from_bits(val)
    }
}
impl From<PintRst> for u8 {
    #[inline(always)]
    fn from(val: PintRst) -> u8 {
        PintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Pll0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivHalt {
        Pll0clkdivHalt::from_bits(val)
    }
}
impl From<Pll0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivHalt) -> u8 {
        Pll0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Pll0clkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivReqflag {
        Pll0clkdivReqflag::from_bits(val)
    }
}
impl From<Pll0clkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivReqflag) -> u8 {
        Pll0clkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Pll0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivReset {
        Pll0clkdivReset::from_bits(val)
    }
}
impl From<Pll0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivReset) -> u8 {
        Pll0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkselSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Pll0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkselSel {
        Pll0clkselSel::from_bits(val)
    }
}
impl From<Pll0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkselSel) -> u8 {
        Pll0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBwdirect {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl Pll0ctrlBwdirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBwdirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBwdirect {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBwdirect {
        Pll0ctrlBwdirect::from_bits(val)
    }
}
impl From<Pll0ctrlBwdirect> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBwdirect) -> u8 {
        Pll0ctrlBwdirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspll {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "Bypass PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspll {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspll {
        Pll0ctrlBypasspll::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspll> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspll) -> u8 {
        Pll0ctrlBypasspll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspostdiv {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspostdiv {
        Pll0ctrlBypasspostdiv::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspostdiv) -> u8 {
        Pll0ctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspostdiv2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspostdiv2 {
        Pll0ctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspostdiv2) -> u8 {
        Pll0ctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypassprediv {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypassprediv {
        Pll0ctrlBypassprediv::from_bits(val)
    }
}
impl From<Pll0ctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypassprediv) -> u8 {
        Pll0ctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkselSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Pll1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkselSel {
        Pll1clkselSel::from_bits(val)
    }
}
impl From<Pll1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkselSel) -> u8 {
        Pll1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBwdirect {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl Pll1ctrlBwdirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBwdirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBwdirect {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBwdirect {
        Pll1ctrlBwdirect::from_bits(val)
    }
}
impl From<Pll1ctrlBwdirect> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBwdirect) -> u8 {
        Pll1ctrlBwdirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspll {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspll {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspll {
        Pll1ctrlBypasspll::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspll> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspll) -> u8 {
        Pll1ctrlBypasspll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspostdiv {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspostdiv {
        Pll1ctrlBypasspostdiv::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspostdiv) -> u8 {
        Pll1ctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspostdiv2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspostdiv2 {
        Pll1ctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspostdiv2) -> u8 {
        Pll1ctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypassprediv {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypassprediv {
        Pll1ctrlBypassprediv::from_bits(val)
    }
}
impl From<Pll1ctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypassprediv) -> u8 {
        Pll1ctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlulutRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PlulutRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlulutRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlulutRst {
    #[inline(always)]
    fn from(val: u8) -> PlulutRst {
        PlulutRst::from_bits(val)
    }
}
impl From<PlulutRst> for u8 {
    #[inline(always)]
    fn from(val: PlulutRst) -> u8 {
        PlulutRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolFsDevNeedclk {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolFsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolFsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolFsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolFsDevNeedclk {
        PolFsDevNeedclk::from_bits(val)
    }
}
impl From<PolFsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolFsDevNeedclk) -> u8 {
        PolFsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolFsHostNeedclk {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolFsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolFsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolFsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolFsHostNeedclk {
        PolFsHostNeedclk::from_bits(val)
    }
}
impl From<PolFsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolFsHostNeedclk) -> u8 {
        PolFsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolHsDevNeedclk {
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolHsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolHsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolHsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolHsDevNeedclk {
        PolHsDevNeedclk::from_bits(val)
    }
}
impl From<PolHsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolHsDevNeedclk) -> u8 {
        PolHsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolHsHostNeedclk {
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolHsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolHsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolHsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolHsHostNeedclk {
        PolHsHostNeedclk::from_bits(val)
    }
}
impl From<PolHsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolHsHostNeedclk) -> u8 {
        PolHsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PqRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PqRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PqRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PqRst {
    #[inline(always)]
    fn from(val: u8) -> PqRst {
        PqRst::from_bits(val)
    }
}
impl From<PqRst> for u8 {
    #[inline(always)]
    fn from(val: PqRst) -> u8 {
        PqRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prefovr {
    #[doc = "Any previously initiated prefetch will be completed."]
    NORMAL = 0x0,
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    OVERRIDE = 0x01,
}
impl Prefovr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prefovr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prefovr {
    #[inline(always)]
    fn from(val: u8) -> Prefovr {
        Prefovr::from_bits(val)
    }
}
impl From<Prefovr> for u8 {
    #[inline(always)]
    fn from(val: Prefovr) -> u8 {
        Prefovr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psync {
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    USED = 0x0,
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    BYPASS = 0x01,
}
impl Psync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psync {
    #[inline(always)]
    fn from(val: u8) -> Psync {
        Psync::from_bits(val)
    }
}
impl From<Psync> for u8 {
    #[inline(always)]
    fn from(val: Psync) -> u8 {
        Psync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PufRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRst {
    #[inline(always)]
    fn from(val: u8) -> PufRst {
        PufRst::from_bits(val)
    }
}
impl From<PufRst> for u8 {
    #[inline(always)]
    fn from(val: PufRst) -> u8 {
        PufRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RngRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RngRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RngRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RngRst {
    #[inline(always)]
    fn from(val: u8) -> RngRst {
        RngRst::from_bits(val)
    }
}
impl From<RngRst> for u8 {
    #[inline(always)]
    fn from(val: RngRst) -> u8 {
        RngRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RomRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomRst {
    #[inline(always)]
    fn from(val: u8) -> RomRst {
        RomRst::from_bits(val)
    }
}
impl From<RomRst> for u8 {
    #[inline(always)]
    fn from(val: RomRst) -> u8 {
        RomRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RtcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRst {
    #[inline(always)]
    fn from(val: u8) -> RtcRst {
        RtcRst::from_bits(val)
    }
}
impl From<RtcRst> for u8 {
    #[inline(always)]
    fn from(val: RtcRst) -> u8 {
        RtcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SctRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctRst {
    #[inline(always)]
    fn from(val: u8) -> SctRst {
        SctRst::from_bits(val)
    }
}
impl From<SctRst> for u8 {
    #[inline(always)]
    fn from(val: SctRst) -> u8 {
        SctRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl SctclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivHalt {
        SctclkdivHalt::from_bits(val)
    }
}
impl From<SctclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivHalt) -> u8 {
        SctclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SctclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReqflag {
        SctclkdivReqflag::from_bits(val)
    }
}
impl From<SctclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReqflag) -> u8 {
        SctclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SctclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReset {
        SctclkdivReset::from_bits(val)
    }
}
impl From<SctclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReset) -> u8 {
        SctclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl SctclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctclkselSel {
        SctclkselSel::from_bits(val)
    }
}
impl From<SctclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctclkselSel) -> u8 {
        SctclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctipuRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SctipuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctipuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctipuRst {
    #[inline(always)]
    fn from(val: u8) -> SctipuRst {
        SctipuRst::from_bits(val)
    }
}
impl From<SctipuRst> for u8 {
    #[inline(always)]
    fn from(val: SctipuRst) -> u8 {
        SctipuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SdioRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioRst {
    #[inline(always)]
    fn from(val: u8) -> SdioRst {
        SdioRst::from_bits(val)
    }
}
impl From<SdioRst> for u8 {
    #[inline(always)]
    fn from(val: SdioRst) -> u8 {
        SdioRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl SdioclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SdioclkdivHalt {
        SdioclkdivHalt::from_bits(val)
    }
}
impl From<SdioclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SdioclkdivHalt) -> u8 {
        SdioclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SdioclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> SdioclkdivReqflag {
        SdioclkdivReqflag::from_bits(val)
    }
}
impl From<SdioclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: SdioclkdivReqflag) -> u8 {
        SdioclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SdioclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SdioclkdivReset {
        SdioclkdivReset::from_bits(val)
    }
}
impl From<SdioclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SdioclkdivReset) -> u8 {
        SdioclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl SdioclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SdioclkselSel {
        SdioclkselSel::from_bits(val)
    }
}
impl From<SdioclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SdioclkselSel) -> u8 {
        SdioclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl1Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl1Rst {
        SramCtrl1Rst::from_bits(val)
    }
}
impl From<SramCtrl1Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl1Rst) -> u8 {
        SramCtrl1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl2Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl2Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl2Rst {
        SramCtrl2Rst::from_bits(val)
    }
}
impl From<SramCtrl2Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl2Rst) -> u8 {
        SramCtrl2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl3Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl3Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl3Rst {
        SramCtrl3Rst::from_bits(val)
    }
}
impl From<SramCtrl3Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl3Rst) -> u8 {
        SramCtrl3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl4Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl4Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl4Rst {
        SramCtrl4Rst::from_bits(val)
    }
}
impl From<SramCtrl4Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl4Rst) -> u8 {
        SramCtrl4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status {
    #[inline(always)]
    fn from(val: u8) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(val: Status) -> u8 {
        Status::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwrReset(u32);
impl SwrReset {
    #[doc = "Bloc is not reset."]
    pub const RELEASED: Self = Self(0x0);
    #[doc = "Generate a software reset."]
    pub const ASSERTED: Self = Self(0x5a00_0001);
}
impl SwrReset {
    pub const fn from_bits(val: u32) -> SwrReset {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwrReset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RELEASED"),
            0x5a00_0001 => f.write_str("ASSERTED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwrReset {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RELEASED"),
            0x5a00_0001 => defmt::write!(f, "ASSERTED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwrReset {
    #[inline(always)]
    fn from(val: u32) -> SwrReset {
        SwrReset::from_bits(val)
    }
}
impl From<SwrReset> for u32 {
    #[inline(always)]
    fn from(val: SwrReset) -> u32 {
        SwrReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysctlRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SysctlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysctlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysctlRst {
    #[inline(always)]
    fn from(val: u8) -> SysctlRst {
        SysctlRst::from_bits(val)
    }
}
impl From<SysctlRst> for u8 {
    #[inline(always)]
    fn from(val: SysctlRst) -> u8 {
        SysctlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Halt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Systickclkdiv0Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Halt {
        Systickclkdiv0Halt::from_bits(val)
    }
}
impl From<Systickclkdiv0Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Halt) -> u8 {
        Systickclkdiv0Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Systickclkdiv0Reqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reqflag {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reqflag {
        Systickclkdiv0Reqflag::from_bits(val)
    }
}
impl From<Systickclkdiv0Reqflag> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reqflag) -> u8 {
        Systickclkdiv0Reqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Systickclkdiv0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reset {
        Systickclkdiv0Reset::from_bits(val)
    }
}
impl From<Systickclkdiv0Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reset) -> u8 {
        Systickclkdiv0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Halt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Systickclkdiv1Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Halt {
        Systickclkdiv1Halt::from_bits(val)
    }
}
impl From<Systickclkdiv1Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Halt) -> u8 {
        Systickclkdiv1Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Reqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Systickclkdiv1Reqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Reqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Reqflag {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Reqflag {
        Systickclkdiv1Reqflag::from_bits(val)
    }
}
impl From<Systickclkdiv1Reqflag> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Reqflag) -> u8 {
        Systickclkdiv1Reqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Reset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Systickclkdiv1Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Reset {
        Systickclkdiv1Reset::from_bits(val)
    }
}
impl From<Systickclkdiv1Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Reset) -> u8 {
        Systickclkdiv1Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "System Tick 0 divided clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel1Sel {
    #[doc = "System Tick 1 divided clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Systickclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel1Sel {
        Systickclksel1Sel::from_bits(val)
    }
}
impl From<Systickclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel1Sel) -> u8 {
        Systickclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer0Rst {
        Timer0Rst::from_bits(val)
    }
}
impl From<Timer0Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer0Rst) -> u8 {
        Timer0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer1Rst {
        Timer1Rst::from_bits(val)
    }
}
impl From<Timer1Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer1Rst) -> u8 {
        Timer1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer2Rst {
        Timer2Rst::from_bits(val)
    }
}
impl From<Timer2Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer2Rst) -> u8 {
        Timer2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer3Rst {
        Timer3Rst::from_bits(val)
    }
}
impl From<Timer3Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer3Rst) -> u8 {
        Timer3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer4Rst {
        Timer4Rst::from_bits(val)
    }
}
impl From<Timer4Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer4Rst) -> u8 {
        Timer4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl TraceclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivHalt {
        TraceclkdivHalt::from_bits(val)
    }
}
impl From<TraceclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivHalt) -> u8 {
        TraceclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl TraceclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReqflag {
        TraceclkdivReqflag::from_bits(val)
    }
}
impl From<TraceclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReqflag) -> u8 {
        TraceclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl TraceclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReset {
        TraceclkdivReset::from_bits(val)
    }
}
impl From<TraceclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReset) -> u8 {
        TraceclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "Trace divided clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0DevRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0DevRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0DevRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0DevRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0DevRst {
        Usb0DevRst::from_bits(val)
    }
}
impl From<Usb0DevRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0DevRst) -> u8 {
        Usb0DevRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0HostmRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0HostmRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0HostmRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0HostmRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0HostmRst {
        Usb0HostmRst::from_bits(val)
    }
}
impl From<Usb0HostmRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0HostmRst) -> u8 {
        Usb0HostmRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0HostsRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0HostsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0HostsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0HostsRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0HostsRst {
        Usb0HostsRst::from_bits(val)
    }
}
impl From<Usb0HostsRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0HostsRst) -> u8 {
        Usb0HostsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Usb0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivHalt {
        Usb0clkdivHalt::from_bits(val)
    }
}
impl From<Usb0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivHalt) -> u8 {
        Usb0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Usb0clkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReqflag {
        Usb0clkdivReqflag::from_bits(val)
    }
}
impl From<Usb0clkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReqflag) -> u8 {
        Usb0clkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Usb0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReset {
        Usb0clkdivReset::from_bits(val)
    }
}
impl From<Usb0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReset) -> u8 {
        Usb0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Usb0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkselSel {
        Usb0clkselSel::from_bits(val)
    }
}
impl From<Usb0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkselSel) -> u8 {
        Usb0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0needclkstatDevNeedclk {
    #[doc = "USB0 Device clock is low."]
    LOW = 0x0,
    #[doc = "USB0 Device clock is high."]
    HIGH = 0x01,
}
impl Usb0needclkstatDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0needclkstatDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0needclkstatDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb0needclkstatDevNeedclk {
        Usb0needclkstatDevNeedclk::from_bits(val)
    }
}
impl From<Usb0needclkstatDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb0needclkstatDevNeedclk) -> u8 {
        Usb0needclkstatDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0needclkstatHostNeedclk {
    #[doc = "USB0 Host clock is low."]
    LOW = 0x0,
    #[doc = "USB0 Host clock is high."]
    HIGH = 0x01,
}
impl Usb0needclkstatHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0needclkstatHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0needclkstatHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb0needclkstatHostNeedclk {
        Usb0needclkstatHostNeedclk::from_bits(val)
    }
}
impl From<Usb0needclkstatHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb0needclkstatHostNeedclk) -> u8 {
        Usb0needclkstatHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1DevRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1DevRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1DevRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1DevRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1DevRst {
        Usb1DevRst::from_bits(val)
    }
}
impl From<Usb1DevRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1DevRst) -> u8 {
        Usb1DevRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1HostRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1HostRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1HostRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1HostRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1HostRst {
        Usb1HostRst::from_bits(val)
    }
}
impl From<Usb1HostRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1HostRst) -> u8 {
        Usb1HostRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1PhyRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1PhyRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1PhyRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1PhyRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1PhyRst {
        Usb1PhyRst::from_bits(val)
    }
}
impl From<Usb1PhyRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1PhyRst) -> u8 {
        Usb1PhyRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1RamRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1RamRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1RamRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1RamRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1RamRst {
        Usb1RamRst::from_bits(val)
    }
}
impl From<Usb1RamRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1RamRst) -> u8 {
        Usb1RamRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1needclkstatDevNeedclk {
    #[doc = "DEV_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "DEV_NEEDCLK is high."]
    HIGH = 0x01,
}
impl Usb1needclkstatDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1needclkstatDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1needclkstatDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb1needclkstatDevNeedclk {
        Usb1needclkstatDevNeedclk::from_bits(val)
    }
}
impl From<Usb1needclkstatDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb1needclkstatDevNeedclk) -> u8 {
        Usb1needclkstatDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1needclkstatHostNeedclk {
    #[doc = "HOST_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "HOST_NEEDCLK is high."]
    HIGH = 0x01,
}
impl Usb1needclkstatHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1needclkstatHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1needclkstatHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb1needclkstatHostNeedclk {
        Usb1needclkstatHostNeedclk::from_bits(val)
    }
}
impl From<Usb1needclkstatHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb1needclkstatHostNeedclk) -> u8 {
        Usb1needclkstatHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl UtickRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRst {
    #[inline(always)]
    fn from(val: u8) -> UtickRst {
        UtickRst::from_bits(val)
    }
}
impl From<UtickRst> for u8 {
    #[inline(always)]
    fn from(val: UtickRst) -> u8 {
        UtickRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val {
    #[doc = "P+ is smaller than P-."]
    SMALLER = 0x0,
    #[doc = "P+ is greater than P-."]
    GREATER = 0x01,
}
impl Val {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val {
    #[inline(always)]
    fn from(val: u8) -> Val {
        Val::from_bits(val)
    }
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(val: Val) -> u8 {
        Val::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl WdtclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivHalt {
        WdtclkdivHalt::from_bits(val)
    }
}
impl From<WdtclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivHalt) -> u8 {
        WdtclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl WdtclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivReqflag {
        WdtclkdivReqflag::from_bits(val)
    }
}
impl From<WdtclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivReqflag) -> u8 {
        WdtclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl WdtclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivReset {
        WdtclkdivReset::from_bits(val)
    }
}
impl From<WdtclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivReset) -> u8 {
        WdtclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WwdtRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl WwdtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WwdtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WwdtRst {
    #[inline(always)]
    fn from(val: u8) -> WwdtRst {
        WwdtRst::from_bits(val)
    }
}
impl From<WwdtRst> for u8 {
    #[inline(always)]
    fn from(val: WwdtRst) -> u8 {
        WwdtRst::to_bits(val)
    }
}
