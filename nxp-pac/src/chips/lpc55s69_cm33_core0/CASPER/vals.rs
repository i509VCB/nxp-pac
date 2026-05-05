#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ABBPAIR {
    #[doc = "Bank-pair 0 (1st)."]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)."]
    PAIR1 = 0x01,
}
impl ABBPAIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ABBPAIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ABBPAIR {
    #[inline(always)]
    fn from(val: u8) -> ABBPAIR {
        ABBPAIR::from_bits(val)
    }
}
impl From<ABBPAIR> for u8 {
    #[inline(always)]
    fn from(val: ABBPAIR) -> u8 {
        ABBPAIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSY {
    #[doc = "Not busy - is idle."]
    IDLE = 0x0,
    #[doc = "Is busy."]
    BUSY = 0x01,
}
impl BUSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSY {
    #[inline(always)]
    fn from(val: u8) -> BUSY {
        BUSY::from_bits(val)
    }
}
impl From<BUSY> for u8 {
    #[inline(always)]
    fn from(val: BUSY) -> u8 {
        BUSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CDBPAIR {
    #[doc = "Bank-pair 0 (1st)."]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)."]
    PAIR1 = 0x01,
}
impl CDBPAIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CDBPAIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CDBPAIR {
    #[inline(always)]
    fn from(val: u8) -> CDBPAIR {
        CDBPAIR::from_bits(val)
    }
}
impl From<CDBPAIR> for u8 {
    #[inline(always)]
    fn from(val: CDBPAIR) -> u8 {
        CDBPAIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CSKIP {
    #[doc = "No Skip."]
    NO_SKIP = 0x0,
    #[doc = "Skip if Carry is 1."]
    SKIP_IF_1 = 0x01,
    #[doc = "Skip if Carry is 0."]
    SKIP_IF_0 = 0x02,
    #[doc = "Set CTRLOFF to CDOFF and Skip."]
    SET_AND_SKIP = 0x03,
}
impl CSKIP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CSKIP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CSKIP {
    #[inline(always)]
    fn from(val: u8) -> CSKIP {
        CSKIP::from_bits(val)
    }
}
impl From<CSKIP> for u8 {
    #[inline(always)]
    fn from(val: CSKIP) -> u8 {
        CSKIP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTRLBPAIR {
    #[doc = "Bank-pair 0 (1st)."]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)."]
    PAIR1 = 0x01,
}
impl CTRLBPAIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTRLBPAIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTRLBPAIR {
    #[inline(always)]
    fn from(val: u8) -> CTRLBPAIR {
        CTRLBPAIR::from_bits(val)
    }
}
impl From<CTRLBPAIR> for u8 {
    #[inline(always)]
    fn from(val: CTRLBPAIR) -> u8 {
        CTRLBPAIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTENCLR_DONE {
    #[doc = "If written 0, ignored."]
    IGNORED = 0x0,
    #[doc = "If written 1, do not Interrupt when done."]
    NO_INTERRUPT = 0x01,
}
impl INTENCLR_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTENCLR_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTENCLR_DONE {
    #[inline(always)]
    fn from(val: u8) -> INTENCLR_DONE {
        INTENCLR_DONE::from_bits(val)
    }
}
impl From<INTENCLR_DONE> for u8 {
    #[inline(always)]
    fn from(val: INTENCLR_DONE) -> u8 {
        INTENCLR_DONE::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct KEY(u16);
impl KEY {
    #[doc = "If set during write, will allow lock or unlock."]
    pub const KWY_VALUE: Self = Self(0x073d);
}
impl KEY {
    pub const fn from_bits(val: u16) -> KEY {
        Self(val & 0x1fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for KEY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x073d => f.write_str("KWY_VALUE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEY {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x073d => defmt::write!(f, "KWY_VALUE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for KEY {
    #[inline(always)]
    fn from(val: u16) -> KEY {
        KEY::from_bits(val)
    }
}
impl From<KEY> for u16 {
    #[inline(always)]
    fn from(val: KEY) -> u16 {
        KEY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESBPAIR {
    #[doc = "Bank-pair 0 (1st)."]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)."]
    PAIR1 = 0x01,
}
impl RESBPAIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESBPAIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESBPAIR {
    #[inline(always)]
    fn from(val: u8) -> RESBPAIR {
        RESBPAIR::from_bits(val)
    }
}
impl From<RESBPAIR> for u8 {
    #[inline(always)]
    fn from(val: RESBPAIR) -> u8 {
        RESBPAIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STATUS_DONE {
    #[doc = "Busy or just cleared."]
    BUSY = 0x0,
    #[doc = "Completed last operation."]
    COMPLETED = 0x01,
}
impl STATUS_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STATUS_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STATUS_DONE {
    #[inline(always)]
    fn from(val: u8) -> STATUS_DONE {
        STATUS_DONE::from_bits(val)
    }
}
impl From<STATUS_DONE> for u8 {
    #[inline(always)]
    fn from(val: STATUS_DONE) -> u8 {
        STATUS_DONE::to_bits(val)
    }
}
