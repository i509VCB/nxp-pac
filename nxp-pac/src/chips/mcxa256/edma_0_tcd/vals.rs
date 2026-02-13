#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls"]
    NO_STALL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    ENGINE_STALLS_FOUR = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    ENGINE_STALLS_EIGHT = 0x03,
}
impl Bwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwc {
    #[inline(always)]
    fn from(val: u8) -> Bwc {
        Bwc::from_bits(val)
    }
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(val: Bwc) -> u8 {
        Bwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpa {
    #[doc = "Channel can suspend a lower-priority channel"]
    SUSPEND = 0x0,
    #[doc = "Channel cannot suspend any other channel, regardless of channel priority"]
    CANNOT_SUSPEND = 0x01,
}
impl Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dpa {
        Dpa::from_bits(val)
    }
}
impl From<Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dpa) -> u8 {
        Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dreq {
    #[doc = "No operation"]
    CHANNEL_NOT_AFFECTED = 0x0,
    #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests"]
    ERQ_FIELD_CLEAR = 0x01,
}
impl Dreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dreq {
    #[inline(always)]
    fn from(val: u8) -> Dreq {
        Dreq::from_bits(val)
    }
}
impl From<Dreq> for u8 {
    #[inline(always)]
    fn from(val: Dreq) -> u8 {
        Dreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecp {
    #[doc = "Channel cannot be suspended by a higher-priority channel's service request"]
    CANNOT_SUSPEND = 0x0,
    #[doc = "Channel can be temporarily suspended by a higher-priority channel's service request"]
    SUSPEND = 0x01,
}
impl Ecp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecp {
    #[inline(always)]
    fn from(val: u8) -> Ecp {
        Ecp::from_bits(val)
    }
}
impl From<Ecp> for u8 {
    #[inline(always)]
    fn from(val: Ecp) -> u8 {
        Ecp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esg {
    #[doc = "Current channel's TCD is normal format"]
    NORMAL_FORMAT = 0x0,
    #[doc = "Current channel's TCD specifies scatter/gather format."]
    SCATTER_GATHER_FORMAT = 0x01,
}
impl Esg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esg {
    #[inline(always)]
    fn from(val: u8) -> Esg {
        Esg::from_bits(val)
    }
}
impl From<Esg> for u8 {
    #[inline(always)]
    fn from(val: Esg) -> u8 {
        Esg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pal {
    #[doc = "User protection level for DMA transfers"]
    USER_PROTECTION = 0x0,
    #[doc = "Privileged protection level for DMA transfers"]
    PRIVILEGED_PROTECTION = 0x01,
}
impl Pal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pal {
    #[inline(always)]
    fn from(val: u8) -> Pal {
        Pal::from_bits(val)
    }
}
impl From<Pal> for u8 {
    #[inline(always)]
    fn from(val: Pal) -> u8 {
        Pal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Size {
    #[doc = "8-bit"]
    EIGHT_BIT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR_BIT = 0x03,
    #[doc = "16-byte"]
    SIXTEEN_BYTE = 0x04,
    #[doc = "32-byte"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Size {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Size {
    #[inline(always)]
    fn from(val: u8) -> Size {
        Size::from_bits(val)
    }
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(val: Size) -> u8 {
        Size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Channel not explicitly started"]
    CHANNEL_NOT_STARTED = 0x0,
    #[doc = "Channel explicitly started via a software-initiated service request"]
    CHANNEL_STARTED = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoDmloe {
    #[doc = "Minor loop offset not applied to DADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to DADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffnoDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoDmloe {
        TcdNbytesMloffnoDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoDmloe) -> u8 {
        TcdNbytesMloffnoDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoSmloe {
    #[doc = "Minor loop offset not applied to SADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to SADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffnoSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoSmloe {
        TcdNbytesMloffnoSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoSmloe) -> u8 {
        TcdNbytesMloffnoSmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesDmloe {
    #[doc = "Minor loop offset not applied to DADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to DADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffyesDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesDmloe {
        TcdNbytesMloffyesDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesDmloe) -> u8 {
        TcdNbytesMloffyesDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesSmloe {
    #[doc = "Minor loop offset not applied to SADDR"]
    OFFSET_NOT_APPLIED = 0x0,
    #[doc = "Minor loop offset applied to SADDR"]
    OFFSET_APPLIED = 0x01,
}
impl TcdNbytesMloffyesSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesSmloe {
        TcdNbytesMloffyesSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesSmloe) -> u8 {
        TcdNbytesMloffyesSmloe::to_bits(val)
    }
}
