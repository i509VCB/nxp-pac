#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksrc {
    #[doc = "No clock"]
    CLKSRC_0 = 0x0,
    #[doc = "Peripheral Clock (ipg_clk)"]
    CLKSRC_1 = 0x01,
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLKSRC_2 = 0x02,
    #[doc = "External Clock"]
    CLKSRC_3 = 0x03,
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    CLKSRC_4 = 0x04,
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    CLKSRC_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Clksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksrc {
    #[inline(always)]
    fn from(val: u8) -> Clksrc {
        Clksrc::from_bits(val)
    }
}
impl From<Clksrc> for u8 {
    #[inline(always)]
    fn from(val: Clksrc) -> u8 {
        Clksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Im2 {
    #[doc = "capture disabled"]
    IM2_0 = 0x0,
    #[doc = "capture on rising edge only"]
    IM2_1 = 0x01,
    #[doc = "capture on falling edge only"]
    IM2_2 = 0x02,
    #[doc = "capture on both edges"]
    IM2_3 = 0x03,
}
impl Im2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Im2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Im2 {
    #[inline(always)]
    fn from(val: u8) -> Im2 {
        Im2::from_bits(val)
    }
}
impl From<Im2> for u8 {
    #[inline(always)]
    fn from(val: Im2) -> u8 {
        Im2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Om3 {
    #[doc = "Output disconnected. No response on pin."]
    OM3_0 = 0x0,
    #[doc = "Toggle output pin"]
    OM3_1 = 0x01,
    #[doc = "Clear output pin"]
    OM3_2 = 0x02,
    #[doc = "Set output pin"]
    OM3_3 = 0x03,
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    OM3_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Om3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Om3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Om3 {
    #[inline(always)]
    fn from(val: u8) -> Om3 {
        Om3::from_bits(val)
    }
}
impl From<Om3> for u8 {
    #[inline(always)]
    fn from(val: Om3) -> u8 {
        Om3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prescaler(u16);
impl Prescaler {
    #[doc = "Divide by 1"]
    pub const PRESCALER_0: Self = Self(0x0);
    #[doc = "Divide by 2"]
    pub const PRESCALER_1: Self = Self(0x01);
    #[doc = "Divide by 4096"]
    pub const PRESCALER_4095: Self = Self(0x0fff);
}
impl Prescaler {
    pub const fn from_bits(val: u16) -> Prescaler {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Prescaler {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("PRESCALER_0"),
            0x01 => f.write_str("PRESCALER_1"),
            0x0fff => f.write_str("PRESCALER_4095"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prescaler {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "PRESCALER_0"),
            0x01 => defmt::write!(f, "PRESCALER_1"),
            0x0fff => defmt::write!(f, "PRESCALER_4095"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Prescaler {
    #[inline(always)]
    fn from(val: u16) -> Prescaler {
        Prescaler::from_bits(val)
    }
}
impl From<Prescaler> for u16 {
    #[inline(always)]
    fn from(val: Prescaler) -> u16 {
        Prescaler::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescaler24m {
    #[doc = "Divide by 1"]
    PRESCALER24M_0 = 0x0,
    #[doc = "Divide by 2"]
    PRESCALER24M_1 = 0x01,
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
    #[doc = "Divide by 16"]
    PRESCALER24M_15 = 0x0f,
}
impl Prescaler24m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescaler24m {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescaler24m {
    #[inline(always)]
    fn from(val: u8) -> Prescaler24m {
        Prescaler24m::from_bits(val)
    }
}
impl From<Prescaler24m> for u8 {
    #[inline(always)]
    fn from(val: Prescaler24m) -> u8 {
        Prescaler24m::to_bits(val)
    }
}
