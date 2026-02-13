#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigOpt(u8);
impl ConfigOpt {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    pub const CONFIG_OPT_0: Self = Self(0x0);
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
            0x0 => f.write_str("CONFIG_OPT_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CONFIG_OPT_0"),
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
    pub const ECO_REV_0: Self = Self(0x0);
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
            0x0 => f.write_str("ECO_REV_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcoRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECO_REV_0"),
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
    #[doc = "COMPILE_OPT for TRNG."]
    pub const ERA_0: Self = Self(0x0);
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
            0x0 => f.write_str("ERA_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Era {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ERA_0"),
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
pub enum IntCtrlEntVal {
    #[doc = "Same behavior as bit 0 above."]
    ENT_VAL_0 = 0x0,
    #[doc = "Same behavior as bit 0 above."]
    ENT_VAL_1 = 0x01,
}
impl IntCtrlEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlEntVal {
        IntCtrlEntVal::from_bits(val)
    }
}
impl From<IntCtrlEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlEntVal) -> u8 {
        IntCtrlEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlFrqCtFail {
    #[doc = "Same behavior as bit 0 above."]
    FRQ_CT_FAIL_0 = 0x0,
    #[doc = "Same behavior as bit 0 above."]
    FRQ_CT_FAIL_1 = 0x01,
}
impl IntCtrlFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlFrqCtFail {
        IntCtrlFrqCtFail::from_bits(val)
    }
}
impl From<IntCtrlFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlFrqCtFail) -> u8 {
        IntCtrlFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlHwErr {
    #[doc = "Corresponding bit of INT_STATUS cleared."]
    HW_ERR_0 = 0x0,
    #[doc = "Corresponding bit of INT_STATUS active."]
    HW_ERR_1 = 0x01,
}
impl IntCtrlHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlHwErr {
        IntCtrlHwErr::from_bits(val)
    }
}
impl From<IntCtrlHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlHwErr) -> u8 {
        IntCtrlHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskEntVal {
    #[doc = "Same behavior as bit 0 above."]
    ENT_VAL_0 = 0x0,
    #[doc = "Same behavior as bit 0 above."]
    ENT_VAL_1 = 0x01,
}
impl IntMaskEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntMaskEntVal {
        IntMaskEntVal::from_bits(val)
    }
}
impl From<IntMaskEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntMaskEntVal) -> u8 {
        IntMaskEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskFrqCtFail {
    #[doc = "Same behavior as bit 0 above."]
    FRQ_CT_FAIL_0 = 0x0,
    #[doc = "Same behavior as bit 0 above."]
    FRQ_CT_FAIL_1 = 0x01,
}
impl IntMaskFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntMaskFrqCtFail {
        IntMaskFrqCtFail::from_bits(val)
    }
}
impl From<IntMaskFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntMaskFrqCtFail) -> u8 {
        IntMaskFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskHwErr {
    #[doc = "Corresponding interrupt of INT_STATUS is masked."]
    HW_ERR_0 = 0x0,
    #[doc = "Corresponding bit of INT_STATUS is active."]
    HW_ERR_1 = 0x01,
}
impl IntMaskHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntMaskHwErr {
        IntMaskHwErr::from_bits(val)
    }
}
impl From<IntMaskHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntMaskHwErr) -> u8 {
        IntMaskHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusEntVal {
    #[doc = "Busy generation entropy. Any value read is invalid."]
    ENT_VAL_0 = 0x0,
    #[doc = "TRNG can be stopped and entropy is valid if read."]
    ENT_VAL_1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusFrqCtFail {
    #[doc = "No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_0 = 0x0,
    #[doc = "The frequency counter has detected a failure."]
    FRQ_CT_FAIL_1 = 0x01,
}
impl IntStatusFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntStatusFrqCtFail {
        IntStatusFrqCtFail::from_bits(val)
    }
}
impl From<IntStatusFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntStatusFrqCtFail) -> u8 {
        IntStatusFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusHwErr {
    #[doc = "no error"]
    HW_ERR_0 = 0x0,
    #[doc = "error detected."]
    HW_ERR_1 = 0x01,
}
impl IntStatusHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntStatusHwErr {
        IntStatusHwErr::from_bits(val)
    }
}
impl From<IntStatusHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntStatusHwErr) -> u8 {
        IntStatusHwErr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntgOpt(u8);
impl IntgOpt {
    #[doc = "INTG_OPT for TRNG."]
    pub const INTG_OPT_0: Self = Self(0x0);
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
            0x0 => f.write_str("INTG_OPT_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntgOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INTG_OPT_0"),
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
    pub const IP_ID_48: Self = Self(0x30);
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
            0x30 => f.write_str("IP_ID_48"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpId {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x30 => defmt::write!(f, "IP_ID_48"),
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
    pub const MAJ_REV_1: Self = Self(0x01);
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
            0x01 => f.write_str("MAJ_REV_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MajRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MAJ_REV_1"),
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
    pub const MIN_REV_0: Self = Self(0x0);
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
            0x0 => f.write_str("MIN_REV_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MinRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MIN_REV_0"),
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
pub enum NoPrgm {
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NO_PRGM_0 = 0x0,
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NO_PRGM_1 = 0x01,
}
impl NoPrgm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoPrgm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoPrgm {
    #[inline(always)]
    fn from(val: u8) -> NoPrgm {
        NoPrgm::from_bits(val)
    }
}
impl From<NoPrgm> for u8 {
    #[inline(always)]
    fn from(val: NoPrgm) -> u8 {
        NoPrgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscDiv {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_0 = 0x0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_1 = 0x01,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_2 = 0x02,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_3 = 0x03,
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
pub enum SampMode {
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_0 = 0x0,
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_1 = 0x01,
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_2 = 0x02,
    #[doc = "undefined/reserved."]
    SAMP_MODE_3 = 0x03,
}
impl SampMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SampMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SampMode {
    #[inline(always)]
    fn from(val: u8) -> SampMode {
        SampMode::from_bits(val)
    }
}
impl From<SampMode> for u8 {
    #[inline(always)]
    fn from(val: SampMode) -> u8 {
        SampMode::to_bits(val)
    }
}
