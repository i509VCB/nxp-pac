#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesdecrypt {
    #[doc = "Encrypt"]
    ENCRYPT = 0x0,
    #[doc = "Decrypt"]
    DECRYPT = 0x01,
}
impl Aesdecrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesdecrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesdecrypt {
    #[inline(always)]
    fn from(val: u8) -> Aesdecrypt {
        Aesdecrypt::from_bits(val)
    }
}
impl From<Aesdecrypt> for u8 {
    #[inline(always)]
    fn from(val: Aesdecrypt) -> u8 {
        Aesdecrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aeskeysz {
    #[doc = "128 bit key"]
    BITS_128 = 0x0,
    #[doc = "192 bit key"]
    BITS_192 = 0x01,
    #[doc = "256 bit key"]
    BITS_256 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aeskeysz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aeskeysz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aeskeysz {
    #[inline(always)]
    fn from(val: u8) -> Aeskeysz {
        Aeskeysz::from_bits(val)
    }
}
impl From<Aeskeysz> for u8 {
    #[inline(always)]
    fn from(val: Aeskeysz) -> u8 {
        Aeskeysz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesmode {
    #[doc = "ECB - used as is"]
    ECB = 0x0,
    #[doc = "CBC mode (see details on IV/nonce)"]
    CBC = 0x01,
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aesmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesmode {
    #[inline(always)]
    fn from(val: u8) -> Aesmode {
        Aesmode::from_bits(val)
    }
}
impl From<Aesmode> for u8 {
    #[inline(always)]
    fn from(val: Aesmode) -> u8 {
        Aesmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aessecret {
    #[doc = "User key provided in normal way"]
    NORMAL_WAY = 0x0,
    #[doc = "Secret key provided in hidden way by HW"]
    HIDDEN_WAY = 0x01,
}
impl Aessecret {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aessecret {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aessecret {
    #[inline(always)]
    fn from(val: u8) -> Aessecret {
        Aessecret::from_bits(val)
    }
}
impl From<Aessecret> for u8 {
    #[inline(always)]
    fn from(val: Aessecret) -> u8 {
        Aessecret::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaI {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0x0,
    #[doc = "DMA will push in the data."]
    PUSH = 0x01,
}
impl DmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaI {
    #[inline(always)]
    fn from(val: u8) -> DmaI {
        DmaI::from_bits(val)
    }
}
impl From<DmaI> for u8 {
    #[inline(always)]
    fn from(val: DmaI) -> u8 {
        DmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icbstrm {
    #[doc = "8 blocks"]
    BLOCKS_8 = 0x0,
    #[doc = "16 blocks"]
    BLOCKS_16 = 0x01,
    #[doc = "32 blocks"]
    BLOCKS_32 = 0x02,
    #[doc = "64 blocks"]
    BLOCKS_64 = 0x03,
}
impl Icbstrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icbstrm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icbstrm {
    #[inline(always)]
    fn from(val: u8) -> Icbstrm {
        Icbstrm::from_bits(val)
    }
}
impl From<Icbstrm> for u8 {
    #[inline(always)]
    fn from(val: Icbstrm) -> u8 {
        Icbstrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icbsz {
    #[doc = "32 bits of the IV/ctr are used (from 127:96)"]
    BITS_32 = 0x0,
    #[doc = "64 bits of the IV/ctr are used (from 127:64)"]
    BITS_64 = 0x01,
    #[doc = "96 bits of the IV/ctr are used (from 127:32)"]
    BITS_96 = 0x02,
    #[doc = "All 128 bits of the IV/ctr are used"]
    BIT_128 = 0x03,
}
impl Icbsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icbsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icbsz {
    #[inline(always)]
    fn from(val: u8) -> Icbsz {
        Icbsz::from_bits(val)
    }
}
impl From<Icbsz> for u8 {
    #[inline(always)]
    fn from(val: Icbsz) -> u8 {
        Icbsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED = 0x0,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "SHA1 is enabled"]
    SHA1 = 0x01,
    #[doc = "SHA2-256 is enabled"]
    SHA2_256 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    AES = 0x04,
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    ICB_AES = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seclock {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK = 0x0,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Seclock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seclock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seclock {
    #[inline(always)]
    fn from(val: u8) -> Seclock {
        Seclock::from_bits(val)
    }
}
impl From<Seclock> for u8 {
    #[inline(always)]
    fn from(val: Seclock) -> u8 {
        Seclock::to_bits(val)
    }
}
