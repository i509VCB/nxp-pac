#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In0Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In0Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In0Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In0Status {
    #[inline(always)]
    fn from(val: u8) -> In0Status {
        In0Status::from_bits(val)
    }
}
impl From<In0Status> for u8 {
    #[inline(always)]
    fn from(val: In0Status) -> u8 {
        In0Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In10Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In10Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In10Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In10Status {
    #[inline(always)]
    fn from(val: u8) -> In10Status {
        In10Status::from_bits(val)
    }
}
impl From<In10Status> for u8 {
    #[inline(always)]
    fn from(val: In10Status) -> u8 {
        In10Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In112Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In112Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In112Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In112Status {
    #[inline(always)]
    fn from(val: u8) -> In112Status {
        In112Status::from_bits(val)
    }
}
impl From<In112Status> for u8 {
    #[inline(always)]
    fn from(val: In112Status) -> u8 {
        In112Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In113Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In113Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In113Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In113Status {
    #[inline(always)]
    fn from(val: u8) -> In113Status {
        In113Status::from_bits(val)
    }
}
impl From<In113Status> for u8 {
    #[inline(always)]
    fn from(val: In113Status) -> u8 {
        In113Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In11Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In11Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In11Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In11Status {
    #[inline(always)]
    fn from(val: u8) -> In11Status {
        In11Status::from_bits(val)
    }
}
impl From<In11Status> for u8 {
    #[inline(always)]
    fn from(val: In11Status) -> u8 {
        In11Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In14Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In14Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In14Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In14Status {
    #[inline(always)]
    fn from(val: u8) -> In14Status {
        In14Status::from_bits(val)
    }
}
impl From<In14Status> for u8 {
    #[inline(always)]
    fn from(val: In14Status) -> u8 {
        In14Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In15Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In15Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In15Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In15Status {
    #[inline(always)]
    fn from(val: u8) -> In15Status {
        In15Status::from_bits(val)
    }
}
impl From<In15Status> for u8 {
    #[inline(always)]
    fn from(val: In15Status) -> u8 {
        In15Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In16Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In16Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In16Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In16Status {
    #[inline(always)]
    fn from(val: u8) -> In16Status {
        In16Status::from_bits(val)
    }
}
impl From<In16Status> for u8 {
    #[inline(always)]
    fn from(val: In16Status) -> u8 {
        In16Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In17Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In17Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In17Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In17Status {
    #[inline(always)]
    fn from(val: u8) -> In17Status {
        In17Status::from_bits(val)
    }
}
impl From<In17Status> for u8 {
    #[inline(always)]
    fn from(val: In17Status) -> u8 {
        In17Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In18Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In18Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In18Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In18Status {
    #[inline(always)]
    fn from(val: u8) -> In18Status {
        In18Status::from_bits(val)
    }
}
impl From<In18Status> for u8 {
    #[inline(always)]
    fn from(val: In18Status) -> u8 {
        In18Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In19Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In19Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In19Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In19Status {
    #[inline(always)]
    fn from(val: u8) -> In19Status {
        In19Status::from_bits(val)
    }
}
impl From<In19Status> for u8 {
    #[inline(always)]
    fn from(val: In19Status) -> u8 {
        In19Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In1Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In1Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In1Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In1Status {
    #[inline(always)]
    fn from(val: u8) -> In1Status {
        In1Status::from_bits(val)
    }
}
impl From<In1Status> for u8 {
    #[inline(always)]
    fn from(val: In1Status) -> u8 {
        In1Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In20Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In20Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In20Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In20Status {
    #[inline(always)]
    fn from(val: u8) -> In20Status {
        In20Status::from_bits(val)
    }
}
impl From<In20Status> for u8 {
    #[inline(always)]
    fn from(val: In20Status) -> u8 {
        In20Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In2421Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
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
impl In2421Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In2421Status {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In2421Status {
    #[inline(always)]
    fn from(val: u8) -> In2421Status {
        In2421Status::from_bits(val)
    }
}
impl From<In2421Status> for u8 {
    #[inline(always)]
    fn from(val: In2421Status) -> u8 {
        In2421Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In2Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In2Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In2Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In2Status {
    #[inline(always)]
    fn from(val: u8) -> In2Status {
        In2Status::from_bits(val)
    }
}
impl From<In2Status> for u8 {
    #[inline(always)]
    fn from(val: In2Status) -> u8 {
        In2Status::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct In3225Status(u8);
impl In3225Status {
    #[doc = "Output not triggered."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Output has been triggered."]
    pub const ENABLE: Self = Self(0x01);
}
impl In3225Status {
    pub const fn from_bits(val: u8) -> In3225Status {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for In3225Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for In3225Status {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for In3225Status {
    #[inline(always)]
    fn from(val: u8) -> In3225Status {
        In3225Status::from_bits(val)
    }
}
impl From<In3225Status> for u8 {
    #[inline(always)]
    fn from(val: In3225Status) -> u8 {
        In3225Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In33Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In33Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In33Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In33Status {
    #[inline(always)]
    fn from(val: u8) -> In33Status {
        In33Status::from_bits(val)
    }
}
impl From<In33Status> for u8 {
    #[inline(always)]
    fn from(val: In33Status) -> u8 {
        In33Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In34Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In34Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In34Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In34Status {
    #[inline(always)]
    fn from(val: u8) -> In34Status {
        In34Status::from_bits(val)
    }
}
impl From<In34Status> for u8 {
    #[inline(always)]
    fn from(val: In34Status) -> u8 {
        In34Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In35Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In35Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In35Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In35Status {
    #[inline(always)]
    fn from(val: u8) -> In35Status {
        In35Status::from_bits(val)
    }
}
impl From<In35Status> for u8 {
    #[inline(always)]
    fn from(val: In35Status) -> u8 {
        In35Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In36Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In36Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In36Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In36Status {
    #[inline(always)]
    fn from(val: u8) -> In36Status {
        In36Status::from_bits(val)
    }
}
impl From<In36Status> for u8 {
    #[inline(always)]
    fn from(val: In36Status) -> u8 {
        In36Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In37Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In37Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In37Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In37Status {
    #[inline(always)]
    fn from(val: u8) -> In37Status {
        In37Status::from_bits(val)
    }
}
impl From<In37Status> for u8 {
    #[inline(always)]
    fn from(val: In37Status) -> u8 {
        In37Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In3Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In3Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In3Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In3Status {
    #[inline(always)]
    fn from(val: u8) -> In3Status {
        In3Status::from_bits(val)
    }
}
impl From<In3Status> for u8 {
    #[inline(always)]
    fn from(val: In3Status) -> u8 {
        In3Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In46Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In46Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In46Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In46Status {
    #[inline(always)]
    fn from(val: u8) -> In46Status {
        In46Status::from_bits(val)
    }
}
impl From<In46Status> for u8 {
    #[inline(always)]
    fn from(val: In46Status) -> u8 {
        In46Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In47Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In47Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In47Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In47Status {
    #[inline(always)]
    fn from(val: u8) -> In47Status {
        In47Status::from_bits(val)
    }
}
impl From<In47Status> for u8 {
    #[inline(always)]
    fn from(val: In47Status) -> u8 {
        In47Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In4Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In4Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In4Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In4Status {
    #[inline(always)]
    fn from(val: u8) -> In4Status {
        In4Status::from_bits(val)
    }
}
impl From<In4Status> for u8 {
    #[inline(always)]
    fn from(val: In4Status) -> u8 {
        In4Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In5Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In5Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In5Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In5Status {
    #[inline(always)]
    fn from(val: u8) -> In5Status {
        In5Status::from_bits(val)
    }
}
impl From<In5Status> for u8 {
    #[inline(always)]
    fn from(val: In5Status) -> u8 {
        In5Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In6Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In6Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In6Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In6Status {
    #[inline(always)]
    fn from(val: u8) -> In6Status {
        In6Status::from_bits(val)
    }
}
impl From<In6Status> for u8 {
    #[inline(always)]
    fn from(val: In6Status) -> u8 {
        In6Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In7Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In7Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In7Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In7Status {
    #[inline(always)]
    fn from(val: u8) -> In7Status {
        In7Status::from_bits(val)
    }
}
impl From<In7Status> for u8 {
    #[inline(always)]
    fn from(val: In7Status) -> u8 {
        In7Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In8Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In8Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In8Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In8Status {
    #[inline(always)]
    fn from(val: u8) -> In8Status {
        In8Status::from_bits(val)
    }
}
impl From<In8Status> for u8 {
    #[inline(always)]
    fn from(val: In8Status) -> u8 {
        In8Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In9Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl In9Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In9Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In9Status {
    #[inline(always)]
    fn from(val: u8) -> In9Status {
        In9Status::from_bits(val)
    }
}
impl From<In9Status> for u8 {
    #[inline(always)]
    fn from(val: In9Status) -> u8 {
        In9Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out0Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out0Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out0Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out0Status {
    #[inline(always)]
    fn from(val: u8) -> Out0Status {
        Out0Status::from_bits(val)
    }
}
impl From<Out0Status> for u8 {
    #[inline(always)]
    fn from(val: Out0Status) -> u8 {
        Out0Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out1Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out1Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out1Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out1Status {
    #[inline(always)]
    fn from(val: u8) -> Out1Status {
        Out1Status::from_bits(val)
    }
}
impl From<Out1Status> for u8 {
    #[inline(always)]
    fn from(val: Out1Status) -> u8 {
        Out1Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out2Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out2Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out2Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out2Status {
    #[inline(always)]
    fn from(val: u8) -> Out2Status {
        Out2Status::from_bits(val)
    }
}
impl From<Out2Status> for u8 {
    #[inline(always)]
    fn from(val: Out2Status) -> u8 {
        Out2Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out3Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out3Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out3Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out3Status {
    #[inline(always)]
    fn from(val: u8) -> Out3Status {
        Out3Status::from_bits(val)
    }
}
impl From<Out3Status> for u8 {
    #[inline(always)]
    fn from(val: Out3Status) -> u8 {
        Out3Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out4Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out4Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out4Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out4Status {
    #[inline(always)]
    fn from(val: u8) -> Out4Status {
        Out4Status::from_bits(val)
    }
}
impl From<Out4Status> for u8 {
    #[inline(always)]
    fn from(val: Out4Status) -> u8 {
        Out4Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out5Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out5Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out5Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out5Status {
    #[inline(always)]
    fn from(val: u8) -> Out5Status {
        Out5Status::from_bits(val)
    }
}
impl From<Out5Status> for u8 {
    #[inline(always)]
    fn from(val: Out5Status) -> u8 {
        Out5Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out6Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
}
impl Out6Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out6Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out6Status {
    #[inline(always)]
    fn from(val: u8) -> Out6Status {
        Out6Status::from_bits(val)
    }
}
impl From<Out6Status> for u8 {
    #[inline(always)]
    fn from(val: Out6Status) -> u8 {
        Out6Status::to_bits(val)
    }
}
