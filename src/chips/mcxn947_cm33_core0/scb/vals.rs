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
pub enum Busfaultact {
    #[doc = "BusFault exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "BusFault exception is active."]
    ACTIVE = 0x01,
}
impl Busfaultact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busfaultact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busfaultact {
    #[inline(always)]
    fn from(val: u8) -> Busfaultact {
        Busfaultact::from_bits(val)
    }
}
impl From<Busfaultact> for u8 {
    #[inline(always)]
    fn from(val: Busfaultact) -> u8 {
        Busfaultact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busfaultena {
    #[doc = "BusFault is disabled."]
    DISABLED = 0x0,
    #[doc = "BusFault is enabled."]
    ENABLED = 0x01,
}
impl Busfaultena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busfaultena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busfaultena {
    #[inline(always)]
    fn from(val: u8) -> Busfaultena {
        Busfaultena::from_bits(val)
    }
}
impl From<Busfaultena> for u8 {
    #[inline(always)]
    fn from(val: Busfaultena) -> u8 {
        Busfaultena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busfaultpended {
    #[doc = "BusFault exception is pending."]
    NOT_PENDING = 0x0,
    #[doc = "BusFault exception is not pending."]
    PENDING = 0x01,
}
impl Busfaultpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busfaultpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busfaultpended {
    #[inline(always)]
    fn from(val: u8) -> Busfaultpended {
        Busfaultpended::from_bits(val)
    }
}
impl From<Busfaultpended> for u8 {
    #[inline(always)]
    fn from(val: Busfaultpended) -> u8 {
        Busfaultpended::to_bits(val)
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
pub enum Hardfaultact {
    #[doc = "HardFault exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "HardFault exception is active."]
    ACTIVE = 0x01,
}
impl Hardfaultact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hardfaultact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hardfaultact {
    #[inline(always)]
    fn from(val: u8) -> Hardfaultact {
        Hardfaultact::from_bits(val)
    }
}
impl From<Hardfaultact> for u8 {
    #[inline(always)]
    fn from(val: Hardfaultact) -> u8 {
        Hardfaultact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hardfaultpended {
    #[doc = "HardFault exception modification is disabled."]
    DISABLED = 0x0,
    #[doc = "HardFault exception modification is enabled."]
    ENABLED = 0x01,
}
impl Hardfaultpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hardfaultpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hardfaultpended {
    #[inline(always)]
    fn from(val: u8) -> Hardfaultpended {
        Hardfaultpended::from_bits(val)
    }
}
impl From<Hardfaultpended> for u8 {
    #[inline(always)]
    fn from(val: Hardfaultpended) -> u8 {
        Hardfaultpended::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memfaultact {
    #[doc = "MemManage exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "MemManage exception is active."]
    ACTIVE = 0x01,
}
impl Memfaultact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memfaultact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memfaultact {
    #[inline(always)]
    fn from(val: u8) -> Memfaultact {
        Memfaultact::from_bits(val)
    }
}
impl From<Memfaultact> for u8 {
    #[inline(always)]
    fn from(val: Memfaultact) -> u8 {
        Memfaultact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memfaultena {
    #[doc = "MemManage exception is disabled."]
    DISABLED = 0x0,
    #[doc = "MemManage exception is enabled."]
    ENABLED = 0x01,
}
impl Memfaultena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memfaultena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memfaultena {
    #[inline(always)]
    fn from(val: u8) -> Memfaultena {
        Memfaultena::from_bits(val)
    }
}
impl From<Memfaultena> for u8 {
    #[inline(always)]
    fn from(val: Memfaultena) -> u8 {
        Memfaultena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memfaultpended {
    #[doc = "MemManage exception is not pending."]
    NOT_PENDING = 0x0,
    #[doc = "MemManage exception is pending."]
    PENDING = 0x01,
}
impl Memfaultpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memfaultpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memfaultpended {
    #[inline(always)]
    fn from(val: u8) -> Memfaultpended {
        Memfaultpended::from_bits(val)
    }
}
impl From<Memfaultpended> for u8 {
    #[inline(always)]
    fn from(val: Memfaultpended) -> u8 {
        Memfaultpended::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Monitoract {
    #[doc = "Debug monitor exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "Debug monitor exception is active."]
    ACTIVE = 0x01,
}
impl Monitoract {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Monitoract {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Monitoract {
    #[inline(always)]
    fn from(val: u8) -> Monitoract {
        Monitoract::from_bits(val)
    }
}
impl From<Monitoract> for u8 {
    #[inline(always)]
    fn from(val: Monitoract) -> u8 {
        Monitoract::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmiact {
    #[doc = "NMI exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "NMI exception is active."]
    ACTIVE = 0x01,
}
impl Nmiact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmiact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmiact {
    #[inline(always)]
    fn from(val: u8) -> Nmiact {
        Nmiact::from_bits(val)
    }
}
impl From<Nmiact> for u8 {
    #[inline(always)]
    fn from(val: Nmiact) -> u8 {
        Nmiact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp0 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp0 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp0 {
        NsacrCp0::from_bits(val)
    }
}
impl From<NsacrCp0> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp0) -> u8 {
        NsacrCp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp1 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp1 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp1 {
        NsacrCp1::from_bits(val)
    }
}
impl From<NsacrCp1> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp1) -> u8 {
        NsacrCp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp10 {
    #[doc = "Non-secure accesses to the Floating-point Extension generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to the Floatingpoint Extension permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp10 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp10 {
        NsacrCp10::from_bits(val)
    }
}
impl From<NsacrCp10> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp10) -> u8 {
        NsacrCp10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp2 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp2 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp2 {
        NsacrCp2::from_bits(val)
    }
}
impl From<NsacrCp2> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp2) -> u8 {
        NsacrCp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp3 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp3 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp3 {
        NsacrCp3::from_bits(val)
    }
}
impl From<NsacrCp3> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp3) -> u8 {
        NsacrCp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp4 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp4 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp4 {
        NsacrCp4::from_bits(val)
    }
}
impl From<NsacrCp4> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp4) -> u8 {
        NsacrCp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp5 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp5 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp5 {
        NsacrCp5::from_bits(val)
    }
}
impl From<NsacrCp5> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp5) -> u8 {
        NsacrCp5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp6 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp6 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp6 {
        NsacrCp6::from_bits(val)
    }
}
impl From<NsacrCp6> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp6) -> u8 {
        NsacrCp6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsacrCp7 {
    #[doc = "Non-secure accesses to this coprocessor generate a NOCP UsageFault."]
    NOT_PERMITTED = 0x0,
    #[doc = "Non-secure access to this coprocessor permitted."]
    PERMITTED = 0x01,
}
impl NsacrCp7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsacrCp7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsacrCp7 {
    #[inline(always)]
    fn from(val: u8) -> NsacrCp7 {
        NsacrCp7::from_bits(val)
    }
}
impl From<NsacrCp7> for u8 {
    #[inline(always)]
    fn from(val: NsacrCp7) -> u8 {
        NsacrCp7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pendsvact {
    #[doc = "PendSV exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "PendSV exception is active."]
    ACTIVE = 0x01,
}
impl Pendsvact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pendsvact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pendsvact {
    #[inline(always)]
    fn from(val: u8) -> Pendsvact {
        Pendsvact::from_bits(val)
    }
}
impl From<Pendsvact> for u8 {
    #[inline(always)]
    fn from(val: Pendsvact) -> u8 {
        Pendsvact::to_bits(val)
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
pub enum Securefaultact {
    #[doc = "SecureFault exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "SecureFault exception is active."]
    ACTIVE = 0x01,
}
impl Securefaultact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Securefaultact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Securefaultact {
    #[inline(always)]
    fn from(val: u8) -> Securefaultact {
        Securefaultact::from_bits(val)
    }
}
impl From<Securefaultact> for u8 {
    #[inline(always)]
    fn from(val: Securefaultact) -> u8 {
        Securefaultact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Securefaultena {
    #[doc = "SecureFault exception is disabled."]
    DISABLED = 0x0,
    #[doc = "SecureFault exception is enabled."]
    ENABLED = 0x01,
}
impl Securefaultena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Securefaultena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Securefaultena {
    #[inline(always)]
    fn from(val: u8) -> Securefaultena {
        Securefaultena::from_bits(val)
    }
}
impl From<Securefaultena> for u8 {
    #[inline(always)]
    fn from(val: Securefaultena) -> u8 {
        Securefaultena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Securefaultpended {
    #[doc = "SecureFault exception modification is disabled."]
    DISABLED = 0x0,
    #[doc = "SecureFault exception modification is enabled."]
    ENABLED = 0x01,
}
impl Securefaultpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Securefaultpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Securefaultpended {
    #[inline(always)]
    fn from(val: u8) -> Securefaultpended {
        Securefaultpended::from_bits(val)
    }
}
impl From<Securefaultpended> for u8 {
    #[inline(always)]
    fn from(val: Securefaultpended) -> u8 {
        Securefaultpended::to_bits(val)
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
pub enum Sleeponexit {
    #[doc = "Do not sleep when returning to Thread mode."]
    NOT_SLEEP = 0x0,
    #[doc = "Enter sleep, or deep sleep, on return from an ISR"]
    SLEEP = 0x01,
}
impl Sleeponexit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleeponexit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleeponexit {
    #[inline(always)]
    fn from(val: u8) -> Sleeponexit {
        Sleeponexit::from_bits(val)
    }
}
impl From<Sleeponexit> for u8 {
    #[inline(always)]
    fn from(val: Sleeponexit) -> u8 {
        Sleeponexit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Svcallact {
    #[doc = "SVCall exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "SVCall exception is active."]
    ACTIVE = 0x01,
}
impl Svcallact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Svcallact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Svcallact {
    #[inline(always)]
    fn from(val: u8) -> Svcallact {
        Svcallact::from_bits(val)
    }
}
impl From<Svcallact> for u8 {
    #[inline(always)]
    fn from(val: Svcallact) -> u8 {
        Svcallact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Svcallpended {
    #[doc = "SVCall exception is not pending."]
    NOT_PENDING = 0x0,
    #[doc = "SVCall exception is pending."]
    PENDING = 0x01,
}
impl Svcallpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Svcallpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Svcallpended {
    #[inline(always)]
    fn from(val: u8) -> Svcallpended {
        Svcallpended::from_bits(val)
    }
}
impl From<Svcallpended> for u8 {
    #[inline(always)]
    fn from(val: Svcallpended) -> u8 {
        Svcallpended::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickact {
    #[doc = "SysTick exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "SysTick exception is active."]
    ACTIVE = 0x01,
}
impl Systickact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickact {
    #[inline(always)]
    fn from(val: u8) -> Systickact {
        Systickact::from_bits(val)
    }
}
impl From<Systickact> for u8 {
    #[inline(always)]
    fn from(val: Systickact) -> u8 {
        Systickact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgfaultact {
    #[doc = "UsageFault exception is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "UsageFault exception is active."]
    ACTIVE = 0x01,
}
impl Usgfaultact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgfaultact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgfaultact {
    #[inline(always)]
    fn from(val: u8) -> Usgfaultact {
        Usgfaultact::from_bits(val)
    }
}
impl From<Usgfaultact> for u8 {
    #[inline(always)]
    fn from(val: Usgfaultact) -> u8 {
        Usgfaultact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgfaultena {
    #[doc = "UsageFault is disabled."]
    DISABLED = 0x0,
    #[doc = "UsageFault is enabled."]
    ENABLED = 0x01,
}
impl Usgfaultena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgfaultena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgfaultena {
    #[inline(always)]
    fn from(val: u8) -> Usgfaultena {
        Usgfaultena::from_bits(val)
    }
}
impl From<Usgfaultena> for u8 {
    #[inline(always)]
    fn from(val: Usgfaultena) -> u8 {
        Usgfaultena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usgfaultpended {
    #[doc = "UsageFault exception is not pending."]
    NOT_PENDING = 0x0,
    #[doc = "UsageFault exception is pending."]
    PENDING = 0x01,
}
impl Usgfaultpended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usgfaultpended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usgfaultpended {
    #[inline(always)]
    fn from(val: u8) -> Usgfaultpended {
        Usgfaultpended::from_bits(val)
    }
}
impl From<Usgfaultpended> for u8 {
    #[inline(always)]
    fn from(val: Usgfaultpended) -> u8 {
        Usgfaultpended::to_bits(val)
    }
}
