#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AntiPoleSecLevel {
    #[doc = "Secure and privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Secure and non-privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Non-secure and privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Non-secure and non-privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl AntiPoleSecLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AntiPoleSecLevel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AntiPoleSecLevel {
    #[inline(always)]
    fn from(val: u8) -> AntiPoleSecLevel {
        AntiPoleSecLevel::from_bits(val)
    }
}
impl From<AntiPoleSecLevel> for u8 {
    #[inline(always)]
    fn from(val: AntiPoleSecLevel) -> u8 {
        AntiPoleSecLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufEnroll {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufEnroll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufEnroll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufEnroll {
    #[inline(always)]
    fn from(val: u8) -> DisPufEnroll {
        DisPufEnroll::from_bits(val)
    }
}
impl From<DisPufEnroll> for u8 {
    #[inline(always)]
    fn from(val: DisPufEnroll) -> u8 {
        DisPufEnroll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGenRandomNumber {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufGenRandomNumber {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGenRandomNumber {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGenRandomNumber {
    #[inline(always)]
    fn from(val: u8) -> DisPufGenRandomNumber {
        DisPufGenRandomNumber::from_bits(val)
    }
}
impl From<DisPufGenRandomNumber> for u8 {
    #[inline(always)]
    fn from(val: DisPufGenRandomNumber) -> u8 {
        DisPufGenRandomNumber::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGenWrapKey {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufGenWrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGenWrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGenWrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufGenWrapKey {
        DisPufGenWrapKey::from_bits(val)
    }
}
impl From<DisPufGenWrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufGenWrapKey) -> u8 {
        DisPufGenWrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGetKey {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufGetKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGetKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGetKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufGetKey {
        DisPufGetKey::from_bits(val)
    }
}
impl From<DisPufGetKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufGetKey) -> u8 {
        DisPufGetKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufStart {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufStart {
    #[inline(always)]
    fn from(val: u8) -> DisPufStart {
        DisPufStart::from_bits(val)
    }
}
impl From<DisPufStart> for u8 {
    #[inline(always)]
    fn from(val: DisPufStart) -> u8 {
        DisPufStart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufStop {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufStop {
    #[inline(always)]
    fn from(val: u8) -> DisPufStop {
        DisPufStop::from_bits(val)
    }
}
impl From<DisPufStop> for u8 {
    #[inline(always)]
    fn from(val: DisPufStop) -> u8 {
        DisPufStop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufTest {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufTest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufTest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufTest {
    #[inline(always)]
    fn from(val: u8) -> DisPufTest {
        DisPufTest::from_bits(val)
    }
}
impl From<DisPufTest> for u8 {
    #[inline(always)]
    fn from(val: DisPufTest) -> u8 {
        DisPufTest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufUnwrapKey {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufUnwrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufUnwrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufUnwrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufUnwrapKey {
        DisPufUnwrapKey::from_bits(val)
    }
}
impl From<DisPufUnwrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufUnwrapKey) -> u8 {
        DisPufUnwrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufWrapKey {
    #[doc = "Command enabled"]
    ENABLE = 0x0,
    #[doc = "Command disabled"]
    DISABLE = 0x01,
}
impl DisPufWrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufWrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufWrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufWrapKey {
        DisPufWrapKey::from_bits(val)
    }
}
impl From<DisPufWrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufWrapKey) -> u8 {
        DisPufWrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecLevel {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl SecLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecLevel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecLevel {
    #[inline(always)]
    fn from(val: u8) -> SecLevel {
        SecLevel::from_bits(val)
    }
}
impl From<SecLevel> for u8 {
    #[inline(always)]
    fn from(val: SecLevel) -> u8 {
        SecLevel::to_bits(val)
    }
}
