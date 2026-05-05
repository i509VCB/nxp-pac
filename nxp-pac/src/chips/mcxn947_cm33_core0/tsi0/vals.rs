#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseNochargeNum {
    #[doc = "1."]
    Ssc1 = 0x0,
    #[doc = "2."]
    Ssc2 = 0x01,
    #[doc = "3."]
    Ssc3 = 0x02,
    #[doc = "4."]
    Ssc4 = 0x03,
    #[doc = "5."]
    Ssc5 = 0x04,
    #[doc = "6."]
    Ssc6 = 0x05,
    #[doc = "7."]
    Ssc7 = 0x06,
    #[doc = "8."]
    Ssc8 = 0x07,
    #[doc = "9."]
    Ssc9 = 0x08,
    #[doc = "10."]
    Ssc10 = 0x09,
    #[doc = "11."]
    Ssc11 = 0x0a,
    #[doc = "12."]
    Ssc12 = 0x0b,
    #[doc = "13."]
    Ssc13 = 0x0c,
    #[doc = "14."]
    Ssc14 = 0x0d,
    #[doc = "15."]
    Ssc15 = 0x0e,
    #[doc = "16."]
    Ssc16 = 0x0f,
}
impl BaseNochargeNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseNochargeNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseNochargeNum {
    #[inline(always)]
    fn from(val: u8) -> BaseNochargeNum {
        BaseNochargeNum::from_bits(val)
    }
}
impl From<BaseNochargeNum> for u8 {
    #[inline(always)]
    fn from(val: BaseNochargeNum) -> u8 {
        BaseNochargeNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseTraceDebounce {
    #[doc = "0."]
    Ctr0 = 0x0,
    #[doc = "1 / 16."]
    Ctr1 = 0x01,
    #[doc = "2 / 16."]
    Ctr2 = 0x02,
    #[doc = "3 / 16."]
    Ctr3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "n / 16."]
    CtrN = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl BaseTraceDebounce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseTraceDebounce {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseTraceDebounce {
    #[inline(always)]
    fn from(val: u8) -> BaseTraceDebounce {
        BaseTraceDebounce::from_bits(val)
    }
}
impl From<BaseTraceDebounce> for u8 {
    #[inline(always)]
    fn from(val: BaseTraceDebounce) -> u8 {
        BaseTraceDebounce::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelEnable(u32);
impl ChannelEnable {
    #[doc = "Channel not chosen for proximity pad."]
    pub const Notchosen: Self = Self(0x0);
    #[doc = "Channel chosen for proximity pad."]
    pub const Chosen: Self = Self(0x01);
}
impl ChannelEnable {
    pub const fn from_bits(val: u32) -> ChannelEnable {
        Self(val & 0x01ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for ChannelEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Notchosen"),
            0x01 => f.write_str("Chosen"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Notchosen"),
            0x01 => defmt::write!(f, "Chosen"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for ChannelEnable {
    #[inline(always)]
    fn from(val: u32) -> ChannelEnable {
        ChannelEnable::from_bits(val)
    }
}
impl From<ChannelEnable> for u32 {
    #[inline(always)]
    fn from(val: ChannelEnable) -> u32 {
        ChannelEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeNum {
    #[doc = "1."]
    Ssc1 = 0x0,
    #[doc = "2."]
    Ssc2 = 0x01,
    #[doc = "3."]
    Ssc3 = 0x02,
    #[doc = "4."]
    Ssc4 = 0x03,
    #[doc = "5."]
    Ssc5 = 0x04,
    #[doc = "6."]
    Ssc6 = 0x05,
    #[doc = "7."]
    Ssc7 = 0x06,
    #[doc = "8."]
    Ssc8 = 0x07,
    #[doc = "9."]
    Ssc9 = 0x08,
    #[doc = "10."]
    Ssc10 = 0x09,
    #[doc = "11."]
    Ssc11 = 0x0a,
    #[doc = "12."]
    Ssc12 = 0x0b,
    #[doc = "13."]
    Ssc13 = 0x0c,
    #[doc = "14."]
    Ssc14 = 0x0d,
    #[doc = "15."]
    Ssc15 = 0x0e,
    #[doc = "16."]
    Ssc16 = 0x0f,
}
impl ChargeNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChargeNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChargeNum {
    #[inline(always)]
    fn from(val: u8) -> ChargeNum {
        ChargeNum::from_bits(val)
    }
}
impl From<ChargeNum> for u8 {
    #[inline(always)]
    fn from(val: ChargeNum) -> u8 {
        ChargeNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigMode {
    #[doc = "Self capacitance."]
    SelfCp = 0x0,
    #[doc = "Mutual capacitance."]
    MtlCp = 0x01,
}
impl ConfigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigMode {
    #[inline(always)]
    fn from(val: u8) -> ConfigMode {
        ConfigMode::from_bits(val)
    }
}
impl From<ConfigMode> for u8 {
    #[inline(always)]
    fn from(val: ConfigMode) -> u8 {
        ConfigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigMutualMode {
    #[doc = "Self capacitance."]
    ConfigSfCp = 0x0,
    #[doc = "Mutual capacitance."]
    ConfigMtCp = 0x01,
}
impl ConfigMutualMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigMutualMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigMutualMode {
    #[inline(always)]
    fn from(val: u8) -> ConfigMutualMode {
        ConfigMutualMode::from_bits(val)
    }
}
impl From<ConfigMutualMode> for u8 {
    #[inline(always)]
    fn from(val: ConfigMutualMode) -> u8 {
        ConfigMutualMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrimFine {
    #[doc = "0.3125 pF."]
    Ctrim3125 = 0x0,
    #[doc = "0.625 pF."]
    Ctrim625 = 0x01,
    #[doc = "0.3125 * 3 pF."]
    Ctrim31253 = 0x02,
    #[doc = "0.3125 * 4 pF."]
    Ctrim31254 = 0x03,
    #[doc = "0.3125 * 5 pF."]
    Ctrim31255 = 0x04,
    #[doc = "0.3125 * 6 pF."]
    Ctrim31256 = 0x05,
    #[doc = "2.1875 pF."]
    Ctrim1875 = 0x06,
    #[doc = "2.5 pF."]
    Ctrim25 = 0x07,
}
impl CtrimFine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrimFine {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrimFine {
    #[inline(always)]
    fn from(val: u8) -> CtrimFine {
        CtrimFine::from_bits(val)
    }
}
impl From<CtrimFine> for u8 {
    #[inline(always)]
    fn from(val: CtrimFine) -> u8 {
        CtrimFine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cutoff {
    #[doc = "div = 1."]
    Div1 = 0x0,
    #[doc = "div = 2."]
    Div2 = 0x01,
    #[doc = "div = 4."]
    Div4 = 0x02,
    #[doc = "div = 8."]
    Div8 = 0x03,
    #[doc = "div = 16."]
    Div16 = 0x04,
    #[doc = "div = 32."]
    Div32 = 0x05,
    #[doc = "div = 64."]
    Div64 = 0x06,
    #[doc = "div = 128."]
    Div128 = 0x07,
    #[doc = "Do not use."]
    DivNc1 = 0x08,
    #[doc = "Do not use."]
    DivNc2 = 0x09,
    #[doc = "Do not use."]
    DivNc3 = 0x0a,
    #[doc = "Do not use."]
    DivNc4 = 0x0b,
    #[doc = "Do not use."]
    DivNc5 = 0x0c,
    #[doc = "Do not use."]
    DivNc6 = 0x0d,
    #[doc = "Do not use."]
    DivNc7 = 0x0e,
    #[doc = "Do not use."]
    DivNc8 = 0x0f,
}
impl Cutoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cutoff {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cutoff {
    #[inline(always)]
    fn from(val: u8) -> Cutoff {
        Cutoff::from_bits(val)
    }
}
impl From<Cutoff> for u8 {
    #[inline(always)]
    fn from(val: Cutoff) -> u8 {
        Cutoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debounce {
    #[doc = "1."]
    Int1 = 0x0,
    #[doc = "2."]
    Int2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "n."]
    IntN = 0x10,
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
}
impl Debounce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debounce {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debounce {
    #[inline(always)]
    fn from(val: u8) -> Debounce {
        Debounce::from_bits(val)
    }
}
impl From<Debounce> for u8 {
    #[inline(always)]
    fn from(val: Debounce) -> u8 {
        Debounce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decimation {
    #[doc = "1."]
    Dec1 = 0x0,
    #[doc = "2."]
    Dec2 = 0x01,
    #[doc = "3."]
    Dec3 = 0x02,
    #[doc = "4."]
    Dec4 = 0x03,
    #[doc = "5."]
    Dec5 = 0x04,
    #[doc = "6."]
    Dec6 = 0x05,
    #[doc = "7."]
    Dec7 = 0x06,
    #[doc = "8."]
    Dec8 = 0x07,
    #[doc = "9."]
    Dec9 = 0x08,
    #[doc = "10."]
    Dec10 = 0x09,
    #[doc = "11."]
    Dec11 = 0x0a,
    #[doc = "12."]
    Dec12 = 0x0b,
    #[doc = "13."]
    Dec13 = 0x0c,
    #[doc = "14."]
    Dec14 = 0x0d,
    #[doc = "15."]
    Dec15 = 0x0e,
    #[doc = "16."]
    Dec16 = 0x0f,
    #[doc = "17."]
    Dec17 = 0x10,
    #[doc = "18."]
    Dec18 = 0x11,
    #[doc = "19."]
    Dec19 = 0x12,
    #[doc = "20."]
    Dec20 = 0x13,
    #[doc = "21."]
    Dec21 = 0x14,
    #[doc = "22."]
    Dec22 = 0x15,
    #[doc = "23."]
    Dec23 = 0x16,
    #[doc = "24."]
    Dec24 = 0x17,
    #[doc = "25."]
    Dec25 = 0x18,
    #[doc = "26."]
    Dec26 = 0x19,
    #[doc = "27."]
    Dec27 = 0x1a,
    #[doc = "28."]
    Dec28 = 0x1b,
    #[doc = "29."]
    Dec29 = 0x1c,
    #[doc = "30."]
    Dec30 = 0x1d,
    #[doc = "31."]
    Dec31 = 0x1e,
    #[doc = "32."]
    Dec32 = 0x1f,
}
impl Decimation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Decimation {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Decimation {
    #[inline(always)]
    fn from(val: u8) -> Decimation {
        Decimation::from_bits(val)
    }
}
impl From<Decimation> for u8 {
    #[inline(always)]
    fn from(val: Decimation) -> u8 {
        Decimation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvolt {
    #[doc = "Vm = 0.6 V, Vp = 1.7 V."]
    Volt17 = 0x0,
    #[doc = "Vm = 0.6 V, Vp = 1.9 V."]
    Volt19 = 0x01,
    #[doc = "Vm = 0.6 V, Vp = 2.1 V."]
    Volt21 = 0x02,
    #[doc = "Vm = 0.6 V, Vp = 2.3 V."]
    Volt23 = 0x03,
    #[doc = "Vm = 0.6 V, Vp = 2.5 V."]
    Volt25 = 0x04,
    #[doc = "Vm = 0.6 V, Vp = 2.7 V."]
    Volt27 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Dvolt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvolt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvolt {
    #[inline(always)]
    fn from(val: u8) -> Dvolt {
        Dvolt::from_bits(val)
    }
}
impl From<Dvolt> for u8 {
    #[inline(always)]
    fn from(val: Dvolt) -> u8 {
        Dvolt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MMode {
    #[doc = "- 5 V ~ + 5 V."]
    Mode0 = 0x0,
    #[doc = "0 V ~ + 5 V."]
    Mode5 = 0x01,
}
impl MMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MMode {
    #[inline(always)]
    fn from(val: u8) -> MMode {
        MMode::from_bits(val)
    }
}
impl From<MMode> for u8 {
    #[inline(always)]
    fn from(val: MMode) -> u8 {
        MMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MNmirror {
    #[doc = "m = 1."]
    M1 = 0x0,
    #[doc = "m = 2."]
    M2 = 0x01,
    #[doc = "m = 3."]
    M3 = 0x02,
    #[doc = "m = 4."]
    M4 = 0x03,
}
impl MNmirror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MNmirror {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MNmirror {
    #[inline(always)]
    fn from(val: u8) -> MNmirror {
        MNmirror::from_bits(val)
    }
}
impl From<MNmirror> for u8 {
    #[inline(always)]
    fn from(val: MNmirror) -> u8 {
        MNmirror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPmirrorl {
    #[doc = "m = 4."]
    Mpl4 = 0x0,
    #[doc = "m = 8."]
    Mpl8 = 0x01,
    #[doc = "m = 12."]
    Mpl12 = 0x02,
    #[doc = "m = 16."]
    Mpl16 = 0x03,
    #[doc = "m = 20."]
    Mpl20 = 0x04,
    #[doc = "m = 24."]
    Mpl24 = 0x05,
    #[doc = "m = 28."]
    Mpl28 = 0x06,
    #[doc = "m = 32."]
    Mpl32 = 0x07,
}
impl MPmirrorl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPmirrorl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPmirrorl {
    #[inline(always)]
    fn from(val: u8) -> MPmirrorl {
        MPmirrorl::from_bits(val)
    }
}
impl From<MPmirrorl> for u8 {
    #[inline(always)]
    fn from(val: MPmirrorl) -> u8 {
        MPmirrorl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPmirrorr {
    #[doc = "m = 1."]
    Mp1 = 0x0,
    #[doc = "m = 2."]
    Mp2 = 0x01,
    #[doc = "m = 3."]
    Mp3 = 0x02,
    #[doc = "m = 4."]
    Mp4 = 0x03,
}
impl MPmirrorr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPmirrorr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPmirrorr {
    #[inline(always)]
    fn from(val: u8) -> MPmirrorr {
        MPmirrorr::from_bits(val)
    }
}
impl From<MPmirrorr> for u8 {
    #[inline(always)]
    fn from(val: MPmirrorr) -> u8 {
        MPmirrorr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPreCurrent {
    #[doc = "1 uA."]
    Cur1 = 0x0,
    #[doc = "2 uA."]
    Cur2 = 0x01,
    #[doc = "3 uA."]
    Cur3 = 0x02,
    #[doc = "4 uA."]
    Cur4 = 0x03,
    #[doc = "5 uA."]
    Cur5 = 0x04,
    #[doc = "6 uA."]
    Cur6 = 0x05,
    #[doc = "7 uA."]
    Cur7 = 0x06,
    #[doc = "8 uA."]
    Cur8 = 0x07,
}
impl MPreCurrent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPreCurrent {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPreCurrent {
    #[inline(always)]
    fn from(val: u8) -> MPreCurrent {
        MPreCurrent::from_bits(val)
    }
}
impl From<MPreCurrent> for u8 {
    #[inline(always)]
    fn from(val: MPreCurrent) -> u8 {
        MPreCurrent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPreRes {
    #[doc = "1 k."]
    Res1 = 0x0,
    #[doc = "2 k."]
    Res2 = 0x01,
    #[doc = "3 k."]
    Res3 = 0x02,
    #[doc = "4 k."]
    Res4 = 0x03,
    #[doc = "5 k."]
    Res5 = 0x04,
    #[doc = "6 k."]
    Res6 = 0x05,
    #[doc = "7 k."]
    Res7 = 0x06,
    #[doc = "8 k."]
    Res8 = 0x07,
}
impl MPreRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPreRes {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPreRes {
    #[inline(always)]
    fn from(val: u8) -> MPreRes {
        MPreRes::from_bits(val)
    }
}
impl From<MPreRes> for u8 {
    #[inline(always)]
    fn from(val: MPreRes) -> u8 {
        MPreRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSelRx {
    #[doc = "TSI\\[8\\]."]
    Tsi8 = 0x0,
    #[doc = "TSI\\[9\\]."]
    Tsi9 = 0x01,
    #[doc = "TSI\\[10\\]."]
    Tsi10 = 0x02,
    #[doc = "TSI\\[11\\]."]
    Tsi11 = 0x03,
    #[doc = "TSI\\[12\\]."]
    Tsi12 = 0x04,
    #[doc = "TSI\\[13\\]."]
    Tsi13 = 0x05,
    #[doc = "TSI\\[14\\]."]
    Tsi14 = 0x06,
    #[doc = "TSI\\[15\\]."]
    Tsi15 = 0x07,
    #[doc = "TSI\\[16\\]."]
    Tsi16 = 0x08,
    #[doc = "TSI\\[17\\]."]
    Tsi17 = 0x09,
    #[doc = "TSI\\[18\\]."]
    Tsi18 = 0x0a,
    #[doc = "TSI\\[19\\]."]
    Tsi19 = 0x0b,
    #[doc = "TSI\\[20\\]."]
    Tsi20 = 0x0c,
    #[doc = "TSI\\[21\\]."]
    Tsi21 = 0x0d,
    #[doc = "TSI\\[22\\]."]
    Tsi22 = 0x0e,
    #[doc = "TSI\\[23\\]."]
    Tsi23 = 0x0f,
    #[doc = "TSI\\[24\\]."]
    Tsi24 = 0x10,
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
}
impl MSelRx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSelRx {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSelRx {
    #[inline(always)]
    fn from(val: u8) -> MSelRx {
        MSelRx::from_bits(val)
    }
}
impl From<MSelRx> for u8 {
    #[inline(always)]
    fn from(val: MSelRx) -> u8 {
        MSelRx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSelTx {
    #[doc = "TSI\\[0\\]."]
    TsiTx0 = 0x0,
    #[doc = "TSI\\[1\\]."]
    TsiTx1 = 0x01,
    #[doc = "TSI\\[2\\]."]
    TsiTx2 = 0x02,
    #[doc = "TSI\\[3\\]."]
    TsiTx3 = 0x03,
    #[doc = "TSI\\[4\\]."]
    TsiTx4 = 0x04,
    #[doc = "TSI\\[5\\]."]
    TsiTx5 = 0x05,
    #[doc = "TSI\\[6\\]."]
    TsiTx6 = 0x06,
    #[doc = "TSI\\[7\\]."]
    TsiTx7 = 0x07,
}
impl MSelTx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSelTx {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSelTx {
    #[inline(always)]
    fn from(val: u8) -> MSelTx {
        MSelTx::from_bits(val)
    }
}
impl From<MSelTx> for u8 {
    #[inline(always)]
    fn from(val: MSelTx) -> u8 {
        MSelTx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSenBoost {
    #[doc = "0 uA."]
    Bst0 = 0x0,
    #[doc = "2 uA."]
    Bst2 = 0x01,
    #[doc = "4 uA."]
    Bst4 = 0x02,
    #[doc = "6 uA."]
    Bst6 = 0x03,
    #[doc = "8 uA."]
    Bst8 = 0x04,
    #[doc = "10 uA."]
    Bst10 = 0x05,
    #[doc = "12 uA."]
    Bst12 = 0x06,
    #[doc = "14 uA."]
    Bst14 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "2 * n uA."]
    Bst2n = 0x10,
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
}
impl MSenBoost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSenBoost {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSenBoost {
    #[inline(always)]
    fn from(val: u8) -> MSenBoost {
        MSenBoost::from_bits(val)
    }
}
impl From<MSenBoost> for u8 {
    #[inline(always)]
    fn from(val: MSenBoost) -> u8 {
        MSenBoost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSenRes {
    #[doc = "10 k."]
    Res10 = 0x0,
    #[doc = "10 k + (2.5 / 3) k (just for auto-calibration)."]
    Res253 = 0x01,
    #[doc = "12.5 k (default)."]
    Res125 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "25 k."]
    Res25 = 0x0e,
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
impl MSenRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSenRes {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSenRes {
    #[inline(always)]
    fn from(val: u8) -> MSenRes {
        MSenRes::from_bits(val)
    }
}
impl From<MSenRes> for u8 {
    #[inline(always)]
    fn from(val: MSenRes) -> u8 {
        MSenRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MTrimCap {
    #[doc = "0 pF."]
    Cp0 = 0x0,
    #[doc = "10 pF."]
    Cp10 = 0x01,
    #[doc = "10 pF."]
    Cp101 = 0x02,
    #[doc = "20 pF."]
    Cp20 = 0x03,
}
impl MTrimCap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MTrimCap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MTrimCap {
    #[inline(always)]
    fn from(val: u8) -> MTrimCap {
        MTrimCap::from_bits(val)
    }
}
impl From<MTrimCap> for u8 {
    #[inline(always)]
    fn from(val: MTrimCap) -> u8 {
        MTrimCap::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MTxUsed(u8);
impl MTxUsed {
    #[doc = "GPIO."]
    pub const Gpio: Self = Self(0x0);
    #[doc = "Mutual capacitance."]
    pub const Mtcp: Self = Self(0x01);
}
impl MTxUsed {
    pub const fn from_bits(val: u8) -> MTxUsed {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MTxUsed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Gpio"),
            0x01 => f.write_str("Mtcp"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MTxUsed {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Gpio"),
            0x01 => defmt::write!(f, "Mtcp"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MTxUsed {
    #[inline(always)]
    fn from(val: u8) -> MTxUsed {
        MTxUsed::from_bits(val)
    }
}
impl From<MTxUsed> for u8 {
    #[inline(always)]
    fn from(val: MTxUsed) -> u8 {
        MTxUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MVpreChoose {
    #[doc = "Internal 1.2 V."]
    Internal = 0x0,
    #[doc = "External 1.2 V from PMC."]
    External = 0x01,
}
impl MVpreChoose {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MVpreChoose {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MVpreChoose {
    #[inline(always)]
    fn from(val: u8) -> MVpreChoose {
        MVpreChoose::from_bits(val)
    }
}
impl From<MVpreChoose> for u8 {
    #[inline(always)]
    fn from(val: MVpreChoose) -> u8 {
        MVpreChoose::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveNochargeMin {
    #[doc = "(1 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv1 = 0x0,
    #[doc = "(2 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv2 = 0x01,
    #[doc = "(3 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv3 = 0x02,
    #[doc = "(4 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv4 = 0x03,
    #[doc = "(5 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv5 = 0x04,
    #[doc = "(6 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv6 = 0x05,
    #[doc = "(7 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv7 = 0x06,
    #[doc = "(8 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv8 = 0x07,
    #[doc = "(9 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv9 = 0x08,
    #[doc = "(10 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv10 = 0x09,
    #[doc = "(11 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv11 = 0x0a,
    #[doc = "(12 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv12 = 0x0b,
    #[doc = "(13 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv13 = 0x0c,
    #[doc = "(14 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv14 = 0x0d,
    #[doc = "(15 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv15 = 0x0e,
    #[doc = "(16 + SSC0\\[BASE_NOCHARGE_NUM\\])."]
    Mv16 = 0x0f,
}
impl MoveNochargeMin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveNochargeMin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveNochargeMin {
    #[inline(always)]
    fn from(val: u8) -> MoveNochargeMin {
        MoveNochargeMin::from_bits(val)
    }
}
impl From<MoveNochargeMin> for u8 {
    #[inline(always)]
    fn from(val: MoveNochargeMin) -> u8 {
        MoveNochargeMin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveRepeatNum {
    #[doc = "1."]
    Updn1 = 0x0,
    #[doc = "2."]
    Updn2 = 0x01,
    #[doc = "3."]
    Updn3 = 0x02,
    #[doc = "4."]
    Updn4 = 0x03,
    #[doc = "5."]
    Updn5 = 0x04,
    #[doc = "6."]
    Updn6 = 0x05,
    #[doc = "7."]
    Updn7 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
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
}
impl MoveRepeatNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveRepeatNum {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveRepeatNum {
    #[inline(always)]
    fn from(val: u8) -> MoveRepeatNum {
        MoveRepeatNum::from_bits(val)
    }
}
impl From<MoveRepeatNum> for u8 {
    #[inline(always)]
    fn from(val: MoveRepeatNum) -> u8 {
        MoveRepeatNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveStepsNum {
    #[doc = "0."]
    Updn0 = 0x0,
    #[doc = "1."]
    Updn1 = 0x01,
    #[doc = "2."]
    Updn2 = 0x02,
    #[doc = "3."]
    Updn3 = 0x03,
    #[doc = "4."]
    Updn4 = 0x04,
    #[doc = "5."]
    Updn5 = 0x05,
    #[doc = "6."]
    Updn6 = 0x06,
    #[doc = "7."]
    Updn7 = 0x07,
}
impl MoveStepsNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveStepsNum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveStepsNum {
    #[inline(always)]
    fn from(val: u8) -> MoveStepsNum {
        MoveStepsNum::from_bits(val)
    }
}
impl From<MoveStepsNum> for u8 {
    #[inline(always)]
    fn from(val: MoveStepsNum) -> u8 {
        MoveStepsNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Order {
    #[doc = "Order 1."]
    Ord1 = 0x0,
    #[doc = "Order 2."]
    Ord2 = 0x01,
}
impl Order {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Order {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Order {
    #[inline(always)]
    fn from(val: u8) -> Order {
        Order::from_bits(val)
    }
}
impl From<Order> for u8 {
    #[inline(always)]
    fn from(val: Order) -> u8 {
        Order::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscClkSel {
    #[doc = "Analog oscillator."]
    OscTsi = 0x0,
    #[doc = "Chip."]
    OscSoc = 0x01,
}
impl OscClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscClkSel {
    #[inline(always)]
    fn from(val: u8) -> OscClkSel {
        OscClkSel::from_bits(val)
    }
}
impl From<OscClkSel> for u8 {
    #[inline(always)]
    fn from(val: OscClkSel) -> u8 {
        OscClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrbsOutsel {
    #[doc = "Do not use."]
    Nc1 = 0x0,
    #[doc = "Do not use."]
    Nc2 = 0x01,
    #[doc = "2."]
    Prbs2 = 0x02,
    #[doc = "3."]
    Prbs3 = 0x03,
    #[doc = "4."]
    Prbs4 = 0x04,
    #[doc = "5."]
    Prbs5 = 0x05,
    #[doc = "6."]
    Prbs6 = 0x06,
    #[doc = "7."]
    Prbs7 = 0x07,
    #[doc = "8."]
    Prbs8 = 0x08,
    #[doc = "9."]
    Prbs9 = 0x09,
    #[doc = "10."]
    Prbs10 = 0x0a,
    #[doc = "11."]
    Prbs11 = 0x0b,
    #[doc = "12."]
    Prbs12 = 0x0c,
    #[doc = "13."]
    Prbs13 = 0x0d,
    #[doc = "14."]
    Prbs14 = 0x0e,
    #[doc = "15."]
    Prbs15 = 0x0f,
}
impl PrbsOutsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrbsOutsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrbsOutsel {
    #[inline(always)]
    fn from(val: u8) -> PrbsOutsel {
        PrbsOutsel::from_bits(val)
    }
}
impl From<PrbsOutsel> for u8 {
    #[inline(always)]
    fn from(val: PrbsOutsel) -> u8 {
        PrbsOutsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCtrim {
    #[doc = "2.5 pF."]
    Ctrim25 = 0x0,
    #[doc = "5.0 pF."]
    Ctrim5 = 0x01,
    #[doc = "7.5 pF."]
    Ctrim75 = 0x02,
    #[doc = "10 pF."]
    Ctrim10 = 0x03,
    #[doc = "12.5 pF."]
    Ctrim125 = 0x04,
    #[doc = "15.0 pF."]
    Ctrim15 = 0x05,
    #[doc = "17.5 pF."]
    Ctrim175 = 0x06,
    #[doc = "20 pF."]
    Ctrim20 = 0x07,
}
impl SCtrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCtrim {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCtrim {
    #[inline(always)]
    fn from(val: u8) -> SCtrim {
        SCtrim::from_bits(val)
    }
}
impl From<SCtrim> for u8 {
    #[inline(always)]
    fn from(val: SCtrim) -> u8 {
        SCtrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXch {
    #[doc = "1 / 16."]
    Def16 = 0x0,
    #[doc = "1 / 8."]
    Def8 = 0x01,
    #[doc = "1 / 4."]
    Def4 = 0x02,
    #[doc = "1 / 2."]
    Def2 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SXch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXch {
    #[inline(always)]
    fn from(val: u8) -> SXch {
        SXch::from_bits(val)
    }
}
impl From<SXch> for u8 {
    #[inline(always)]
    fn from(val: SXch) -> u8 {
        SXch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXdn {
    #[doc = "1 / 16."]
    XdnDef16 = 0x0,
    #[doc = "1 / 8."]
    XdnDef8 = 0x01,
    #[doc = "1 / 4."]
    XdnDef4 = 0x02,
    #[doc = "1 / 2."]
    XdnDef2 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SXdn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXdn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXdn {
    #[inline(always)]
    fn from(val: u8) -> SXdn {
        SXdn::from_bits(val)
    }
}
impl From<SXdn> for u8 {
    #[inline(always)]
    fn from(val: SXdn) -> u8 {
        SXdn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXin {
    #[doc = "1 / 8."]
    XinDef8 = 0x0,
    #[doc = "1 / 4."]
    XinDef4 = 0x01,
}
impl SXin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXin {
    #[inline(always)]
    fn from(val: u8) -> SXin {
        SXin::from_bits(val)
    }
}
impl From<SXin> for u8 {
    #[inline(always)]
    fn from(val: SXin) -> u8 {
        SXin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclk {
    #[doc = "27.37 MHz."]
    Setclk2737 = 0x0,
    #[doc = "22.23 MHz."]
    Setclk2223 = 0x01,
    #[doc = "18.73 MHz."]
    Setclk1873 = 0x02,
    #[doc = "16.65 MHz."]
    Setclk1616 = 0x03,
    #[doc = "14.27 MHz."]
    Setclk1427 = 0x04,
    #[doc = "12.73 MHz."]
    Setclk1273 = 0x05,
    #[doc = "11.49 MHz."]
    Setclk1149 = 0x06,
    #[doc = "10.46 MHz."]
    Setclk1046 = 0x07,
}
impl Setclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclk {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclk {
    #[inline(always)]
    fn from(val: u8) -> Setclk {
        Setclk::from_bits(val)
    }
}
impl From<Setclk> for u8 {
    #[inline(always)]
    fn from(val: Setclk) -> u8 {
        Setclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShieldEnable {
    #[doc = "Disables."]
    Disabled = 0x0,
    #[doc = "Enables."]
    Enabled = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
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
impl ShieldEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShieldEnable {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShieldEnable {
    #[inline(always)]
    fn from(val: u8) -> ShieldEnable {
        ShieldEnable::from_bits(val)
    }
}
impl From<ShieldEnable> for u8 {
    #[inline(always)]
    fn from(val: ShieldEnable) -> u8 {
        ShieldEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SscControlReverse {
    #[doc = "Polarity retained."]
    Enabled = 0x0,
    #[doc = "Polarity reversed."]
    Disabled = 0x01,
}
impl SscControlReverse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SscControlReverse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SscControlReverse {
    #[inline(always)]
    fn from(val: u8) -> SscControlReverse {
        SscControlReverse::from_bits(val)
    }
}
impl From<SscControlReverse> for u8 {
    #[inline(always)]
    fn from(val: SscControlReverse) -> u8 {
        SscControlReverse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SscMode {
    #[doc = "PRBS mode."]
    Prbs = 0x0,
    #[doc = "Up-Down Counter mode."]
    Updn = 0x01,
    #[doc = "Disables SSC function."]
    Disabled = 0x02,
    #[doc = "Do not use."]
    Nc = 0x03,
}
impl SscMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SscMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SscMode {
    #[inline(always)]
    fn from(val: u8) -> SscMode {
        SscMode::from_bits(val)
    }
}
impl From<SscMode> for u8 {
    #[inline(always)]
    fn from(val: SscMode) -> u8 {
        SscMode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SscPrescaleNum(u8);
impl SscPrescaleNum {
    #[doc = "div = 1."]
    pub const Div1: Self = Self(0x0);
    #[doc = "div = 2."]
    pub const Div2: Self = Self(0x01);
    #[doc = "div = 4."]
    pub const Div4: Self = Self(0x03);
    #[doc = "div = 8."]
    pub const Div8: Self = Self(0x07);
    #[doc = "div = 16."]
    pub const Div16: Self = Self(0x0f);
    #[doc = "div = 32."]
    pub const Div32: Self = Self(0x1f);
    #[doc = "div = 64."]
    pub const Div64: Self = Self(0x3f);
    #[doc = "div = 128."]
    pub const Div128: Self = Self(0x7f);
    #[doc = "div = 256."]
    pub const Div256: Self = Self(0xff);
}
impl SscPrescaleNum {
    pub const fn from_bits(val: u8) -> SscPrescaleNum {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for SscPrescaleNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Div1"),
            0x01 => f.write_str("Div2"),
            0x03 => f.write_str("Div4"),
            0x07 => f.write_str("Div8"),
            0x0f => f.write_str("Div16"),
            0x1f => f.write_str("Div32"),
            0x3f => f.write_str("Div64"),
            0x7f => f.write_str("Div128"),
            0xff => f.write_str("Div256"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SscPrescaleNum {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Div1"),
            0x01 => defmt::write!(f, "Div2"),
            0x03 => defmt::write!(f, "Div4"),
            0x07 => defmt::write!(f, "Div8"),
            0x0f => defmt::write!(f, "Div16"),
            0x1f => defmt::write!(f, "Div32"),
            0x3f => defmt::write!(f, "Div64"),
            0x7f => defmt::write!(f, "Div128"),
            0xff => defmt::write!(f, "Div256"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for SscPrescaleNum {
    #[inline(always)]
    fn from(val: u8) -> SscPrescaleNum {
        SscPrescaleNum::from_bits(val)
    }
}
impl From<SscPrescaleNum> for u8 {
    #[inline(always)]
    fn from(val: SscPrescaleNum) -> u8 {
        SscPrescaleNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stm {
    #[doc = "Software trigger scan."]
    SwtrigScn = 0x0,
    #[doc = "Hardware trigger scan."]
    HwtrigScn = 0x01,
}
impl Stm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stm {
    #[inline(always)]
    fn from(val: u8) -> Stm {
        Stm::from_bits(val)
    }
}
impl From<Stm> for u8 {
    #[inline(always)]
    fn from(val: Stm) -> u8 {
        Stm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TestFinger {
    #[doc = "Finger capacitor is 148 pF."]
    Fin148 = 0x0,
    #[doc = "Finger capacitor is 296 pF."]
    Fin296 = 0x01,
    #[doc = "Finger capacitor is 444 pF."]
    Fin444 = 0x02,
    #[doc = "Finger capacitor is 592 pF."]
    Fin592 = 0x03,
    #[doc = "Finger capacitor is 740 pF."]
    Fin740 = 0x04,
    #[doc = "Finger capacitor is 888 pF."]
    Fin888 = 0x05,
    #[doc = "Finger capacitor is 1036 pF."]
    Fin1036 = 0x06,
    #[doc = "Finger capacitor is 1184 pF."]
    Fin1184 = 0x07,
}
impl TestFinger {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TestFinger {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TestFinger {
    #[inline(always)]
    fn from(val: u8) -> TestFinger {
        TestFinger::from_bits(val)
    }
}
impl From<TestFinger> for u8 {
    #[inline(always)]
    fn from(val: TestFinger) -> u8 {
        TestFinger::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThesholdRatio {
    #[doc = "thresholdh = (baseline + counter) / 2 and thresholdl = (baseline - counter) / 2."]
    Tshd2 = 0x0,
    #[doc = "thresholdh = (baseline + counter) / 4 and thresholdl = (baseline - counter) / 4."]
    Tshd4 = 0x01,
    #[doc = "thresholdh = (baseline + counter) / 8 and thresholdl = (baseline - counter) / 8."]
    Tshd8 = 0x02,
    #[doc = "thresholdh = (baseline + counter) / 16 and thresholdl = (baseline - counter) / 16."]
    Tshd16 = 0x03,
    #[doc = "thresholdh = (baseline + counter) / 32 and thresholdl = (baseline - counter) / 32."]
    Tshd32 = 0x04,
    #[doc = "thresholdh = (baseline + counter) / 64 and thresholdl = (baseline - counter) / 64."]
    Tshd64 = 0x05,
    #[doc = "thresholdh = (baseline + counter) / 128 and thresholdl = (baseline - counter) / 128."]
    Tshd128 = 0x06,
    #[doc = "thresholdh = (baseline + counter) / 256 and thresholdl = (baseline - counter) / 256."]
    Tshd256 = 0x07,
}
impl ThesholdRatio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ThesholdRatio {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ThesholdRatio {
    #[inline(always)]
    fn from(val: u8) -> ThesholdRatio {
        ThesholdRatio::from_bits(val)
    }
}
impl From<ThesholdRatio> for u8 {
    #[inline(always)]
    fn from(val: ThesholdRatio) -> u8 {
        ThesholdRatio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigClkDivider {
    #[doc = "No divider."]
    DivNo = 0x0,
    #[doc = "Divided by 2."]
    Div2 = 0x01,
    #[doc = "Divided by 3."]
    Div3 = 0x02,
    #[doc = "Divided by 4."]
    Div4 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Divided by n."]
    DivN = 0x10,
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
}
impl TrigClkDivider {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigClkDivider {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigClkDivider {
    #[inline(always)]
    fn from(val: u8) -> TrigClkDivider {
        TrigClkDivider::from_bits(val)
    }
}
impl From<TrigClkDivider> for u8 {
    #[inline(always)]
    fn from(val: TrigClkDivider) -> u8 {
        TrigClkDivider::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigClkSel {
    #[doc = "32 k clock."]
    Clk32 = 0x0,
    #[doc = "clksoc."]
    ClkSoc = 0x01,
}
impl TrigClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigClkSel {
    #[inline(always)]
    fn from(val: u8) -> TrigClkSel {
        TrigClkSel::from_bits(val)
    }
}
impl From<TrigClkSel> for u8 {
    #[inline(always)]
    fn from(val: TrigClkSel) -> u8 {
        TrigClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsich {
    #[doc = "Channel 0."]
    SelfCp0 = 0x0,
    #[doc = "Channel 1."]
    SelfCp1 = 0x01,
    #[doc = "Channel 2."]
    SelfCp2 = 0x02,
    #[doc = "Channel 3."]
    SelfCp3 = 0x03,
    #[doc = "Channel 4."]
    SelfCp4 = 0x04,
    #[doc = "Channel 5."]
    SelfCp5 = 0x05,
    #[doc = "Channel 6."]
    SelfCp6 = 0x06,
    #[doc = "Channel 7."]
    SelfCp7 = 0x07,
    #[doc = "Channel 8."]
    SelfCp8 = 0x08,
    #[doc = "Channel 9."]
    SelfCp9 = 0x09,
    #[doc = "Channel 10."]
    SelfCp10 = 0x0a,
    #[doc = "Channel 11."]
    SelfCp11 = 0x0b,
    #[doc = "Channel 12."]
    SelfCp12 = 0x0c,
    #[doc = "Channel 13."]
    SelfCp13 = 0x0d,
    #[doc = "Channel 14."]
    SelfCp14 = 0x0e,
    #[doc = "Channel 15."]
    SelfCp15 = 0x0f,
    #[doc = "Channel 16."]
    SelfCp16 = 0x10,
    #[doc = "Channel 17."]
    SelfCp17 = 0x11,
    #[doc = "Channel 18."]
    SelfCp18 = 0x12,
    #[doc = "Channel 19."]
    SelfCp19 = 0x13,
    #[doc = "Channel 20."]
    SelfCp20 = 0x14,
    #[doc = "Channel 21."]
    SelfCp21 = 0x15,
    #[doc = "Channel 22."]
    SelfCp22 = 0x16,
    #[doc = "Channel 23."]
    SelfCp23 = 0x17,
    #[doc = "Channel 24."]
    SelfCp24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Tsich {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsich {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsich {
    #[inline(always)]
    fn from(val: u8) -> Tsich {
        Tsich::from_bits(val)
    }
}
impl From<Tsich> for u8 {
    #[inline(always)]
    fn from(val: Tsich) -> u8 {
        Tsich::to_bits(val)
    }
}
