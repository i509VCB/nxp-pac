#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ByteOrder {
    #[doc = "Little endian"]
    LIT = 0x0,
    #[doc = "Big endian"]
    BIG = 0x01,
}
impl ByteOrder {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ByteOrder {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ByteOrder {
    #[inline(always)]
    fn from(val: u8) -> ByteOrder {
        ByteOrder::from_bits(val)
    }
}
impl From<ByteOrder> for u8 {
    #[inline(always)]
    fn from(val: ByteOrder) -> u8 {
        ByteOrder::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdcrcEn {
    #[doc = "Disables the CRC command CRC. The CRC command will not be updated on completion of each ELS command."]
    EXIT = 0x0,
    #[doc = "Enables the CRC command. The CRC command will be updated on completion of each ELS command."]
    CLR = 0x01,
}
impl CmdcrcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdcrcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdcrcEn {
    #[inline(always)]
    fn from(val: u8) -> CmdcrcEn {
        CmdcrcEn::from_bits(val)
    }
}
impl From<CmdcrcEn> for u8 {
    #[inline(always)]
    fn from(val: CmdcrcEn) -> u8 {
        CmdcrcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdcrcRst {
    #[doc = "No effect"]
    EXIT = 0x0,
    #[doc = "Resets the CRC command to its default value"]
    CLR = 0x01,
}
impl CmdcrcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdcrcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdcrcRst {
    #[inline(always)]
    fn from(val: u8) -> CmdcrcRst {
        CmdcrcRst::from_bits(val)
    }
}
impl From<CmdcrcRst> for u8 {
    #[inline(always)]
    fn from(val: CmdcrcRst) -> u8 {
        CmdcrcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DrbgEntLvl {
    #[doc = "NONE"]
    NONE = 0x0,
    #[doc = "LOW, DRBG generates random numbers of low quality entropy"]
    LOW = 0x01,
    #[doc = "HIGH, DRBG generates random numbers of high quality entropy"]
    HIGH = 0x02,
    #[doc = "RFU, Reserved for Future Use"]
    RFU = 0x03,
}
impl DrbgEntLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DrbgEntLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DrbgEntLvl {
    #[inline(always)]
    fn from(val: u8) -> DrbgEntLvl {
        DrbgEntLvl::from_bits(val)
    }
}
impl From<DrbgEntLvl> for u8 {
    #[inline(always)]
    fn from(val: DrbgEntLvl) -> u8 {
        DrbgEntLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcdsaVfyStatus {
    #[doc = "No verify run"]
    NO_V_RUN = 0x0,
    #[doc = "Signature verify failed"]
    SIG_FAIL = 0x01,
    #[doc = "Signature verify passed"]
    SIG_PASS = 0x02,
    #[doc = "Invalid, Error"]
    ERR = 0x03,
}
impl EcdsaVfyStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcdsaVfyStatus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcdsaVfyStatus {
    #[inline(always)]
    fn from(val: u8) -> EcdsaVfyStatus {
        EcdsaVfyStatus::from_bits(val)
    }
}
impl From<EcdsaVfyStatus> for u8 {
    #[inline(always)]
    fn from(val: EcdsaVfyStatus) -> u8 {
        EcdsaVfyStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsBusy {
    #[doc = "Crypto sequence not executing"]
    NTCRY = 0x0,
    #[doc = "Crypto sequence executing"]
    CRYP = 0x01,
}
impl ElsBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsBusy {
    #[inline(always)]
    fn from(val: u8) -> ElsBusy {
        ElsBusy::from_bits(val)
    }
}
impl From<ElsBusy> for u8 {
    #[inline(always)]
    fn from(val: ElsBusy) -> u8 {
        ElsBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsErr {
    #[doc = "Internal error not detected"]
    NTERR = 0x0,
    #[doc = "Internal error detected"]
    ERR = 0x01,
}
impl ElsErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsErr {
    #[inline(always)]
    fn from(val: u8) -> ElsErr {
        ElsErr::from_bits(val)
    }
}
impl From<ElsErr> for u8 {
    #[inline(always)]
    fn from(val: ElsErr) -> u8 {
        ElsErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsIrq {
    #[doc = "No active interrupt"]
    NTINT = 0x0,
    #[doc = "Active interrupt"]
    INT = 0x01,
}
impl ElsIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsIrq {
    #[inline(always)]
    fn from(val: u8) -> ElsIrq {
        ElsIrq::from_bits(val)
    }
}
impl From<ElsIrq> for u8 {
    #[inline(always)]
    fn from(val: ElsIrq) -> u8 {
        ElsIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsLocked {
    #[doc = "Not locked by master"]
    NOTL = 0x0,
    #[doc = "Locked by master"]
    LOCK = 0x01,
}
impl ElsLocked {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsLocked {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsLocked {
    #[inline(always)]
    fn from(val: u8) -> ElsLocked {
        ElsLocked::from_bits(val)
    }
}
impl From<ElsLocked> for u8 {
    #[inline(always)]
    fn from(val: ElsLocked) -> u8 {
        ElsLocked::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrClr {
    #[doc = "Exits ELS error state"]
    EXIT = 0x0,
    #[doc = "Clears ELS error state"]
    CLR = 0x01,
}
impl ErrClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrClr {
    #[inline(always)]
    fn from(val: u8) -> ErrClr {
        ErrClr::from_bits(val)
    }
}
impl From<ErrClr> for u8 {
    #[inline(always)]
    fn from(val: ErrClr) -> u8 {
        ErrClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks0Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks0Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks0Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks0Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks0Ksize {
        Ks0Ksize::from_bits(val)
    }
}
impl From<Ks0Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks0Ksize) -> u8 {
        Ks0Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks10Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks10Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks10Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks10Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks10Ksize {
        Ks10Ksize::from_bits(val)
    }
}
impl From<Ks10Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks10Ksize) -> u8 {
        Ks10Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks11Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks11Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks11Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks11Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks11Ksize {
        Ks11Ksize::from_bits(val)
    }
}
impl From<Ks11Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks11Ksize) -> u8 {
        Ks11Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks12Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks12Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks12Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks12Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks12Ksize {
        Ks12Ksize::from_bits(val)
    }
}
impl From<Ks12Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks12Ksize) -> u8 {
        Ks12Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks13Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks13Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks13Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks13Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks13Ksize {
        Ks13Ksize::from_bits(val)
    }
}
impl From<Ks13Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks13Ksize) -> u8 {
        Ks13Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks14Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks14Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks14Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks14Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks14Ksize {
        Ks14Ksize::from_bits(val)
    }
}
impl From<Ks14Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks14Ksize) -> u8 {
        Ks14Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks15Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks15Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks15Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks15Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks15Ksize {
        Ks15Ksize::from_bits(val)
    }
}
impl From<Ks15Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks15Ksize) -> u8 {
        Ks15Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks16Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks16Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks16Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks16Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks16Ksize {
        Ks16Ksize::from_bits(val)
    }
}
impl From<Ks16Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks16Ksize) -> u8 {
        Ks16Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks17Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks17Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks17Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks17Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks17Ksize {
        Ks17Ksize::from_bits(val)
    }
}
impl From<Ks17Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks17Ksize) -> u8 {
        Ks17Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks18Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks18Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks18Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks18Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks18Ksize {
        Ks18Ksize::from_bits(val)
    }
}
impl From<Ks18Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks18Ksize) -> u8 {
        Ks18Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks19Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks19Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks19Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks19Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks19Ksize {
        Ks19Ksize::from_bits(val)
    }
}
impl From<Ks19Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks19Ksize) -> u8 {
        Ks19Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks1Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks1Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks1Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks1Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks1Ksize {
        Ks1Ksize::from_bits(val)
    }
}
impl From<Ks1Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks1Ksize) -> u8 {
        Ks1Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks2Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks2Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks2Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks2Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks2Ksize {
        Ks2Ksize::from_bits(val)
    }
}
impl From<Ks2Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks2Ksize) -> u8 {
        Ks2Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks3Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks3Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks3Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks3Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks3Ksize {
        Ks3Ksize::from_bits(val)
    }
}
impl From<Ks3Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks3Ksize) -> u8 {
        Ks3Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks4Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks4Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks4Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks4Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks4Ksize {
        Ks4Ksize::from_bits(val)
    }
}
impl From<Ks4Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks4Ksize) -> u8 {
        Ks4Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks5Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks5Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks5Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks5Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks5Ksize {
        Ks5Ksize::from_bits(val)
    }
}
impl From<Ks5Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks5Ksize) -> u8 {
        Ks5Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks6Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks6Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks6Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks6Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks6Ksize {
        Ks6Ksize::from_bits(val)
    }
}
impl From<Ks6Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks6Ksize) -> u8 {
        Ks6Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks7Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks7Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks7Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks7Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks7Ksize {
        Ks7Ksize::from_bits(val)
    }
}
impl From<Ks7Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks7Ksize) -> u8 {
        Ks7Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks8Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks8Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks8Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks8Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks8Ksize {
        Ks8Ksize::from_bits(val)
    }
}
impl From<Ks8Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks8Ksize) -> u8 {
        Ks8Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks9Ksize {
    #[doc = "128"]
    SIZE128 = 0x0,
    #[doc = "256"]
    SIZE256 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ks9Ksize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks9Ksize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks9Ksize {
    #[inline(always)]
    fn from(val: u8) -> Ks9Ksize {
        Ks9Ksize::from_bits(val)
    }
}
impl From<Ks9Ksize> for u8 {
    #[inline(always)]
    fn from(val: Ks9Ksize) -> u8 {
        Ks9Ksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pprot {
    #[doc = "Secure, non-privileged"]
    SECNP = 0x0,
    #[doc = "Secure, privileged"]
    SECP = 0x01,
    #[doc = "Non-secure, non-privileged"]
    NSECNP = 0x02,
    #[doc = "Non-secure, privileged"]
    NSECP = 0x03,
}
impl Pprot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pprot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pprot {
    #[inline(always)]
    fn from(val: u8) -> Pprot {
        Pprot::from_bits(val)
    }
}
impl From<Pprot> for u8 {
    #[inline(always)]
    fn from(val: Pprot) -> u8 {
        Pprot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrngRdy {
    #[doc = "Internal PRNG not ready"]
    NTREADY = 0x0,
    #[doc = "Internal PRNG ready"]
    READY = 0x01,
}
impl PrngRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrngRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrngRdy {
    #[inline(always)]
    fn from(val: u8) -> PrngRdy {
        PrngRdy::from_bits(val)
    }
}
impl From<PrngRdy> for u8 {
    #[inline(always)]
    fn from(val: PrngRdy) -> u8 {
        PrngRdy::to_bits(val)
    }
}
