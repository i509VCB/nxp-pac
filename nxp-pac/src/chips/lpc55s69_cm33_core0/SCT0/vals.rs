#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BIDIR_H {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    UP = 0x0,
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 0x01,
}
impl BIDIR_H {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BIDIR_H {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BIDIR_H {
    #[inline(always)]
    fn from(val: u8) -> BIDIR_H {
        BIDIR_H::from_bits(val)
    }
}
impl From<BIDIR_H> for u8 {
    #[inline(always)]
    fn from(val: BIDIR_H) -> u8 {
        BIDIR_H::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BIDIR_L {
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    UP = 0x0,
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 0x01,
}
impl BIDIR_L {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BIDIR_L {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BIDIR_L {
    #[inline(always)]
    fn from(val: u8) -> BIDIR_L {
        BIDIR_L::from_bits(val)
    }
}
impl From<BIDIR_L> for u8 {
    #[inline(always)]
    fn from(val: BIDIR_L) -> u8 {
        BIDIR_L::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CKSEL {
    #[doc = "Rising edges on input 0."]
    INPUT_0_RISING_EDGES = 0x0,
    #[doc = "Falling edges on input 0."]
    INPUT_0_FALLING_EDGE = 0x01,
    #[doc = "Rising edges on input 1."]
    INPUT_1_RISING_EDGES = 0x02,
    #[doc = "Falling edges on input 1."]
    INPUT_1_FALLING_EDGE = 0x03,
    #[doc = "Rising edges on input 2."]
    INPUT_2_RISING_EDGES = 0x04,
    #[doc = "Falling edges on input 2."]
    INPUT_2_FALLING_EDGE = 0x05,
    #[doc = "Rising edges on input 3."]
    INPUT_3_RISING_EDGES = 0x06,
    #[doc = "Falling edges on input 3."]
    INPUT_3_FALLING_EDGE = 0x07,
    #[doc = "Rising edges on input 4."]
    INPUT_4_RISING_EDGES = 0x08,
    #[doc = "Falling edges on input 4."]
    INPUT_4_FALLING_EDGE = 0x09,
    #[doc = "Rising edges on input 5."]
    INPUT_5_RISING_EDGES = 0x0a,
    #[doc = "Falling edges on input 5."]
    INPUT_5_FALLING_EDGE = 0x0b,
    #[doc = "Rising edges on input 6."]
    INPUT_6_RISING_EDGES = 0x0c,
    #[doc = "Falling edges on input 6."]
    INPUT_6_FALLING_EDGE = 0x0d,
    #[doc = "Rising edges on input 7."]
    INPUT_7_RISING_EDGES = 0x0e,
    #[doc = "Falling edges on input 7."]
    INPUT_7_FALLING_EDGE = 0x0f,
}
impl CKSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CKSEL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CKSEL {
    #[inline(always)]
    fn from(val: u8) -> CKSEL {
        CKSEL::from_bits(val)
    }
}
impl From<CKSEL> for u8 {
    #[inline(always)]
    fn from(val: CKSEL) -> u8 {
        CKSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKMODE {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE = 0x0,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE = 0x01,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE = 0x02,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE = 0x03,
}
impl CLKMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKMODE {
    #[inline(always)]
    fn from(val: u8) -> CLKMODE {
        CLKMODE::from_bits(val)
    }
}
impl From<CLKMODE> for u8 {
    #[inline(always)]
    fn from(val: CLKMODE) -> u8 {
        CLKMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMBMODE {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    OR = 0x0,
    #[doc = "MATCH. Uses the specified match only."]
    MATCH = 0x01,
    #[doc = "IO. Uses the specified I/O condition only."]
    IO = 0x02,
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND = 0x03,
}
impl COMBMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMBMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMBMODE {
    #[inline(always)]
    fn from(val: u8) -> COMBMODE {
        COMBMODE::from_bits(val)
    }
}
impl From<COMBMODE> for u8 {
    #[inline(always)]
    fn from(val: COMBMODE) -> u8 {
        COMBMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DIRECTION {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDENT = 0x0,
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP = 0x01,
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN = 0x02,
    _RESERVED_3 = 0x03,
}
impl DIRECTION {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DIRECTION {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DIRECTION {
    #[inline(always)]
    fn from(val: u8) -> DIRECTION {
        DIRECTION::from_bits(val)
    }
}
impl From<DIRECTION> for u8 {
    #[inline(always)]
    fn from(val: DIRECTION) -> u8 {
        DIRECTION::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HEVENT {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    L_COUNTER = 0x0,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    H_COUNTER = 0x01,
}
impl HEVENT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HEVENT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HEVENT {
    #[inline(always)]
    fn from(val: u8) -> HEVENT {
        HEVENT::from_bits(val)
    }
}
impl From<HEVENT> for u8 {
    #[inline(always)]
    fn from(val: HEVENT) -> u8 {
        HEVENT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCOND {
    #[doc = "LOW."]
    LOW = 0x0,
    #[doc = "Rise."]
    RISE = 0x01,
    #[doc = "Fall."]
    FALL = 0x02,
    #[doc = "HIGH."]
    HIGH = 0x03,
}
impl IOCOND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCOND {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCOND {
    #[inline(always)]
    fn from(val: u8) -> IOCOND {
        IOCOND::from_bits(val)
    }
}
impl From<IOCOND> for u8 {
    #[inline(always)]
    fn from(val: IOCOND) -> u8 {
        IOCOND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O0RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O0RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O0RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O0RES {
    #[inline(always)]
    fn from(val: u8) -> O0RES {
        O0RES::from_bits(val)
    }
}
impl From<O0RES> for u8 {
    #[inline(always)]
    fn from(val: O0RES) -> u8 {
        O0RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O10RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O10RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O10RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O10RES {
    #[inline(always)]
    fn from(val: u8) -> O10RES {
        O10RES::from_bits(val)
    }
}
impl From<O10RES> for u8 {
    #[inline(always)]
    fn from(val: O10RES) -> u8 {
        O10RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O11RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O11RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O11RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O11RES {
    #[inline(always)]
    fn from(val: u8) -> O11RES {
        O11RES::from_bits(val)
    }
}
impl From<O11RES> for u8 {
    #[inline(always)]
    fn from(val: O11RES) -> u8 {
        O11RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O12RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O12RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O12RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O12RES {
    #[inline(always)]
    fn from(val: u8) -> O12RES {
        O12RES::from_bits(val)
    }
}
impl From<O12RES> for u8 {
    #[inline(always)]
    fn from(val: O12RES) -> u8 {
        O12RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O13RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O13RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O13RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O13RES {
    #[inline(always)]
    fn from(val: u8) -> O13RES {
        O13RES::from_bits(val)
    }
}
impl From<O13RES> for u8 {
    #[inline(always)]
    fn from(val: O13RES) -> u8 {
        O13RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O14RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O14RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O14RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O14RES {
    #[inline(always)]
    fn from(val: u8) -> O14RES {
        O14RES::from_bits(val)
    }
}
impl From<O14RES> for u8 {
    #[inline(always)]
    fn from(val: O14RES) -> u8 {
        O14RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O15RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O15RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O15RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O15RES {
    #[inline(always)]
    fn from(val: u8) -> O15RES {
        O15RES::from_bits(val)
    }
}
impl From<O15RES> for u8 {
    #[inline(always)]
    fn from(val: O15RES) -> u8 {
        O15RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O1RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O1RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O1RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O1RES {
    #[inline(always)]
    fn from(val: u8) -> O1RES {
        O1RES::from_bits(val)
    }
}
impl From<O1RES> for u8 {
    #[inline(always)]
    fn from(val: O1RES) -> u8 {
        O1RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O2RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O2RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O2RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O2RES {
    #[inline(always)]
    fn from(val: u8) -> O2RES {
        O2RES::from_bits(val)
    }
}
impl From<O2RES> for u8 {
    #[inline(always)]
    fn from(val: O2RES) -> u8 {
        O2RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O3RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O3RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O3RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O3RES {
    #[inline(always)]
    fn from(val: u8) -> O3RES {
        O3RES::from_bits(val)
    }
}
impl From<O3RES> for u8 {
    #[inline(always)]
    fn from(val: O3RES) -> u8 {
        O3RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O4RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O4RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O4RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O4RES {
    #[inline(always)]
    fn from(val: u8) -> O4RES {
        O4RES::from_bits(val)
    }
}
impl From<O4RES> for u8 {
    #[inline(always)]
    fn from(val: O4RES) -> u8 {
        O4RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O5RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O5RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O5RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O5RES {
    #[inline(always)]
    fn from(val: u8) -> O5RES {
        O5RES::from_bits(val)
    }
}
impl From<O5RES> for u8 {
    #[inline(always)]
    fn from(val: O5RES) -> u8 {
        O5RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O6RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O6RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O6RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O6RES {
    #[inline(always)]
    fn from(val: u8) -> O6RES {
        O6RES::from_bits(val)
    }
}
impl From<O6RES> for u8 {
    #[inline(always)]
    fn from(val: O6RES) -> u8 {
        O6RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O7RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O7RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O7RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O7RES {
    #[inline(always)]
    fn from(val: u8) -> O7RES {
        O7RES::from_bits(val)
    }
}
impl From<O7RES> for u8 {
    #[inline(always)]
    fn from(val: O7RES) -> u8 {
        O7RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O8RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O8RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O8RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O8RES {
    #[inline(always)]
    fn from(val: u8) -> O8RES {
        O8RES::from_bits(val)
    }
}
impl From<O8RES> for u8 {
    #[inline(always)]
    fn from(val: O8RES) -> u8 {
        O8RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O9RES {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O9RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O9RES {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O9RES {
    #[inline(always)]
    fn from(val: u8) -> O9RES {
        O9RES::from_bits(val)
    }
}
impl From<O9RES> for u8 {
    #[inline(always)]
    fn from(val: O9RES) -> u8 {
        O9RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OUTSEL {
    #[doc = "Selects the inputs selected by IOSEL."]
    INPUT = 0x0,
    #[doc = "Selects the outputs selected by IOSEL."]
    OUTPUT = 0x01,
}
impl OUTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OUTSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OUTSEL {
    #[inline(always)]
    fn from(val: u8) -> OUTSEL {
        OUTSEL::from_bits(val)
    }
}
impl From<OUTSEL> for u8 {
    #[inline(always)]
    fn from(val: OUTSEL) -> u8 {
        OUTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR0 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR0 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR0 {
        SETCLR0::from_bits(val)
    }
}
impl From<SETCLR0> for u8 {
    #[inline(always)]
    fn from(val: SETCLR0) -> u8 {
        SETCLR0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR1 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR1 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR1 {
        SETCLR1::from_bits(val)
    }
}
impl From<SETCLR1> for u8 {
    #[inline(always)]
    fn from(val: SETCLR1) -> u8 {
        SETCLR1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR10 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR10 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR10 {
        SETCLR10::from_bits(val)
    }
}
impl From<SETCLR10> for u8 {
    #[inline(always)]
    fn from(val: SETCLR10) -> u8 {
        SETCLR10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR11 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR11 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR11 {
        SETCLR11::from_bits(val)
    }
}
impl From<SETCLR11> for u8 {
    #[inline(always)]
    fn from(val: SETCLR11) -> u8 {
        SETCLR11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR12 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR12 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR12 {
        SETCLR12::from_bits(val)
    }
}
impl From<SETCLR12> for u8 {
    #[inline(always)]
    fn from(val: SETCLR12) -> u8 {
        SETCLR12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR13 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR13 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR13 {
        SETCLR13::from_bits(val)
    }
}
impl From<SETCLR13> for u8 {
    #[inline(always)]
    fn from(val: SETCLR13) -> u8 {
        SETCLR13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR14 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR14 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR14 {
        SETCLR14::from_bits(val)
    }
}
impl From<SETCLR14> for u8 {
    #[inline(always)]
    fn from(val: SETCLR14) -> u8 {
        SETCLR14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR15 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR15 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR15 {
        SETCLR15::from_bits(val)
    }
}
impl From<SETCLR15> for u8 {
    #[inline(always)]
    fn from(val: SETCLR15) -> u8 {
        SETCLR15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR2 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR2 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR2 {
        SETCLR2::from_bits(val)
    }
}
impl From<SETCLR2> for u8 {
    #[inline(always)]
    fn from(val: SETCLR2) -> u8 {
        SETCLR2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR3 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR3 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR3 {
        SETCLR3::from_bits(val)
    }
}
impl From<SETCLR3> for u8 {
    #[inline(always)]
    fn from(val: SETCLR3) -> u8 {
        SETCLR3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR4 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR4 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR4 {
        SETCLR4::from_bits(val)
    }
}
impl From<SETCLR4> for u8 {
    #[inline(always)]
    fn from(val: SETCLR4) -> u8 {
        SETCLR4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR5 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR5 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR5 {
        SETCLR5::from_bits(val)
    }
}
impl From<SETCLR5> for u8 {
    #[inline(always)]
    fn from(val: SETCLR5) -> u8 {
        SETCLR5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR6 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR6 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR6 {
        SETCLR6::from_bits(val)
    }
}
impl From<SETCLR6> for u8 {
    #[inline(always)]
    fn from(val: SETCLR6) -> u8 {
        SETCLR6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR7 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR7 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR7 {
        SETCLR7::from_bits(val)
    }
}
impl From<SETCLR7> for u8 {
    #[inline(always)]
    fn from(val: SETCLR7) -> u8 {
        SETCLR7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR8 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR8 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR8 {
        SETCLR8::from_bits(val)
    }
}
impl From<SETCLR8> for u8 {
    #[inline(always)]
    fn from(val: SETCLR8) -> u8 {
        SETCLR8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SETCLR9 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl SETCLR9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SETCLR9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SETCLR9 {
    #[inline(always)]
    fn from(val: u8) -> SETCLR9 {
        SETCLR9::from_bits(val)
    }
}
impl From<SETCLR9> for u8 {
    #[inline(always)]
    fn from(val: SETCLR9) -> u8 {
        SETCLR9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STATELD {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    ADD = 0x0,
    #[doc = "STATEV value is loaded into STATE."]
    LOAD = 0x01,
}
impl STATELD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STATELD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STATELD {
    #[inline(always)]
    fn from(val: u8) -> STATELD {
        STATELD::from_bits(val)
    }
}
impl From<STATELD> for u8 {
    #[inline(always)]
    fn from(val: STATELD) -> u8 {
        STATELD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UNIFY {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER = 0x0,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER = 0x01,
}
impl UNIFY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UNIFY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UNIFY {
    #[inline(always)]
    fn from(val: u8) -> UNIFY {
        UNIFY::from_bits(val)
    }
}
impl From<UNIFY> for u8 {
    #[inline(always)]
    fn from(val: UNIFY) -> u8 {
        UNIFY::to_bits(val)
    }
}
