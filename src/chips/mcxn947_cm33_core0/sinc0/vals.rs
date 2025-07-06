#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admasel {
    #[doc = "Alternate DMA disabled"]
    DISABLED = 0x0,
    #[doc = "PF conversion complete"]
    PF_CONV_COMPLETE = 0x01,
    #[doc = "PF data output ready"]
    PF_DATA_READY = 0x02,
    #[doc = "Zero crossing detected"]
    ZCD = 0x03,
    #[doc = "Short circuit detected"]
    SCD = 0x04,
    #[doc = "Window limit detected"]
    WINDOW_LMT = 0x05,
    #[doc = "Low limit detected"]
    LOW_LMT = 0x06,
    #[doc = "High limit"]
    HIGH_LMT = 0x07,
    #[doc = "FIFO underflow"]
    FIFO_UF = 0x08,
    #[doc = "FIFO overflow"]
    FIFO_OF = 0x09,
    #[doc = "Clock absence"]
    CLK_ABS = 0x0a,
    #[doc = "Saturation"]
    SAT = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Admasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admasel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admasel {
    #[inline(always)]
    fn from(val: u8) -> Admasel {
        Admasel::from_bits(val)
    }
}
impl From<Admasel> for u8 {
    #[inline(always)]
    fn from(val: Admasel) -> u8 {
        Admasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Biassat {
    #[doc = "Did not occur"]
    SAT_NO = 0x0,
    #[doc = "Occurred"]
    SAT_YES = 0x01,
}
impl Biassat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Biassat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Biassat {
    #[inline(always)]
    fn from(val: u8) -> Biassat {
        Biassat::from_bits(val)
    }
}
impl From<Biassat> for u8 {
    #[inline(always)]
    fn from(val: Biassat) -> u8 {
        Biassat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad0 {
    #[doc = "Clock present"]
    CAD_NO = 0x0,
    #[doc = "Clock absent"]
    CAD_YES = 0x01,
}
impl Cad0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad0 {
    #[inline(always)]
    fn from(val: u8) -> Cad0 {
        Cad0::from_bits(val)
    }
}
impl From<Cad0> for u8 {
    #[inline(always)]
    fn from(val: Cad0) -> u8 {
        Cad0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad1 {
    #[doc = "Clock present"]
    CAD_NO = 0x0,
    #[doc = "Clock absent"]
    CAD_YES = 0x01,
}
impl Cad1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad1 {
    #[inline(always)]
    fn from(val: u8) -> Cad1 {
        Cad1::from_bits(val)
    }
}
impl From<Cad1> for u8 {
    #[inline(always)]
    fn from(val: Cad1) -> u8 {
        Cad1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad2 {
    #[doc = "Clock present"]
    CAD_NO = 0x0,
    #[doc = "Clock absent"]
    CAD_YES = 0x01,
}
impl Cad2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad2 {
    #[inline(always)]
    fn from(val: u8) -> Cad2 {
        Cad2::from_bits(val)
    }
}
impl From<Cad2> for u8 {
    #[inline(always)]
    fn from(val: Cad2) -> u8 {
        Cad2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad3 {
    #[doc = "Clock present"]
    CAD_NO = 0x0,
    #[doc = "Clock absent"]
    CAD_YES = 0x01,
}
impl Cad3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad3 {
    #[inline(always)]
    fn from(val: u8) -> Cad3 {
        Cad3::from_bits(val)
    }
}
impl From<Cad3> for u8 {
    #[inline(always)]
    fn from(val: Cad3) -> u8 {
        Cad3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad4 {
    #[doc = "Clock present"]
    CAD_NO = 0x0,
    #[doc = "Clock absent"]
    CAD_YES = 0x01,
}
impl Cad4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad4 {
    #[inline(always)]
    fn from(val: u8) -> Cad4 {
        Cad4::from_bits(val)
    }
}
impl From<Cad4> for u8 {
    #[inline(always)]
    fn from(val: Cad4) -> u8 {
        Cad4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadbk {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadbk {
    #[inline(always)]
    fn from(val: u8) -> Cadbk {
        Cadbk::from_bits(val)
    }
}
impl From<Cadbk> for u8 {
    #[inline(always)]
    fn from(val: Cadbk) -> u8 {
        Cadbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caden {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Caden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caden {
    #[inline(always)]
    fn from(val: u8) -> Caden {
        Caden::from_bits(val)
    }
}
impl From<Caden> for u8 {
    #[inline(always)]
    fn from(val: Caden) -> u8 {
        Caden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie0 {
    #[inline(always)]
    fn from(val: u8) -> Cadie0 {
        Cadie0::from_bits(val)
    }
}
impl From<Cadie0> for u8 {
    #[inline(always)]
    fn from(val: Cadie0) -> u8 {
        Cadie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie1 {
    #[inline(always)]
    fn from(val: u8) -> Cadie1 {
        Cadie1::from_bits(val)
    }
}
impl From<Cadie1> for u8 {
    #[inline(always)]
    fn from(val: Cadie1) -> u8 {
        Cadie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie2 {
    #[inline(always)]
    fn from(val: u8) -> Cadie2 {
        Cadie2::from_bits(val)
    }
}
impl From<Cadie2> for u8 {
    #[inline(always)]
    fn from(val: Cadie2) -> u8 {
        Cadie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie3 {
    #[inline(always)]
    fn from(val: u8) -> Cadie3 {
        Cadie3::from_bits(val)
    }
}
impl From<Cadie3> for u8 {
    #[inline(always)]
    fn from(val: Cadie3) -> u8 {
        Cadie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cadie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie4 {
    #[inline(always)]
    fn from(val: u8) -> Cadie4 {
        Cadie4::from_bits(val)
    }
}
impl From<Cadie4> for u8 {
    #[inline(always)]
    fn from(val: Cadie4) -> u8 {
        Cadie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadlmt {
    #[doc = "Disables CAD"]
    DISABLES = 0x0,
    _RESERVED_1 = 0x01,
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
impl Cadlmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadlmt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadlmt {
    #[inline(always)]
    fn from(val: u8) -> Cadlmt {
        Cadlmt::from_bits(val)
    }
}
impl From<Cadlmt> for u8 {
    #[inline(always)]
    fn from(val: Cadlmt) -> u8 {
        Cadlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chen {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chen {
    #[inline(always)]
    fn from(val: u8) -> Chen {
        Chen::from_bits(val)
    }
}
impl From<Chen> for u8 {
    #[inline(always)]
    fn from(val: Chen) -> u8 {
        Chen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf0 {
    #[doc = "No overflow; data not available"]
    OVFLW_NO = 0x0,
    #[doc = "Overflow; data available"]
    OVFLW_YES = 0x01,
}
impl Chf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf0 {
    #[inline(always)]
    fn from(val: u8) -> Chf0 {
        Chf0::from_bits(val)
    }
}
impl From<Chf0> for u8 {
    #[inline(always)]
    fn from(val: Chf0) -> u8 {
        Chf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf1 {
    #[doc = "No overflow; data not available"]
    OVFLW_NO = 0x0,
    #[doc = "Overflow; data available"]
    OVFLW_YES = 0x01,
}
impl Chf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf1 {
    #[inline(always)]
    fn from(val: u8) -> Chf1 {
        Chf1::from_bits(val)
    }
}
impl From<Chf1> for u8 {
    #[inline(always)]
    fn from(val: Chf1) -> u8 {
        Chf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf2 {
    #[doc = "No overflow; data not available"]
    OVFLW_NO = 0x0,
    #[doc = "Overflow; data available"]
    OVFLW_YES = 0x01,
}
impl Chf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf2 {
    #[inline(always)]
    fn from(val: u8) -> Chf2 {
        Chf2::from_bits(val)
    }
}
impl From<Chf2> for u8 {
    #[inline(always)]
    fn from(val: Chf2) -> u8 {
        Chf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf3 {
    #[doc = "No overflow; data not available"]
    OVFLW_NO = 0x0,
    #[doc = "Overflow; data available"]
    OVFLW_YES = 0x01,
}
impl Chf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf3 {
    #[inline(always)]
    fn from(val: u8) -> Chf3 {
        Chf3::from_bits(val)
    }
}
impl From<Chf3> for u8 {
    #[inline(always)]
    fn from(val: Chf3) -> u8 {
        Chf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf4 {
    #[doc = "No overflow; data not available"]
    OVFLW_NO = 0x0,
    #[doc = "Overflow; data available"]
    OVFLW_YES = 0x01,
}
impl Chf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf4 {
    #[inline(always)]
    fn from(val: u8) -> Chf4 {
        Chf4::from_bits(val)
    }
}
impl From<Chf4> for u8 {
    #[inline(always)]
    fn from(val: Chf4) -> u8 {
        Chf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chfie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie0 {
    #[inline(always)]
    fn from(val: u8) -> Chfie0 {
        Chfie0::from_bits(val)
    }
}
impl From<Chfie0> for u8 {
    #[inline(always)]
    fn from(val: Chfie0) -> u8 {
        Chfie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chfie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie1 {
    #[inline(always)]
    fn from(val: u8) -> Chfie1 {
        Chfie1::from_bits(val)
    }
}
impl From<Chfie1> for u8 {
    #[inline(always)]
    fn from(val: Chfie1) -> u8 {
        Chfie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chfie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie2 {
    #[inline(always)]
    fn from(val: u8) -> Chfie2 {
        Chfie2::from_bits(val)
    }
}
impl From<Chfie2> for u8 {
    #[inline(always)]
    fn from(val: Chfie2) -> u8 {
        Chfie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chfie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie3 {
    #[inline(always)]
    fn from(val: u8) -> Chfie3 {
        Chfie3::from_bits(val)
    }
}
impl From<Chfie3> for u8 {
    #[inline(always)]
    fn from(val: Chfie3) -> u8 {
        Chfie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Chfie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie4 {
    #[inline(always)]
    fn from(val: u8) -> Chfie4 {
        Chfie4::from_bits(val)
    }
}
impl From<Chfie4> for u8 {
    #[inline(always)]
    fn from(val: Chfie4) -> u8 {
        Chfie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy0 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Chrdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy0 {
    #[inline(always)]
    fn from(val: u8) -> Chrdy0 {
        Chrdy0::from_bits(val)
    }
}
impl From<Chrdy0> for u8 {
    #[inline(always)]
    fn from(val: Chrdy0) -> u8 {
        Chrdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy1 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Chrdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy1 {
    #[inline(always)]
    fn from(val: u8) -> Chrdy1 {
        Chrdy1::from_bits(val)
    }
}
impl From<Chrdy1> for u8 {
    #[inline(always)]
    fn from(val: Chrdy1) -> u8 {
        Chrdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy2 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Chrdy2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy2 {
    #[inline(always)]
    fn from(val: u8) -> Chrdy2 {
        Chrdy2::from_bits(val)
    }
}
impl From<Chrdy2> for u8 {
    #[inline(always)]
    fn from(val: Chrdy2) -> u8 {
        Chrdy2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy3 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Chrdy3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy3 {
    #[inline(always)]
    fn from(val: u8) -> Chrdy3 {
        Chrdy3::from_bits(val)
    }
}
impl From<Chrdy3> for u8 {
    #[inline(always)]
    fn from(val: Chrdy3) -> u8 {
        Chrdy3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy4 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Chrdy4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy4 {
    #[inline(always)]
    fn from(val: u8) -> Chrdy4 {
        Chrdy4::from_bits(val)
    }
}
impl From<Chrdy4> for u8 {
    #[inline(always)]
    fn from(val: Chrdy4) -> u8 {
        Chrdy4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip0 {
    #[doc = "Not in progress"]
    CONV_NO = 0x0,
    #[doc = "In progress"]
    CONV_YES = 0x01,
}
impl Cip0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip0 {
    #[inline(always)]
    fn from(val: u8) -> Cip0 {
        Cip0::from_bits(val)
    }
}
impl From<Cip0> for u8 {
    #[inline(always)]
    fn from(val: Cip0) -> u8 {
        Cip0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip1 {
    #[doc = "Not in progress"]
    CONV_NO = 0x0,
    #[doc = "In progress"]
    CONV_YES = 0x01,
}
impl Cip1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip1 {
    #[inline(always)]
    fn from(val: u8) -> Cip1 {
        Cip1::from_bits(val)
    }
}
impl From<Cip1> for u8 {
    #[inline(always)]
    fn from(val: Cip1) -> u8 {
        Cip1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip2 {
    #[doc = "Not in progress"]
    CONV_NO = 0x0,
    #[doc = "In progress"]
    CONV_YES = 0x01,
}
impl Cip2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip2 {
    #[inline(always)]
    fn from(val: u8) -> Cip2 {
        Cip2::from_bits(val)
    }
}
impl From<Cip2> for u8 {
    #[inline(always)]
    fn from(val: Cip2) -> u8 {
        Cip2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip3 {
    #[doc = "Not in progress"]
    CONV_NO = 0x0,
    #[doc = "In progress"]
    CONV_YES = 0x01,
}
impl Cip3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip3 {
    #[inline(always)]
    fn from(val: u8) -> Cip3 {
        Cip3::from_bits(val)
    }
}
impl From<Cip3> for u8 {
    #[inline(always)]
    fn from(val: Cip3) -> u8 {
        Cip3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip4 {
    #[doc = "Not in progress"]
    CONV_NO = 0x0,
    #[doc = "In progress"]
    CONV_YES = 0x01,
}
impl Cip4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip4 {
    #[inline(always)]
    fn from(val: u8) -> Cip4 {
        Cip4::from_bits(val)
    }
}
impl From<Cip4> for u8 {
    #[inline(always)]
    fn from(val: Cip4) -> u8 {
        Cip4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CnumOv {
    #[doc = "No overflow"]
    OFLW_NO = 0x0,
    #[doc = "Overflow"]
    OFLW_YES = 0x01,
}
impl CnumOv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CnumOv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CnumOv {
    #[inline(always)]
    fn from(val: u8) -> CnumOv {
        CnumOv::from_bits(val)
    }
}
impl From<CnumOv> for u8 {
    #[inline(always)]
    fn from(val: CnumOv) -> u8 {
        CnumOv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc0 {
    #[doc = "Not finished; data not available"]
    COC_NO = 0x0,
    #[doc = "Finished; data available"]
    COC_YES = 0x01,
}
impl Coc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc0 {
    #[inline(always)]
    fn from(val: u8) -> Coc0 {
        Coc0::from_bits(val)
    }
}
impl From<Coc0> for u8 {
    #[inline(always)]
    fn from(val: Coc0) -> u8 {
        Coc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc1 {
    #[doc = "Not finished; data not available"]
    COC_NO = 0x0,
    #[doc = "Finished; data available"]
    COC_YES = 0x01,
}
impl Coc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc1 {
    #[inline(always)]
    fn from(val: u8) -> Coc1 {
        Coc1::from_bits(val)
    }
}
impl From<Coc1> for u8 {
    #[inline(always)]
    fn from(val: Coc1) -> u8 {
        Coc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc2 {
    #[doc = "Not finished; data not available"]
    COC_NO = 0x0,
    #[doc = "Finished; data available"]
    COC_YES = 0x01,
}
impl Coc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc2 {
    #[inline(always)]
    fn from(val: u8) -> Coc2 {
        Coc2::from_bits(val)
    }
}
impl From<Coc2> for u8 {
    #[inline(always)]
    fn from(val: Coc2) -> u8 {
        Coc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc3 {
    #[doc = "Not finished; data not available"]
    COC_NO = 0x0,
    #[doc = "Finished; data available"]
    COC_YES = 0x01,
}
impl Coc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc3 {
    #[inline(always)]
    fn from(val: u8) -> Coc3 {
        Coc3::from_bits(val)
    }
}
impl From<Coc3> for u8 {
    #[inline(always)]
    fn from(val: Coc3) -> u8 {
        Coc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc4 {
    #[doc = "Not finished; data not available"]
    COC_NO = 0x0,
    #[doc = "Finished; data available"]
    COC_YES = 0x01,
}
impl Coc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc4 {
    #[inline(always)]
    fn from(val: u8) -> Coc4 {
        Coc4::from_bits(val)
    }
}
impl From<Coc4> for u8 {
    #[inline(always)]
    fn from(val: Coc4) -> u8 {
        Coc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cocie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie0 {
    #[inline(always)]
    fn from(val: u8) -> Cocie0 {
        Cocie0::from_bits(val)
    }
}
impl From<Cocie0> for u8 {
    #[inline(always)]
    fn from(val: Cocie0) -> u8 {
        Cocie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cocie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie1 {
    #[inline(always)]
    fn from(val: u8) -> Cocie1 {
        Cocie1::from_bits(val)
    }
}
impl From<Cocie1> for u8 {
    #[inline(always)]
    fn from(val: Cocie1) -> u8 {
        Cocie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cocie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie2 {
    #[inline(always)]
    fn from(val: u8) -> Cocie2 {
        Cocie2::from_bits(val)
    }
}
impl From<Cocie2> for u8 {
    #[inline(always)]
    fn from(val: Cocie2) -> u8 {
        Cocie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cocie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie3 {
    #[inline(always)]
    fn from(val: u8) -> Cocie3 {
        Cocie3::from_bits(val)
    }
}
impl From<Cocie3> for u8 {
    #[inline(always)]
    fn from(val: Cocie3) -> u8 {
        Cocie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Cocie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie4 {
    #[inline(always)]
    fn from(val: u8) -> Cocie4 {
        Cocie4::from_bits(val)
    }
}
impl From<Cocie4> for u8 {
    #[inline(always)]
    fn from(val: Cocie4) -> u8 {
        Cocie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgrs {
    #[doc = "Valid"]
    VALID = 0x0,
    #[doc = "Invalid"]
    INVALID_1 = 0x01,
    #[doc = "Invalid"]
    INVALID_2 = 0x02,
    #[doc = "Invalid"]
    INVALID_3 = 0x03,
}
impl Dbgrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgrs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgrs {
    #[inline(always)]
    fn from(val: u8) -> Dbgrs {
        Dbgrs::from_bits(val)
    }
}
impl From<Dbgrs> for u8 {
    #[inline(always)]
    fn from(val: Dbgrs) -> u8 {
        Dbgrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgsel {
    #[doc = "Final data from the PF (24 bits)"]
    RSLT = 0x0,
    #[doc = "Offset data (24 bits)"]
    PFBIS = 0x01,
    #[doc = "Shifted data from the PF (24 bits)"]
    PFSFT = 0x02,
    #[doc = "DC remover (HPF) data (32 bits)"]
    HPF = 0x03,
    #[doc = "Raw data from the PF's CIC filter"]
    PFCIC = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Historical data from SCD"]
    SCD = 0x06,
    #[doc = "Data from the Manchester decoder"]
    MM = 0x07,
    #[doc = "Data from CAD"]
    CAD = 0x08,
    #[doc = "Number of available entries in the FIFO"]
    FIFO = 0x09,
    #[doc = "Status of the parallel or serial data converter"]
    PS = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dbgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgsel {
    #[inline(always)]
    fn from(val: u8) -> Dbgsel {
        Dbgsel::from_bits(val)
    }
}
impl From<Dbgsel> for u8 {
    #[inline(always)]
    fn from(val: Dbgsel) -> u8 {
        Dbgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty0 {
    #[doc = "Not empty"]
    EMPTY_NO = 0x0,
    #[doc = "Empty"]
    EMPTY_YES = 0x01,
}
impl Fifoempty0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty0 {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty0 {
        Fifoempty0::from_bits(val)
    }
}
impl From<Fifoempty0> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty0) -> u8 {
        Fifoempty0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty1 {
    #[doc = "Not empty"]
    EMPTY_NO = 0x0,
    #[doc = "Empty"]
    EMPTY_YES = 0x01,
}
impl Fifoempty1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty1 {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty1 {
        Fifoempty1::from_bits(val)
    }
}
impl From<Fifoempty1> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty1) -> u8 {
        Fifoempty1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty2 {
    #[doc = "Not empty"]
    EMPTY_NO = 0x0,
    #[doc = "Empty"]
    EMPTY_YES = 0x01,
}
impl Fifoempty2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty2 {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty2 {
        Fifoempty2::from_bits(val)
    }
}
impl From<Fifoempty2> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty2) -> u8 {
        Fifoempty2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty3 {
    #[doc = "Not empty"]
    EMPTY_NO = 0x0,
    #[doc = "Empty"]
    EMPTY_YES = 0x01,
}
impl Fifoempty3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty3 {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty3 {
        Fifoempty3::from_bits(val)
    }
}
impl From<Fifoempty3> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty3) -> u8 {
        Fifoempty3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty4 {
    #[doc = "Not empty"]
    EMPTY_NO = 0x0,
    #[doc = "Empty"]
    EMPTY_YES = 0x01,
}
impl Fifoempty4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty4 {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty4 {
        Fifoempty4::from_bits(val)
    }
}
impl From<Fifoempty4> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty4) -> u8 {
        Fifoempty4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoen {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
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
pub enum Fovf0 {
    #[doc = "Did not occur"]
    FOVF_NO = 0x0,
    #[doc = "Occurred"]
    FOVF_YES = 0x01,
}
impl Fovf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf0 {
    #[inline(always)]
    fn from(val: u8) -> Fovf0 {
        Fovf0::from_bits(val)
    }
}
impl From<Fovf0> for u8 {
    #[inline(always)]
    fn from(val: Fovf0) -> u8 {
        Fovf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovf1 {
    #[doc = "Did not occur"]
    FOVF_NO = 0x0,
    #[doc = "Occurred"]
    FOVF_YES = 0x01,
}
impl Fovf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf1 {
    #[inline(always)]
    fn from(val: u8) -> Fovf1 {
        Fovf1::from_bits(val)
    }
}
impl From<Fovf1> for u8 {
    #[inline(always)]
    fn from(val: Fovf1) -> u8 {
        Fovf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovf2 {
    #[doc = "Did not occur"]
    FOVF_NO = 0x0,
    #[doc = "Occurred"]
    FOVF_YES = 0x01,
}
impl Fovf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf2 {
    #[inline(always)]
    fn from(val: u8) -> Fovf2 {
        Fovf2::from_bits(val)
    }
}
impl From<Fovf2> for u8 {
    #[inline(always)]
    fn from(val: Fovf2) -> u8 {
        Fovf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovf3 {
    #[doc = "Did not occur"]
    FOVF_NO = 0x0,
    #[doc = "Occurred"]
    FOVF_YES = 0x01,
}
impl Fovf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf3 {
    #[inline(always)]
    fn from(val: u8) -> Fovf3 {
        Fovf3::from_bits(val)
    }
}
impl From<Fovf3> for u8 {
    #[inline(always)]
    fn from(val: Fovf3) -> u8 {
        Fovf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovf4 {
    #[doc = "Did not occur"]
    FOVF_NO = 0x0,
    #[doc = "Occurred"]
    FOVF_YES = 0x01,
}
impl Fovf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf4 {
    #[inline(always)]
    fn from(val: u8) -> Fovf4 {
        Fovf4::from_bits(val)
    }
}
impl From<Fovf4> for u8 {
    #[inline(always)]
    fn from(val: Fovf4) -> u8 {
        Fovf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Fovfie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie0 {
    #[inline(always)]
    fn from(val: u8) -> Fovfie0 {
        Fovfie0::from_bits(val)
    }
}
impl From<Fovfie0> for u8 {
    #[inline(always)]
    fn from(val: Fovfie0) -> u8 {
        Fovfie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Fovfie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie1 {
    #[inline(always)]
    fn from(val: u8) -> Fovfie1 {
        Fovfie1::from_bits(val)
    }
}
impl From<Fovfie1> for u8 {
    #[inline(always)]
    fn from(val: Fovfie1) -> u8 {
        Fovfie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Fovfie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie2 {
    #[inline(always)]
    fn from(val: u8) -> Fovfie2 {
        Fovfie2::from_bits(val)
    }
}
impl From<Fovfie2> for u8 {
    #[inline(always)]
    fn from(val: Fovfie2) -> u8 {
        Fovfie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Fovfie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie3 {
    #[inline(always)]
    fn from(val: u8) -> Fovfie3 {
        Fovfie3::from_bits(val)
    }
}
impl From<Fovfie3> for u8 {
    #[inline(always)]
    fn from(val: Fovfie3) -> u8 {
        Fovfie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Fovfie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie4 {
    #[inline(always)]
    fn from(val: u8) -> Fovfie4 {
        Fovfie4::from_bits(val)
    }
}
impl From<Fovfie4> for u8 {
    #[inline(always)]
    fn from(val: Fovfie4) -> u8 {
        Fovfie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf0 {
    #[doc = "Did not occur"]
    FUNF_NO = 0x0,
    #[doc = "Occurred"]
    FUNF_YES = 0x01,
}
impl Funf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf0 {
    #[inline(always)]
    fn from(val: u8) -> Funf0 {
        Funf0::from_bits(val)
    }
}
impl From<Funf0> for u8 {
    #[inline(always)]
    fn from(val: Funf0) -> u8 {
        Funf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf1 {
    #[doc = "Did not occur"]
    FUNF_NO = 0x0,
    #[doc = "Occurred"]
    FUNF_YES = 0x01,
}
impl Funf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf1 {
    #[inline(always)]
    fn from(val: u8) -> Funf1 {
        Funf1::from_bits(val)
    }
}
impl From<Funf1> for u8 {
    #[inline(always)]
    fn from(val: Funf1) -> u8 {
        Funf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf2 {
    #[doc = "Did not occur"]
    FUNF_NO = 0x0,
    #[doc = "Occurred"]
    FUNF_YES = 0x01,
}
impl Funf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf2 {
    #[inline(always)]
    fn from(val: u8) -> Funf2 {
        Funf2::from_bits(val)
    }
}
impl From<Funf2> for u8 {
    #[inline(always)]
    fn from(val: Funf2) -> u8 {
        Funf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf3 {
    #[doc = "Did not occur"]
    FUNF_NO = 0x0,
    #[doc = "Occurred"]
    FUNF_YES = 0x01,
}
impl Funf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf3 {
    #[inline(always)]
    fn from(val: u8) -> Funf3 {
        Funf3::from_bits(val)
    }
}
impl From<Funf3> for u8 {
    #[inline(always)]
    fn from(val: Funf3) -> u8 {
        Funf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf4 {
    #[doc = "Did not occur"]
    FUNF_NO = 0x0,
    #[doc = "Occurred"]
    FUNF_YES = 0x01,
}
impl Funf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf4 {
    #[inline(always)]
    fn from(val: u8) -> Funf4 {
        Funf4::from_bits(val)
    }
}
impl From<Funf4> for u8 {
    #[inline(always)]
    fn from(val: Funf4) -> u8 {
        Funf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Funfie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie0 {
    #[inline(always)]
    fn from(val: u8) -> Funfie0 {
        Funfie0::from_bits(val)
    }
}
impl From<Funfie0> for u8 {
    #[inline(always)]
    fn from(val: Funfie0) -> u8 {
        Funfie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Funfie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie1 {
    #[inline(always)]
    fn from(val: u8) -> Funfie1 {
        Funfie1::from_bits(val)
    }
}
impl From<Funfie1> for u8 {
    #[inline(always)]
    fn from(val: Funfie1) -> u8 {
        Funfie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Funfie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie2 {
    #[inline(always)]
    fn from(val: u8) -> Funfie2 {
        Funfie2::from_bits(val)
    }
}
impl From<Funfie2> for u8 {
    #[inline(always)]
    fn from(val: Funfie2) -> u8 {
        Funfie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Funfie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie3 {
    #[inline(always)]
    fn from(val: u8) -> Funfie3 {
        Funfie3::from_bits(val)
    }
}
impl From<Funfie3> for u8 {
    #[inline(always)]
    fn from(val: Funfie3) -> u8 {
        Funfie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Funfie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie4 {
    #[inline(always)]
    fn from(val: u8) -> Funfie4 {
        Funfie4::from_bits(val)
    }
}
impl From<Funfie4> for u8 {
    #[inline(always)]
    fn from(val: Funfie4) -> u8 {
        Funfie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt0 {
    #[doc = "Not exceeded"]
    HLMT_NO = 0x0,
    #[doc = "Exceeded"]
    HLMT_YES = 0x01,
}
impl Hlmt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt0 {
    #[inline(always)]
    fn from(val: u8) -> Hlmt0 {
        Hlmt0::from_bits(val)
    }
}
impl From<Hlmt0> for u8 {
    #[inline(always)]
    fn from(val: Hlmt0) -> u8 {
        Hlmt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt1 {
    #[doc = "Not exceeded"]
    HLMT_NO = 0x0,
    #[doc = "Exceeded"]
    HLMT_YES = 0x01,
}
impl Hlmt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt1 {
    #[inline(always)]
    fn from(val: u8) -> Hlmt1 {
        Hlmt1::from_bits(val)
    }
}
impl From<Hlmt1> for u8 {
    #[inline(always)]
    fn from(val: Hlmt1) -> u8 {
        Hlmt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt2 {
    #[doc = "Not exceeded"]
    HLMT_NO = 0x0,
    #[doc = "Exceeded"]
    HLMT_YES = 0x01,
}
impl Hlmt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt2 {
    #[inline(always)]
    fn from(val: u8) -> Hlmt2 {
        Hlmt2::from_bits(val)
    }
}
impl From<Hlmt2> for u8 {
    #[inline(always)]
    fn from(val: Hlmt2) -> u8 {
        Hlmt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt3 {
    #[doc = "Not exceeded"]
    HLMT_NO = 0x0,
    #[doc = "Exceeded"]
    HLMT_YES = 0x01,
}
impl Hlmt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt3 {
    #[inline(always)]
    fn from(val: u8) -> Hlmt3 {
        Hlmt3::from_bits(val)
    }
}
impl From<Hlmt3> for u8 {
    #[inline(always)]
    fn from(val: Hlmt3) -> u8 {
        Hlmt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt4 {
    #[doc = "Not exceeded"]
    HLMT_NO = 0x0,
    #[doc = "Exceeded"]
    HLMT_YES = 0x01,
}
impl Hlmt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt4 {
    #[inline(always)]
    fn from(val: u8) -> Hlmt4 {
        Hlmt4::from_bits(val)
    }
}
impl From<Hlmt4> for u8 {
    #[inline(always)]
    fn from(val: Hlmt4) -> u8 {
        Hlmt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtbk {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtbk {
    #[inline(always)]
    fn from(val: u8) -> Hlmtbk {
        Hlmtbk::from_bits(val)
    }
}
impl From<Hlmtbk> for u8 {
    #[inline(always)]
    fn from(val: Hlmtbk) -> u8 {
        Hlmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie0 {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie0 {
        Hlmtie0::from_bits(val)
    }
}
impl From<Hlmtie0> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie0) -> u8 {
        Hlmtie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie1 {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie1 {
        Hlmtie1::from_bits(val)
    }
}
impl From<Hlmtie1> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie1) -> u8 {
        Hlmtie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie2 {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie2 {
        Hlmtie2::from_bits(val)
    }
}
impl From<Hlmtie2> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie2) -> u8 {
        Hlmtie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie3 {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie3 {
        Hlmtie3::from_bits(val)
    }
}
impl From<Hlmtie3> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie3) -> u8 {
        Hlmtie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Hlmtie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie4 {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie4 {
        Hlmtie4::from_bits(val)
    }
}
impl From<Hlmtie4> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie4) -> u8 {
        Hlmtie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpfsat {
    #[doc = "Did not occur"]
    SAT_NO = 0x0,
    #[doc = "Occurred"]
    SAT_YES = 0x01,
}
impl Hpfsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpfsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpfsat {
    #[inline(always)]
    fn from(val: u8) -> Hpfsat {
        Hpfsat::from_bits(val)
    }
}
impl From<Hpfsat> for u8 {
    #[inline(always)]
    fn from(val: Hpfsat) -> u8 {
        Hpfsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibdly {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
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
impl Ibdly {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibdly {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibdly {
    #[inline(always)]
    fn from(val: u8) -> Ibdly {
        Ibdly::from_bits(val)
    }
}
impl From<Ibdly> for u8 {
    #[inline(always)]
    fn from(val: Ibdly) -> u8 {
        Ibdly::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibfmt {
    #[doc = "External bitstream from the MBIT\\[n\\] signal"]
    E1B = 0x0,
    #[doc = "External Manchester code; ICESEL selects the rise or fall decoder"]
    EMB = 0x01,
    #[doc = "Internal 16-bit parallel data from MPDATA"]
    IPB = 0x02,
    #[doc = "Internal 32-bit serial data from MPDATA"]
    ISB = 0x03,
}
impl Ibfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibfmt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibfmt {
    #[inline(always)]
    fn from(val: u8) -> Ibfmt {
        Ibfmt::from_bits(val)
    }
}
impl From<Ibfmt> for u8 {
    #[inline(always)]
    fn from(val: Ibfmt) -> u8 {
        Ibfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibsel {
    #[doc = "External bitstream from the MBIT\\[n\\] signal"]
    EPB = 0x0,
    #[doc = "Alternate internal bitstream from the INP\\[n\\] signal"]
    ESB = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Grouped bitstream shared with an adjacent channel; the adjacent channel's IBSEL field determines the input"]
    GRP = 0x03,
}
impl Ibsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibsel {
    #[inline(always)]
    fn from(val: u8) -> Ibsel {
        Ibsel::from_bits(val)
    }
}
impl From<Ibsel> for u8 {
    #[inline(always)]
    fn from(val: Ibsel) -> u8 {
        Ibsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icesel {
    _RESERVED_0 = 0x0,
    #[doc = "Positive edge"]
    POS = 0x01,
    #[doc = "Negative edge"]
    NEG = 0x02,
    #[doc = "Both edges"]
    BOTH = 0x03,
    #[doc = "Every other odd positive edge"]
    OPOS = 0x04,
    #[doc = "Every other even positive edge"]
    EPOS = 0x05,
    #[doc = "Every other odd negative edge"]
    ONEG = 0x06,
    #[doc = "Every other even negative edge"]
    ENEG = 0x07,
}
impl Icesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icesel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icesel {
    #[inline(always)]
    fn from(val: u8) -> Icesel {
        Icesel::from_bits(val)
    }
}
impl From<Icesel> for u8 {
    #[inline(always)]
    fn from(val: Icesel) -> u8 {
        Icesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icsel {
    #[doc = "MCLK_OUT0 with internal routeback"]
    MCLK_OUT0 = 0x0,
    #[doc = "MCLK_OUT1 with internal routeback"]
    MCLK_OUT1 = 0x01,
    #[doc = "MCLK_OUT2 with internal routeback"]
    MCLK_OUT2 = 0x02,
    #[doc = "External modulator clock dedicated to this channel"]
    EXT = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Grouped clock shared with an adjacent channel; the adjacent channel's ICSEL field determines the input clock"]
    GRP = 0x07,
}
impl Icsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icsel {
    #[inline(always)]
    fn from(val: u8) -> Icsel {
        Icsel::from_bits(val)
    }
}
impl From<Icsel> for u8 {
    #[inline(always)]
    fn from(val: Icsel) -> u8 {
        Icsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itlvl {
    #[doc = "Edge"]
    EDGE = 0x0,
    #[doc = "Level"]
    LEVEL = 0x01,
}
impl Itlvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itlvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itlvl {
    #[inline(always)]
    fn from(val: u8) -> Itlvl {
        Itlvl::from_bits(val)
    }
}
impl From<Itlvl> for u8 {
    #[inline(always)]
    fn from(val: Itlvl) -> u8 {
        Itlvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itsel {
    #[doc = "Software"]
    SW = 0x0,
    #[doc = "Hardware trigger dedicated to the channel"]
    HW = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Grouped trigger shared with an adjacent channel; the adjacent channel's ITSEL field determines the trigger"]
    GP = 0x03,
}
impl Itsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itsel {
    #[inline(always)]
    fn from(val: u8) -> Itsel {
        Itsel::from_bits(val)
    }
}
impl From<Itsel> for u8 {
    #[inline(always)]
    fn from(val: Itsel) -> u8 {
        Itsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt0 {
    #[doc = "Not exceeded"]
    LLMT_NO = 0x0,
    #[doc = "Exceeded"]
    LLMT_YES = 0x01,
}
impl Llmt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt0 {
    #[inline(always)]
    fn from(val: u8) -> Llmt0 {
        Llmt0::from_bits(val)
    }
}
impl From<Llmt0> for u8 {
    #[inline(always)]
    fn from(val: Llmt0) -> u8 {
        Llmt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt1 {
    #[doc = "Not exceeded"]
    LLMT_NO = 0x0,
    #[doc = "Exceeded"]
    LLMT_YES = 0x01,
}
impl Llmt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt1 {
    #[inline(always)]
    fn from(val: u8) -> Llmt1 {
        Llmt1::from_bits(val)
    }
}
impl From<Llmt1> for u8 {
    #[inline(always)]
    fn from(val: Llmt1) -> u8 {
        Llmt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt2 {
    #[doc = "Not exceeded"]
    LLMT_NO = 0x0,
    #[doc = "Exceeded"]
    LLMT_YES = 0x01,
}
impl Llmt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt2 {
    #[inline(always)]
    fn from(val: u8) -> Llmt2 {
        Llmt2::from_bits(val)
    }
}
impl From<Llmt2> for u8 {
    #[inline(always)]
    fn from(val: Llmt2) -> u8 {
        Llmt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt3 {
    #[doc = "Not exceeded"]
    LLMT_NO = 0x0,
    #[doc = "Exceeded"]
    LLMT_YES = 0x01,
}
impl Llmt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt3 {
    #[inline(always)]
    fn from(val: u8) -> Llmt3 {
        Llmt3::from_bits(val)
    }
}
impl From<Llmt3> for u8 {
    #[inline(always)]
    fn from(val: Llmt3) -> u8 {
        Llmt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt4 {
    #[doc = "Not exceeded"]
    LLMT_NO = 0x0,
    #[doc = "Exceeded"]
    LLMT_YES = 0x01,
}
impl Llmt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt4 {
    #[inline(always)]
    fn from(val: u8) -> Llmt4 {
        Llmt4::from_bits(val)
    }
}
impl From<Llmt4> for u8 {
    #[inline(always)]
    fn from(val: Llmt4) -> u8 {
        Llmt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtbk {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtbk {
    #[inline(always)]
    fn from(val: u8) -> Llmtbk {
        Llmtbk::from_bits(val)
    }
}
impl From<Llmtbk> for u8 {
    #[inline(always)]
    fn from(val: Llmtbk) -> u8 {
        Llmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie0 {
    #[inline(always)]
    fn from(val: u8) -> Llmtie0 {
        Llmtie0::from_bits(val)
    }
}
impl From<Llmtie0> for u8 {
    #[inline(always)]
    fn from(val: Llmtie0) -> u8 {
        Llmtie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie1 {
    #[inline(always)]
    fn from(val: u8) -> Llmtie1 {
        Llmtie1::from_bits(val)
    }
}
impl From<Llmtie1> for u8 {
    #[inline(always)]
    fn from(val: Llmtie1) -> u8 {
        Llmtie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie2 {
    #[inline(always)]
    fn from(val: u8) -> Llmtie2 {
        Llmtie2::from_bits(val)
    }
}
impl From<Llmtie2> for u8 {
    #[inline(always)]
    fn from(val: Llmtie2) -> u8 {
        Llmtie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie3 {
    #[inline(always)]
    fn from(val: u8) -> Llmtie3 {
        Llmtie3::from_bits(val)
    }
}
impl From<Llmtie3> for u8 {
    #[inline(always)]
    fn from(val: Llmtie3) -> u8 {
        Llmtie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Llmtie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie4 {
    #[inline(always)]
    fn from(val: u8) -> Llmtie4 {
        Llmtie4::from_bits(val)
    }
}
impl From<Llmtie4> for u8 {
    #[inline(always)]
    fn from(val: Llmtie4) -> u8 {
        Llmtie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lmten {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Lmten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lmten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lmten {
    #[inline(always)]
    fn from(val: u8) -> Lmten {
        Lmten::from_bits(val)
    }
}
impl From<Lmten> for u8 {
    #[inline(always)]
    fn from(val: Lmten) -> u8 {
        Lmten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lmtop {
    #[doc = "Both high and low limits"]
    BOTH = 0x0,
    #[doc = "High limit"]
    HIGH = 0x01,
    #[doc = "Low limit"]
    LOW = 0x02,
    #[doc = "Windowed value"]
    WINDOW = 0x03,
}
impl Lmtop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lmtop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lmtop {
    #[inline(always)]
    fn from(val: u8) -> Lmtop {
        Lmtop::from_bits(val)
    }
}
impl From<Lmtop> for u8 {
    #[inline(always)]
    fn from(val: Lmtop) -> u8 {
        Lmtop::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Major(u8);
impl Major {
    #[doc = "1.x"]
    pub const VER_1: Self = Self(0x01);
    #[doc = "2.x"]
    pub const VER_2: Self = Self(0x02);
}
impl Major {
    pub const fn from_bits(val: u8) -> Major {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Major {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VER_1"),
            0x02 => f.write_str("VER_2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Major {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VER_1"),
            0x02 => defmt::write!(f, "VER_2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Major {
    #[inline(always)]
    fn from(val: u8) -> Major {
        Major::from_bits(val)
    }
}
impl From<Major> for u8 {
    #[inline(always)]
    fn from(val: Major) -> u8 {
        Major::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclk0dis {
    #[doc = "Enabled when MEN = 1"]
    ENABLED = 0x0,
    #[doc = "Disabled regardless of MEN value"]
    DISABLED = 0x01,
}
impl Mclk0dis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclk0dis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclk0dis {
    #[inline(always)]
    fn from(val: u8) -> Mclk0dis {
        Mclk0dis::from_bits(val)
    }
}
impl From<Mclk0dis> for u8 {
    #[inline(always)]
    fn from(val: Mclk0dis) -> u8 {
        Mclk0dis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclk1dis {
    #[doc = "Enabled when MEN = 1"]
    ENABLED = 0x0,
    #[doc = "Disabled regardless of MEN value"]
    DISABLED = 0x01,
}
impl Mclk1dis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclk1dis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclk1dis {
    #[inline(always)]
    fn from(val: u8) -> Mclk1dis {
        Mclk1dis::from_bits(val)
    }
}
impl From<Mclk1dis> for u8 {
    #[inline(always)]
    fn from(val: Mclk1dis) -> u8 {
        Mclk1dis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclk2dis {
    #[doc = "Enabled when MEN = 1"]
    ENABLED = 0x0,
    #[doc = "Disabled regardless of MEN value"]
    DISABLED = 0x01,
}
impl Mclk2dis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclk2dis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclk2dis {
    #[inline(always)]
    fn from(val: u8) -> Mclk2dis {
        Mclk2dis::from_bits(val)
    }
}
impl From<Mclk2dis> for u8 {
    #[inline(always)]
    fn from(val: Mclk2dis) -> u8 {
        Mclk2dis::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mclkdiv(u8);
impl Mclkdiv {
    #[doc = "Prohibited"]
    pub const PROHIBITED: Self = Self(0x0);
}
impl Mclkdiv {
    pub const fn from_bits(val: u8) -> Mclkdiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Mclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("PROHIBITED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkdiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "PROHIBITED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Mclkdiv {
        Mclkdiv::from_bits(val)
    }
}
impl From<Mclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Mclkdiv) -> u8 {
        Mclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkrdy0 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Mclkrdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkrdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkrdy0 {
    #[inline(always)]
    fn from(val: u8) -> Mclkrdy0 {
        Mclkrdy0::from_bits(val)
    }
}
impl From<Mclkrdy0> for u8 {
    #[inline(always)]
    fn from(val: Mclkrdy0) -> u8 {
        Mclkrdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkrdy1 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Mclkrdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkrdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkrdy1 {
    #[inline(always)]
    fn from(val: u8) -> Mclkrdy1 {
        Mclkrdy1::from_bits(val)
    }
}
impl From<Mclkrdy1> for u8 {
    #[inline(always)]
    fn from(val: Mclkrdy1) -> u8 {
        Mclkrdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkrdy2 {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Mclkrdy2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkrdy2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkrdy2 {
    #[inline(always)]
    fn from(val: u8) -> Mclkrdy2 {
        Mclkrdy2::from_bits(val)
    }
}
impl From<Mclkrdy2> for u8 {
    #[inline(always)]
    fn from(val: Mclkrdy2) -> u8 {
        Mclkrdy2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Men {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Men {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Men {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Men {
    #[inline(always)]
    fn from(val: u8) -> Men {
        Men::from_bits(val)
    }
}
impl From<Men> for u8 {
    #[inline(always)]
    fn from(val: Men) -> u8 {
        Men::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Minor(u8);
impl Minor {
    #[doc = "x.0"]
    pub const VER_2_0: Self = Self(0x0);
}
impl Minor {
    pub const fn from_bits(val: u8) -> Minor {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Minor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("VER_2_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Minor {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "VER_2_0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Minor {
    #[inline(always)]
    fn from(val: u8) -> Minor {
        Minor::from_bits(val)
    }
}
impl From<Minor> for u8 {
    #[inline(always)]
    fn from(val: Minor) -> u8 {
        Minor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfOrdSel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "3"]
    ORDER_3 = 0x02,
    #[doc = "2"]
    ORDER_2 = 0x03,
}
impl PfOrdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfOrdSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfOrdSel {
    #[inline(always)]
    fn from(val: u8) -> PfOrdSel {
        PfOrdSel::from_bits(val)
    }
}
impl From<PfOrdSel> for u8 {
    #[inline(always)]
    fn from(val: PfOrdSel) -> u8 {
        PfOrdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfcm {
    #[doc = "Single"]
    SINGLE = 0x0,
    #[doc = "Continuous"]
    CONTINUOUS = 0x01,
    #[doc = "Always"]
    ALWAYS = 0x02,
    #[doc = "Fixed number"]
    FIX = 0x03,
}
impl Pfcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfcm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfcm {
    #[inline(always)]
    fn from(val: u8) -> Pfcm {
        Pfcm::from_bits(val)
    }
}
impl From<Pfcm> for u8 {
    #[inline(always)]
    fn from(val: Pfcm) -> u8 {
        Pfcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfen {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Pfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfen {
    #[inline(always)]
    fn from(val: u8) -> Pfen {
        Pfen::from_bits(val)
    }
}
impl From<Pfen> for u8 {
    #[inline(always)]
    fn from(val: Pfen) -> u8 {
        Pfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pford {
    #[doc = "FastSinc"]
    FASTSINC = 0x0,
    #[doc = "First order"]
    ORDER_1 = 0x01,
    #[doc = "Second order"]
    ORDER_2 = 0x02,
    #[doc = "Third order"]
    ORDER_3 = 0x03,
}
impl Pford {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pford {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pford {
    #[inline(always)]
    fn from(val: u8) -> Pford {
        Pford::from_bits(val)
    }
}
impl From<Pford> for u8 {
    #[inline(always)]
    fn from(val: Pford) -> u8 {
        Pford::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsat {
    #[doc = "Did not occur"]
    SAT_NO = 0x0,
    #[doc = "Occurred"]
    SAT_YES = 0x01,
}
impl Pfsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsat {
    #[inline(always)]
    fn from(val: u8) -> Pfsat {
        Pfsat::from_bits(val)
    }
}
impl From<Pfsat> for u8 {
    #[inline(always)]
    fn from(val: Pfsat) -> u8 {
        Pfsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "No prescale"]
    PRESCALE_NO = 0x0,
    #[doc = "2"]
    PRESCALE_2 = 0x01,
    #[doc = "4"]
    PRESCALE_4 = 0x02,
    #[doc = "8"]
    PRESCALE_8 = 0x03,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psrdy {
    #[doc = "Not ready"]
    READY_NO = 0x0,
    #[doc = "Ready"]
    READY_YES = 0x01,
}
impl Psrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psrdy {
    #[inline(always)]
    fn from(val: u8) -> Psrdy {
        Psrdy::from_bits(val)
    }
}
impl From<Psrdy> for u8 {
    #[inline(always)]
    fn from(val: Psrdy) -> u8 {
        Psrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdfmt {
    #[doc = "Left justified, signed"]
    SIGNED = 0x0,
    #[doc = "Left justified, unsigned"]
    UNSIGNED = 0x01,
}
impl Rdfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdfmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdfmt {
    #[inline(always)]
    fn from(val: u8) -> Rdfmt {
        Rdfmt::from_bits(val)
    }
}
impl From<Rdfmt> for u8 {
    #[inline(always)]
    fn from(val: Rdfmt) -> u8 {
        Rdfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdrs {
    #[doc = "Valid"]
    VALID = 0x0,
    #[doc = "Invalid"]
    INVALID = 0x01,
}
impl Rdrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdrs {
    #[inline(always)]
    fn from(val: u8) -> Rdrs {
        Rdrs::from_bits(val)
    }
}
impl From<Rdrs> for u8 {
    #[inline(always)]
    fn from(val: Rdrs) -> u8 {
        Rdrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "Do not reset"]
    RESET_NO = 0x0,
    #[doc = "Reset"]
    RESET_YES = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat0 {
    #[doc = "Not saturated"]
    SAT_NO = 0x0,
    #[doc = "Saturated"]
    SAT_YES = 0x01,
}
impl Sat0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat0 {
    #[inline(always)]
    fn from(val: u8) -> Sat0 {
        Sat0::from_bits(val)
    }
}
impl From<Sat0> for u8 {
    #[inline(always)]
    fn from(val: Sat0) -> u8 {
        Sat0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat1 {
    #[doc = "Not saturated"]
    SAT_NO = 0x0,
    #[doc = "Saturated"]
    SAT_YES = 0x01,
}
impl Sat1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat1 {
    #[inline(always)]
    fn from(val: u8) -> Sat1 {
        Sat1::from_bits(val)
    }
}
impl From<Sat1> for u8 {
    #[inline(always)]
    fn from(val: Sat1) -> u8 {
        Sat1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat2 {
    #[doc = "Not saturated"]
    SAT_NO = 0x0,
    #[doc = "Saturated"]
    SAT_YES = 0x01,
}
impl Sat2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat2 {
    #[inline(always)]
    fn from(val: u8) -> Sat2 {
        Sat2::from_bits(val)
    }
}
impl From<Sat2> for u8 {
    #[inline(always)]
    fn from(val: Sat2) -> u8 {
        Sat2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat3 {
    #[doc = "Not saturated"]
    SAT_NO = 0x0,
    #[doc = "Saturated"]
    SAT_YES = 0x01,
}
impl Sat3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat3 {
    #[inline(always)]
    fn from(val: u8) -> Sat3 {
        Sat3::from_bits(val)
    }
}
impl From<Sat3> for u8 {
    #[inline(always)]
    fn from(val: Sat3) -> u8 {
        Sat3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat4 {
    #[doc = "Not saturated"]
    SAT_NO = 0x0,
    #[doc = "Saturated"]
    SAT_YES = 0x01,
}
impl Sat4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat4 {
    #[inline(always)]
    fn from(val: u8) -> Sat4 {
        Sat4::from_bits(val)
    }
}
impl From<Sat4> for u8 {
    #[inline(always)]
    fn from(val: Sat4) -> u8 {
        Sat4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Satie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie0 {
    #[inline(always)]
    fn from(val: u8) -> Satie0 {
        Satie0::from_bits(val)
    }
}
impl From<Satie0> for u8 {
    #[inline(always)]
    fn from(val: Satie0) -> u8 {
        Satie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Satie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie1 {
    #[inline(always)]
    fn from(val: u8) -> Satie1 {
        Satie1::from_bits(val)
    }
}
impl From<Satie1> for u8 {
    #[inline(always)]
    fn from(val: Satie1) -> u8 {
        Satie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Satie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie2 {
    #[inline(always)]
    fn from(val: u8) -> Satie2 {
        Satie2::from_bits(val)
    }
}
impl From<Satie2> for u8 {
    #[inline(always)]
    fn from(val: Satie2) -> u8 {
        Satie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Satie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie3 {
    #[inline(always)]
    fn from(val: u8) -> Satie3 {
        Satie3::from_bits(val)
    }
}
impl From<Satie3> for u8 {
    #[inline(always)]
    fn from(val: Satie3) -> u8 {
        Satie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Satie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie4 {
    #[inline(always)]
    fn from(val: u8) -> Satie4 {
        Satie4::from_bits(val)
    }
}
impl From<Satie4> for u8 {
    #[inline(always)]
    fn from(val: Satie4) -> u8 {
        Satie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd0 {
    #[doc = "Not detected"]
    SC_NO = 0x0,
    #[doc = "Detected"]
    SC_YES = 0x01,
}
impl Scd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd0 {
    #[inline(always)]
    fn from(val: u8) -> Scd0 {
        Scd0::from_bits(val)
    }
}
impl From<Scd0> for u8 {
    #[inline(always)]
    fn from(val: Scd0) -> u8 {
        Scd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd1 {
    #[doc = "Not detected"]
    SC_NO = 0x0,
    #[doc = "Detected"]
    SC_YES = 0x01,
}
impl Scd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd1 {
    #[inline(always)]
    fn from(val: u8) -> Scd1 {
        Scd1::from_bits(val)
    }
}
impl From<Scd1> for u8 {
    #[inline(always)]
    fn from(val: Scd1) -> u8 {
        Scd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd2 {
    #[doc = "Not detected"]
    SC_NO = 0x0,
    #[doc = "Detected"]
    SC_YES = 0x01,
}
impl Scd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd2 {
    #[inline(always)]
    fn from(val: u8) -> Scd2 {
        Scd2::from_bits(val)
    }
}
impl From<Scd2> for u8 {
    #[inline(always)]
    fn from(val: Scd2) -> u8 {
        Scd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd3 {
    #[doc = "Not detected"]
    SC_NO = 0x0,
    #[doc = "Detected"]
    SC_YES = 0x01,
}
impl Scd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd3 {
    #[inline(always)]
    fn from(val: u8) -> Scd3 {
        Scd3::from_bits(val)
    }
}
impl From<Scd3> for u8 {
    #[inline(always)]
    fn from(val: Scd3) -> u8 {
        Scd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd4 {
    #[doc = "Not detected"]
    SC_NO = 0x0,
    #[doc = "Detected"]
    SC_YES = 0x01,
}
impl Scd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd4 {
    #[inline(always)]
    fn from(val: u8) -> Scd4 {
        Scd4::from_bits(val)
    }
}
impl From<Scd4> for u8 {
    #[inline(always)]
    fn from(val: Scd4) -> u8 {
        Scd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdbk {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdbk {
    #[inline(always)]
    fn from(val: u8) -> Scdbk {
        Scdbk::from_bits(val)
    }
}
impl From<Scdbk> for u8 {
    #[inline(always)]
    fn from(val: Scdbk) -> u8 {
        Scdbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdcm {
    #[doc = "Constantly when CnCR\\[CHEN\\] = MCR\\[MEN\\] = 1"]
    ALWAYS = 0x0,
    #[doc = "Only when the PF is performing a conversion"]
    DURING_CONV = 0x01,
}
impl Scdcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdcm {
    #[inline(always)]
    fn from(val: u8) -> Scdcm {
        Scdcm::from_bits(val)
    }
}
impl From<Scdcm> for u8 {
    #[inline(always)]
    fn from(val: Scdcm) -> u8 {
        Scdcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scden {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scden {
    #[inline(always)]
    fn from(val: u8) -> Scden {
        Scden::from_bits(val)
    }
}
impl From<Scden> for u8 {
    #[inline(always)]
    fn from(val: Scden) -> u8 {
        Scden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie0 {
    #[inline(always)]
    fn from(val: u8) -> Scdie0 {
        Scdie0::from_bits(val)
    }
}
impl From<Scdie0> for u8 {
    #[inline(always)]
    fn from(val: Scdie0) -> u8 {
        Scdie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie1 {
    #[inline(always)]
    fn from(val: u8) -> Scdie1 {
        Scdie1::from_bits(val)
    }
}
impl From<Scdie1> for u8 {
    #[inline(always)]
    fn from(val: Scdie1) -> u8 {
        Scdie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie2 {
    #[inline(always)]
    fn from(val: u8) -> Scdie2 {
        Scdie2::from_bits(val)
    }
}
impl From<Scdie2> for u8 {
    #[inline(always)]
    fn from(val: Scdie2) -> u8 {
        Scdie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie3 {
    #[inline(always)]
    fn from(val: u8) -> Scdie3 {
        Scdie3::from_bits(val)
    }
}
impl From<Scdie3> for u8 {
    #[inline(always)]
    fn from(val: Scdie3) -> u8 {
        Scdie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Scdie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie4 {
    #[inline(always)]
    fn from(val: u8) -> Scdie4 {
        Scdie4::from_bits(val)
    }
}
impl From<Scdie4> for u8 {
    #[inline(always)]
    fn from(val: Scdie4) -> u8 {
        Scdie4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Scdlmt(u8);
impl Scdlmt {
    #[doc = "Disables SCD"]
    pub const DISABLES_0: Self = Self(0x0);
    #[doc = "Disables SCD"]
    pub const DISABLES_1: Self = Self(0x01);
}
impl Scdlmt {
    pub const fn from_bits(val: u8) -> Scdlmt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Scdlmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLES_0"),
            0x01 => f.write_str("DISABLES_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scdlmt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLES_0"),
            0x01 => defmt::write!(f, "DISABLES_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Scdlmt {
    #[inline(always)]
    fn from(val: u8) -> Scdlmt {
        Scdlmt::from_bits(val)
    }
}
impl From<Scdlmt> for u8 {
    #[inline(always)]
    fn from(val: Scdlmt) -> u8 {
        Scdlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdop {
    #[doc = "Both 0 and 1"]
    BOTH = 0x0,
    #[doc = "Only 1"]
    ONE = 0x01,
    #[doc = "Only 0"]
    ZERO = 0x02,
    _RESERVED_3 = 0x03,
}
impl Scdop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdop {
    #[inline(always)]
    fn from(val: u8) -> Scdop {
        Scdop::from_bits(val)
    }
}
impl From<Scdop> for u8 {
    #[inline(always)]
    fn from(val: Scdop) -> u8 {
        Scdop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sftsat {
    #[doc = "Did not occur"]
    SAT_NO = 0x0,
    #[doc = "Occurred"]
    SAT_YES = 0x01,
}
impl Sftsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftsat {
    #[inline(always)]
    fn from(val: u8) -> Sftsat {
        Sftsat::from_bits(val)
    }
}
impl From<Sftsat> for u8 {
    #[inline(always)]
    fn from(val: Sftsat) -> u8 {
        Sftsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srds {
    #[doc = "Data valid"]
    DATA_VALID = 0x0,
    #[doc = "Procedure in progress"]
    IN_PROGRESS = 0x01,
}
impl Srds {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srds {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srds {
    #[inline(always)]
    fn from(val: u8) -> Srds {
        Srds::from_bits(val)
    }
}
impl From<Srds> for u8 {
    #[inline(always)]
    fn from(val: Srds) -> u8 {
        Srds::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Trigger"]
    TRIGGER = 0x01,
}
impl Strig0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig0 {
    #[inline(always)]
    fn from(val: u8) -> Strig0 {
        Strig0::from_bits(val)
    }
}
impl From<Strig0> for u8 {
    #[inline(always)]
    fn from(val: Strig0) -> u8 {
        Strig0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Trigger"]
    TRIGGER = 0x01,
}
impl Strig1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig1 {
    #[inline(always)]
    fn from(val: u8) -> Strig1 {
        Strig1::from_bits(val)
    }
}
impl From<Strig1> for u8 {
    #[inline(always)]
    fn from(val: Strig1) -> u8 {
        Strig1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Trigger"]
    TRIGGER = 0x01,
}
impl Strig2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig2 {
    #[inline(always)]
    fn from(val: u8) -> Strig2 {
        Strig2::from_bits(val)
    }
}
impl From<Strig2> for u8 {
    #[inline(always)]
    fn from(val: Strig2) -> u8 {
        Strig2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Trigger"]
    TRIGGER = 0x01,
}
impl Strig3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig3 {
    #[inline(always)]
    fn from(val: u8) -> Strig3 {
        Strig3::from_bits(val)
    }
}
impl From<Strig3> for u8 {
    #[inline(always)]
    fn from(val: Strig3) -> u8 {
        Strig3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Trigger"]
    TRIGGER = 0x01,
}
impl Strig4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig4 {
    #[inline(always)]
    fn from(val: u8) -> Strig4 {
        Strig4::from_bits(val)
    }
}
impl From<Strig4> for u8 {
    #[inline(always)]
    fn from(val: Strig4) -> u8 {
        Strig4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt0 {
    #[doc = "Not exceeded"]
    WLMT_NO = 0x0,
    #[doc = "Exceeded"]
    WLMT_YES = 0x01,
}
impl Wlmt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt0 {
    #[inline(always)]
    fn from(val: u8) -> Wlmt0 {
        Wlmt0::from_bits(val)
    }
}
impl From<Wlmt0> for u8 {
    #[inline(always)]
    fn from(val: Wlmt0) -> u8 {
        Wlmt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt1 {
    #[doc = "Not exceeded"]
    WLMT_NO = 0x0,
    #[doc = "Exceeded"]
    WLMT_YES = 0x01,
}
impl Wlmt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt1 {
    #[inline(always)]
    fn from(val: u8) -> Wlmt1 {
        Wlmt1::from_bits(val)
    }
}
impl From<Wlmt1> for u8 {
    #[inline(always)]
    fn from(val: Wlmt1) -> u8 {
        Wlmt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt2 {
    #[doc = "Not exceeded"]
    WLMT_NO = 0x0,
    #[doc = "Exceeded"]
    WLMT_YES = 0x01,
}
impl Wlmt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt2 {
    #[inline(always)]
    fn from(val: u8) -> Wlmt2 {
        Wlmt2::from_bits(val)
    }
}
impl From<Wlmt2> for u8 {
    #[inline(always)]
    fn from(val: Wlmt2) -> u8 {
        Wlmt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt3 {
    #[doc = "Not exceeded"]
    WLMT_NO = 0x0,
    #[doc = "Exceeded"]
    WLMT_YES = 0x01,
}
impl Wlmt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt3 {
    #[inline(always)]
    fn from(val: u8) -> Wlmt3 {
        Wlmt3::from_bits(val)
    }
}
impl From<Wlmt3> for u8 {
    #[inline(always)]
    fn from(val: Wlmt3) -> u8 {
        Wlmt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt4 {
    #[doc = "Not exceeded"]
    WLMT_NO = 0x0,
    #[doc = "Exceeded"]
    WLMT_YES = 0x01,
}
impl Wlmt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt4 {
    #[inline(always)]
    fn from(val: u8) -> Wlmt4 {
        Wlmt4::from_bits(val)
    }
}
impl From<Wlmt4> for u8 {
    #[inline(always)]
    fn from(val: Wlmt4) -> u8 {
        Wlmt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtbk {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtbk {
    #[inline(always)]
    fn from(val: u8) -> Wlmtbk {
        Wlmtbk::from_bits(val)
    }
}
impl From<Wlmtbk> for u8 {
    #[inline(always)]
    fn from(val: Wlmtbk) -> u8 {
        Wlmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie0 {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie0 {
        Wlmtie0::from_bits(val)
    }
}
impl From<Wlmtie0> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie0) -> u8 {
        Wlmtie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie1 {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie1 {
        Wlmtie1::from_bits(val)
    }
}
impl From<Wlmtie1> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie1) -> u8 {
        Wlmtie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie2 {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie2 {
        Wlmtie2::from_bits(val)
    }
}
impl From<Wlmtie2> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie2) -> u8 {
        Wlmtie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie3 {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie3 {
        Wlmtie3::from_bits(val)
    }
}
impl From<Wlmtie3> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie3) -> u8 {
        Wlmtie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Wlmtie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie4 {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie4 {
        Wlmtie4::from_bits(val)
    }
}
impl From<Wlmtie4> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie4) -> u8 {
        Wlmtie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd0 {
    #[doc = "Not detected"]
    ZC_NO = 0x0,
    #[doc = "Detected"]
    ZC_YES = 0x01,
}
impl Zcd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd0 {
    #[inline(always)]
    fn from(val: u8) -> Zcd0 {
        Zcd0::from_bits(val)
    }
}
impl From<Zcd0> for u8 {
    #[inline(always)]
    fn from(val: Zcd0) -> u8 {
        Zcd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd1 {
    #[doc = "Not detected"]
    ZC_NO = 0x0,
    #[doc = "Detected"]
    ZC_YES = 0x01,
}
impl Zcd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd1 {
    #[inline(always)]
    fn from(val: u8) -> Zcd1 {
        Zcd1::from_bits(val)
    }
}
impl From<Zcd1> for u8 {
    #[inline(always)]
    fn from(val: Zcd1) -> u8 {
        Zcd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd2 {
    #[doc = "Not detected"]
    ZC_NO = 0x0,
    #[doc = "Detected"]
    ZC_YES = 0x01,
}
impl Zcd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd2 {
    #[inline(always)]
    fn from(val: u8) -> Zcd2 {
        Zcd2::from_bits(val)
    }
}
impl From<Zcd2> for u8 {
    #[inline(always)]
    fn from(val: Zcd2) -> u8 {
        Zcd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd3 {
    #[doc = "Not detected"]
    ZC_NO = 0x0,
    #[doc = "Detected"]
    ZC_YES = 0x01,
}
impl Zcd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd3 {
    #[inline(always)]
    fn from(val: u8) -> Zcd3 {
        Zcd3::from_bits(val)
    }
}
impl From<Zcd3> for u8 {
    #[inline(always)]
    fn from(val: Zcd3) -> u8 {
        Zcd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd4 {
    #[doc = "Not detected"]
    ZC_NO = 0x0,
    #[doc = "Detected"]
    ZC_YES = 0x01,
}
impl Zcd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd4 {
    #[inline(always)]
    fn from(val: u8) -> Zcd4 {
        Zcd4::from_bits(val)
    }
}
impl From<Zcd4> for u8 {
    #[inline(always)]
    fn from(val: Zcd4) -> u8 {
        Zcd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcden {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcden {
    #[inline(always)]
    fn from(val: u8) -> Zcden {
        Zcden::from_bits(val)
    }
}
impl From<Zcden> for u8 {
    #[inline(always)]
    fn from(val: Zcden) -> u8 {
        Zcden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie0 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcdie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie0 {
    #[inline(always)]
    fn from(val: u8) -> Zcdie0 {
        Zcdie0::from_bits(val)
    }
}
impl From<Zcdie0> for u8 {
    #[inline(always)]
    fn from(val: Zcdie0) -> u8 {
        Zcdie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie1 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcdie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie1 {
    #[inline(always)]
    fn from(val: u8) -> Zcdie1 {
        Zcdie1::from_bits(val)
    }
}
impl From<Zcdie1> for u8 {
    #[inline(always)]
    fn from(val: Zcdie1) -> u8 {
        Zcdie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie2 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcdie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie2 {
    #[inline(always)]
    fn from(val: u8) -> Zcdie2 {
        Zcdie2::from_bits(val)
    }
}
impl From<Zcdie2> for u8 {
    #[inline(always)]
    fn from(val: Zcdie2) -> u8 {
        Zcdie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie3 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcdie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie3 {
    #[inline(always)]
    fn from(val: u8) -> Zcdie3 {
        Zcdie3::from_bits(val)
    }
}
impl From<Zcdie3> for u8 {
    #[inline(always)]
    fn from(val: Zcdie3) -> u8 {
        Zcdie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie4 {
    #[doc = "Disables"]
    DISABLES = 0x0,
    #[doc = "Enables"]
    ENABLES = 0x01,
}
impl Zcdie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie4 {
    #[inline(always)]
    fn from(val: u8) -> Zcdie4 {
        Zcdie4::from_bits(val)
    }
}
impl From<Zcdie4> for u8 {
    #[inline(always)]
    fn from(val: Zcdie4) -> u8 {
        Zcdie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcop {
    #[doc = "Both rise and fall"]
    BOTH = 0x0,
    #[doc = "Fall"]
    FALL = 0x01,
    #[doc = "Rise"]
    RISE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Zcop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcop {
    #[inline(always)]
    fn from(val: u8) -> Zcop {
        Zcop::from_bits(val)
    }
}
impl From<Zcop> for u8 {
    #[inline(always)]
    fn from(val: Zcop) -> u8 {
        Zcop::to_bits(val)
    }
}
