#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CppreSat8 {
    #[doc = "8 bits"]
    SAT_8_BITS = 0x0,
    #[doc = "16 bits"]
    SAT_16_BITS = 0x01,
}
impl CppreSat8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CppreSat8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CppreSat8 {
    #[inline(always)]
    fn from(val: u8) -> CppreSat8 {
        CppreSat8::from_bits(val)
    }
}
impl From<CppreSat8> for u8 {
    #[inline(always)]
    fn from(val: CppreSat8) -> u8 {
        CppreSat8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodeMachine {
    #[doc = "Coprocessor"]
    COPROCESSOR = 0x0,
    #[doc = "Matrix engine"]
    MATRIX = 0x01,
    #[doc = "Transform engine"]
    TRANSFORM = 0x02,
    #[doc = "Filter engine"]
    FILTER = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CORDIC engine"]
    CORDIC = 0x05,
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
impl DecodeMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DecodeMachine {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DecodeMachine {
    #[inline(always)]
    fn from(val: u8) -> DecodeMachine {
        DecodeMachine::from_bits(val)
    }
}
impl From<DecodeMachine> for u8 {
    #[inline(always)]
    fn from(val: DecodeMachine) -> u8 {
        DecodeMachine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatext {
    #[inline(always)]
    fn from(val: u8) -> InaFormatext {
        InaFormatext::from_bits(val)
    }
}
impl From<InaFormatext> for u8 {
    #[inline(always)]
    fn from(val: InaFormatext) -> u8 {
        InaFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatint {
    #[inline(always)]
    fn from(val: u8) -> InaFormatint {
        InaFormatint::from_bits(val)
    }
}
impl From<InaFormatint> for u8 {
    #[inline(always)]
    fn from(val: InaFormatint) -> u8 {
        InaFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatext {
    #[inline(always)]
    fn from(val: u8) -> InbFormatext {
        InbFormatext::from_bits(val)
    }
}
impl From<InbFormatext> for u8 {
    #[inline(always)]
    fn from(val: InbFormatext) -> u8 {
        InbFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatint {
    #[inline(always)]
    fn from(val: u8) -> InbFormatint {
        InbFormatint::from_bits(val)
    }
}
impl From<InbFormatint> for u8 {
    #[inline(always)]
    fn from(val: InbFormatint) -> u8 {
        InbFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatext {
    #[inline(always)]
    fn from(val: u8) -> OutFormatext {
        OutFormatext::from_bits(val)
    }
}
impl From<OutFormatext> for u8 {
    #[inline(always)]
    fn from(val: OutFormatext) -> u8 {
        OutFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatint {
    #[inline(always)]
    fn from(val: u8) -> OutFormatint {
        OutFormatint::from_bits(val)
    }
}
impl From<OutFormatint> for u8 {
    #[inline(always)]
    fn from(val: OutFormatint) -> u8 {
        OutFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatext {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatext {
        TmpFormatext::from_bits(val)
    }
}
impl From<TmpFormatext> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatext) -> u8 {
        TmpFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatint {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatint {
        TmpFormatint::from_bits(val)
    }
}
impl From<TmpFormatint> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatint) -> u8 {
        TmpFormatint::to_bits(val)
    }
}
