#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlmMatch {
    #[doc = "Only seconds, minutes, and hours matched."]
    Hours = 0x0,
    #[doc = "Only seconds, minutes, hours, and days matched."]
    Days = 0x01,
    #[doc = "Only seconds, minutes, hours, days, and months matched."]
    Months = 0x02,
    #[doc = "Only seconds, minutes, hours, days, months, and year (offset) matched."]
    Year = 0x03,
}
impl AlmMatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlmMatch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlmMatch {
    #[inline(always)]
    fn from(val: u8) -> AlmMatch {
        AlmMatch::from_bits(val)
    }
}
impl From<AlmMatch> for u8 {
    #[inline(always)]
    fn from(val: AlmMatch) -> u8 {
        AlmMatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusErr {
    #[doc = "Read and write accesses are normal."]
    Normal = 0x0,
    #[doc = "Read or write accesses occurred when STATUS\\[INVAL_BIT\\] was asserted."]
    Asserted = 0x01,
}
impl BusErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusErr {
    #[inline(always)]
    fn from(val: u8) -> BusErr {
        BusErr::from_bits(val)
    }
}
impl From<BusErr> for u8 {
    #[inline(always)]
    fn from(val: BusErr) -> u8 {
        BusErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel {
    #[doc = "16.384 kHz clock is selected."]
    Clk16384khz = 0x0,
    #[doc = "32.768 kHz clock is selected."]
    Clk32768khz = 0x01,
}
impl ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel {
    #[inline(always)]
    fn from(val: u8) -> ClkSel {
        ClkSel::from_bits(val)
    }
}
impl From<ClkSel> for u8 {
    #[inline(always)]
    fn from(val: ClkSel) -> u8 {
        ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoDis {
    #[doc = "The selected clock is output to other peripherals."]
    ToOtherPeripherals = 0x0,
    #[doc = "The selected clock is not output to other peripherals."]
    NotToOtherPeripherals = 0x01,
}
impl ClkoDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoDis {
    #[inline(always)]
    fn from(val: u8) -> ClkoDis {
        ClkoDis::from_bits(val)
    }
}
impl From<ClkoDis> for u8 {
    #[inline(always)]
    fn from(val: ClkoDis) -> u8 {
        ClkoDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkout {
    #[doc = "No output clock."]
    NoOutputClk = 0x0,
    #[doc = "Fine 1 Hz clock with both precise edges."]
    Clk1hzFine = 0x01,
    #[doc = "32.768 or 16.384 kHz clock."]
    Clk32168khz = 0x02,
    #[doc = "Coarse 1 Hz clock with both precise edges."]
    Clk1hzCoarse = 0x03,
}
impl Clkout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkout {
    #[inline(always)]
    fn from(val: u8) -> Clkout {
        Clkout::from_bits(val)
    }
}
impl From<Clkout> for u8 {
    #[inline(always)]
    fn from(val: Clkout) -> u8 {
        Clkout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dow {
    #[doc = "Sunday."]
    Sunday = 0x0,
    #[doc = "Monday."]
    Monday = 0x01,
    #[doc = "Tuesday."]
    Tuesday = 0x02,
    #[doc = "Wednesday."]
    Wednesday = 0x03,
    #[doc = "Thursday."]
    Thursday = 0x04,
    #[doc = "Friday."]
    Friday = 0x05,
    #[doc = "Saturday."]
    Saturday = 0x06,
    _RESERVED_7 = 0x07,
}
impl Dow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dow {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dow {
    #[inline(always)]
    fn from(val: u8) -> Dow {
        Dow::from_bits(val)
    }
}
impl From<Dow> for u8 {
    #[inline(always)]
    fn from(val: Dow) -> u8 {
        Dow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalBit {
    #[doc = "Time and date counters can be read or written. Time and date is valid."]
    Valid = 0x0,
    #[doc = "Time and date counter values are changing or time and date is invalid and cannot be read or written."]
    Invalid = 0x01,
}
impl InvalBit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InvalBit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InvalBit {
    #[inline(always)]
    fn from(val: u8) -> InvalBit {
        InvalBit::from_bits(val)
    }
}
impl From<InvalBit> for u8 {
    #[inline(always)]
    fn from(val: InvalBit) -> u8 {
        InvalBit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MonCnt {
    #[doc = "Illegal Value."]
    IllegalValue0 = 0x0,
    #[doc = "January."]
    January = 0x01,
    #[doc = "February."]
    February = 0x02,
    #[doc = "March."]
    March = 0x03,
    #[doc = "April."]
    April = 0x04,
    #[doc = "May."]
    May = 0x05,
    #[doc = "June."]
    June = 0x06,
    #[doc = "July."]
    July = 0x07,
    #[doc = "August."]
    August = 0x08,
    #[doc = "September."]
    September = 0x09,
    #[doc = "October."]
    October = 0x0a,
    #[doc = "November."]
    November = 0x0b,
    #[doc = "December."]
    December = 0x0c,
    #[doc = "Illegal Value."]
    IllegalValue13 = 0x0d,
    #[doc = "Illegal Value."]
    IllegalValue14 = 0x0e,
    #[doc = "Illegal Value."]
    IllegalValue15 = 0x0f,
}
impl MonCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MonCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MonCnt {
    #[inline(always)]
    fn from(val: u8) -> MonCnt {
        MonCnt::from_bits(val)
    }
}
impl From<MonCnt> for u8 {
    #[inline(always)]
    fn from(val: MonCnt) -> u8 {
        MonCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "Software Reset cleared."]
    Cleared = 0x0,
    #[doc = "Software Reset asserted."]
    Asserted = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum We {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable Write Protection - Registers are locked."]
    Locked = 0x02,
    _RESERVED_3 = 0x03,
}
impl We {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> We {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for We {
    #[inline(always)]
    fn from(val: u8) -> We {
        We::from_bits(val)
    }
}
impl From<We> for u8 {
    #[inline(always)]
    fn from(val: We) -> u8 {
        We::to_bits(val)
    }
}
