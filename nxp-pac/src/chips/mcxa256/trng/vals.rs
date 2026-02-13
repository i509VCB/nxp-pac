#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigOpt(u8);
impl ConfigOpt {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    pub const CONFIG_OPT_VAL: Self = Self(0x0);
}
impl ConfigOpt {
    pub const fn from_bits(val: u8) -> ConfigOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ConfigOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CONFIG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CONFIG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ConfigOpt {
    #[inline(always)]
    fn from(val: u8) -> ConfigOpt {
        ConfigOpt::from_bits(val)
    }
}
impl From<ConfigOpt> for u8 {
    #[inline(always)]
    fn from(val: ConfigOpt) -> u8 {
        ConfigOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcoRev(u8);
impl EcoRev {
    #[doc = "TRNG_ECO_REV for TRNG."]
    pub const ECO_REV_VAL: Self = Self(0x0);
}
impl EcoRev {
    pub const fn from_bits(val: u8) -> EcoRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EcoRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECO_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcoRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECO_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EcoRev {
    #[inline(always)]
    fn from(val: u8) -> EcoRev {
        EcoRev::from_bits(val)
    }
}
impl From<EcoRev> for u8 {
    #[inline(always)]
    fn from(val: EcoRev) -> u8 {
        EcoRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Era(u8);
impl Era {
    #[doc = "ERA of the TRNG."]
    pub const ERA_VAL: Self = Self(0x0c);
}
impl Era {
    pub const fn from_bits(val: u8) -> Era {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Era {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("ERA_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Era {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "ERA_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Era {
    #[inline(always)]
    fn from(val: u8) -> Era {
        Era::from_bits(val)
    }
}
impl From<Era> for u8 {
    #[inline(always)]
    fn from(val: Era) -> u8 {
        Era::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusEntVal {
    #[doc = "Busy generating entropy. Any value read from the Entropy registers is invalid."]
    ENT_VAL_INVALID = 0x0,
    #[doc = "Values read from the Entropy registers are valid."]
    ENT_VAL_VALID = 0x01,
}
impl IntStatusEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntStatusEntVal {
        IntStatusEntVal::from_bits(val)
    }
}
impl From<IntStatusEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntStatusEntVal) -> u8 {
        IntStatusEntVal::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntgOpt(u8);
impl IntgOpt {
    #[doc = "INTG_OPT for TRNG."]
    pub const INTG_OPT_VAL: Self = Self(0x0a);
}
impl IntgOpt {
    pub const fn from_bits(val: u8) -> IntgOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for IntgOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0a => f.write_str("INTG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntgOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0a => defmt::write!(f, "INTG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for IntgOpt {
    #[inline(always)]
    fn from(val: u8) -> IntgOpt {
        IntgOpt::from_bits(val)
    }
}
impl From<IntgOpt> for u8 {
    #[inline(always)]
    fn from(val: IntgOpt) -> u8 {
        IntgOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IpId(u16);
impl IpId {
    #[doc = "ID for TRNG."]
    pub const IP_ID_VAL: Self = Self(0x30);
}
impl IpId {
    pub const fn from_bits(val: u16) -> IpId {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for IpId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x30 => f.write_str("IP_ID_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpId {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x30 => defmt::write!(f, "IP_ID_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for IpId {
    #[inline(always)]
    fn from(val: u16) -> IpId {
        IpId::from_bits(val)
    }
}
impl From<IpId> for u16 {
    #[inline(always)]
    fn from(val: IpId) -> u16 {
        IpId::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MajRev(u8);
impl MajRev {
    #[doc = "Major revision number for TRNG."]
    pub const MAJ_REV_VAL: Self = Self(0x14);
}
impl MajRev {
    pub const fn from_bits(val: u8) -> MajRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MajRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x14 => f.write_str("MAJ_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MajRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x14 => defmt::write!(f, "MAJ_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MajRev {
    #[inline(always)]
    fn from(val: u8) -> MajRev {
        MajRev::from_bits(val)
    }
}
impl From<MajRev> for u8 {
    #[inline(always)]
    fn from(val: MajRev) -> u8 {
        MajRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinRev(u8);
impl MinRev {
    #[doc = "Minor revision number for TRNG."]
    pub const MIN_REV_VAL: Self = Self(0x0c);
}
impl MinRev {
    pub const fn from_bits(val: u8) -> MinRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MinRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("MIN_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MinRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "MIN_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MinRev {
    #[inline(always)]
    fn from(val: u8) -> MinRev {
        MinRev::from_bits(val)
    }
}
impl From<MinRev> for u8 {
    #[inline(always)]
    fn from(val: MinRev) -> u8 {
        MinRev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2Div {
    #[doc = "Use ring oscillator 2 with no divide"]
    OSC2_DIV_DIV1 = 0x0,
    #[doc = "Use ring oscillator 2 divided-by-2"]
    OSC2_DIV_DIV2 = 0x01,
    #[doc = "Use ring oscillator 2 divided-by-4"]
    OSC2_DIV_DIV4 = 0x02,
    #[doc = "Use ring oscillator 2 divided-by-8"]
    OSC2_DIV_DIV8 = 0x03,
}
impl Osc2Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2Div {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2Div {
    #[inline(always)]
    fn from(val: u8) -> Osc2Div {
        Osc2Div::from_bits(val)
    }
}
impl From<Osc2Div> for u8 {
    #[inline(always)]
    fn from(val: Osc2Div) -> u8 {
        Osc2Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2OutEn {
    #[doc = "Ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_0 = 0x0,
    #[doc = "Allows external viewing of divided-by-2 ring oscillator 2 if MCTL\\[PRGM\\] = 1 mode is also selected, else ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_1 = 0x01,
}
impl Osc2OutEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2OutEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2OutEn {
    #[inline(always)]
    fn from(val: u8) -> Osc2OutEn {
        Osc2OutEn::from_bits(val)
    }
}
impl From<Osc2OutEn> for u8 {
    #[inline(always)]
    fn from(val: Osc2OutEn) -> u8 {
        Osc2OutEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscDiv {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_DIV1 = 0x0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_DIV2 = 0x01,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_DIV4 = 0x02,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_DIV8 = 0x03,
}
impl OscDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscDiv {
    #[inline(always)]
    fn from(val: u8) -> OscDiv {
        OscDiv::from_bits(val)
    }
}
impl From<OscDiv> for u8 {
    #[inline(always)]
    fn from(val: OscDiv) -> u8 {
        OscDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscFailsafeLmt {
    #[doc = "The limit N is 4096 (2^12) system clocks."]
    OSC_FAILSAFE_LMT_4K = 0x0,
    #[doc = "The limit N is 65536 (2^16) system clocks. (default)"]
    OSC_FAILSAFE_LMT_64K = 0x01,
    #[doc = "N is 2^20 system clocks."]
    OSC_FAILSAFE_LMT_1M = 0x02,
    #[doc = "N is 2^22 system clocks (full range of the counter being used)."]
    OSC_FAILSAFE_LMT_4M = 0x03,
}
impl OscFailsafeLmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscFailsafeLmt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscFailsafeLmt {
    #[inline(always)]
    fn from(val: u8) -> OscFailsafeLmt {
        OscFailsafeLmt::from_bits(val)
    }
}
impl From<OscFailsafeLmt> for u8 {
    #[inline(always)]
    fn from(val: OscFailsafeLmt) -> u8 {
        OscFailsafeLmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngEntCtl {
    #[doc = "Single oscillator mode, using OSC1 (default)"]
    TRNG_ENT_CTL_SINGLE_OSC1 = 0x0,
    #[doc = "Dual oscillator mode"]
    TRNG_ENT_CTL_DUAL_OSCS = 0x01,
    #[doc = "Single oscillator mode, using OSC2"]
    TRNG_ENT_CTL_SINGLE_OSC2 = 0x02,
    #[doc = "Unused, (bit field cannot be written to this value)"]
    OSC2_DIV_DIV8 = 0x03,
}
impl TrngEntCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngEntCtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngEntCtl {
    #[inline(always)]
    fn from(val: u8) -> TrngEntCtl {
        TrngEntCtl::from_bits(val)
    }
}
impl From<TrngEntCtl> for u8 {
    #[inline(always)]
    fn from(val: TrngEntCtl) -> u8 {
        TrngEntCtl::to_bits(val)
    }
}
