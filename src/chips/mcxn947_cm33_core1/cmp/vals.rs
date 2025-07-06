#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cff {
    #[doc = "Not detected"]
    NOT_DETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Cff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cff {
    #[inline(always)]
    fn from(val: u8) -> Cff {
        Cff::from_bits(val)
    }
}
impl From<Cff> for u8 {
    #[inline(always)]
    fn from(val: Cff) -> u8 {
        Cff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CffIe {
    #[doc = "Disables the comparator flag falling interrupt."]
    DISABLE = 0x0,
    #[doc = "Enables the comparator flag falling interrupt when CFF is set."]
    ENABLE = 0x01,
}
impl CffIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CffIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CffIe {
    #[inline(always)]
    fn from(val: u8) -> CffIe {
        CffIe::from_bits(val)
    }
}
impl From<CffIe> for u8 {
    #[inline(always)]
    fn from(val: CffIe) -> u8 {
        CffIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfr {
    #[doc = "Not detected"]
    NOT_DETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Cfr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfr {
    #[inline(always)]
    fn from(val: u8) -> Cfr {
        Cfr::from_bits(val)
    }
}
impl From<Cfr> for u8 {
    #[inline(always)]
    fn from(val: Cfr) -> u8 {
        Cfr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CfrIe {
    #[doc = "Disables the comparator flag rising interrupt."]
    DISABLE = 0x0,
    #[doc = "Enables the comparator flag rising interrupt when CFR is set."]
    ENABLE = 0x01,
}
impl CfrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CfrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CfrIe {
    #[inline(always)]
    fn from(val: u8) -> CfrIe {
        CfrIe::from_bits(val)
    }
}
impl From<CfrIe> for u8 {
    #[inline(always)]
    fn from(val: CfrIe) -> u8 {
        CfrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpEn {
    #[doc = "Disable (The analog logic remains off and consumes no power.)"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CmpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpEn {
    #[inline(always)]
    fn from(val: u8) -> CmpEn {
        CmpEn::from_bits(val)
    }
}
impl From<CmpEn> for u8 {
    #[inline(always)]
    fn from(val: CmpEn) -> u8 {
        CmpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpHpmd {
    #[doc = "Low power (speed) comparison mode"]
    LOW = 0x0,
    #[doc = "High power (speed) comparison mode"]
    HIGH = 0x01,
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
pub enum CmpNpmd {
    #[doc = "Disables CMP Nano power mode. CCR2\\[CMP_HPMD\\] determines the mode for the comparator."]
    NO_NANO = 0x0,
    #[doc = "Enables CMP Nano power mode."]
    NANO = 0x01,
}
impl CmpNpmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpNpmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpNpmd {
    #[inline(always)]
    fn from(val: u8) -> CmpNpmd {
        CmpNpmd::from_bits(val)
    }
}
impl From<CmpNpmd> for u8 {
    #[inline(always)]
    fn from(val: CmpNpmd) -> u8 {
        CmpNpmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutInv {
    #[doc = "Do not invert"]
    NO_INVERT = 0x0,
    #[doc = "Invert"]
    INVERT = 0x01,
}
impl CoutInv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutInv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutInv {
    #[inline(always)]
    fn from(val: u8) -> CoutInv {
        CoutInv::from_bits(val)
    }
}
impl From<CoutInv> for u8 {
    #[inline(always)]
    fn from(val: CoutInv) -> u8 {
        CoutInv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutPen {
    #[doc = "Not available"]
    UNAVAILABLE = 0x0,
    #[doc = "Available"]
    AVAILABLE = 0x01,
}
impl CoutPen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutPen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutPen {
    #[inline(always)]
    fn from(val: u8) -> CoutPen {
        CoutPen::from_bits(val)
    }
}
impl From<CoutPen> for u8 {
    #[inline(always)]
    fn from(val: CoutPen) -> u8 {
        CoutPen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutSel {
    #[doc = "Use COUT (filtered)"]
    COUT = 0x0,
    #[doc = "Use COUTA (unfiltered)"]
    COUTA = 0x01,
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
    #[doc = "COUTA is 0"]
    COUTA_0 = 0x0,
    #[doc = "COUTA is 1"]
    COUTA_1 = 0x01,
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
    SAMPLED = 0x0,
    #[doc = "Enables the COUTA signal value to be defined by COUTA_OW."]
    COUTA_OW = 0x01,
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
pub enum DacEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DacEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacEn {
    #[inline(always)]
    fn from(val: u8) -> DacEn {
        DacEn::from_bits(val)
    }
}
impl From<DacEn> for u8 {
    #[inline(always)]
    fn from(val: DacEn) -> u8 {
        DacEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacHpmd {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DacHpmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacHpmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacHpmd {
    #[inline(always)]
    fn from(val: u8) -> DacHpmd {
        DacHpmd::from_bits(val)
    }
}
impl From<DacHpmd> for u8 {
    #[inline(always)]
    fn from(val: DacHpmd) -> u8 {
        DacHpmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacRes {
    #[doc = "4-bit DAC"]
    RESO_4 = 0x0,
    #[doc = "6-bit DAC"]
    RESO_6 = 0x01,
    #[doc = "8-bit DAC"]
    RESO_8 = 0x02,
    #[doc = "10-bit DAC"]
    RESO_10 = 0x03,
    #[doc = "12-bit DAC"]
    RESO_12 = 0x04,
    #[doc = "14-bit DAC"]
    RESO_14 = 0x05,
    #[doc = "16-bit DAC"]
    RESO_16 = 0x06,
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
pub enum DmaEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DmaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaEn {
    #[inline(always)]
    fn from(val: u8) -> DmaEn {
        DmaEn::from_bits(val)
    }
}
impl From<DmaEn> for u8 {
    #[inline(always)]
    fn from(val: DmaEn) -> u8 {
        DmaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtSel {
    #[doc = "Rising edge"]
    RISING = 0x0,
    #[doc = "Falling edge"]
    FALLING = 0x01,
    #[doc = "Both edges"]
    BOTH = 0x02,
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
    #[doc = "Round robin feature"]
    pub const ROUND_ROBIN: Self = Self(0x01);
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
            0x01 => f.write_str("ROUND_ROBIN"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "ROUND_ROBIN"),
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
    #[doc = "Filter is bypassed: COUT = COUTA"]
    BYPASSED = 0x0,
    #[doc = "1 consecutive sample (Comparator output is simply sampled.)"]
    SAMPLE_1 = 0x01,
    #[doc = "2 consecutive samples"]
    SAMPLE_2 = 0x02,
    #[doc = "3 consecutive samples"]
    SAMPLE_3 = 0x03,
    #[doc = "4 consecutive samples"]
    SAMPLE_4 = 0x04,
    #[doc = "5 consecutive samples"]
    SAMPLE_5 = 0x05,
    #[doc = "6 consecutive samples"]
    SAMPLE_6 = 0x06,
    #[doc = "7 consecutive samples"]
    SAMPLE_7 = 0x07,
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
    #[doc = "Channel 0"]
    FIX_CH0 = 0x0,
    #[doc = "Channel 1"]
    FIX_CH1 = 0x01,
    #[doc = "Channel 2"]
    FIX_CH2 = 0x02,
    #[doc = "Channel 3"]
    FIX_CH3 = 0x03,
    #[doc = "Channel 4"]
    FIX_CH4 = 0x04,
    #[doc = "Channel 5"]
    FIX_CH5 = 0x05,
    #[doc = "Channel 6"]
    FIX_CH6 = 0x06,
    #[doc = "Channel 7"]
    FIX_CH7 = 0x07,
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
    FIX_PLUS = 0x0,
    #[doc = "Fix the minus port. Sweep only the inputs to the plus port."]
    FIX_MINUS = 0x01,
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
    #[doc = "Select functional clock source 0"]
    FUNC0 = 0x0,
    #[doc = "Select functional clock source 1"]
    FUNC1 = 0x01,
    #[doc = "Select functional clock source 2"]
    FUNC2 = 0x02,
    #[doc = "Select functional clock source 3"]
    FUNC3 = 0x03,
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
    LEVEL_0 = 0x0,
    #[doc = "Level 1: Analog comparator hysteresis 10 mV."]
    LEVEL_1 = 0x01,
    #[doc = "Level 2: Analog comparator hysteresis 20 mV."]
    LEVEL_2 = 0x02,
    #[doc = "Level 3: Analog comparator hysteresis 30 mV."]
    LEVEL_3 = 0x03,
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
    #[doc = "Input 0m"]
    INPUT_0 = 0x0,
    #[doc = "Input 1m"]
    INPUT_1 = 0x01,
    #[doc = "Input 2m"]
    INPUT_2 = 0x02,
    #[doc = "Input 3m"]
    INPUT_3 = 0x03,
    #[doc = "Input 4m"]
    INPUT_4 = 0x04,
    #[doc = "Input 5m"]
    INPUT_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output"]
    INPUT_7 = 0x07,
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
    #[doc = "Input 0p"]
    INPUT_0 = 0x0,
    #[doc = "Input 1p"]
    INPUT_1 = 0x01,
    #[doc = "Input 2p"]
    INPUT_2 = 0x02,
    #[doc = "Input 3p"]
    INPUT_3 = 0x03,
    #[doc = "Input 4p"]
    INPUT_4 = 0x04,
    #[doc = "Input 5p"]
    INPUT_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output"]
    INPUT_7 = 0x07,
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
pub enum RrCh0en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh0en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh0en {
    #[inline(always)]
    fn from(val: u8) -> RrCh0en {
        RrCh0en::from_bits(val)
    }
}
impl From<RrCh0en> for u8 {
    #[inline(always)]
    fn from(val: RrCh0en) -> u8 {
        RrCh0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh0f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh0f {
    #[inline(always)]
    fn from(val: u8) -> RrCh0f {
        RrCh0f::from_bits(val)
    }
}
impl From<RrCh0f> for u8 {
    #[inline(always)]
    fn from(val: RrCh0f) -> u8 {
        RrCh0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh1en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh1en {
    #[inline(always)]
    fn from(val: u8) -> RrCh1en {
        RrCh1en::from_bits(val)
    }
}
impl From<RrCh1en> for u8 {
    #[inline(always)]
    fn from(val: RrCh1en) -> u8 {
        RrCh1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh1f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh1f {
    #[inline(always)]
    fn from(val: u8) -> RrCh1f {
        RrCh1f::from_bits(val)
    }
}
impl From<RrCh1f> for u8 {
    #[inline(always)]
    fn from(val: RrCh1f) -> u8 {
        RrCh1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh2en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh2en {
    #[inline(always)]
    fn from(val: u8) -> RrCh2en {
        RrCh2en::from_bits(val)
    }
}
impl From<RrCh2en> for u8 {
    #[inline(always)]
    fn from(val: RrCh2en) -> u8 {
        RrCh2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh2f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh2f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh2f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh2f {
    #[inline(always)]
    fn from(val: u8) -> RrCh2f {
        RrCh2f::from_bits(val)
    }
}
impl From<RrCh2f> for u8 {
    #[inline(always)]
    fn from(val: RrCh2f) -> u8 {
        RrCh2f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh3en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh3en {
    #[inline(always)]
    fn from(val: u8) -> RrCh3en {
        RrCh3en::from_bits(val)
    }
}
impl From<RrCh3en> for u8 {
    #[inline(always)]
    fn from(val: RrCh3en) -> u8 {
        RrCh3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh3f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh3f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh3f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh3f {
    #[inline(always)]
    fn from(val: u8) -> RrCh3f {
        RrCh3f::from_bits(val)
    }
}
impl From<RrCh3f> for u8 {
    #[inline(always)]
    fn from(val: RrCh3f) -> u8 {
        RrCh3f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh4en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh4en {
    #[inline(always)]
    fn from(val: u8) -> RrCh4en {
        RrCh4en::from_bits(val)
    }
}
impl From<RrCh4en> for u8 {
    #[inline(always)]
    fn from(val: RrCh4en) -> u8 {
        RrCh4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh4f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh4f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh4f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh4f {
    #[inline(always)]
    fn from(val: u8) -> RrCh4f {
        RrCh4f::from_bits(val)
    }
}
impl From<RrCh4f> for u8 {
    #[inline(always)]
    fn from(val: RrCh4f) -> u8 {
        RrCh4f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh5en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh5en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh5en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh5en {
    #[inline(always)]
    fn from(val: u8) -> RrCh5en {
        RrCh5en::from_bits(val)
    }
}
impl From<RrCh5en> for u8 {
    #[inline(always)]
    fn from(val: RrCh5en) -> u8 {
        RrCh5en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh5f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh5f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh5f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh5f {
    #[inline(always)]
    fn from(val: u8) -> RrCh5f {
        RrCh5f::from_bits(val)
    }
}
impl From<RrCh5f> for u8 {
    #[inline(always)]
    fn from(val: RrCh5f) -> u8 {
        RrCh5f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh6en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh6en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh6en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh6en {
    #[inline(always)]
    fn from(val: u8) -> RrCh6en {
        RrCh6en::from_bits(val)
    }
}
impl From<RrCh6en> for u8 {
    #[inline(always)]
    fn from(val: RrCh6en) -> u8 {
        RrCh6en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh6f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh6f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh6f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh6f {
    #[inline(always)]
    fn from(val: u8) -> RrCh6f {
        RrCh6f::from_bits(val)
    }
}
impl From<RrCh6f> for u8 {
    #[inline(always)]
    fn from(val: RrCh6f) -> u8 {
        RrCh6f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh7en {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrCh7en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh7en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh7en {
    #[inline(always)]
    fn from(val: u8) -> RrCh7en {
        RrCh7en::from_bits(val)
    }
}
impl From<RrCh7en> for u8 {
    #[inline(always)]
    fn from(val: RrCh7en) -> u8 {
        RrCh7en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrCh7f {
    #[doc = "No different"]
    NOT_DIFFERENT = 0x0,
    #[doc = "Different"]
    DIFFERENT = 0x01,
}
impl RrCh7f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrCh7f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrCh7f {
    #[inline(always)]
    fn from(val: u8) -> RrCh7f {
        RrCh7f::from_bits(val)
    }
}
impl From<RrCh7f> for u8 {
    #[inline(always)]
    fn from(val: RrCh7f) -> u8 {
        RrCh7f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkSel {
    #[doc = "Select Round Robin clock Source 0"]
    RR0 = 0x0,
    #[doc = "Select Round Robin clock Source 1"]
    RR1 = 0x01,
    #[doc = "Select Round Robin clock Source 2"]
    RR2 = 0x02,
    #[doc = "Select Round Robin clock Source 3"]
    RR3 = 0x03,
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
pub enum RrEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrEn {
    #[inline(always)]
    fn from(val: u8) -> RrEn {
        RrEn::from_bits(val)
    }
}
impl From<RrEn> for u8 {
    #[inline(always)]
    fn from(val: RrEn) -> u8 {
        RrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrInitmod {
    #[doc = "63 cycles (same as 111111b)"]
    MOD_63 = 0x0,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_1 = 0x01,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_2 = 0x02,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_3 = 0x03,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_4 = 0x04,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_5 = 0x05,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_6 = 0x06,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_7 = 0x07,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_8 = 0x08,
    #[doc = "1 to 63 cycles"]
    MOD_1_63_9 = 0x09,
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
    #[doc = "0 clock"]
    WAIT_0 = 0x0,
    #[doc = "1 clock"]
    WAIT_1 = 0x01,
    #[doc = "2 clocks"]
    WAIT_2 = 0x02,
    #[doc = "3 clocks"]
    WAIT_3 = 0x03,
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
    #[doc = "1 samples"]
    SAMPLE_0 = 0x0,
    #[doc = "2 samples"]
    SAMPLE_1 = 0x01,
    #[doc = "3 samples"]
    SAMPLE_2 = 0x02,
    #[doc = "4 samples"]
    SAMPLE_3 = 0x03,
    #[doc = "5 samples"]
    SAMPLE_4 = 0x04,
    #[doc = "6 samples"]
    SAMPLE_5 = 0x05,
    #[doc = "7 samples"]
    SAMPLE_6 = 0x06,
    #[doc = "8 samples"]
    SAMPLE_7 = 0x07,
    #[doc = "9 samples"]
    SAMPLE_8 = 0x08,
    #[doc = "10 samples"]
    SAMPLE_9 = 0x09,
    #[doc = "11 samples"]
    SAMPLE_10 = 0x0a,
    #[doc = "12 samples"]
    SAMPLE_11 = 0x0b,
    #[doc = "13 samples"]
    SAMPLE_12 = 0x0c,
    #[doc = "14 samples"]
    SAMPLE_13 = 0x0d,
    #[doc = "15 samples"]
    SAMPLE_14 = 0x0e,
    #[doc = "16 samples"]
    SAMPLE_15 = 0x0f,
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
    #[doc = "At least 1 sampled \"1\", the final result is \"1\""]
    SAMPLE_0 = 0x0,
    #[doc = "At least 2 sampled \"1\", the final result is \"1\""]
    SAMPLE_1 = 0x01,
    #[doc = "At least 3 sampled \"1\", the final result is \"1\""]
    SAMPLE_2 = 0x02,
    #[doc = "At least 4 sampled \"1\", the final result is \"1\""]
    SAMPLE_3 = 0x03,
    #[doc = "At least 5 sampled \"1\", the final result is \"1\""]
    SAMPLE_4 = 0x04,
    #[doc = "At least 6 sampled \"1\", the final result is \"1\""]
    SAMPLE_5 = 0x05,
    #[doc = "At least 7 sampled \"1\", the final result is \"1\""]
    SAMPLE_6 = 0x06,
    #[doc = "At least 8 sampled \"1\", the final result is \"1\""]
    SAMPLE_7 = 0x07,
    #[doc = "At least 9 sampled \"1\", the final result is \"1\""]
    SAMPLE_8 = 0x08,
    #[doc = "At least 10 sampled \"1\", the final result is \"1\""]
    SAMPLE_9 = 0x09,
    #[doc = "At least 11 sampled \"1\", the final result is \"1\""]
    SAMPLE_10 = 0x0a,
    #[doc = "At least 12 sampled \"1\", the final result is \"1\""]
    SAMPLE_11 = 0x0b,
    #[doc = "At least 13 sampled \"1\", the final result is \"1\""]
    SAMPLE_12 = 0x0c,
    #[doc = "At least 14 sampled \"1\", the final result is \"1\""]
    SAMPLE_13 = 0x0d,
    #[doc = "At least 15 sampled \"1\", the final result is \"1\""]
    SAMPLE_14 = 0x0e,
    #[doc = "At least 16 sampled \"1\", the final result is \"1\""]
    SAMPLE_15 = 0x0f,
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
pub enum RrTimerEn {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl RrTimerEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrTimerEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrTimerEn {
    #[inline(always)]
    fn from(val: u8) -> RrTimerEn {
        RrTimerEn::from_bits(val)
    }
}
impl From<RrTimerEn> for u8 {
    #[inline(always)]
    fn from(val: RrTimerEn) -> u8 {
        RrTimerEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrTrgSel {
    #[doc = "External trigger"]
    ENABLE = 0x0,
    #[doc = "Internal trigger"]
    DISABLE = 0x01,
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
pub enum Rrf {
    #[doc = "Not detected"]
    NOT_DETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Rrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrf {
    #[inline(always)]
    fn from(val: u8) -> Rrf {
        Rrf::from_bits(val)
    }
}
impl From<Rrf> for u8 {
    #[inline(always)]
    fn from(val: Rrf) -> u8 {
        Rrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrfIe {
    #[doc = "Disables the round-robin flag interrupt."]
    DISABLE = 0x0,
    #[doc = "Enables the round-robin flag interrupt when the comparison result changes for a given channel."]
    ENABLE = 0x01,
}
impl RrfIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrfIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrfIe {
    #[inline(always)]
    fn from(val: u8) -> RrfIe {
        RrfIe::from_bits(val)
    }
}
impl From<RrfIe> for u8 {
    #[inline(always)]
    fn from(val: RrfIe) -> u8 {
        RrfIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SampleEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SampleEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SampleEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SampleEn {
    #[inline(always)]
    fn from(val: u8) -> SampleEn {
        SampleEn::from_bits(val)
    }
}
impl From<SampleEn> for u8 {
    #[inline(always)]
    fn from(val: SampleEn) -> u8 {
        SampleEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "VREFH0"]
    VREF0 = 0x0,
    #[doc = "VREFH1"]
    VREF1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WindowCls {
    #[doc = "COUT event cannot close the window"]
    NO_CLOSE = 0x0,
    #[doc = "COUT event can close the window"]
    CLOSE = 0x01,
}
impl WindowCls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WindowCls {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WindowCls {
    #[inline(always)]
    fn from(val: u8) -> WindowCls {
        WindowCls::from_bits(val)
    }
}
impl From<WindowCls> for u8 {
    #[inline(always)]
    fn from(val: WindowCls) -> u8 {
        WindowCls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WindowEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl WindowEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WindowEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WindowEn {
    #[inline(always)]
    fn from(val: u8) -> WindowEn {
        WindowEn::from_bits(val)
    }
}
impl From<WindowEn> for u8 {
    #[inline(always)]
    fn from(val: WindowEn) -> u8 {
        WindowEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WindowInv {
    #[doc = "Do not invert"]
    NO_INVERT = 0x0,
    #[doc = "Invert"]
    INVERT = 0x01,
}
impl WindowInv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WindowInv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WindowInv {
    #[inline(always)]
    fn from(val: u8) -> WindowInv {
        WindowInv::from_bits(val)
    }
}
impl From<WindowInv> for u8 {
    #[inline(always)]
    fn from(val: WindowInv) -> u8 {
        WindowInv::to_bits(val)
    }
}
