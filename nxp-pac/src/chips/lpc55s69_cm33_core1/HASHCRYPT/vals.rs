#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AESDECRYPT {
    #[doc = "Encrypt."]
    ENCRYPT = 0x0,
    #[doc = "Decrypt."]
    DECRYPT = 0x01,
}
impl AESDECRYPT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AESDECRYPT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AESDECRYPT {
    #[inline(always)]
    fn from(val: u8) -> AESDECRYPT {
        AESDECRYPT::from_bits(val)
    }
}
impl From<AESDECRYPT> for u8 {
    #[inline(always)]
    fn from(val: AESDECRYPT) -> u8 {
        AESDECRYPT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AESKEYSZ {
    #[doc = "128 bit key."]
    BITS_128 = 0x0,
    #[doc = "192 bit key."]
    BITS_192 = 0x01,
    #[doc = "256 bit key."]
    BITS_256 = 0x02,
    _RESERVED_3 = 0x03,
}
impl AESKEYSZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AESKEYSZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AESKEYSZ {
    #[inline(always)]
    fn from(val: u8) -> AESKEYSZ {
        AESKEYSZ::from_bits(val)
    }
}
impl From<AESKEYSZ> for u8 {
    #[inline(always)]
    fn from(val: AESKEYSZ) -> u8 {
        AESKEYSZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AESMODE {
    #[doc = "ECB - used as is."]
    ECB = 0x0,
    #[doc = "CBC mode (see details on IV/nonce)."]
    CBC = 0x01,
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR = 0x02,
    _RESERVED_3 = 0x03,
}
impl AESMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AESMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AESMODE {
    #[inline(always)]
    fn from(val: u8) -> AESMODE {
        AESMODE::from_bits(val)
    }
}
impl From<AESMODE> for u8 {
    #[inline(always)]
    fn from(val: AESMODE) -> u8 {
        AESMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AESSECRET {
    #[doc = "User key provided in normal way."]
    NORMAL_WAY = 0x0,
    #[doc = "Secret key provided in hidden way by HW."]
    HIDDEN_WAY = 0x01,
}
impl AESSECRET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AESSECRET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AESSECRET {
    #[inline(always)]
    fn from(val: u8) -> AESSECRET {
        AESSECRET::from_bits(val)
    }
}
impl From<AESSECRET> for u8 {
    #[inline(always)]
    fn from(val: AESSECRET) -> u8 {
        AESSECRET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA_I {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0x0,
    #[doc = "DMA will push in the data."]
    PUSH = 0x01,
}
impl DMA_I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA_I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA_I {
    #[inline(always)]
    fn from(val: u8) -> DMA_I {
        DMA_I::from_bits(val)
    }
}
impl From<DMA_I> for u8 {
    #[inline(always)]
    fn from(val: DMA_I) -> u8 {
        DMA_I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ICBSTRM {
    #[doc = "8 blocks."]
    BLOCKS_8 = 0x0,
    #[doc = "16 blocks."]
    BLOCKS_16 = 0x01,
    #[doc = "32 blocks."]
    BLOCKS_32 = 0x02,
    #[doc = "64 blocks."]
    BLOCKS_64 = 0x03,
}
impl ICBSTRM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ICBSTRM {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ICBSTRM {
    #[inline(always)]
    fn from(val: u8) -> ICBSTRM {
        ICBSTRM::from_bits(val)
    }
}
impl From<ICBSTRM> for u8 {
    #[inline(always)]
    fn from(val: ICBSTRM) -> u8 {
        ICBSTRM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ICBSZ {
    #[doc = "32 bits of the IV/ctr are used (from 127:96)."]
    BITS_32 = 0x0,
    #[doc = "64 bits of the IV/ctr are used (from 127:64)."]
    BITS_64 = 0x01,
    #[doc = "96 bits of the IV/ctr are used (from 127:32)."]
    BITS_96 = 0x02,
    #[doc = "All 128 bits of the IV/ctr are used."]
    BIT_128 = 0x03,
}
impl ICBSZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ICBSZ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ICBSZ {
    #[inline(always)]
    fn from(val: u8) -> ICBSZ {
        ICBSZ::from_bits(val)
    }
}
impl From<ICBSZ> for u8 {
    #[inline(always)]
    fn from(val: ICBSZ) -> u8 {
        ICBSZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED = 0x0,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED = 0x01,
}
impl MASTER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER {
    #[inline(always)]
    fn from(val: u8) -> MASTER {
        MASTER::from_bits(val)
    }
}
impl From<MASTER> for u8 {
    #[inline(always)]
    fn from(val: MASTER) -> u8 {
        MASTER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "SHA1 is enabled."]
    SHA1 = 0x01,
    #[doc = "SHA2-256 is enabled."]
    SHA2_256 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "AES if available (see also CRYPTCFG register for more controls)."]
    AES = 0x04,
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)."]
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
pub enum SECLOCK {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK = 0x0,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SECLOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SECLOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SECLOCK {
    #[inline(always)]
    fn from(val: u8) -> SECLOCK {
        SECLOCK::from_bits(val)
    }
}
impl From<SECLOCK> for u8 {
    #[inline(always)]
    fn from(val: SECLOCK) -> u8 {
        SECLOCK::to_bits(val)
    }
}
