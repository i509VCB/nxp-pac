#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BidirH {
    #[doc = "Up"]
    UP = 0x0,
    #[doc = "Up-down"]
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
    #[doc = "Up"]
    UP = 0x0,
    #[doc = "Up-down"]
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
    #[doc = "Rising edges on input 0"]
    INPUT_0_RISING_EDGES = 0x0,
    #[doc = "Falling edges on input 0"]
    INPUT_0_FALLING_EDGES = 0x01,
    #[doc = "Rising edges on input 1"]
    INPUT_1_RISING_EDGES = 0x02,
    #[doc = "Falling edges on input 1"]
    INPUT_1_FALLING_EDGES = 0x03,
    #[doc = "Rising edges on input 2"]
    INPUT_2_RISING_EDGES = 0x04,
    #[doc = "Falling edges on input 2"]
    INPUT_2_FALLING_EDGES = 0x05,
    #[doc = "Rising edges on input 3"]
    INPUT_3_RISING_EDGES = 0x06,
    #[doc = "Falling edges on input 3"]
    INPUT_3_FALLING_EDGES = 0x07,
    #[doc = "Rising edges on input 4"]
    INPUT_4_RISING_EDGES = 0x08,
    #[doc = "Falling edges on input 4"]
    INPUT_4_FALLING_EDGES = 0x09,
    #[doc = "Rising edges on input 5"]
    INPUT_5_RISING_EDGES = 0x0a,
    #[doc = "Falling edges on input 5"]
    INPUT_5_FALLING_EDGES = 0x0b,
    #[doc = "Rising edges on input 6"]
    INPUT_6_RISING_EDGES = 0x0c,
    #[doc = "Falling edges on input 6"]
    INPUT_6_FALLING_EDGES = 0x0d,
    #[doc = "Rising edges on input 7"]
    INPUT_7_RISING_EDGES = 0x0e,
    #[doc = "Falling edges on input 7"]
    INPUT_7_FALLING_EDGES = 0x0f,
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
    #[doc = "System Clock mode"]
    SYSTEM_CLOCK_MODE = 0x0,
    #[doc = "Sampled System Clock mode"]
    SAMPLED_SYSTEM_CLOCK_MODE = 0x01,
    #[doc = "SCT Input Clock mode"]
    SCT_INPUT_CLOCK_MODE = 0x02,
    #[doc = "Asynchronous mode"]
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
    #[doc = "OR (the event occurs when either the specified match or I/O condition occurs)"]
    OR = 0x0,
    #[doc = "MATCH (uses the specified match only)"]
    MATCH = 0x01,
    #[doc = "IO (uses the specified I/O condition only)"]
    IO = 0x02,
    #[doc = "AND (the event occurs when the specified match and I/O condition occur simultaneously)"]
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
    #[doc = "Direction independent (event triggered regardless of the count direction)"]
    DIRECTION_INDEPENDENT = 0x0,
    #[doc = "Counting up (event triggered only during up-counting when CTRL\\[BIDIR\\] = 1)"]
    COUNTING_UP = 0x01,
    #[doc = "Counting down (event triggered only during down-counting when CTRL\\[BIDIR\\] = 1)"]
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
pub enum DownH {
    #[doc = "Up"]
    UP = 0x0,
    #[doc = "Down"]
    DOWN = 0x01,
}
impl DownH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DownH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DownH {
    #[inline(always)]
    fn from(val: u8) -> DownH {
        DownH::from_bits(val)
    }
}
impl From<DownH> for u8 {
    #[inline(always)]
    fn from(val: DownH) -> u8 {
        DownH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DownL {
    #[doc = "Up"]
    UP = 0x0,
    #[doc = "Down"]
    DOWN = 0x01,
}
impl DownL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DownL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DownL {
    #[inline(always)]
    fn from(val: u8) -> DownL {
        DownL::from_bits(val)
    }
}
impl From<DownL> for u8 {
    #[inline(always)]
    fn from(val: DownL) -> u8 {
        DownL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hevent {
    #[doc = "Low counter (selects the L state and the L match register that the MATCHSEL field specifies)"]
    L_COUNTER = 0x0,
    #[doc = "High counter (selects the H state and the H match register that the MATCHSEL field specifies)"]
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
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Rise"]
    RISE = 0x01,
    #[doc = "Fall"]
    FALL = 0x02,
    #[doc = "High"]
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
pub enum NoreloadH {
    #[doc = "Reloaded"]
    RELOAD_H = 0x0,
    #[doc = "Not reloaded"]
    NO_RELOAD_H = 0x01,
}
impl NoreloadH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoreloadH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoreloadH {
    #[inline(always)]
    fn from(val: u8) -> NoreloadH {
        NoreloadH::from_bits(val)
    }
}
impl From<NoreloadH> for u8 {
    #[inline(always)]
    fn from(val: NoreloadH) -> u8 {
        NoreloadH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoreloadL {
    #[doc = "Reloaded"]
    RELOAD = 0x0,
    #[doc = "Not reloaded"]
    NO_RELOAD = 0x01,
}
impl NoreloadL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoreloadL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoreloadL {
    #[inline(always)]
    fn from(val: u8) -> NoreloadL {
        NoreloadL::from_bits(val)
    }
}
impl From<NoreloadL> for u8 {
    #[inline(always)]
    fn from(val: NoreloadL) -> u8 {
        NoreloadL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum O0res {
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
pub enum O1res {
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
    #[doc = "No change"]
    NO_CHANGE = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    SET = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])"]
    CLEAR = 0x02,
    #[doc = "Toggle output"]
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
pub enum Out0 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out0 {
    #[inline(always)]
    fn from(val: u8) -> Out0 {
        Out0::from_bits(val)
    }
}
impl From<Out0> for u8 {
    #[inline(always)]
    fn from(val: Out0) -> u8 {
        Out0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out1 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out1 {
    #[inline(always)]
    fn from(val: u8) -> Out1 {
        Out1::from_bits(val)
    }
}
impl From<Out1> for u8 {
    #[inline(always)]
    fn from(val: Out1) -> u8 {
        Out1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out2 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out2 {
    #[inline(always)]
    fn from(val: u8) -> Out2 {
        Out2::from_bits(val)
    }
}
impl From<Out2> for u8 {
    #[inline(always)]
    fn from(val: Out2) -> u8 {
        Out2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out3 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out3 {
    #[inline(always)]
    fn from(val: u8) -> Out3 {
        Out3::from_bits(val)
    }
}
impl From<Out3> for u8 {
    #[inline(always)]
    fn from(val: Out3) -> u8 {
        Out3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out4 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out4 {
    #[inline(always)]
    fn from(val: u8) -> Out4 {
        Out4::from_bits(val)
    }
}
impl From<Out4> for u8 {
    #[inline(always)]
    fn from(val: Out4) -> u8 {
        Out4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out5 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out5 {
    #[inline(always)]
    fn from(val: u8) -> Out5 {
        Out5::from_bits(val)
    }
}
impl From<Out5> for u8 {
    #[inline(always)]
    fn from(val: Out5) -> u8 {
        Out5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out6 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out6 {
    #[inline(always)]
    fn from(val: u8) -> Out6 {
        Out6::from_bits(val)
    }
}
impl From<Out6> for u8 {
    #[inline(always)]
    fn from(val: Out6) -> u8 {
        Out6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out7 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out7 {
    #[inline(always)]
    fn from(val: u8) -> Out7 {
        Out7::from_bits(val)
    }
}
impl From<Out7> for u8 {
    #[inline(always)]
    fn from(val: Out7) -> u8 {
        Out7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out8 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out8 {
    #[inline(always)]
    fn from(val: u8) -> Out8 {
        Out8::from_bits(val)
    }
}
impl From<Out8> for u8 {
    #[inline(always)]
    fn from(val: Out8) -> u8 {
        Out8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out9 {
    #[doc = "Forces the corresponding output low"]
    LOW = 0x0,
    #[doc = "Forces the corresponding output high"]
    HIGH = 0x01,
}
impl Out9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out9 {
    #[inline(always)]
    fn from(val: u8) -> Out9 {
        Out9::from_bits(val)
    }
}
impl From<Out9> for u8 {
    #[inline(always)]
    fn from(val: Out9) -> u8 {
        Out9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "Inputs"]
    INPUT = 0x0,
    #[doc = "Outputs"]
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
pub enum RegmodH0 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH0 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH0 {
        RegmodH0::from_bits(val)
    }
}
impl From<RegmodH0> for u8 {
    #[inline(always)]
    fn from(val: RegmodH0) -> u8 {
        RegmodH0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH1 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH1 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH1 {
        RegmodH1::from_bits(val)
    }
}
impl From<RegmodH1> for u8 {
    #[inline(always)]
    fn from(val: RegmodH1) -> u8 {
        RegmodH1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH10 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH10 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH10 {
        RegmodH10::from_bits(val)
    }
}
impl From<RegmodH10> for u8 {
    #[inline(always)]
    fn from(val: RegmodH10) -> u8 {
        RegmodH10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH11 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH11 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH11 {
        RegmodH11::from_bits(val)
    }
}
impl From<RegmodH11> for u8 {
    #[inline(always)]
    fn from(val: RegmodH11) -> u8 {
        RegmodH11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH12 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH12 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH12 {
        RegmodH12::from_bits(val)
    }
}
impl From<RegmodH12> for u8 {
    #[inline(always)]
    fn from(val: RegmodH12) -> u8 {
        RegmodH12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH13 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH13 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH13 {
        RegmodH13::from_bits(val)
    }
}
impl From<RegmodH13> for u8 {
    #[inline(always)]
    fn from(val: RegmodH13) -> u8 {
        RegmodH13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH14 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH14 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH14 {
        RegmodH14::from_bits(val)
    }
}
impl From<RegmodH14> for u8 {
    #[inline(always)]
    fn from(val: RegmodH14) -> u8 {
        RegmodH14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH15 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH15 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH15 {
        RegmodH15::from_bits(val)
    }
}
impl From<RegmodH15> for u8 {
    #[inline(always)]
    fn from(val: RegmodH15) -> u8 {
        RegmodH15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH2 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH2 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH2 {
        RegmodH2::from_bits(val)
    }
}
impl From<RegmodH2> for u8 {
    #[inline(always)]
    fn from(val: RegmodH2) -> u8 {
        RegmodH2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH3 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH3 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH3 {
        RegmodH3::from_bits(val)
    }
}
impl From<RegmodH3> for u8 {
    #[inline(always)]
    fn from(val: RegmodH3) -> u8 {
        RegmodH3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH4 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH4 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH4 {
        RegmodH4::from_bits(val)
    }
}
impl From<RegmodH4> for u8 {
    #[inline(always)]
    fn from(val: RegmodH4) -> u8 {
        RegmodH4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH5 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH5 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH5 {
        RegmodH5::from_bits(val)
    }
}
impl From<RegmodH5> for u8 {
    #[inline(always)]
    fn from(val: RegmodH5) -> u8 {
        RegmodH5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH6 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH6 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH6 {
        RegmodH6::from_bits(val)
    }
}
impl From<RegmodH6> for u8 {
    #[inline(always)]
    fn from(val: RegmodH6) -> u8 {
        RegmodH6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH7 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH7 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH7 {
        RegmodH7::from_bits(val)
    }
}
impl From<RegmodH7> for u8 {
    #[inline(always)]
    fn from(val: RegmodH7) -> u8 {
        RegmodH7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH8 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH8 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH8 {
        RegmodH8::from_bits(val)
    }
}
impl From<RegmodH8> for u8 {
    #[inline(always)]
    fn from(val: RegmodH8) -> u8 {
        RegmodH8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH9 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodH9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH9 {
    #[inline(always)]
    fn from(val: u8) -> RegmodH9 {
        RegmodH9::from_bits(val)
    }
}
impl From<RegmodH9> for u8 {
    #[inline(always)]
    fn from(val: RegmodH9) -> u8 {
        RegmodH9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL0 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL0 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL0 {
        RegmodL0::from_bits(val)
    }
}
impl From<RegmodL0> for u8 {
    #[inline(always)]
    fn from(val: RegmodL0) -> u8 {
        RegmodL0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL1 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL1 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL1 {
        RegmodL1::from_bits(val)
    }
}
impl From<RegmodL1> for u8 {
    #[inline(always)]
    fn from(val: RegmodL1) -> u8 {
        RegmodL1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL10 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL10 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL10 {
        RegmodL10::from_bits(val)
    }
}
impl From<RegmodL10> for u8 {
    #[inline(always)]
    fn from(val: RegmodL10) -> u8 {
        RegmodL10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL11 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL11 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL11 {
        RegmodL11::from_bits(val)
    }
}
impl From<RegmodL11> for u8 {
    #[inline(always)]
    fn from(val: RegmodL11) -> u8 {
        RegmodL11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL12 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL12 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL12 {
        RegmodL12::from_bits(val)
    }
}
impl From<RegmodL12> for u8 {
    #[inline(always)]
    fn from(val: RegmodL12) -> u8 {
        RegmodL12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL13 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL13 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL13 {
        RegmodL13::from_bits(val)
    }
}
impl From<RegmodL13> for u8 {
    #[inline(always)]
    fn from(val: RegmodL13) -> u8 {
        RegmodL13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL14 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL14 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL14 {
        RegmodL14::from_bits(val)
    }
}
impl From<RegmodL14> for u8 {
    #[inline(always)]
    fn from(val: RegmodL14) -> u8 {
        RegmodL14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL15 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL15 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL15 {
        RegmodL15::from_bits(val)
    }
}
impl From<RegmodL15> for u8 {
    #[inline(always)]
    fn from(val: RegmodL15) -> u8 {
        RegmodL15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL2 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL2 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL2 {
        RegmodL2::from_bits(val)
    }
}
impl From<RegmodL2> for u8 {
    #[inline(always)]
    fn from(val: RegmodL2) -> u8 {
        RegmodL2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL3 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL3 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL3 {
        RegmodL3::from_bits(val)
    }
}
impl From<RegmodL3> for u8 {
    #[inline(always)]
    fn from(val: RegmodL3) -> u8 {
        RegmodL3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL4 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL4 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL4 {
        RegmodL4::from_bits(val)
    }
}
impl From<RegmodL4> for u8 {
    #[inline(always)]
    fn from(val: RegmodL4) -> u8 {
        RegmodL4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL5 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL5 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL5 {
        RegmodL5::from_bits(val)
    }
}
impl From<RegmodL5> for u8 {
    #[inline(always)]
    fn from(val: RegmodL5) -> u8 {
        RegmodL5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL6 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL6 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL6 {
        RegmodL6::from_bits(val)
    }
}
impl From<RegmodL6> for u8 {
    #[inline(always)]
    fn from(val: RegmodL6) -> u8 {
        RegmodL6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL7 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL7 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL7 {
        RegmodL7::from_bits(val)
    }
}
impl From<RegmodL7> for u8 {
    #[inline(always)]
    fn from(val: RegmodL7) -> u8 {
        RegmodL7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL8 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL8 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL8 {
        RegmodL8::from_bits(val)
    }
}
impl From<RegmodL8> for u8 {
    #[inline(always)]
    fn from(val: RegmodL8) -> u8 {
        RegmodL8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL9 {
    #[doc = "Match"]
    MATCH = 0x0,
    #[doc = "Capture"]
    CAPTURE = 0x01,
}
impl RegmodL9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL9 {
    #[inline(always)]
    fn from(val: u8) -> RegmodL9 {
        RegmodL9::from_bits(val)
    }
}
impl From<RegmodL9> for u8 {
    #[inline(always)]
    fn from(val: RegmodL9) -> u8 {
        RegmodL9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr0 {
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
pub enum Setclr2 {
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Not dependent on the direction of any counter"]
    INDEPENDENT = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down"]
    L_REVERSED = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)"]
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
    #[doc = "Value of STATEV added to that of STATE (the carry out is ignored)"]
    ADD = 0x0,
    #[doc = "Value of STATEV loaded into that of STATE"]
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
    #[doc = "Dual counters, COUNTER_L and COUNTER_H"]
    DUAL_COUNTER = 0x0,
    #[doc = "Unified counter"]
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
