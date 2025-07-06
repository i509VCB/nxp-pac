#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksrc {
    #[doc = "No clock"]
    NO_CLOCK = 0x0,
    #[doc = "Peripheral Clock (ipg_clk)"]
    CLOCK_001 = 0x01,
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLOCK_010 = 0x02,
    #[doc = "External Clock"]
    CLOCK_011 = 0x03,
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    CLOCK_100 = 0x04,
    #[doc = "Oscillator as Reference Clock (ipg_clk_24M)"]
    CLOCK_101 = 0x05,
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
pub enum Dbgen {
    #[doc = "Disable in Debug mode"]
    DEBUG_DIS = 0x0,
    #[doc = "Enable in Debug mode"]
    DEBUG_EN = 0x01,
}
impl Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgen {
    #[inline(always)]
    fn from(val: u8) -> Dbgen {
        Dbgen::from_bits(val)
    }
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(val: Dbgen) -> u8 {
        Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Disable in Doze mode"]
    DOZE_DIS = 0x0,
    #[doc = "Enable in Doze mode"]
    DOZE_EN = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enmod {
    #[doc = "Restart counting from their frozen values after GPT is enabled (EN=1)."]
    RESUME_COUNT = 0x0,
    #[doc = "Reset counting from 0 after GPT is enabled (EN=1)."]
    ZERO_COUNT = 0x01,
}
impl Enmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enmod {
    #[inline(always)]
    fn from(val: u8) -> Enmod {
        Enmod::from_bits(val)
    }
}
impl From<Enmod> for u8 {
    #[inline(always)]
    fn from(val: Enmod) -> u8 {
        Enmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frr {
    #[doc = "Restart mode. After a compare event, the counter resets to 0x0000_0000 and resumes counting."]
    RESTART = 0x0,
    #[doc = "Free-Run mode. After a compare event, the counter continues counting until 0xFFFF_FFFF and then rolls over to 0."]
    FREE_RUN = 0x01,
}
impl Frr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frr {
    #[inline(always)]
    fn from(val: u8) -> Frr {
        Frr::from_bits(val)
    }
}
impl From<Frr> for u8 {
    #[inline(always)]
    fn from(val: Frr) -> u8 {
        Frr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Im1 {
    #[doc = "Capture disabled"]
    DISABLED = 0x0,
    #[doc = "Capture on rising edge only"]
    RISING = 0x01,
    #[doc = "Capture on falling edge only"]
    FALLING = 0x02,
    #[doc = "Capture on both edges"]
    BOTH = 0x03,
}
impl Im1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Im1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Im1 {
    #[inline(always)]
    fn from(val: u8) -> Im1 {
        Im1::from_bits(val)
    }
}
impl From<Im1> for u8 {
    #[inline(always)]
    fn from(val: Im1) -> u8 {
        Im1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Im2 {
    #[doc = "Capture disabled"]
    DISABLED = 0x0,
    #[doc = "Capture on rising edge only"]
    RISING = 0x01,
    #[doc = "Capture on falling edge only"]
    FALLING = 0x02,
    #[doc = "Capture on both edges"]
    BOTH = 0x03,
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
pub enum Om1 {
    #[doc = "Output disabled. No response on pin."]
    DISABLED = 0x0,
    #[doc = "Toggle output pin"]
    TOGGLE = 0x01,
    #[doc = "Clear output pin"]
    CLEAR = 0x02,
    #[doc = "Set output pin"]
    SET = 0x03,
    #[doc = "Generate a low pulse that is one input clock cycle wide on the output pin. When OMn is first programmed as 1xx, the output pin is set to one immediately on the next input clock (if it was not one already). \"Input clock\" here refers to the clock selected by the CLKSRC field of this register."]
    PULSE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Om1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Om1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Om1 {
    #[inline(always)]
    fn from(val: u8) -> Om1 {
        Om1::from_bits(val)
    }
}
impl From<Om1> for u8 {
    #[inline(always)]
    fn from(val: Om1) -> u8 {
        Om1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Om2 {
    #[doc = "Output disabled. No response on pin."]
    DISABLED = 0x0,
    #[doc = "Toggle output pin"]
    TOGGLE = 0x01,
    #[doc = "Clear output pin"]
    CLEAR = 0x02,
    #[doc = "Set output pin"]
    SET = 0x03,
    #[doc = "Generate a low pulse that is one input clock cycle wide on the output pin. When OMn is first programmed as 1xx, the output pin is set to one immediately on the next input clock (if it was not one already). \"Input clock\" here refers to the clock selected by the CLKSRC field of this register."]
    PULSE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Om2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Om2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Om2 {
    #[inline(always)]
    fn from(val: u8) -> Om2 {
        Om2::from_bits(val)
    }
}
impl From<Om2> for u8 {
    #[inline(always)]
    fn from(val: Om2) -> u8 {
        Om2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Om3 {
    #[doc = "Output disabled. No response on pin."]
    DISABLED = 0x0,
    #[doc = "Toggle output pin"]
    TOGGLE = 0x01,
    #[doc = "Clear output pin"]
    CLEAR = 0x02,
    #[doc = "Set output pin"]
    SET = 0x03,
    #[doc = "Generate a low pulse that is one input clock cycle wide on the output pin. When OMn is first programmed as 1xx, the output pin is set to one immediately on the next input clock (if it was not one already). \"Input clock\" here refers to the clock selected by the CLKSRC field of this register."]
    PULSE = 0x04,
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
    pub const DIV_BY_1: Self = Self(0x0);
    #[doc = "Divide by 2"]
    pub const DIV_BY_2: Self = Self(0x01);
    #[doc = "Divide by 4096"]
    pub const DIV_BY_4096: Self = Self(0x0fff);
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
            0x0 => f.write_str("DIV_BY_1"),
            0x01 => f.write_str("DIV_BY_2"),
            0x0fff => f.write_str("DIV_BY_4096"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prescaler {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DIV_BY_1"),
            0x01 => defmt::write!(f, "DIV_BY_2"),
            0x0fff => defmt::write!(f, "DIV_BY_4096"),
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
    DIV_BY_1 = 0x0,
    #[doc = "Divide by 2"]
    DIV_BY_2 = 0x01,
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
    DIV_BY_16 = 0x0f,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopen {
    #[doc = "Disable in Stop mode"]
    STOP_DIS = 0x0,
    #[doc = "Enable in Stop mode"]
    STOP_EN = 0x01,
}
impl Stopen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopen {
    #[inline(always)]
    fn from(val: u8) -> Stopen {
        Stopen::from_bits(val)
    }
}
impl From<Stopen> for u8 {
    #[inline(always)]
    fn from(val: Stopen) -> u8 {
        Stopen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Waiten {
    #[doc = "Disable in Wait mode"]
    WAIT_DIS = 0x0,
    #[doc = "Enable in Wait mode"]
    WAIT_EN = 0x01,
}
impl Waiten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Waiten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Waiten {
    #[inline(always)]
    fn from(val: u8) -> Waiten {
        Waiten::from_bits(val)
    }
}
impl From<Waiten> for u8 {
    #[inline(always)]
    fn from(val: Waiten) -> u8 {
        Waiten::to_bits(val)
    }
}
