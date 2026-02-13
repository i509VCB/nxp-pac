#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigType {
    _RESERVED_0 = 0x0,
    #[doc = "Indicates that PUF configuration is Safe."]
    SAFE = 0x01,
    #[doc = "Indicates that PUF configuration is Plus."]
    PLUS = 0x02,
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
}
impl ConfigType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigType {
    #[inline(always)]
    fn from(val: u8) -> ConfigType {
        ConfigType::from_bits(val)
    }
}
impl From<ConfigType> for u8 {
    #[inline(always)]
    fn from(val: ConfigType) -> u8 {
        ConfigType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigWrap {
    #[doc = "Indicates that Wrap is not included"]
    ENABLE = 0x0,
    #[doc = "Indicates that Wrap is included"]
    DISABLE = 0x01,
}
impl ConfigWrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigWrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigWrap {
    #[inline(always)]
    fn from(val: u8) -> ConfigWrap {
        ConfigWrap::from_bits(val)
    }
}
impl From<ConfigWrap> for u8 {
    #[inline(always)]
    fn from(val: ConfigWrap) -> u8 {
        ConfigWrap::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LastOperation(u8);
impl LastOperation {
    #[doc = "Indicates that the operation is in progress."]
    pub const LO_PROGRESS: Self = Self(0x0);
    #[doc = "Indicates that the last operation was Enroll."]
    pub const LO_ENROLL: Self = Self(0x01);
    #[doc = "Indicates that the last operation was Start."]
    pub const LO_START: Self = Self(0x02);
    #[doc = "Indicates that the last operation was Reconstruct"]
    pub const LO_RECONSTRUCT: Self = Self(0x03);
    #[doc = "Indicates that the last operation was Stop."]
    pub const LO_STOP: Self = Self(0x05);
    #[doc = "Indicates that the last operation was Get Key."]
    pub const LO_GET_KEY: Self = Self(0x06);
    #[doc = "Indicates that the last operation was Unwrap."]
    pub const LO_UNWRAP: Self = Self(0x07);
    #[doc = "Indicates that the last operation was Wrap Generated Random."]
    pub const LO_WRAP_GEN_RND: Self = Self(0x08);
    #[doc = "Indicates that the last operation was Wrap."]
    pub const LO_WRAP: Self = Self(0x09);
    #[doc = "Indicates that the last operation was Generate Random."]
    pub const LO_GEN_RND: Self = Self(0x0f);
    #[doc = "Indicates that the last operation was Test Memory."]
    pub const LO_TEST_MEMORY: Self = Self(0x1e);
    #[doc = "Indicates that the last operation was Test PUF."]
    pub const LO_TEST_PUF: Self = Self(0x1f);
    #[doc = "Indicates that the last operation was Initialization."]
    pub const LO_INITIALIZATION: Self = Self(0x20);
    #[doc = "Indicates that the last operation was Zeroize."]
    pub const LO_ZEROIZE: Self = Self(0x2f);
}
impl LastOperation {
    pub const fn from_bits(val: u8) -> LastOperation {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LastOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LO_PROGRESS"),
            0x01 => f.write_str("LO_ENROLL"),
            0x02 => f.write_str("LO_START"),
            0x03 => f.write_str("LO_RECONSTRUCT"),
            0x05 => f.write_str("LO_STOP"),
            0x06 => f.write_str("LO_GET_KEY"),
            0x07 => f.write_str("LO_UNWRAP"),
            0x08 => f.write_str("LO_WRAP_GEN_RND"),
            0x09 => f.write_str("LO_WRAP"),
            0x0f => f.write_str("LO_GEN_RND"),
            0x1e => f.write_str("LO_TEST_MEMORY"),
            0x1f => f.write_str("LO_TEST_PUF"),
            0x20 => f.write_str("LO_INITIALIZATION"),
            0x2f => f.write_str("LO_ZEROIZE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LastOperation {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LO_PROGRESS"),
            0x01 => defmt::write!(f, "LO_ENROLL"),
            0x02 => defmt::write!(f, "LO_START"),
            0x03 => defmt::write!(f, "LO_RECONSTRUCT"),
            0x05 => defmt::write!(f, "LO_STOP"),
            0x06 => defmt::write!(f, "LO_GET_KEY"),
            0x07 => defmt::write!(f, "LO_UNWRAP"),
            0x08 => defmt::write!(f, "LO_WRAP_GEN_RND"),
            0x09 => defmt::write!(f, "LO_WRAP"),
            0x0f => defmt::write!(f, "LO_GEN_RND"),
            0x1e => defmt::write!(f, "LO_TEST_MEMORY"),
            0x1f => defmt::write!(f, "LO_TEST_PUF"),
            0x20 => defmt::write!(f, "LO_INITIALIZATION"),
            0x2f => defmt::write!(f, "LO_ZEROIZE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LastOperation {
    #[inline(always)]
    fn from(val: u8) -> LastOperation {
        LastOperation::from_bits(val)
    }
}
impl From<LastOperation> for u8 {
    #[inline(always)]
    fn from(val: LastOperation) -> u8 {
        LastOperation::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LcState(u8);
impl LcState {
    #[doc = "OEM Develop"]
    pub const OEM_OPEN: Self = Self(0x03);
    #[doc = "OEM Develop 2"]
    pub const OEM_SECURE_WORLD: Self = Self(0x07);
    #[doc = "OEM In-field"]
    pub const OEM_CLOSED: Self = Self(0x0f);
    #[doc = "OEM Field return"]
    pub const OEM_FIELD_RETURN: Self = Self(0x1f);
    #[doc = "NXP Field Return/Failure Analysis"]
    pub const FIELD_RETURN_NXP: Self = Self(0x3f);
    #[doc = "In-field Locked"]
    pub const OEM_LOCKED: Self = Self(0xcf);
    #[doc = "Bricked"]
    pub const OEM_SHREDDED: Self = Self(0xff);
}
impl LcState {
    pub const fn from_bits(val: u8) -> LcState {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x03 => f.write_str("OEM_OPEN"),
            0x07 => f.write_str("OEM_SECURE_WORLD"),
            0x0f => f.write_str("OEM_CLOSED"),
            0x1f => f.write_str("OEM_FIELD_RETURN"),
            0x3f => f.write_str("FIELD_RETURN_NXP"),
            0xcf => f.write_str("OEM_LOCKED"),
            0xff => f.write_str("OEM_SHREDDED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcState {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "OEM_OPEN"),
            0x07 => defmt::write!(f, "OEM_SECURE_WORLD"),
            0x0f => defmt::write!(f, "OEM_CLOSED"),
            0x1f => defmt::write!(f, "OEM_FIELD_RETURN"),
            0x3f => defmt::write!(f, "FIELD_RETURN_NXP"),
            0xcf => defmt::write!(f, "OEM_LOCKED"),
            0xff => defmt::write!(f, "OEM_SHREDDED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LcState {
    #[inline(always)]
    fn from(val: u8) -> LcState {
        LcState::from_bits(val)
    }
}
impl From<LcState> for u8 {
    #[inline(always)]
    fn from(val: LcState) -> u8 {
        LcState::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResultCode(u8);
impl ResultCode {
    #[doc = "Indicates that the last operation was successful or operation is in progress."]
    pub const OK: Self = Self(0x0);
    #[doc = "Indicates that the AC is not for the current product/version."]
    pub const ERR_PRODUCT: Self = Self(0xf0);
    #[doc = "Indicates that the AC in the second phase is not for the current product/version."]
    pub const ERR_PRODUCT_PH2: Self = Self(0xf1);
    #[doc = "Indicates that the AC is corrupted."]
    pub const ERR_TRANSFER: Self = Self(0xf2);
    #[doc = "Indicates that the AC in the second phase is corrupted."]
    pub const ERR_TRANSFER_PH2: Self = Self(0xf3);
    #[doc = "Indicates that the authentication of the provided AC failed."]
    pub const ERR_AUTH: Self = Self(0xf4);
    #[doc = "Indicates that the authentication of the provided AC failed in the second phase."]
    pub const ERR_AUTH_PH2: Self = Self(0xf5);
    #[doc = "Indicates that the SRAM PUF quality verification fails."]
    pub const ERR_PUF_QUALITY: Self = Self(0xf6);
    #[doc = "Indicates that the incorrect or unsupported context is provided."]
    pub const ERR_CONTEXT: Self = Self(0xf7);
    #[doc = "Indicates that a data destination was set that is not allowed according to other settings and the current PUF state."]
    pub const ERR_DESTINATION: Self = Self(0xf8);
    #[doc = "Indicates that the PUF SRAM access has failed."]
    pub const FAILURE: Self = Self(0xff);
}
impl ResultCode {
    pub const fn from_bits(val: u8) -> ResultCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ResultCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("OK"),
            0xf0 => f.write_str("ERR_PRODUCT"),
            0xf1 => f.write_str("ERR_PRODUCT_PH2"),
            0xf2 => f.write_str("ERR_TRANSFER"),
            0xf3 => f.write_str("ERR_TRANSFER_PH2"),
            0xf4 => f.write_str("ERR_AUTH"),
            0xf5 => f.write_str("ERR_AUTH_PH2"),
            0xf6 => f.write_str("ERR_PUF_QUALITY"),
            0xf7 => f.write_str("ERR_CONTEXT"),
            0xf8 => f.write_str("ERR_DESTINATION"),
            0xff => f.write_str("FAILURE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResultCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "OK"),
            0xf0 => defmt::write!(f, "ERR_PRODUCT"),
            0xf1 => defmt::write!(f, "ERR_PRODUCT_PH2"),
            0xf2 => defmt::write!(f, "ERR_TRANSFER"),
            0xf3 => defmt::write!(f, "ERR_TRANSFER_PH2"),
            0xf4 => defmt::write!(f, "ERR_AUTH"),
            0xf5 => defmt::write!(f, "ERR_AUTH_PH2"),
            0xf6 => defmt::write!(f, "ERR_PUF_QUALITY"),
            0xf7 => defmt::write!(f, "ERR_CONTEXT"),
            0xf8 => defmt::write!(f, "ERR_DESTINATION"),
            0xff => defmt::write!(f, "FAILURE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ResultCode {
    #[inline(always)]
    fn from(val: u8) -> ResultCode {
        ResultCode::from_bits(val)
    }
}
impl From<ResultCode> for u8 {
    #[inline(always)]
    fn from(val: ResultCode) -> u8 {
        ResultCode::to_bits(val)
    }
}
