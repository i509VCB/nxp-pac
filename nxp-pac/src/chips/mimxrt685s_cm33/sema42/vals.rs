#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    GTFSM_0 = 0x0,
    #[doc = "The gate has been locked by processor 0."]
    GTFSM_1 = 0x01,
    #[doc = "The gate has been locked by processor 1."]
    GTFSM_2 = 0x02,
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
impl Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gtfsm {
        Gtfsm::from_bits(val)
    }
}
impl From<Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gtfsm) -> u8 {
        Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstgsm {
    #[doc = "Idle, waiting for the first data pattern write."]
    RSTGSM_0 = 0x0,
    #[doc = "Waiting for the second data pattern write."]
    RSTGSM_1 = 0x01,
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software cannot observe this state."]
    RSTGSM_2 = 0x02,
    #[doc = "This state encoding is never used and therefore reserved."]
    RSTGSM_3 = 0x03,
}
impl Rstgsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstgsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstgsm {
    #[inline(always)]
    fn from(val: u8) -> Rstgsm {
        Rstgsm::from_bits(val)
    }
}
impl From<Rstgsm> for u8 {
    #[inline(always)]
    fn from(val: Rstgsm) -> u8 {
        Rstgsm::to_bits(val)
    }
}
