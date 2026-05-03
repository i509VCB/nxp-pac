#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClcrLgo {
    #[doc = "Write: no effect. Read: no line command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    InitCmd = 0x01,
}
impl ClcrLgo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClcrLgo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClcrLgo {
    #[inline(always)]
    fn from(val: u8) -> ClcrLgo {
        ClcrLgo::from_bits(val)
    }
}
impl From<ClcrLgo> for u8 {
    #[inline(always)]
    fn from(val: ClcrLgo) -> u8 {
        ClcrLgo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsarLgo {
    #[doc = "Write: no effect. Read: no line command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    InitCmd = 0x01,
}
impl CsarLgo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsarLgo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsarLgo {
    #[inline(always)]
    fn from(val: u8) -> CsarLgo {
        CsarLgo::from_bits(val)
    }
}
impl From<CsarLgo> for u8 {
    #[inline(always)]
    fn from(val: CsarLgo) -> u8 {
        CsarLgo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Go {
    #[doc = "Write: no effect. Read: no cache command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    InitCmd = 0x01,
}
impl Go {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Go {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Go {
    #[inline(always)]
    fn from(val: u8) -> Go {
        Go::from_bits(val)
    }
}
impl From<Go> for u8 {
    #[inline(always)]
    fn from(val: Go) -> u8 {
        Go::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invw0 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    Invw0 = 0x01,
}
impl Invw0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invw0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invw0 {
    #[inline(always)]
    fn from(val: u8) -> Invw0 {
        Invw0::from_bits(val)
    }
}
impl From<Invw0> for u8 {
    #[inline(always)]
    fn from(val: Invw0) -> u8 {
        Invw0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invw1 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "When setting the GO bit, invalidate all lines in way 1."]
    Invw1 = 0x01,
}
impl Invw1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invw1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invw1 {
    #[inline(always)]
    fn from(val: u8) -> Invw1 {
        Invw1::from_bits(val)
    }
}
impl From<Invw1> for u8 {
    #[inline(always)]
    fn from(val: Invw1) -> u8 {
        Invw1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lacc {
    #[doc = "Read."]
    Read = 0x0,
    #[doc = "Write."]
    Write = 0x01,
}
impl Lacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lacc {
    #[inline(always)]
    fn from(val: u8) -> Lacc {
        Lacc::from_bits(val)
    }
}
impl From<Lacc> for u8 {
    #[inline(always)]
    fn from(val: Lacc) -> u8 {
        Lacc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ladsel {
    #[doc = "Cache address."]
    CacheAddr = 0x0,
    #[doc = "Physical address."]
    PhysAddr = 0x01,
}
impl Ladsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ladsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ladsel {
    #[inline(always)]
    fn from(val: u8) -> Ladsel {
        Ladsel::from_bits(val)
    }
}
impl From<Ladsel> for u8 {
    #[inline(always)]
    fn from(val: Ladsel) -> u8 {
        Ladsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcmd {
    #[doc = "Search and read or write."]
    SearchRw = 0x0,
    #[doc = "Invalidate."]
    Invalidate = 0x01,
    #[doc = "Push."]
    Push = 0x02,
    #[doc = "Clear."]
    Clear = 0x03,
}
impl Lcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcmd {
    #[inline(always)]
    fn from(val: u8) -> Lcmd {
        Lcmd::from_bits(val)
    }
}
impl From<Lcmd> for u8 {
    #[inline(always)]
    fn from(val: Lcmd) -> u8 {
        Lcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pushw0 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "When setting the GO bit, push all modified lines in way 0."]
    Pushw0 = 0x01,
}
impl Pushw0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pushw0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pushw0 {
    #[inline(always)]
    fn from(val: u8) -> Pushw0 {
        Pushw0::from_bits(val)
    }
}
impl From<Pushw0> for u8 {
    #[inline(always)]
    fn from(val: Pushw0) -> u8 {
        Pushw0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pushw1 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "When setting the GO bit, push all modified lines in way 1."]
    Pushw1 = 0x01,
}
impl Pushw1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pushw1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pushw1 {
    #[inline(always)]
    fn from(val: u8) -> Pushw1 {
        Pushw1::from_bits(val)
    }
}
impl From<Pushw1> for u8 {
    #[inline(always)]
    fn from(val: Pushw1) -> u8 {
        Pushw1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdsel {
    #[doc = "Data."]
    Data = 0x0,
    #[doc = "Tag."]
    Tag = 0x01,
}
impl Tdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdsel {
    #[inline(always)]
    fn from(val: u8) -> Tdsel {
        Tdsel::from_bits(val)
    }
}
impl From<Tdsel> for u8 {
    #[inline(always)]
    fn from(val: Tdsel) -> u8 {
        Tdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wsel {
    #[doc = "Way 0."]
    Way0 = 0x0,
    #[doc = "Way 1."]
    Way1 = 0x01,
}
impl Wsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wsel {
    #[inline(always)]
    fn from(val: u8) -> Wsel {
        Wsel::from_bits(val)
    }
}
impl From<Wsel> for u8 {
    #[inline(always)]
    fn from(val: Wsel) -> u8 {
        Wsel::to_bits(val)
    }
}
