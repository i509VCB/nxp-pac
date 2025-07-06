#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlmIe {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl AlmIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlmIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlmIe {
    #[inline(always)]
    fn from(val: u8) -> AlmIe {
        AlmIe::from_bits(val)
    }
}
impl From<AlmIe> for u8 {
    #[inline(always)]
    fn from(val: AlmIe) -> u8 {
        AlmIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlmIs {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl AlmIs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlmIs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlmIs {
    #[inline(always)]
    fn from(val: u8) -> AlmIs {
        AlmIs::from_bits(val)
    }
}
impl From<AlmIs> for u8 {
    #[inline(always)]
    fn from(val: AlmIs) -> u8 {
        AlmIs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlmMatch {
    #[doc = "Only seconds, minutes, and hours matched."]
    HOURS = 0x0,
    #[doc = "Only seconds, minutes, hours, and days matched."]
    DAYS = 0x01,
    #[doc = "Only seconds, minutes, hours, days, and months matched."]
    MONTHS = 0x02,
    #[doc = "Only seconds, minutes, hours, days, months, and year (offset) matched."]
    YEAR = 0x03,
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
    NORMAL = 0x0,
    #[doc = "Read or write accesses occurred when STATUS\\[INVAL_BIT\\] was asserted."]
    ASSERTED = 0x01,
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
    #[doc = "16.384 kHz clock is selected"]
    CLK_16_384KHZ = 0x0,
    #[doc = "32.768 kHz clock is selected"]
    CLK_32_768KHZ = 0x01,
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
    TO_OTHER_PERIPHERALS = 0x0,
    #[doc = "The selected clock is not output to other peripherals."]
    NOT_TO_OTHER_PERIPHERALS = 0x01,
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
    #[doc = "No output clock"]
    NO_OUTPUT_CLK = 0x0,
    #[doc = "Fine 1 Hz clock with both precise edges"]
    CLK_1HZ_FINE = 0x01,
    #[doc = "32.768 or 16.384 kHz clock"]
    CLK_32_168KHZ = 0x02,
    #[doc = "Coarse 1 Hz clock with both precise edges"]
    CLK_1HZ_COARSE = 0x03,
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
pub enum CmpDone {
    #[doc = "Compensation busy or not enabled"]
    DISABLED = 0x0,
    #[doc = "Compensation completed"]
    ENABLED = 0x01,
}
impl CmpDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpDone {
    #[inline(always)]
    fn from(val: u8) -> CmpDone {
        CmpDone::from_bits(val)
    }
}
impl From<CmpDone> for u8 {
    #[inline(always)]
    fn from(val: CmpDone) -> u8 {
        CmpDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CompEn {
    #[doc = "Coarse compensation is disabled."]
    DISABLED = 0x0,
    #[doc = "Coarse compensation is enabled."]
    ENABLED = 0x01,
}
impl CompEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompEn {
    #[inline(always)]
    fn from(val: u8) -> CompEn {
        CompEn::from_bits(val)
    }
}
impl From<CompEn> for u8 {
    #[inline(always)]
    fn from(val: CompEn) -> u8 {
        CompEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DayIe {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl DayIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DayIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DayIe {
    #[inline(always)]
    fn from(val: u8) -> DayIe {
        DayIe::from_bits(val)
    }
}
impl From<DayIe> for u8 {
    #[inline(always)]
    fn from(val: DayIe) -> u8 {
        DayIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DayIs {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl DayIs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DayIs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DayIs {
    #[inline(always)]
    fn from(val: u8) -> DayIs {
        DayIs::from_bits(val)
    }
}
impl From<DayIs> for u8 {
    #[inline(always)]
    fn from(val: DayIs) -> u8 {
        DayIs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dow {
    #[doc = "Sunday"]
    SUNDAY = 0x0,
    #[doc = "Monday"]
    MONDAY = 0x01,
    #[doc = "Tuesday"]
    TUESDAY = 0x02,
    #[doc = "Wednesday"]
    WEDNESDAY = 0x03,
    #[doc = "Thursday"]
    THURSDAY = 0x04,
    #[doc = "Friday"]
    FRIDAY = 0x05,
    #[doc = "Saturday"]
    SATURDAY = 0x06,
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
pub enum DstEn {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl DstEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DstEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DstEn {
    #[inline(always)]
    fn from(val: u8) -> DstEn {
        DstEn::from_bits(val)
    }
}
impl From<DstEn> for u8 {
    #[inline(always)]
    fn from(val: DstEn) -> u8 {
        DstEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fineen {
    #[doc = "Fine compensation is disabled"]
    DISABLED = 0x0,
    #[doc = "Fine compensation is enabled."]
    ENABLED = 0x01,
}
impl Fineen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fineen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fineen {
    #[inline(always)]
    fn from(val: u8) -> Fineen {
        Fineen::from_bits(val)
    }
}
impl From<Fineen> for u8 {
    #[inline(always)]
    fn from(val: Fineen) -> u8 {
        Fineen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HourIe {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl HourIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HourIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HourIe {
    #[inline(always)]
    fn from(val: u8) -> HourIe {
        HourIe::from_bits(val)
    }
}
impl From<HourIe> for u8 {
    #[inline(always)]
    fn from(val: HourIe) -> u8 {
        HourIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HourIs {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl HourIs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HourIs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HourIs {
    #[inline(always)]
    fn from(val: u8) -> HourIs {
        HourIs::from_bits(val)
    }
}
impl From<HourIs> for u8 {
    #[inline(always)]
    fn from(val: HourIs) -> u8 {
        HourIs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie128hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie128hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie128hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie128hz {
    #[inline(always)]
    fn from(val: u8) -> Ie128hz {
        Ie128hz::from_bits(val)
    }
}
impl From<Ie128hz> for u8 {
    #[inline(always)]
    fn from(val: Ie128hz) -> u8 {
        Ie128hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie16hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie16hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie16hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie16hz {
    #[inline(always)]
    fn from(val: u8) -> Ie16hz {
        Ie16hz::from_bits(val)
    }
}
impl From<Ie16hz> for u8 {
    #[inline(always)]
    fn from(val: Ie16hz) -> u8 {
        Ie16hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie1hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie1hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie1hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie1hz {
    #[inline(always)]
    fn from(val: u8) -> Ie1hz {
        Ie1hz::from_bits(val)
    }
}
impl From<Ie1hz> for u8 {
    #[inline(always)]
    fn from(val: Ie1hz) -> u8 {
        Ie1hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie256hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie256hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie256hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie256hz {
    #[inline(always)]
    fn from(val: u8) -> Ie256hz {
        Ie256hz::from_bits(val)
    }
}
impl From<Ie256hz> for u8 {
    #[inline(always)]
    fn from(val: Ie256hz) -> u8 {
        Ie256hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie2hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie2hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie2hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie2hz {
    #[inline(always)]
    fn from(val: u8) -> Ie2hz {
        Ie2hz::from_bits(val)
    }
}
impl From<Ie2hz> for u8 {
    #[inline(always)]
    fn from(val: Ie2hz) -> u8 {
        Ie2hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie32hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie32hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie32hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie32hz {
    #[inline(always)]
    fn from(val: u8) -> Ie32hz {
        Ie32hz::from_bits(val)
    }
}
impl From<Ie32hz> for u8 {
    #[inline(always)]
    fn from(val: Ie32hz) -> u8 {
        Ie32hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie4hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie4hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie4hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie4hz {
    #[inline(always)]
    fn from(val: u8) -> Ie4hz {
        Ie4hz::from_bits(val)
    }
}
impl From<Ie4hz> for u8 {
    #[inline(always)]
    fn from(val: Ie4hz) -> u8 {
        Ie4hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie512hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie512hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie512hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie512hz {
    #[inline(always)]
    fn from(val: u8) -> Ie512hz {
        Ie512hz::from_bits(val)
    }
}
impl From<Ie512hz> for u8 {
    #[inline(always)]
    fn from(val: Ie512hz) -> u8 {
        Ie512hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie64hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie64hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie64hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie64hz {
    #[inline(always)]
    fn from(val: u8) -> Ie64hz {
        Ie64hz::from_bits(val)
    }
}
impl From<Ie64hz> for u8 {
    #[inline(always)]
    fn from(val: Ie64hz) -> u8 {
        Ie64hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie8hz {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl Ie8hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie8hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie8hz {
    #[inline(always)]
    fn from(val: u8) -> Ie8hz {
        Ie8hz::from_bits(val)
    }
}
impl From<Ie8hz> for u8 {
    #[inline(always)]
    fn from(val: Ie8hz) -> u8 {
        Ie8hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalBit {
    #[doc = "Time and date counters can be read or written. Time and date is valid."]
    VALID = 0x0,
    #[doc = "Time and date counter values are changing or time and date is invalid and cannot be read or written."]
    INVALID = 0x01,
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
pub enum Is128hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is128hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is128hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is128hz {
    #[inline(always)]
    fn from(val: u8) -> Is128hz {
        Is128hz::from_bits(val)
    }
}
impl From<Is128hz> for u8 {
    #[inline(always)]
    fn from(val: Is128hz) -> u8 {
        Is128hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is16hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is16hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is16hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is16hz {
    #[inline(always)]
    fn from(val: u8) -> Is16hz {
        Is16hz::from_bits(val)
    }
}
impl From<Is16hz> for u8 {
    #[inline(always)]
    fn from(val: Is16hz) -> u8 {
        Is16hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is1hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is1hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is1hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is1hz {
    #[inline(always)]
    fn from(val: u8) -> Is1hz {
        Is1hz::from_bits(val)
    }
}
impl From<Is1hz> for u8 {
    #[inline(always)]
    fn from(val: Is1hz) -> u8 {
        Is1hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is256hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is256hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is256hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is256hz {
    #[inline(always)]
    fn from(val: u8) -> Is256hz {
        Is256hz::from_bits(val)
    }
}
impl From<Is256hz> for u8 {
    #[inline(always)]
    fn from(val: Is256hz) -> u8 {
        Is256hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is2hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is2hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is2hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is2hz {
    #[inline(always)]
    fn from(val: u8) -> Is2hz {
        Is2hz::from_bits(val)
    }
}
impl From<Is2hz> for u8 {
    #[inline(always)]
    fn from(val: Is2hz) -> u8 {
        Is2hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is32hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is32hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is32hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is32hz {
    #[inline(always)]
    fn from(val: u8) -> Is32hz {
        Is32hz::from_bits(val)
    }
}
impl From<Is32hz> for u8 {
    #[inline(always)]
    fn from(val: Is32hz) -> u8 {
        Is32hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is4hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is4hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is4hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is4hz {
    #[inline(always)]
    fn from(val: u8) -> Is4hz {
        Is4hz::from_bits(val)
    }
}
impl From<Is4hz> for u8 {
    #[inline(always)]
    fn from(val: Is4hz) -> u8 {
        Is4hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is512hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is512hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is512hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is512hz {
    #[inline(always)]
    fn from(val: u8) -> Is512hz {
        Is512hz::from_bits(val)
    }
}
impl From<Is512hz> for u8 {
    #[inline(always)]
    fn from(val: Is512hz) -> u8 {
        Is512hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is64hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is64hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is64hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is64hz {
    #[inline(always)]
    fn from(val: u8) -> Is64hz {
        Is64hz::from_bits(val)
    }
}
impl From<Is64hz> for u8 {
    #[inline(always)]
    fn from(val: Is64hz) -> u8 {
        Is64hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is8hz {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl Is8hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is8hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is8hz {
    #[inline(always)]
    fn from(val: u8) -> Is8hz {
        Is8hz::from_bits(val)
    }
}
impl From<Is8hz> for u8 {
    #[inline(always)]
    fn from(val: Is8hz) -> u8 {
        Is8hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinIe {
    #[doc = "Interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Interrupt is enabled."]
    ENABLED = 0x01,
}
impl MinIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MinIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MinIe {
    #[inline(always)]
    fn from(val: u8) -> MinIe {
        MinIe::from_bits(val)
    }
}
impl From<MinIe> for u8 {
    #[inline(always)]
    fn from(val: MinIe) -> u8 {
        MinIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinIs {
    #[doc = "Interrupt is de-asserted."]
    DEASSERTED = 0x0,
    #[doc = "Interrupt is asserted."]
    ASSERTED = 0x01,
}
impl MinIs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MinIs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MinIs {
    #[inline(always)]
    fn from(val: u8) -> MinIs {
        MinIs::from_bits(val)
    }
}
impl From<MinIs> for u8 {
    #[inline(always)]
    fn from(val: MinIs) -> u8 {
        MinIs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MonCnt {
    #[doc = "Illegal Value"]
    ILLEGAL_VALUE_0 = 0x0,
    #[doc = "January"]
    JANUARY = 0x01,
    #[doc = "February"]
    FEBRUARY = 0x02,
    #[doc = "March"]
    MARCH = 0x03,
    #[doc = "April"]
    APRIL = 0x04,
    #[doc = "May"]
    MAY = 0x05,
    #[doc = "June"]
    JUNE = 0x06,
    #[doc = "July"]
    JULY = 0x07,
    #[doc = "August"]
    AUGUST = 0x08,
    #[doc = "September"]
    SEPTEMBER = 0x09,
    #[doc = "October"]
    OCTOBER = 0x0a,
    #[doc = "November"]
    NOVEMBER = 0x0b,
    #[doc = "December"]
    DECEMBER = 0x0c,
    #[doc = "Illegal Value"]
    ILLEGAL_VALUE_13 = 0x0d,
    #[doc = "Illegal Value"]
    ILLEGAL_VALUE_14 = 0x0e,
    #[doc = "Illegal Value"]
    ILLEGAL_VALUE_15 = 0x0f,
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
    #[doc = "Software Reset cleared"]
    CLEARED = 0x0,
    #[doc = "Software Reset asserted"]
    ASSERTED = 0x01,
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
    LOCKED = 0x02,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteProtEn {
    #[doc = "Registers are unlocked and can be accessed."]
    UNLOCKED = 0x0,
    #[doc = "Registers are locked and in read-only mode."]
    LOCKED = 0x01,
}
impl WriteProtEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WriteProtEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WriteProtEn {
    #[inline(always)]
    fn from(val: u8) -> WriteProtEn {
        WriteProtEn::from_bits(val)
    }
}
impl From<WriteProtEn> for u8 {
    #[inline(always)]
    fn from(val: WriteProtEn) -> u8 {
        WriteProtEn::to_bits(val)
    }
}
