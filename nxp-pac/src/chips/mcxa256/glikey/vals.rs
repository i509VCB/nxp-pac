#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorStatus {
    #[doc = "No error"]
    STAT0 = 0x0,
    #[doc = "FSM error has occurred"]
    STAT1 = 0x01,
    #[doc = "Write index out of the bound (OOB) error"]
    STAT2 = 0x02,
    #[doc = "Write index OOB and FSM error"]
    STAT3 = 0x03,
    #[doc = "Read index OOB error"]
    STAT4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Write index and read index OOB error"]
    STAT5 = 0x06,
    #[doc = "Read index OOB, write index OOB, and FSM error"]
    STAT6 = 0x07,
}
impl ErrorStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrorStatus {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrorStatus {
    #[inline(always)]
    fn from(val: u8) -> ErrorStatus {
        ErrorStatus::from_bits(val)
    }
}
impl From<ErrorStatus> for u8 {
    #[inline(always)]
    fn from(val: ErrorStatus) -> u8 {
        ErrorStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockStatus {
    #[doc = "Current read index is not locked"]
    LOCK0 = 0x0,
    #[doc = "Current read index is locked"]
    LOCK1 = 0x01,
}
impl LockStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockStatus {
    #[inline(always)]
    fn from(val: u8) -> LockStatus {
        LockStatus::from_bits(val)
    }
}
impl From<LockStatus> for u8 {
    #[inline(always)]
    fn from(val: LockStatus) -> u8 {
        LockStatus::to_bits(val)
    }
}
