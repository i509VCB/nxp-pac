#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrCode {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "PID encoding error"]
    PID_ENCODING_ERROR = 0x01,
    #[doc = "PID unknown"]
    PID_UNKNOWN = 0x02,
    #[doc = "Packet unexpected"]
    PACKET_UNEXPECTED = 0x03,
    #[doc = "Token CRC error"]
    TOKEN_CRC_ERROR = 0x04,
    #[doc = "Data CRC error"]
    DATA_CRC_ERROR = 0x05,
    #[doc = "Time out"]
    TIMEOUT = 0x06,
    #[doc = "Babble"]
    BABBLE = 0x07,
    #[doc = "Truncated EOP"]
    TRUNCATED_EOP = 0x08,
    #[doc = "Sent/Received NAK"]
    SENT_RECEIVED_NAK = 0x09,
    #[doc = "Sent Stall"]
    SENT_STALL = 0x0a,
    #[doc = "Overrun"]
    OVERRUN = 0x0b,
    #[doc = "Sent empty packet"]
    SENT_EMPTY_PACKET = 0x0c,
    #[doc = "Bitstuff error"]
    BITSTUFF_ERROR = 0x0d,
    #[doc = "Sync error"]
    SYNC_ERROR = 0x0e,
    #[doc = "Wrong data toggle"]
    WRONG_DATA_TOGGLE = 0x0f,
}
impl ErrCode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrCode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrCode {
    #[inline(always)]
    fn from(val: u8) -> ErrCode {
        ErrCode::from_bits(val)
    }
}
impl From<ErrCode> for u8 {
    #[inline(always)]
    fn from(val: ErrCode) -> u8 {
        ErrCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceNeedclk {
    #[doc = "USB_NEEDCLK has normal function."]
    NORMAL = 0x0,
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    ALWAYS_ON = 0x01,
}
impl ForceNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ForceNeedclk {
        ForceNeedclk::from_bits(val)
    }
}
impl From<ForceNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ForceNeedclk) -> u8 {
        ForceNeedclk::to_bits(val)
    }
}
