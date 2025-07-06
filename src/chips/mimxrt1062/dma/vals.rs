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
pub enum DchpriDpa {
    #[doc = "Channel n can suspend a lower priority channel"]
    ENABLED = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    DISABLED = 0x01,
}
impl DchpriDpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DchpriDpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DchpriDpa {
    #[inline(always)]
    fn from(val: u8) -> DchpriDpa {
        DchpriDpa::from_bits(val)
    }
}
impl From<DchpriDpa> for u8 {
    #[inline(always)]
    fn from(val: DchpriDpa) -> u8 {
        DchpriDpa::to_bits(val)
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
pub enum TcdAttrSsize {
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
impl TcdAttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdAttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdAttrSsize {
    #[inline(always)]
    fn from(val: u8) -> TcdAttrSsize {
        TcdAttrSsize::from_bits(val)
    }
}
impl From<TcdAttrSsize> for u8 {
    #[inline(always)]
    fn from(val: TcdAttrSsize) -> u8 {
        TcdAttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdCsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl TcdCsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdCsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdCsrBwc {
    #[inline(always)]
    fn from(val: u8) -> TcdCsrBwc {
        TcdCsrBwc::from_bits(val)
    }
}
impl From<TcdCsrBwc> for u8 {
    #[inline(always)]
    fn from(val: TcdCsrBwc) -> u8 {
        TcdCsrBwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdCsrEsg {
    #[doc = "The current channel's TCD is normal format"]
    NORMAL = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    SCATTER = 0x01,
}
impl TcdCsrEsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdCsrEsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdCsrEsg {
    #[inline(always)]
    fn from(val: u8) -> TcdCsrEsg {
        TcdCsrEsg::from_bits(val)
    }
}
impl From<TcdCsrEsg> for u8 {
    #[inline(always)]
    fn from(val: TcdCsrEsg) -> u8 {
        TcdCsrEsg::to_bits(val)
    }
}
