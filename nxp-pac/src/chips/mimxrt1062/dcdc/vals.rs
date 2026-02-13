#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CurSnsThrsh {
    #[doc = "150 mA"]
    SELECT_ZERO = 0x0,
    #[doc = "250 mA"]
    SELECT_ONE = 0x01,
    #[doc = "350 mA"]
    SELECT_TWO = 0x02,
    #[doc = "450 mA"]
    SELECT_THREE = 0x03,
    #[doc = "550 mA"]
    SELECT_FOUR = 0x04,
    #[doc = "650 mA"]
    SELECT_FIVE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CurSnsThrsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CurSnsThrsh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CurSnsThrsh {
    #[inline(always)]
    fn from(val: u8) -> CurSnsThrsh {
        CurSnsThrsh::from_bits(val)
    }
}
impl From<CurSnsThrsh> for u8 {
    #[inline(always)]
    fn from(val: CurSnsThrsh) -> u8 {
        CurSnsThrsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableAutoClkSwitch {
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring OSC to 24M xtal automatically"]
    XTAL_CLK = 0x0,
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses"]
    SEL_CLK = 0x01,
}
impl DisableAutoClkSwitch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableAutoClkSwitch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableAutoClkSwitch {
    #[inline(always)]
    fn from(val: u8) -> DisableAutoClkSwitch {
        DisableAutoClkSwitch::from_bits(val)
    }
}
impl From<DisableAutoClkSwitch> for u8 {
    #[inline(always)]
    fn from(val: DisableAutoClkSwitch) -> u8 {
        DisableAutoClkSwitch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisablePulseSkip {
    #[doc = "DCDC will be idle to save current dissipation when the duty cycle get to the low limit which is set by NEGLIMIT_IN."]
    IDLE = 0x0,
    #[doc = "DCDC will keep working with the low limited duty cycle NEGLIMIT_IN."]
    NOT_IDLE = 0x01,
}
impl DisablePulseSkip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisablePulseSkip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisablePulseSkip {
    #[inline(always)]
    fn from(val: u8) -> DisablePulseSkip {
        DisablePulseSkip::from_bits(val)
    }
}
impl From<DisablePulseSkip> for u8 {
    #[inline(always)]
    fn from(val: DisablePulseSkip) -> u8 {
        DisablePulseSkip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableStep {
    #[doc = "Enable stepping for the output of VDD_SOC of DCDC"]
    ENABLE = 0x0,
    #[doc = "Disable stepping for the output of VDD_SOC of DCDC"]
    DISABLE = 0x01,
}
impl DisableStep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableStep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableStep {
    #[inline(always)]
    fn from(val: u8) -> DisableStep {
        DisableStep::from_bits(val)
    }
}
impl From<DisableStep> for u8 {
    #[inline(always)]
    fn from(val: DisableStep) -> u8 {
        DisableStep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LoopctrlHstThresh {
    #[doc = "Lower hysteresis threshold (about 2.5mV in typical, but this value can vary with PVT corners"]
    LOW_HYST_THRESH = 0x0,
    #[doc = "Higher hysteresis threshold (about 5mV in typical)"]
    HIGH_HYST_THRESH = 0x01,
}
impl LoopctrlHstThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LoopctrlHstThresh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LoopctrlHstThresh {
    #[inline(always)]
    fn from(val: u8) -> LoopctrlHstThresh {
        LoopctrlHstThresh::from_bits(val)
    }
}
impl From<LoopctrlHstThresh> for u8 {
    #[inline(always)]
    fn from(val: LoopctrlHstThresh) -> u8 {
        LoopctrlHstThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCmpIsrcSel {
    #[doc = "50 nA"]
    SEL0 = 0x0,
    #[doc = "100 nA"]
    SEL1 = 0x01,
    #[doc = "200 nA"]
    SEL2 = 0x02,
    #[doc = "400 nA"]
    SEL3 = 0x03,
}
impl LpCmpIsrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCmpIsrcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCmpIsrcSel {
    #[inline(always)]
    fn from(val: u8) -> LpCmpIsrcSel {
        LpCmpIsrcSel::from_bits(val)
    }
}
impl From<LpCmpIsrcSel> for u8 {
    #[inline(always)]
    fn from(val: LpCmpIsrcSel) -> u8 {
        LpCmpIsrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpHighHys {
    #[doc = "Adjust hysteretic value in low power to 12.5mV"]
    LP_12P5M_V = 0x0,
    #[doc = "Adjust hysteretic value in low power to 25mV"]
    LP_25M_V = 0x01,
}
impl LpHighHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpHighHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpHighHys {
    #[inline(always)]
    fn from(val: u8) -> LpHighHys {
        LpHighHys::from_bits(val)
    }
}
impl From<LpHighHys> for u8 {
    #[inline(always)]
    fn from(val: LpHighHys) -> u8 {
        LpHighHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpOverloadFreqSel {
    #[doc = "eight 32k cycle"]
    EIGHT_32K_CYCLE = 0x0,
    #[doc = "sixteen 32k cycle"]
    SIXTEEN_32K_CYCLE = 0x01,
}
impl LpOverloadFreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpOverloadFreqSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpOverloadFreqSel {
    #[inline(always)]
    fn from(val: u8) -> LpOverloadFreqSel {
        LpOverloadFreqSel::from_bits(val)
    }
}
impl From<LpOverloadFreqSel> for u8 {
    #[inline(always)]
    fn from(val: LpOverloadFreqSel) -> u8 {
        LpOverloadFreqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpOverloadThrsh {
    #[doc = "32"]
    THRSH_32 = 0x0,
    #[doc = "64"]
    THRSH_64 = 0x01,
    #[doc = "16"]
    THRSH_16 = 0x02,
    #[doc = "8"]
    THRSH_8 = 0x03,
}
impl LpOverloadThrsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpOverloadThrsh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpOverloadThrsh {
    #[inline(always)]
    fn from(val: u8) -> LpOverloadThrsh {
        LpOverloadThrsh::from_bits(val)
    }
}
impl From<LpOverloadThrsh> for u8 {
    #[inline(always)]
    fn from(val: LpOverloadThrsh) -> u8 {
        LpOverloadThrsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinpwrDcHalfclk {
    #[doc = "DCDC clock remains at full frequency for continuous mode"]
    FULLFREQ = 0x0,
    #[doc = "DCDC clock set to half frequency for continuous mode"]
    HALFFREQ = 0x01,
}
impl MinpwrDcHalfclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MinpwrDcHalfclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MinpwrDcHalfclk {
    #[inline(always)]
    fn from(val: u8) -> MinpwrDcHalfclk {
        MinpwrDcHalfclk::from_bits(val)
    }
}
impl From<MinpwrDcHalfclk> for u8 {
    #[inline(always)]
    fn from(val: MinpwrDcHalfclk) -> u8 {
        MinpwrDcHalfclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvercurTrigAdj {
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.25 A"]
    SELECT_ZERO = 0x0,
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.25 A"]
    SELECT_ONE = 0x01,
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.2 A"]
    SELECT_TWO = 0x02,
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.2 A"]
    SELECT_THREE = 0x03,
}
impl OvercurTrigAdj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvercurTrigAdj {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvercurTrigAdj {
    #[inline(always)]
    fn from(val: u8) -> OvercurTrigAdj {
        OvercurTrigAdj::from_bits(val)
    }
}
impl From<OvercurTrigAdj> for u8 {
    #[inline(always)]
    fn from(val: OvercurTrigAdj) -> u8 {
        OvercurTrigAdj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdCmpBattDet {
    #[doc = "Low voltage detection comparator is enabled"]
    ENABLED = 0x0,
    #[doc = "Low voltage detection comparator is disabled"]
    DISABLED = 0x01,
}
impl PwdCmpBattDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdCmpBattDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdCmpBattDet {
    #[inline(always)]
    fn from(val: u8) -> PwdCmpBattDet {
        PwdCmpBattDet::from_bits(val)
    }
}
impl From<PwdCmpBattDet> for u8 {
    #[inline(always)]
    fn from(val: PwdCmpBattDet) -> u8 {
        PwdCmpBattDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdCmpOffset {
    #[doc = "Output range comparator powered up"]
    POWERED_UP = 0x0,
    #[doc = "Output range comparator powered down"]
    POWERED_DOWN = 0x01,
}
impl PwdCmpOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdCmpOffset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdCmpOffset {
    #[inline(always)]
    fn from(val: u8) -> PwdCmpOffset {
        PwdCmpOffset::from_bits(val)
    }
}
impl From<PwdCmpOffset> for u8 {
    #[inline(always)]
    fn from(val: PwdCmpOffset) -> u8 {
        PwdCmpOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdCurSnsCmp {
    #[doc = "Current Detector powered up"]
    POWERED_UP = 0x0,
    #[doc = "Current Detector powered down"]
    POWERED_DOWN = 0x01,
}
impl PwdCurSnsCmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdCurSnsCmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdCurSnsCmp {
    #[inline(always)]
    fn from(val: u8) -> PwdCurSnsCmp {
        PwdCurSnsCmp::from_bits(val)
    }
}
impl From<PwdCurSnsCmp> for u8 {
    #[inline(always)]
    fn from(val: PwdCurSnsCmp) -> u8 {
        PwdCurSnsCmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdHighVoltDet {
    #[doc = "Overvoltage detection comparator is enabled"]
    ENABLED = 0x0,
    #[doc = "Overvoltage detection comparator is disabled"]
    DISABLED = 0x01,
}
impl PwdHighVoltDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdHighVoltDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdHighVoltDet {
    #[inline(always)]
    fn from(val: u8) -> PwdHighVoltDet {
        PwdHighVoltDet::from_bits(val)
    }
}
impl From<PwdHighVoltDet> for u8 {
    #[inline(always)]
    fn from(val: PwdHighVoltDet) -> u8 {
        PwdHighVoltDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdOscInt {
    #[doc = "Internal oscillator powered up"]
    POWERED_UP = 0x0,
    #[doc = "Internal oscillator powered down"]
    POWERED_DOWN = 0x01,
}
impl PwdOscInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdOscInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdOscInt {
    #[inline(always)]
    fn from(val: u8) -> PwdOscInt {
        PwdOscInt::from_bits(val)
    }
}
impl From<PwdOscInt> for u8 {
    #[inline(always)]
    fn from(val: PwdOscInt) -> u8 {
        PwdOscInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdOvercurDet {
    #[doc = "Overcurrent detection comparator is enabled"]
    ENABLED = 0x0,
    #[doc = "Overcurrent detection comparator is disabled"]
    DISABLED = 0x01,
}
impl PwdOvercurDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdOvercurDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdOvercurDet {
    #[inline(always)]
    fn from(val: u8) -> PwdOvercurDet {
        PwdOvercurDet::from_bits(val)
    }
}
impl From<PwdOvercurDet> for u8 {
    #[inline(always)]
    fn from(val: PwdOvercurDet) -> u8 {
        PwdOvercurDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdZcd {
    #[doc = "Zero cross detetion function powered up"]
    POWERED_UP = 0x0,
    #[doc = "Zero cross detetion function powered down"]
    POWERED_DOWN = 0x01,
}
impl PwdZcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdZcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdZcd {
    #[inline(always)]
    fn from(val: u8) -> PwdZcd {
        PwdZcd::from_bits(val)
    }
}
impl From<PwdZcd> for u8 {
    #[inline(always)]
    fn from(val: PwdZcd) -> u8 {
        PwdZcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegFbkSel {
    #[doc = "The regulator outputs 1.0V with 1.2V reference voltage"]
    REG_FBK_SEL0 = 0x0,
    #[doc = "The regulator outputs 1.1V with 1.2V reference voltage"]
    REG_FBK_SEL1 = 0x01,
    #[doc = "The regulator outputs 1.0V with 1.3V reference voltage"]
    REG_FBK_SEL2 = 0x02,
    #[doc = "The regulator outputs 1.1V with 1.3V reference voltage"]
    REG_FBK_SEL3 = 0x03,
}
impl RegFbkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegFbkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegFbkSel {
    #[inline(always)]
    fn from(val: u8) -> RegFbkSel {
        RegFbkSel::from_bits(val)
    }
}
impl From<RegFbkSel> for u8 {
    #[inline(always)]
    fn from(val: RegFbkSel) -> u8 {
        RegFbkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegRloadSw {
    #[doc = "Load resistor disconnected"]
    LOAD_R_DISCONNECT = 0x0,
    #[doc = "Load resistor connected"]
    LOAD_R_CONNECT = 0x01,
}
impl RegRloadSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegRloadSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegRloadSw {
    #[inline(always)]
    fn from(val: u8) -> RegRloadSw {
        RegRloadSw::from_bits(val)
    }
}
impl From<RegRloadSw> for u8 {
    #[inline(always)]
    fn from(val: RegRloadSw) -> u8 {
        RegRloadSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelClk {
    #[doc = "DCDC uses internal ring oscillator"]
    INT_RNG_OSC = 0x0,
    #[doc = "DCDC uses 24M xtal"]
    XTAL_24M = 0x01,
}
impl SelClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelClk {
    #[inline(always)]
    fn from(val: u8) -> SelClk {
        SelClk::from_bits(val)
    }
}
impl From<SelClk> for u8 {
    #[inline(always)]
    fn from(val: SelClk) -> u8 {
        SelClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetLp {
    #[doc = "0.9 V"]
    SEL0 = 0x0,
    #[doc = "0.925 V"]
    SEL1 = 0x01,
    #[doc = "0.95 V"]
    SEL2 = 0x02,
    #[doc = "0.975 V"]
    SEL3 = 0x03,
    #[doc = "1.0 V"]
    SEL4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TargetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TargetLp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TargetLp {
    #[inline(always)]
    fn from(val: u8) -> TargetLp {
        TargetLp::from_bits(val)
    }
}
impl From<TargetLp> for u8 {
    #[inline(always)]
    fn from(val: TargetLp) -> u8 {
        TargetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal24mOk {
    #[doc = "DCDC uses internal ring OSC"]
    INT_RNG_OSC = 0x0,
    #[doc = "DCDC uses xtal 24M"]
    XTAL_24M = 0x01,
}
impl Xtal24mOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal24mOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal24mOk {
    #[inline(always)]
    fn from(val: u8) -> Xtal24mOk {
        Xtal24mOk::from_bits(val)
    }
}
impl From<Xtal24mOk> for u8 {
    #[inline(always)]
    fn from(val: Xtal24mOk) -> u8 {
        Xtal24mOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XtalokDisable {
    #[doc = "Enable xtalok detection circuit"]
    ENABLED = 0x0,
    #[doc = "Disable xtalok detection circuit and always outputs OK signal \"1\""]
    DISABLED = 0x01,
}
impl XtalokDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XtalokDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XtalokDisable {
    #[inline(always)]
    fn from(val: u8) -> XtalokDisable {
        XtalokDisable::from_bits(val)
    }
}
impl From<XtalokDisable> for u8 {
    #[inline(always)]
    fn from(val: XtalokDisable) -> u8 {
        XtalokDisable::to_bits(val)
    }
}
