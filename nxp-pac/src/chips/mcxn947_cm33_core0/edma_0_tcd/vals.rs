#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls."]
    NoStall = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    EngineStallsFour = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    EngineStallsEight = 0x03,
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
    #[doc = "Channel can suspend a lower-priority channel."]
    Suspend = 0x0,
    #[doc = "Channel cannot suspend any other channel, regardless of channel priority."]
    CannotSuspend = 0x01,
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
    #[doc = "No operation."]
    ChannelNotAffected = 0x0,
    #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests."]
    ErqFieldClear = 0x01,
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
    #[doc = "Channel cannot be suspended by a higher-priority channel's service request."]
    CannotSuspend = 0x0,
    #[doc = "Channel can be temporarily suspended by a higher-priority channel's service request."]
    Suspend = 0x01,
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
    #[doc = "Current channel's TCD is normal format."]
    NormalFormat = 0x0,
    #[doc = "Current channel's TCD specifies scatter/gather format."]
    ScatterGatherFormat = 0x01,
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
pub enum Int {
    #[doc = "Interrupt request for corresponding channel cleared."]
    InterruptCleared = 0x0,
    #[doc = "Interrupt request for corresponding channel active."]
    InterruptActive = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pal {
    #[doc = "User protection level for DMA transfers."]
    UserProtection = 0x0,
    #[doc = "Privileged protection level for DMA transfers."]
    PrivilegedProtection = 0x01,
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
pub enum Sec {
    #[doc = "Nonsecure protection level for DMA transfers."]
    NonsecureProtection = 0x0,
    #[doc = "Secure protection level for DMA transfers."]
    SecureProtection = 0x01,
}
impl Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sec {
    #[inline(always)]
    fn from(val: u8) -> Sec {
        Sec::from_bits(val)
    }
}
impl From<Sec> for u8 {
    #[inline(always)]
    fn from(val: Sec) -> u8 {
        Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Source address modulo feature disabled."]
    Disable = 0x0,
    #[doc = "Source address modulo feature enabled for any non-zero value \\[1-31\\]."]
    Enable = 0x01,
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
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "8-bit."]
    EightBit = 0x0,
    #[doc = "16-bit."]
    SixteenBit = 0x01,
    #[doc = "32-bit."]
    ThirtytwoBit = 0x02,
    #[doc = "64-bit."]
    SixtyfourBit = 0x03,
    #[doc = "16-byte."]
    SixteenByte = 0x04,
    #[doc = "32-byte."]
    ThirtytwoByte = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Channel not explicitly started."]
    ChannelNotStarted = 0x0,
    #[doc = "Channel explicitly started via a software-initiated service request."]
    ChannelStarted = 0x01,
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
    #[doc = "Minor loop offset not applied to DADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to DADDR."]
    OffsetApplied = 0x01,
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
    #[doc = "Minor loop offset not applied to SADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to SADDR."]
    OffsetApplied = 0x01,
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
    #[doc = "Minor loop offset not applied to DADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to DADDR."]
    OffsetApplied = 0x01,
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
    #[doc = "Minor loop offset not applied to SADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to SADDR."]
    OffsetApplied = 0x01,
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
