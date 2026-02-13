#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dir {
    #[doc = "Down direction"]
    DOWN = 0x0,
    #[doc = "Up direction"]
    UP = 0x01,
}
impl Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dir {
    #[inline(always)]
    fn from(val: u8) -> Dir {
        Dir::from_bits(val)
    }
}
impl From<Dir> for u8 {
    #[inline(always)]
    fn from(val: Dir) -> u8 {
        Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hip {
    #[doc = "No action"]
    NOACTION = 0x0,
    #[doc = "HOME signal initializes the position counter"]
    HOME = 0x01,
}
impl Hip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hip {
    #[inline(always)]
    fn from(val: u8) -> Hip {
        Hip::from_bits(val)
    }
}
impl From<Hip> for u8 {
    #[inline(always)]
    fn from(val: Hip) -> u8 {
        Hip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hirq {
    #[doc = "Not occurred"]
    NOTRANS = 0x0,
    #[doc = "Occurred"]
    TRANSI = 0x01,
}
impl Hirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hirq {
    #[inline(always)]
    fn from(val: u8) -> Hirq {
        Hirq::from_bits(val)
    }
}
impl From<Hirq> for u8 {
    #[inline(always)]
    fn from(val: Hirq) -> u8 {
        Hirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hne {
    #[doc = "Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    POS = 0x0,
    #[doc = "Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    NEG = 0x01,
}
impl Hne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hne {
    #[inline(always)]
    fn from(val: u8) -> Hne {
        Hne::from_bits(val)
    }
}
impl From<Hne> for u8 {
    #[inline(always)]
    fn from(val: Hne) -> u8 {
        Hne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outctl {
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )"]
    COMPARE = 0x0,
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read"]
    READ = 0x01,
}
impl Outctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outctl {
    #[inline(always)]
    fn from(val: u8) -> Outctl {
        Outctl::from_bits(val)
    }
}
impl From<Outctl> for u8 {
    #[inline(always)]
    fn from(val: Outctl) -> u8 {
        Outctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ph1 {
    #[doc = "Uses the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
    USE = 0x0,
    #[doc = "Bypasses the quadrature decoder. A positive transition of the PHASEA input generates a count signal. PHASEB input and CTRL\\[REV\\] controls the counter direction. If the value of CTRL\\[REV\\] and PHASEB are identical; then count is up. If the value of CTRL\\[REV\\] and PHASEB is different, then count is down."]
    BYPASS = 0x01,
}
impl Ph1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ph1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ph1 {
    #[inline(always)]
    fn from(val: u8) -> Ph1 {
        Ph1::from_bits(val)
    }
}
impl From<Ph1> for u8 {
    #[inline(always)]
    fn from(val: Ph1) -> u8 {
        Ph1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdn {
    #[doc = "Positive quadrature decoder signal"]
    POSITIVE = 0x0,
    #[doc = "Negative quadrature decoder signal"]
    NEGATIVE = 0x01,
}
impl Qdn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdn {
    #[inline(always)]
    fn from(val: u8) -> Qdn {
        Qdn::from_bits(val)
    }
}
impl From<Qdn> for u8 {
    #[inline(always)]
    fn from(val: Qdn) -> u8 {
        Qdn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rev {
    #[doc = "Counts normally"]
    NORMAL = 0x0,
    #[doc = "Counts in the reverse direction"]
    REVERSE = 0x01,
}
impl Rev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rev {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rev {
    #[inline(always)]
    fn from(val: u8) -> Rev {
        Rev::from_bits(val)
    }
}
impl From<Rev> for u8 {
    #[inline(always)]
    fn from(val: Rev) -> u8 {
        Rev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Revmod {
    #[doc = "Use INDEX pulse"]
    INDEX = 0x0,
    #[doc = "Use modulus counting roll-over or roll-under"]
    COUNT = 0x01,
}
impl Revmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revmod {
    #[inline(always)]
    fn from(val: u8) -> Revmod {
        Revmod::from_bits(val)
    }
}
impl From<Revmod> for u8 {
    #[inline(always)]
    fn from(val: Revmod) -> u8 {
        Revmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swip {
    #[doc = "No action"]
    NOACTION = 0x0,
    #[doc = "Initialize position counter"]
    INIT = 0x01,
}
impl Swip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swip {
    #[inline(always)]
    fn from(val: u8) -> Swip {
        Swip::from_bits(val)
    }
}
impl From<Swip> for u8 {
    #[inline(always)]
    fn from(val: Swip) -> u8 {
        Swip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Updpos {
    #[doc = "No action"]
    NOACTION = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Updpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Updpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Updpos {
    #[inline(always)]
    fn from(val: u8) -> Updpos {
        Updpos::from_bits(val)
    }
}
impl From<Updpos> for u8 {
    #[inline(always)]
    fn from(val: Updpos) -> u8 {
        Updpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xirq {
    #[doc = "Not occurred"]
    NOIND = 0x0,
    #[doc = "Occurred"]
    INDEX = 0x01,
}
impl Xirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xirq {
    #[inline(always)]
    fn from(val: u8) -> Xirq {
        Xirq::from_bits(val)
    }
}
impl From<Xirq> for u8 {
    #[inline(always)]
    fn from(val: Xirq) -> u8 {
        Xirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xne {
    #[doc = "Use positive edge"]
    XNE_0 = 0x0,
    #[doc = "Use negative edge"]
    XNE_1 = 0x01,
}
impl Xne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xne {
    #[inline(always)]
    fn from(val: u8) -> Xne {
        Xne::from_bits(val)
    }
}
impl From<Xne> for u8 {
    #[inline(always)]
    fn from(val: Xne) -> u8 {
        Xne::to_bits(val)
    }
}
