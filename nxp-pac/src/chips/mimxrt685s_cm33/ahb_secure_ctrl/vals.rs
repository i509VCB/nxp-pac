#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm33LockRegLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cm33LockRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm33LockRegLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm33LockRegLock {
    #[inline(always)]
    fn from(val: u8) -> Cm33LockRegLock {
        Cm33LockRegLock::from_bits(val)
    }
}
impl From<Cm33LockRegLock> for u8 {
    #[inline(always)]
    fn from(val: Cm33LockRegLock) -> u8 {
        Cm33LockRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule0 {
        Flexspi0Region0RuleRule0::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule0) -> u8 {
        Flexspi0Region0RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule1 {
        Flexspi0Region0RuleRule1::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule1) -> u8 {
        Flexspi0Region0RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule2 {
        Flexspi0Region0RuleRule2::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule2) -> u8 {
        Flexspi0Region0RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule3 {
        Flexspi0Region0RuleRule3::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule3) -> u8 {
        Flexspi0Region0RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule4 {
        Flexspi0Region0RuleRule4::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule4) -> u8 {
        Flexspi0Region0RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule5 {
        Flexspi0Region0RuleRule5::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule5) -> u8 {
        Flexspi0Region0RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule6 {
        Flexspi0Region0RuleRule6::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule6) -> u8 {
        Flexspi0Region0RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0RuleRule7 {
        Flexspi0Region0RuleRule7::from_bits(val)
    }
}
impl From<Flexspi0Region0RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0RuleRule7) -> u8 {
        Flexspi0Region0RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region1Rule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region1Rule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region1Rule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region1Rule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region1Rule0Rule0 {
        Flexspi0Region1Rule0Rule0::from_bits(val)
    }
}
impl From<Flexspi0Region1Rule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region1Rule0Rule0) -> u8 {
        Flexspi0Region1Rule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region1Rule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region1Rule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region1Rule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region1Rule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region1Rule0Rule1 {
        Flexspi0Region1Rule0Rule1::from_bits(val)
    }
}
impl From<Flexspi0Region1Rule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region1Rule0Rule1) -> u8 {
        Flexspi0Region1Rule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region1Rule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region1Rule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region1Rule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region1Rule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region1Rule0Rule2 {
        Flexspi0Region1Rule0Rule2::from_bits(val)
    }
}
impl From<Flexspi0Region1Rule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region1Rule0Rule2) -> u8 {
        Flexspi0Region1Rule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region1Rule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region1Rule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region1Rule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region1Rule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region1Rule0Rule3 {
        Flexspi0Region1Rule0Rule3::from_bits(val)
    }
}
impl From<Flexspi0Region1Rule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region1Rule0Rule3) -> u8 {
        Flexspi0Region1Rule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region2Rule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region2Rule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region2Rule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region2Rule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region2Rule0Rule0 {
        Flexspi0Region2Rule0Rule0::from_bits(val)
    }
}
impl From<Flexspi0Region2Rule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region2Rule0Rule0) -> u8 {
        Flexspi0Region2Rule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region2Rule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region2Rule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region2Rule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region2Rule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region2Rule0Rule1 {
        Flexspi0Region2Rule0Rule1::from_bits(val)
    }
}
impl From<Flexspi0Region2Rule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region2Rule0Rule1) -> u8 {
        Flexspi0Region2Rule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region2Rule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region2Rule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region2Rule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region2Rule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region2Rule0Rule2 {
        Flexspi0Region2Rule0Rule2::from_bits(val)
    }
}
impl From<Flexspi0Region2Rule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region2Rule0Rule2) -> u8 {
        Flexspi0Region2Rule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region2Rule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region2Rule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region2Rule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region2Rule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region2Rule0Rule3 {
        Flexspi0Region2Rule0Rule3::from_bits(val)
    }
}
impl From<Flexspi0Region2Rule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region2Rule0Rule3) -> u8 {
        Flexspi0Region2Rule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region3Rule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region3Rule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region3Rule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region3Rule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region3Rule0Rule0 {
        Flexspi0Region3Rule0Rule0::from_bits(val)
    }
}
impl From<Flexspi0Region3Rule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region3Rule0Rule0) -> u8 {
        Flexspi0Region3Rule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region3Rule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region3Rule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region3Rule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region3Rule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region3Rule0Rule1 {
        Flexspi0Region3Rule0Rule1::from_bits(val)
    }
}
impl From<Flexspi0Region3Rule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region3Rule0Rule1) -> u8 {
        Flexspi0Region3Rule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region3Rule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region3Rule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region3Rule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region3Rule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region3Rule0Rule2 {
        Flexspi0Region3Rule0Rule2::from_bits(val)
    }
}
impl From<Flexspi0Region3Rule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region3Rule0Rule2) -> u8 {
        Flexspi0Region3Rule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region3Rule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region3Rule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region3Rule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region3Rule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region3Rule0Rule3 {
        Flexspi0Region3Rule0Rule3::from_bits(val)
    }
}
impl From<Flexspi0Region3Rule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region3Rule0Rule3) -> u8 {
        Flexspi0Region3Rule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region4Rule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region4Rule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region4Rule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region4Rule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region4Rule0Rule0 {
        Flexspi0Region4Rule0Rule0::from_bits(val)
    }
}
impl From<Flexspi0Region4Rule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region4Rule0Rule0) -> u8 {
        Flexspi0Region4Rule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region4Rule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region4Rule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region4Rule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region4Rule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region4Rule0Rule1 {
        Flexspi0Region4Rule0Rule1::from_bits(val)
    }
}
impl From<Flexspi0Region4Rule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region4Rule0Rule1) -> u8 {
        Flexspi0Region4Rule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region4Rule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region4Rule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region4Rule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region4Rule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region4Rule0Rule2 {
        Flexspi0Region4Rule0Rule2::from_bits(val)
    }
}
impl From<Flexspi0Region4Rule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region4Rule0Rule2) -> u8 {
        Flexspi0Region4Rule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region4Rule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region4Rule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region4Rule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region4Rule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region4Rule0Rule3 {
        Flexspi0Region4Rule0Rule3::from_bits(val)
    }
}
impl From<Flexspi0Region4Rule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region4Rule0Rule3) -> u8 {
        Flexspi0Region4Rule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> LockNsMpu {
        LockNsMpu::from_bits(val)
    }
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: LockNsMpu) -> u8 {
        LockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> LockNsVtor {
        LockNsVtor::from_bits(val)
    }
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: LockNsVtor) -> u8 {
        LockNsVtor::to_bits(val)
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
pub enum LockSVtor {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtor {
    #[inline(always)]
    fn from(val: u8) -> LockSVtor {
        LockSVtor::from_bits(val)
    }
}
impl From<LockSVtor> for u8 {
    #[inline(always)]
    fn from(val: LockSVtor) -> u8 {
        LockSVtor::to_bits(val)
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
pub enum MasterSecLevelAntiPolDma0Sec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolDma0Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolDma0Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolDma0Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolDma0Sec {
        MasterSecLevelAntiPolDma0Sec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolDma0Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolDma0Sec) -> u8 {
        MasterSecLevelAntiPolDma0Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPolDma1Sec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolDma1Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolDma1Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolDma1Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolDma1Sec {
        MasterSecLevelAntiPolDma1Sec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolDma1Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolDma1Sec) -> u8 {
        MasterSecLevelAntiPolDma1Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPolDspSec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolDspSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolDspSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolDspSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolDspSec {
        MasterSecLevelAntiPolDspSec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolDspSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolDspSec) -> u8 {
        MasterSecLevelAntiPolDspSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPolPowerquadSec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolPowerquadSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolPowerquadSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolPowerquadSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolPowerquadSec {
        MasterSecLevelAntiPolPowerquadSec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolPowerquadSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolPowerquadSec) -> u8 {
        MasterSecLevelAntiPolPowerquadSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPolSdio0Sec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolSdio0Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolSdio0Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolSdio0Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolSdio0Sec {
        MasterSecLevelAntiPolSdio0Sec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolSdio0Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolSdio0Sec) -> u8 {
        MasterSecLevelAntiPolSdio0Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPolSdio1Sec {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MasterSecLevelAntiPolSdio1Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPolSdio1Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPolSdio1Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPolSdio1Sec {
        MasterSecLevelAntiPolSdio1Sec::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPolSdio1Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPolSdio1Sec) -> u8 {
        MasterSecLevelAntiPolSdio1Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntiPoleLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelAntiPoleLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntiPoleLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntiPoleLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntiPoleLock {
        MasterSecLevelAntiPoleLock::from_bits(val)
    }
}
impl From<MasterSecLevelAntiPoleLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntiPoleLock) -> u8 {
        MasterSecLevelAntiPoleLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma0Sec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelDma0Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma0Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma0Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma0Sec {
        MasterSecLevelDma0Sec::from_bits(val)
    }
}
impl From<MasterSecLevelDma0Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma0Sec) -> u8 {
        MasterSecLevelDma0Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma1Sec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelDma1Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma1Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma1Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma1Sec {
        MasterSecLevelDma1Sec::from_bits(val)
    }
}
impl From<MasterSecLevelDma1Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma1Sec) -> u8 {
        MasterSecLevelDma1Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDspSec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelDspSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDspSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDspSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDspSec {
        MasterSecLevelDspSec::from_bits(val)
    }
}
impl From<MasterSecLevelDspSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDspSec) -> u8 {
        MasterSecLevelDspSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
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
pub enum MasterSecLevelPowerquadSec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelPowerquadSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPowerquadSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPowerquadSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPowerquadSec {
        MasterSecLevelPowerquadSec::from_bits(val)
    }
}
impl From<MasterSecLevelPowerquadSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPowerquadSec) -> u8 {
        MasterSecLevelPowerquadSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSdio0Sec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelSdio0Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSdio0Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSdio0Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSdio0Sec {
        MasterSecLevelSdio0Sec::from_bits(val)
    }
}
impl From<MasterSecLevelSdio0Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSdio0Sec) -> u8 {
        MasterSecLevelSdio0Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSdio1Sec {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MasterSecLevelSdio1Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSdio1Sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSdio1Sec {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSdio1Sec {
        MasterSecLevelSdio1Sec::from_bits(val)
    }
}
impl From<MasterSecLevelSdio1Sec> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSdio1Sec) -> u8 {
        MasterSecLevelSdio1Sec::to_bits(val)
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
    #[doc = "Violation assert secure_violation_irq."]
    DISABLE = 0x01,
    #[doc = "Violation causes abort."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "IDAU is disabled."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "Violation assert secure_violation_irq."]
    DISABLE = 0x01,
    #[doc = "Violation causes abort."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "Restrictive mode."]
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
    #[doc = "IDAU is disabled."]
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
    #[doc = "Restrictive mode."]
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
pub enum Ram00RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule0 {
        Ram00RuleRule0::from_bits(val)
    }
}
impl From<Ram00RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule0) -> u8 {
        Ram00RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule1 {
        Ram00RuleRule1::from_bits(val)
    }
}
impl From<Ram00RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule1) -> u8 {
        Ram00RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule2 {
        Ram00RuleRule2::from_bits(val)
    }
}
impl From<Ram00RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule2) -> u8 {
        Ram00RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule3 {
        Ram00RuleRule3::from_bits(val)
    }
}
impl From<Ram00RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule3) -> u8 {
        Ram00RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule4 {
        Ram00RuleRule4::from_bits(val)
    }
}
impl From<Ram00RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule4) -> u8 {
        Ram00RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule5 {
        Ram00RuleRule5::from_bits(val)
    }
}
impl From<Ram00RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule5) -> u8 {
        Ram00RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule6 {
        Ram00RuleRule6::from_bits(val)
    }
}
impl From<Ram00RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule6) -> u8 {
        Ram00RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram00RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram00RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram00RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram00RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram00RuleRule7 {
        Ram00RuleRule7::from_bits(val)
    }
}
impl From<Ram00RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram00RuleRule7) -> u8 {
        Ram00RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule0 {
        Ram01RuleRule0::from_bits(val)
    }
}
impl From<Ram01RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule0) -> u8 {
        Ram01RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule1 {
        Ram01RuleRule1::from_bits(val)
    }
}
impl From<Ram01RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule1) -> u8 {
        Ram01RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule2 {
        Ram01RuleRule2::from_bits(val)
    }
}
impl From<Ram01RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule2) -> u8 {
        Ram01RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule3 {
        Ram01RuleRule3::from_bits(val)
    }
}
impl From<Ram01RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule3) -> u8 {
        Ram01RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule4 {
        Ram01RuleRule4::from_bits(val)
    }
}
impl From<Ram01RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule4) -> u8 {
        Ram01RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule5 {
        Ram01RuleRule5::from_bits(val)
    }
}
impl From<Ram01RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule5) -> u8 {
        Ram01RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule6 {
        Ram01RuleRule6::from_bits(val)
    }
}
impl From<Ram01RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule6) -> u8 {
        Ram01RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram01RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram01RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram01RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram01RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram01RuleRule7 {
        Ram01RuleRule7::from_bits(val)
    }
}
impl From<Ram01RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram01RuleRule7) -> u8 {
        Ram01RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule0 {
        Ram02RuleRule0::from_bits(val)
    }
}
impl From<Ram02RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule0) -> u8 {
        Ram02RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule1 {
        Ram02RuleRule1::from_bits(val)
    }
}
impl From<Ram02RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule1) -> u8 {
        Ram02RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule2 {
        Ram02RuleRule2::from_bits(val)
    }
}
impl From<Ram02RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule2) -> u8 {
        Ram02RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule3 {
        Ram02RuleRule3::from_bits(val)
    }
}
impl From<Ram02RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule3) -> u8 {
        Ram02RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule4 {
        Ram02RuleRule4::from_bits(val)
    }
}
impl From<Ram02RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule4) -> u8 {
        Ram02RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule5 {
        Ram02RuleRule5::from_bits(val)
    }
}
impl From<Ram02RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule5) -> u8 {
        Ram02RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule6 {
        Ram02RuleRule6::from_bits(val)
    }
}
impl From<Ram02RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule6) -> u8 {
        Ram02RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram02RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram02RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram02RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram02RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram02RuleRule7 {
        Ram02RuleRule7::from_bits(val)
    }
}
impl From<Ram02RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram02RuleRule7) -> u8 {
        Ram02RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule0 {
        Ram03RuleRule0::from_bits(val)
    }
}
impl From<Ram03RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule0) -> u8 {
        Ram03RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule1 {
        Ram03RuleRule1::from_bits(val)
    }
}
impl From<Ram03RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule1) -> u8 {
        Ram03RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule2 {
        Ram03RuleRule2::from_bits(val)
    }
}
impl From<Ram03RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule2) -> u8 {
        Ram03RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule3 {
        Ram03RuleRule3::from_bits(val)
    }
}
impl From<Ram03RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule3) -> u8 {
        Ram03RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule4 {
        Ram03RuleRule4::from_bits(val)
    }
}
impl From<Ram03RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule4) -> u8 {
        Ram03RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule5 {
        Ram03RuleRule5::from_bits(val)
    }
}
impl From<Ram03RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule5) -> u8 {
        Ram03RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule6 {
        Ram03RuleRule6::from_bits(val)
    }
}
impl From<Ram03RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule6) -> u8 {
        Ram03RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram03RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram03RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram03RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram03RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram03RuleRule7 {
        Ram03RuleRule7::from_bits(val)
    }
}
impl From<Ram03RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram03RuleRule7) -> u8 {
        Ram03RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule0 {
        Ram04RuleRule0::from_bits(val)
    }
}
impl From<Ram04RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule0) -> u8 {
        Ram04RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule1 {
        Ram04RuleRule1::from_bits(val)
    }
}
impl From<Ram04RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule1) -> u8 {
        Ram04RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule2 {
        Ram04RuleRule2::from_bits(val)
    }
}
impl From<Ram04RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule2) -> u8 {
        Ram04RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule3 {
        Ram04RuleRule3::from_bits(val)
    }
}
impl From<Ram04RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule3) -> u8 {
        Ram04RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule4 {
        Ram04RuleRule4::from_bits(val)
    }
}
impl From<Ram04RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule4) -> u8 {
        Ram04RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule5 {
        Ram04RuleRule5::from_bits(val)
    }
}
impl From<Ram04RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule5) -> u8 {
        Ram04RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule6 {
        Ram04RuleRule6::from_bits(val)
    }
}
impl From<Ram04RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule6) -> u8 {
        Ram04RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram04RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram04RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram04RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram04RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram04RuleRule7 {
        Ram04RuleRule7::from_bits(val)
    }
}
impl From<Ram04RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram04RuleRule7) -> u8 {
        Ram04RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule0 {
        Ram05RuleRule0::from_bits(val)
    }
}
impl From<Ram05RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule0) -> u8 {
        Ram05RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule1 {
        Ram05RuleRule1::from_bits(val)
    }
}
impl From<Ram05RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule1) -> u8 {
        Ram05RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule2 {
        Ram05RuleRule2::from_bits(val)
    }
}
impl From<Ram05RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule2) -> u8 {
        Ram05RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule3 {
        Ram05RuleRule3::from_bits(val)
    }
}
impl From<Ram05RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule3) -> u8 {
        Ram05RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule4 {
        Ram05RuleRule4::from_bits(val)
    }
}
impl From<Ram05RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule4) -> u8 {
        Ram05RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule5 {
        Ram05RuleRule5::from_bits(val)
    }
}
impl From<Ram05RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule5) -> u8 {
        Ram05RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule6 {
        Ram05RuleRule6::from_bits(val)
    }
}
impl From<Ram05RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule6) -> u8 {
        Ram05RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram05RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram05RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram05RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram05RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram05RuleRule7 {
        Ram05RuleRule7::from_bits(val)
    }
}
impl From<Ram05RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram05RuleRule7) -> u8 {
        Ram05RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule0 {
        Ram06RuleRule0::from_bits(val)
    }
}
impl From<Ram06RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule0) -> u8 {
        Ram06RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule1 {
        Ram06RuleRule1::from_bits(val)
    }
}
impl From<Ram06RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule1) -> u8 {
        Ram06RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule2 {
        Ram06RuleRule2::from_bits(val)
    }
}
impl From<Ram06RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule2) -> u8 {
        Ram06RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule3 {
        Ram06RuleRule3::from_bits(val)
    }
}
impl From<Ram06RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule3) -> u8 {
        Ram06RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule4 {
        Ram06RuleRule4::from_bits(val)
    }
}
impl From<Ram06RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule4) -> u8 {
        Ram06RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule5 {
        Ram06RuleRule5::from_bits(val)
    }
}
impl From<Ram06RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule5) -> u8 {
        Ram06RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule6 {
        Ram06RuleRule6::from_bits(val)
    }
}
impl From<Ram06RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule6) -> u8 {
        Ram06RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram06RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram06RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram06RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram06RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram06RuleRule7 {
        Ram06RuleRule7::from_bits(val)
    }
}
impl From<Ram06RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram06RuleRule7) -> u8 {
        Ram06RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule0 {
        Ram07RuleRule0::from_bits(val)
    }
}
impl From<Ram07RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule0) -> u8 {
        Ram07RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule1 {
        Ram07RuleRule1::from_bits(val)
    }
}
impl From<Ram07RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule1) -> u8 {
        Ram07RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule2 {
        Ram07RuleRule2::from_bits(val)
    }
}
impl From<Ram07RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule2) -> u8 {
        Ram07RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule3 {
        Ram07RuleRule3::from_bits(val)
    }
}
impl From<Ram07RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule3) -> u8 {
        Ram07RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule4 {
        Ram07RuleRule4::from_bits(val)
    }
}
impl From<Ram07RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule4) -> u8 {
        Ram07RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule5 {
        Ram07RuleRule5::from_bits(val)
    }
}
impl From<Ram07RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule5) -> u8 {
        Ram07RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule6 {
        Ram07RuleRule6::from_bits(val)
    }
}
impl From<Ram07RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule6) -> u8 {
        Ram07RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram07RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram07RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram07RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram07RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram07RuleRule7 {
        Ram07RuleRule7::from_bits(val)
    }
}
impl From<Ram07RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram07RuleRule7) -> u8 {
        Ram07RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule0 {
        Ram08RuleRule0::from_bits(val)
    }
}
impl From<Ram08RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule0) -> u8 {
        Ram08RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule1 {
        Ram08RuleRule1::from_bits(val)
    }
}
impl From<Ram08RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule1) -> u8 {
        Ram08RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule2 {
        Ram08RuleRule2::from_bits(val)
    }
}
impl From<Ram08RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule2) -> u8 {
        Ram08RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule3 {
        Ram08RuleRule3::from_bits(val)
    }
}
impl From<Ram08RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule3) -> u8 {
        Ram08RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule4 {
        Ram08RuleRule4::from_bits(val)
    }
}
impl From<Ram08RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule4) -> u8 {
        Ram08RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule5 {
        Ram08RuleRule5::from_bits(val)
    }
}
impl From<Ram08RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule5) -> u8 {
        Ram08RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule6 {
        Ram08RuleRule6::from_bits(val)
    }
}
impl From<Ram08RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule6) -> u8 {
        Ram08RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram08RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram08RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram08RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram08RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram08RuleRule7 {
        Ram08RuleRule7::from_bits(val)
    }
}
impl From<Ram08RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram08RuleRule7) -> u8 {
        Ram08RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule0 {
        Ram09RuleRule0::from_bits(val)
    }
}
impl From<Ram09RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule0) -> u8 {
        Ram09RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule1 {
        Ram09RuleRule1::from_bits(val)
    }
}
impl From<Ram09RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule1) -> u8 {
        Ram09RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule2 {
        Ram09RuleRule2::from_bits(val)
    }
}
impl From<Ram09RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule2) -> u8 {
        Ram09RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule3 {
        Ram09RuleRule3::from_bits(val)
    }
}
impl From<Ram09RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule3) -> u8 {
        Ram09RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule4 {
        Ram09RuleRule4::from_bits(val)
    }
}
impl From<Ram09RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule4) -> u8 {
        Ram09RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule5 {
        Ram09RuleRule5::from_bits(val)
    }
}
impl From<Ram09RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule5) -> u8 {
        Ram09RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule6 {
        Ram09RuleRule6::from_bits(val)
    }
}
impl From<Ram09RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule6) -> u8 {
        Ram09RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram09RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram09RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram09RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram09RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram09RuleRule7 {
        Ram09RuleRule7::from_bits(val)
    }
}
impl From<Ram09RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram09RuleRule7) -> u8 {
        Ram09RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule0 {
        Ram10RuleRule0::from_bits(val)
    }
}
impl From<Ram10RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule0) -> u8 {
        Ram10RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule1 {
        Ram10RuleRule1::from_bits(val)
    }
}
impl From<Ram10RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule1) -> u8 {
        Ram10RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule2 {
        Ram10RuleRule2::from_bits(val)
    }
}
impl From<Ram10RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule2) -> u8 {
        Ram10RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule3 {
        Ram10RuleRule3::from_bits(val)
    }
}
impl From<Ram10RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule3) -> u8 {
        Ram10RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule4 {
        Ram10RuleRule4::from_bits(val)
    }
}
impl From<Ram10RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule4) -> u8 {
        Ram10RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule5 {
        Ram10RuleRule5::from_bits(val)
    }
}
impl From<Ram10RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule5) -> u8 {
        Ram10RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule6 {
        Ram10RuleRule6::from_bits(val)
    }
}
impl From<Ram10RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule6) -> u8 {
        Ram10RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram10RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram10RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram10RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram10RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram10RuleRule7 {
        Ram10RuleRule7::from_bits(val)
    }
}
impl From<Ram10RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram10RuleRule7) -> u8 {
        Ram10RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule0 {
        Ram11RuleRule0::from_bits(val)
    }
}
impl From<Ram11RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule0) -> u8 {
        Ram11RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule1 {
        Ram11RuleRule1::from_bits(val)
    }
}
impl From<Ram11RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule1) -> u8 {
        Ram11RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule2 {
        Ram11RuleRule2::from_bits(val)
    }
}
impl From<Ram11RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule2) -> u8 {
        Ram11RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule3 {
        Ram11RuleRule3::from_bits(val)
    }
}
impl From<Ram11RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule3) -> u8 {
        Ram11RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule4 {
        Ram11RuleRule4::from_bits(val)
    }
}
impl From<Ram11RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule4) -> u8 {
        Ram11RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule5 {
        Ram11RuleRule5::from_bits(val)
    }
}
impl From<Ram11RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule5) -> u8 {
        Ram11RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule6 {
        Ram11RuleRule6::from_bits(val)
    }
}
impl From<Ram11RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule6) -> u8 {
        Ram11RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram11RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram11RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram11RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram11RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram11RuleRule7 {
        Ram11RuleRule7::from_bits(val)
    }
}
impl From<Ram11RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram11RuleRule7) -> u8 {
        Ram11RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule0 {
        Ram12RuleRule0::from_bits(val)
    }
}
impl From<Ram12RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule0) -> u8 {
        Ram12RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule1 {
        Ram12RuleRule1::from_bits(val)
    }
}
impl From<Ram12RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule1) -> u8 {
        Ram12RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule2 {
        Ram12RuleRule2::from_bits(val)
    }
}
impl From<Ram12RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule2) -> u8 {
        Ram12RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule3 {
        Ram12RuleRule3::from_bits(val)
    }
}
impl From<Ram12RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule3) -> u8 {
        Ram12RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule4 {
        Ram12RuleRule4::from_bits(val)
    }
}
impl From<Ram12RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule4) -> u8 {
        Ram12RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule5 {
        Ram12RuleRule5::from_bits(val)
    }
}
impl From<Ram12RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule5) -> u8 {
        Ram12RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule6 {
        Ram12RuleRule6::from_bits(val)
    }
}
impl From<Ram12RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule6) -> u8 {
        Ram12RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram12RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram12RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram12RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram12RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram12RuleRule7 {
        Ram12RuleRule7::from_bits(val)
    }
}
impl From<Ram12RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram12RuleRule7) -> u8 {
        Ram12RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule0 {
        Ram13RuleRule0::from_bits(val)
    }
}
impl From<Ram13RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule0) -> u8 {
        Ram13RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule1 {
        Ram13RuleRule1::from_bits(val)
    }
}
impl From<Ram13RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule1) -> u8 {
        Ram13RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule2 {
        Ram13RuleRule2::from_bits(val)
    }
}
impl From<Ram13RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule2) -> u8 {
        Ram13RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule3 {
        Ram13RuleRule3::from_bits(val)
    }
}
impl From<Ram13RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule3) -> u8 {
        Ram13RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule4 {
        Ram13RuleRule4::from_bits(val)
    }
}
impl From<Ram13RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule4) -> u8 {
        Ram13RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule5 {
        Ram13RuleRule5::from_bits(val)
    }
}
impl From<Ram13RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule5) -> u8 {
        Ram13RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule6 {
        Ram13RuleRule6::from_bits(val)
    }
}
impl From<Ram13RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule6) -> u8 {
        Ram13RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram13RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram13RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram13RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram13RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram13RuleRule7 {
        Ram13RuleRule7::from_bits(val)
    }
}
impl From<Ram13RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram13RuleRule7) -> u8 {
        Ram13RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule0 {
        Ram14RuleRule0::from_bits(val)
    }
}
impl From<Ram14RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule0) -> u8 {
        Ram14RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule1 {
        Ram14RuleRule1::from_bits(val)
    }
}
impl From<Ram14RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule1) -> u8 {
        Ram14RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule2 {
        Ram14RuleRule2::from_bits(val)
    }
}
impl From<Ram14RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule2) -> u8 {
        Ram14RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule3 {
        Ram14RuleRule3::from_bits(val)
    }
}
impl From<Ram14RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule3) -> u8 {
        Ram14RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule4 {
        Ram14RuleRule4::from_bits(val)
    }
}
impl From<Ram14RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule4) -> u8 {
        Ram14RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule5 {
        Ram14RuleRule5::from_bits(val)
    }
}
impl From<Ram14RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule5) -> u8 {
        Ram14RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule6 {
        Ram14RuleRule6::from_bits(val)
    }
}
impl From<Ram14RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule6) -> u8 {
        Ram14RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram14RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram14RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram14RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram14RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram14RuleRule7 {
        Ram14RuleRule7::from_bits(val)
    }
}
impl From<Ram14RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram14RuleRule7) -> u8 {
        Ram14RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule0 {
        Ram15RuleRule0::from_bits(val)
    }
}
impl From<Ram15RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule0) -> u8 {
        Ram15RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule1 {
        Ram15RuleRule1::from_bits(val)
    }
}
impl From<Ram15RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule1) -> u8 {
        Ram15RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule2 {
        Ram15RuleRule2::from_bits(val)
    }
}
impl From<Ram15RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule2) -> u8 {
        Ram15RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule3 {
        Ram15RuleRule3::from_bits(val)
    }
}
impl From<Ram15RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule3) -> u8 {
        Ram15RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule4 {
        Ram15RuleRule4::from_bits(val)
    }
}
impl From<Ram15RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule4) -> u8 {
        Ram15RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule5 {
        Ram15RuleRule5::from_bits(val)
    }
}
impl From<Ram15RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule5) -> u8 {
        Ram15RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule6 {
        Ram15RuleRule6::from_bits(val)
    }
}
impl From<Ram15RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule6) -> u8 {
        Ram15RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram15RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram15RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram15RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram15RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram15RuleRule7 {
        Ram15RuleRule7::from_bits(val)
    }
}
impl From<Ram15RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram15RuleRule7) -> u8 {
        Ram15RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule0 {
        Ram16RuleRule0::from_bits(val)
    }
}
impl From<Ram16RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule0) -> u8 {
        Ram16RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule1 {
        Ram16RuleRule1::from_bits(val)
    }
}
impl From<Ram16RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule1) -> u8 {
        Ram16RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule2 {
        Ram16RuleRule2::from_bits(val)
    }
}
impl From<Ram16RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule2) -> u8 {
        Ram16RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule3 {
        Ram16RuleRule3::from_bits(val)
    }
}
impl From<Ram16RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule3) -> u8 {
        Ram16RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule4 {
        Ram16RuleRule4::from_bits(val)
    }
}
impl From<Ram16RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule4) -> u8 {
        Ram16RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule5 {
        Ram16RuleRule5::from_bits(val)
    }
}
impl From<Ram16RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule5) -> u8 {
        Ram16RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule6 {
        Ram16RuleRule6::from_bits(val)
    }
}
impl From<Ram16RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule6) -> u8 {
        Ram16RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram16RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram16RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram16RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram16RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram16RuleRule7 {
        Ram16RuleRule7::from_bits(val)
    }
}
impl From<Ram16RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram16RuleRule7) -> u8 {
        Ram16RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule0 {
        Ram17RuleRule0::from_bits(val)
    }
}
impl From<Ram17RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule0) -> u8 {
        Ram17RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule1 {
        Ram17RuleRule1::from_bits(val)
    }
}
impl From<Ram17RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule1) -> u8 {
        Ram17RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule2 {
        Ram17RuleRule2::from_bits(val)
    }
}
impl From<Ram17RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule2) -> u8 {
        Ram17RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule3 {
        Ram17RuleRule3::from_bits(val)
    }
}
impl From<Ram17RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule3) -> u8 {
        Ram17RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule4 {
        Ram17RuleRule4::from_bits(val)
    }
}
impl From<Ram17RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule4) -> u8 {
        Ram17RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule5 {
        Ram17RuleRule5::from_bits(val)
    }
}
impl From<Ram17RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule5) -> u8 {
        Ram17RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule6 {
        Ram17RuleRule6::from_bits(val)
    }
}
impl From<Ram17RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule6) -> u8 {
        Ram17RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram17RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram17RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram17RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram17RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram17RuleRule7 {
        Ram17RuleRule7::from_bits(val)
    }
}
impl From<Ram17RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram17RuleRule7) -> u8 {
        Ram17RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule0 {
        Ram18RuleRule0::from_bits(val)
    }
}
impl From<Ram18RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule0) -> u8 {
        Ram18RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule1 {
        Ram18RuleRule1::from_bits(val)
    }
}
impl From<Ram18RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule1) -> u8 {
        Ram18RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule2 {
        Ram18RuleRule2::from_bits(val)
    }
}
impl From<Ram18RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule2) -> u8 {
        Ram18RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule3 {
        Ram18RuleRule3::from_bits(val)
    }
}
impl From<Ram18RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule3) -> u8 {
        Ram18RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule4 {
        Ram18RuleRule4::from_bits(val)
    }
}
impl From<Ram18RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule4) -> u8 {
        Ram18RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule5 {
        Ram18RuleRule5::from_bits(val)
    }
}
impl From<Ram18RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule5) -> u8 {
        Ram18RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule6 {
        Ram18RuleRule6::from_bits(val)
    }
}
impl From<Ram18RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule6) -> u8 {
        Ram18RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram18RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram18RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram18RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram18RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram18RuleRule7 {
        Ram18RuleRule7::from_bits(val)
    }
}
impl From<Ram18RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram18RuleRule7) -> u8 {
        Ram18RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule0 {
        Ram19RuleRule0::from_bits(val)
    }
}
impl From<Ram19RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule0) -> u8 {
        Ram19RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule1 {
        Ram19RuleRule1::from_bits(val)
    }
}
impl From<Ram19RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule1) -> u8 {
        Ram19RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule2 {
        Ram19RuleRule2::from_bits(val)
    }
}
impl From<Ram19RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule2) -> u8 {
        Ram19RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule3 {
        Ram19RuleRule3::from_bits(val)
    }
}
impl From<Ram19RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule3) -> u8 {
        Ram19RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule4 {
        Ram19RuleRule4::from_bits(val)
    }
}
impl From<Ram19RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule4) -> u8 {
        Ram19RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule5 {
        Ram19RuleRule5::from_bits(val)
    }
}
impl From<Ram19RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule5) -> u8 {
        Ram19RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule6 {
        Ram19RuleRule6::from_bits(val)
    }
}
impl From<Ram19RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule6) -> u8 {
        Ram19RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram19RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram19RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram19RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram19RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram19RuleRule7 {
        Ram19RuleRule7::from_bits(val)
    }
}
impl From<Ram19RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram19RuleRule7) -> u8 {
        Ram19RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule0 {
        Ram20RuleRule0::from_bits(val)
    }
}
impl From<Ram20RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule0) -> u8 {
        Ram20RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule1 {
        Ram20RuleRule1::from_bits(val)
    }
}
impl From<Ram20RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule1) -> u8 {
        Ram20RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule2 {
        Ram20RuleRule2::from_bits(val)
    }
}
impl From<Ram20RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule2) -> u8 {
        Ram20RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule3 {
        Ram20RuleRule3::from_bits(val)
    }
}
impl From<Ram20RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule3) -> u8 {
        Ram20RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule4 {
        Ram20RuleRule4::from_bits(val)
    }
}
impl From<Ram20RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule4) -> u8 {
        Ram20RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule5 {
        Ram20RuleRule5::from_bits(val)
    }
}
impl From<Ram20RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule5) -> u8 {
        Ram20RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule6 {
        Ram20RuleRule6::from_bits(val)
    }
}
impl From<Ram20RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule6) -> u8 {
        Ram20RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram20RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram20RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram20RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram20RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram20RuleRule7 {
        Ram20RuleRule7::from_bits(val)
    }
}
impl From<Ram20RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram20RuleRule7) -> u8 {
        Ram20RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule0 {
        Ram21RuleRule0::from_bits(val)
    }
}
impl From<Ram21RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule0) -> u8 {
        Ram21RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule1 {
        Ram21RuleRule1::from_bits(val)
    }
}
impl From<Ram21RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule1) -> u8 {
        Ram21RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule2 {
        Ram21RuleRule2::from_bits(val)
    }
}
impl From<Ram21RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule2) -> u8 {
        Ram21RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule3 {
        Ram21RuleRule3::from_bits(val)
    }
}
impl From<Ram21RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule3) -> u8 {
        Ram21RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule4 {
        Ram21RuleRule4::from_bits(val)
    }
}
impl From<Ram21RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule4) -> u8 {
        Ram21RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule5 {
        Ram21RuleRule5::from_bits(val)
    }
}
impl From<Ram21RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule5) -> u8 {
        Ram21RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule6 {
        Ram21RuleRule6::from_bits(val)
    }
}
impl From<Ram21RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule6) -> u8 {
        Ram21RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram21RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram21RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram21RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram21RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram21RuleRule7 {
        Ram21RuleRule7::from_bits(val)
    }
}
impl From<Ram21RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram21RuleRule7) -> u8 {
        Ram21RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule0 {
        Ram22RuleRule0::from_bits(val)
    }
}
impl From<Ram22RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule0) -> u8 {
        Ram22RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule1 {
        Ram22RuleRule1::from_bits(val)
    }
}
impl From<Ram22RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule1) -> u8 {
        Ram22RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule2 {
        Ram22RuleRule2::from_bits(val)
    }
}
impl From<Ram22RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule2) -> u8 {
        Ram22RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule3 {
        Ram22RuleRule3::from_bits(val)
    }
}
impl From<Ram22RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule3) -> u8 {
        Ram22RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule4 {
        Ram22RuleRule4::from_bits(val)
    }
}
impl From<Ram22RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule4) -> u8 {
        Ram22RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule5 {
        Ram22RuleRule5::from_bits(val)
    }
}
impl From<Ram22RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule5) -> u8 {
        Ram22RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule6 {
        Ram22RuleRule6::from_bits(val)
    }
}
impl From<Ram22RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule6) -> u8 {
        Ram22RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram22RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram22RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram22RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram22RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram22RuleRule7 {
        Ram22RuleRule7::from_bits(val)
    }
}
impl From<Ram22RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram22RuleRule7) -> u8 {
        Ram22RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule0 {
        Ram23RuleRule0::from_bits(val)
    }
}
impl From<Ram23RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule0) -> u8 {
        Ram23RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule1 {
        Ram23RuleRule1::from_bits(val)
    }
}
impl From<Ram23RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule1) -> u8 {
        Ram23RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule2 {
        Ram23RuleRule2::from_bits(val)
    }
}
impl From<Ram23RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule2) -> u8 {
        Ram23RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule3 {
        Ram23RuleRule3::from_bits(val)
    }
}
impl From<Ram23RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule3) -> u8 {
        Ram23RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule4 {
        Ram23RuleRule4::from_bits(val)
    }
}
impl From<Ram23RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule4) -> u8 {
        Ram23RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule5 {
        Ram23RuleRule5::from_bits(val)
    }
}
impl From<Ram23RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule5) -> u8 {
        Ram23RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule6 {
        Ram23RuleRule6::from_bits(val)
    }
}
impl From<Ram23RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule6) -> u8 {
        Ram23RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram23RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram23RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram23RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram23RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram23RuleRule7 {
        Ram23RuleRule7::from_bits(val)
    }
}
impl From<Ram23RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram23RuleRule7) -> u8 {
        Ram23RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule0 {
        Ram24RuleRule0::from_bits(val)
    }
}
impl From<Ram24RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule0) -> u8 {
        Ram24RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule1 {
        Ram24RuleRule1::from_bits(val)
    }
}
impl From<Ram24RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule1) -> u8 {
        Ram24RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule2 {
        Ram24RuleRule2::from_bits(val)
    }
}
impl From<Ram24RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule2) -> u8 {
        Ram24RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule3 {
        Ram24RuleRule3::from_bits(val)
    }
}
impl From<Ram24RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule3) -> u8 {
        Ram24RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule4 {
        Ram24RuleRule4::from_bits(val)
    }
}
impl From<Ram24RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule4) -> u8 {
        Ram24RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule5 {
        Ram24RuleRule5::from_bits(val)
    }
}
impl From<Ram24RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule5) -> u8 {
        Ram24RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule6 {
        Ram24RuleRule6::from_bits(val)
    }
}
impl From<Ram24RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule6) -> u8 {
        Ram24RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram24RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram24RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram24RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram24RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram24RuleRule7 {
        Ram24RuleRule7::from_bits(val)
    }
}
impl From<Ram24RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram24RuleRule7) -> u8 {
        Ram24RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule0 {
        Ram25RuleRule0::from_bits(val)
    }
}
impl From<Ram25RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule0) -> u8 {
        Ram25RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule1 {
        Ram25RuleRule1::from_bits(val)
    }
}
impl From<Ram25RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule1) -> u8 {
        Ram25RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule2 {
        Ram25RuleRule2::from_bits(val)
    }
}
impl From<Ram25RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule2) -> u8 {
        Ram25RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule3 {
        Ram25RuleRule3::from_bits(val)
    }
}
impl From<Ram25RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule3) -> u8 {
        Ram25RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule4 {
        Ram25RuleRule4::from_bits(val)
    }
}
impl From<Ram25RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule4) -> u8 {
        Ram25RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule5 {
        Ram25RuleRule5::from_bits(val)
    }
}
impl From<Ram25RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule5) -> u8 {
        Ram25RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule6 {
        Ram25RuleRule6::from_bits(val)
    }
}
impl From<Ram25RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule6) -> u8 {
        Ram25RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram25RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram25RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram25RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram25RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram25RuleRule7 {
        Ram25RuleRule7::from_bits(val)
    }
}
impl From<Ram25RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram25RuleRule7) -> u8 {
        Ram25RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule0 {
        Ram26RuleRule0::from_bits(val)
    }
}
impl From<Ram26RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule0) -> u8 {
        Ram26RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule1 {
        Ram26RuleRule1::from_bits(val)
    }
}
impl From<Ram26RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule1) -> u8 {
        Ram26RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule2 {
        Ram26RuleRule2::from_bits(val)
    }
}
impl From<Ram26RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule2) -> u8 {
        Ram26RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule3 {
        Ram26RuleRule3::from_bits(val)
    }
}
impl From<Ram26RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule3) -> u8 {
        Ram26RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule4 {
        Ram26RuleRule4::from_bits(val)
    }
}
impl From<Ram26RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule4) -> u8 {
        Ram26RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule5 {
        Ram26RuleRule5::from_bits(val)
    }
}
impl From<Ram26RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule5) -> u8 {
        Ram26RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule6 {
        Ram26RuleRule6::from_bits(val)
    }
}
impl From<Ram26RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule6) -> u8 {
        Ram26RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram26RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram26RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram26RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram26RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram26RuleRule7 {
        Ram26RuleRule7::from_bits(val)
    }
}
impl From<Ram26RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram26RuleRule7) -> u8 {
        Ram26RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule0 {
        Ram27RuleRule0::from_bits(val)
    }
}
impl From<Ram27RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule0) -> u8 {
        Ram27RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule1 {
        Ram27RuleRule1::from_bits(val)
    }
}
impl From<Ram27RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule1) -> u8 {
        Ram27RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule2 {
        Ram27RuleRule2::from_bits(val)
    }
}
impl From<Ram27RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule2) -> u8 {
        Ram27RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule3 {
        Ram27RuleRule3::from_bits(val)
    }
}
impl From<Ram27RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule3) -> u8 {
        Ram27RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule4 {
        Ram27RuleRule4::from_bits(val)
    }
}
impl From<Ram27RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule4) -> u8 {
        Ram27RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule5 {
        Ram27RuleRule5::from_bits(val)
    }
}
impl From<Ram27RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule5) -> u8 {
        Ram27RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule6 {
        Ram27RuleRule6::from_bits(val)
    }
}
impl From<Ram27RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule6) -> u8 {
        Ram27RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram27RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram27RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram27RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram27RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram27RuleRule7 {
        Ram27RuleRule7::from_bits(val)
    }
}
impl From<Ram27RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram27RuleRule7) -> u8 {
        Ram27RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule0 {
        Ram28RuleRule0::from_bits(val)
    }
}
impl From<Ram28RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule0) -> u8 {
        Ram28RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule1 {
        Ram28RuleRule1::from_bits(val)
    }
}
impl From<Ram28RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule1) -> u8 {
        Ram28RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule2 {
        Ram28RuleRule2::from_bits(val)
    }
}
impl From<Ram28RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule2) -> u8 {
        Ram28RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule3 {
        Ram28RuleRule3::from_bits(val)
    }
}
impl From<Ram28RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule3) -> u8 {
        Ram28RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule4 {
        Ram28RuleRule4::from_bits(val)
    }
}
impl From<Ram28RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule4) -> u8 {
        Ram28RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule5 {
        Ram28RuleRule5::from_bits(val)
    }
}
impl From<Ram28RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule5) -> u8 {
        Ram28RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule6 {
        Ram28RuleRule6::from_bits(val)
    }
}
impl From<Ram28RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule6) -> u8 {
        Ram28RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram28RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram28RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram28RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram28RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram28RuleRule7 {
        Ram28RuleRule7::from_bits(val)
    }
}
impl From<Ram28RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram28RuleRule7) -> u8 {
        Ram28RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule0 {
        Ram29RuleRule0::from_bits(val)
    }
}
impl From<Ram29RuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule0) -> u8 {
        Ram29RuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule1 {
        Ram29RuleRule1::from_bits(val)
    }
}
impl From<Ram29RuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule1) -> u8 {
        Ram29RuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule2 {
        Ram29RuleRule2::from_bits(val)
    }
}
impl From<Ram29RuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule2) -> u8 {
        Ram29RuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule3 {
        Ram29RuleRule3::from_bits(val)
    }
}
impl From<Ram29RuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule3) -> u8 {
        Ram29RuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule4 {
        Ram29RuleRule4::from_bits(val)
    }
}
impl From<Ram29RuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule4) -> u8 {
        Ram29RuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule5 {
        Ram29RuleRule5::from_bits(val)
    }
}
impl From<Ram29RuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule5) -> u8 {
        Ram29RuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule6 {
        Ram29RuleRule6::from_bits(val)
    }
}
impl From<Ram29RuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule6) -> u8 {
        Ram29RuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram29RuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ram29RuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram29RuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram29RuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Ram29RuleRule7 {
        Ram29RuleRule7::from_bits(val)
    }
}
impl From<Ram29RuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Ram29RuleRule7) -> u8 {
        Ram29RuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule0 {
        RomMemRuleRule0::from_bits(val)
    }
}
impl From<RomMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule0) -> u8 {
        RomMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule1 {
        RomMemRuleRule1::from_bits(val)
    }
}
impl From<RomMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule1) -> u8 {
        RomMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule2 {
        RomMemRuleRule2::from_bits(val)
    }
}
impl From<RomMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule2) -> u8 {
        RomMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule3 {
        RomMemRuleRule3::from_bits(val)
    }
}
impl From<RomMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule3) -> u8 {
        RomMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule4 {
        RomMemRuleRule4::from_bits(val)
    }
}
impl From<RomMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule4) -> u8 {
        RomMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule5 {
        RomMemRuleRule5::from_bits(val)
    }
}
impl From<RomMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule5) -> u8 {
        RomMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule6 {
        RomMemRuleRule6::from_bits(val)
    }
}
impl From<RomMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule6) -> u8 {
        RomMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule7 {
        RomMemRuleRule7::from_bits(val)
    }
}
impl From<RomMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule7) -> u8 {
        RomMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecDspIntLock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecDspIntLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecDspIntLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecDspIntLock {
    #[inline(always)]
    fn from(val: u8) -> SecDspIntLock {
        SecDspIntLock::from_bits(val)
    }
}
impl From<SecDspIntLock> for u8 {
    #[inline(always)]
    fn from(val: SecDspIntLock) -> u8 {
        SecDspIntLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask0Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
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
    #[doc = "Restrictive mode."]
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
pub enum SecGpioMask2Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask2Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask2Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask2Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask2Lock {
        SecGpioMask2Lock::from_bits(val)
    }
}
impl From<SecGpioMask2Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask2Lock) -> u8 {
        SecGpioMask2Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask3Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask3Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask3Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask3Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask3Lock {
        SecGpioMask3Lock::from_bits(val)
    }
}
impl From<SecGpioMask3Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask3Lock) -> u8 {
        SecGpioMask3Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask4Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask4Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask4Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask4Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask4Lock {
        SecGpioMask4Lock::from_bits(val)
    }
}
impl From<SecGpioMask4Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask4Lock) -> u8 {
        SecGpioMask4Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask5Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask5Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask5Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask5Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask5Lock {
        SecGpioMask5Lock::from_bits(val)
    }
}
impl From<SecGpioMask5Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask5Lock) -> u8 {
        SecGpioMask5Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask6Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask6Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask6Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask6Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask6Lock {
        SecGpioMask6Lock::from_bits(val)
    }
}
impl From<SecGpioMask6Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask6Lock) -> u8 {
        SecGpioMask6Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask7Lock {
    _RESERVED_0 = 0x0,
    #[doc = "Restrictive mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask7Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask7Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask7Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask7Lock {
        SecGpioMask7Lock::from_bits(val)
    }
}
impl From<SecGpioMask7Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask7Lock) -> u8 {
        SecGpioMask7Lock::to_bits(val)
    }
}
