#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "The MPU is disabled."]
    MPU_DISABLE = 0x0,
    #[doc = "The MPU is enabled."]
    MPU_ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hfnmiena {
    #[doc = "MPU is disabled during HardFault and NMI handlers."]
    DISABLE = 0x0,
    #[doc = "The MPU is enabled during HardFault and NMI handlers."]
    ENABLE = 0x01,
}
impl Hfnmiena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfnmiena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfnmiena {
    #[inline(always)]
    fn from(val: u8) -> Hfnmiena {
        Hfnmiena::from_bits(val)
    }
}
impl From<Hfnmiena> for u8 {
    #[inline(always)]
    fn from(val: Hfnmiena) -> u8 {
        Hfnmiena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Privdefena {
    #[doc = "Disables the default memory map. Any instruction or data access that does not access a defined region faults."]
    ENABLE = 0x0,
    #[doc = "Enables the default memory map as a background region for privileged accesses."]
    DISABLE = 0x01,
}
impl Privdefena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Privdefena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Privdefena {
    #[inline(always)]
    fn from(val: u8) -> Privdefena {
        Privdefena::from_bits(val)
    }
}
impl From<Privdefena> for u8 {
    #[inline(always)]
    fn from(val: Privdefena) -> u8 {
        Privdefena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA1Ap {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RbarA1Ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA1Ap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA1Ap {
    #[inline(always)]
    fn from(val: u8) -> RbarA1Ap {
        RbarA1Ap::from_bits(val)
    }
}
impl From<RbarA1Ap> for u8 {
    #[inline(always)]
    fn from(val: RbarA1Ap) -> u8 {
        RbarA1Ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA1Sh {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable"]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RbarA1Sh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA1Sh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA1Sh {
    #[inline(always)]
    fn from(val: u8) -> RbarA1Sh {
        RbarA1Sh::from_bits(val)
    }
}
impl From<RbarA1Sh> for u8 {
    #[inline(always)]
    fn from(val: RbarA1Sh) -> u8 {
        RbarA1Sh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA1Xn {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RbarA1Xn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA1Xn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA1Xn {
    #[inline(always)]
    fn from(val: u8) -> RbarA1Xn {
        RbarA1Xn::from_bits(val)
    }
}
impl From<RbarA1Xn> for u8 {
    #[inline(always)]
    fn from(val: RbarA1Xn) -> u8 {
        RbarA1Xn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA2Ap {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RbarA2Ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA2Ap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA2Ap {
    #[inline(always)]
    fn from(val: u8) -> RbarA2Ap {
        RbarA2Ap::from_bits(val)
    }
}
impl From<RbarA2Ap> for u8 {
    #[inline(always)]
    fn from(val: RbarA2Ap) -> u8 {
        RbarA2Ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA2Sh {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable"]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RbarA2Sh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA2Sh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA2Sh {
    #[inline(always)]
    fn from(val: u8) -> RbarA2Sh {
        RbarA2Sh::from_bits(val)
    }
}
impl From<RbarA2Sh> for u8 {
    #[inline(always)]
    fn from(val: RbarA2Sh) -> u8 {
        RbarA2Sh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA2Xn {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RbarA2Xn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA2Xn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA2Xn {
    #[inline(always)]
    fn from(val: u8) -> RbarA2Xn {
        RbarA2Xn::from_bits(val)
    }
}
impl From<RbarA2Xn> for u8 {
    #[inline(always)]
    fn from(val: RbarA2Xn) -> u8 {
        RbarA2Xn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA3Ap {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RbarA3Ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA3Ap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA3Ap {
    #[inline(always)]
    fn from(val: u8) -> RbarA3Ap {
        RbarA3Ap::from_bits(val)
    }
}
impl From<RbarA3Ap> for u8 {
    #[inline(always)]
    fn from(val: RbarA3Ap) -> u8 {
        RbarA3Ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA3Sh {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable"]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RbarA3Sh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA3Sh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA3Sh {
    #[inline(always)]
    fn from(val: u8) -> RbarA3Sh {
        RbarA3Sh::from_bits(val)
    }
}
impl From<RbarA3Sh> for u8 {
    #[inline(always)]
    fn from(val: RbarA3Sh) -> u8 {
        RbarA3Sh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarA3Xn {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RbarA3Xn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarA3Xn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarA3Xn {
    #[inline(always)]
    fn from(val: u8) -> RbarA3Xn {
        RbarA3Xn::from_bits(val)
    }
}
impl From<RbarA3Xn> for u8 {
    #[inline(always)]
    fn from(val: RbarA3Xn) -> u8 {
        RbarA3Xn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarAp {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RbarAp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarAp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarAp {
    #[inline(always)]
    fn from(val: u8) -> RbarAp {
        RbarAp::from_bits(val)
    }
}
impl From<RbarAp> for u8 {
    #[inline(always)]
    fn from(val: RbarAp) -> u8 {
        RbarAp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarSh {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable"]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RbarSh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarSh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarSh {
    #[inline(always)]
    fn from(val: u8) -> RbarSh {
        RbarSh::from_bits(val)
    }
}
impl From<RbarSh> for u8 {
    #[inline(always)]
    fn from(val: RbarSh) -> u8 {
        RbarSh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbarXn {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RbarXn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbarXn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbarXn {
    #[inline(always)]
    fn from(val: u8) -> RbarXn {
        RbarXn::from_bits(val)
    }
}
impl From<RbarXn> for u8 {
    #[inline(always)]
    fn from(val: RbarXn) -> u8 {
        RbarXn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarA1En {
    #[doc = "Region is disabled."]
    DISABLED = 0x0,
    #[doc = "Region is enabled."]
    ENABLED = 0x01,
}
impl RlarA1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarA1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarA1En {
    #[inline(always)]
    fn from(val: u8) -> RlarA1En {
        RlarA1En::from_bits(val)
    }
}
impl From<RlarA1En> for u8 {
    #[inline(always)]
    fn from(val: RlarA1En) -> u8 {
        RlarA1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarA2En {
    #[doc = "Region is disabled."]
    DISABLED = 0x0,
    #[doc = "Region is enabled."]
    ENABLED = 0x01,
}
impl RlarA2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarA2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarA2En {
    #[inline(always)]
    fn from(val: u8) -> RlarA2En {
        RlarA2En::from_bits(val)
    }
}
impl From<RlarA2En> for u8 {
    #[inline(always)]
    fn from(val: RlarA2En) -> u8 {
        RlarA2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarA3En {
    #[doc = "Region is disabled."]
    DISABLED = 0x0,
    #[doc = "Region is enabled."]
    ENABLED = 0x01,
}
impl RlarA3En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarA3En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarA3En {
    #[inline(always)]
    fn from(val: u8) -> RlarA3En {
        RlarA3En::from_bits(val)
    }
}
impl From<RlarA3En> for u8 {
    #[inline(always)]
    fn from(val: RlarA3En) -> u8 {
        RlarA3En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarEn {
    #[doc = "Region is disabled."]
    DISABLED = 0x0,
    #[doc = "Region is enabled."]
    ENABLED = 0x01,
}
impl RlarEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarEn {
    #[inline(always)]
    fn from(val: u8) -> RlarEn {
        RlarEn::from_bits(val)
    }
}
impl From<RlarEn> for u8 {
    #[inline(always)]
    fn from(val: RlarEn) -> u8 {
        RlarEn::to_bits(val)
    }
}
