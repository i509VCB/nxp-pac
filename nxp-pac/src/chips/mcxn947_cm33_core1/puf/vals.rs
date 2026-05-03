#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigType {
    _RESERVED_0 = 0x0,
    #[doc = "Indicates that PUF configuration is Safe."]
    Safe = 0x01,
    #[doc = "Indicates that PUF configuration is Plus."]
    Plus = 0x02,
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
    #[doc = "Indicates that Wrap is not included."]
    Enable = 0x0,
    #[doc = "Indicates that Wrap is included."]
    Disable = 0x01,
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
    pub const LoProgress: Self = Self(0x0);
    #[doc = "Indicates that the last operation was Enroll."]
    pub const LoEnroll: Self = Self(0x01);
    #[doc = "Indicates that the last operation was Start."]
    pub const LoStart: Self = Self(0x02);
    #[doc = "Indicates that the last operation was Reconstruct."]
    pub const LoReconstruct: Self = Self(0x03);
    #[doc = "Indicates that the last operation was Stop."]
    pub const LoStop: Self = Self(0x05);
    #[doc = "Indicates that the last operation was Get Key."]
    pub const LoGetKey: Self = Self(0x06);
    #[doc = "Indicates that the last operation was Unwrap."]
    pub const LoUnwrap: Self = Self(0x07);
    #[doc = "Indicates that the last operation was Wrap Generated Random."]
    pub const LoWrapGenRnd: Self = Self(0x08);
    #[doc = "Indicates that the last operation was Wrap."]
    pub const LoWrap: Self = Self(0x09);
    #[doc = "Indicates that the last operation was Generate Random."]
    pub const LoGenRnd: Self = Self(0x0f);
    #[doc = "Indicates that the last operation was Test Memory."]
    pub const LoTestMemory: Self = Self(0x1e);
    #[doc = "Indicates that the last operation was Test PUF."]
    pub const LoTestPuf: Self = Self(0x1f);
    #[doc = "Indicates that the last operation was Initialization."]
    pub const LoInitialization: Self = Self(0x20);
    #[doc = "Indicates that the last operation was Zeroize."]
    pub const LoZeroize: Self = Self(0x2f);
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
            0x0 => f.write_str("LoProgress"),
            0x01 => f.write_str("LoEnroll"),
            0x02 => f.write_str("LoStart"),
            0x03 => f.write_str("LoReconstruct"),
            0x05 => f.write_str("LoStop"),
            0x06 => f.write_str("LoGetKey"),
            0x07 => f.write_str("LoUnwrap"),
            0x08 => f.write_str("LoWrapGenRnd"),
            0x09 => f.write_str("LoWrap"),
            0x0f => f.write_str("LoGenRnd"),
            0x1e => f.write_str("LoTestMemory"),
            0x1f => f.write_str("LoTestPuf"),
            0x20 => f.write_str("LoInitialization"),
            0x2f => f.write_str("LoZeroize"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LastOperation {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LoProgress"),
            0x01 => defmt::write!(f, "LoEnroll"),
            0x02 => defmt::write!(f, "LoStart"),
            0x03 => defmt::write!(f, "LoReconstruct"),
            0x05 => defmt::write!(f, "LoStop"),
            0x06 => defmt::write!(f, "LoGetKey"),
            0x07 => defmt::write!(f, "LoUnwrap"),
            0x08 => defmt::write!(f, "LoWrapGenRnd"),
            0x09 => defmt::write!(f, "LoWrap"),
            0x0f => defmt::write!(f, "LoGenRnd"),
            0x1e => defmt::write!(f, "LoTestMemory"),
            0x1f => defmt::write!(f, "LoTestPuf"),
            0x20 => defmt::write!(f, "LoInitialization"),
            0x2f => defmt::write!(f, "LoZeroize"),
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
    #[doc = "OEM Develop."]
    pub const OemOpen: Self = Self(0x03);
    #[doc = "OEM Develop 2."]
    pub const OemSecureWorld: Self = Self(0x07);
    #[doc = "OEM In-field."]
    pub const OemClosed: Self = Self(0x0f);
    #[doc = "OEM Field return."]
    pub const OemFieldReturn: Self = Self(0x1f);
    #[doc = "NXP Field Return/Failure Analysis."]
    pub const FieldReturnNxp: Self = Self(0x3f);
    #[doc = "In-field Locked."]
    pub const OemLocked: Self = Self(0xcf);
    #[doc = "Bricked."]
    pub const OemShredded: Self = Self(0xff);
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
            0x03 => f.write_str("OemOpen"),
            0x07 => f.write_str("OemSecureWorld"),
            0x0f => f.write_str("OemClosed"),
            0x1f => f.write_str("OemFieldReturn"),
            0x3f => f.write_str("FieldReturnNxp"),
            0xcf => f.write_str("OemLocked"),
            0xff => f.write_str("OemShredded"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcState {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "OemOpen"),
            0x07 => defmt::write!(f, "OemSecureWorld"),
            0x0f => defmt::write!(f, "OemClosed"),
            0x1f => defmt::write!(f, "OemFieldReturn"),
            0x3f => defmt::write!(f, "FieldReturnNxp"),
            0xcf => defmt::write!(f, "OemLocked"),
            0xff => defmt::write!(f, "OemShredded"),
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
    pub const Ok: Self = Self(0x0);
    #[doc = "Indicates that the AC is not for the current product/version."]
    pub const ErrProduct: Self = Self(0xf0);
    #[doc = "Indicates that the AC in the second phase is not for the current product/version."]
    pub const ErrProductPh2: Self = Self(0xf1);
    #[doc = "Indicates that the AC is corrupted."]
    pub const ErrTransfer: Self = Self(0xf2);
    #[doc = "Indicates that the AC in the second phase is corrupted."]
    pub const ErrTransferPh2: Self = Self(0xf3);
    #[doc = "Indicates that the authentication of the provided AC failed."]
    pub const ErrAuth: Self = Self(0xf4);
    #[doc = "Indicates that the authentication of the provided AC failed in the second phase."]
    pub const ErrAuthPh2: Self = Self(0xf5);
    #[doc = "Indicates that the SRAM PUF quality verification fails."]
    pub const ErrPufQuality: Self = Self(0xf6);
    #[doc = "Indicates that the incorrect or unsupported context is provided."]
    pub const ErrContext: Self = Self(0xf7);
    #[doc = "Indicates that a data destination was set that is not allowed according to other settings and the current PUF state."]
    pub const ErrDestination: Self = Self(0xf8);
    #[doc = "Indicates that the PUF SRAM access has failed."]
    pub const Failure: Self = Self(0xff);
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
            0x0 => f.write_str("Ok"),
            0xf0 => f.write_str("ErrProduct"),
            0xf1 => f.write_str("ErrProductPh2"),
            0xf2 => f.write_str("ErrTransfer"),
            0xf3 => f.write_str("ErrTransferPh2"),
            0xf4 => f.write_str("ErrAuth"),
            0xf5 => f.write_str("ErrAuthPh2"),
            0xf6 => f.write_str("ErrPufQuality"),
            0xf7 => f.write_str("ErrContext"),
            0xf8 => f.write_str("ErrDestination"),
            0xff => f.write_str("Failure"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResultCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ok"),
            0xf0 => defmt::write!(f, "ErrProduct"),
            0xf1 => defmt::write!(f, "ErrProductPh2"),
            0xf2 => defmt::write!(f, "ErrTransfer"),
            0xf3 => defmt::write!(f, "ErrTransferPh2"),
            0xf4 => defmt::write!(f, "ErrAuth"),
            0xf5 => defmt::write!(f, "ErrAuthPh2"),
            0xf6 => defmt::write!(f, "ErrPufQuality"),
            0xf7 => defmt::write!(f, "ErrContext"),
            0xf8 => defmt::write!(f, "ErrDestination"),
            0xff => defmt::write!(f, "Failure"),
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIntClrStatusApbErr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware."]
    Enable = 0x01,
}
impl SramIntClrStatusApbErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIntClrStatusApbErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIntClrStatusApbErr {
    #[inline(always)]
    fn from(val: u8) -> SramIntClrStatusApbErr {
        SramIntClrStatusApbErr::from_bits(val)
    }
}
impl From<SramIntClrStatusApbErr> for u8 {
    #[inline(always)]
    fn from(val: SramIntClrStatusApbErr) -> u8 {
        SramIntClrStatusApbErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIntSetStatusApbErr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware."]
    Enable = 0x01,
}
impl SramIntSetStatusApbErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIntSetStatusApbErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIntSetStatusApbErr {
    #[inline(always)]
    fn from(val: u8) -> SramIntSetStatusApbErr {
        SramIntSetStatusApbErr::from_bits(val)
    }
}
impl From<SramIntSetStatusApbErr> for u8 {
    #[inline(always)]
    fn from(val: SramIntSetStatusApbErr) -> u8 {
        SramIntSetStatusApbErr::to_bits(val)
    }
}
