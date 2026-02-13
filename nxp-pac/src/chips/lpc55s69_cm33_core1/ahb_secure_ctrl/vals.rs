#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AdcRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcRule {
    #[inline(always)]
    fn from(val: u8) -> AdcRule {
        AdcRule::from_bits(val)
    }
}
impl From<AdcRule> for u8 {
    #[inline(always)]
    fn from(val: AdcRule) -> u8 {
        AdcRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecCtrlRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AhbSecCtrlRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecCtrlRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecCtrlRule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecCtrlRule {
        AhbSecCtrlRule::from_bits(val)
    }
}
impl From<AhbSecCtrlRule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecCtrlRule) -> u8 {
        AhbSecCtrlRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecCtrlSect0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AhbSecCtrlSect0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecCtrlSect0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecCtrlSect0Rule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecCtrlSect0Rule {
        AhbSecCtrlSect0Rule::from_bits(val)
    }
}
impl From<AhbSecCtrlSect0Rule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecCtrlSect0Rule) -> u8 {
        AhbSecCtrlSect0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecCtrlSect1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AhbSecCtrlSect1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecCtrlSect1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecCtrlSect1Rule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecCtrlSect1Rule {
        AhbSecCtrlSect1Rule::from_bits(val)
    }
}
impl From<AhbSecCtrlSect1Rule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecCtrlSect1Rule) -> u8 {
        AhbSecCtrlSect1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecCtrlSect2Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AhbSecCtrlSect2Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecCtrlSect2Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecCtrlSect2Rule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecCtrlSect2Rule {
        AhbSecCtrlSect2Rule::from_bits(val)
    }
}
impl From<AhbSecCtrlSect2Rule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecCtrlSect2Rule) -> u8 {
        AhbSecCtrlSect2Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecCtrlSect3Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AhbSecCtrlSect3Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecCtrlSect3Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecCtrlSect3Rule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecCtrlSect3Rule {
        AhbSecCtrlSect3Rule::from_bits(val)
    }
}
impl From<AhbSecCtrlSect3Rule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecCtrlSect3Rule) -> u8 {
        AhbSecCtrlSect3Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AnactrlRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnactrlRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnactrlRule {
    #[inline(always)]
    fn from(val: u8) -> AnactrlRule {
        AnactrlRule::from_bits(val)
    }
}
impl From<AnactrlRule> for u8 {
    #[inline(always)]
    fn from(val: AnactrlRule) -> u8 {
        AnactrlRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apbbridge0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Apbbridge0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apbbridge0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apbbridge0Rule {
    #[inline(always)]
    fn from(val: u8) -> Apbbridge0Rule {
        Apbbridge0Rule::from_bits(val)
    }
}
impl From<Apbbridge0Rule> for u8 {
    #[inline(always)]
    fn from(val: Apbbridge0Rule) -> u8 {
        Apbbridge0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apbbridge1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Apbbridge1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apbbridge1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apbbridge1Rule {
    #[inline(always)]
    fn from(val: u8) -> Apbbridge1Rule {
        Apbbridge1Rule::from_bits(val)
    }
}
impl From<Apbbridge1Rule> for u8 {
    #[inline(always)]
    fn from(val: Apbbridge1Rule) -> u8 {
        Apbbridge1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CasperRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CasperRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CasperRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CasperRule {
    #[inline(always)]
    fn from(val: u8) -> CasperRule {
        CasperRule::from_bits(val)
    }
}
impl From<CasperRule> for u8 {
    #[inline(always)]
    fn from(val: CasperRule) -> u8 {
        CasperRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0LockRegLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu0LockRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0LockRegLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0LockRegLock {
    #[inline(always)]
    fn from(val: u8) -> Cpu0LockRegLock {
        Cpu0LockRegLock::from_bits(val)
    }
}
impl From<Cpu0LockRegLock> for u8 {
    #[inline(always)]
    fn from(val: Cpu0LockRegLock) -> u8 {
        Cpu0LockRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0LockRegLockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu0LockRegLockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0LockRegLockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0LockRegLockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> Cpu0LockRegLockNsMpu {
        Cpu0LockRegLockNsMpu::from_bits(val)
    }
}
impl From<Cpu0LockRegLockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: Cpu0LockRegLockNsMpu) -> u8 {
        Cpu0LockRegLockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0LockRegLockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu0LockRegLockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0LockRegLockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0LockRegLockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> Cpu0LockRegLockNsVtor {
        Cpu0LockRegLockNsVtor::from_bits(val)
    }
}
impl From<Cpu0LockRegLockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: Cpu0LockRegLockNsVtor) -> u8 {
        Cpu0LockRegLockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1LockRegLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu1LockRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1LockRegLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1LockRegLock {
    #[inline(always)]
    fn from(val: u8) -> Cpu1LockRegLock {
        Cpu1LockRegLock::from_bits(val)
    }
}
impl From<Cpu1LockRegLock> for u8 {
    #[inline(always)]
    fn from(val: Cpu1LockRegLock) -> u8 {
        Cpu1LockRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1LockRegLockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu1LockRegLockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1LockRegLockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1LockRegLockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> Cpu1LockRegLockNsMpu {
        Cpu1LockRegLockNsMpu::from_bits(val)
    }
}
impl From<Cpu1LockRegLockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: Cpu1LockRegLockNsMpu) -> u8 {
        Cpu1LockRegLockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1LockRegLockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu1LockRegLockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1LockRegLockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1LockRegLockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> Cpu1LockRegLockNsVtor {
        Cpu1LockRegLockNsVtor::from_bits(val)
    }
}
impl From<Cpu1LockRegLockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: Cpu1LockRegLockNsVtor) -> u8 {
        Cpu1LockRegLockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CrcRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRule {
    #[inline(always)]
    fn from(val: u8) -> CrcRule {
        CrcRule::from_bits(val)
    }
}
impl From<CrcRule> for u8 {
    #[inline(always)]
    fn from(val: CrcRule) -> u8 {
        CrcRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ctimer0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0Rule {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0Rule {
        Ctimer0Rule::from_bits(val)
    }
}
impl From<Ctimer0Rule> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0Rule) -> u8 {
        Ctimer0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ctimer1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1Rule {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1Rule {
        Ctimer1Rule::from_bits(val)
    }
}
impl From<Ctimer1Rule> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1Rule) -> u8 {
        Ctimer1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ctimer2Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2Rule {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2Rule {
        Ctimer2Rule::from_bits(val)
    }
}
impl From<Ctimer2Rule> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2Rule) -> u8 {
        Ctimer2Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ctimer3Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3Rule {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3Rule {
        Ctimer3Rule::from_bits(val)
    }
}
impl From<Ctimer3Rule> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3Rule) -> u8 {
        Ctimer3Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ctimer4Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4Rule {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4Rule {
        Ctimer4Rule::from_bits(val)
    }
}
impl From<Ctimer4Rule> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4Rule) -> u8 {
        Ctimer4Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgMailboxRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl DbgMailboxRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgMailboxRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgMailboxRule {
    #[inline(always)]
    fn from(val: u8) -> DbgMailboxRule {
        DbgMailboxRule::from_bits(val)
    }
}
impl From<DbgMailboxRule> for u8 {
    #[inline(always)]
    fn from(val: DbgMailboxRule) -> u8 {
        DbgMailboxRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Dma0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rule {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rule {
        Dma0Rule::from_bits(val)
    }
}
impl From<Dma0Rule> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rule) -> u8 {
        Dma0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Dma1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rule {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rule {
        Dma1Rule::from_bits(val)
    }
}
impl From<Dma1Rule> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rule) -> u8 {
        Dma1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashCtrlRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FlashCtrlRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashCtrlRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashCtrlRule {
    #[inline(always)]
    fn from(val: u8) -> FlashCtrlRule {
        FlashCtrlRule::from_bits(val)
    }
}
impl From<FlashCtrlRule> for u8 {
    #[inline(always)]
    fn from(val: FlashCtrlRule) -> u8 {
        FlashCtrlRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FlashRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashRule {
    #[inline(always)]
    fn from(val: u8) -> FlashRule {
        FlashRule::from_bits(val)
    }
}
impl From<FlashRule> for u8 {
    #[inline(always)]
    fn from(val: FlashRule) -> u8 {
        FlashRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0Rule {
        Flexcomm0Rule::from_bits(val)
    }
}
impl From<Flexcomm0Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0Rule) -> u8 {
        Flexcomm0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1Rule {
        Flexcomm1Rule::from_bits(val)
    }
}
impl From<Flexcomm1Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1Rule) -> u8 {
        Flexcomm1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm2Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2Rule {
        Flexcomm2Rule::from_bits(val)
    }
}
impl From<Flexcomm2Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2Rule) -> u8 {
        Flexcomm2Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm3Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3Rule {
        Flexcomm3Rule::from_bits(val)
    }
}
impl From<Flexcomm3Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3Rule) -> u8 {
        Flexcomm3Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm4Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4Rule {
        Flexcomm4Rule::from_bits(val)
    }
}
impl From<Flexcomm4Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4Rule) -> u8 {
        Flexcomm4Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm5Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5Rule {
        Flexcomm5Rule::from_bits(val)
    }
}
impl From<Flexcomm5Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5Rule) -> u8 {
        Flexcomm5Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm6Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6Rule {
        Flexcomm6Rule::from_bits(val)
    }
}
impl From<Flexcomm6Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6Rule) -> u8 {
        Flexcomm6Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Flexcomm7Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7Rule {
        Flexcomm7Rule::from_bits(val)
    }
}
impl From<Flexcomm7Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7Rule) -> u8 {
        Flexcomm7Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FsUsbDevRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FsUsbDevRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FsUsbDevRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FsUsbDevRule {
    #[inline(always)]
    fn from(val: u8) -> FsUsbDevRule {
        FsUsbDevRule::from_bits(val)
    }
}
impl From<FsUsbDevRule> for u8 {
    #[inline(always)]
    fn from(val: FsUsbDevRule) -> u8 {
        FsUsbDevRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gint0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Gint0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gint0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gint0Rule {
    #[inline(always)]
    fn from(val: u8) -> Gint0Rule {
        Gint0Rule::from_bits(val)
    }
}
impl From<Gint0Rule> for u8 {
    #[inline(always)]
    fn from(val: Gint0Rule) -> u8 {
        Gint0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gint1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Gint1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gint1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gint1Rule {
    #[inline(always)]
    fn from(val: u8) -> Gint1Rule {
        Gint1Rule::from_bits(val)
    }
}
impl From<Gint1Rule> for u8 {
    #[inline(always)]
    fn from(val: Gint1Rule) -> u8 {
        Gint1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Gpio0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Rule {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Rule {
        Gpio0Rule::from_bits(val)
    }
}
impl From<Gpio0Rule> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Rule) -> u8 {
        Gpio0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Gpio1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Rule {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Rule {
        Gpio1Rule::from_bits(val)
    }
}
impl From<Gpio1Rule> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Rule) -> u8 {
        Gpio1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl HashRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashRule {
    #[inline(always)]
    fn from(val: u8) -> HashRule {
        HashRule::from_bits(val)
    }
}
impl From<HashRule> for u8 {
    #[inline(always)]
    fn from(val: HashRule) -> u8 {
        HashRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsLspiRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl HsLspiRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsLspiRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsLspiRule {
    #[inline(always)]
    fn from(val: u8) -> HsLspiRule {
        HsLspiRule::from_bits(val)
    }
}
impl From<HsLspiRule> for u8 {
    #[inline(always)]
    fn from(val: HsLspiRule) -> u8 {
        HsLspiRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputmuxRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl InputmuxRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InputmuxRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InputmuxRule {
    #[inline(always)]
    fn from(val: u8) -> InputmuxRule {
        InputmuxRule::from_bits(val)
    }
}
impl From<InputmuxRule> for u8 {
    #[inline(always)]
    fn from(val: InputmuxRule) -> u8 {
        InputmuxRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoconRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl IoconRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IoconRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IoconRule {
    #[inline(always)]
    fn from(val: u8) -> IoconRule {
        IoconRule::from_bits(val)
    }
}
impl From<IoconRule> for u8 {
    #[inline(always)]
    fn from(val: IoconRule) -> u8 {
        IoconRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MailboxRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MailboxRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MailboxRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MailboxRule {
    #[inline(always)]
    fn from(val: u8) -> MailboxRule {
        MailboxRule::from_bits(val)
    }
}
impl From<MailboxRule> for u8 {
    #[inline(always)]
    fn from(val: MailboxRule) -> u8 {
        MailboxRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegCpu1c {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegCpu1c {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegCpu1c {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegCpu1c {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegCpu1c {
        MasterSecAntiPolRegCpu1c::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegCpu1c> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegCpu1c) -> u8 {
        MasterSecAntiPolRegCpu1c::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegCpu1s {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegCpu1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegCpu1s {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegCpu1s {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegCpu1s {
        MasterSecAntiPolRegCpu1s::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegCpu1s> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegCpu1s) -> u8 {
        MasterSecAntiPolRegCpu1s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegHash {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegHash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegHash {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegHash {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegHash {
        MasterSecAntiPolRegHash::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegHash> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegHash) -> u8 {
        MasterSecAntiPolRegHash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPq {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPq {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPq {
        MasterSecAntiPolRegPq::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPq> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPq) -> u8 {
        MasterSecAntiPolRegPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSdio {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegSdio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSdio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSdio {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSdio {
        MasterSecAntiPolRegSdio::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSdio> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSdio) -> u8 {
        MasterSecAntiPolRegSdio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSdma0 {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegSdma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSdma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSdma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSdma0 {
        MasterSecAntiPolRegSdma0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSdma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSdma0) -> u8 {
        MasterSecAntiPolRegSdma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSdma1 {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegSdma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSdma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSdma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSdma1 {
        MasterSecAntiPolRegSdma1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSdma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSdma1) -> u8 {
        MasterSecAntiPolRegSdma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsbfsd {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegUsbfsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsbfsd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsbfsd {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsbfsd {
        MasterSecAntiPolRegUsbfsd::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsbfsd> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsbfsd) -> u8 {
        MasterSecAntiPolRegUsbfsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsbfsh {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecAntiPolRegUsbfsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsbfsh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsbfsh {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsbfsh {
        MasterSecAntiPolRegUsbfsh::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsbfsh> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsbfsh) -> u8 {
        MasterSecAntiPolRegUsbfsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntipolLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelAntipolLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntipolLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntipolLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntipolLock {
        MasterSecLevelAntipolLock::from_bits(val)
    }
}
impl From<MasterSecLevelAntipolLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntipolLock) -> u8 {
        MasterSecLevelAntipolLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelCpu1c {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelCpu1c {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelCpu1c {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelCpu1c {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelCpu1c {
        MasterSecLevelCpu1c::from_bits(val)
    }
}
impl From<MasterSecLevelCpu1c> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelCpu1c) -> u8 {
        MasterSecLevelCpu1c::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelCpu1s {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelCpu1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelCpu1s {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelCpu1s {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelCpu1s {
        MasterSecLevelCpu1s::from_bits(val)
    }
}
impl From<MasterSecLevelCpu1s> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelCpu1s) -> u8 {
        MasterSecLevelCpu1s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelHash {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelHash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelHash {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelHash {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelHash {
        MasterSecLevelHash::from_bits(val)
    }
}
impl From<MasterSecLevelHash> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelHash) -> u8 {
        MasterSecLevelHash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelLock {
        MasterSecLevelLock::from_bits(val)
    }
}
impl From<MasterSecLevelLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelLock) -> u8 {
        MasterSecLevelLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPq {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPq {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPq {
        MasterSecLevelPq::from_bits(val)
    }
}
impl From<MasterSecLevelPq> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPq) -> u8 {
        MasterSecLevelPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSdio {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelSdio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSdio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSdio {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSdio {
        MasterSecLevelSdio::from_bits(val)
    }
}
impl From<MasterSecLevelSdio> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSdio) -> u8 {
        MasterSecLevelSdio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSdma0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelSdma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSdma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSdma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSdma0 {
        MasterSecLevelSdma0::from_bits(val)
    }
}
impl From<MasterSecLevelSdma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSdma0) -> u8 {
        MasterSecLevelSdma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSdma1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelSdma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSdma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSdma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSdma1 {
        MasterSecLevelSdma1::from_bits(val)
    }
}
impl From<MasterSecLevelSdma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSdma1) -> u8 {
        MasterSecLevelSdma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsbfsd {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelUsbfsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsbfsd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsbfsd {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsbfsd {
        MasterSecLevelUsbfsd::from_bits(val)
    }
}
impl From<MasterSecLevelUsbfsd> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsbfsd) -> u8 {
        MasterSecLevelUsbfsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsbfsh {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelUsbfsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsbfsh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsbfsh {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsbfsh {
        MasterSecLevelUsbfsh::from_bits(val)
    }
}
impl From<MasterSecLevelUsbfsh> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsbfsh) -> u8 {
        MasterSecLevelUsbfsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableSimpleMasterStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Simple master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableSimpleMasterStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableSimpleMasterStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableSimpleMasterStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableSimpleMasterStrictMode {
        MiscCtrlDpRegDisableSimpleMasterStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableSimpleMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableSimpleMasterStrictMode) -> u8 {
        MiscCtrlDpRegDisableSimpleMasterStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableSmartMasterStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Smart master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableSmartMasterStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableSmartMasterStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableSmartMasterStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableSmartMasterStrictMode {
        MiscCtrlDpRegDisableSmartMasterStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableSmartMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableSmartMasterStrictMode) -> u8 {
        MiscCtrlDpRegDisableSmartMasterStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "Disable abort fort secure checker."]
    DISABLE = 0x01,
    #[doc = "Enable abort fort secure checker."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        MiscCtrlDpRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableViolationAbort) -> u8 {
        MiscCtrlDpRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableNsPrivCheck) -> u8 {
        MiscCtrlDpRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSPrivCheck) -> u8 {
        MiscCtrlDpRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        MiscCtrlDpRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSecureChecking) -> u8 {
        MiscCtrlDpRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disable."]
    DISABLE = 0x01,
    #[doc = "IDAU is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegIdauAllNs {
        MiscCtrlDpRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlDpRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegIdauAllNs) -> u8 {
        MiscCtrlDpRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    RESTRICTED = 0x01,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegWriteLock {
        MiscCtrlDpRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlDpRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegWriteLock) -> u8 {
        MiscCtrlDpRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableSimpleMasterStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Simple master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableSimpleMasterStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableSimpleMasterStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableSimpleMasterStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableSimpleMasterStrictMode {
        MiscCtrlRegDisableSimpleMasterStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableSimpleMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableSimpleMasterStrictMode) -> u8 {
        MiscCtrlRegDisableSimpleMasterStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableSmartMasterStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Smart master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableSmartMasterStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableSmartMasterStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableSmartMasterStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableSmartMasterStrictMode {
        MiscCtrlRegDisableSmartMasterStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableSmartMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableSmartMasterStrictMode) -> u8 {
        MiscCtrlRegDisableSmartMasterStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "Disable abort fort secure checker."]
    DISABLE = 0x01,
    #[doc = "Enable abort fort secure checker."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)"]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        MiscCtrlRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableNsPrivCheck) -> u8 {
        MiscCtrlRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)"]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        MiscCtrlRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSPrivCheck) -> u8 {
        MiscCtrlRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)"]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSecureChecking {
        MiscCtrlRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSecureChecking) -> u8 {
        MiscCtrlRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disable."]
    DISABLE = 0x01,
    #[doc = "IDAU is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    RESTRICTED = 0x01,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MrtRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRule {
    #[inline(always)]
    fn from(val: u8) -> MrtRule {
        MrtRule::from_bits(val)
    }
}
impl From<MrtRule> for u8 {
    #[inline(always)]
    fn from(val: MrtRule) -> u8 {
        MrtRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OseventRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl OseventRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OseventRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OseventRule {
    #[inline(always)]
    fn from(val: u8) -> OseventRule {
        OseventRule::from_bits(val)
    }
}
impl From<OseventRule> for u8 {
    #[inline(always)]
    fn from(val: OseventRule) -> u8 {
        OseventRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PintRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRule {
    #[inline(always)]
    fn from(val: u8) -> PintRule {
        PintRule::from_bits(val)
    }
}
impl From<PintRule> for u8 {
    #[inline(always)]
    fn from(val: PintRule) -> u8 {
        PintRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin0SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin0SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin0SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin0SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin0SecMask {
        Pio0Pin0SecMask::from_bits(val)
    }
}
impl From<Pio0Pin0SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin0SecMask) -> u8 {
        Pio0Pin0SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin10SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin10SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin10SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin10SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin10SecMask {
        Pio0Pin10SecMask::from_bits(val)
    }
}
impl From<Pio0Pin10SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin10SecMask) -> u8 {
        Pio0Pin10SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin11SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin11SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin11SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin11SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin11SecMask {
        Pio0Pin11SecMask::from_bits(val)
    }
}
impl From<Pio0Pin11SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin11SecMask) -> u8 {
        Pio0Pin11SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin12SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin12SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin12SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin12SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin12SecMask {
        Pio0Pin12SecMask::from_bits(val)
    }
}
impl From<Pio0Pin12SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin12SecMask) -> u8 {
        Pio0Pin12SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin13SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin13SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin13SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin13SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin13SecMask {
        Pio0Pin13SecMask::from_bits(val)
    }
}
impl From<Pio0Pin13SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin13SecMask) -> u8 {
        Pio0Pin13SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin14SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin14SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin14SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin14SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin14SecMask {
        Pio0Pin14SecMask::from_bits(val)
    }
}
impl From<Pio0Pin14SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin14SecMask) -> u8 {
        Pio0Pin14SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin15SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin15SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin15SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin15SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin15SecMask {
        Pio0Pin15SecMask::from_bits(val)
    }
}
impl From<Pio0Pin15SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin15SecMask) -> u8 {
        Pio0Pin15SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin16SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin16SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin16SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin16SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin16SecMask {
        Pio0Pin16SecMask::from_bits(val)
    }
}
impl From<Pio0Pin16SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin16SecMask) -> u8 {
        Pio0Pin16SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin17SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin17SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin17SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin17SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin17SecMask {
        Pio0Pin17SecMask::from_bits(val)
    }
}
impl From<Pio0Pin17SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin17SecMask) -> u8 {
        Pio0Pin17SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin18SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin18SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin18SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin18SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin18SecMask {
        Pio0Pin18SecMask::from_bits(val)
    }
}
impl From<Pio0Pin18SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin18SecMask) -> u8 {
        Pio0Pin18SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin19SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin19SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin19SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin19SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin19SecMask {
        Pio0Pin19SecMask::from_bits(val)
    }
}
impl From<Pio0Pin19SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin19SecMask) -> u8 {
        Pio0Pin19SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin1SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin1SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin1SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin1SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin1SecMask {
        Pio0Pin1SecMask::from_bits(val)
    }
}
impl From<Pio0Pin1SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin1SecMask) -> u8 {
        Pio0Pin1SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin20SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin20SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin20SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin20SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin20SecMask {
        Pio0Pin20SecMask::from_bits(val)
    }
}
impl From<Pio0Pin20SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin20SecMask) -> u8 {
        Pio0Pin20SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin21SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin21SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin21SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin21SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin21SecMask {
        Pio0Pin21SecMask::from_bits(val)
    }
}
impl From<Pio0Pin21SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin21SecMask) -> u8 {
        Pio0Pin21SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin22SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin22SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin22SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin22SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin22SecMask {
        Pio0Pin22SecMask::from_bits(val)
    }
}
impl From<Pio0Pin22SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin22SecMask) -> u8 {
        Pio0Pin22SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin23SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin23SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin23SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin23SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin23SecMask {
        Pio0Pin23SecMask::from_bits(val)
    }
}
impl From<Pio0Pin23SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin23SecMask) -> u8 {
        Pio0Pin23SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin24SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin24SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin24SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin24SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin24SecMask {
        Pio0Pin24SecMask::from_bits(val)
    }
}
impl From<Pio0Pin24SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin24SecMask) -> u8 {
        Pio0Pin24SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin25SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin25SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin25SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin25SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin25SecMask {
        Pio0Pin25SecMask::from_bits(val)
    }
}
impl From<Pio0Pin25SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin25SecMask) -> u8 {
        Pio0Pin25SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin26SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin26SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin26SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin26SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin26SecMask {
        Pio0Pin26SecMask::from_bits(val)
    }
}
impl From<Pio0Pin26SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin26SecMask) -> u8 {
        Pio0Pin26SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin27SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin27SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin27SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin27SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin27SecMask {
        Pio0Pin27SecMask::from_bits(val)
    }
}
impl From<Pio0Pin27SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin27SecMask) -> u8 {
        Pio0Pin27SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin28SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin28SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin28SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin28SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin28SecMask {
        Pio0Pin28SecMask::from_bits(val)
    }
}
impl From<Pio0Pin28SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin28SecMask) -> u8 {
        Pio0Pin28SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin29SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin29SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin29SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin29SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin29SecMask {
        Pio0Pin29SecMask::from_bits(val)
    }
}
impl From<Pio0Pin29SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin29SecMask) -> u8 {
        Pio0Pin29SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin2SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin2SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin2SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin2SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin2SecMask {
        Pio0Pin2SecMask::from_bits(val)
    }
}
impl From<Pio0Pin2SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin2SecMask) -> u8 {
        Pio0Pin2SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin30SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin30SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin30SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin30SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin30SecMask {
        Pio0Pin30SecMask::from_bits(val)
    }
}
impl From<Pio0Pin30SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin30SecMask) -> u8 {
        Pio0Pin30SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin31SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin31SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin31SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin31SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin31SecMask {
        Pio0Pin31SecMask::from_bits(val)
    }
}
impl From<Pio0Pin31SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin31SecMask) -> u8 {
        Pio0Pin31SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin3SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin3SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin3SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin3SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin3SecMask {
        Pio0Pin3SecMask::from_bits(val)
    }
}
impl From<Pio0Pin3SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin3SecMask) -> u8 {
        Pio0Pin3SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin4SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin4SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin4SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin4SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin4SecMask {
        Pio0Pin4SecMask::from_bits(val)
    }
}
impl From<Pio0Pin4SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin4SecMask) -> u8 {
        Pio0Pin4SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin5SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin5SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin5SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin5SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin5SecMask {
        Pio0Pin5SecMask::from_bits(val)
    }
}
impl From<Pio0Pin5SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin5SecMask) -> u8 {
        Pio0Pin5SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin6SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin6SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin6SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin6SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin6SecMask {
        Pio0Pin6SecMask::from_bits(val)
    }
}
impl From<Pio0Pin6SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin6SecMask) -> u8 {
        Pio0Pin6SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin7SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin7SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin7SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin7SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin7SecMask {
        Pio0Pin7SecMask::from_bits(val)
    }
}
impl From<Pio0Pin7SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin7SecMask) -> u8 {
        Pio0Pin7SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin8SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin8SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin8SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin8SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin8SecMask {
        Pio0Pin8SecMask::from_bits(val)
    }
}
impl From<Pio0Pin8SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin8SecMask) -> u8 {
        Pio0Pin8SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0Pin9SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio0Pin9SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0Pin9SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0Pin9SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0Pin9SecMask {
        Pio0Pin9SecMask::from_bits(val)
    }
}
impl From<Pio0Pin9SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0Pin9SecMask) -> u8 {
        Pio0Pin9SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin0SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin0SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin0SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin0SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin0SecMask {
        Pio1Pin0SecMask::from_bits(val)
    }
}
impl From<Pio1Pin0SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin0SecMask) -> u8 {
        Pio1Pin0SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin10SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin10SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin10SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin10SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin10SecMask {
        Pio1Pin10SecMask::from_bits(val)
    }
}
impl From<Pio1Pin10SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin10SecMask) -> u8 {
        Pio1Pin10SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin11SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin11SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin11SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin11SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin11SecMask {
        Pio1Pin11SecMask::from_bits(val)
    }
}
impl From<Pio1Pin11SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin11SecMask) -> u8 {
        Pio1Pin11SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin12SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin12SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin12SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin12SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin12SecMask {
        Pio1Pin12SecMask::from_bits(val)
    }
}
impl From<Pio1Pin12SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin12SecMask) -> u8 {
        Pio1Pin12SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin13SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin13SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin13SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin13SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin13SecMask {
        Pio1Pin13SecMask::from_bits(val)
    }
}
impl From<Pio1Pin13SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin13SecMask) -> u8 {
        Pio1Pin13SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin14SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin14SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin14SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin14SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin14SecMask {
        Pio1Pin14SecMask::from_bits(val)
    }
}
impl From<Pio1Pin14SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin14SecMask) -> u8 {
        Pio1Pin14SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin15SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin15SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin15SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin15SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin15SecMask {
        Pio1Pin15SecMask::from_bits(val)
    }
}
impl From<Pio1Pin15SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin15SecMask) -> u8 {
        Pio1Pin15SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin16SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin16SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin16SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin16SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin16SecMask {
        Pio1Pin16SecMask::from_bits(val)
    }
}
impl From<Pio1Pin16SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin16SecMask) -> u8 {
        Pio1Pin16SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin17SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin17SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin17SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin17SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin17SecMask {
        Pio1Pin17SecMask::from_bits(val)
    }
}
impl From<Pio1Pin17SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin17SecMask) -> u8 {
        Pio1Pin17SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin18SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin18SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin18SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin18SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin18SecMask {
        Pio1Pin18SecMask::from_bits(val)
    }
}
impl From<Pio1Pin18SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin18SecMask) -> u8 {
        Pio1Pin18SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin19SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin19SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin19SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin19SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin19SecMask {
        Pio1Pin19SecMask::from_bits(val)
    }
}
impl From<Pio1Pin19SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin19SecMask) -> u8 {
        Pio1Pin19SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin1SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin1SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin1SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin1SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin1SecMask {
        Pio1Pin1SecMask::from_bits(val)
    }
}
impl From<Pio1Pin1SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin1SecMask) -> u8 {
        Pio1Pin1SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin20SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin20SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin20SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin20SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin20SecMask {
        Pio1Pin20SecMask::from_bits(val)
    }
}
impl From<Pio1Pin20SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin20SecMask) -> u8 {
        Pio1Pin20SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin21SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin21SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin21SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin21SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin21SecMask {
        Pio1Pin21SecMask::from_bits(val)
    }
}
impl From<Pio1Pin21SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin21SecMask) -> u8 {
        Pio1Pin21SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin22SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin22SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin22SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin22SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin22SecMask {
        Pio1Pin22SecMask::from_bits(val)
    }
}
impl From<Pio1Pin22SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin22SecMask) -> u8 {
        Pio1Pin22SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin23SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin23SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin23SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin23SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin23SecMask {
        Pio1Pin23SecMask::from_bits(val)
    }
}
impl From<Pio1Pin23SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin23SecMask) -> u8 {
        Pio1Pin23SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin24SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin24SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin24SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin24SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin24SecMask {
        Pio1Pin24SecMask::from_bits(val)
    }
}
impl From<Pio1Pin24SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin24SecMask) -> u8 {
        Pio1Pin24SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin25SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin25SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin25SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin25SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin25SecMask {
        Pio1Pin25SecMask::from_bits(val)
    }
}
impl From<Pio1Pin25SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin25SecMask) -> u8 {
        Pio1Pin25SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin26SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin26SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin26SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin26SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin26SecMask {
        Pio1Pin26SecMask::from_bits(val)
    }
}
impl From<Pio1Pin26SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin26SecMask) -> u8 {
        Pio1Pin26SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin27SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin27SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin27SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin27SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin27SecMask {
        Pio1Pin27SecMask::from_bits(val)
    }
}
impl From<Pio1Pin27SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin27SecMask) -> u8 {
        Pio1Pin27SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin28SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin28SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin28SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin28SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin28SecMask {
        Pio1Pin28SecMask::from_bits(val)
    }
}
impl From<Pio1Pin28SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin28SecMask) -> u8 {
        Pio1Pin28SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin29SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin29SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin29SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin29SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin29SecMask {
        Pio1Pin29SecMask::from_bits(val)
    }
}
impl From<Pio1Pin29SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin29SecMask) -> u8 {
        Pio1Pin29SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin2SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin2SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin2SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin2SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin2SecMask {
        Pio1Pin2SecMask::from_bits(val)
    }
}
impl From<Pio1Pin2SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin2SecMask) -> u8 {
        Pio1Pin2SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin30SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin30SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin30SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin30SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin30SecMask {
        Pio1Pin30SecMask::from_bits(val)
    }
}
impl From<Pio1Pin30SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin30SecMask) -> u8 {
        Pio1Pin30SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin31SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin31SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin31SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin31SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin31SecMask {
        Pio1Pin31SecMask::from_bits(val)
    }
}
impl From<Pio1Pin31SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin31SecMask) -> u8 {
        Pio1Pin31SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin3SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin3SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin3SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin3SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin3SecMask {
        Pio1Pin3SecMask::from_bits(val)
    }
}
impl From<Pio1Pin3SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin3SecMask) -> u8 {
        Pio1Pin3SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin4SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin4SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin4SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin4SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin4SecMask {
        Pio1Pin4SecMask::from_bits(val)
    }
}
impl From<Pio1Pin4SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin4SecMask) -> u8 {
        Pio1Pin4SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin5SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin5SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin5SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin5SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin5SecMask {
        Pio1Pin5SecMask::from_bits(val)
    }
}
impl From<Pio1Pin5SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin5SecMask) -> u8 {
        Pio1Pin5SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin6SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin6SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin6SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin6SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin6SecMask {
        Pio1Pin6SecMask::from_bits(val)
    }
}
impl From<Pio1Pin6SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin6SecMask) -> u8 {
        Pio1Pin6SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin7SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin7SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin7SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin7SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin7SecMask {
        Pio1Pin7SecMask::from_bits(val)
    }
}
impl From<Pio1Pin7SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin7SecMask) -> u8 {
        Pio1Pin7SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin8SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin8SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin8SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin8SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin8SecMask {
        Pio1Pin8SecMask::from_bits(val)
    }
}
impl From<Pio1Pin8SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin8SecMask) -> u8 {
        Pio1Pin8SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio1Pin9SecMask {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl Pio1Pin9SecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio1Pin9SecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio1Pin9SecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio1Pin9SecMask {
        Pio1Pin9SecMask::from_bits(val)
    }
}
impl From<Pio1Pin9SecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio1Pin9SecMask) -> u8 {
        Pio1Pin9SecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PluRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluRule {
    #[inline(always)]
    fn from(val: u8) -> PluRule {
        PluRule::from_bits(val)
    }
}
impl From<PluRule> for u8 {
    #[inline(always)]
    fn from(val: PluRule) -> u8 {
        PluRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PmcRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcRule {
    #[inline(always)]
    fn from(val: u8) -> PmcRule {
        PmcRule::from_bits(val)
    }
}
impl From<PmcRule> for u8 {
    #[inline(always)]
    fn from(val: PmcRule) -> u8 {
        PmcRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PqRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PqRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PqRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PqRule {
    #[inline(always)]
    fn from(val: u8) -> PqRule {
        PqRule::from_bits(val)
    }
}
impl From<PqRule> for u8 {
    #[inline(always)]
    fn from(val: PqRule) -> u8 {
        PqRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrinceRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PrinceRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrinceRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrinceRule {
    #[inline(always)]
    fn from(val: u8) -> PrinceRule {
        PrinceRule::from_bits(val)
    }
}
impl From<PrinceRule> for u8 {
    #[inline(always)]
    fn from(val: PrinceRule) -> u8 {
        PrinceRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PufRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRule {
    #[inline(always)]
    fn from(val: u8) -> PufRule {
        PufRule::from_bits(val)
    }
}
impl From<PufRule> for u8 {
    #[inline(always)]
    fn from(val: PufRule) -> u8 {
        PufRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ram0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram0Rule {
    #[inline(always)]
    fn from(val: u8) -> Ram0Rule {
        Ram0Rule::from_bits(val)
    }
}
impl From<Ram0Rule> for u8 {
    #[inline(always)]
    fn from(val: Ram0Rule) -> u8 {
        Ram0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ram1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram1Rule {
    #[inline(always)]
    fn from(val: u8) -> Ram1Rule {
        Ram1Rule::from_bits(val)
    }
}
impl From<Ram1Rule> for u8 {
    #[inline(always)]
    fn from(val: Ram1Rule) -> u8 {
        Ram1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram2Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ram2Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram2Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram2Rule {
    #[inline(always)]
    fn from(val: u8) -> Ram2Rule {
        Ram2Rule::from_bits(val)
    }
}
impl From<Ram2Rule> for u8 {
    #[inline(always)]
    fn from(val: Ram2Rule) -> u8 {
        Ram2Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram3Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ram3Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram3Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram3Rule {
    #[inline(always)]
    fn from(val: u8) -> Ram3Rule {
        Ram3Rule::from_bits(val)
    }
}
impl From<Ram3Rule> for u8 {
    #[inline(always)]
    fn from(val: Ram3Rule) -> u8 {
        Ram3Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram4Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl Ram4Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram4Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram4Rule {
    #[inline(always)]
    fn from(val: u8) -> Ram4Rule {
        Ram4Rule::from_bits(val)
    }
}
impl From<Ram4Rule> for u8 {
    #[inline(always)]
    fn from(val: Ram4Rule) -> u8 {
        Ram4Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamUsbHsRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RamUsbHsRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamUsbHsRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamUsbHsRule {
    #[inline(always)]
    fn from(val: u8) -> RamUsbHsRule {
        RamUsbHsRule::from_bits(val)
    }
}
impl From<RamUsbHsRule> for u8 {
    #[inline(always)]
    fn from(val: RamUsbHsRule) -> u8 {
        RamUsbHsRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RamxRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxRule {
    #[inline(always)]
    fn from(val: u8) -> RamxRule {
        RamxRule::from_bits(val)
    }
}
impl From<RamxRule> for u8 {
    #[inline(always)]
    fn from(val: RamxRule) -> u8 {
        RamxRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RngRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RngRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RngRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RngRule {
    #[inline(always)]
    fn from(val: u8) -> RngRule {
        RngRule::from_bits(val)
    }
}
impl From<RngRule> for u8 {
    #[inline(always)]
    fn from(val: RngRule) -> u8 {
        RngRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RomRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomRule {
    #[inline(always)]
    fn from(val: u8) -> RomRule {
        RomRule::from_bits(val)
    }
}
impl From<RomRule> for u8 {
    #[inline(always)]
    fn from(val: RomRule) -> u8 {
        RomRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RtcRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRule {
    #[inline(always)]
    fn from(val: u8) -> RtcRule {
        RtcRule::from_bits(val)
    }
}
impl From<RtcRule> for u8 {
    #[inline(always)]
    fn from(val: RtcRule) -> u8 {
        RtcRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SctRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctRule {
    #[inline(always)]
    fn from(val: u8) -> SctRule {
        SctRule::from_bits(val)
    }
}
impl From<SctRule> for u8 {
    #[inline(always)]
    fn from(val: SctRule) -> u8 {
        SctRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdioRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SdioRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdioRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdioRule {
    #[inline(always)]
    fn from(val: u8) -> SdioRule {
        SdioRule::from_bits(val)
    }
}
impl From<SdioRule> for u8 {
    #[inline(always)]
    fn from(val: SdioRule) -> u8 {
        SdioRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCpu1IntMask0Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecCpu1IntMask0Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCpu1IntMask0Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCpu1IntMask0Lock {
    #[inline(always)]
    fn from(val: u8) -> SecCpu1IntMask0Lock {
        SecCpu1IntMask0Lock::from_bits(val)
    }
}
impl From<SecCpu1IntMask0Lock> for u8 {
    #[inline(always)]
    fn from(val: SecCpu1IntMask0Lock) -> u8 {
        SecCpu1IntMask0Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCpu1IntMask1Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecCpu1IntMask1Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCpu1IntMask1Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCpu1IntMask1Lock {
    #[inline(always)]
    fn from(val: u8) -> SecCpu1IntMask1Lock {
        SecCpu1IntMask1Lock::from_bits(val)
    }
}
impl From<SecCpu1IntMask1Lock> for u8 {
    #[inline(always)]
    fn from(val: SecCpu1IntMask1Lock) -> u8 {
        SecCpu1IntMask1Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule0 {
        SecCtrlFlashMemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule0) -> u8 {
        SecCtrlFlashMemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule1 {
        SecCtrlFlashMemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule1) -> u8 {
        SecCtrlFlashMemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule2 {
        SecCtrlFlashMemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule2) -> u8 {
        SecCtrlFlashMemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule3 {
        SecCtrlFlashMemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule3) -> u8 {
        SecCtrlFlashMemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule4 {
        SecCtrlFlashMemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule4) -> u8 {
        SecCtrlFlashMemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule5 {
        SecCtrlFlashMemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule5) -> u8 {
        SecCtrlFlashMemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule6 {
        SecCtrlFlashMemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule6) -> u8 {
        SecCtrlFlashMemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule0Rule7 {
        SecCtrlFlashMemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule0Rule7) -> u8 {
        SecCtrlFlashMemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule0 {
        SecCtrlFlashMemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule0) -> u8 {
        SecCtrlFlashMemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule1 {
        SecCtrlFlashMemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule1) -> u8 {
        SecCtrlFlashMemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule2 {
        SecCtrlFlashMemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule2) -> u8 {
        SecCtrlFlashMemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule3 {
        SecCtrlFlashMemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule3) -> u8 {
        SecCtrlFlashMemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule4 {
        SecCtrlFlashMemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule4) -> u8 {
        SecCtrlFlashMemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule5 {
        SecCtrlFlashMemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule5) -> u8 {
        SecCtrlFlashMemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule6 {
        SecCtrlFlashMemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule6) -> u8 {
        SecCtrlFlashMemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule1Rule7 {
        SecCtrlFlashMemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule1Rule7) -> u8 {
        SecCtrlFlashMemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule0 {
        SecCtrlFlashMemRule2Rule0::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule0) -> u8 {
        SecCtrlFlashMemRule2Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule1 {
        SecCtrlFlashMemRule2Rule1::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule1) -> u8 {
        SecCtrlFlashMemRule2Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule2 {
        SecCtrlFlashMemRule2Rule2::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule2) -> u8 {
        SecCtrlFlashMemRule2Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule3 {
        SecCtrlFlashMemRule2Rule3::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule3) -> u8 {
        SecCtrlFlashMemRule2Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule4 {
        SecCtrlFlashMemRule2Rule4::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule4) -> u8 {
        SecCtrlFlashMemRule2Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule5 {
        SecCtrlFlashMemRule2Rule5::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule5) -> u8 {
        SecCtrlFlashMemRule2Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule6 {
        SecCtrlFlashMemRule2Rule6::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule6) -> u8 {
        SecCtrlFlashMemRule2Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlFlashMemRule2Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlFlashMemRule2Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlFlashMemRule2Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlFlashMemRule2Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlFlashMemRule2Rule7 {
        SecCtrlFlashMemRule2Rule7::from_bits(val)
    }
}
impl From<SecCtrlFlashMemRule2Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlFlashMemRule2Rule7) -> u8 {
        SecCtrlFlashMemRule2Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule0 {
        SecCtrlRam0MemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule0) -> u8 {
        SecCtrlRam0MemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule1 {
        SecCtrlRam0MemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule1) -> u8 {
        SecCtrlRam0MemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule2 {
        SecCtrlRam0MemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule2) -> u8 {
        SecCtrlRam0MemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule3 {
        SecCtrlRam0MemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule3) -> u8 {
        SecCtrlRam0MemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule4 {
        SecCtrlRam0MemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule4) -> u8 {
        SecCtrlRam0MemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule5 {
        SecCtrlRam0MemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule5) -> u8 {
        SecCtrlRam0MemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule6 {
        SecCtrlRam0MemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule6) -> u8 {
        SecCtrlRam0MemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule0Rule7 {
        SecCtrlRam0MemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule0Rule7) -> u8 {
        SecCtrlRam0MemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule0 {
        SecCtrlRam0MemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule0) -> u8 {
        SecCtrlRam0MemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule1 {
        SecCtrlRam0MemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule1) -> u8 {
        SecCtrlRam0MemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule2 {
        SecCtrlRam0MemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule2) -> u8 {
        SecCtrlRam0MemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule3 {
        SecCtrlRam0MemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule3) -> u8 {
        SecCtrlRam0MemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule4 {
        SecCtrlRam0MemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule4) -> u8 {
        SecCtrlRam0MemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule5 {
        SecCtrlRam0MemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule5) -> u8 {
        SecCtrlRam0MemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule6 {
        SecCtrlRam0MemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule6) -> u8 {
        SecCtrlRam0MemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam0MemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam0MemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam0MemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam0MemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam0MemRule1Rule7 {
        SecCtrlRam0MemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam0MemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam0MemRule1Rule7) -> u8 {
        SecCtrlRam0MemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule0 {
        SecCtrlRam1MemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule0) -> u8 {
        SecCtrlRam1MemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule1 {
        SecCtrlRam1MemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule1) -> u8 {
        SecCtrlRam1MemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule2 {
        SecCtrlRam1MemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule2) -> u8 {
        SecCtrlRam1MemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule3 {
        SecCtrlRam1MemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule3) -> u8 {
        SecCtrlRam1MemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule4 {
        SecCtrlRam1MemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule4) -> u8 {
        SecCtrlRam1MemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule5 {
        SecCtrlRam1MemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule5) -> u8 {
        SecCtrlRam1MemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule6 {
        SecCtrlRam1MemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule6) -> u8 {
        SecCtrlRam1MemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule0Rule7 {
        SecCtrlRam1MemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule0Rule7) -> u8 {
        SecCtrlRam1MemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule0 {
        SecCtrlRam1MemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule0) -> u8 {
        SecCtrlRam1MemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule1 {
        SecCtrlRam1MemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule1) -> u8 {
        SecCtrlRam1MemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule2 {
        SecCtrlRam1MemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule2) -> u8 {
        SecCtrlRam1MemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule3 {
        SecCtrlRam1MemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule3) -> u8 {
        SecCtrlRam1MemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule4 {
        SecCtrlRam1MemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule4) -> u8 {
        SecCtrlRam1MemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule5 {
        SecCtrlRam1MemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule5) -> u8 {
        SecCtrlRam1MemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule6 {
        SecCtrlRam1MemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule6) -> u8 {
        SecCtrlRam1MemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam1MemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam1MemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam1MemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam1MemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam1MemRule1Rule7 {
        SecCtrlRam1MemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam1MemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam1MemRule1Rule7) -> u8 {
        SecCtrlRam1MemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule0 {
        SecCtrlRam2MemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule0) -> u8 {
        SecCtrlRam2MemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule1 {
        SecCtrlRam2MemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule1) -> u8 {
        SecCtrlRam2MemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule2 {
        SecCtrlRam2MemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule2) -> u8 {
        SecCtrlRam2MemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule3 {
        SecCtrlRam2MemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule3) -> u8 {
        SecCtrlRam2MemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule4 {
        SecCtrlRam2MemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule4) -> u8 {
        SecCtrlRam2MemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule5 {
        SecCtrlRam2MemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule5) -> u8 {
        SecCtrlRam2MemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule6 {
        SecCtrlRam2MemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule6) -> u8 {
        SecCtrlRam2MemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule0Rule7 {
        SecCtrlRam2MemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule0Rule7) -> u8 {
        SecCtrlRam2MemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule0 {
        SecCtrlRam2MemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule0) -> u8 {
        SecCtrlRam2MemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule1 {
        SecCtrlRam2MemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule1) -> u8 {
        SecCtrlRam2MemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule2 {
        SecCtrlRam2MemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule2) -> u8 {
        SecCtrlRam2MemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule3 {
        SecCtrlRam2MemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule3) -> u8 {
        SecCtrlRam2MemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule4 {
        SecCtrlRam2MemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule4) -> u8 {
        SecCtrlRam2MemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule5 {
        SecCtrlRam2MemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule5) -> u8 {
        SecCtrlRam2MemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule6 {
        SecCtrlRam2MemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule6) -> u8 {
        SecCtrlRam2MemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam2MemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam2MemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam2MemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam2MemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam2MemRule1Rule7 {
        SecCtrlRam2MemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam2MemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam2MemRule1Rule7) -> u8 {
        SecCtrlRam2MemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule0 {
        SecCtrlRam3MemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule0) -> u8 {
        SecCtrlRam3MemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule1 {
        SecCtrlRam3MemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule1) -> u8 {
        SecCtrlRam3MemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule2 {
        SecCtrlRam3MemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule2) -> u8 {
        SecCtrlRam3MemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule3 {
        SecCtrlRam3MemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule3) -> u8 {
        SecCtrlRam3MemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule4 {
        SecCtrlRam3MemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule4) -> u8 {
        SecCtrlRam3MemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule5 {
        SecCtrlRam3MemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule5) -> u8 {
        SecCtrlRam3MemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule6 {
        SecCtrlRam3MemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule6) -> u8 {
        SecCtrlRam3MemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule0Rule7 {
        SecCtrlRam3MemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule0Rule7) -> u8 {
        SecCtrlRam3MemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule0 {
        SecCtrlRam3MemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule0) -> u8 {
        SecCtrlRam3MemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule1 {
        SecCtrlRam3MemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule1) -> u8 {
        SecCtrlRam3MemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule2 {
        SecCtrlRam3MemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule2) -> u8 {
        SecCtrlRam3MemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule3 {
        SecCtrlRam3MemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule3) -> u8 {
        SecCtrlRam3MemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule4 {
        SecCtrlRam3MemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule4) -> u8 {
        SecCtrlRam3MemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule5 {
        SecCtrlRam3MemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule5) -> u8 {
        SecCtrlRam3MemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule6 {
        SecCtrlRam3MemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule6) -> u8 {
        SecCtrlRam3MemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam3MemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam3MemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam3MemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam3MemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam3MemRule1Rule7 {
        SecCtrlRam3MemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlRam3MemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam3MemRule1Rule7) -> u8 {
        SecCtrlRam3MemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam4MemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam4MemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam4MemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam4MemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam4MemRule0Rule0 {
        SecCtrlRam4MemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRam4MemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam4MemRule0Rule0) -> u8 {
        SecCtrlRam4MemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam4MemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam4MemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam4MemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam4MemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam4MemRule0Rule1 {
        SecCtrlRam4MemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRam4MemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam4MemRule0Rule1) -> u8 {
        SecCtrlRam4MemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam4MemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam4MemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam4MemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam4MemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam4MemRule0Rule2 {
        SecCtrlRam4MemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRam4MemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam4MemRule0Rule2) -> u8 {
        SecCtrlRam4MemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRam4MemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRam4MemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRam4MemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRam4MemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRam4MemRule0Rule3 {
        SecCtrlRam4MemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRam4MemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRam4MemRule0Rule3) -> u8 {
        SecCtrlRam4MemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule0 {
        SecCtrlRamxMemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule0) -> u8 {
        SecCtrlRamxMemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule1 {
        SecCtrlRamxMemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule1) -> u8 {
        SecCtrlRamxMemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule2 {
        SecCtrlRamxMemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule2) -> u8 {
        SecCtrlRamxMemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule3 {
        SecCtrlRamxMemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule3) -> u8 {
        SecCtrlRamxMemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule4 {
        SecCtrlRamxMemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule4) -> u8 {
        SecCtrlRamxMemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule5 {
        SecCtrlRamxMemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule5) -> u8 {
        SecCtrlRamxMemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule6 {
        SecCtrlRamxMemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule6) -> u8 {
        SecCtrlRamxMemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRamxMemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRamxMemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRamxMemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRamxMemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRamxMemRule0Rule7 {
        SecCtrlRamxMemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRamxMemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRamxMemRule0Rule7) -> u8 {
        SecCtrlRamxMemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule0 {
        SecCtrlRomMemRule0Rule0::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule0) -> u8 {
        SecCtrlRomMemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule1 {
        SecCtrlRomMemRule0Rule1::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule1) -> u8 {
        SecCtrlRomMemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule2 {
        SecCtrlRomMemRule0Rule2::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule2) -> u8 {
        SecCtrlRomMemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule3 {
        SecCtrlRomMemRule0Rule3::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule3) -> u8 {
        SecCtrlRomMemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule4 {
        SecCtrlRomMemRule0Rule4::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule4) -> u8 {
        SecCtrlRomMemRule0Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule5 {
        SecCtrlRomMemRule0Rule5::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule5) -> u8 {
        SecCtrlRomMemRule0Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule6 {
        SecCtrlRomMemRule0Rule6::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule6) -> u8 {
        SecCtrlRomMemRule0Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule0Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule0Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule0Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule0Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule0Rule7 {
        SecCtrlRomMemRule0Rule7::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule0Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule0Rule7) -> u8 {
        SecCtrlRomMemRule0Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule0 {
        SecCtrlRomMemRule1Rule0::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule0) -> u8 {
        SecCtrlRomMemRule1Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule1 {
        SecCtrlRomMemRule1Rule1::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule1) -> u8 {
        SecCtrlRomMemRule1Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule2 {
        SecCtrlRomMemRule1Rule2::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule2) -> u8 {
        SecCtrlRomMemRule1Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule3 {
        SecCtrlRomMemRule1Rule3::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule3) -> u8 {
        SecCtrlRomMemRule1Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule4 {
        SecCtrlRomMemRule1Rule4::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule4) -> u8 {
        SecCtrlRomMemRule1Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule5 {
        SecCtrlRomMemRule1Rule5::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule5) -> u8 {
        SecCtrlRomMemRule1Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule6 {
        SecCtrlRomMemRule1Rule6::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule6) -> u8 {
        SecCtrlRomMemRule1Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule1Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule1Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule1Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule1Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule1Rule7 {
        SecCtrlRomMemRule1Rule7::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule1Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule1Rule7) -> u8 {
        SecCtrlRomMemRule1Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule0 {
        SecCtrlRomMemRule2Rule0::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule0) -> u8 {
        SecCtrlRomMemRule2Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule1 {
        SecCtrlRomMemRule2Rule1::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule1) -> u8 {
        SecCtrlRomMemRule2Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule2 {
        SecCtrlRomMemRule2Rule2::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule2) -> u8 {
        SecCtrlRomMemRule2Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule3 {
        SecCtrlRomMemRule2Rule3::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule3) -> u8 {
        SecCtrlRomMemRule2Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule4 {
        SecCtrlRomMemRule2Rule4::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule4) -> u8 {
        SecCtrlRomMemRule2Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule5 {
        SecCtrlRomMemRule2Rule5::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule5) -> u8 {
        SecCtrlRomMemRule2Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule6 {
        SecCtrlRomMemRule2Rule6::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule6) -> u8 {
        SecCtrlRomMemRule2Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule2Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule2Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule2Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule2Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule2Rule7 {
        SecCtrlRomMemRule2Rule7::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule2Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule2Rule7) -> u8 {
        SecCtrlRomMemRule2Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule0 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule0 {
        SecCtrlRomMemRule3Rule0::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule0> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule0) -> u8 {
        SecCtrlRomMemRule3Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule1 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule1 {
        SecCtrlRomMemRule3Rule1::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule1> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule1) -> u8 {
        SecCtrlRomMemRule3Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule2 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule2 {
        SecCtrlRomMemRule3Rule2::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule2> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule2) -> u8 {
        SecCtrlRomMemRule3Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule3 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule3 {
        SecCtrlRomMemRule3Rule3::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule3> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule3) -> u8 {
        SecCtrlRomMemRule3Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule4 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule4 {
        SecCtrlRomMemRule3Rule4::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule4> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule4) -> u8 {
        SecCtrlRomMemRule3Rule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule5 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule5 {
        SecCtrlRomMemRule3Rule5::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule5> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule5) -> u8 {
        SecCtrlRomMemRule3Rule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule6 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule6 {
        SecCtrlRomMemRule3Rule6::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule6> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule6) -> u8 {
        SecCtrlRomMemRule3Rule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCtrlRomMemRule3Rule7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecCtrlRomMemRule3Rule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCtrlRomMemRule3Rule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCtrlRomMemRule3Rule7 {
    #[inline(always)]
    fn from(val: u8) -> SecCtrlRomMemRule3Rule7 {
        SecCtrlRomMemRule3Rule7::from_bits(val)
    }
}
impl From<SecCtrlRomMemRule3Rule7> for u8 {
    #[inline(always)]
    fn from(val: SecCtrlRomMemRule3Rule7) -> u8 {
        SecCtrlRomMemRule3Rule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask0Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask0Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask0Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask0Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask0Lock {
        SecGpioMask0Lock::from_bits(val)
    }
}
impl From<SecGpioMask0Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask0Lock) -> u8 {
        SecGpioMask0Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask1Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask1Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask1Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask1Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask1Lock {
        SecGpioMask1Lock::from_bits(val)
    }
}
impl From<SecGpioMask1Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask1Lock) -> u8 {
        SecGpioMask1Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecPintRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SecPintRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecPintRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecPintRule {
    #[inline(always)]
    fn from(val: u8) -> SecPintRule {
        SecPintRule::from_bits(val)
    }
}
impl From<SecPintRule> for u8 {
    #[inline(always)]
    fn from(val: SecPintRule) -> u8 {
        SecPintRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code access."]
    CODE = 0x0,
    #[doc = "Data access."]
    DATA = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "CPU0 Code."]
    VALUE_0 = 0x0,
    #[doc = "CPU0 System."]
    VALUE_1 = 0x01,
    #[doc = "CPU1 Data."]
    VALUE_2 = 0x02,
    #[doc = "CPU1 System."]
    VALUE_3 = 0x03,
    #[doc = "USB-HS Device."]
    VALUE_4 = 0x04,
    #[doc = "SDMA0."]
    VALUE_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SDIO."]
    VALUE_8 = 0x08,
    #[doc = "PowerQuad."]
    VALUE_9 = 0x09,
    #[doc = "HASH."]
    VALUE_10 = 0x0a,
    #[doc = "USB-FS Host."]
    VALUE_11 = 0x0b,
    #[doc = "SDMA1."]
    VALUE_12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access."]
    READ = 0x0,
    #[doc = "Write access."]
    WRITE = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramSect0Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SramSect0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramSect0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramSect0Rule {
    #[inline(always)]
    fn from(val: u8) -> SramSect0Rule {
        SramSect0Rule::from_bits(val)
    }
}
impl From<SramSect0Rule> for u8 {
    #[inline(always)]
    fn from(val: SramSect0Rule) -> u8 {
        SramSect0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramSect1Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SramSect1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramSect1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramSect1Rule {
    #[inline(always)]
    fn from(val: u8) -> SramSect1Rule {
        SramSect1Rule::from_bits(val)
    }
}
impl From<SramSect1Rule> for u8 {
    #[inline(always)]
    fn from(val: SramSect1Rule) -> u8 {
        SramSect1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramSect2Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SramSect2Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramSect2Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramSect2Rule {
    #[inline(always)]
    fn from(val: u8) -> SramSect2Rule {
        SramSect2Rule::from_bits(val)
    }
}
impl From<SramSect2Rule> for u8 {
    #[inline(always)]
    fn from(val: SramSect2Rule) -> u8 {
        SramSect2Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramSect3Rule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SramSect3Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramSect3Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramSect3Rule {
    #[inline(always)]
    fn from(val: u8) -> SramSect3Rule {
        SramSect3Rule::from_bits(val)
    }
}
impl From<SramSect3Rule> for u8 {
    #[inline(always)]
    fn from(val: SramSect3Rule) -> u8 {
        SramSect3Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysconRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SysconRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysconRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysconRule {
    #[inline(always)]
    fn from(val: u8) -> SysconRule {
        SysconRule::from_bits(val)
    }
}
impl From<SysconRule> for u8 {
    #[inline(always)]
    fn from(val: SysconRule) -> u8 {
        SysconRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysctrlRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SysctrlRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysctrlRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysctrlRule {
    #[inline(always)]
    fn from(val: u8) -> SysctrlRule {
        SysctrlRule::from_bits(val)
    }
}
impl From<SysctrlRule> for u8 {
    #[inline(always)]
    fn from(val: SysctrlRule) -> u8 {
        SysctrlRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbFsHostRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UsbFsHostRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbFsHostRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbFsHostRule {
    #[inline(always)]
    fn from(val: u8) -> UsbFsHostRule {
        UsbFsHostRule::from_bits(val)
    }
}
impl From<UsbFsHostRule> for u8 {
    #[inline(always)]
    fn from(val: UsbFsHostRule) -> u8 {
        UsbFsHostRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsDevRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UsbHsDevRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsDevRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsDevRule {
    #[inline(always)]
    fn from(val: u8) -> UsbHsDevRule {
        UsbHsDevRule::from_bits(val)
    }
}
impl From<UsbHsDevRule> for u8 {
    #[inline(always)]
    fn from(val: UsbHsDevRule) -> u8 {
        UsbHsDevRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsHostRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UsbHsHostRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsHostRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsHostRule {
    #[inline(always)]
    fn from(val: u8) -> UsbHsHostRule {
        UsbHsHostRule::from_bits(val)
    }
}
impl From<UsbHsHostRule> for u8 {
    #[inline(always)]
    fn from(val: UsbHsHostRule) -> u8 {
        UsbHsHostRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbhphyRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UsbhphyRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbhphyRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbhphyRule {
    #[inline(always)]
    fn from(val: u8) -> UsbhphyRule {
        UsbhphyRule::from_bits(val)
    }
}
impl From<UsbhphyRule> for u8 {
    #[inline(always)]
    fn from(val: UsbhphyRule) -> u8 {
        UsbhphyRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UtickRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRule {
    #[inline(always)]
    fn from(val: u8) -> UtickRule {
        UtickRule::from_bits(val)
    }
}
impl From<UtickRule> for u8 {
    #[inline(always)]
    fn from(val: UtickRule) -> u8 {
        UtickRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WwdtRule {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl WwdtRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WwdtRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WwdtRule {
    #[inline(always)]
    fn from(val: u8) -> WwdtRule {
        WwdtRule::from_bits(val)
    }
}
impl From<WwdtRule> for u8 {
    #[inline(always)]
    fn from(val: WwdtRule) -> u8 {
        WwdtRule::to_bits(val)
    }
}
