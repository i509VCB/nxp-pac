#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfhfnmins {
    #[doc = "BusFault, HardFault, and NMI are Secure."]
    SECURE = 0x0,
    #[doc = "BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    NON_SECURE = 0x01,
}
impl Bfhfnmins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfhfnmins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfhfnmins {
    #[inline(always)]
    fn from(val: u8) -> Bfhfnmins {
        Bfhfnmins::from_bits(val)
    }
}
impl From<Bfhfnmins> for u8 {
    #[inline(always)]
    fn from(val: Bfhfnmins) -> u8 {
        Bfhfnmins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cp11 {
    #[doc = "Access Denied. All accesses to the Floating-point Extension result in NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. Unprivileged access to the Floatingpoint Extension result in NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights to the Floatingpoint Extension."]
    FULL_ACCESS = 0x03,
}
impl Cp11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp11 {
    #[inline(always)]
    fn from(val: u8) -> Cp11 {
        Cp11::from_bits(val)
    }
}
impl From<Cp11> for u8 {
    #[inline(always)]
    fn from(val: Cp11) -> u8 {
        Cp11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp0 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp0 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp0 {
        CpacrCp0::from_bits(val)
    }
}
impl From<CpacrCp0> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp0) -> u8 {
        CpacrCp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp1 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp1 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp1 {
        CpacrCp1::from_bits(val)
    }
}
impl From<CpacrCp1> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp1) -> u8 {
        CpacrCp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp10 {
    #[doc = "Access Denied. All accesses to the Floating-point Extension result in NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. Unprivileged access to the Floatingpoint Extension result in NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights to the Floatingpoint Extension."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp10 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp10 {
        CpacrCp10::from_bits(val)
    }
}
impl From<CpacrCp10> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp10) -> u8 {
        CpacrCp10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp2 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp2 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp2 {
        CpacrCp2::from_bits(val)
    }
}
impl From<CpacrCp2> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp2) -> u8 {
        CpacrCp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp3 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp3 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp3 {
        CpacrCp3::from_bits(val)
    }
}
impl From<CpacrCp3> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp3) -> u8 {
        CpacrCp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp4 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp4 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp4 {
        CpacrCp4::from_bits(val)
    }
}
impl From<CpacrCp4> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp4) -> u8 {
        CpacrCp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp5 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp5 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp5 {
        CpacrCp5::from_bits(val)
    }
}
impl From<CpacrCp5> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp5) -> u8 {
        CpacrCp5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp6 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp6 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp6 {
        CpacrCp6::from_bits(val)
    }
}
impl From<CpacrCp6> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp6) -> u8 {
        CpacrCp6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp7 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp7 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp7 {
        CpacrCp7::from_bits(val)
    }
}
impl From<CpacrCp7> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp7) -> u8 {
        CpacrCp7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endianness {
    #[doc = "Little-endian."]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big-endian"]
    BIG_ENDIAN = 0x01,
}
impl Endianness {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endianness {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endianness {
    #[inline(always)]
    fn from(val: u8) -> Endianness {
        Endianness::from_bits(val)
    }
}
impl From<Endianness> for u8 {
    #[inline(always)]
    fn from(val: Endianness) -> u8 {
        Endianness::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pris {
    #[doc = "Priority ranges of Secure and Non-secure exceptions are identical"]
    SAME_PRIORITY = 0x0,
    #[doc = "Non-secure exceptions are de-prioritized"]
    SECURE_PRIORITIZED = 0x01,
}
impl Pris {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pris {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pris {
    #[inline(always)]
    fn from(val: u8) -> Pris {
        Pris::from_bits(val)
    }
}
impl From<Pris> for u8 {
    #[inline(always)]
    fn from(val: Pris) -> u8 {
        Pris::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sevonpend {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded."]
    EXCLUDE_DISABLED_INTERRUPTS = 0x0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    INCLUDE_DISABLED_INTERRUPTS = 0x01,
}
impl Sevonpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sevonpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sevonpend {
    #[inline(always)]
    fn from(val: u8) -> Sevonpend {
        Sevonpend::from_bits(val)
    }
}
impl From<Sevonpend> for u8 {
    #[inline(always)]
    fn from(val: Sevonpend) -> u8 {
        Sevonpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeep {
    #[doc = "Sleep."]
    SLEEP = 0x0,
    #[doc = "Deep sleep."]
    DEEP_SLEEP = 0x01,
}
impl Sleepdeep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeep {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeep {
        Sleepdeep::from_bits(val)
    }
}
impl From<Sleepdeep> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeep) -> u8 {
        Sleepdeep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeeps {
    #[doc = "The SLEEPDEEP bit is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SLEEPDEEP bit behaves as RAZ/WI when accessed from the Non-secure state."]
    SECURE_ONLY = 0x01,
}
impl Sleepdeeps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeeps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeeps {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeeps {
        Sleepdeeps::from_bits(val)
    }
}
impl From<Sleepdeeps> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeeps) -> u8 {
        Sleepdeeps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreq {
    #[doc = "Do not request a system reset."]
    NO_REQUEST = 0x0,
    #[doc = "Request a system reset."]
    REQUEST_RESET = 0x01,
}
impl Sysresetreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreq {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreq {
        Sysresetreq::from_bits(val)
    }
}
impl From<Sysresetreq> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreq) -> u8 {
        Sysresetreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreqs {
    #[doc = "SYSRESETREQ functionality is available to both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "SYSRESETREQ functionality is only available to Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sysresetreqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreqs {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreqs {
        Sysresetreqs::from_bits(val)
    }
}
impl From<Sysresetreqs> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreqs) -> u8 {
        Sysresetreqs::to_bits(val)
    }
}
