#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaModeSel {
    #[doc = "Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared."]
    DMA_MODE_SEL_0 = 0x0,
    #[doc = "Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only."]
    DMA_MODE_SEL_1 = 0x01,
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
pub enum Trig0Chain10Csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
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
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    IE0_3 = 0x03,
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
pub enum Trig0Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    IE1_3 = 0x03,
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
pub enum Trig0Chain32Csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
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
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    IE2_3 = 0x03,
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
pub enum Trig0Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    IE3_3 = 0x03,
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
pub enum Trig0Chain54Csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
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
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    IE4_3 = 0x03,
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
pub enum Trig0Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    IE5_3 = 0x03,
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
pub enum Trig0Chain76Csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
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
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
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
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    IE6_3 = 0x03,
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
pub enum Trig0Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    IE7_3 = 0x03,
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
pub enum Trig0CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SYNC_MODE_0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SYNC_MODE_1 = 0x01,
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
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
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
    TRIG_MODE_0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TRIG_MODE_1 = 0x01,
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
pub enum Trig1Chain10Csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
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
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    IE0_3 = 0x03,
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
pub enum Trig1Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    IE1_3 = 0x03,
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
pub enum Trig1Chain32Csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
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
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    IE2_3 = 0x03,
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
pub enum Trig1Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    IE3_3 = 0x03,
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
pub enum Trig1Chain54Csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
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
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    IE4_3 = 0x03,
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
pub enum Trig1Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    IE5_3 = 0x03,
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
pub enum Trig1Chain76Csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
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
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
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
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    IE6_3 = 0x03,
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
pub enum Trig1Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    IE7_3 = 0x03,
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
pub enum Trig1CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SYNC_MODE_0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SYNC_MODE_1 = 0x01,
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
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
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
    TRIG_MODE_0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TRIG_MODE_1 = 0x01,
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
pub enum Trig2Chain10Csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
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
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    IE0_3 = 0x03,
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
pub enum Trig2Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    IE1_3 = 0x03,
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
pub enum Trig2Chain32Csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
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
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    IE2_3 = 0x03,
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
pub enum Trig2Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    IE3_3 = 0x03,
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
pub enum Trig2Chain54Csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
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
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    IE4_3 = 0x03,
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
pub enum Trig2Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    IE5_3 = 0x03,
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
pub enum Trig2Chain76Csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
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
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
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
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    IE6_3 = 0x03,
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
pub enum Trig2Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    IE7_3 = 0x03,
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
pub enum Trig2CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SYNC_MODE_0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SYNC_MODE_1 = 0x01,
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
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
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
    TRIG_MODE_0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TRIG_MODE_1 = 0x01,
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
pub enum Trig3Chain10Csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain10Hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain10Hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
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
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 0 finish."]
    IE0_3 = 0x03,
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
pub enum Trig3Chain10Ie1 {
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when Segment 1 finish."]
    IE1_3 = 0x03,
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
pub enum Trig3Chain32Csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain32Hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain32Hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
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
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 2 finish."]
    IE2_3 = 0x03,
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
pub enum Trig3Chain32Ie3 {
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 3 finish."]
    IE3_3 = 0x03,
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
pub enum Trig3Chain54Csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
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
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain54Hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain54Hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
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
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 4 finish."]
    IE4_3 = 0x03,
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
pub enum Trig3Chain54Ie5 {
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 5 finish."]
    IE5_3 = 0x03,
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
pub enum Trig3Chain76Csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
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
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
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
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain76Hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
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
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
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
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain76Hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
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
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 6 finish."]
    IE6_3 = 0x03,
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
pub enum Trig3Chain76Ie7 {
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done3 when segment 7 finish."]
    IE7_3 = 0x03,
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
pub enum Trig3CtrlSyncMode {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SYNC_MODE_0 = 0x0,
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SYNC_MODE_1 = 0x01,
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
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
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
    TRIG_MODE_0 = 0x0,
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    TRIG_MODE_1 = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TrigEnable(u8);
impl TrigEnable {
    #[doc = "disable all 8 external XBAR triggers."]
    pub const TRIG_ENABLE_0: Self = Self(0x0);
    #[doc = "enable external XBAR trigger0."]
    pub const TRIG_ENABLE_1: Self = Self(0x01);
    #[doc = "enable external XBAR trigger1."]
    pub const TRIG_ENABLE_2: Self = Self(0x02);
    #[doc = "enable external XBAR trigger0 and trigger1."]
    pub const TRIG_ENABLE_3: Self = Self(0x03);
    #[doc = "enable all 8 external XBAR triggers."]
    pub const TRIG_ENABLE_255: Self = Self(0xff);
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
            0x0 => f.write_str("TRIG_ENABLE_0"),
            0x01 => f.write_str("TRIG_ENABLE_1"),
            0x02 => f.write_str("TRIG_ENABLE_2"),
            0x03 => f.write_str("TRIG_ENABLE_3"),
            0xff => f.write_str("TRIG_ENABLE_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRIG_ENABLE_0"),
            0x01 => defmt::write!(f, "TRIG_ENABLE_1"),
            0x02 => defmt::write!(f, "TRIG_ENABLE_2"),
            0x03 => defmt::write!(f, "TRIG_ENABLE_3"),
            0xff => defmt::write!(f, "TRIG_ENABLE_255"),
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
