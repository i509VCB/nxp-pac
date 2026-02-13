#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erefs {
    #[doc = "External reference clock selected."]
    EXTERNAL = 0x0,
    #[doc = "Internal crystal oscillator of OSC selected."]
    INTERNAL = 0x01,
}
impl Erefs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erefs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erefs {
    #[inline(always)]
    fn from(val: u8) -> Erefs {
        Erefs::from_bits(val)
    }
}
impl From<Erefs> for u8 {
    #[inline(always)]
    fn from(val: Erefs) -> u8 {
        Erefs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircacc {
    #[doc = "FIRC is not enabled or clock is not accurate."]
    NOT_ENABLED_OR_NOT_VALID = 0x0,
    #[doc = "FIRC is enabled and output clock is accurate after some preparation time which is obtained by counting FRO_HF clock."]
    ENABLED_AND_VALID = 0x01,
}
impl Fircacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircacc {
    #[inline(always)]
    fn from(val: u8) -> Fircacc {
        Fircacc::from_bits(val)
    }
}
impl From<Fircacc> for u8 {
    #[inline(always)]
    fn from(val: Fircacc) -> u8 {
        Fircacc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircaccIe {
    #[doc = "FIRCACC interrupt is not enabled"]
    FIRCACCNOT = 0x0,
    #[doc = "FIRCACC interrupt is enabled"]
    FIRCACCYES = 0x01,
}
impl FircaccIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircaccIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircaccIe {
    #[inline(always)]
    fn from(val: u8) -> FircaccIe {
        FircaccIe::from_bits(val)
    }
}
impl From<FircaccIe> for u8 {
    #[inline(always)]
    fn from(val: FircaccIe) -> u8 {
        FircaccIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirccsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl FirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> FirccsrLk {
        FirccsrLk::from_bits(val)
    }
}
impl From<FirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: FirccsrLk) -> u8 {
        FirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircerr {
    #[doc = "Error not detected with the FIRC trimming"]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "Error detected with the FIRC trimming"]
    ERROR_DETECTED = 0x01,
}
impl Fircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircerr {
    #[inline(always)]
    fn from(val: u8) -> Fircerr {
        Fircerr::from_bits(val)
    }
}
impl From<Fircerr> for u8 {
    #[inline(always)]
    fn from(val: Fircerr) -> u8 {
        Fircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircerrIe {
    #[doc = "FIRCERR interrupt is not enabled"]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "FIRCERR interrupt is enabled"]
    ERROR_DETECTED = 0x01,
}
impl FircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircerrIe {
    #[inline(always)]
    fn from(val: u8) -> FircerrIe {
        FircerrIe::from_bits(val)
    }
}
impl From<FircerrIe> for u8 {
    #[inline(always)]
    fn from(val: FircerrIe) -> u8 {
        FircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircsten {
    #[doc = "FIRC is disabled in Deep Sleep mode"]
    DISABLED_IN_STOP_MODES = 0x0,
    #[doc = "FIRC is enabled in Deep Sleep mode"]
    ENABLED_IN_STOP_MODES = 0x01,
}
impl Fircsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircsten {
    #[inline(always)]
    fn from(val: u8) -> Fircsten {
        Fircsten::from_bits(val)
    }
}
impl From<Fircsten> for u8 {
    #[inline(always)]
    fn from(val: Fircsten) -> u8 {
        Fircsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircvld {
    #[doc = "FIRC is not enabled or clock is not valid."]
    NOT_ENABLED_OR_NOT_VALID = 0x0,
    #[doc = "FIRC is enabled and output clock is valid. The clock is valid after there is an output clock from the FIRC analog."]
    ENABLED_AND_VALID = 0x01,
}
impl Fircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircvld {
    #[inline(always)]
    fn from(val: u8) -> Fircvld {
        Fircvld::from_bits(val)
    }
}
impl From<Fircvld> for u8 {
    #[inline(always)]
    fn from(val: Fircvld) -> u8 {
        Fircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqSel {
    _RESERVED_0 = 0x0,
    #[doc = "45 MHz FIRC clock selected, divided from 180 MHz"]
    FIRC_48MHZ_192S = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "60 MHz FIRC clock selected"]
    FIRC_64MHZ = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "90 MHz FIRC clock selected"]
    FIRC_96MHZ = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "180 MHz FIRC clock selected"]
    FIRC_192MHZ = 0x07,
}
impl FreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqSel {
    #[inline(always)]
    fn from(val: u8) -> FreqSel {
        FreqSel::from_bits(val)
    }
}
impl From<FreqSel> for u8 {
    #[inline(always)]
    fn from(val: FreqSel) -> u8 {
        FreqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrDisable {
    #[doc = "IFR write access to SCG trim registers not disabled. The SCG Trim registers are reprogrammed with the IFR values after any system reset."]
    ENABLED = 0x0,
    #[doc = "IFR write access to SCG trim registers during system reset is blocked."]
    DISABLED = 0x01,
}
impl IfrDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrDisable {
    #[inline(always)]
    fn from(val: u8) -> IfrDisable {
        IfrDisable::from_bits(val)
    }
}
impl From<IfrDisable> for u8 {
    #[inline(always)]
    fn from(val: IfrDisable) -> u8 {
        IfrDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "Frequency range select of 8-16 MHz."]
    FREQ_16TO20MHZ = 0x0,
    #[doc = "Frequency range select of 16-25 MHz."]
    LOW_FREQ = 0x01,
    #[doc = "Frequency range select of 25-40 MHz."]
    MEDIUM_FREQ = 0x02,
    #[doc = "Frequency range select of 40-50 MHz."]
    HIGH_FREQ = 0x03,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RosccsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl RosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> RosccsrLk {
        RosccsrLk::from_bits(val)
    }
}
impl From<RosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: RosccsrLk) -> u8 {
        RosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscerr {
    #[doc = "ROSC Clock has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "ROSC Clock has detected an error"]
    ENABLED_AND_ERROR = 0x01,
}
impl Roscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscerr {
    #[inline(always)]
    fn from(val: u8) -> Roscerr {
        Roscerr::from_bits(val)
    }
}
impl From<Roscerr> for u8 {
    #[inline(always)]
    fn from(val: Roscerr) -> u8 {
        Roscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscvld {
    #[doc = "ROSC is not enabled or clock is not valid"]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "ROSC is enabled and output clock is valid"]
    ENABLED_AND_VALID = 0x01,
}
impl Roscvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscvld {
    #[inline(always)]
    fn from(val: u8) -> Roscvld {
        Roscvld::from_bits(val)
    }
}
impl From<Roscvld> for u8 {
    #[inline(always)]
    fn from(val: Roscvld) -> u8 {
        Roscvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC"]
    SOSC = 0x01,
    #[doc = "SIRC"]
    SIRC = 0x02,
    #[doc = "FIRC"]
    FIRC = 0x03,
    #[doc = "ROSC"]
    ROSC = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SPLL"]
    SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scs {
    #[inline(always)]
    fn from(val: u8) -> Scs {
        Scs::from_bits(val)
    }
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(val: Scs) -> u8 {
        Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirccsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl SirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SirccsrLk {
        SirccsrLk::from_bits(val)
    }
}
impl From<SirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SirccsrLk) -> u8 {
        SirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircerr {
    #[doc = "Error not detected with the SIRC trimming"]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "Error detected with the SIRC trimming"]
    ERROR_DETECTED = 0x01,
}
impl Sircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircerr {
    #[inline(always)]
    fn from(val: u8) -> Sircerr {
        Sircerr::from_bits(val)
    }
}
impl From<Sircerr> for u8 {
    #[inline(always)]
    fn from(val: Sircerr) -> u8 {
        Sircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SircerrIe {
    #[doc = "SIRCERR interrupt is not enabled"]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "SIRCERR interrupt is enabled"]
    ERROR_DETECTED = 0x01,
}
impl SircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SircerrIe {
    #[inline(always)]
    fn from(val: u8) -> SircerrIe {
        SircerrIe::from_bits(val)
    }
}
impl From<SircerrIe> for u8 {
    #[inline(always)]
    fn from(val: SircerrIe) -> u8 {
        SircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircvld {
    #[doc = "SIRC is not enabled or clock is not valid"]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "SIRC is enabled and output clock is valid"]
    ENABLED_AND_VALID = 0x01,
}
impl Sircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircvld {
    #[inline(always)]
    fn from(val: u8) -> Sircvld {
        Sircvld::from_bits(val)
    }
}
impl From<Sircvld> for u8 {
    #[inline(always)]
    fn from(val: Sircvld) -> u8 {
        Sircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sosccmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected"]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected"]
    GENERATE_RESET = 0x01,
}
impl Sosccmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sosccmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sosccmre {
    #[inline(always)]
    fn from(val: u8) -> Sosccmre {
        Sosccmre::from_bits(val)
    }
}
impl From<Sosccmre> for u8 {
    #[inline(always)]
    fn from(val: Sosccmre) -> u8 {
        Sosccmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SosccsrLk {
    #[doc = "This Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "This Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl SosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SosccsrLk {
        SosccsrLk::from_bits(val)
    }
}
impl From<SosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SosccsrLk) -> u8 {
        SosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soscerr {
    #[doc = "SOSC Clock Monitor is disabled or has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "SOSC Clock Monitor is enabled and detected an error"]
    ENABLED_AND_ERROR = 0x01,
}
impl Soscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscerr {
    #[inline(always)]
    fn from(val: u8) -> Soscerr {
        Soscerr::from_bits(val)
    }
}
impl From<Soscerr> for u8 {
    #[inline(always)]
    fn from(val: Soscerr) -> u8 {
        Soscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    #[doc = "SOSC"]
    SOSC = 0x0,
    #[doc = "FIRC 45 MHz clock. FIRC_SCLK_PERIPH_EN needs to be set to use FIRC 45 MHz clock."]
    FIRC = 0x01,
    #[doc = "ROSC"]
    RSVD = 0x02,
    #[doc = "SIRC 12 MHz clock"]
    SIRC = 0x03,
}
impl Source {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Source {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Source {
    #[inline(always)]
    fn from(val: u8) -> Source {
        Source::from_bits(val)
    }
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(val: Source) -> u8 {
        Source::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllLock {
    #[doc = "SPLL is not powered on or not locked"]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "SPLL is locked"]
    ENABLED_AND_VALID = 0x01,
}
impl SpllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllLock {
    #[inline(always)]
    fn from(val: u8) -> SpllLock {
        SpllLock::from_bits(val)
    }
}
impl From<SpllLock> for u8 {
    #[inline(always)]
    fn from(val: SpllLock) -> u8 {
        SpllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected"]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected"]
    GENERATE_RESET = 0x01,
}
impl Spllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllcmre {
    #[inline(always)]
    fn from(val: u8) -> Spllcmre {
        Spllcmre::from_bits(val)
    }
}
impl From<Spllcmre> for u8 {
    #[inline(always)]
    fn from(val: Spllcmre) -> u8 {
        Spllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllcsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl SpllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> SpllcsrLk {
        SpllcsrLk::from_bits(val)
    }
}
impl From<SpllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: SpllcsrLk) -> u8 {
        SpllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllerr {
    #[doc = "SPLL Clock Monitor is disabled or has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "SPLL Clock Monitor is enabled and detected an error"]
    ENABLED_AND_ERROR = 0x01,
}
impl Spllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllerr {
    #[inline(always)]
    fn from(val: u8) -> Spllerr {
        Spllerr::from_bits(val)
    }
}
impl From<Spllerr> for u8 {
    #[inline(always)]
    fn from(val: Spllerr) -> u8 {
        Spllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsten {
    #[doc = "SPLL is disabled in Deep Sleep mode"]
    DISABLED_IN_STOP = 0x0,
    #[doc = "SPLL is enabled in Deep Sleep mode"]
    ENABLED_IN_STOP = 0x01,
}
impl Spllsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsten {
    #[inline(always)]
    fn from(val: u8) -> Spllsten {
        Spllsten::from_bits(val)
    }
}
impl From<Spllsten> for u8 {
    #[inline(always)]
    fn from(val: Spllsten) -> u8 {
        Spllsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimLock {
    #[doc = "SIRC auto trim not locked to target frequency range"]
    SIRC_NOT_LOCKED = 0x0,
    #[doc = "SIRC auto trim locked to target frequency range"]
    SIRC_LOCKED = 0x01,
}
impl TrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimLock {
    #[inline(always)]
    fn from(val: u8) -> TrimLock {
        TrimLock::from_bits(val)
    }
}
impl From<TrimLock> for u8 {
    #[inline(always)]
    fn from(val: TrimLock) -> u8 {
        TrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimUnlock {
    #[doc = "SCG Trim Registers locked and not writable."]
    LOCKED = 0x0,
    #[doc = "SCG Trim registers unlocked and writable."]
    NOT_LOCKED = 0x01,
}
impl TrimUnlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimUnlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimUnlock {
    #[inline(always)]
    fn from(val: u8) -> TrimUnlock {
        TrimUnlock::from_bits(val)
    }
}
impl From<TrimUnlock> for u8 {
    #[inline(always)]
    fn from(val: TrimUnlock) -> u8 {
        TrimUnlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trimsrc {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    SOSC = 0x02,
    _RESERVED_3 = 0x03,
}
impl Trimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trimsrc {
    #[inline(always)]
    fn from(val: u8) -> Trimsrc {
        Trimsrc::from_bits(val)
    }
}
impl From<Trimsrc> for u8 {
    #[inline(always)]
    fn from(val: Trimsrc) -> u8 {
        Trimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoutSel {
    #[doc = "VOUT = 1V"]
    VOUT_1V_1 = 0x0,
    #[doc = "VOUT = 1V"]
    VOUT_1V_2 = 0x01,
    #[doc = "VOUT = 1V"]
    VOUT_1V_3 = 0x02,
    #[doc = "VOUT = 1.05V"]
    VOUT_105V = 0x03,
    #[doc = "VOUT = 1.1V"]
    VOUT_11V = 0x04,
    #[doc = "VOUT = 1.15V"]
    VOUT_115V = 0x05,
    #[doc = "VOUT = 1.2V"]
    VOUT_12V = 0x06,
    #[doc = "VOUT = 1.25V"]
    VOUT_125V = 0x07,
}
impl VoutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoutSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoutSel {
    #[inline(always)]
    fn from(val: u8) -> VoutSel {
        VoutSel::from_bits(val)
    }
}
impl From<VoutSel> for u8 {
    #[inline(always)]
    fn from(val: VoutSel) -> u8 {
        VoutSel::to_bits(val)
    }
}
