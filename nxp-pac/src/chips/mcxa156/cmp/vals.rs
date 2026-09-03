#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpHpmd {
    #[doc = "Low power (speed) comparison mode."]
    Low = 0x0,
    #[doc = "High power (speed) comparison mode."]
    High = 0x01,
}
impl CmpHpmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpHpmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpHpmd {
    #[inline(always)]
    fn from(val: u8) -> CmpHpmd {
        CmpHpmd::from_bits(val)
    }
}
impl From<CmpHpmd> for u8 {
    #[inline(always)]
    fn from(val: CmpHpmd) -> u8 {
        CmpHpmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutSel {
    #[doc = "Use COUT (filtered)."]
    Cout = 0x0,
    #[doc = "Use COUTA (unfiltered)."]
    Couta = 0x01,
}
impl CoutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutSel {
    #[inline(always)]
    fn from(val: u8) -> CoutSel {
        CoutSel::from_bits(val)
    }
}
impl From<CoutSel> for u8 {
    #[inline(always)]
    fn from(val: CoutSel) -> u8 {
        CoutSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutaOw {
    #[doc = "COUTA is 0."]
    Couta0 = 0x0,
    #[doc = "COUTA is 1."]
    Couta1 = 0x01,
}
impl CoutaOw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutaOw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutaOw {
    #[inline(always)]
    fn from(val: u8) -> CoutaOw {
        CoutaOw::from_bits(val)
    }
}
impl From<CoutaOw> for u8 {
    #[inline(always)]
    fn from(val: CoutaOw) -> u8 {
        CoutaOw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutaOwen {
    #[doc = "COUTA holds the last sampled value."]
    Sampled = 0x0,
    #[doc = "Enables the COUTA signal value to be defined by COUTA_OW."]
    CoutaOw = 0x01,
}
impl CoutaOwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutaOwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutaOwen {
    #[inline(always)]
    fn from(val: u8) -> CoutaOwen {
        CoutaOwen::from_bits(val)
    }
}
impl From<CoutaOwen> for u8 {
    #[inline(always)]
    fn from(val: CoutaOwen) -> u8 {
        CoutaOwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacRes {
    #[doc = "4-bit DAC."]
    Reso4 = 0x0,
    #[doc = "6-bit DAC."]
    Reso6 = 0x01,
    #[doc = "8-bit DAC."]
    Reso8 = 0x02,
    #[doc = "10-bit DAC."]
    Reso10 = 0x03,
    #[doc = "12-bit DAC."]
    Reso12 = 0x04,
    #[doc = "14-bit DAC."]
    Reso14 = 0x05,
    #[doc = "16-bit DAC."]
    Reso16 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DacRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacRes {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacRes {
    #[inline(always)]
    fn from(val: u8) -> DacRes {
        DacRes::from_bits(val)
    }
}
impl From<DacRes> for u8 {
    #[inline(always)]
    fn from(val: DacRes) -> u8 {
        DacRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtSel {
    #[doc = "Rising edge."]
    Rising = 0x0,
    #[doc = "Falling edge."]
    Falling = 0x01,
    #[doc = "Both edges."]
    Both = 0x02,
    _RESERVED_3 = 0x03,
}
impl EvtSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtSel {
    #[inline(always)]
    fn from(val: u8) -> EvtSel {
        EvtSel::from_bits(val)
    }
}
impl From<EvtSel> for u8 {
    #[inline(always)]
    fn from(val: EvtSel) -> u8 {
        EvtSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Round robin feature."]
    pub const RoundRobin: Self = Self(0x01);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("RoundRobin"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "RoundRobin"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltCnt {
    #[doc = "Filter is bypassed: COUT = COUTA."]
    Bypassed = 0x0,
    #[doc = "1 consecutive sample (Comparator output is simply sampled.)."]
    Sample1 = 0x01,
    #[doc = "2 consecutive samples."]
    Sample2 = 0x02,
    #[doc = "3 consecutive samples."]
    Sample3 = 0x03,
    #[doc = "4 consecutive samples."]
    Sample4 = 0x04,
    #[doc = "5 consecutive samples."]
    Sample5 = 0x05,
    #[doc = "6 consecutive samples."]
    Sample6 = 0x06,
    #[doc = "7 consecutive samples."]
    Sample7 = 0x07,
}
impl FiltCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltCnt {
    #[inline(always)]
    fn from(val: u8) -> FiltCnt {
        FiltCnt::from_bits(val)
    }
}
impl From<FiltCnt> for u8 {
    #[inline(always)]
    fn from(val: FiltCnt) -> u8 {
        FiltCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fixch {
    #[doc = "Channel 0."]
    FixCh0 = 0x0,
    #[doc = "Channel 1."]
    FixCh1 = 0x01,
    #[doc = "Channel 2."]
    FixCh2 = 0x02,
    #[doc = "Channel 3."]
    FixCh3 = 0x03,
    #[doc = "Channel 4."]
    FixCh4 = 0x04,
    #[doc = "Channel 5."]
    FixCh5 = 0x05,
    #[doc = "Channel 6."]
    FixCh6 = 0x06,
    #[doc = "Channel 7."]
    FixCh7 = 0x07,
}
impl Fixch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fixch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fixch {
    #[inline(always)]
    fn from(val: u8) -> Fixch {
        Fixch::from_bits(val)
    }
}
impl From<Fixch> for u8 {
    #[inline(always)]
    fn from(val: Fixch) -> u8 {
        Fixch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fixp {
    #[doc = "Fix the plus port. Sweep only the inputs to the minus port."]
    FixPlus = 0x0,
    #[doc = "Fix the minus port. Sweep only the inputs to the plus port."]
    FixMinus = 0x01,
}
impl Fixp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fixp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fixp {
    #[inline(always)]
    fn from(val: u8) -> Fixp {
        Fixp::from_bits(val)
    }
}
impl From<Fixp> for u8 {
    #[inline(always)]
    fn from(val: Fixp) -> u8 {
        Fixp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FuncClkSel {
    #[doc = "Select functional clock source 0."]
    Func0 = 0x0,
    #[doc = "Select functional clock source 1."]
    Func1 = 0x01,
    #[doc = "Select functional clock source 2."]
    Func2 = 0x02,
    #[doc = "Select functional clock source 3."]
    Func3 = 0x03,
}
impl FuncClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FuncClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FuncClkSel {
    #[inline(always)]
    fn from(val: u8) -> FuncClkSel {
        FuncClkSel::from_bits(val)
    }
}
impl From<FuncClkSel> for u8 {
    #[inline(always)]
    fn from(val: FuncClkSel) -> u8 {
        FuncClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hystctr {
    #[doc = "Level 0: Analog comparator hysteresis 0 mV."]
    Level0 = 0x0,
    #[doc = "Level 1: Analog comparator hysteresis 10 mV."]
    Level1 = 0x01,
    #[doc = "Level 2: Analog comparator hysteresis 20 mV."]
    Level2 = 0x02,
    #[doc = "Level 3: Analog comparator hysteresis 30 mV."]
    Level3 = 0x03,
}
impl Hystctr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hystctr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hystctr {
    #[inline(always)]
    fn from(val: u8) -> Hystctr {
        Hystctr::from_bits(val)
    }
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(val: Hystctr) -> u8 {
        Hystctr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "Input 0m."]
    Input0 = 0x0,
    #[doc = "Input 1m."]
    Input1 = 0x01,
    #[doc = "Input 2m."]
    Input2 = 0x02,
    #[doc = "Input 3m."]
    Input3 = 0x03,
    #[doc = "Input 4m."]
    Input4 = 0x04,
    #[doc = "Input 5m."]
    Input5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output."]
    Input7 = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "Input 0p."]
    Input0 = 0x0,
    #[doc = "Input 1p."]
    Input1 = 0x01,
    #[doc = "Input 2p."]
    Input2 = 0x02,
    #[doc = "Input 3p."]
    Input3 = 0x03,
    #[doc = "Input 4p."]
    Input4 = 0x04,
    #[doc = "Input 5p."]
    Input5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output."]
    Input7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkSel {
    #[doc = "Select Round Robin clock Source 0."]
    Rr0 = 0x0,
    #[doc = "Select Round Robin clock Source 1."]
    Rr1 = 0x01,
    #[doc = "Select Round Robin clock Source 2."]
    Rr2 = 0x02,
    #[doc = "Select Round Robin clock Source 3."]
    Rr3 = 0x03,
}
impl RrClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrClkSel {
    #[inline(always)]
    fn from(val: u8) -> RrClkSel {
        RrClkSel::from_bits(val)
    }
}
impl From<RrClkSel> for u8 {
    #[inline(always)]
    fn from(val: RrClkSel) -> u8 {
        RrClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrInitmod {
    #[doc = "63 cycles (same as 111111b)."]
    Mod63 = 0x0,
    #[doc = "1 to 63 cycles."]
    Mod1631 = 0x01,
    #[doc = "1 to 63 cycles."]
    Mod1632 = 0x02,
    #[doc = "1 to 63 cycles."]
    Mod1633 = 0x03,
    #[doc = "1 to 63 cycles."]
    Mod1634 = 0x04,
    #[doc = "1 to 63 cycles."]
    Mod1635 = 0x05,
    #[doc = "1 to 63 cycles."]
    Mod1636 = 0x06,
    #[doc = "1 to 63 cycles."]
    Mod1637 = 0x07,
    #[doc = "1 to 63 cycles."]
    Mod1638 = 0x08,
    #[doc = "1 to 63 cycles."]
    Mod1639 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl RrInitmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrInitmod {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrInitmod {
    #[inline(always)]
    fn from(val: u8) -> RrInitmod {
        RrInitmod::from_bits(val)
    }
}
impl From<RrInitmod> for u8 {
    #[inline(always)]
    fn from(val: RrInitmod) -> u8 {
        RrInitmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrNsam {
    #[doc = "0 clock."]
    Wait0 = 0x0,
    #[doc = "1 clock."]
    Wait1 = 0x01,
    #[doc = "2 clocks."]
    Wait2 = 0x02,
    #[doc = "3 clocks."]
    Wait3 = 0x03,
}
impl RrNsam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrNsam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrNsam {
    #[inline(always)]
    fn from(val: u8) -> RrNsam {
        RrNsam::from_bits(val)
    }
}
impl From<RrNsam> for u8 {
    #[inline(always)]
    fn from(val: RrNsam) -> u8 {
        RrNsam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrSampleCnt {
    #[doc = "1 samples."]
    Sample0 = 0x0,
    #[doc = "2 samples."]
    Sample1 = 0x01,
    #[doc = "3 samples."]
    Sample2 = 0x02,
    #[doc = "4 samples."]
    Sample3 = 0x03,
    #[doc = "5 samples."]
    Sample4 = 0x04,
    #[doc = "6 samples."]
    Sample5 = 0x05,
    #[doc = "7 samples."]
    Sample6 = 0x06,
    #[doc = "8 samples."]
    Sample7 = 0x07,
    #[doc = "9 samples."]
    Sample8 = 0x08,
    #[doc = "10 samples."]
    Sample9 = 0x09,
    #[doc = "11 samples."]
    Sample10 = 0x0a,
    #[doc = "12 samples."]
    Sample11 = 0x0b,
    #[doc = "13 samples."]
    Sample12 = 0x0c,
    #[doc = "14 samples."]
    Sample13 = 0x0d,
    #[doc = "15 samples."]
    Sample14 = 0x0e,
    #[doc = "16 samples."]
    Sample15 = 0x0f,
}
impl RrSampleCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrSampleCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrSampleCnt {
    #[inline(always)]
    fn from(val: u8) -> RrSampleCnt {
        RrSampleCnt::from_bits(val)
    }
}
impl From<RrSampleCnt> for u8 {
    #[inline(always)]
    fn from(val: RrSampleCnt) -> u8 {
        RrSampleCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrSampleThreshold {
    #[doc = "At least 1 sampled \"1\", the final result is \"1\"."]
    Sample0 = 0x0,
    #[doc = "At least 2 sampled \"1\", the final result is \"1\"."]
    Sample1 = 0x01,
    #[doc = "At least 3 sampled \"1\", the final result is \"1\"."]
    Sample2 = 0x02,
    #[doc = "At least 4 sampled \"1\", the final result is \"1\"."]
    Sample3 = 0x03,
    #[doc = "At least 5 sampled \"1\", the final result is \"1\"."]
    Sample4 = 0x04,
    #[doc = "At least 6 sampled \"1\", the final result is \"1\"."]
    Sample5 = 0x05,
    #[doc = "At least 7 sampled \"1\", the final result is \"1\"."]
    Sample6 = 0x06,
    #[doc = "At least 8 sampled \"1\", the final result is \"1\"."]
    Sample7 = 0x07,
    #[doc = "At least 9 sampled \"1\", the final result is \"1\"."]
    Sample8 = 0x08,
    #[doc = "At least 10 sampled \"1\", the final result is \"1\"."]
    Sample9 = 0x09,
    #[doc = "At least 11 sampled \"1\", the final result is \"1\"."]
    Sample10 = 0x0a,
    #[doc = "At least 12 sampled \"1\", the final result is \"1\"."]
    Sample11 = 0x0b,
    #[doc = "At least 13 sampled \"1\", the final result is \"1\"."]
    Sample12 = 0x0c,
    #[doc = "At least 14 sampled \"1\", the final result is \"1\"."]
    Sample13 = 0x0d,
    #[doc = "At least 15 sampled \"1\", the final result is \"1\"."]
    Sample14 = 0x0e,
    #[doc = "At least 16 sampled \"1\", the final result is \"1\"."]
    Sample15 = 0x0f,
}
impl RrSampleThreshold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrSampleThreshold {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrSampleThreshold {
    #[inline(always)]
    fn from(val: u8) -> RrSampleThreshold {
        RrSampleThreshold::from_bits(val)
    }
}
impl From<RrSampleThreshold> for u8 {
    #[inline(always)]
    fn from(val: RrSampleThreshold) -> u8 {
        RrSampleThreshold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrTrgSel {
    #[doc = "External trigger."]
    Enable = 0x0,
    #[doc = "Internal trigger."]
    Disable = 0x01,
}
impl RrTrgSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrTrgSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrTrgSel {
    #[inline(always)]
    fn from(val: u8) -> RrTrgSel {
        RrTrgSel::from_bits(val)
    }
}
impl From<RrTrgSel> for u8 {
    #[inline(always)]
    fn from(val: RrTrgSel) -> u8 {
        RrTrgSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "VREFH0."]
    Vref0 = 0x0,
    #[doc = "VREFH1."]
    Vref1 = 0x01,
}
impl Vrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrsel {
    #[inline(always)]
    fn from(val: u8) -> Vrsel {
        Vrsel::from_bits(val)
    }
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(val: Vrsel) -> u8 {
        Vrsel::to_bits(val)
    }
}
