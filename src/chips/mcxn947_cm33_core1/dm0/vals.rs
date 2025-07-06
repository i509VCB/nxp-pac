#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbOrErr {
    #[doc = "No AHB Overrun Error"]
    NO_AHB_OVERRUN_ERR = 0x0,
    #[doc = "AHB Overrun Error. An AHB overrun occurred."]
    AHB_OVERRUN_ERR = 0x01,
}
impl AhbOrErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbOrErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbOrErr {
    #[inline(always)]
    fn from(val: u8) -> AhbOrErr {
        AhbOrErr::from_bits(val)
    }
}
impl From<AhbOrErr> for u8 {
    #[inline(always)]
    fn from(val: AhbOrErr) -> u8 {
        AhbOrErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgOrErr {
    #[doc = "No DBGMB Overrun error"]
    NO_OVERRUN_ERR = 0x0,
    #[doc = "DBGMB overrun error. A DBGMB overrun occurred."]
    OVERRUN_ERR = 0x01,
}
impl DbgOrErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgOrErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgOrErr {
    #[inline(always)]
    fn from(val: u8) -> DbgOrErr {
        DbgOrErr::from_bits(val)
    }
}
impl From<DbgOrErr> for u8 {
    #[inline(always)]
    fn from(val: DbgOrErr) -> u8 {
        DbgOrErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReqPending {
    #[doc = "No request pending"]
    NO_REQUEST_PENDING = 0x0,
    #[doc = "Request for resynchronization pending"]
    REQUEST_PENDING = 0x01,
}
impl ReqPending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqPending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqPending {
    #[inline(always)]
    fn from(val: u8) -> ReqPending {
        ReqPending::from_bits(val)
    }
}
impl From<ReqPending> for u8 {
    #[inline(always)]
    fn from(val: ReqPending) -> u8 {
        ReqPending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResynchReq {
    #[doc = "No request"]
    NO_REQUEST = 0x0,
    #[doc = "Request for resynchronization"]
    REQUEST = 0x01,
}
impl ResynchReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResynchReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResynchReq {
    #[inline(always)]
    fn from(val: u8) -> ResynchReq {
        ResynchReq::from_bits(val)
    }
}
impl From<ResynchReq> for u8 {
    #[inline(always)]
    fn from(val: ResynchReq) -> u8 {
        ResynchReq::to_bits(val)
    }
}
