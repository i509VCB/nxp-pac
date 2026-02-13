#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BidirH {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    UP = 0x0,
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 0x01,
}
impl BidirH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BidirH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BidirH {
    #[inline(always)]
    fn from(val: u8) -> BidirH {
        BidirH::from_bits(val)
    }
}
impl From<BidirH> for u8 {
    #[inline(always)]
    fn from(val: BidirH) -> u8 {
        BidirH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BidirL {
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    UP = 0x0,
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 0x01,
}
impl BidirL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BidirL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BidirL {
    #[inline(always)]
    fn from(val: u8) -> BidirL {
        BidirL::from_bits(val)
    }
}
impl From<BidirL> for u8 {
    #[inline(always)]
    fn from(val: BidirL) -> u8 {
        BidirL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cksel {
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
impl Cksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cksel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cksel {
    #[inline(always)]
    fn from(val: u8) -> Cksel {
        Cksel::from_bits(val)
    }
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(val: Cksel) -> u8 {
        Cksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkmode {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE = 0x0,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE = 0x01,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE = 0x02,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE = 0x03,
}
impl Clkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkmode {
    #[inline(always)]
    fn from(val: u8) -> Clkmode {
        Clkmode::from_bits(val)
    }
}
impl From<Clkmode> for u8 {
    #[inline(always)]
    fn from(val: Clkmode) -> u8 {
        Clkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Combmode {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    OR = 0x0,
    #[doc = "MATCH. Uses the specified match only."]
    MATCH = 0x01,
    #[doc = "IO. Uses the specified I/O condition only."]
    IO = 0x02,
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND = 0x03,
}
impl Combmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Combmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Combmode {
    #[inline(always)]
    fn from(val: u8) -> Combmode {
        Combmode::from_bits(val)
    }
}
impl From<Combmode> for u8 {
    #[inline(always)]
    fn from(val: Combmode) -> u8 {
        Combmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDENT = 0x0,
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP = 0x01,
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN = 0x02,
    _RESERVED_3 = 0x03,
}
impl Direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Direction {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Direction {
    #[inline(always)]
    fn from(val: u8) -> Direction {
        Direction::from_bits(val)
    }
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(val: Direction) -> u8 {
        Direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hevent {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    L_COUNTER = 0x0,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    H_COUNTER = 0x01,
}
impl Hevent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hevent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hevent {
    #[inline(always)]
    fn from(val: u8) -> Hevent {
        Hevent::from_bits(val)
    }
}
impl From<Hevent> for u8 {
    #[inline(always)]
    fn from(val: Hevent) -> u8 {
        Hevent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iocond {
    #[doc = "LOW"]
    LOW = 0x0,
    #[doc = "Rise"]
    RISE = 0x01,
    #[doc = "Fall"]
    FALL = 0x02,
    #[doc = "HIGH"]
    HIGH = 0x03,
}
impl Iocond {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iocond {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iocond {
    #[inline(always)]
    fn from(val: u8) -> Iocond {
        Iocond::from_bits(val)
    }
}
impl From<Iocond> for u8 {
    #[inline(always)]
    fn from(val: Iocond) -> u8 {
        Iocond::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O0res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O0res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O0res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O0res {
    #[inline(always)]
    fn from(val: u8) -> O0res {
        O0res::from_bits(val)
    }
}
impl From<O0res> for u8 {
    #[inline(always)]
    fn from(val: O0res) -> u8 {
        O0res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O10res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O10res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O10res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O10res {
    #[inline(always)]
    fn from(val: u8) -> O10res {
        O10res::from_bits(val)
    }
}
impl From<O10res> for u8 {
    #[inline(always)]
    fn from(val: O10res) -> u8 {
        O10res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O11res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O11res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O11res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O11res {
    #[inline(always)]
    fn from(val: u8) -> O11res {
        O11res::from_bits(val)
    }
}
impl From<O11res> for u8 {
    #[inline(always)]
    fn from(val: O11res) -> u8 {
        O11res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O12res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O12res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O12res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O12res {
    #[inline(always)]
    fn from(val: u8) -> O12res {
        O12res::from_bits(val)
    }
}
impl From<O12res> for u8 {
    #[inline(always)]
    fn from(val: O12res) -> u8 {
        O12res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O13res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O13res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O13res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O13res {
    #[inline(always)]
    fn from(val: u8) -> O13res {
        O13res::from_bits(val)
    }
}
impl From<O13res> for u8 {
    #[inline(always)]
    fn from(val: O13res) -> u8 {
        O13res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O14res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O14res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O14res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O14res {
    #[inline(always)]
    fn from(val: u8) -> O14res {
        O14res::from_bits(val)
    }
}
impl From<O14res> for u8 {
    #[inline(always)]
    fn from(val: O14res) -> u8 {
        O14res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O15res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O15res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O15res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O15res {
    #[inline(always)]
    fn from(val: u8) -> O15res {
        O15res::from_bits(val)
    }
}
impl From<O15res> for u8 {
    #[inline(always)]
    fn from(val: O15res) -> u8 {
        O15res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O1res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O1res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O1res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O1res {
    #[inline(always)]
    fn from(val: u8) -> O1res {
        O1res::from_bits(val)
    }
}
impl From<O1res> for u8 {
    #[inline(always)]
    fn from(val: O1res) -> u8 {
        O1res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O2res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O2res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O2res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O2res {
    #[inline(always)]
    fn from(val: u8) -> O2res {
        O2res::from_bits(val)
    }
}
impl From<O2res> for u8 {
    #[inline(always)]
    fn from(val: O2res) -> u8 {
        O2res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O3res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O3res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O3res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O3res {
    #[inline(always)]
    fn from(val: u8) -> O3res {
        O3res::from_bits(val)
    }
}
impl From<O3res> for u8 {
    #[inline(always)]
    fn from(val: O3res) -> u8 {
        O3res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O4res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O4res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O4res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O4res {
    #[inline(always)]
    fn from(val: u8) -> O4res {
        O4res::from_bits(val)
    }
}
impl From<O4res> for u8 {
    #[inline(always)]
    fn from(val: O4res) -> u8 {
        O4res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O5res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O5res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O5res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O5res {
    #[inline(always)]
    fn from(val: u8) -> O5res {
        O5res::from_bits(val)
    }
}
impl From<O5res> for u8 {
    #[inline(always)]
    fn from(val: O5res) -> u8 {
        O5res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O6res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O6res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O6res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O6res {
    #[inline(always)]
    fn from(val: u8) -> O6res {
        O6res::from_bits(val)
    }
}
impl From<O6res> for u8 {
    #[inline(always)]
    fn from(val: O6res) -> u8 {
        O6res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O7res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O7res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O7res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O7res {
    #[inline(always)]
    fn from(val: u8) -> O7res {
        O7res::from_bits(val)
    }
}
impl From<O7res> for u8 {
    #[inline(always)]
    fn from(val: O7res) -> u8 {
        O7res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O8res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O8res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O8res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O8res {
    #[inline(always)]
    fn from(val: u8) -> O8res {
        O8res::from_bits(val)
    }
}
impl From<O8res> for u8 {
    #[inline(always)]
    fn from(val: O8res) -> u8 {
        O8res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O9res {
    #[doc = "No change."]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET = 0x01,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR = 0x02,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT = 0x03,
}
impl O9res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> O9res {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for O9res {
    #[inline(always)]
    fn from(val: u8) -> O9res {
        O9res::from_bits(val)
    }
}
impl From<O9res> for u8 {
    #[inline(always)]
    fn from(val: O9res) -> u8 {
        O9res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "Selects the inputs selected by IOSEL."]
    INPUT = 0x0,
    #[doc = "Selects the outputs selected by IOSEL."]
    OUTPUT = 0x01,
}
impl Outsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outsel {
    #[inline(always)]
    fn from(val: u8) -> Outsel {
        Outsel::from_bits(val)
    }
}
impl From<Outsel> for u8 {
    #[inline(always)]
    fn from(val: Outsel) -> u8 {
        Outsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr0 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr0 {
    #[inline(always)]
    fn from(val: u8) -> Setclr0 {
        Setclr0::from_bits(val)
    }
}
impl From<Setclr0> for u8 {
    #[inline(always)]
    fn from(val: Setclr0) -> u8 {
        Setclr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr1 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr1 {
    #[inline(always)]
    fn from(val: u8) -> Setclr1 {
        Setclr1::from_bits(val)
    }
}
impl From<Setclr1> for u8 {
    #[inline(always)]
    fn from(val: Setclr1) -> u8 {
        Setclr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr10 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr10 {
    #[inline(always)]
    fn from(val: u8) -> Setclr10 {
        Setclr10::from_bits(val)
    }
}
impl From<Setclr10> for u8 {
    #[inline(always)]
    fn from(val: Setclr10) -> u8 {
        Setclr10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr11 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr11 {
    #[inline(always)]
    fn from(val: u8) -> Setclr11 {
        Setclr11::from_bits(val)
    }
}
impl From<Setclr11> for u8 {
    #[inline(always)]
    fn from(val: Setclr11) -> u8 {
        Setclr11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr12 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr12 {
    #[inline(always)]
    fn from(val: u8) -> Setclr12 {
        Setclr12::from_bits(val)
    }
}
impl From<Setclr12> for u8 {
    #[inline(always)]
    fn from(val: Setclr12) -> u8 {
        Setclr12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr13 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr13 {
    #[inline(always)]
    fn from(val: u8) -> Setclr13 {
        Setclr13::from_bits(val)
    }
}
impl From<Setclr13> for u8 {
    #[inline(always)]
    fn from(val: Setclr13) -> u8 {
        Setclr13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr14 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr14 {
    #[inline(always)]
    fn from(val: u8) -> Setclr14 {
        Setclr14::from_bits(val)
    }
}
impl From<Setclr14> for u8 {
    #[inline(always)]
    fn from(val: Setclr14) -> u8 {
        Setclr14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr15 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr15 {
    #[inline(always)]
    fn from(val: u8) -> Setclr15 {
        Setclr15::from_bits(val)
    }
}
impl From<Setclr15> for u8 {
    #[inline(always)]
    fn from(val: Setclr15) -> u8 {
        Setclr15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr2 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr2 {
    #[inline(always)]
    fn from(val: u8) -> Setclr2 {
        Setclr2::from_bits(val)
    }
}
impl From<Setclr2> for u8 {
    #[inline(always)]
    fn from(val: Setclr2) -> u8 {
        Setclr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr3 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr3 {
    #[inline(always)]
    fn from(val: u8) -> Setclr3 {
        Setclr3::from_bits(val)
    }
}
impl From<Setclr3> for u8 {
    #[inline(always)]
    fn from(val: Setclr3) -> u8 {
        Setclr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr4 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr4 {
    #[inline(always)]
    fn from(val: u8) -> Setclr4 {
        Setclr4::from_bits(val)
    }
}
impl From<Setclr4> for u8 {
    #[inline(always)]
    fn from(val: Setclr4) -> u8 {
        Setclr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr5 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr5 {
    #[inline(always)]
    fn from(val: u8) -> Setclr5 {
        Setclr5::from_bits(val)
    }
}
impl From<Setclr5> for u8 {
    #[inline(always)]
    fn from(val: Setclr5) -> u8 {
        Setclr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr6 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr6 {
    #[inline(always)]
    fn from(val: u8) -> Setclr6 {
        Setclr6::from_bits(val)
    }
}
impl From<Setclr6> for u8 {
    #[inline(always)]
    fn from(val: Setclr6) -> u8 {
        Setclr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr7 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr7 {
    #[inline(always)]
    fn from(val: u8) -> Setclr7 {
        Setclr7::from_bits(val)
    }
}
impl From<Setclr7> for u8 {
    #[inline(always)]
    fn from(val: Setclr7) -> u8 {
        Setclr7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr8 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr8 {
    #[inline(always)]
    fn from(val: u8) -> Setclr8 {
        Setclr8::from_bits(val)
    }
}
impl From<Setclr8> for u8 {
    #[inline(always)]
    fn from(val: Setclr8) -> u8 {
        Setclr8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr9 {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr9 {
    #[inline(always)]
    fn from(val: u8) -> Setclr9 {
        Setclr9::from_bits(val)
    }
}
impl From<Setclr9> for u8 {
    #[inline(always)]
    fn from(val: Setclr9) -> u8 {
        Setclr9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stateld {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    ADD = 0x0,
    #[doc = "STATEV value is loaded into STATE."]
    LOAD = 0x01,
}
impl Stateld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stateld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stateld {
    #[inline(always)]
    fn from(val: u8) -> Stateld {
        Stateld::from_bits(val)
    }
}
impl From<Stateld> for u8 {
    #[inline(always)]
    fn from(val: Stateld) -> u8 {
        Stateld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unify {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER = 0x0,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER = 0x01,
}
impl Unify {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unify {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unify {
    #[inline(always)]
    fn from(val: u8) -> Unify {
        Unify::from_bits(val)
    }
}
impl From<Unify> for u8 {
    #[inline(always)]
    fn from(val: Unify) -> u8 {
        Unify::to_bits(val)
    }
}
