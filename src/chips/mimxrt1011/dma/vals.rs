#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "eDMA is idle"]
    IDLE = 0x0,
    #[doc = "eDMA is executing a channel"]
    ACTIVE = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadn {
    #[doc = "Writes 0 to only the TCDn_CSR\\[DONE\\] field specified in the CDNE field"]
    CLEAR_DONE = 0x0,
    #[doc = "Writes 0 to all bits in TCDn_CSR\\[DONE\\]"]
    CLEAR_ALL = 0x01,
}
impl Cadn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadn {
    #[inline(always)]
    fn from(val: u8) -> Cadn {
        Cadn::from_bits(val)
    }
}
impl From<Cadn> for u8 {
    #[inline(always)]
    fn from(val: Cadn) -> u8 {
        Cadn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caee {
    #[doc = "Write 0 only to the EEI field specified in the CEEI field"]
    CLEAR_EEI = 0x0,
    #[doc = "Write 0 to all fields in EEI"]
    CLEAR_ALL = 0x01,
}
impl Caee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caee {
    #[inline(always)]
    fn from(val: u8) -> Caee {
        Caee::from_bits(val)
    }
}
impl From<Caee> for u8 {
    #[inline(always)]
    fn from(val: Caee) -> u8 {
        Caee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caei {
    #[doc = "Write 0 to only the ERR field specified in the CERR field"]
    CLEAR_ERR = 0x0,
    #[doc = "Write 0 to all fields in ERR"]
    CLEAR_ALL = 0x01,
}
impl Caei {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caei {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caei {
    #[inline(always)]
    fn from(val: u8) -> Caei {
        Caei::from_bits(val)
    }
}
impl From<Caei> for u8 {
    #[inline(always)]
    fn from(val: Caei) -> u8 {
        Caei::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caer {
    #[doc = "Write 0 to only the ERQ field specified in the CERQ field"]
    CLEAR_ERQ = 0x0,
    #[doc = "Write 0 to all fields in ERQ"]
    CLEAR_ALL = 0x01,
}
impl Caer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caer {
    #[inline(always)]
    fn from(val: u8) -> Caer {
        Caer::from_bits(val)
    }
}
impl From<Caer> for u8 {
    #[inline(always)]
    fn from(val: Caer) -> u8 {
        Caer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cair {
    #[doc = "Clear only the INT field specified in the CINT field"]
    CLEAR_INT = 0x0,
    #[doc = "Clear all bits in INT"]
    CLEAR_ALL = 0x01,
}
impl Cair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cair {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cair {
    #[inline(always)]
    fn from(val: u8) -> Cair {
        Cair::from_bits(val)
    }
}
impl From<Cair> for u8 {
    #[inline(always)]
    fn from(val: Cair) -> u8 {
        Cair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CdneNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NO_OPS = 0x01,
}
impl CdneNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CdneNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CdneNop {
    #[inline(always)]
    fn from(val: u8) -> CdneNop {
        CdneNop::from_bits(val)
    }
}
impl From<CdneNop> for u8 {
    #[inline(always)]
    fn from(val: CdneNop) -> u8 {
        CdneNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CeeiNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation, ignore the other fields in this register"]
    NO_OPS = 0x01,
}
impl CeeiNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CeeiNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CeeiNop {
    #[inline(always)]
    fn from(val: u8) -> CeeiNop {
        CeeiNop::from_bits(val)
    }
}
impl From<CeeiNop> for u8 {
    #[inline(always)]
    fn from(val: CeeiNop) -> u8 {
        CeeiNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CerqNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation, ignore the other fields in this register"]
    NO_OPS = 0x01,
}
impl CerqNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CerqNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CerqNop {
    #[inline(always)]
    fn from(val: u8) -> CerqNop {
        CerqNop::from_bits(val)
    }
}
impl From<CerqNop> for u8 {
    #[inline(always)]
    fn from(val: CerqNop) -> u8 {
        CerqNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CerrNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NO_OPS = 0x01,
}
impl CerrNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CerrNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CerrNop {
    #[inline(always)]
    fn from(val: u8) -> CerrNop {
        CerrNop::from_bits(val)
    }
}
impl From<CerrNop> for u8 {
    #[inline(always)]
    fn from(val: CerrNop) -> u8 {
        CerrNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CintNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NO_OPS = 0x01,
}
impl CintNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CintNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CintNop {
    #[inline(always)]
    fn from(val: u8) -> CintNop {
        CintNop::from_bits(val)
    }
}
impl From<CintNop> for u8 {
    #[inline(always)]
    fn from(val: CintNop) -> u8 {
        CintNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clm {
    #[doc = "Continuous link mode is off"]
    CLM_OFF = 0x0,
    #[doc = "Continuous link mode is on"]
    CLM_ON = 0x01,
}
impl Clm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clm {
    #[inline(always)]
    fn from(val: u8) -> Clm {
        Clm::from_bits(val)
    }
}
impl From<Clm> for u8 {
    #[inline(always)]
    fn from(val: Clm) -> u8 {
        Clm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrEcx {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "Cancel the remaining data transfer"]
    CANCEL = 0x01,
}
impl CrEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrEcx {
    #[inline(always)]
    fn from(val: u8) -> CrEcx {
        CrEcx::from_bits(val)
    }
}
impl From<CrEcx> for u8 {
    #[inline(always)]
    fn from(val: CrEcx) -> u8 {
        CrEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cx {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "Cancel the remaining data transfer"]
    CANCEL = 0x01,
}
impl Cx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cx {
    #[inline(always)]
    fn from(val: u8) -> Cx {
        Cx::from_bits(val)
    }
}
impl From<Cx> for u8 {
    #[inline(always)]
    fn from(val: Cx) -> u8 {
        Cx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri0Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri0Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri0Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri0Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri0Dpa {
        Dchpri0Dpa::from_bits(val)
    }
}
impl From<Dchpri0Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri0Dpa) -> u8 {
        Dchpri0Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri10Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri10Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri10Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri10Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri10Dpa {
        Dchpri10Dpa::from_bits(val)
    }
}
impl From<Dchpri10Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri10Dpa) -> u8 {
        Dchpri10Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri11Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri11Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri11Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri11Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri11Dpa {
        Dchpri11Dpa::from_bits(val)
    }
}
impl From<Dchpri11Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri11Dpa) -> u8 {
        Dchpri11Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri12Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri12Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri12Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri12Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri12Dpa {
        Dchpri12Dpa::from_bits(val)
    }
}
impl From<Dchpri12Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri12Dpa) -> u8 {
        Dchpri12Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri13Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri13Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri13Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri13Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri13Dpa {
        Dchpri13Dpa::from_bits(val)
    }
}
impl From<Dchpri13Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri13Dpa) -> u8 {
        Dchpri13Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri14Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri14Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri14Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri14Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri14Dpa {
        Dchpri14Dpa::from_bits(val)
    }
}
impl From<Dchpri14Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri14Dpa) -> u8 {
        Dchpri14Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri15Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri15Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri15Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri15Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri15Dpa {
        Dchpri15Dpa::from_bits(val)
    }
}
impl From<Dchpri15Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri15Dpa) -> u8 {
        Dchpri15Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri1Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri1Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri1Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri1Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri1Dpa {
        Dchpri1Dpa::from_bits(val)
    }
}
impl From<Dchpri1Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri1Dpa) -> u8 {
        Dchpri1Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri2Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri2Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri2Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri2Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri2Dpa {
        Dchpri2Dpa::from_bits(val)
    }
}
impl From<Dchpri2Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri2Dpa) -> u8 {
        Dchpri2Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri3Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri3Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri3Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri3Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri3Dpa {
        Dchpri3Dpa::from_bits(val)
    }
}
impl From<Dchpri3Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri3Dpa) -> u8 {
        Dchpri3Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri4Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri4Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri4Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri4Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri4Dpa {
        Dchpri4Dpa::from_bits(val)
    }
}
impl From<Dchpri4Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri4Dpa) -> u8 {
        Dchpri4Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri5Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri5Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri5Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri5Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri5Dpa {
        Dchpri5Dpa::from_bits(val)
    }
}
impl From<Dchpri5Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri5Dpa) -> u8 {
        Dchpri5Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri6Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri6Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri6Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri6Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri6Dpa {
        Dchpri6Dpa::from_bits(val)
    }
}
impl From<Dchpri6Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri6Dpa) -> u8 {
        Dchpri6Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri7Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri7Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri7Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri7Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri7Dpa {
        Dchpri7Dpa::from_bits(val)
    }
}
impl From<Dchpri7Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri7Dpa) -> u8 {
        Dchpri7Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri8Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri8Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri8Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri8Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri8Dpa {
        Dchpri8Dpa::from_bits(val)
    }
}
impl From<Dchpri8Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri8Dpa) -> u8 {
        Dchpri8Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri9Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri9Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri9Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri9Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri9Dpa {
        Dchpri9Dpa::from_bits(val)
    }
}
impl From<Dchpri9Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri9Dpa) -> u8 {
        Dchpri9Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EsEcx {
    #[doc = "No canceled transfers"]
    NO_CANCELS = 0x0,
    #[doc = "The most-recently recorded entry was a canceled transfer initiated by the error cancel transfer field"]
    CANCELED = 0x01,
}
impl EsEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EsEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EsEcx {
    #[inline(always)]
    fn from(val: u8) -> EsEcx {
        EsEcx::from_bits(val)
    }
}
impl From<EsEcx> for u8 {
    #[inline(always)]
    fn from(val: EsEcx) -> u8 {
        EsEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "eDMA operations halted"]
    HALT_DMA = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hoe {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "Error causes HALT field to be automatically set to 1"]
    HALT_ON_ERROR = 0x01,
}
impl Hoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hoe {
    #[inline(always)]
    fn from(val: u8) -> Hoe {
        Hoe::from_bits(val)
    }
}
impl From<Hoe> for u8 {
    #[inline(always)]
    fn from(val: Hoe) -> u8 {
        Hoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int6 {
    #[doc = "The interrupt request for channel 6 is cleared"]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt request for channel 6 is active"]
    CTIVE = 0x01,
}
impl Int6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int6 {
    #[inline(always)]
    fn from(val: u8) -> Int6 {
        Int6::from_bits(val)
    }
}
impl From<Int6> for u8 {
    #[inline(always)]
    fn from(val: Int6) -> u8 {
        Int6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saee {
    #[doc = "Write 1 only to the EEI field specified in the SEEI field"]
    SET_EEI = 0x0,
    #[doc = "Writes 1 to all fields in EEI"]
    SET_ALL = 0x01,
}
impl Saee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saee {
    #[inline(always)]
    fn from(val: u8) -> Saee {
        Saee::from_bits(val)
    }
}
impl From<Saee> for u8 {
    #[inline(always)]
    fn from(val: Saee) -> u8 {
        Saee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saer {
    #[doc = "Write 1 to only the ERQ field specified in the SERQ field"]
    SET_ERQ = 0x0,
    #[doc = "Write 1 to all fields in ERQ"]
    SET_ALL = 0x01,
}
impl Saer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saer {
    #[inline(always)]
    fn from(val: u8) -> Saer {
        Saer::from_bits(val)
    }
}
impl From<Saer> for u8 {
    #[inline(always)]
    fn from(val: Saer) -> u8 {
        Saer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sast {
    #[doc = "Write 1 to only the TCDn_CSR\\[START\\] field specified in the SSRT field"]
    SET_START = 0x0,
    #[doc = "Write 1 to all bits in TCDn_CSR\\[START\\]"]
    SET_ALL = 0x01,
}
impl Sast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sast {
    #[inline(always)]
    fn from(val: u8) -> Sast {
        Sast::from_bits(val)
    }
}
impl From<Sast> for u8 {
    #[inline(always)]
    fn from(val: Sast) -> u8 {
        Sast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeeiNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation, ignore the other fields in this register"]
    NO_OPS = 0x01,
}
impl SeeiNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeeiNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeeiNop {
    #[inline(always)]
    fn from(val: u8) -> SeeiNop {
        SeeiNop::from_bits(val)
    }
}
impl From<SeeiNop> for u8 {
    #[inline(always)]
    fn from(val: SeeiNop) -> u8 {
        SeeiNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SerqNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation, ignore the other fields in this register"]
    NO_OPS = 0x01,
}
impl SerqNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerqNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerqNop {
    #[inline(always)]
    fn from(val: u8) -> SerqNop {
        SerqNop::from_bits(val)
    }
}
impl From<SerqNop> for u8 {
    #[inline(always)]
    fn from(val: SerqNop) -> u8 {
        SerqNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrtNop {
    #[doc = "Normal operation"]
    NORMAL_OPS = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NO_OPS = 0x01,
}
impl SsrtNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrtNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrtNop {
    #[inline(always)]
    fn from(val: u8) -> SsrtNop {
        SsrtNop::from_bits(val)
    }
}
impl From<SsrtNop> for u8 {
    #[inline(always)]
    fn from(val: SsrtNop) -> u8 {
        SsrtNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd0AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd0AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd0AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd0AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd0AttrSsize {
        Tcd0AttrSsize::from_bits(val)
    }
}
impl From<Tcd0AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd0AttrSsize) -> u8 {
        Tcd0AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd0CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd0CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd0CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd0CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd0CsrBwc {
        Tcd0CsrBwc::from_bits(val)
    }
}
impl From<Tcd0CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd0CsrBwc) -> u8 {
        Tcd0CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd0CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd0CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd0CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd0CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd0CsrEsg {
        Tcd0CsrEsg::from_bits(val)
    }
}
impl From<Tcd0CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd0CsrEsg) -> u8 {
        Tcd0CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd10AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd10AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd10AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd10AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd10AttrSsize {
        Tcd10AttrSsize::from_bits(val)
    }
}
impl From<Tcd10AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd10AttrSsize) -> u8 {
        Tcd10AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd10CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd10CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd10CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd10CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd10CsrBwc {
        Tcd10CsrBwc::from_bits(val)
    }
}
impl From<Tcd10CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd10CsrBwc) -> u8 {
        Tcd10CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd10CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd10CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd10CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd10CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd10CsrEsg {
        Tcd10CsrEsg::from_bits(val)
    }
}
impl From<Tcd10CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd10CsrEsg) -> u8 {
        Tcd10CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd11AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd11AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd11AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd11AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd11AttrSsize {
        Tcd11AttrSsize::from_bits(val)
    }
}
impl From<Tcd11AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd11AttrSsize) -> u8 {
        Tcd11AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd11CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd11CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd11CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd11CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd11CsrBwc {
        Tcd11CsrBwc::from_bits(val)
    }
}
impl From<Tcd11CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd11CsrBwc) -> u8 {
        Tcd11CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd11CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd11CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd11CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd11CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd11CsrEsg {
        Tcd11CsrEsg::from_bits(val)
    }
}
impl From<Tcd11CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd11CsrEsg) -> u8 {
        Tcd11CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd12AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd12AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd12AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd12AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd12AttrSsize {
        Tcd12AttrSsize::from_bits(val)
    }
}
impl From<Tcd12AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd12AttrSsize) -> u8 {
        Tcd12AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd12CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd12CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd12CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd12CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd12CsrBwc {
        Tcd12CsrBwc::from_bits(val)
    }
}
impl From<Tcd12CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd12CsrBwc) -> u8 {
        Tcd12CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd12CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd12CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd12CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd12CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd12CsrEsg {
        Tcd12CsrEsg::from_bits(val)
    }
}
impl From<Tcd12CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd12CsrEsg) -> u8 {
        Tcd12CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd13AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd13AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd13AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd13AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd13AttrSsize {
        Tcd13AttrSsize::from_bits(val)
    }
}
impl From<Tcd13AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd13AttrSsize) -> u8 {
        Tcd13AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd13CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd13CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd13CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd13CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd13CsrBwc {
        Tcd13CsrBwc::from_bits(val)
    }
}
impl From<Tcd13CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd13CsrBwc) -> u8 {
        Tcd13CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd13CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd13CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd13CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd13CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd13CsrEsg {
        Tcd13CsrEsg::from_bits(val)
    }
}
impl From<Tcd13CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd13CsrEsg) -> u8 {
        Tcd13CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd14AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd14AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd14AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd14AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd14AttrSsize {
        Tcd14AttrSsize::from_bits(val)
    }
}
impl From<Tcd14AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd14AttrSsize) -> u8 {
        Tcd14AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd14CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd14CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd14CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd14CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd14CsrBwc {
        Tcd14CsrBwc::from_bits(val)
    }
}
impl From<Tcd14CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd14CsrBwc) -> u8 {
        Tcd14CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd14CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd14CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd14CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd14CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd14CsrEsg {
        Tcd14CsrEsg::from_bits(val)
    }
}
impl From<Tcd14CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd14CsrEsg) -> u8 {
        Tcd14CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd15AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd15AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd15AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd15AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd15AttrSsize {
        Tcd15AttrSsize::from_bits(val)
    }
}
impl From<Tcd15AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd15AttrSsize) -> u8 {
        Tcd15AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd15CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd15CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd15CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd15CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd15CsrBwc {
        Tcd15CsrBwc::from_bits(val)
    }
}
impl From<Tcd15CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd15CsrBwc) -> u8 {
        Tcd15CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd15CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd15CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd15CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd15CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd15CsrEsg {
        Tcd15CsrEsg::from_bits(val)
    }
}
impl From<Tcd15CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd15CsrEsg) -> u8 {
        Tcd15CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd1AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd1AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd1AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd1AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd1AttrSsize {
        Tcd1AttrSsize::from_bits(val)
    }
}
impl From<Tcd1AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd1AttrSsize) -> u8 {
        Tcd1AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd1CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd1CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd1CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd1CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd1CsrBwc {
        Tcd1CsrBwc::from_bits(val)
    }
}
impl From<Tcd1CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd1CsrBwc) -> u8 {
        Tcd1CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd1CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd1CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd1CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd1CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd1CsrEsg {
        Tcd1CsrEsg::from_bits(val)
    }
}
impl From<Tcd1CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd1CsrEsg) -> u8 {
        Tcd1CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd2AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd2AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd2AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd2AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd2AttrSsize {
        Tcd2AttrSsize::from_bits(val)
    }
}
impl From<Tcd2AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd2AttrSsize) -> u8 {
        Tcd2AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd2CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd2CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd2CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd2CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd2CsrBwc {
        Tcd2CsrBwc::from_bits(val)
    }
}
impl From<Tcd2CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd2CsrBwc) -> u8 {
        Tcd2CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd2CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd2CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd2CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd2CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd2CsrEsg {
        Tcd2CsrEsg::from_bits(val)
    }
}
impl From<Tcd2CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd2CsrEsg) -> u8 {
        Tcd2CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd3AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd3AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd3AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd3AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd3AttrSsize {
        Tcd3AttrSsize::from_bits(val)
    }
}
impl From<Tcd3AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd3AttrSsize) -> u8 {
        Tcd3AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd3CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd3CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd3CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd3CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd3CsrBwc {
        Tcd3CsrBwc::from_bits(val)
    }
}
impl From<Tcd3CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd3CsrBwc) -> u8 {
        Tcd3CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd3CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd3CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd3CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd3CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd3CsrEsg {
        Tcd3CsrEsg::from_bits(val)
    }
}
impl From<Tcd3CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd3CsrEsg) -> u8 {
        Tcd3CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd4AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd4AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd4AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd4AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd4AttrSsize {
        Tcd4AttrSsize::from_bits(val)
    }
}
impl From<Tcd4AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd4AttrSsize) -> u8 {
        Tcd4AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd4CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd4CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd4CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd4CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd4CsrBwc {
        Tcd4CsrBwc::from_bits(val)
    }
}
impl From<Tcd4CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd4CsrBwc) -> u8 {
        Tcd4CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd4CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd4CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd4CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd4CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd4CsrEsg {
        Tcd4CsrEsg::from_bits(val)
    }
}
impl From<Tcd4CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd4CsrEsg) -> u8 {
        Tcd4CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd5AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd5AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd5AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd5AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd5AttrSsize {
        Tcd5AttrSsize::from_bits(val)
    }
}
impl From<Tcd5AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd5AttrSsize) -> u8 {
        Tcd5AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd5CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd5CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd5CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd5CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd5CsrBwc {
        Tcd5CsrBwc::from_bits(val)
    }
}
impl From<Tcd5CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd5CsrBwc) -> u8 {
        Tcd5CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd5CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd5CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd5CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd5CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd5CsrEsg {
        Tcd5CsrEsg::from_bits(val)
    }
}
impl From<Tcd5CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd5CsrEsg) -> u8 {
        Tcd5CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd6AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd6AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd6AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd6AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd6AttrSsize {
        Tcd6AttrSsize::from_bits(val)
    }
}
impl From<Tcd6AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd6AttrSsize) -> u8 {
        Tcd6AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd6CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd6CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd6CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd6CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd6CsrBwc {
        Tcd6CsrBwc::from_bits(val)
    }
}
impl From<Tcd6CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd6CsrBwc) -> u8 {
        Tcd6CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd6CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd6CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd6CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd6CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd6CsrEsg {
        Tcd6CsrEsg::from_bits(val)
    }
}
impl From<Tcd6CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd6CsrEsg) -> u8 {
        Tcd6CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd7AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd7AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd7AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd7AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd7AttrSsize {
        Tcd7AttrSsize::from_bits(val)
    }
}
impl From<Tcd7AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd7AttrSsize) -> u8 {
        Tcd7AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd7CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd7CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd7CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd7CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd7CsrBwc {
        Tcd7CsrBwc::from_bits(val)
    }
}
impl From<Tcd7CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd7CsrBwc) -> u8 {
        Tcd7CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd7CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd7CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd7CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd7CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd7CsrEsg {
        Tcd7CsrEsg::from_bits(val)
    }
}
impl From<Tcd7CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd7CsrEsg) -> u8 {
        Tcd7CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd8AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd8AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd8AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd8AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd8AttrSsize {
        Tcd8AttrSsize::from_bits(val)
    }
}
impl From<Tcd8AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd8AttrSsize) -> u8 {
        Tcd8AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd8CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd8CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd8CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd8CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd8CsrBwc {
        Tcd8CsrBwc::from_bits(val)
    }
}
impl From<Tcd8CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd8CsrBwc) -> u8 {
        Tcd8CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd8CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd8CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd8CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd8CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd8CsrEsg {
        Tcd8CsrEsg::from_bits(val)
    }
}
impl From<Tcd8CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd8CsrEsg) -> u8 {
        Tcd8CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd9AttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tcd9AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd9AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd9AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd9AttrSsize {
        Tcd9AttrSsize::from_bits(val)
    }
}
impl From<Tcd9AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd9AttrSsize) -> u8 {
        Tcd9AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd9CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd9CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd9CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd9CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd9CsrBwc {
        Tcd9CsrBwc::from_bits(val)
    }
}
impl From<Tcd9CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd9CsrBwc) -> u8 {
        Tcd9CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd9CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd9CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd9CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd9CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd9CsrEsg {
        Tcd9CsrEsg::from_bits(val)
    }
}
impl From<Tcd9CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd9CsrEsg) -> u8 {
        Tcd9CsrEsg::to_bits(val)
    }
}
