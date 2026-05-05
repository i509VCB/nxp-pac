#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaModeSel {
    #[doc = "Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared."]
    DmaModeSel0 = 0x0,
    #[doc = "Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only."]
    DmaModeSel1 = 0x01,
}
impl DmaModeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaModeSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaModeSel {
    #[inline(always)]
    fn from(val: u8) -> DmaModeSel {
        DmaModeSel::from_bits(val)
    }
}
impl From<DmaModeSel> for u8 {
    #[inline(always)]
    fn from(val: DmaModeSel) -> u8 {
        DmaModeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softrst {
    #[doc = "ADC_ETC works normally."]
    Softrst0 = 0x0,
    #[doc = "All registers inside ADC_ETC will be reset to the default value."]
    Softrst1 = 0x01,
}
impl Softrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softrst {
    #[inline(always)]
    fn from(val: u8) -> Softrst {
        Softrst::from_bits(val)
    }
}
impl From<Softrst> for u8 {
    #[inline(always)]
    fn from(val: Softrst) -> u8 {
        Softrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10B2b0 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b00 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b01 = 0x01,
}
impl Trig0Chain10B2b0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10B2b0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10B2b0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10B2b0 {
        Trig0Chain10B2b0::from_bits(val)
    }
}
impl From<Trig0Chain10B2b0> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10B2b0) -> u8 {
        Trig0Chain10B2b0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10B2b1 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b10 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b11 = 0x01,
}
impl Trig0Chain10B2b1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10B2b1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10B2b1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10B2b1 {
        Trig0Chain10B2b1::from_bits(val)
    }
}
impl From<Trig0Chain10B2b1> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10B2b1) -> u8 {
        Trig0Chain10B2b1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Csel0 {
    #[doc = "ADC Channel 0 selected."]
    Csel00 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel01 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel02 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel03 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel04 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel05 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel06 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel07 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel08 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel09 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel010 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel011 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel012 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel013 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel014 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel015 = 0x0f,
}
impl Trig0Chain10Csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Csel0 {
        Trig0Chain10Csel0::from_bits(val)
    }
}
impl From<Trig0Chain10Csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Csel0) -> u8 {
        Trig0Chain10Csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Csel1 {
    #[doc = "ADC Channel 0 selected."]
    Csel10 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel11 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel12 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel13 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel14 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel15 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel16 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel17 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel18 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel19 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel110 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel111 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel112 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel113 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel114 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel115 = 0x0f,
}
impl Trig0Chain10Csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Csel1 {
        Trig0Chain10Csel1::from_bits(val)
    }
}
impl From<Trig0Chain10Csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Csel1) -> u8 {
        Trig0Chain10Csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain10Hwts0(u8);
impl Trig0Chain10Hwts0 {
    #[doc = "no trigger selected."]
    pub const Hwts00: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts01: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts02: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts04: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts08: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts016: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts032: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts064: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts0128: Self = Self(0x80);
}
impl Trig0Chain10Hwts0 {
    pub const fn from_bits(val: u8) -> Trig0Chain10Hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain10Hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts00"),
            0x01 => f.write_str("Hwts01"),
            0x02 => f.write_str("Hwts02"),
            0x04 => f.write_str("Hwts04"),
            0x08 => f.write_str("Hwts08"),
            0x10 => f.write_str("Hwts016"),
            0x20 => f.write_str("Hwts032"),
            0x40 => f.write_str("Hwts064"),
            0x80 => f.write_str("Hwts0128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts00"),
            0x01 => defmt::write!(f, "Hwts01"),
            0x02 => defmt::write!(f, "Hwts02"),
            0x04 => defmt::write!(f, "Hwts04"),
            0x08 => defmt::write!(f, "Hwts08"),
            0x10 => defmt::write!(f, "Hwts016"),
            0x20 => defmt::write!(f, "Hwts032"),
            0x40 => defmt::write!(f, "Hwts064"),
            0x80 => defmt::write!(f, "Hwts0128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain10Hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Hwts0 {
        Trig0Chain10Hwts0::from_bits(val)
    }
}
impl From<Trig0Chain10Hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Hwts0) -> u8 {
        Trig0Chain10Hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain10Hwts1(u8);
impl Trig0Chain10Hwts1 {
    #[doc = "no trigger selected."]
    pub const Hwts10: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts11: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts12: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts14: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts18: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts116: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts132: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts164: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts1128: Self = Self(0x80);
}
impl Trig0Chain10Hwts1 {
    pub const fn from_bits(val: u8) -> Trig0Chain10Hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain10Hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts10"),
            0x01 => f.write_str("Hwts11"),
            0x02 => f.write_str("Hwts12"),
            0x04 => f.write_str("Hwts14"),
            0x08 => f.write_str("Hwts18"),
            0x10 => f.write_str("Hwts116"),
            0x20 => f.write_str("Hwts132"),
            0x40 => f.write_str("Hwts164"),
            0x80 => f.write_str("Hwts1128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts10"),
            0x01 => defmt::write!(f, "Hwts11"),
            0x02 => defmt::write!(f, "Hwts12"),
            0x04 => defmt::write!(f, "Hwts14"),
            0x08 => defmt::write!(f, "Hwts18"),
            0x10 => defmt::write!(f, "Hwts116"),
            0x20 => defmt::write!(f, "Hwts132"),
            0x40 => defmt::write!(f, "Hwts164"),
            0x80 => defmt::write!(f, "Hwts1128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain10Hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Hwts1 {
        Trig0Chain10Hwts1::from_bits(val)
    }
}
impl From<Trig0Chain10Hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Hwts1) -> u8 {
        Trig0Chain10Hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Ie0 {
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    Ie00 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    Ie01 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    Ie02 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    Ie03 = 0x03,
}
impl Trig0Chain10Ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Ie0 {
        Trig0Chain10Ie0::from_bits(val)
    }
}
impl From<Trig0Chain10Ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Ie0) -> u8 {
        Trig0Chain10Ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Ie0En {
    #[doc = "Interrupt DONE disabled."]
    Ie0En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
    Ie0En1 = 0x01,
}
impl Trig0Chain10Ie0En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Ie0En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Ie0En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Ie0En {
        Trig0Chain10Ie0En::from_bits(val)
    }
}
impl From<Trig0Chain10Ie0En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Ie0En) -> u8 {
        Trig0Chain10Ie0En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    Ie10 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    Ie11 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    Ie12 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    Ie13 = 0x03,
}
impl Trig0Chain10Ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Ie1 {
        Trig0Chain10Ie1::from_bits(val)
    }
}
impl From<Trig0Chain10Ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Ie1) -> u8 {
        Trig0Chain10Ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain10Ie1En {
    #[doc = "Interrupt DONE disabled."]
    Ie1En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
    Ie1En1 = 0x01,
}
impl Trig0Chain10Ie1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain10Ie1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain10Ie1En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain10Ie1En {
        Trig0Chain10Ie1En::from_bits(val)
    }
}
impl From<Trig0Chain10Ie1En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain10Ie1En) -> u8 {
        Trig0Chain10Ie1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32B2b2 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b20 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b21 = 0x01,
}
impl Trig0Chain32B2b2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32B2b2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32B2b2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32B2b2 {
        Trig0Chain32B2b2::from_bits(val)
    }
}
impl From<Trig0Chain32B2b2> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32B2b2) -> u8 {
        Trig0Chain32B2b2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32B2b3 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b30 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b31 = 0x01,
}
impl Trig0Chain32B2b3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32B2b3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32B2b3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32B2b3 {
        Trig0Chain32B2b3::from_bits(val)
    }
}
impl From<Trig0Chain32B2b3> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32B2b3) -> u8 {
        Trig0Chain32B2b3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Csel2 {
    #[doc = "ADC Channel 0 selected."]
    Csel20 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel21 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel22 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel23 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel24 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel25 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel26 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel27 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel28 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel29 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel210 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel211 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel212 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel213 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel214 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel215 = 0x0f,
}
impl Trig0Chain32Csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Csel2 {
        Trig0Chain32Csel2::from_bits(val)
    }
}
impl From<Trig0Chain32Csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Csel2) -> u8 {
        Trig0Chain32Csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Csel3 {
    #[doc = "ADC Channel 0 selected."]
    Csel30 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel31 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel32 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel33 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel34 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel35 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel36 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel37 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel38 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel39 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel310 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel311 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel312 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel313 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel314 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel315 = 0x0f,
}
impl Trig0Chain32Csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Csel3 {
        Trig0Chain32Csel3::from_bits(val)
    }
}
impl From<Trig0Chain32Csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Csel3) -> u8 {
        Trig0Chain32Csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain32Hwts2(u8);
impl Trig0Chain32Hwts2 {
    #[doc = "no trigger selected."]
    pub const Hwts20: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts21: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts22: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts24: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts28: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts216: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts232: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts264: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts2128: Self = Self(0x80);
}
impl Trig0Chain32Hwts2 {
    pub const fn from_bits(val: u8) -> Trig0Chain32Hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain32Hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts20"),
            0x01 => f.write_str("Hwts21"),
            0x02 => f.write_str("Hwts22"),
            0x04 => f.write_str("Hwts24"),
            0x08 => f.write_str("Hwts28"),
            0x10 => f.write_str("Hwts216"),
            0x20 => f.write_str("Hwts232"),
            0x40 => f.write_str("Hwts264"),
            0x80 => f.write_str("Hwts2128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts20"),
            0x01 => defmt::write!(f, "Hwts21"),
            0x02 => defmt::write!(f, "Hwts22"),
            0x04 => defmt::write!(f, "Hwts24"),
            0x08 => defmt::write!(f, "Hwts28"),
            0x10 => defmt::write!(f, "Hwts216"),
            0x20 => defmt::write!(f, "Hwts232"),
            0x40 => defmt::write!(f, "Hwts264"),
            0x80 => defmt::write!(f, "Hwts2128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain32Hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Hwts2 {
        Trig0Chain32Hwts2::from_bits(val)
    }
}
impl From<Trig0Chain32Hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Hwts2) -> u8 {
        Trig0Chain32Hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain32Hwts3(u8);
impl Trig0Chain32Hwts3 {
    #[doc = "no trigger selected."]
    pub const Hwts30: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts31: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts32: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts34: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts38: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts316: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts332: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts364: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts3128: Self = Self(0x80);
}
impl Trig0Chain32Hwts3 {
    pub const fn from_bits(val: u8) -> Trig0Chain32Hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain32Hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts30"),
            0x01 => f.write_str("Hwts31"),
            0x02 => f.write_str("Hwts32"),
            0x04 => f.write_str("Hwts34"),
            0x08 => f.write_str("Hwts38"),
            0x10 => f.write_str("Hwts316"),
            0x20 => f.write_str("Hwts332"),
            0x40 => f.write_str("Hwts364"),
            0x80 => f.write_str("Hwts3128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts30"),
            0x01 => defmt::write!(f, "Hwts31"),
            0x02 => defmt::write!(f, "Hwts32"),
            0x04 => defmt::write!(f, "Hwts34"),
            0x08 => defmt::write!(f, "Hwts38"),
            0x10 => defmt::write!(f, "Hwts316"),
            0x20 => defmt::write!(f, "Hwts332"),
            0x40 => defmt::write!(f, "Hwts364"),
            0x80 => defmt::write!(f, "Hwts3128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain32Hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Hwts3 {
        Trig0Chain32Hwts3::from_bits(val)
    }
}
impl From<Trig0Chain32Hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Hwts3) -> u8 {
        Trig0Chain32Hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Ie2 {
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    Ie20 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    Ie21 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    Ie22 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    Ie23 = 0x03,
}
impl Trig0Chain32Ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Ie2 {
        Trig0Chain32Ie2::from_bits(val)
    }
}
impl From<Trig0Chain32Ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Ie2) -> u8 {
        Trig0Chain32Ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Ie2En {
    #[doc = "Interrupt DONE disabled."]
    Ie2En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
    Ie2En1 = 0x01,
}
impl Trig0Chain32Ie2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Ie2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Ie2En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Ie2En {
        Trig0Chain32Ie2En::from_bits(val)
    }
}
impl From<Trig0Chain32Ie2En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Ie2En) -> u8 {
        Trig0Chain32Ie2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    Ie30 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    Ie31 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    Ie32 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    Ie33 = 0x03,
}
impl Trig0Chain32Ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Ie3 {
        Trig0Chain32Ie3::from_bits(val)
    }
}
impl From<Trig0Chain32Ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Ie3) -> u8 {
        Trig0Chain32Ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain32Ie3En {
    #[doc = "Interrupt DONE disabled."]
    Ie3En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
    Ie3En1 = 0x01,
}
impl Trig0Chain32Ie3En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain32Ie3En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain32Ie3En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain32Ie3En {
        Trig0Chain32Ie3En::from_bits(val)
    }
}
impl From<Trig0Chain32Ie3En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain32Ie3En) -> u8 {
        Trig0Chain32Ie3En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54B2b4 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b40 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b41 = 0x01,
}
impl Trig0Chain54B2b4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54B2b4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54B2b4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54B2b4 {
        Trig0Chain54B2b4::from_bits(val)
    }
}
impl From<Trig0Chain54B2b4> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54B2b4) -> u8 {
        Trig0Chain54B2b4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54B2b5 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b50 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b51 = 0x01,
}
impl Trig0Chain54B2b5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54B2b5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54B2b5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54B2b5 {
        Trig0Chain54B2b5::from_bits(val)
    }
}
impl From<Trig0Chain54B2b5> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54B2b5) -> u8 {
        Trig0Chain54B2b5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Csel4 {
    #[doc = "ADC Channel 0 selected."]
    Csel40 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel41 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel42 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel43 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel44 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel45 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel46 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel47 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel48 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel49 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel410 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel411 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel412 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel413 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel414 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel415 = 0x0f,
}
impl Trig0Chain54Csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Csel4 {
        Trig0Chain54Csel4::from_bits(val)
    }
}
impl From<Trig0Chain54Csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Csel4) -> u8 {
        Trig0Chain54Csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Csel5 {
    #[doc = "ADC Channel 0 selected."]
    Csel50 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel51 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel52 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel53 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel54 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel55 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel56 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel57 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel58 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel59 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel510 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel511 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel512 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel513 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel514 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel515 = 0x0f,
}
impl Trig0Chain54Csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Csel5 {
        Trig0Chain54Csel5::from_bits(val)
    }
}
impl From<Trig0Chain54Csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Csel5) -> u8 {
        Trig0Chain54Csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain54Hwts4(u8);
impl Trig0Chain54Hwts4 {
    #[doc = "no trigger selected."]
    pub const Hwts40: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts41: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts42: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts44: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts48: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts416: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts432: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts464: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts4128: Self = Self(0x80);
}
impl Trig0Chain54Hwts4 {
    pub const fn from_bits(val: u8) -> Trig0Chain54Hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain54Hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts40"),
            0x01 => f.write_str("Hwts41"),
            0x02 => f.write_str("Hwts42"),
            0x04 => f.write_str("Hwts44"),
            0x08 => f.write_str("Hwts48"),
            0x10 => f.write_str("Hwts416"),
            0x20 => f.write_str("Hwts432"),
            0x40 => f.write_str("Hwts464"),
            0x80 => f.write_str("Hwts4128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts40"),
            0x01 => defmt::write!(f, "Hwts41"),
            0x02 => defmt::write!(f, "Hwts42"),
            0x04 => defmt::write!(f, "Hwts44"),
            0x08 => defmt::write!(f, "Hwts48"),
            0x10 => defmt::write!(f, "Hwts416"),
            0x20 => defmt::write!(f, "Hwts432"),
            0x40 => defmt::write!(f, "Hwts464"),
            0x80 => defmt::write!(f, "Hwts4128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain54Hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Hwts4 {
        Trig0Chain54Hwts4::from_bits(val)
    }
}
impl From<Trig0Chain54Hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Hwts4) -> u8 {
        Trig0Chain54Hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain54Hwts5(u8);
impl Trig0Chain54Hwts5 {
    #[doc = "no trigger selected."]
    pub const Hwts50: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts51: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts52: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts54: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts58: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts516: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts532: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts564: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts5128: Self = Self(0x80);
}
impl Trig0Chain54Hwts5 {
    pub const fn from_bits(val: u8) -> Trig0Chain54Hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain54Hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts50"),
            0x01 => f.write_str("Hwts51"),
            0x02 => f.write_str("Hwts52"),
            0x04 => f.write_str("Hwts54"),
            0x08 => f.write_str("Hwts58"),
            0x10 => f.write_str("Hwts516"),
            0x20 => f.write_str("Hwts532"),
            0x40 => f.write_str("Hwts564"),
            0x80 => f.write_str("Hwts5128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts50"),
            0x01 => defmt::write!(f, "Hwts51"),
            0x02 => defmt::write!(f, "Hwts52"),
            0x04 => defmt::write!(f, "Hwts54"),
            0x08 => defmt::write!(f, "Hwts58"),
            0x10 => defmt::write!(f, "Hwts516"),
            0x20 => defmt::write!(f, "Hwts532"),
            0x40 => defmt::write!(f, "Hwts564"),
            0x80 => defmt::write!(f, "Hwts5128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain54Hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Hwts5 {
        Trig0Chain54Hwts5::from_bits(val)
    }
}
impl From<Trig0Chain54Hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Hwts5) -> u8 {
        Trig0Chain54Hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Ie4 {
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    Ie40 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    Ie41 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    Ie42 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    Ie43 = 0x03,
}
impl Trig0Chain54Ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Ie4 {
        Trig0Chain54Ie4::from_bits(val)
    }
}
impl From<Trig0Chain54Ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Ie4) -> u8 {
        Trig0Chain54Ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Ie4En {
    #[doc = "Interrupt DONE disabled."]
    Ie4En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
    Ie4En1 = 0x01,
}
impl Trig0Chain54Ie4En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Ie4En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Ie4En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Ie4En {
        Trig0Chain54Ie4En::from_bits(val)
    }
}
impl From<Trig0Chain54Ie4En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Ie4En) -> u8 {
        Trig0Chain54Ie4En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    Ie50 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    Ie51 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    Ie52 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    Ie53 = 0x03,
}
impl Trig0Chain54Ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Ie5 {
        Trig0Chain54Ie5::from_bits(val)
    }
}
impl From<Trig0Chain54Ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Ie5) -> u8 {
        Trig0Chain54Ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain54Ie5En {
    #[doc = "Interrupt DONE disabled."]
    Ie5En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
    Ie5En1 = 0x01,
}
impl Trig0Chain54Ie5En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain54Ie5En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain54Ie5En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain54Ie5En {
        Trig0Chain54Ie5En::from_bits(val)
    }
}
impl From<Trig0Chain54Ie5En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain54Ie5En) -> u8 {
        Trig0Chain54Ie5En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76B2b6 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b60 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b61 = 0x01,
}
impl Trig0Chain76B2b6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76B2b6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76B2b6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76B2b6 {
        Trig0Chain76B2b6::from_bits(val)
    }
}
impl From<Trig0Chain76B2b6> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76B2b6) -> u8 {
        Trig0Chain76B2b6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76B2b7 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b70 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b71 = 0x01,
}
impl Trig0Chain76B2b7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76B2b7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76B2b7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76B2b7 {
        Trig0Chain76B2b7::from_bits(val)
    }
}
impl From<Trig0Chain76B2b7> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76B2b7) -> u8 {
        Trig0Chain76B2b7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Csel6 {
    #[doc = "ADC Channel 0 selected."]
    Csel60 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel61 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel62 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel63 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel64 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel65 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel66 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel67 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel68 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel69 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel610 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel611 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel612 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel613 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel614 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel615 = 0x0f,
}
impl Trig0Chain76Csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Csel6 {
        Trig0Chain76Csel6::from_bits(val)
    }
}
impl From<Trig0Chain76Csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Csel6) -> u8 {
        Trig0Chain76Csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Csel7 {
    #[doc = "ADC Channel 0 selected."]
    Csel70 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel71 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel72 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel73 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel74 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel75 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel76 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel77 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel78 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel79 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel710 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel711 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel712 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel713 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel714 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel715 = 0x0f,
}
impl Trig0Chain76Csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Csel7 {
        Trig0Chain76Csel7::from_bits(val)
    }
}
impl From<Trig0Chain76Csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Csel7) -> u8 {
        Trig0Chain76Csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain76Hwts6(u8);
impl Trig0Chain76Hwts6 {
    #[doc = "no trigger selected."]
    pub const Hwts60: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts61: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts62: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts64: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts68: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts616: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts632: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts664: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts6128: Self = Self(0x80);
}
impl Trig0Chain76Hwts6 {
    pub const fn from_bits(val: u8) -> Trig0Chain76Hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain76Hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts60"),
            0x01 => f.write_str("Hwts61"),
            0x02 => f.write_str("Hwts62"),
            0x04 => f.write_str("Hwts64"),
            0x08 => f.write_str("Hwts68"),
            0x10 => f.write_str("Hwts616"),
            0x20 => f.write_str("Hwts632"),
            0x40 => f.write_str("Hwts664"),
            0x80 => f.write_str("Hwts6128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts60"),
            0x01 => defmt::write!(f, "Hwts61"),
            0x02 => defmt::write!(f, "Hwts62"),
            0x04 => defmt::write!(f, "Hwts64"),
            0x08 => defmt::write!(f, "Hwts68"),
            0x10 => defmt::write!(f, "Hwts616"),
            0x20 => defmt::write!(f, "Hwts632"),
            0x40 => defmt::write!(f, "Hwts664"),
            0x80 => defmt::write!(f, "Hwts6128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain76Hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Hwts6 {
        Trig0Chain76Hwts6::from_bits(val)
    }
}
impl From<Trig0Chain76Hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Hwts6) -> u8 {
        Trig0Chain76Hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0Chain76Hwts7(u8);
impl Trig0Chain76Hwts7 {
    #[doc = "no trigger selected."]
    pub const Hwts70: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts71: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts72: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts74: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts78: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts716: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts732: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts764: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts7128: Self = Self(0x80);
}
impl Trig0Chain76Hwts7 {
    pub const fn from_bits(val: u8) -> Trig0Chain76Hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0Chain76Hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts70"),
            0x01 => f.write_str("Hwts71"),
            0x02 => f.write_str("Hwts72"),
            0x04 => f.write_str("Hwts74"),
            0x08 => f.write_str("Hwts78"),
            0x10 => f.write_str("Hwts716"),
            0x20 => f.write_str("Hwts732"),
            0x40 => f.write_str("Hwts764"),
            0x80 => f.write_str("Hwts7128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts70"),
            0x01 => defmt::write!(f, "Hwts71"),
            0x02 => defmt::write!(f, "Hwts72"),
            0x04 => defmt::write!(f, "Hwts74"),
            0x08 => defmt::write!(f, "Hwts78"),
            0x10 => defmt::write!(f, "Hwts716"),
            0x20 => defmt::write!(f, "Hwts732"),
            0x40 => defmt::write!(f, "Hwts764"),
            0x80 => defmt::write!(f, "Hwts7128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0Chain76Hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Hwts7 {
        Trig0Chain76Hwts7::from_bits(val)
    }
}
impl From<Trig0Chain76Hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Hwts7) -> u8 {
        Trig0Chain76Hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Ie6 {
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    Ie60 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    Ie61 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    Ie62 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    Ie63 = 0x03,
}
impl Trig0Chain76Ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Ie6 {
        Trig0Chain76Ie6::from_bits(val)
    }
}
impl From<Trig0Chain76Ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Ie6) -> u8 {
        Trig0Chain76Ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Ie6En {
    #[doc = "Interrupt DONE disabled."]
    Ie6En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
    Ie6En1 = 0x01,
}
impl Trig0Chain76Ie6En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Ie6En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Ie6En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Ie6En {
        Trig0Chain76Ie6En::from_bits(val)
    }
}
impl From<Trig0Chain76Ie6En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Ie6En) -> u8 {
        Trig0Chain76Ie6En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    Ie70 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    Ie71 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    Ie72 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    Ie73 = 0x03,
}
impl Trig0Chain76Ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Ie7 {
        Trig0Chain76Ie7::from_bits(val)
    }
}
impl From<Trig0Chain76Ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Ie7) -> u8 {
        Trig0Chain76Ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Chain76Ie7En {
    #[doc = "Interrupt DONE disabled."]
    Ie7En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
    Ie7En1 = 0x01,
}
impl Trig0Chain76Ie7En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Chain76Ie7En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Chain76Ie7En {
    #[inline(always)]
    fn from(val: u8) -> Trig0Chain76Ie7En {
        Trig0Chain76Ie7En::from_bits(val)
    }
}
impl From<Trig0Chain76Ie7En> for u8 {
    #[inline(always)]
    fn from(val: Trig0Chain76Ie7En) -> u8 {
        Trig0Chain76Ie7En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0CtrlSwTrig {
    #[doc = "No software trigger event generated."]
    SwTrig0 = 0x0,
    #[doc = "Software trigger event generated."]
    SwTrig1 = 0x01,
}
impl Trig0CtrlSwTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0CtrlSwTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0CtrlSwTrig {
    #[inline(always)]
    fn from(val: u8) -> Trig0CtrlSwTrig {
        Trig0CtrlSwTrig::from_bits(val)
    }
}
impl From<Trig0CtrlSwTrig> for u8 {
    #[inline(always)]
    fn from(val: Trig0CtrlSwTrig) -> u8 {
        Trig0CtrlSwTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SyncMode0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SyncMode1 = 0x01,
}
impl Trig0CtrlSyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0CtrlSyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0CtrlSyncMode {
    #[inline(always)]
    fn from(val: u8) -> Trig0CtrlSyncMode {
        Trig0CtrlSyncMode::from_bits(val)
    }
}
impl From<Trig0CtrlSyncMode> for u8 {
    #[inline(always)]
    fn from(val: Trig0CtrlSyncMode) -> u8 {
        Trig0CtrlSyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0CtrlTrigChain {
    #[doc = "Trigger chain length is 1."]
    TrigChain0 = 0x0,
    #[doc = "Trigger chain length is 2."]
    TrigChain1 = 0x01,
    #[doc = "Trigger chain length is 3."]
    TrigChain2 = 0x02,
    #[doc = "Trigger chain length is 4."]
    TrigChain3 = 0x03,
    #[doc = "Trigger chain length is 5."]
    TrigChain4 = 0x04,
    #[doc = "Trigger chain length is 6."]
    TrigChain5 = 0x05,
    #[doc = "Trigger chain length is 7."]
    TrigChain6 = 0x06,
    #[doc = "Trigger chain length is 8."]
    TrigChain7 = 0x07,
}
impl Trig0CtrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0CtrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0CtrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig0CtrlTrigChain {
        Trig0CtrlTrigChain::from_bits(val)
    }
}
impl From<Trig0CtrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig0CtrlTrigChain) -> u8 {
        Trig0CtrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0CtrlTrigMode {
    #[doc = "Hardware trigger. The softerware trigger will be ignored."]
    TrigMode0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TrigMode1 = 0x01,
}
impl Trig0CtrlTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0CtrlTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0CtrlTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Trig0CtrlTrigMode {
        Trig0CtrlTrigMode::from_bits(val)
    }
}
impl From<Trig0CtrlTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Trig0CtrlTrigMode) -> u8 {
        Trig0CtrlTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Done0 {
    #[doc = "No TRIG0_DONE0 interrupt detected."]
    Trig0Done00 = 0x0,
    #[doc = "TRIG0_DONE0 interrupt detected."]
    Trig0Done01 = 0x01,
}
impl Trig0Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Done0 {
        Trig0Done0::from_bits(val)
    }
}
impl From<Trig0Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig0Done0) -> u8 {
        Trig0Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Done1 {
    #[doc = "No TRIG0_DONE1 interrupt detected."]
    Trig0Done10 = 0x0,
    #[doc = "TRIG0_DONE1 interrupt detected."]
    Trig0Done11 = 0x01,
}
impl Trig0Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Done1 {
        Trig0Done1::from_bits(val)
    }
}
impl From<Trig0Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig0Done1) -> u8 {
        Trig0Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Done2 {
    #[doc = "No TRIG0_DONE2 interrupt detected."]
    Trig0Done20 = 0x0,
    #[doc = "TRIG0_DONE2 interrupt detected."]
    Trig0Done21 = 0x01,
}
impl Trig0Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Done2 {
        Trig0Done2::from_bits(val)
    }
}
impl From<Trig0Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig0Done2) -> u8 {
        Trig0Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Done3 {
    #[doc = "No TRIG0_DONE3 interrupt detected."]
    Trig0Done30 = 0x0,
    #[doc = "TRIG0_DONE3 interrupt detected."]
    Trig0Done31 = 0x01,
}
impl Trig0Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0Done3 {
        Trig0Done3::from_bits(val)
    }
}
impl From<Trig0Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig0Done3) -> u8 {
        Trig0Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Enable {
    #[doc = "TRIG0 DMA request disabled."]
    Trig0Enable0 = 0x0,
    #[doc = "TRIG0 DMA request enabled."]
    Trig0Enable1 = 0x01,
}
impl Trig0Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig0Enable {
        Trig0Enable::from_bits(val)
    }
}
impl From<Trig0Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig0Enable) -> u8 {
        Trig0Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Err {
    #[doc = "No TRIG0_ERR interrupt detected."]
    Trig0Err0 = 0x0,
    #[doc = "TRIG0_ERR interrupt detected."]
    Trig0Err1 = 0x01,
}
impl Trig0Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Err {
    #[inline(always)]
    fn from(val: u8) -> Trig0Err {
        Trig0Err::from_bits(val)
    }
}
impl From<Trig0Err> for u8 {
    #[inline(always)]
    fn from(val: Trig0Err) -> u8 {
        Trig0Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0Req {
    #[doc = "TRIG0_REQ not detected."]
    Trig0Req0 = 0x0,
    #[doc = "TRIG0_REQ detected."]
    Trig0Req1 = 0x01,
}
impl Trig0Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0Req {
    #[inline(always)]
    fn from(val: u8) -> Trig0Req {
        Trig0Req::from_bits(val)
    }
}
impl From<Trig0Req> for u8 {
    #[inline(always)]
    fn from(val: Trig0Req) -> u8 {
        Trig0Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10B2b0 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b00 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b01 = 0x01,
}
impl Trig1Chain10B2b0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10B2b0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10B2b0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10B2b0 {
        Trig1Chain10B2b0::from_bits(val)
    }
}
impl From<Trig1Chain10B2b0> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10B2b0) -> u8 {
        Trig1Chain10B2b0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10B2b1 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b10 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b11 = 0x01,
}
impl Trig1Chain10B2b1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10B2b1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10B2b1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10B2b1 {
        Trig1Chain10B2b1::from_bits(val)
    }
}
impl From<Trig1Chain10B2b1> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10B2b1) -> u8 {
        Trig1Chain10B2b1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Csel0 {
    #[doc = "ADC Channel 0 selected."]
    Csel00 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel01 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel02 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel03 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel04 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel05 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel06 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel07 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel08 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel09 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel010 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel011 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel012 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel013 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel014 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel015 = 0x0f,
}
impl Trig1Chain10Csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Csel0 {
        Trig1Chain10Csel0::from_bits(val)
    }
}
impl From<Trig1Chain10Csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Csel0) -> u8 {
        Trig1Chain10Csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Csel1 {
    #[doc = "ADC Channel 0 selected."]
    Csel10 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel11 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel12 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel13 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel14 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel15 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel16 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel17 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel18 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel19 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel110 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel111 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel112 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel113 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel114 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel115 = 0x0f,
}
impl Trig1Chain10Csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Csel1 {
        Trig1Chain10Csel1::from_bits(val)
    }
}
impl From<Trig1Chain10Csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Csel1) -> u8 {
        Trig1Chain10Csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain10Hwts0(u8);
impl Trig1Chain10Hwts0 {
    #[doc = "no trigger selected."]
    pub const Hwts00: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts01: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts02: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts04: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts08: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts016: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts032: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts064: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts0128: Self = Self(0x80);
}
impl Trig1Chain10Hwts0 {
    pub const fn from_bits(val: u8) -> Trig1Chain10Hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain10Hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts00"),
            0x01 => f.write_str("Hwts01"),
            0x02 => f.write_str("Hwts02"),
            0x04 => f.write_str("Hwts04"),
            0x08 => f.write_str("Hwts08"),
            0x10 => f.write_str("Hwts016"),
            0x20 => f.write_str("Hwts032"),
            0x40 => f.write_str("Hwts064"),
            0x80 => f.write_str("Hwts0128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts00"),
            0x01 => defmt::write!(f, "Hwts01"),
            0x02 => defmt::write!(f, "Hwts02"),
            0x04 => defmt::write!(f, "Hwts04"),
            0x08 => defmt::write!(f, "Hwts08"),
            0x10 => defmt::write!(f, "Hwts016"),
            0x20 => defmt::write!(f, "Hwts032"),
            0x40 => defmt::write!(f, "Hwts064"),
            0x80 => defmt::write!(f, "Hwts0128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain10Hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Hwts0 {
        Trig1Chain10Hwts0::from_bits(val)
    }
}
impl From<Trig1Chain10Hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Hwts0) -> u8 {
        Trig1Chain10Hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain10Hwts1(u8);
impl Trig1Chain10Hwts1 {
    #[doc = "no trigger selected."]
    pub const Hwts10: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts11: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts12: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts14: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts18: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts116: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts132: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts164: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts1128: Self = Self(0x80);
}
impl Trig1Chain10Hwts1 {
    pub const fn from_bits(val: u8) -> Trig1Chain10Hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain10Hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts10"),
            0x01 => f.write_str("Hwts11"),
            0x02 => f.write_str("Hwts12"),
            0x04 => f.write_str("Hwts14"),
            0x08 => f.write_str("Hwts18"),
            0x10 => f.write_str("Hwts116"),
            0x20 => f.write_str("Hwts132"),
            0x40 => f.write_str("Hwts164"),
            0x80 => f.write_str("Hwts1128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts10"),
            0x01 => defmt::write!(f, "Hwts11"),
            0x02 => defmt::write!(f, "Hwts12"),
            0x04 => defmt::write!(f, "Hwts14"),
            0x08 => defmt::write!(f, "Hwts18"),
            0x10 => defmt::write!(f, "Hwts116"),
            0x20 => defmt::write!(f, "Hwts132"),
            0x40 => defmt::write!(f, "Hwts164"),
            0x80 => defmt::write!(f, "Hwts1128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain10Hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Hwts1 {
        Trig1Chain10Hwts1::from_bits(val)
    }
}
impl From<Trig1Chain10Hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Hwts1) -> u8 {
        Trig1Chain10Hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Ie0 {
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    Ie00 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    Ie01 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    Ie02 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    Ie03 = 0x03,
}
impl Trig1Chain10Ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Ie0 {
        Trig1Chain10Ie0::from_bits(val)
    }
}
impl From<Trig1Chain10Ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Ie0) -> u8 {
        Trig1Chain10Ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Ie0En {
    #[doc = "Interrupt DONE disabled."]
    Ie0En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
    Ie0En1 = 0x01,
}
impl Trig1Chain10Ie0En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Ie0En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Ie0En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Ie0En {
        Trig1Chain10Ie0En::from_bits(val)
    }
}
impl From<Trig1Chain10Ie0En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Ie0En) -> u8 {
        Trig1Chain10Ie0En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    Ie10 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    Ie11 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    Ie12 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    Ie13 = 0x03,
}
impl Trig1Chain10Ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Ie1 {
        Trig1Chain10Ie1::from_bits(val)
    }
}
impl From<Trig1Chain10Ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Ie1) -> u8 {
        Trig1Chain10Ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain10Ie1En {
    #[doc = "Interrupt DONE disabled."]
    Ie1En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
    Ie1En1 = 0x01,
}
impl Trig1Chain10Ie1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain10Ie1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain10Ie1En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain10Ie1En {
        Trig1Chain10Ie1En::from_bits(val)
    }
}
impl From<Trig1Chain10Ie1En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain10Ie1En) -> u8 {
        Trig1Chain10Ie1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32B2b2 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b20 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b21 = 0x01,
}
impl Trig1Chain32B2b2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32B2b2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32B2b2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32B2b2 {
        Trig1Chain32B2b2::from_bits(val)
    }
}
impl From<Trig1Chain32B2b2> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32B2b2) -> u8 {
        Trig1Chain32B2b2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32B2b3 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b30 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b31 = 0x01,
}
impl Trig1Chain32B2b3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32B2b3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32B2b3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32B2b3 {
        Trig1Chain32B2b3::from_bits(val)
    }
}
impl From<Trig1Chain32B2b3> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32B2b3) -> u8 {
        Trig1Chain32B2b3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Csel2 {
    #[doc = "ADC Channel 0 selected."]
    Csel20 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel21 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel22 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel23 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel24 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel25 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel26 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel27 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel28 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel29 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel210 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel211 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel212 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel213 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel214 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel215 = 0x0f,
}
impl Trig1Chain32Csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Csel2 {
        Trig1Chain32Csel2::from_bits(val)
    }
}
impl From<Trig1Chain32Csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Csel2) -> u8 {
        Trig1Chain32Csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Csel3 {
    #[doc = "ADC Channel 0 selected."]
    Csel30 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel31 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel32 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel33 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel34 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel35 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel36 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel37 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel38 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel39 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel310 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel311 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel312 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel313 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel314 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel315 = 0x0f,
}
impl Trig1Chain32Csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Csel3 {
        Trig1Chain32Csel3::from_bits(val)
    }
}
impl From<Trig1Chain32Csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Csel3) -> u8 {
        Trig1Chain32Csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain32Hwts2(u8);
impl Trig1Chain32Hwts2 {
    #[doc = "no trigger selected."]
    pub const Hwts20: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts21: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts22: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts24: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts28: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts216: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts232: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts264: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts2128: Self = Self(0x80);
}
impl Trig1Chain32Hwts2 {
    pub const fn from_bits(val: u8) -> Trig1Chain32Hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain32Hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts20"),
            0x01 => f.write_str("Hwts21"),
            0x02 => f.write_str("Hwts22"),
            0x04 => f.write_str("Hwts24"),
            0x08 => f.write_str("Hwts28"),
            0x10 => f.write_str("Hwts216"),
            0x20 => f.write_str("Hwts232"),
            0x40 => f.write_str("Hwts264"),
            0x80 => f.write_str("Hwts2128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts20"),
            0x01 => defmt::write!(f, "Hwts21"),
            0x02 => defmt::write!(f, "Hwts22"),
            0x04 => defmt::write!(f, "Hwts24"),
            0x08 => defmt::write!(f, "Hwts28"),
            0x10 => defmt::write!(f, "Hwts216"),
            0x20 => defmt::write!(f, "Hwts232"),
            0x40 => defmt::write!(f, "Hwts264"),
            0x80 => defmt::write!(f, "Hwts2128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain32Hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Hwts2 {
        Trig1Chain32Hwts2::from_bits(val)
    }
}
impl From<Trig1Chain32Hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Hwts2) -> u8 {
        Trig1Chain32Hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain32Hwts3(u8);
impl Trig1Chain32Hwts3 {
    #[doc = "no trigger selected."]
    pub const Hwts30: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts31: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts32: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts34: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts38: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts316: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts332: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts364: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts3128: Self = Self(0x80);
}
impl Trig1Chain32Hwts3 {
    pub const fn from_bits(val: u8) -> Trig1Chain32Hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain32Hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts30"),
            0x01 => f.write_str("Hwts31"),
            0x02 => f.write_str("Hwts32"),
            0x04 => f.write_str("Hwts34"),
            0x08 => f.write_str("Hwts38"),
            0x10 => f.write_str("Hwts316"),
            0x20 => f.write_str("Hwts332"),
            0x40 => f.write_str("Hwts364"),
            0x80 => f.write_str("Hwts3128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts30"),
            0x01 => defmt::write!(f, "Hwts31"),
            0x02 => defmt::write!(f, "Hwts32"),
            0x04 => defmt::write!(f, "Hwts34"),
            0x08 => defmt::write!(f, "Hwts38"),
            0x10 => defmt::write!(f, "Hwts316"),
            0x20 => defmt::write!(f, "Hwts332"),
            0x40 => defmt::write!(f, "Hwts364"),
            0x80 => defmt::write!(f, "Hwts3128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain32Hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Hwts3 {
        Trig1Chain32Hwts3::from_bits(val)
    }
}
impl From<Trig1Chain32Hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Hwts3) -> u8 {
        Trig1Chain32Hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Ie2 {
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    Ie20 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    Ie21 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    Ie22 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    Ie23 = 0x03,
}
impl Trig1Chain32Ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Ie2 {
        Trig1Chain32Ie2::from_bits(val)
    }
}
impl From<Trig1Chain32Ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Ie2) -> u8 {
        Trig1Chain32Ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Ie2En {
    #[doc = "Interrupt DONE disabled."]
    Ie2En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
    Ie2En1 = 0x01,
}
impl Trig1Chain32Ie2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Ie2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Ie2En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Ie2En {
        Trig1Chain32Ie2En::from_bits(val)
    }
}
impl From<Trig1Chain32Ie2En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Ie2En) -> u8 {
        Trig1Chain32Ie2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    Ie30 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    Ie31 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    Ie32 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    Ie33 = 0x03,
}
impl Trig1Chain32Ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Ie3 {
        Trig1Chain32Ie3::from_bits(val)
    }
}
impl From<Trig1Chain32Ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Ie3) -> u8 {
        Trig1Chain32Ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain32Ie3En {
    #[doc = "Interrupt DONE disabled."]
    Ie3En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
    Ie3En1 = 0x01,
}
impl Trig1Chain32Ie3En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain32Ie3En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain32Ie3En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain32Ie3En {
        Trig1Chain32Ie3En::from_bits(val)
    }
}
impl From<Trig1Chain32Ie3En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain32Ie3En) -> u8 {
        Trig1Chain32Ie3En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54B2b4 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b40 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b41 = 0x01,
}
impl Trig1Chain54B2b4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54B2b4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54B2b4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54B2b4 {
        Trig1Chain54B2b4::from_bits(val)
    }
}
impl From<Trig1Chain54B2b4> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54B2b4) -> u8 {
        Trig1Chain54B2b4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54B2b5 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b50 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b51 = 0x01,
}
impl Trig1Chain54B2b5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54B2b5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54B2b5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54B2b5 {
        Trig1Chain54B2b5::from_bits(val)
    }
}
impl From<Trig1Chain54B2b5> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54B2b5) -> u8 {
        Trig1Chain54B2b5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Csel4 {
    #[doc = "ADC Channel 0 selected."]
    Csel40 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel41 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel42 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel43 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel44 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel45 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel46 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel47 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel48 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel49 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel410 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel411 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel412 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel413 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel414 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel415 = 0x0f,
}
impl Trig1Chain54Csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Csel4 {
        Trig1Chain54Csel4::from_bits(val)
    }
}
impl From<Trig1Chain54Csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Csel4) -> u8 {
        Trig1Chain54Csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Csel5 {
    #[doc = "ADC Channel 0 selected."]
    Csel50 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel51 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel52 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel53 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel54 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel55 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel56 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel57 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel58 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel59 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel510 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel511 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel512 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel513 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel514 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel515 = 0x0f,
}
impl Trig1Chain54Csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Csel5 {
        Trig1Chain54Csel5::from_bits(val)
    }
}
impl From<Trig1Chain54Csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Csel5) -> u8 {
        Trig1Chain54Csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain54Hwts4(u8);
impl Trig1Chain54Hwts4 {
    #[doc = "no trigger selected."]
    pub const Hwts40: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts41: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts42: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts44: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts48: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts416: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts432: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts464: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts4128: Self = Self(0x80);
}
impl Trig1Chain54Hwts4 {
    pub const fn from_bits(val: u8) -> Trig1Chain54Hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain54Hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts40"),
            0x01 => f.write_str("Hwts41"),
            0x02 => f.write_str("Hwts42"),
            0x04 => f.write_str("Hwts44"),
            0x08 => f.write_str("Hwts48"),
            0x10 => f.write_str("Hwts416"),
            0x20 => f.write_str("Hwts432"),
            0x40 => f.write_str("Hwts464"),
            0x80 => f.write_str("Hwts4128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts40"),
            0x01 => defmt::write!(f, "Hwts41"),
            0x02 => defmt::write!(f, "Hwts42"),
            0x04 => defmt::write!(f, "Hwts44"),
            0x08 => defmt::write!(f, "Hwts48"),
            0x10 => defmt::write!(f, "Hwts416"),
            0x20 => defmt::write!(f, "Hwts432"),
            0x40 => defmt::write!(f, "Hwts464"),
            0x80 => defmt::write!(f, "Hwts4128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain54Hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Hwts4 {
        Trig1Chain54Hwts4::from_bits(val)
    }
}
impl From<Trig1Chain54Hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Hwts4) -> u8 {
        Trig1Chain54Hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain54Hwts5(u8);
impl Trig1Chain54Hwts5 {
    #[doc = "no trigger selected."]
    pub const Hwts50: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts51: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts52: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts54: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts58: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts516: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts532: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts564: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts5128: Self = Self(0x80);
}
impl Trig1Chain54Hwts5 {
    pub const fn from_bits(val: u8) -> Trig1Chain54Hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain54Hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts50"),
            0x01 => f.write_str("Hwts51"),
            0x02 => f.write_str("Hwts52"),
            0x04 => f.write_str("Hwts54"),
            0x08 => f.write_str("Hwts58"),
            0x10 => f.write_str("Hwts516"),
            0x20 => f.write_str("Hwts532"),
            0x40 => f.write_str("Hwts564"),
            0x80 => f.write_str("Hwts5128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts50"),
            0x01 => defmt::write!(f, "Hwts51"),
            0x02 => defmt::write!(f, "Hwts52"),
            0x04 => defmt::write!(f, "Hwts54"),
            0x08 => defmt::write!(f, "Hwts58"),
            0x10 => defmt::write!(f, "Hwts516"),
            0x20 => defmt::write!(f, "Hwts532"),
            0x40 => defmt::write!(f, "Hwts564"),
            0x80 => defmt::write!(f, "Hwts5128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain54Hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Hwts5 {
        Trig1Chain54Hwts5::from_bits(val)
    }
}
impl From<Trig1Chain54Hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Hwts5) -> u8 {
        Trig1Chain54Hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Ie4 {
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    Ie40 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    Ie41 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    Ie42 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    Ie43 = 0x03,
}
impl Trig1Chain54Ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Ie4 {
        Trig1Chain54Ie4::from_bits(val)
    }
}
impl From<Trig1Chain54Ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Ie4) -> u8 {
        Trig1Chain54Ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Ie4En {
    #[doc = "Interrupt DONE disabled."]
    Ie4En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
    Ie4En1 = 0x01,
}
impl Trig1Chain54Ie4En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Ie4En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Ie4En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Ie4En {
        Trig1Chain54Ie4En::from_bits(val)
    }
}
impl From<Trig1Chain54Ie4En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Ie4En) -> u8 {
        Trig1Chain54Ie4En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    Ie50 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    Ie51 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    Ie52 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    Ie53 = 0x03,
}
impl Trig1Chain54Ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Ie5 {
        Trig1Chain54Ie5::from_bits(val)
    }
}
impl From<Trig1Chain54Ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Ie5) -> u8 {
        Trig1Chain54Ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain54Ie5En {
    #[doc = "Interrupt DONE disabled."]
    Ie5En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
    Ie5En1 = 0x01,
}
impl Trig1Chain54Ie5En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain54Ie5En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain54Ie5En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain54Ie5En {
        Trig1Chain54Ie5En::from_bits(val)
    }
}
impl From<Trig1Chain54Ie5En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain54Ie5En) -> u8 {
        Trig1Chain54Ie5En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76B2b6 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b60 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b61 = 0x01,
}
impl Trig1Chain76B2b6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76B2b6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76B2b6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76B2b6 {
        Trig1Chain76B2b6::from_bits(val)
    }
}
impl From<Trig1Chain76B2b6> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76B2b6) -> u8 {
        Trig1Chain76B2b6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76B2b7 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b70 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b71 = 0x01,
}
impl Trig1Chain76B2b7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76B2b7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76B2b7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76B2b7 {
        Trig1Chain76B2b7::from_bits(val)
    }
}
impl From<Trig1Chain76B2b7> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76B2b7) -> u8 {
        Trig1Chain76B2b7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Csel6 {
    #[doc = "ADC Channel 0 selected."]
    Csel60 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel61 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel62 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel63 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel64 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel65 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel66 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel67 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel68 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel69 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel610 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel611 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel612 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel613 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel614 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel615 = 0x0f,
}
impl Trig1Chain76Csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Csel6 {
        Trig1Chain76Csel6::from_bits(val)
    }
}
impl From<Trig1Chain76Csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Csel6) -> u8 {
        Trig1Chain76Csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Csel7 {
    #[doc = "ADC Channel 0 selected."]
    Csel70 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel71 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel72 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel73 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel74 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel75 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel76 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel77 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel78 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel79 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel710 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel711 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel712 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel713 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel714 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel715 = 0x0f,
}
impl Trig1Chain76Csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Csel7 {
        Trig1Chain76Csel7::from_bits(val)
    }
}
impl From<Trig1Chain76Csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Csel7) -> u8 {
        Trig1Chain76Csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain76Hwts6(u8);
impl Trig1Chain76Hwts6 {
    #[doc = "no trigger selected."]
    pub const Hwts60: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts61: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts62: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts64: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts68: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts616: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts632: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts664: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts6128: Self = Self(0x80);
}
impl Trig1Chain76Hwts6 {
    pub const fn from_bits(val: u8) -> Trig1Chain76Hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain76Hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts60"),
            0x01 => f.write_str("Hwts61"),
            0x02 => f.write_str("Hwts62"),
            0x04 => f.write_str("Hwts64"),
            0x08 => f.write_str("Hwts68"),
            0x10 => f.write_str("Hwts616"),
            0x20 => f.write_str("Hwts632"),
            0x40 => f.write_str("Hwts664"),
            0x80 => f.write_str("Hwts6128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts60"),
            0x01 => defmt::write!(f, "Hwts61"),
            0x02 => defmt::write!(f, "Hwts62"),
            0x04 => defmt::write!(f, "Hwts64"),
            0x08 => defmt::write!(f, "Hwts68"),
            0x10 => defmt::write!(f, "Hwts616"),
            0x20 => defmt::write!(f, "Hwts632"),
            0x40 => defmt::write!(f, "Hwts664"),
            0x80 => defmt::write!(f, "Hwts6128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain76Hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Hwts6 {
        Trig1Chain76Hwts6::from_bits(val)
    }
}
impl From<Trig1Chain76Hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Hwts6) -> u8 {
        Trig1Chain76Hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1Chain76Hwts7(u8);
impl Trig1Chain76Hwts7 {
    #[doc = "no trigger selected."]
    pub const Hwts70: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts71: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts72: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts74: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts78: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts716: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts732: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts764: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts7128: Self = Self(0x80);
}
impl Trig1Chain76Hwts7 {
    pub const fn from_bits(val: u8) -> Trig1Chain76Hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1Chain76Hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts70"),
            0x01 => f.write_str("Hwts71"),
            0x02 => f.write_str("Hwts72"),
            0x04 => f.write_str("Hwts74"),
            0x08 => f.write_str("Hwts78"),
            0x10 => f.write_str("Hwts716"),
            0x20 => f.write_str("Hwts732"),
            0x40 => f.write_str("Hwts764"),
            0x80 => f.write_str("Hwts7128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts70"),
            0x01 => defmt::write!(f, "Hwts71"),
            0x02 => defmt::write!(f, "Hwts72"),
            0x04 => defmt::write!(f, "Hwts74"),
            0x08 => defmt::write!(f, "Hwts78"),
            0x10 => defmt::write!(f, "Hwts716"),
            0x20 => defmt::write!(f, "Hwts732"),
            0x40 => defmt::write!(f, "Hwts764"),
            0x80 => defmt::write!(f, "Hwts7128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1Chain76Hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Hwts7 {
        Trig1Chain76Hwts7::from_bits(val)
    }
}
impl From<Trig1Chain76Hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Hwts7) -> u8 {
        Trig1Chain76Hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Ie6 {
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    Ie60 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    Ie61 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    Ie62 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    Ie63 = 0x03,
}
impl Trig1Chain76Ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Ie6 {
        Trig1Chain76Ie6::from_bits(val)
    }
}
impl From<Trig1Chain76Ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Ie6) -> u8 {
        Trig1Chain76Ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Ie6En {
    #[doc = "Interrupt DONE disabled."]
    Ie6En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
    Ie6En1 = 0x01,
}
impl Trig1Chain76Ie6En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Ie6En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Ie6En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Ie6En {
        Trig1Chain76Ie6En::from_bits(val)
    }
}
impl From<Trig1Chain76Ie6En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Ie6En) -> u8 {
        Trig1Chain76Ie6En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    Ie70 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    Ie71 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    Ie72 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    Ie73 = 0x03,
}
impl Trig1Chain76Ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Ie7 {
        Trig1Chain76Ie7::from_bits(val)
    }
}
impl From<Trig1Chain76Ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Ie7) -> u8 {
        Trig1Chain76Ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Chain76Ie7En {
    #[doc = "Interrupt DONE disabled."]
    Ie7En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
    Ie7En1 = 0x01,
}
impl Trig1Chain76Ie7En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Chain76Ie7En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Chain76Ie7En {
    #[inline(always)]
    fn from(val: u8) -> Trig1Chain76Ie7En {
        Trig1Chain76Ie7En::from_bits(val)
    }
}
impl From<Trig1Chain76Ie7En> for u8 {
    #[inline(always)]
    fn from(val: Trig1Chain76Ie7En) -> u8 {
        Trig1Chain76Ie7En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1CtrlSwTrig {
    #[doc = "No software trigger event generated."]
    SwTrig0 = 0x0,
    #[doc = "Software trigger event generated."]
    SwTrig1 = 0x01,
}
impl Trig1CtrlSwTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1CtrlSwTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1CtrlSwTrig {
    #[inline(always)]
    fn from(val: u8) -> Trig1CtrlSwTrig {
        Trig1CtrlSwTrig::from_bits(val)
    }
}
impl From<Trig1CtrlSwTrig> for u8 {
    #[inline(always)]
    fn from(val: Trig1CtrlSwTrig) -> u8 {
        Trig1CtrlSwTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SyncMode0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SyncMode1 = 0x01,
}
impl Trig1CtrlSyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1CtrlSyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1CtrlSyncMode {
    #[inline(always)]
    fn from(val: u8) -> Trig1CtrlSyncMode {
        Trig1CtrlSyncMode::from_bits(val)
    }
}
impl From<Trig1CtrlSyncMode> for u8 {
    #[inline(always)]
    fn from(val: Trig1CtrlSyncMode) -> u8 {
        Trig1CtrlSyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1CtrlTrigChain {
    #[doc = "Trigger chain length is 1."]
    TrigChain0 = 0x0,
    #[doc = "Trigger chain length is 2."]
    TrigChain1 = 0x01,
    #[doc = "Trigger chain length is 3."]
    TrigChain2 = 0x02,
    #[doc = "Trigger chain length is 4."]
    TrigChain3 = 0x03,
    #[doc = "Trigger chain length is 5."]
    TrigChain4 = 0x04,
    #[doc = "Trigger chain length is 6."]
    TrigChain5 = 0x05,
    #[doc = "Trigger chain length is 7."]
    TrigChain6 = 0x06,
    #[doc = "Trigger chain length is 8."]
    TrigChain7 = 0x07,
}
impl Trig1CtrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1CtrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1CtrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig1CtrlTrigChain {
        Trig1CtrlTrigChain::from_bits(val)
    }
}
impl From<Trig1CtrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig1CtrlTrigChain) -> u8 {
        Trig1CtrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1CtrlTrigMode {
    #[doc = "Hardware trigger. The softerware trigger will be ignored."]
    TrigMode0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TrigMode1 = 0x01,
}
impl Trig1CtrlTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1CtrlTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1CtrlTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Trig1CtrlTrigMode {
        Trig1CtrlTrigMode::from_bits(val)
    }
}
impl From<Trig1CtrlTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Trig1CtrlTrigMode) -> u8 {
        Trig1CtrlTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Done0 {
    #[doc = "No TRIG1_DONE0 interrupt detected."]
    Trig1Done00 = 0x0,
    #[doc = "TRIG1_DONE0 interrupt detected."]
    Trig1Done01 = 0x01,
}
impl Trig1Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Done0 {
        Trig1Done0::from_bits(val)
    }
}
impl From<Trig1Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig1Done0) -> u8 {
        Trig1Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Done1 {
    #[doc = "No TRIG1_DONE1 interrupt detected."]
    Trig1Done10 = 0x0,
    #[doc = "TRIG1_DONE1 interrupt detected."]
    Trig1Done11 = 0x01,
}
impl Trig1Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Done1 {
        Trig1Done1::from_bits(val)
    }
}
impl From<Trig1Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig1Done1) -> u8 {
        Trig1Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Done2 {
    #[doc = "No TRIG1_DONE2 interrupt detected."]
    Trig1Done20 = 0x0,
    #[doc = "TRIG1_DONE2 interrupt detected."]
    Trig1Done21 = 0x01,
}
impl Trig1Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Done2 {
        Trig1Done2::from_bits(val)
    }
}
impl From<Trig1Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig1Done2) -> u8 {
        Trig1Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Done3 {
    #[doc = "No TRIG1_DONE3 interrupt detected."]
    Trig1Done30 = 0x0,
    #[doc = "TRIG1_DONE3 interrupt detected."]
    Trig1Done31 = 0x01,
}
impl Trig1Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1Done3 {
        Trig1Done3::from_bits(val)
    }
}
impl From<Trig1Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig1Done3) -> u8 {
        Trig1Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Enable {
    #[doc = "TRIG1 DMA request disabled."]
    Trig1Enable0 = 0x0,
    #[doc = "TRIG1 DMA request enabled."]
    Trig1Enable1 = 0x01,
}
impl Trig1Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig1Enable {
        Trig1Enable::from_bits(val)
    }
}
impl From<Trig1Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig1Enable) -> u8 {
        Trig1Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Err {
    #[doc = "No TRIG1_ERR interrupt detected."]
    Trig1Err0 = 0x0,
    #[doc = "TRIG1_ERR interrupt detected."]
    Trig1Err1 = 0x01,
}
impl Trig1Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Err {
    #[inline(always)]
    fn from(val: u8) -> Trig1Err {
        Trig1Err::from_bits(val)
    }
}
impl From<Trig1Err> for u8 {
    #[inline(always)]
    fn from(val: Trig1Err) -> u8 {
        Trig1Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1Req {
    #[doc = "TRIG1_REQ not detected."]
    Trig1Req0 = 0x0,
    #[doc = "TRIG1_REQ detected."]
    Trig1Req1 = 0x01,
}
impl Trig1Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1Req {
    #[inline(always)]
    fn from(val: u8) -> Trig1Req {
        Trig1Req::from_bits(val)
    }
}
impl From<Trig1Req> for u8 {
    #[inline(always)]
    fn from(val: Trig1Req) -> u8 {
        Trig1Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10B2b0 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b00 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b01 = 0x01,
}
impl Trig2Chain10B2b0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10B2b0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10B2b0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10B2b0 {
        Trig2Chain10B2b0::from_bits(val)
    }
}
impl From<Trig2Chain10B2b0> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10B2b0) -> u8 {
        Trig2Chain10B2b0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10B2b1 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b10 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b11 = 0x01,
}
impl Trig2Chain10B2b1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10B2b1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10B2b1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10B2b1 {
        Trig2Chain10B2b1::from_bits(val)
    }
}
impl From<Trig2Chain10B2b1> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10B2b1) -> u8 {
        Trig2Chain10B2b1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Csel0 {
    #[doc = "ADC Channel 0 selected."]
    Csel00 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel01 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel02 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel03 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel04 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel05 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel06 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel07 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel08 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel09 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel010 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel011 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel012 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel013 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel014 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel015 = 0x0f,
}
impl Trig2Chain10Csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Csel0 {
        Trig2Chain10Csel0::from_bits(val)
    }
}
impl From<Trig2Chain10Csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Csel0) -> u8 {
        Trig2Chain10Csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Csel1 {
    #[doc = "ADC Channel 0 selected."]
    Csel10 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel11 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel12 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel13 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel14 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel15 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel16 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel17 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel18 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel19 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel110 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel111 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel112 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel113 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel114 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel115 = 0x0f,
}
impl Trig2Chain10Csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Csel1 {
        Trig2Chain10Csel1::from_bits(val)
    }
}
impl From<Trig2Chain10Csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Csel1) -> u8 {
        Trig2Chain10Csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain10Hwts0(u8);
impl Trig2Chain10Hwts0 {
    #[doc = "no trigger selected."]
    pub const Hwts00: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts01: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts02: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts04: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts08: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts016: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts032: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts064: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts0128: Self = Self(0x80);
}
impl Trig2Chain10Hwts0 {
    pub const fn from_bits(val: u8) -> Trig2Chain10Hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain10Hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts00"),
            0x01 => f.write_str("Hwts01"),
            0x02 => f.write_str("Hwts02"),
            0x04 => f.write_str("Hwts04"),
            0x08 => f.write_str("Hwts08"),
            0x10 => f.write_str("Hwts016"),
            0x20 => f.write_str("Hwts032"),
            0x40 => f.write_str("Hwts064"),
            0x80 => f.write_str("Hwts0128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts00"),
            0x01 => defmt::write!(f, "Hwts01"),
            0x02 => defmt::write!(f, "Hwts02"),
            0x04 => defmt::write!(f, "Hwts04"),
            0x08 => defmt::write!(f, "Hwts08"),
            0x10 => defmt::write!(f, "Hwts016"),
            0x20 => defmt::write!(f, "Hwts032"),
            0x40 => defmt::write!(f, "Hwts064"),
            0x80 => defmt::write!(f, "Hwts0128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain10Hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Hwts0 {
        Trig2Chain10Hwts0::from_bits(val)
    }
}
impl From<Trig2Chain10Hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Hwts0) -> u8 {
        Trig2Chain10Hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain10Hwts1(u8);
impl Trig2Chain10Hwts1 {
    #[doc = "no trigger selected."]
    pub const Hwts10: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts11: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts12: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts14: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts18: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts116: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts132: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts164: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts1128: Self = Self(0x80);
}
impl Trig2Chain10Hwts1 {
    pub const fn from_bits(val: u8) -> Trig2Chain10Hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain10Hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts10"),
            0x01 => f.write_str("Hwts11"),
            0x02 => f.write_str("Hwts12"),
            0x04 => f.write_str("Hwts14"),
            0x08 => f.write_str("Hwts18"),
            0x10 => f.write_str("Hwts116"),
            0x20 => f.write_str("Hwts132"),
            0x40 => f.write_str("Hwts164"),
            0x80 => f.write_str("Hwts1128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts10"),
            0x01 => defmt::write!(f, "Hwts11"),
            0x02 => defmt::write!(f, "Hwts12"),
            0x04 => defmt::write!(f, "Hwts14"),
            0x08 => defmt::write!(f, "Hwts18"),
            0x10 => defmt::write!(f, "Hwts116"),
            0x20 => defmt::write!(f, "Hwts132"),
            0x40 => defmt::write!(f, "Hwts164"),
            0x80 => defmt::write!(f, "Hwts1128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain10Hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Hwts1 {
        Trig2Chain10Hwts1::from_bits(val)
    }
}
impl From<Trig2Chain10Hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Hwts1) -> u8 {
        Trig2Chain10Hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Ie0 {
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    Ie00 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    Ie01 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    Ie02 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    Ie03 = 0x03,
}
impl Trig2Chain10Ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Ie0 {
        Trig2Chain10Ie0::from_bits(val)
    }
}
impl From<Trig2Chain10Ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Ie0) -> u8 {
        Trig2Chain10Ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Ie0En {
    #[doc = "Interrupt DONE disabled."]
    Ie0En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
    Ie0En1 = 0x01,
}
impl Trig2Chain10Ie0En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Ie0En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Ie0En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Ie0En {
        Trig2Chain10Ie0En::from_bits(val)
    }
}
impl From<Trig2Chain10Ie0En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Ie0En) -> u8 {
        Trig2Chain10Ie0En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    Ie10 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    Ie11 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    Ie12 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    Ie13 = 0x03,
}
impl Trig2Chain10Ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Ie1 {
        Trig2Chain10Ie1::from_bits(val)
    }
}
impl From<Trig2Chain10Ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Ie1) -> u8 {
        Trig2Chain10Ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain10Ie1En {
    #[doc = "Interrupt DONE disabled."]
    Ie1En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
    Ie1En1 = 0x01,
}
impl Trig2Chain10Ie1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain10Ie1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain10Ie1En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain10Ie1En {
        Trig2Chain10Ie1En::from_bits(val)
    }
}
impl From<Trig2Chain10Ie1En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain10Ie1En) -> u8 {
        Trig2Chain10Ie1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32B2b2 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b20 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b21 = 0x01,
}
impl Trig2Chain32B2b2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32B2b2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32B2b2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32B2b2 {
        Trig2Chain32B2b2::from_bits(val)
    }
}
impl From<Trig2Chain32B2b2> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32B2b2) -> u8 {
        Trig2Chain32B2b2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32B2b3 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b30 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b31 = 0x01,
}
impl Trig2Chain32B2b3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32B2b3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32B2b3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32B2b3 {
        Trig2Chain32B2b3::from_bits(val)
    }
}
impl From<Trig2Chain32B2b3> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32B2b3) -> u8 {
        Trig2Chain32B2b3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Csel2 {
    #[doc = "ADC Channel 0 selected."]
    Csel20 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel21 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel22 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel23 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel24 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel25 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel26 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel27 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel28 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel29 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel210 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel211 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel212 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel213 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel214 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel215 = 0x0f,
}
impl Trig2Chain32Csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Csel2 {
        Trig2Chain32Csel2::from_bits(val)
    }
}
impl From<Trig2Chain32Csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Csel2) -> u8 {
        Trig2Chain32Csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Csel3 {
    #[doc = "ADC Channel 0 selected."]
    Csel30 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel31 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel32 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel33 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel34 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel35 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel36 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel37 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel38 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel39 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel310 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel311 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel312 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel313 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel314 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel315 = 0x0f,
}
impl Trig2Chain32Csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Csel3 {
        Trig2Chain32Csel3::from_bits(val)
    }
}
impl From<Trig2Chain32Csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Csel3) -> u8 {
        Trig2Chain32Csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain32Hwts2(u8);
impl Trig2Chain32Hwts2 {
    #[doc = "no trigger selected."]
    pub const Hwts20: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts21: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts22: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts24: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts28: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts216: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts232: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts264: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts2128: Self = Self(0x80);
}
impl Trig2Chain32Hwts2 {
    pub const fn from_bits(val: u8) -> Trig2Chain32Hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain32Hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts20"),
            0x01 => f.write_str("Hwts21"),
            0x02 => f.write_str("Hwts22"),
            0x04 => f.write_str("Hwts24"),
            0x08 => f.write_str("Hwts28"),
            0x10 => f.write_str("Hwts216"),
            0x20 => f.write_str("Hwts232"),
            0x40 => f.write_str("Hwts264"),
            0x80 => f.write_str("Hwts2128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts20"),
            0x01 => defmt::write!(f, "Hwts21"),
            0x02 => defmt::write!(f, "Hwts22"),
            0x04 => defmt::write!(f, "Hwts24"),
            0x08 => defmt::write!(f, "Hwts28"),
            0x10 => defmt::write!(f, "Hwts216"),
            0x20 => defmt::write!(f, "Hwts232"),
            0x40 => defmt::write!(f, "Hwts264"),
            0x80 => defmt::write!(f, "Hwts2128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain32Hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Hwts2 {
        Trig2Chain32Hwts2::from_bits(val)
    }
}
impl From<Trig2Chain32Hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Hwts2) -> u8 {
        Trig2Chain32Hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain32Hwts3(u8);
impl Trig2Chain32Hwts3 {
    #[doc = "no trigger selected."]
    pub const Hwts30: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts31: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts32: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts34: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts38: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts316: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts332: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts364: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts3128: Self = Self(0x80);
}
impl Trig2Chain32Hwts3 {
    pub const fn from_bits(val: u8) -> Trig2Chain32Hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain32Hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts30"),
            0x01 => f.write_str("Hwts31"),
            0x02 => f.write_str("Hwts32"),
            0x04 => f.write_str("Hwts34"),
            0x08 => f.write_str("Hwts38"),
            0x10 => f.write_str("Hwts316"),
            0x20 => f.write_str("Hwts332"),
            0x40 => f.write_str("Hwts364"),
            0x80 => f.write_str("Hwts3128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts30"),
            0x01 => defmt::write!(f, "Hwts31"),
            0x02 => defmt::write!(f, "Hwts32"),
            0x04 => defmt::write!(f, "Hwts34"),
            0x08 => defmt::write!(f, "Hwts38"),
            0x10 => defmt::write!(f, "Hwts316"),
            0x20 => defmt::write!(f, "Hwts332"),
            0x40 => defmt::write!(f, "Hwts364"),
            0x80 => defmt::write!(f, "Hwts3128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain32Hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Hwts3 {
        Trig2Chain32Hwts3::from_bits(val)
    }
}
impl From<Trig2Chain32Hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Hwts3) -> u8 {
        Trig2Chain32Hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Ie2 {
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    Ie20 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    Ie21 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    Ie22 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    Ie23 = 0x03,
}
impl Trig2Chain32Ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Ie2 {
        Trig2Chain32Ie2::from_bits(val)
    }
}
impl From<Trig2Chain32Ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Ie2) -> u8 {
        Trig2Chain32Ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Ie2En {
    #[doc = "Interrupt DONE disabled."]
    Ie2En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
    Ie2En1 = 0x01,
}
impl Trig2Chain32Ie2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Ie2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Ie2En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Ie2En {
        Trig2Chain32Ie2En::from_bits(val)
    }
}
impl From<Trig2Chain32Ie2En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Ie2En) -> u8 {
        Trig2Chain32Ie2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    Ie30 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    Ie31 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    Ie32 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    Ie33 = 0x03,
}
impl Trig2Chain32Ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Ie3 {
        Trig2Chain32Ie3::from_bits(val)
    }
}
impl From<Trig2Chain32Ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Ie3) -> u8 {
        Trig2Chain32Ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain32Ie3En {
    #[doc = "Interrupt DONE disabled."]
    Ie3En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
    Ie3En1 = 0x01,
}
impl Trig2Chain32Ie3En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain32Ie3En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain32Ie3En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain32Ie3En {
        Trig2Chain32Ie3En::from_bits(val)
    }
}
impl From<Trig2Chain32Ie3En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain32Ie3En) -> u8 {
        Trig2Chain32Ie3En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54B2b4 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b40 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b41 = 0x01,
}
impl Trig2Chain54B2b4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54B2b4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54B2b4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54B2b4 {
        Trig2Chain54B2b4::from_bits(val)
    }
}
impl From<Trig2Chain54B2b4> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54B2b4) -> u8 {
        Trig2Chain54B2b4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54B2b5 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b50 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b51 = 0x01,
}
impl Trig2Chain54B2b5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54B2b5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54B2b5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54B2b5 {
        Trig2Chain54B2b5::from_bits(val)
    }
}
impl From<Trig2Chain54B2b5> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54B2b5) -> u8 {
        Trig2Chain54B2b5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Csel4 {
    #[doc = "ADC Channel 0 selected."]
    Csel40 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel41 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel42 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel43 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel44 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel45 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel46 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel47 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel48 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel49 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel410 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel411 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel412 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel413 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel414 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel415 = 0x0f,
}
impl Trig2Chain54Csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Csel4 {
        Trig2Chain54Csel4::from_bits(val)
    }
}
impl From<Trig2Chain54Csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Csel4) -> u8 {
        Trig2Chain54Csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Csel5 {
    #[doc = "ADC Channel 0 selected."]
    Csel50 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel51 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel52 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel53 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel54 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel55 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel56 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel57 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel58 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel59 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel510 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel511 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel512 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel513 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel514 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel515 = 0x0f,
}
impl Trig2Chain54Csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Csel5 {
        Trig2Chain54Csel5::from_bits(val)
    }
}
impl From<Trig2Chain54Csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Csel5) -> u8 {
        Trig2Chain54Csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain54Hwts4(u8);
impl Trig2Chain54Hwts4 {
    #[doc = "no trigger selected."]
    pub const Hwts40: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts41: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts42: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts44: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts48: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts416: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts432: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts464: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts4128: Self = Self(0x80);
}
impl Trig2Chain54Hwts4 {
    pub const fn from_bits(val: u8) -> Trig2Chain54Hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain54Hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts40"),
            0x01 => f.write_str("Hwts41"),
            0x02 => f.write_str("Hwts42"),
            0x04 => f.write_str("Hwts44"),
            0x08 => f.write_str("Hwts48"),
            0x10 => f.write_str("Hwts416"),
            0x20 => f.write_str("Hwts432"),
            0x40 => f.write_str("Hwts464"),
            0x80 => f.write_str("Hwts4128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts40"),
            0x01 => defmt::write!(f, "Hwts41"),
            0x02 => defmt::write!(f, "Hwts42"),
            0x04 => defmt::write!(f, "Hwts44"),
            0x08 => defmt::write!(f, "Hwts48"),
            0x10 => defmt::write!(f, "Hwts416"),
            0x20 => defmt::write!(f, "Hwts432"),
            0x40 => defmt::write!(f, "Hwts464"),
            0x80 => defmt::write!(f, "Hwts4128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain54Hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Hwts4 {
        Trig2Chain54Hwts4::from_bits(val)
    }
}
impl From<Trig2Chain54Hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Hwts4) -> u8 {
        Trig2Chain54Hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain54Hwts5(u8);
impl Trig2Chain54Hwts5 {
    #[doc = "no trigger selected."]
    pub const Hwts50: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts51: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts52: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts54: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts58: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts516: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts532: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts564: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts5128: Self = Self(0x80);
}
impl Trig2Chain54Hwts5 {
    pub const fn from_bits(val: u8) -> Trig2Chain54Hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain54Hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts50"),
            0x01 => f.write_str("Hwts51"),
            0x02 => f.write_str("Hwts52"),
            0x04 => f.write_str("Hwts54"),
            0x08 => f.write_str("Hwts58"),
            0x10 => f.write_str("Hwts516"),
            0x20 => f.write_str("Hwts532"),
            0x40 => f.write_str("Hwts564"),
            0x80 => f.write_str("Hwts5128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts50"),
            0x01 => defmt::write!(f, "Hwts51"),
            0x02 => defmt::write!(f, "Hwts52"),
            0x04 => defmt::write!(f, "Hwts54"),
            0x08 => defmt::write!(f, "Hwts58"),
            0x10 => defmt::write!(f, "Hwts516"),
            0x20 => defmt::write!(f, "Hwts532"),
            0x40 => defmt::write!(f, "Hwts564"),
            0x80 => defmt::write!(f, "Hwts5128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain54Hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Hwts5 {
        Trig2Chain54Hwts5::from_bits(val)
    }
}
impl From<Trig2Chain54Hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Hwts5) -> u8 {
        Trig2Chain54Hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Ie4 {
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    Ie40 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    Ie41 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    Ie42 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    Ie43 = 0x03,
}
impl Trig2Chain54Ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Ie4 {
        Trig2Chain54Ie4::from_bits(val)
    }
}
impl From<Trig2Chain54Ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Ie4) -> u8 {
        Trig2Chain54Ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Ie4En {
    #[doc = "Interrupt DONE disabled."]
    Ie4En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
    Ie4En1 = 0x01,
}
impl Trig2Chain54Ie4En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Ie4En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Ie4En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Ie4En {
        Trig2Chain54Ie4En::from_bits(val)
    }
}
impl From<Trig2Chain54Ie4En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Ie4En) -> u8 {
        Trig2Chain54Ie4En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    Ie50 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    Ie51 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    Ie52 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    Ie53 = 0x03,
}
impl Trig2Chain54Ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Ie5 {
        Trig2Chain54Ie5::from_bits(val)
    }
}
impl From<Trig2Chain54Ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Ie5) -> u8 {
        Trig2Chain54Ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain54Ie5En {
    #[doc = "Interrupt DONE disabled."]
    Ie5En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
    Ie5En1 = 0x01,
}
impl Trig2Chain54Ie5En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain54Ie5En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain54Ie5En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain54Ie5En {
        Trig2Chain54Ie5En::from_bits(val)
    }
}
impl From<Trig2Chain54Ie5En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain54Ie5En) -> u8 {
        Trig2Chain54Ie5En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76B2b6 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b60 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b61 = 0x01,
}
impl Trig2Chain76B2b6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76B2b6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76B2b6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76B2b6 {
        Trig2Chain76B2b6::from_bits(val)
    }
}
impl From<Trig2Chain76B2b6> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76B2b6) -> u8 {
        Trig2Chain76B2b6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76B2b7 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b70 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b71 = 0x01,
}
impl Trig2Chain76B2b7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76B2b7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76B2b7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76B2b7 {
        Trig2Chain76B2b7::from_bits(val)
    }
}
impl From<Trig2Chain76B2b7> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76B2b7) -> u8 {
        Trig2Chain76B2b7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Csel6 {
    #[doc = "ADC Channel 0 selected."]
    Csel60 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel61 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel62 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel63 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel64 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel65 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel66 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel67 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel68 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel69 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel610 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel611 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel612 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel613 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel614 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel615 = 0x0f,
}
impl Trig2Chain76Csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Csel6 {
        Trig2Chain76Csel6::from_bits(val)
    }
}
impl From<Trig2Chain76Csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Csel6) -> u8 {
        Trig2Chain76Csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Csel7 {
    #[doc = "ADC Channel 0 selected."]
    Csel70 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel71 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel72 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel73 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel74 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel75 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel76 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel77 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel78 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel79 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel710 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel711 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel712 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel713 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel714 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel715 = 0x0f,
}
impl Trig2Chain76Csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Csel7 {
        Trig2Chain76Csel7::from_bits(val)
    }
}
impl From<Trig2Chain76Csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Csel7) -> u8 {
        Trig2Chain76Csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain76Hwts6(u8);
impl Trig2Chain76Hwts6 {
    #[doc = "no trigger selected."]
    pub const Hwts60: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts61: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts62: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts64: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts68: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts616: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts632: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts664: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts6128: Self = Self(0x80);
}
impl Trig2Chain76Hwts6 {
    pub const fn from_bits(val: u8) -> Trig2Chain76Hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain76Hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts60"),
            0x01 => f.write_str("Hwts61"),
            0x02 => f.write_str("Hwts62"),
            0x04 => f.write_str("Hwts64"),
            0x08 => f.write_str("Hwts68"),
            0x10 => f.write_str("Hwts616"),
            0x20 => f.write_str("Hwts632"),
            0x40 => f.write_str("Hwts664"),
            0x80 => f.write_str("Hwts6128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts60"),
            0x01 => defmt::write!(f, "Hwts61"),
            0x02 => defmt::write!(f, "Hwts62"),
            0x04 => defmt::write!(f, "Hwts64"),
            0x08 => defmt::write!(f, "Hwts68"),
            0x10 => defmt::write!(f, "Hwts616"),
            0x20 => defmt::write!(f, "Hwts632"),
            0x40 => defmt::write!(f, "Hwts664"),
            0x80 => defmt::write!(f, "Hwts6128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain76Hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Hwts6 {
        Trig2Chain76Hwts6::from_bits(val)
    }
}
impl From<Trig2Chain76Hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Hwts6) -> u8 {
        Trig2Chain76Hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2Chain76Hwts7(u8);
impl Trig2Chain76Hwts7 {
    #[doc = "no trigger selected."]
    pub const Hwts70: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts71: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts72: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts74: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts78: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts716: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts732: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts764: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts7128: Self = Self(0x80);
}
impl Trig2Chain76Hwts7 {
    pub const fn from_bits(val: u8) -> Trig2Chain76Hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2Chain76Hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts70"),
            0x01 => f.write_str("Hwts71"),
            0x02 => f.write_str("Hwts72"),
            0x04 => f.write_str("Hwts74"),
            0x08 => f.write_str("Hwts78"),
            0x10 => f.write_str("Hwts716"),
            0x20 => f.write_str("Hwts732"),
            0x40 => f.write_str("Hwts764"),
            0x80 => f.write_str("Hwts7128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts70"),
            0x01 => defmt::write!(f, "Hwts71"),
            0x02 => defmt::write!(f, "Hwts72"),
            0x04 => defmt::write!(f, "Hwts74"),
            0x08 => defmt::write!(f, "Hwts78"),
            0x10 => defmt::write!(f, "Hwts716"),
            0x20 => defmt::write!(f, "Hwts732"),
            0x40 => defmt::write!(f, "Hwts764"),
            0x80 => defmt::write!(f, "Hwts7128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2Chain76Hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Hwts7 {
        Trig2Chain76Hwts7::from_bits(val)
    }
}
impl From<Trig2Chain76Hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Hwts7) -> u8 {
        Trig2Chain76Hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Ie6 {
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    Ie60 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    Ie61 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    Ie62 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    Ie63 = 0x03,
}
impl Trig2Chain76Ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Ie6 {
        Trig2Chain76Ie6::from_bits(val)
    }
}
impl From<Trig2Chain76Ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Ie6) -> u8 {
        Trig2Chain76Ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Ie6En {
    #[doc = "Interrupt DONE disabled."]
    Ie6En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
    Ie6En1 = 0x01,
}
impl Trig2Chain76Ie6En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Ie6En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Ie6En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Ie6En {
        Trig2Chain76Ie6En::from_bits(val)
    }
}
impl From<Trig2Chain76Ie6En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Ie6En) -> u8 {
        Trig2Chain76Ie6En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    Ie70 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    Ie71 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    Ie72 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    Ie73 = 0x03,
}
impl Trig2Chain76Ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Ie7 {
        Trig2Chain76Ie7::from_bits(val)
    }
}
impl From<Trig2Chain76Ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Ie7) -> u8 {
        Trig2Chain76Ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Chain76Ie7En {
    #[doc = "Interrupt DONE disabled."]
    Ie7En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
    Ie7En1 = 0x01,
}
impl Trig2Chain76Ie7En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Chain76Ie7En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Chain76Ie7En {
    #[inline(always)]
    fn from(val: u8) -> Trig2Chain76Ie7En {
        Trig2Chain76Ie7En::from_bits(val)
    }
}
impl From<Trig2Chain76Ie7En> for u8 {
    #[inline(always)]
    fn from(val: Trig2Chain76Ie7En) -> u8 {
        Trig2Chain76Ie7En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2CtrlSwTrig {
    #[doc = "No software trigger event generated."]
    SwTrig0 = 0x0,
    #[doc = "Software trigger event generated."]
    SwTrig1 = 0x01,
}
impl Trig2CtrlSwTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2CtrlSwTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2CtrlSwTrig {
    #[inline(always)]
    fn from(val: u8) -> Trig2CtrlSwTrig {
        Trig2CtrlSwTrig::from_bits(val)
    }
}
impl From<Trig2CtrlSwTrig> for u8 {
    #[inline(always)]
    fn from(val: Trig2CtrlSwTrig) -> u8 {
        Trig2CtrlSwTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SyncMode0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SyncMode1 = 0x01,
}
impl Trig2CtrlSyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2CtrlSyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2CtrlSyncMode {
    #[inline(always)]
    fn from(val: u8) -> Trig2CtrlSyncMode {
        Trig2CtrlSyncMode::from_bits(val)
    }
}
impl From<Trig2CtrlSyncMode> for u8 {
    #[inline(always)]
    fn from(val: Trig2CtrlSyncMode) -> u8 {
        Trig2CtrlSyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2CtrlTrigChain {
    #[doc = "Trigger chain length is 1."]
    TrigChain0 = 0x0,
    #[doc = "Trigger chain length is 2."]
    TrigChain1 = 0x01,
    #[doc = "Trigger chain length is 3."]
    TrigChain2 = 0x02,
    #[doc = "Trigger chain length is 4."]
    TrigChain3 = 0x03,
    #[doc = "Trigger chain length is 5."]
    TrigChain4 = 0x04,
    #[doc = "Trigger chain length is 6."]
    TrigChain5 = 0x05,
    #[doc = "Trigger chain length is 7."]
    TrigChain6 = 0x06,
    #[doc = "Trigger chain length is 8."]
    TrigChain7 = 0x07,
}
impl Trig2CtrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2CtrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2CtrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig2CtrlTrigChain {
        Trig2CtrlTrigChain::from_bits(val)
    }
}
impl From<Trig2CtrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig2CtrlTrigChain) -> u8 {
        Trig2CtrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2CtrlTrigMode {
    #[doc = "Hardware trigger. The softerware trigger will be ignored."]
    TrigMode0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TrigMode1 = 0x01,
}
impl Trig2CtrlTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2CtrlTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2CtrlTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Trig2CtrlTrigMode {
        Trig2CtrlTrigMode::from_bits(val)
    }
}
impl From<Trig2CtrlTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Trig2CtrlTrigMode) -> u8 {
        Trig2CtrlTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Done0 {
    #[doc = "No TRIG2_DONE0 interrupt detected."]
    Trig2Done00 = 0x0,
    #[doc = "TRIG2_DONE0 interrupt detected."]
    Trig2Done01 = 0x01,
}
impl Trig2Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Done0 {
        Trig2Done0::from_bits(val)
    }
}
impl From<Trig2Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig2Done0) -> u8 {
        Trig2Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Done1 {
    #[doc = "No TRIG2_DONE1 interrupt detected."]
    Trig2Done10 = 0x0,
    #[doc = "TRIG2_DONE1 interrupt detected."]
    Trig2Done11 = 0x01,
}
impl Trig2Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Done1 {
        Trig2Done1::from_bits(val)
    }
}
impl From<Trig2Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig2Done1) -> u8 {
        Trig2Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Done2 {
    #[doc = "No TRIG2_DONE2 interrupt detected."]
    Trig2Done20 = 0x0,
    #[doc = "TRIG2_DONE2 interrupt detected."]
    Trig2Done21 = 0x01,
}
impl Trig2Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Done2 {
        Trig2Done2::from_bits(val)
    }
}
impl From<Trig2Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig2Done2) -> u8 {
        Trig2Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Done3 {
    #[doc = "No TRIG2_DONE3 interrupt detected."]
    Trig2Done30 = 0x0,
    #[doc = "TRIG2_DONE3 interrupt detected."]
    Trig2Done31 = 0x01,
}
impl Trig2Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2Done3 {
        Trig2Done3::from_bits(val)
    }
}
impl From<Trig2Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig2Done3) -> u8 {
        Trig2Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Enable {
    #[doc = "TRIG2 DMA request disabled."]
    Trig2Enable0 = 0x0,
    #[doc = "TRIG2 DMA request enabled."]
    Trig2Enable1 = 0x01,
}
impl Trig2Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig2Enable {
        Trig2Enable::from_bits(val)
    }
}
impl From<Trig2Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig2Enable) -> u8 {
        Trig2Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Err {
    #[doc = "No TRIG2_ERR interrupt detected."]
    Trig2Err0 = 0x0,
    #[doc = "TRIG2_ERR interrupt detected."]
    Trig2Err1 = 0x01,
}
impl Trig2Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Err {
    #[inline(always)]
    fn from(val: u8) -> Trig2Err {
        Trig2Err::from_bits(val)
    }
}
impl From<Trig2Err> for u8 {
    #[inline(always)]
    fn from(val: Trig2Err) -> u8 {
        Trig2Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2Req {
    #[doc = "TRIG2_REQ not detected."]
    Trig2Req0 = 0x0,
    #[doc = "TRIG2_REQ detected."]
    Trig2Req1 = 0x01,
}
impl Trig2Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2Req {
    #[inline(always)]
    fn from(val: u8) -> Trig2Req {
        Trig2Req::from_bits(val)
    }
}
impl From<Trig2Req> for u8 {
    #[inline(always)]
    fn from(val: Trig2Req) -> u8 {
        Trig2Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10B2b0 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b00 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b01 = 0x01,
}
impl Trig3Chain10B2b0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10B2b0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10B2b0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10B2b0 {
        Trig3Chain10B2b0::from_bits(val)
    }
}
impl From<Trig3Chain10B2b0> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10B2b0) -> u8 {
        Trig3Chain10B2b0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10B2b1 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b10 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b11 = 0x01,
}
impl Trig3Chain10B2b1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10B2b1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10B2b1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10B2b1 {
        Trig3Chain10B2b1::from_bits(val)
    }
}
impl From<Trig3Chain10B2b1> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10B2b1) -> u8 {
        Trig3Chain10B2b1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Csel0 {
    #[doc = "ADC Channel 0 selected."]
    Csel00 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel01 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel02 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel03 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel04 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel05 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel06 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel07 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel08 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel09 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel010 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel011 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel012 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel013 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel014 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel015 = 0x0f,
}
impl Trig3Chain10Csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Csel0 {
        Trig3Chain10Csel0::from_bits(val)
    }
}
impl From<Trig3Chain10Csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Csel0) -> u8 {
        Trig3Chain10Csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Csel1 {
    #[doc = "ADC Channel 0 selected."]
    Csel10 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel11 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel12 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel13 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel14 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel15 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel16 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel17 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel18 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel19 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel110 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel111 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel112 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel113 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel114 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel115 = 0x0f,
}
impl Trig3Chain10Csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Csel1 {
        Trig3Chain10Csel1::from_bits(val)
    }
}
impl From<Trig3Chain10Csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Csel1) -> u8 {
        Trig3Chain10Csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain10Hwts0(u8);
impl Trig3Chain10Hwts0 {
    #[doc = "no trigger selected."]
    pub const Hwts00: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts01: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts02: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts04: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts08: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts016: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts032: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts064: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts0128: Self = Self(0x80);
}
impl Trig3Chain10Hwts0 {
    pub const fn from_bits(val: u8) -> Trig3Chain10Hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain10Hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts00"),
            0x01 => f.write_str("Hwts01"),
            0x02 => f.write_str("Hwts02"),
            0x04 => f.write_str("Hwts04"),
            0x08 => f.write_str("Hwts08"),
            0x10 => f.write_str("Hwts016"),
            0x20 => f.write_str("Hwts032"),
            0x40 => f.write_str("Hwts064"),
            0x80 => f.write_str("Hwts0128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts00"),
            0x01 => defmt::write!(f, "Hwts01"),
            0x02 => defmt::write!(f, "Hwts02"),
            0x04 => defmt::write!(f, "Hwts04"),
            0x08 => defmt::write!(f, "Hwts08"),
            0x10 => defmt::write!(f, "Hwts016"),
            0x20 => defmt::write!(f, "Hwts032"),
            0x40 => defmt::write!(f, "Hwts064"),
            0x80 => defmt::write!(f, "Hwts0128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain10Hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Hwts0 {
        Trig3Chain10Hwts0::from_bits(val)
    }
}
impl From<Trig3Chain10Hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Hwts0) -> u8 {
        Trig3Chain10Hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain10Hwts1(u8);
impl Trig3Chain10Hwts1 {
    #[doc = "no trigger selected."]
    pub const Hwts10: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts11: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts12: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts14: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts18: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts116: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts132: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts164: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts1128: Self = Self(0x80);
}
impl Trig3Chain10Hwts1 {
    pub const fn from_bits(val: u8) -> Trig3Chain10Hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain10Hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts10"),
            0x01 => f.write_str("Hwts11"),
            0x02 => f.write_str("Hwts12"),
            0x04 => f.write_str("Hwts14"),
            0x08 => f.write_str("Hwts18"),
            0x10 => f.write_str("Hwts116"),
            0x20 => f.write_str("Hwts132"),
            0x40 => f.write_str("Hwts164"),
            0x80 => f.write_str("Hwts1128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts10"),
            0x01 => defmt::write!(f, "Hwts11"),
            0x02 => defmt::write!(f, "Hwts12"),
            0x04 => defmt::write!(f, "Hwts14"),
            0x08 => defmt::write!(f, "Hwts18"),
            0x10 => defmt::write!(f, "Hwts116"),
            0x20 => defmt::write!(f, "Hwts132"),
            0x40 => defmt::write!(f, "Hwts164"),
            0x80 => defmt::write!(f, "Hwts1128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain10Hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Hwts1 {
        Trig3Chain10Hwts1::from_bits(val)
    }
}
impl From<Trig3Chain10Hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Hwts1) -> u8 {
        Trig3Chain10Hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Ie0 {
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    Ie00 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    Ie01 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    Ie02 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    Ie03 = 0x03,
}
impl Trig3Chain10Ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Ie0 {
        Trig3Chain10Ie0::from_bits(val)
    }
}
impl From<Trig3Chain10Ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Ie0) -> u8 {
        Trig3Chain10Ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Ie0En {
    #[doc = "Interrupt DONE disabled."]
    Ie0En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0."]
    Ie0En1 = 0x01,
}
impl Trig3Chain10Ie0En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Ie0En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Ie0En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Ie0En {
        Trig3Chain10Ie0En::from_bits(val)
    }
}
impl From<Trig3Chain10Ie0En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Ie0En) -> u8 {
        Trig3Chain10Ie0En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    Ie10 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    Ie11 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    Ie12 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    Ie13 = 0x03,
}
impl Trig3Chain10Ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Ie1 {
        Trig3Chain10Ie1::from_bits(val)
    }
}
impl From<Trig3Chain10Ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Ie1) -> u8 {
        Trig3Chain10Ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain10Ie1En {
    #[doc = "Interrupt DONE disabled."]
    Ie1En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1."]
    Ie1En1 = 0x01,
}
impl Trig3Chain10Ie1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain10Ie1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain10Ie1En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain10Ie1En {
        Trig3Chain10Ie1En::from_bits(val)
    }
}
impl From<Trig3Chain10Ie1En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain10Ie1En) -> u8 {
        Trig3Chain10Ie1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32B2b2 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b20 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b21 = 0x01,
}
impl Trig3Chain32B2b2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32B2b2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32B2b2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32B2b2 {
        Trig3Chain32B2b2::from_bits(val)
    }
}
impl From<Trig3Chain32B2b2> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32B2b2) -> u8 {
        Trig3Chain32B2b2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32B2b3 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b30 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b31 = 0x01,
}
impl Trig3Chain32B2b3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32B2b3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32B2b3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32B2b3 {
        Trig3Chain32B2b3::from_bits(val)
    }
}
impl From<Trig3Chain32B2b3> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32B2b3) -> u8 {
        Trig3Chain32B2b3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Csel2 {
    #[doc = "ADC Channel 0 selected."]
    Csel20 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel21 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel22 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel23 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel24 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel25 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel26 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel27 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel28 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel29 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel210 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel211 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel212 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel213 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel214 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel215 = 0x0f,
}
impl Trig3Chain32Csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Csel2 {
        Trig3Chain32Csel2::from_bits(val)
    }
}
impl From<Trig3Chain32Csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Csel2) -> u8 {
        Trig3Chain32Csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Csel3 {
    #[doc = "ADC Channel 0 selected."]
    Csel30 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel31 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel32 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel33 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel34 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel35 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel36 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel37 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel38 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel39 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel310 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel311 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel312 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel313 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel314 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel315 = 0x0f,
}
impl Trig3Chain32Csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Csel3 {
        Trig3Chain32Csel3::from_bits(val)
    }
}
impl From<Trig3Chain32Csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Csel3) -> u8 {
        Trig3Chain32Csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain32Hwts2(u8);
impl Trig3Chain32Hwts2 {
    #[doc = "no trigger selected."]
    pub const Hwts20: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts21: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts22: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts24: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts28: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts216: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts232: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts264: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts2128: Self = Self(0x80);
}
impl Trig3Chain32Hwts2 {
    pub const fn from_bits(val: u8) -> Trig3Chain32Hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain32Hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts20"),
            0x01 => f.write_str("Hwts21"),
            0x02 => f.write_str("Hwts22"),
            0x04 => f.write_str("Hwts24"),
            0x08 => f.write_str("Hwts28"),
            0x10 => f.write_str("Hwts216"),
            0x20 => f.write_str("Hwts232"),
            0x40 => f.write_str("Hwts264"),
            0x80 => f.write_str("Hwts2128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts20"),
            0x01 => defmt::write!(f, "Hwts21"),
            0x02 => defmt::write!(f, "Hwts22"),
            0x04 => defmt::write!(f, "Hwts24"),
            0x08 => defmt::write!(f, "Hwts28"),
            0x10 => defmt::write!(f, "Hwts216"),
            0x20 => defmt::write!(f, "Hwts232"),
            0x40 => defmt::write!(f, "Hwts264"),
            0x80 => defmt::write!(f, "Hwts2128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain32Hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Hwts2 {
        Trig3Chain32Hwts2::from_bits(val)
    }
}
impl From<Trig3Chain32Hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Hwts2) -> u8 {
        Trig3Chain32Hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain32Hwts3(u8);
impl Trig3Chain32Hwts3 {
    #[doc = "no trigger selected."]
    pub const Hwts30: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts31: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts32: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts34: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts38: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts316: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts332: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts364: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts3128: Self = Self(0x80);
}
impl Trig3Chain32Hwts3 {
    pub const fn from_bits(val: u8) -> Trig3Chain32Hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain32Hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts30"),
            0x01 => f.write_str("Hwts31"),
            0x02 => f.write_str("Hwts32"),
            0x04 => f.write_str("Hwts34"),
            0x08 => f.write_str("Hwts38"),
            0x10 => f.write_str("Hwts316"),
            0x20 => f.write_str("Hwts332"),
            0x40 => f.write_str("Hwts364"),
            0x80 => f.write_str("Hwts3128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts30"),
            0x01 => defmt::write!(f, "Hwts31"),
            0x02 => defmt::write!(f, "Hwts32"),
            0x04 => defmt::write!(f, "Hwts34"),
            0x08 => defmt::write!(f, "Hwts38"),
            0x10 => defmt::write!(f, "Hwts316"),
            0x20 => defmt::write!(f, "Hwts332"),
            0x40 => defmt::write!(f, "Hwts364"),
            0x80 => defmt::write!(f, "Hwts3128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain32Hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Hwts3 {
        Trig3Chain32Hwts3::from_bits(val)
    }
}
impl From<Trig3Chain32Hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Hwts3) -> u8 {
        Trig3Chain32Hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Ie2 {
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    Ie20 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    Ie21 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    Ie22 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    Ie23 = 0x03,
}
impl Trig3Chain32Ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Ie2 {
        Trig3Chain32Ie2::from_bits(val)
    }
}
impl From<Trig3Chain32Ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Ie2) -> u8 {
        Trig3Chain32Ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Ie2En {
    #[doc = "Interrupt DONE disabled."]
    Ie2En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2."]
    Ie2En1 = 0x01,
}
impl Trig3Chain32Ie2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Ie2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Ie2En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Ie2En {
        Trig3Chain32Ie2En::from_bits(val)
    }
}
impl From<Trig3Chain32Ie2En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Ie2En) -> u8 {
        Trig3Chain32Ie2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    Ie30 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    Ie31 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    Ie32 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    Ie33 = 0x03,
}
impl Trig3Chain32Ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Ie3 {
        Trig3Chain32Ie3::from_bits(val)
    }
}
impl From<Trig3Chain32Ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Ie3) -> u8 {
        Trig3Chain32Ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain32Ie3En {
    #[doc = "Interrupt DONE disabled."]
    Ie3En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3."]
    Ie3En1 = 0x01,
}
impl Trig3Chain32Ie3En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain32Ie3En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain32Ie3En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain32Ie3En {
        Trig3Chain32Ie3En::from_bits(val)
    }
}
impl From<Trig3Chain32Ie3En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain32Ie3En) -> u8 {
        Trig3Chain32Ie3En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54B2b4 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b40 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b41 = 0x01,
}
impl Trig3Chain54B2b4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54B2b4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54B2b4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54B2b4 {
        Trig3Chain54B2b4::from_bits(val)
    }
}
impl From<Trig3Chain54B2b4> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54B2b4) -> u8 {
        Trig3Chain54B2b4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54B2b5 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b50 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b51 = 0x01,
}
impl Trig3Chain54B2b5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54B2b5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54B2b5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54B2b5 {
        Trig3Chain54B2b5::from_bits(val)
    }
}
impl From<Trig3Chain54B2b5> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54B2b5) -> u8 {
        Trig3Chain54B2b5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Csel4 {
    #[doc = "ADC Channel 0 selected."]
    Csel40 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel41 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel42 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel43 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel44 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel45 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel46 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel47 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel48 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel49 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel410 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel411 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel412 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel413 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel414 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel415 = 0x0f,
}
impl Trig3Chain54Csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Csel4 {
        Trig3Chain54Csel4::from_bits(val)
    }
}
impl From<Trig3Chain54Csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Csel4) -> u8 {
        Trig3Chain54Csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Csel5 {
    #[doc = "ADC Channel 0 selected."]
    Csel50 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel51 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel52 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel53 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel54 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel55 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel56 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel57 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel58 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel59 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel510 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel511 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel512 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel513 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel514 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel515 = 0x0f,
}
impl Trig3Chain54Csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Csel5 {
        Trig3Chain54Csel5::from_bits(val)
    }
}
impl From<Trig3Chain54Csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Csel5) -> u8 {
        Trig3Chain54Csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain54Hwts4(u8);
impl Trig3Chain54Hwts4 {
    #[doc = "no trigger selected."]
    pub const Hwts40: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts41: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts42: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts44: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts48: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts416: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts432: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts464: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts4128: Self = Self(0x80);
}
impl Trig3Chain54Hwts4 {
    pub const fn from_bits(val: u8) -> Trig3Chain54Hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain54Hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts40"),
            0x01 => f.write_str("Hwts41"),
            0x02 => f.write_str("Hwts42"),
            0x04 => f.write_str("Hwts44"),
            0x08 => f.write_str("Hwts48"),
            0x10 => f.write_str("Hwts416"),
            0x20 => f.write_str("Hwts432"),
            0x40 => f.write_str("Hwts464"),
            0x80 => f.write_str("Hwts4128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts40"),
            0x01 => defmt::write!(f, "Hwts41"),
            0x02 => defmt::write!(f, "Hwts42"),
            0x04 => defmt::write!(f, "Hwts44"),
            0x08 => defmt::write!(f, "Hwts48"),
            0x10 => defmt::write!(f, "Hwts416"),
            0x20 => defmt::write!(f, "Hwts432"),
            0x40 => defmt::write!(f, "Hwts464"),
            0x80 => defmt::write!(f, "Hwts4128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain54Hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Hwts4 {
        Trig3Chain54Hwts4::from_bits(val)
    }
}
impl From<Trig3Chain54Hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Hwts4) -> u8 {
        Trig3Chain54Hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain54Hwts5(u8);
impl Trig3Chain54Hwts5 {
    #[doc = "no trigger selected."]
    pub const Hwts50: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts51: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts52: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts54: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts58: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts516: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts532: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts564: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts5128: Self = Self(0x80);
}
impl Trig3Chain54Hwts5 {
    pub const fn from_bits(val: u8) -> Trig3Chain54Hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain54Hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts50"),
            0x01 => f.write_str("Hwts51"),
            0x02 => f.write_str("Hwts52"),
            0x04 => f.write_str("Hwts54"),
            0x08 => f.write_str("Hwts58"),
            0x10 => f.write_str("Hwts516"),
            0x20 => f.write_str("Hwts532"),
            0x40 => f.write_str("Hwts564"),
            0x80 => f.write_str("Hwts5128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts50"),
            0x01 => defmt::write!(f, "Hwts51"),
            0x02 => defmt::write!(f, "Hwts52"),
            0x04 => defmt::write!(f, "Hwts54"),
            0x08 => defmt::write!(f, "Hwts58"),
            0x10 => defmt::write!(f, "Hwts516"),
            0x20 => defmt::write!(f, "Hwts532"),
            0x40 => defmt::write!(f, "Hwts564"),
            0x80 => defmt::write!(f, "Hwts5128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain54Hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Hwts5 {
        Trig3Chain54Hwts5::from_bits(val)
    }
}
impl From<Trig3Chain54Hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Hwts5) -> u8 {
        Trig3Chain54Hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Ie4 {
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    Ie40 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    Ie41 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    Ie42 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    Ie43 = 0x03,
}
impl Trig3Chain54Ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Ie4 {
        Trig3Chain54Ie4::from_bits(val)
    }
}
impl From<Trig3Chain54Ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Ie4) -> u8 {
        Trig3Chain54Ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Ie4En {
    #[doc = "Interrupt DONE disabled."]
    Ie4En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4."]
    Ie4En1 = 0x01,
}
impl Trig3Chain54Ie4En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Ie4En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Ie4En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Ie4En {
        Trig3Chain54Ie4En::from_bits(val)
    }
}
impl From<Trig3Chain54Ie4En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Ie4En) -> u8 {
        Trig3Chain54Ie4En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    Ie50 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    Ie51 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    Ie52 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    Ie53 = 0x03,
}
impl Trig3Chain54Ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Ie5 {
        Trig3Chain54Ie5::from_bits(val)
    }
}
impl From<Trig3Chain54Ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Ie5) -> u8 {
        Trig3Chain54Ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain54Ie5En {
    #[doc = "Interrupt DONE disabled."]
    Ie5En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5."]
    Ie5En1 = 0x01,
}
impl Trig3Chain54Ie5En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain54Ie5En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain54Ie5En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain54Ie5En {
        Trig3Chain54Ie5En::from_bits(val)
    }
}
impl From<Trig3Chain54Ie5En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain54Ie5En) -> u8 {
        Trig3Chain54Ie5En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76B2b6 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b60 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b61 = 0x01,
}
impl Trig3Chain76B2b6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76B2b6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76B2b6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76B2b6 {
        Trig3Chain76B2b6::from_bits(val)
    }
}
impl From<Trig3Chain76B2b6> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76B2b6) -> u8 {
        Trig3Chain76B2b6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76B2b7 {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\] is reached."]
    B2b70 = 0x0,
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2b71 = 0x01,
}
impl Trig3Chain76B2b7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76B2b7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76B2b7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76B2b7 {
        Trig3Chain76B2b7::from_bits(val)
    }
}
impl From<Trig3Chain76B2b7> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76B2b7) -> u8 {
        Trig3Chain76B2b7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Csel6 {
    #[doc = "ADC Channel 0 selected."]
    Csel60 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel61 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel62 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel63 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel64 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel65 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel66 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel67 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel68 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel69 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel610 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel611 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel612 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel613 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel614 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel615 = 0x0f,
}
impl Trig3Chain76Csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Csel6 {
        Trig3Chain76Csel6::from_bits(val)
    }
}
impl From<Trig3Chain76Csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Csel6) -> u8 {
        Trig3Chain76Csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Csel7 {
    #[doc = "ADC Channel 0 selected."]
    Csel70 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    Csel71 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    Csel72 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    Csel73 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    Csel74 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    Csel75 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    Csel76 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    Csel77 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    Csel78 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    Csel79 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    Csel710 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    Csel711 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    Csel712 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    Csel713 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    Csel714 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    Csel715 = 0x0f,
}
impl Trig3Chain76Csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Csel7 {
        Trig3Chain76Csel7::from_bits(val)
    }
}
impl From<Trig3Chain76Csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Csel7) -> u8 {
        Trig3Chain76Csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain76Hwts6(u8);
impl Trig3Chain76Hwts6 {
    #[doc = "no trigger selected."]
    pub const Hwts60: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts61: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts62: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts64: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts68: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts616: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts632: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts664: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts6128: Self = Self(0x80);
}
impl Trig3Chain76Hwts6 {
    pub const fn from_bits(val: u8) -> Trig3Chain76Hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain76Hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts60"),
            0x01 => f.write_str("Hwts61"),
            0x02 => f.write_str("Hwts62"),
            0x04 => f.write_str("Hwts64"),
            0x08 => f.write_str("Hwts68"),
            0x10 => f.write_str("Hwts616"),
            0x20 => f.write_str("Hwts632"),
            0x40 => f.write_str("Hwts664"),
            0x80 => f.write_str("Hwts6128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts60"),
            0x01 => defmt::write!(f, "Hwts61"),
            0x02 => defmt::write!(f, "Hwts62"),
            0x04 => defmt::write!(f, "Hwts64"),
            0x08 => defmt::write!(f, "Hwts68"),
            0x10 => defmt::write!(f, "Hwts616"),
            0x20 => defmt::write!(f, "Hwts632"),
            0x40 => defmt::write!(f, "Hwts664"),
            0x80 => defmt::write!(f, "Hwts6128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain76Hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Hwts6 {
        Trig3Chain76Hwts6::from_bits(val)
    }
}
impl From<Trig3Chain76Hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Hwts6) -> u8 {
        Trig3Chain76Hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3Chain76Hwts7(u8);
impl Trig3Chain76Hwts7 {
    #[doc = "no trigger selected."]
    pub const Hwts70: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected."]
    pub const Hwts71: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected."]
    pub const Hwts72: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected."]
    pub const Hwts74: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected."]
    pub const Hwts78: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected."]
    pub const Hwts716: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected."]
    pub const Hwts732: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected."]
    pub const Hwts764: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected."]
    pub const Hwts7128: Self = Self(0x80);
}
impl Trig3Chain76Hwts7 {
    pub const fn from_bits(val: u8) -> Trig3Chain76Hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3Chain76Hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hwts70"),
            0x01 => f.write_str("Hwts71"),
            0x02 => f.write_str("Hwts72"),
            0x04 => f.write_str("Hwts74"),
            0x08 => f.write_str("Hwts78"),
            0x10 => f.write_str("Hwts716"),
            0x20 => f.write_str("Hwts732"),
            0x40 => f.write_str("Hwts764"),
            0x80 => f.write_str("Hwts7128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hwts70"),
            0x01 => defmt::write!(f, "Hwts71"),
            0x02 => defmt::write!(f, "Hwts72"),
            0x04 => defmt::write!(f, "Hwts74"),
            0x08 => defmt::write!(f, "Hwts78"),
            0x10 => defmt::write!(f, "Hwts716"),
            0x20 => defmt::write!(f, "Hwts732"),
            0x40 => defmt::write!(f, "Hwts764"),
            0x80 => defmt::write!(f, "Hwts7128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3Chain76Hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Hwts7 {
        Trig3Chain76Hwts7::from_bits(val)
    }
}
impl From<Trig3Chain76Hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Hwts7) -> u8 {
        Trig3Chain76Hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Ie6 {
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    Ie60 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    Ie61 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    Ie62 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    Ie63 = 0x03,
}
impl Trig3Chain76Ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Ie6 {
        Trig3Chain76Ie6::from_bits(val)
    }
}
impl From<Trig3Chain76Ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Ie6) -> u8 {
        Trig3Chain76Ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Ie6En {
    #[doc = "Interrupt DONE disabled."]
    Ie6En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6."]
    Ie6En1 = 0x01,
}
impl Trig3Chain76Ie6En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Ie6En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Ie6En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Ie6En {
        Trig3Chain76Ie6En::from_bits(val)
    }
}
impl From<Trig3Chain76Ie6En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Ie6En) -> u8 {
        Trig3Chain76Ie6En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    Ie70 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    Ie71 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    Ie72 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    Ie73 = 0x03,
}
impl Trig3Chain76Ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Ie7 {
        Trig3Chain76Ie7::from_bits(val)
    }
}
impl From<Trig3Chain76Ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Ie7) -> u8 {
        Trig3Chain76Ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Chain76Ie7En {
    #[doc = "Interrupt DONE disabled."]
    Ie7En0 = 0x0,
    #[doc = "Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7."]
    Ie7En1 = 0x01,
}
impl Trig3Chain76Ie7En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Chain76Ie7En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Chain76Ie7En {
    #[inline(always)]
    fn from(val: u8) -> Trig3Chain76Ie7En {
        Trig3Chain76Ie7En::from_bits(val)
    }
}
impl From<Trig3Chain76Ie7En> for u8 {
    #[inline(always)]
    fn from(val: Trig3Chain76Ie7En) -> u8 {
        Trig3Chain76Ie7En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3CtrlSwTrig {
    #[doc = "No software trigger event generated."]
    SwTrig0 = 0x0,
    #[doc = "Software trigger event generated."]
    SwTrig1 = 0x01,
}
impl Trig3CtrlSwTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3CtrlSwTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3CtrlSwTrig {
    #[inline(always)]
    fn from(val: u8) -> Trig3CtrlSwTrig {
        Trig3CtrlSwTrig::from_bits(val)
    }
}
impl From<Trig3CtrlSwTrig> for u8 {
    #[inline(always)]
    fn from(val: Trig3CtrlSwTrig) -> u8 {
        Trig3CtrlSwTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SyncMode0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SyncMode1 = 0x01,
}
impl Trig3CtrlSyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3CtrlSyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3CtrlSyncMode {
    #[inline(always)]
    fn from(val: u8) -> Trig3CtrlSyncMode {
        Trig3CtrlSyncMode::from_bits(val)
    }
}
impl From<Trig3CtrlSyncMode> for u8 {
    #[inline(always)]
    fn from(val: Trig3CtrlSyncMode) -> u8 {
        Trig3CtrlSyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3CtrlTrigChain {
    #[doc = "Trigger chain length is 1."]
    TrigChain0 = 0x0,
    #[doc = "Trigger chain length is 2."]
    TrigChain1 = 0x01,
    #[doc = "Trigger chain length is 3."]
    TrigChain2 = 0x02,
    #[doc = "Trigger chain length is 4."]
    TrigChain3 = 0x03,
    #[doc = "Trigger chain length is 5."]
    TrigChain4 = 0x04,
    #[doc = "Trigger chain length is 6."]
    TrigChain5 = 0x05,
    #[doc = "Trigger chain length is 7."]
    TrigChain6 = 0x06,
    #[doc = "Trigger chain length is 8."]
    TrigChain7 = 0x07,
}
impl Trig3CtrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3CtrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3CtrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig3CtrlTrigChain {
        Trig3CtrlTrigChain::from_bits(val)
    }
}
impl From<Trig3CtrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig3CtrlTrigChain) -> u8 {
        Trig3CtrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3CtrlTrigMode {
    #[doc = "Hardware trigger. The softerware trigger will be ignored."]
    TrigMode0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TrigMode1 = 0x01,
}
impl Trig3CtrlTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3CtrlTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3CtrlTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Trig3CtrlTrigMode {
        Trig3CtrlTrigMode::from_bits(val)
    }
}
impl From<Trig3CtrlTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Trig3CtrlTrigMode) -> u8 {
        Trig3CtrlTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Done0 {
    #[doc = "No TRIG3_DONE0 interrupt detected."]
    Trig3Done00 = 0x0,
    #[doc = "TRIG3_DONE0 interrupt detected."]
    Trig3Done01 = 0x01,
}
impl Trig3Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Done0 {
        Trig3Done0::from_bits(val)
    }
}
impl From<Trig3Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig3Done0) -> u8 {
        Trig3Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Done1 {
    #[doc = "No TRIG3_DONE1 interrupt detected."]
    Trig3Done10 = 0x0,
    #[doc = "TRIG3_DONE1 interrupt detected."]
    Trig3Done11 = 0x01,
}
impl Trig3Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Done1 {
        Trig3Done1::from_bits(val)
    }
}
impl From<Trig3Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig3Done1) -> u8 {
        Trig3Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Done2 {
    #[doc = "No TRIG3_DONE2 interrupt detected."]
    Trig3Done20 = 0x0,
    #[doc = "TRIG3_DONE2 interrupt detected."]
    Trig3Done21 = 0x01,
}
impl Trig3Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Done2 {
        Trig3Done2::from_bits(val)
    }
}
impl From<Trig3Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig3Done2) -> u8 {
        Trig3Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Done3 {
    #[doc = "No TRIG3_DONE3 interrupt detected."]
    Trig3Done30 = 0x0,
    #[doc = "TRIG3_DONE3 interrupt detected."]
    Trig3Done31 = 0x01,
}
impl Trig3Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3Done3 {
        Trig3Done3::from_bits(val)
    }
}
impl From<Trig3Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig3Done3) -> u8 {
        Trig3Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Enable {
    #[doc = "TRIG3 DMA request disabled."]
    Trig3Enable0 = 0x0,
    #[doc = "TRIG3 DMA request enabled."]
    Trig3Enable1 = 0x01,
}
impl Trig3Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig3Enable {
        Trig3Enable::from_bits(val)
    }
}
impl From<Trig3Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig3Enable) -> u8 {
        Trig3Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Err {
    #[doc = "No TRIG3_ERR interrupt detected."]
    Trig3Err0 = 0x0,
    #[doc = "TRIG3_ERR interrupt detected."]
    Trig3Err1 = 0x01,
}
impl Trig3Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Err {
    #[inline(always)]
    fn from(val: u8) -> Trig3Err {
        Trig3Err::from_bits(val)
    }
}
impl From<Trig3Err> for u8 {
    #[inline(always)]
    fn from(val: Trig3Err) -> u8 {
        Trig3Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3Req {
    #[doc = "TRIG3_REQ not detected."]
    Trig3Req0 = 0x0,
    #[doc = "TRIG3_REQ detected."]
    Trig3Req1 = 0x01,
}
impl Trig3Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3Req {
    #[inline(always)]
    fn from(val: u8) -> Trig3Req {
        Trig3Req::from_bits(val)
    }
}
impl From<Trig3Req> for u8 {
    #[inline(always)]
    fn from(val: Trig3Req) -> u8 {
        Trig3Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Done0 {
    #[doc = "No TRIG4_DONE0 interrupt detected."]
    Trig4Done00 = 0x0,
    #[doc = "TRIG4_DONE0 interrupt detected."]
    Trig4Done01 = 0x01,
}
impl Trig4Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig4Done0 {
        Trig4Done0::from_bits(val)
    }
}
impl From<Trig4Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig4Done0) -> u8 {
        Trig4Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Done1 {
    #[doc = "No TRIG4_DONE1 interrupt detected."]
    Trig4Done10 = 0x0,
    #[doc = "TRIG4_DONE1 interrupt detected."]
    Trig4Done11 = 0x01,
}
impl Trig4Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig4Done1 {
        Trig4Done1::from_bits(val)
    }
}
impl From<Trig4Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig4Done1) -> u8 {
        Trig4Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Done2 {
    #[doc = "No TRIG4_DONE2 interrupt detected."]
    Trig4Done20 = 0x0,
    #[doc = "TRIG4_DONE2 interrupt detected."]
    Trig4Done21 = 0x01,
}
impl Trig4Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig4Done2 {
        Trig4Done2::from_bits(val)
    }
}
impl From<Trig4Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig4Done2) -> u8 {
        Trig4Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Done3 {
    #[doc = "No TRIG4_DONE3 interrupt detected."]
    Trig4Done30 = 0x0,
    #[doc = "TRIG4_DONE3 interrupt detected."]
    Trig4Done31 = 0x01,
}
impl Trig4Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig4Done3 {
        Trig4Done3::from_bits(val)
    }
}
impl From<Trig4Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig4Done3) -> u8 {
        Trig4Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Enable {
    #[doc = "TRIG4 DMA request disabled."]
    Trig4Enable0 = 0x0,
    #[doc = "TRIG4 DMA request enabled."]
    Trig4Enable1 = 0x01,
}
impl Trig4Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig4Enable {
        Trig4Enable::from_bits(val)
    }
}
impl From<Trig4Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig4Enable) -> u8 {
        Trig4Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Err {
    #[doc = "No TRIG4_ERR interrupt detected."]
    Trig4Err0 = 0x0,
    #[doc = "TRIG4_ERR interrupt detected."]
    Trig4Err1 = 0x01,
}
impl Trig4Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Err {
    #[inline(always)]
    fn from(val: u8) -> Trig4Err {
        Trig4Err::from_bits(val)
    }
}
impl From<Trig4Err> for u8 {
    #[inline(always)]
    fn from(val: Trig4Err) -> u8 {
        Trig4Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4Req {
    #[doc = "TRIG4_REQ not detected."]
    Trig4Req0 = 0x0,
    #[doc = "TRIG4_REQ detected."]
    Trig4Req1 = 0x01,
}
impl Trig4Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4Req {
    #[inline(always)]
    fn from(val: u8) -> Trig4Req {
        Trig4Req::from_bits(val)
    }
}
impl From<Trig4Req> for u8 {
    #[inline(always)]
    fn from(val: Trig4Req) -> u8 {
        Trig4Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Done0 {
    #[doc = "No TRIG5_DONE0 interrupt detected."]
    Trig5Done00 = 0x0,
    #[doc = "TRIG5_DONE0 interrupt detected."]
    Trig5Done01 = 0x01,
}
impl Trig5Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig5Done0 {
        Trig5Done0::from_bits(val)
    }
}
impl From<Trig5Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig5Done0) -> u8 {
        Trig5Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Done1 {
    #[doc = "No TRIG5_DONE1 interrupt detected."]
    Trig5Done10 = 0x0,
    #[doc = "TRIG5_DONE1 interrupt detected."]
    Trig5Done11 = 0x01,
}
impl Trig5Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig5Done1 {
        Trig5Done1::from_bits(val)
    }
}
impl From<Trig5Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig5Done1) -> u8 {
        Trig5Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Done2 {
    #[doc = "No TRIG5_DONE2 interrupt detected."]
    Trig5Done20 = 0x0,
    #[doc = "TRIG5_DONE2 interrupt detected."]
    Trig5Done21 = 0x01,
}
impl Trig5Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig5Done2 {
        Trig5Done2::from_bits(val)
    }
}
impl From<Trig5Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig5Done2) -> u8 {
        Trig5Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Done3 {
    #[doc = "No TRIG5_DONE3 interrupt detected."]
    Trig5Done30 = 0x0,
    #[doc = "TRIG5_DONE3 interrupt detected."]
    Trig5Done31 = 0x01,
}
impl Trig5Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig5Done3 {
        Trig5Done3::from_bits(val)
    }
}
impl From<Trig5Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig5Done3) -> u8 {
        Trig5Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Enable {
    #[doc = "TRIG5 DMA request disabled."]
    Trig5Enable0 = 0x0,
    #[doc = "TRIG5 DMA request enabled."]
    Trig5Enable1 = 0x01,
}
impl Trig5Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig5Enable {
        Trig5Enable::from_bits(val)
    }
}
impl From<Trig5Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig5Enable) -> u8 {
        Trig5Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Err {
    #[doc = "No TRIG5_ERR interrupt detected."]
    Trig5Err0 = 0x0,
    #[doc = "TRIG5_ERR interrupt detected."]
    Trig5Err1 = 0x01,
}
impl Trig5Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Err {
    #[inline(always)]
    fn from(val: u8) -> Trig5Err {
        Trig5Err::from_bits(val)
    }
}
impl From<Trig5Err> for u8 {
    #[inline(always)]
    fn from(val: Trig5Err) -> u8 {
        Trig5Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5Req {
    #[doc = "TRIG5_REQ not detected."]
    Trig5Req0 = 0x0,
    #[doc = "TRIG5_REQ detected."]
    Trig5Req1 = 0x01,
}
impl Trig5Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5Req {
    #[inline(always)]
    fn from(val: u8) -> Trig5Req {
        Trig5Req::from_bits(val)
    }
}
impl From<Trig5Req> for u8 {
    #[inline(always)]
    fn from(val: Trig5Req) -> u8 {
        Trig5Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Done0 {
    #[doc = "No TRIG6_DONE0 interrupt detected."]
    Trig6Done00 = 0x0,
    #[doc = "TRIG6_DONE0 interrupt detected."]
    Trig6Done01 = 0x01,
}
impl Trig6Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig6Done0 {
        Trig6Done0::from_bits(val)
    }
}
impl From<Trig6Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig6Done0) -> u8 {
        Trig6Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Done1 {
    #[doc = "No TRIG6_DONE1 interrupt detected."]
    Trig6Done10 = 0x0,
    #[doc = "TRIG6_DONE1 interrupt detected."]
    Trig6Done11 = 0x01,
}
impl Trig6Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig6Done1 {
        Trig6Done1::from_bits(val)
    }
}
impl From<Trig6Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig6Done1) -> u8 {
        Trig6Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Done2 {
    #[doc = "No TRIG6_DONE2 interrupt detected."]
    Trig6Done20 = 0x0,
    #[doc = "TRIG6_DONE2 interrupt detected."]
    Trig6Done21 = 0x01,
}
impl Trig6Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig6Done2 {
        Trig6Done2::from_bits(val)
    }
}
impl From<Trig6Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig6Done2) -> u8 {
        Trig6Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Done3 {
    #[doc = "No TRIG6_DONE3 interrupt detected."]
    Trig6Done30 = 0x0,
    #[doc = "TRIG6_DONE3 interrupt detected."]
    Trig6Done31 = 0x01,
}
impl Trig6Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig6Done3 {
        Trig6Done3::from_bits(val)
    }
}
impl From<Trig6Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig6Done3) -> u8 {
        Trig6Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Enable {
    #[doc = "TRIG6 DMA request disabled."]
    Trig6Enable0 = 0x0,
    #[doc = "TRIG6 DMA request enabled."]
    Trig6Enable1 = 0x01,
}
impl Trig6Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig6Enable {
        Trig6Enable::from_bits(val)
    }
}
impl From<Trig6Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig6Enable) -> u8 {
        Trig6Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Err {
    #[doc = "No TRIG6_ERR interrupt detected."]
    Trig6Err0 = 0x0,
    #[doc = "TRIG6_ERR interrupt detected."]
    Trig6Err1 = 0x01,
}
impl Trig6Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Err {
    #[inline(always)]
    fn from(val: u8) -> Trig6Err {
        Trig6Err::from_bits(val)
    }
}
impl From<Trig6Err> for u8 {
    #[inline(always)]
    fn from(val: Trig6Err) -> u8 {
        Trig6Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6Req {
    #[doc = "TRIG6_REQ not detected."]
    Trig6Req0 = 0x0,
    #[doc = "TRIG6_REQ detected."]
    Trig6Req1 = 0x01,
}
impl Trig6Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6Req {
    #[inline(always)]
    fn from(val: u8) -> Trig6Req {
        Trig6Req::from_bits(val)
    }
}
impl From<Trig6Req> for u8 {
    #[inline(always)]
    fn from(val: Trig6Req) -> u8 {
        Trig6Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Done0 {
    #[doc = "No TRIG7_DONE0 interrupt detected."]
    Trig7Done00 = 0x0,
    #[doc = "TRIG7_DONE0 interrupt detected."]
    Trig7Done01 = 0x01,
}
impl Trig7Done0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Done0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Done0 {
    #[inline(always)]
    fn from(val: u8) -> Trig7Done0 {
        Trig7Done0::from_bits(val)
    }
}
impl From<Trig7Done0> for u8 {
    #[inline(always)]
    fn from(val: Trig7Done0) -> u8 {
        Trig7Done0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Done1 {
    #[doc = "No TRIG7_DONE1 interrupt detected."]
    Trig7Done10 = 0x0,
    #[doc = "TRIG7_DONE1 interrupt detected."]
    Trig7Done11 = 0x01,
}
impl Trig7Done1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Done1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Done1 {
    #[inline(always)]
    fn from(val: u8) -> Trig7Done1 {
        Trig7Done1::from_bits(val)
    }
}
impl From<Trig7Done1> for u8 {
    #[inline(always)]
    fn from(val: Trig7Done1) -> u8 {
        Trig7Done1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Done2 {
    #[doc = "No TRIG7_DONE2 interrupt detected."]
    Trig7Done20 = 0x0,
    #[doc = "TRIG7_DONE2 interrupt detected."]
    Trig7Done21 = 0x01,
}
impl Trig7Done2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Done2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Done2 {
    #[inline(always)]
    fn from(val: u8) -> Trig7Done2 {
        Trig7Done2::from_bits(val)
    }
}
impl From<Trig7Done2> for u8 {
    #[inline(always)]
    fn from(val: Trig7Done2) -> u8 {
        Trig7Done2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Done3 {
    #[doc = "No TRIG7_DONE3 interrupt detected."]
    Trig7Done30 = 0x0,
    #[doc = "TRIG7_DONE3 interrupt detected."]
    Trig7Done31 = 0x01,
}
impl Trig7Done3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Done3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Done3 {
    #[inline(always)]
    fn from(val: u8) -> Trig7Done3 {
        Trig7Done3::from_bits(val)
    }
}
impl From<Trig7Done3> for u8 {
    #[inline(always)]
    fn from(val: Trig7Done3) -> u8 {
        Trig7Done3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Enable {
    #[doc = "TRIG7 DMA request disabled."]
    Trig7Enable0 = 0x0,
    #[doc = "TRIG7 DMA request enabled."]
    Trig7Enable1 = 0x01,
}
impl Trig7Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Enable {
    #[inline(always)]
    fn from(val: u8) -> Trig7Enable {
        Trig7Enable::from_bits(val)
    }
}
impl From<Trig7Enable> for u8 {
    #[inline(always)]
    fn from(val: Trig7Enable) -> u8 {
        Trig7Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Err {
    #[doc = "No TRIG7_ERR interrupt detected."]
    Trig7Err0 = 0x0,
    #[doc = "TRIG7_ERR interrupt detected."]
    Trig7Err1 = 0x01,
}
impl Trig7Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Err {
    #[inline(always)]
    fn from(val: u8) -> Trig7Err {
        Trig7Err::from_bits(val)
    }
}
impl From<Trig7Err> for u8 {
    #[inline(always)]
    fn from(val: Trig7Err) -> u8 {
        Trig7Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7Req {
    #[doc = "TRIG7_REQ not detected."]
    Trig7Req0 = 0x0,
    #[doc = "TRIG7_REQ detected."]
    Trig7Req1 = 0x01,
}
impl Trig7Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7Req {
    #[inline(always)]
    fn from(val: u8) -> Trig7Req {
        Trig7Req::from_bits(val)
    }
}
impl From<Trig7Req> for u8 {
    #[inline(always)]
    fn from(val: Trig7Req) -> u8 {
        Trig7Req::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TrigEnable(u8);
impl TrigEnable {
    #[doc = "disable all 8 external XBAR triggers."]
    pub const TrigEnable0: Self = Self(0x0);
    #[doc = "enable external XBAR trigger0."]
    pub const TrigEnable1: Self = Self(0x01);
    #[doc = "enable external XBAR trigger1."]
    pub const TrigEnable2: Self = Self(0x02);
    #[doc = "enable external XBAR trigger0 and trigger1."]
    pub const TrigEnable3: Self = Self(0x03);
    #[doc = "enable all 8 external XBAR triggers."]
    pub const TrigEnable255: Self = Self(0xff);
}
impl TrigEnable {
    pub const fn from_bits(val: u8) -> TrigEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for TrigEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TrigEnable0"),
            0x01 => f.write_str("TrigEnable1"),
            0x02 => f.write_str("TrigEnable2"),
            0x03 => f.write_str("TrigEnable3"),
            0xff => f.write_str("TrigEnable255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TrigEnable0"),
            0x01 => defmt::write!(f, "TrigEnable1"),
            0x02 => defmt::write!(f, "TrigEnable2"),
            0x03 => defmt::write!(f, "TrigEnable3"),
            0xff => defmt::write!(f, "TrigEnable255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for TrigEnable {
    #[inline(always)]
    fn from(val: u8) -> TrigEnable {
        TrigEnable::from_bits(val)
    }
}
impl From<TrigEnable> for u8 {
    #[inline(always)]
    fn from(val: TrigEnable) -> u8 {
        TrigEnable::to_bits(val)
    }
}
