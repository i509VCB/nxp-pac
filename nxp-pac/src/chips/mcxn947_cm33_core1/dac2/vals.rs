#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BufEn {
    #[doc = "Not used"]
    USE_BUF = 0x0,
    #[doc = "Used"]
    NO_USE_BUF = 0x01,
}
impl BufEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BufEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BufEn {
    #[inline(always)]
    fn from(val: u8) -> BufEn {
        BufEn::from_bits(val)
    }
}
impl From<BufEn> for u8 {
    #[inline(always)]
    fn from(val: BufEn) -> u8 {
        BufEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoen {
    #[doc = "Enables FIFO mode and disables Buffer mode. Any data written to DATA\\[DATA\\] goes to buffer then goes to conversion."]
    BUFFER_MODE = 0x0,
    #[doc = "Enables FIFO mode. Data will be first read from FIFO to buffer and then goes to conversion."]
    FIFO_MODE = 0x01,
}
impl Fifoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoen {
    #[inline(always)]
    fn from(val: u8) -> Fifoen {
        Fifoen::from_bits(val)
    }
}
impl From<Fifoen> for u8 {
    #[inline(always)]
    fn from(val: Fifoen) -> u8 {
        Fifoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fiforst {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "FIFO reset"]
    FIFO_RESET = 0x01,
}
impl Fiforst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fiforst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fiforst {
    #[inline(always)]
    fn from(val: u8) -> Fiforst {
        Fiforst::from_bits(val)
    }
}
impl From<Fiforst> for u8 {
    #[inline(always)]
    fn from(val: Fiforst) -> u8 {
        Fiforst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifosz {
    _RESERVED_0 = 0x0,
    #[doc = "FIFO depth is 4"]
    VAL_1 = 0x01,
    #[doc = "FIFO depth is 8"]
    VAL_2 = 0x02,
    #[doc = "FIFO depth is 16"]
    VAL_3 = 0x03,
    #[doc = "FIFO depth is 32"]
    VAL_4 = 0x04,
    #[doc = "FIFO depth is 64"]
    VAL_5 = 0x05,
    #[doc = "FIFO depth is 128"]
    VAL_6 = 0x06,
    #[doc = "FIFO depth is 256"]
    VAL_7 = 0x07,
}
impl Fifosz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifosz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifosz {
    #[inline(always)]
    fn from(val: u8) -> Fifosz {
        Fifosz::from_bits(val)
    }
}
impl From<Fifosz> for u8 {
    #[inline(always)]
    fn from(val: Fifosz) -> u8 {
        Fifosz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptgcoco {
    #[doc = "Not completed or not started"]
    NOT_START = 0x0,
    #[doc = "Completed"]
    COMPLETED = 0x01,
}
impl Ptgcoco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptgcoco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptgcoco {
    #[inline(always)]
    fn from(val: u8) -> Ptgcoco {
        Ptgcoco::from_bits(val)
    }
}
impl From<Ptgcoco> for u8 {
    #[inline(always)]
    fn from(val: Ptgcoco) -> u8 {
        Ptgcoco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swbk {
    #[doc = "No swing back cycle has completed since the last time the flag was cleared"]
    NO_SWING = 0x0,
    #[doc = "At least one swing back cycle has occurred since the last time the flag was cleared"]
    SWING_BACK = 0x01,
}
impl Swbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swbk {
    #[inline(always)]
    fn from(val: u8) -> Swbk {
        Swbk::from_bits(val)
    }
}
impl From<Swbk> for u8 {
    #[inline(always)]
    fn from(val: Swbk) -> u8 {
        Swbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrst {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Software reset"]
    SOFTWARE_RESET = 0x01,
}
impl Swrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swrst {
    #[inline(always)]
    fn from(val: u8) -> Swrst {
        Swrst::from_bits(val)
    }
}
impl From<Swrst> for u8 {
    #[inline(always)]
    fn from(val: Swrst) -> u8 {
        Swrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Hardware trigger"]
    HARDWARE = 0x0,
    #[doc = "Software trigger"]
    SOFTWARE = 0x01,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wm {
    #[doc = "Data in FIFO is more than watermark level"]
    MORE_THAN_WLEVEL = 0x0,
    #[doc = "Data in FIFO is less than or equal to watermark level"]
    LESS_THAN_WLEVEL = 0x01,
}
impl Wm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wm {
    #[inline(always)]
    fn from(val: u8) -> Wm {
        Wm::from_bits(val)
    }
}
impl From<Wm> for u8 {
    #[inline(always)]
    fn from(val: Wm) -> u8 {
        Wm::to_bits(val)
    }
}
