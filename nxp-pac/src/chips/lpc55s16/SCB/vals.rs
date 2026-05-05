#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BFHFNMINS {
    #[doc = "BusFault, HardFault, and NMI are Secure."]
    SECURE = 0x0,
    #[doc = "BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    NON_SECURE = 0x01,
}
impl BFHFNMINS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BFHFNMINS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BFHFNMINS {
    #[inline(always)]
    fn from(val: u8) -> BFHFNMINS {
        BFHFNMINS::from_bits(val)
    }
}
impl From<BFHFNMINS> for u8 {
    #[inline(always)]
    fn from(val: BFHFNMINS) -> u8 {
        BFHFNMINS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDIANNESS {
    #[doc = "Little-endian."]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big-endian."]
    BIG_ENDIAN = 0x01,
}
impl ENDIANNESS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDIANNESS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDIANNESS {
    #[inline(always)]
    fn from(val: u8) -> ENDIANNESS {
        ENDIANNESS::from_bits(val)
    }
}
impl From<ENDIANNESS> for u8 {
    #[inline(always)]
    fn from(val: ENDIANNESS) -> u8 {
        ENDIANNESS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PRIS {
    #[doc = "Priority ranges of Secure and Non-secure exceptions are identical."]
    SAME_PRIORITY = 0x0,
    #[doc = "Non-secure exceptions are de-prioritized."]
    SECURE_PRIORITIZED = 0x01,
}
impl PRIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PRIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PRIS {
    #[inline(always)]
    fn from(val: u8) -> PRIS {
        PRIS::from_bits(val)
    }
}
impl From<PRIS> for u8 {
    #[inline(always)]
    fn from(val: PRIS) -> u8 {
        PRIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEVONPEND {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded."]
    EXCLUDE_DISABLED_INTERRUPTS = 0x0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    INCLUDE_DISABLED_INTERRUPTS = 0x01,
}
impl SEVONPEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEVONPEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEVONPEND {
    #[inline(always)]
    fn from(val: u8) -> SEVONPEND {
        SEVONPEND::from_bits(val)
    }
}
impl From<SEVONPEND> for u8 {
    #[inline(always)]
    fn from(val: SEVONPEND) -> u8 {
        SEVONPEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLEEPDEEP {
    #[doc = "Sleep."]
    SLEEP = 0x0,
    #[doc = "Deep sleep."]
    DEEP_SLEEP = 0x01,
}
impl SLEEPDEEP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLEEPDEEP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLEEPDEEP {
    #[inline(always)]
    fn from(val: u8) -> SLEEPDEEP {
        SLEEPDEEP::from_bits(val)
    }
}
impl From<SLEEPDEEP> for u8 {
    #[inline(always)]
    fn from(val: SLEEPDEEP) -> u8 {
        SLEEPDEEP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLEEPDEEPS {
    #[doc = "The SLEEPDEEP bit is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SLEEPDEEP bit behaves as RAZ/WI when accessed from the Non-secure state."]
    SECURE_ONLY = 0x01,
}
impl SLEEPDEEPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLEEPDEEPS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLEEPDEEPS {
    #[inline(always)]
    fn from(val: u8) -> SLEEPDEEPS {
        SLEEPDEEPS::from_bits(val)
    }
}
impl From<SLEEPDEEPS> for u8 {
    #[inline(always)]
    fn from(val: SLEEPDEEPS) -> u8 {
        SLEEPDEEPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSRESETREQ {
    #[doc = "Do not request a system reset."]
    NO_REQUEST = 0x0,
    #[doc = "Request a system reset."]
    REQUEST_RESET = 0x01,
}
impl SYSRESETREQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSRESETREQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSRESETREQ {
    #[inline(always)]
    fn from(val: u8) -> SYSRESETREQ {
        SYSRESETREQ::from_bits(val)
    }
}
impl From<SYSRESETREQ> for u8 {
    #[inline(always)]
    fn from(val: SYSRESETREQ) -> u8 {
        SYSRESETREQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSRESETREQS {
    #[doc = "SYSRESETREQ functionality is available to both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "SYSRESETREQ functionality is only available to Secure state."]
    SECURE_ONLY = 0x01,
}
impl SYSRESETREQS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSRESETREQS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSRESETREQS {
    #[inline(always)]
    fn from(val: u8) -> SYSRESETREQS {
        SYSRESETREQS::from_bits(val)
    }
}
impl From<SYSRESETREQS> for u8 {
    #[inline(always)]
    fn from(val: SYSRESETREQS) -> u8 {
        SYSRESETREQS::to_bits(val)
    }
}
