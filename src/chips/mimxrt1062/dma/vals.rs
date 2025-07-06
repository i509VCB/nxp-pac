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
pub enum Dchpri16Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri16Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri16Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri16Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri16Dpa {
        Dchpri16Dpa::from_bits(val)
    }
}
impl From<Dchpri16Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri16Dpa) -> u8 {
        Dchpri16Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri17Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri17Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri17Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri17Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri17Dpa {
        Dchpri17Dpa::from_bits(val)
    }
}
impl From<Dchpri17Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri17Dpa) -> u8 {
        Dchpri17Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri18Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri18Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri18Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri18Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri18Dpa {
        Dchpri18Dpa::from_bits(val)
    }
}
impl From<Dchpri18Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri18Dpa) -> u8 {
        Dchpri18Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri19Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri19Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri19Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri19Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri19Dpa {
        Dchpri19Dpa::from_bits(val)
    }
}
impl From<Dchpri19Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri19Dpa) -> u8 {
        Dchpri19Dpa::to_bits(val)
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
pub enum Dchpri20Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri20Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri20Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri20Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri20Dpa {
        Dchpri20Dpa::from_bits(val)
    }
}
impl From<Dchpri20Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri20Dpa) -> u8 {
        Dchpri20Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri21Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri21Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri21Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri21Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri21Dpa {
        Dchpri21Dpa::from_bits(val)
    }
}
impl From<Dchpri21Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri21Dpa) -> u8 {
        Dchpri21Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri22Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri22Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri22Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri22Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri22Dpa {
        Dchpri22Dpa::from_bits(val)
    }
}
impl From<Dchpri22Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri22Dpa) -> u8 {
        Dchpri22Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri23Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri23Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri23Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri23Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri23Dpa {
        Dchpri23Dpa::from_bits(val)
    }
}
impl From<Dchpri23Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri23Dpa) -> u8 {
        Dchpri23Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri24Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri24Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri24Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri24Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri24Dpa {
        Dchpri24Dpa::from_bits(val)
    }
}
impl From<Dchpri24Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri24Dpa) -> u8 {
        Dchpri24Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri25Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri25Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri25Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri25Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri25Dpa {
        Dchpri25Dpa::from_bits(val)
    }
}
impl From<Dchpri25Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri25Dpa) -> u8 {
        Dchpri25Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri26Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri26Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri26Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri26Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri26Dpa {
        Dchpri26Dpa::from_bits(val)
    }
}
impl From<Dchpri26Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri26Dpa) -> u8 {
        Dchpri26Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri27Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri27Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri27Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri27Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri27Dpa {
        Dchpri27Dpa::from_bits(val)
    }
}
impl From<Dchpri27Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri27Dpa) -> u8 {
        Dchpri27Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri28Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri28Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri28Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri28Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri28Dpa {
        Dchpri28Dpa::from_bits(val)
    }
}
impl From<Dchpri28Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri28Dpa) -> u8 {
        Dchpri28Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri29Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri29Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri29Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri29Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri29Dpa {
        Dchpri29Dpa::from_bits(val)
    }
}
impl From<Dchpri29Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri29Dpa) -> u8 {
        Dchpri29Dpa::to_bits(val)
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
pub enum Dchpri30Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri30Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri30Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri30Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri30Dpa {
        Dchpri30Dpa::from_bits(val)
    }
}
impl From<Dchpri30Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri30Dpa) -> u8 {
        Dchpri30Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dchpri31Dpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl Dchpri31Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dchpri31Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dchpri31Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dchpri31Dpa {
        Dchpri31Dpa::from_bits(val)
    }
}
impl From<Dchpri31Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dchpri31Dpa) -> u8 {
        Dchpri31Dpa::to_bits(val)
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
pub enum Tcd0AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd0AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd0AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd0AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd0AttrSmod {
        Tcd0AttrSmod::from_bits(val)
    }
}
impl From<Tcd0AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd0AttrSmod) -> u8 {
        Tcd0AttrSmod::to_bits(val)
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
pub enum Tcd10AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd10AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd10AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd10AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd10AttrSmod {
        Tcd10AttrSmod::from_bits(val)
    }
}
impl From<Tcd10AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd10AttrSmod) -> u8 {
        Tcd10AttrSmod::to_bits(val)
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
pub enum Tcd11AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd11AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd11AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd11AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd11AttrSmod {
        Tcd11AttrSmod::from_bits(val)
    }
}
impl From<Tcd11AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd11AttrSmod) -> u8 {
        Tcd11AttrSmod::to_bits(val)
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
pub enum Tcd12AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd12AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd12AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd12AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd12AttrSmod {
        Tcd12AttrSmod::from_bits(val)
    }
}
impl From<Tcd12AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd12AttrSmod) -> u8 {
        Tcd12AttrSmod::to_bits(val)
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
pub enum Tcd13AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd13AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd13AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd13AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd13AttrSmod {
        Tcd13AttrSmod::from_bits(val)
    }
}
impl From<Tcd13AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd13AttrSmod) -> u8 {
        Tcd13AttrSmod::to_bits(val)
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
pub enum Tcd14AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd14AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd14AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd14AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd14AttrSmod {
        Tcd14AttrSmod::from_bits(val)
    }
}
impl From<Tcd14AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd14AttrSmod) -> u8 {
        Tcd14AttrSmod::to_bits(val)
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
pub enum Tcd15AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd15AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd15AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd15AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd15AttrSmod {
        Tcd15AttrSmod::from_bits(val)
    }
}
impl From<Tcd15AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd15AttrSmod) -> u8 {
        Tcd15AttrSmod::to_bits(val)
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
pub enum Tcd16AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd16AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd16AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd16AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd16AttrSmod {
        Tcd16AttrSmod::from_bits(val)
    }
}
impl From<Tcd16AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd16AttrSmod) -> u8 {
        Tcd16AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd16AttrSsize {
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
impl Tcd16AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd16AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd16AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd16AttrSsize {
        Tcd16AttrSsize::from_bits(val)
    }
}
impl From<Tcd16AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd16AttrSsize) -> u8 {
        Tcd16AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd16CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd16CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd16CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd16CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd16CsrBwc {
        Tcd16CsrBwc::from_bits(val)
    }
}
impl From<Tcd16CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd16CsrBwc) -> u8 {
        Tcd16CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd16CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd16CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd16CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd16CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd16CsrEsg {
        Tcd16CsrEsg::from_bits(val)
    }
}
impl From<Tcd16CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd16CsrEsg) -> u8 {
        Tcd16CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd17AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd17AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd17AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd17AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd17AttrSmod {
        Tcd17AttrSmod::from_bits(val)
    }
}
impl From<Tcd17AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd17AttrSmod) -> u8 {
        Tcd17AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd17AttrSsize {
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
impl Tcd17AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd17AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd17AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd17AttrSsize {
        Tcd17AttrSsize::from_bits(val)
    }
}
impl From<Tcd17AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd17AttrSsize) -> u8 {
        Tcd17AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd17CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd17CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd17CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd17CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd17CsrBwc {
        Tcd17CsrBwc::from_bits(val)
    }
}
impl From<Tcd17CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd17CsrBwc) -> u8 {
        Tcd17CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd17CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd17CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd17CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd17CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd17CsrEsg {
        Tcd17CsrEsg::from_bits(val)
    }
}
impl From<Tcd17CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd17CsrEsg) -> u8 {
        Tcd17CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd18AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd18AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd18AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd18AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd18AttrSmod {
        Tcd18AttrSmod::from_bits(val)
    }
}
impl From<Tcd18AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd18AttrSmod) -> u8 {
        Tcd18AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd18AttrSsize {
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
impl Tcd18AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd18AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd18AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd18AttrSsize {
        Tcd18AttrSsize::from_bits(val)
    }
}
impl From<Tcd18AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd18AttrSsize) -> u8 {
        Tcd18AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd18CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd18CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd18CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd18CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd18CsrBwc {
        Tcd18CsrBwc::from_bits(val)
    }
}
impl From<Tcd18CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd18CsrBwc) -> u8 {
        Tcd18CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd18CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd18CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd18CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd18CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd18CsrEsg {
        Tcd18CsrEsg::from_bits(val)
    }
}
impl From<Tcd18CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd18CsrEsg) -> u8 {
        Tcd18CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd19AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd19AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd19AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd19AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd19AttrSmod {
        Tcd19AttrSmod::from_bits(val)
    }
}
impl From<Tcd19AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd19AttrSmod) -> u8 {
        Tcd19AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd19AttrSsize {
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
impl Tcd19AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd19AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd19AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd19AttrSsize {
        Tcd19AttrSsize::from_bits(val)
    }
}
impl From<Tcd19AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd19AttrSsize) -> u8 {
        Tcd19AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd19CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd19CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd19CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd19CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd19CsrBwc {
        Tcd19CsrBwc::from_bits(val)
    }
}
impl From<Tcd19CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd19CsrBwc) -> u8 {
        Tcd19CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd19CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd19CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd19CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd19CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd19CsrEsg {
        Tcd19CsrEsg::from_bits(val)
    }
}
impl From<Tcd19CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd19CsrEsg) -> u8 {
        Tcd19CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd1AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd1AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd1AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd1AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd1AttrSmod {
        Tcd1AttrSmod::from_bits(val)
    }
}
impl From<Tcd1AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd1AttrSmod) -> u8 {
        Tcd1AttrSmod::to_bits(val)
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
pub enum Tcd20AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd20AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd20AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd20AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd20AttrSmod {
        Tcd20AttrSmod::from_bits(val)
    }
}
impl From<Tcd20AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd20AttrSmod) -> u8 {
        Tcd20AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd20AttrSsize {
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
impl Tcd20AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd20AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd20AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd20AttrSsize {
        Tcd20AttrSsize::from_bits(val)
    }
}
impl From<Tcd20AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd20AttrSsize) -> u8 {
        Tcd20AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd20CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd20CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd20CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd20CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd20CsrBwc {
        Tcd20CsrBwc::from_bits(val)
    }
}
impl From<Tcd20CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd20CsrBwc) -> u8 {
        Tcd20CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd20CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd20CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd20CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd20CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd20CsrEsg {
        Tcd20CsrEsg::from_bits(val)
    }
}
impl From<Tcd20CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd20CsrEsg) -> u8 {
        Tcd20CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd21AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd21AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd21AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd21AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd21AttrSmod {
        Tcd21AttrSmod::from_bits(val)
    }
}
impl From<Tcd21AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd21AttrSmod) -> u8 {
        Tcd21AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd21AttrSsize {
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
impl Tcd21AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd21AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd21AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd21AttrSsize {
        Tcd21AttrSsize::from_bits(val)
    }
}
impl From<Tcd21AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd21AttrSsize) -> u8 {
        Tcd21AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd21CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd21CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd21CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd21CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd21CsrBwc {
        Tcd21CsrBwc::from_bits(val)
    }
}
impl From<Tcd21CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd21CsrBwc) -> u8 {
        Tcd21CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd21CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd21CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd21CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd21CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd21CsrEsg {
        Tcd21CsrEsg::from_bits(val)
    }
}
impl From<Tcd21CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd21CsrEsg) -> u8 {
        Tcd21CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd22AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd22AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd22AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd22AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd22AttrSmod {
        Tcd22AttrSmod::from_bits(val)
    }
}
impl From<Tcd22AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd22AttrSmod) -> u8 {
        Tcd22AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd22AttrSsize {
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
impl Tcd22AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd22AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd22AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd22AttrSsize {
        Tcd22AttrSsize::from_bits(val)
    }
}
impl From<Tcd22AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd22AttrSsize) -> u8 {
        Tcd22AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd22CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd22CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd22CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd22CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd22CsrBwc {
        Tcd22CsrBwc::from_bits(val)
    }
}
impl From<Tcd22CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd22CsrBwc) -> u8 {
        Tcd22CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd22CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd22CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd22CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd22CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd22CsrEsg {
        Tcd22CsrEsg::from_bits(val)
    }
}
impl From<Tcd22CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd22CsrEsg) -> u8 {
        Tcd22CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd23AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd23AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd23AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd23AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd23AttrSmod {
        Tcd23AttrSmod::from_bits(val)
    }
}
impl From<Tcd23AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd23AttrSmod) -> u8 {
        Tcd23AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd23AttrSsize {
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
impl Tcd23AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd23AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd23AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd23AttrSsize {
        Tcd23AttrSsize::from_bits(val)
    }
}
impl From<Tcd23AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd23AttrSsize) -> u8 {
        Tcd23AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd23CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd23CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd23CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd23CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd23CsrBwc {
        Tcd23CsrBwc::from_bits(val)
    }
}
impl From<Tcd23CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd23CsrBwc) -> u8 {
        Tcd23CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd23CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd23CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd23CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd23CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd23CsrEsg {
        Tcd23CsrEsg::from_bits(val)
    }
}
impl From<Tcd23CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd23CsrEsg) -> u8 {
        Tcd23CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd24AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd24AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd24AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd24AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd24AttrSmod {
        Tcd24AttrSmod::from_bits(val)
    }
}
impl From<Tcd24AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd24AttrSmod) -> u8 {
        Tcd24AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd24AttrSsize {
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
impl Tcd24AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd24AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd24AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd24AttrSsize {
        Tcd24AttrSsize::from_bits(val)
    }
}
impl From<Tcd24AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd24AttrSsize) -> u8 {
        Tcd24AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd24CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd24CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd24CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd24CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd24CsrBwc {
        Tcd24CsrBwc::from_bits(val)
    }
}
impl From<Tcd24CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd24CsrBwc) -> u8 {
        Tcd24CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd24CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd24CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd24CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd24CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd24CsrEsg {
        Tcd24CsrEsg::from_bits(val)
    }
}
impl From<Tcd24CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd24CsrEsg) -> u8 {
        Tcd24CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd25AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd25AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd25AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd25AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd25AttrSmod {
        Tcd25AttrSmod::from_bits(val)
    }
}
impl From<Tcd25AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd25AttrSmod) -> u8 {
        Tcd25AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd25AttrSsize {
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
impl Tcd25AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd25AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd25AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd25AttrSsize {
        Tcd25AttrSsize::from_bits(val)
    }
}
impl From<Tcd25AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd25AttrSsize) -> u8 {
        Tcd25AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd25CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd25CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd25CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd25CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd25CsrBwc {
        Tcd25CsrBwc::from_bits(val)
    }
}
impl From<Tcd25CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd25CsrBwc) -> u8 {
        Tcd25CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd25CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd25CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd25CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd25CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd25CsrEsg {
        Tcd25CsrEsg::from_bits(val)
    }
}
impl From<Tcd25CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd25CsrEsg) -> u8 {
        Tcd25CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd26AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd26AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd26AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd26AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd26AttrSmod {
        Tcd26AttrSmod::from_bits(val)
    }
}
impl From<Tcd26AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd26AttrSmod) -> u8 {
        Tcd26AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd26AttrSsize {
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
impl Tcd26AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd26AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd26AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd26AttrSsize {
        Tcd26AttrSsize::from_bits(val)
    }
}
impl From<Tcd26AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd26AttrSsize) -> u8 {
        Tcd26AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd26CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd26CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd26CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd26CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd26CsrBwc {
        Tcd26CsrBwc::from_bits(val)
    }
}
impl From<Tcd26CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd26CsrBwc) -> u8 {
        Tcd26CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd26CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd26CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd26CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd26CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd26CsrEsg {
        Tcd26CsrEsg::from_bits(val)
    }
}
impl From<Tcd26CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd26CsrEsg) -> u8 {
        Tcd26CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd27AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd27AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd27AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd27AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd27AttrSmod {
        Tcd27AttrSmod::from_bits(val)
    }
}
impl From<Tcd27AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd27AttrSmod) -> u8 {
        Tcd27AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd27AttrSsize {
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
impl Tcd27AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd27AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd27AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd27AttrSsize {
        Tcd27AttrSsize::from_bits(val)
    }
}
impl From<Tcd27AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd27AttrSsize) -> u8 {
        Tcd27AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd27CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd27CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd27CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd27CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd27CsrBwc {
        Tcd27CsrBwc::from_bits(val)
    }
}
impl From<Tcd27CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd27CsrBwc) -> u8 {
        Tcd27CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd27CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd27CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd27CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd27CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd27CsrEsg {
        Tcd27CsrEsg::from_bits(val)
    }
}
impl From<Tcd27CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd27CsrEsg) -> u8 {
        Tcd27CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd28AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd28AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd28AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd28AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd28AttrSmod {
        Tcd28AttrSmod::from_bits(val)
    }
}
impl From<Tcd28AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd28AttrSmod) -> u8 {
        Tcd28AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd28AttrSsize {
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
impl Tcd28AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd28AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd28AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd28AttrSsize {
        Tcd28AttrSsize::from_bits(val)
    }
}
impl From<Tcd28AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd28AttrSsize) -> u8 {
        Tcd28AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd28CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd28CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd28CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd28CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd28CsrBwc {
        Tcd28CsrBwc::from_bits(val)
    }
}
impl From<Tcd28CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd28CsrBwc) -> u8 {
        Tcd28CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd28CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd28CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd28CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd28CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd28CsrEsg {
        Tcd28CsrEsg::from_bits(val)
    }
}
impl From<Tcd28CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd28CsrEsg) -> u8 {
        Tcd28CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd29AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd29AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd29AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd29AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd29AttrSmod {
        Tcd29AttrSmod::from_bits(val)
    }
}
impl From<Tcd29AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd29AttrSmod) -> u8 {
        Tcd29AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd29AttrSsize {
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
impl Tcd29AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd29AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd29AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd29AttrSsize {
        Tcd29AttrSsize::from_bits(val)
    }
}
impl From<Tcd29AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd29AttrSsize) -> u8 {
        Tcd29AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd29CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd29CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd29CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd29CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd29CsrBwc {
        Tcd29CsrBwc::from_bits(val)
    }
}
impl From<Tcd29CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd29CsrBwc) -> u8 {
        Tcd29CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd29CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd29CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd29CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd29CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd29CsrEsg {
        Tcd29CsrEsg::from_bits(val)
    }
}
impl From<Tcd29CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd29CsrEsg) -> u8 {
        Tcd29CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd2AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd2AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd2AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd2AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd2AttrSmod {
        Tcd2AttrSmod::from_bits(val)
    }
}
impl From<Tcd2AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd2AttrSmod) -> u8 {
        Tcd2AttrSmod::to_bits(val)
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
pub enum Tcd30AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd30AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd30AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd30AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd30AttrSmod {
        Tcd30AttrSmod::from_bits(val)
    }
}
impl From<Tcd30AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd30AttrSmod) -> u8 {
        Tcd30AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd30AttrSsize {
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
impl Tcd30AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd30AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd30AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd30AttrSsize {
        Tcd30AttrSsize::from_bits(val)
    }
}
impl From<Tcd30AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd30AttrSsize) -> u8 {
        Tcd30AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd30CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd30CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd30CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd30CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd30CsrBwc {
        Tcd30CsrBwc::from_bits(val)
    }
}
impl From<Tcd30CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd30CsrBwc) -> u8 {
        Tcd30CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd30CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd30CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd30CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd30CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd30CsrEsg {
        Tcd30CsrEsg::from_bits(val)
    }
}
impl From<Tcd30CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd30CsrEsg) -> u8 {
        Tcd30CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd31AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd31AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd31AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd31AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd31AttrSmod {
        Tcd31AttrSmod::from_bits(val)
    }
}
impl From<Tcd31AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd31AttrSmod) -> u8 {
        Tcd31AttrSmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd31AttrSsize {
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
impl Tcd31AttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd31AttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd31AttrSsize {
    #[inline(always)]
    fn from(val: u8) -> Tcd31AttrSsize {
        Tcd31AttrSsize::from_bits(val)
    }
}
impl From<Tcd31AttrSsize> for u8 {
    #[inline(always)]
    fn from(val: Tcd31AttrSsize) -> u8 {
        Tcd31AttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd31CsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl Tcd31CsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd31CsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd31CsrBwc {
    #[inline(always)]
    fn from(val: u8) -> Tcd31CsrBwc {
        Tcd31CsrBwc::from_bits(val)
    }
}
impl From<Tcd31CsrBwc> for u8 {
    #[inline(always)]
    fn from(val: Tcd31CsrBwc) -> u8 {
        Tcd31CsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd31CsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl Tcd31CsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd31CsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd31CsrEsg {
    #[inline(always)]
    fn from(val: u8) -> Tcd31CsrEsg {
        Tcd31CsrEsg::from_bits(val)
    }
}
impl From<Tcd31CsrEsg> for u8 {
    #[inline(always)]
    fn from(val: Tcd31CsrEsg) -> u8 {
        Tcd31CsrEsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcd3AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd3AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd3AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd3AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd3AttrSmod {
        Tcd3AttrSmod::from_bits(val)
    }
}
impl From<Tcd3AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd3AttrSmod) -> u8 {
        Tcd3AttrSmod::to_bits(val)
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
pub enum Tcd4AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd4AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd4AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd4AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd4AttrSmod {
        Tcd4AttrSmod::from_bits(val)
    }
}
impl From<Tcd4AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd4AttrSmod) -> u8 {
        Tcd4AttrSmod::to_bits(val)
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
pub enum Tcd5AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd5AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd5AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd5AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd5AttrSmod {
        Tcd5AttrSmod::from_bits(val)
    }
}
impl From<Tcd5AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd5AttrSmod) -> u8 {
        Tcd5AttrSmod::to_bits(val)
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
pub enum Tcd6AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd6AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd6AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd6AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd6AttrSmod {
        Tcd6AttrSmod::from_bits(val)
    }
}
impl From<Tcd6AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd6AttrSmod) -> u8 {
        Tcd6AttrSmod::to_bits(val)
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
pub enum Tcd7AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd7AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd7AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd7AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd7AttrSmod {
        Tcd7AttrSmod::from_bits(val)
    }
}
impl From<Tcd7AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd7AttrSmod) -> u8 {
        Tcd7AttrSmod::to_bits(val)
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
pub enum Tcd8AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd8AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd8AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd8AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd8AttrSmod {
        Tcd8AttrSmod::from_bits(val)
    }
}
impl From<Tcd8AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd8AttrSmod) -> u8 {
        Tcd8AttrSmod::to_bits(val)
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
pub enum Tcd9AttrSmod {
    #[doc = "Source address modulo feature is disabled"]
    DISABLED = 0x0,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue"]
    ENABLED_9 = 0x09,
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
impl Tcd9AttrSmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcd9AttrSmod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcd9AttrSmod {
    #[inline(always)]
    fn from(val: u8) -> Tcd9AttrSmod {
        Tcd9AttrSmod::from_bits(val)
    }
}
impl From<Tcd9AttrSmod> for u8 {
    #[inline(always)]
    fn from(val: Tcd9AttrSmod) -> u8 {
        Tcd9AttrSmod::to_bits(val)
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
