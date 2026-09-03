#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsrScs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC."]
    Sosc = 0x01,
    #[doc = "SIRC."]
    Sirc = 0x02,
    #[doc = "FIRC."]
    Firc = 0x03,
    #[doc = "ROSC."]
    Rosc = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CsrScs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsrScs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsrScs {
    #[inline(always)]
    fn from(val: u8) -> CsrScs {
        CsrScs::from_bits(val)
    }
}
impl From<CsrScs> for u8 {
    #[inline(always)]
    fn from(val: CsrScs) -> u8 {
        CsrScs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erefs {
    #[doc = "External reference clock selected."]
    External = 0x0,
    #[doc = "Internal crystal oscillator of OSC selected."]
    Internal = 0x01,
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
    NotEnabledOrNotValid = 0x0,
    #[doc = "FIRC is enabled and output clock is accurate after some preparation time which is obtained by counting FRO_HF clock."]
    EnabledAndValid = 0x01,
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
    #[doc = "FIRCACC interrupt is not enabled."]
    Fircaccnot = 0x0,
    #[doc = "FIRCACC interrupt is enabled."]
    Fircaccyes = 0x01,
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
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
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
pub enum FirccsrTrimLock {
    #[doc = "FIRC auto trim not locked to target frequency range."]
    FircNotLocked = 0x0,
    #[doc = "FIRC auto trim locked to target frequency range."]
    FircLocked = 0x01,
}
impl FirccsrTrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrTrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrTrimLock {
    #[inline(always)]
    fn from(val: u8) -> FirccsrTrimLock {
        FirccsrTrimLock::from_bits(val)
    }
}
impl From<FirccsrTrimLock> for u8 {
    #[inline(always)]
    fn from(val: FirccsrTrimLock) -> u8 {
        FirccsrTrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircerr {
    #[doc = "Error not detected with the FIRC trimming."]
    ErrorNotDetected = 0x0,
    #[doc = "Error detected with the FIRC trimming."]
    ErrorDetected = 0x01,
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
    #[doc = "FIRCERR interrupt is not enabled."]
    ErrorNotDetected = 0x0,
    #[doc = "FIRCERR interrupt is enabled."]
    ErrorDetected = 0x01,
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
    #[doc = "FIRC is disabled in Deep Sleep mode."]
    DisabledInStopModes = 0x0,
    #[doc = "FIRC is enabled in Deep Sleep mode."]
    EnabledInStopModes = 0x01,
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
pub enum FirctcfgTrimsrc {
    #[doc = "USB0 Start of Frame (1 KHz). This option does not use TRIMDIV."]
    Usb0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    Sosc = 0x02,
    _RESERVED_3 = 0x03,
}
impl FirctcfgTrimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirctcfgTrimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirctcfgTrimsrc {
    #[inline(always)]
    fn from(val: u8) -> FirctcfgTrimsrc {
        FirctcfgTrimsrc::from_bits(val)
    }
}
impl From<FirctcfgTrimsrc> for u8 {
    #[inline(always)]
    fn from(val: FirctcfgTrimsrc) -> u8 {
        FirctcfgTrimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircvld {
    #[doc = "FIRC is not enabled or clock is not valid."]
    NotEnabledOrNotValid = 0x0,
    #[doc = "FIRC is enabled and output clock is valid. The clock is valid after there is an output clock from the FIRC analog."]
    EnabledAndValid = 0x01,
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
    #[doc = "48 MHz FIRC clock selected, divided from 192 MHz."]
    Firc48mHz192s = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "64 MHz FIRC clock selected."]
    Firc64mHz = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "96 MHz FIRC clock selected."]
    Firc96mHz = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "192 MHz FIRC clock selected."]
    Firc192mHz = 0x07,
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
    Enabled = 0x0,
    #[doc = "IFR write access to SCG trim registers during system reset is blocked."]
    Disabled = 0x01,
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
    Freq16to20mHz = 0x0,
    #[doc = "Frequency range select of 16-25 MHz."]
    LowFreq = 0x01,
    #[doc = "Frequency range select of 25-40 MHz."]
    MediumFreq = 0x02,
    #[doc = "Frequency range select of 40-50 MHz."]
    HighFreq = 0x03,
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
pub enum RccrScs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC."]
    Sosc = 0x01,
    #[doc = "SIRC."]
    Sirc = 0x02,
    #[doc = "FIRC."]
    Firc = 0x03,
    #[doc = "ROSC."]
    Rosc = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RccrScs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RccrScs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RccrScs {
    #[inline(always)]
    fn from(val: u8) -> RccrScs {
        RccrScs::from_bits(val)
    }
}
impl From<RccrScs> for u8 {
    #[inline(always)]
    fn from(val: RccrScs) -> u8 {
        RccrScs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RosccsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
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
    #[doc = "ROSC Clock has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "ROSC Clock has detected an error."]
    EnabledAndError = 0x01,
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
    #[doc = "ROSC is not enabled or clock is not valid."]
    DisabledOrNotValid = 0x0,
    #[doc = "ROSC is enabled and output clock is valid."]
    EnabledAndValid = 0x01,
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
pub enum SirccsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
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
pub enum SirccsrTrimLock {
    #[doc = "SIRC auto trim not locked to target frequency range."]
    SircNotLocked = 0x0,
    #[doc = "SIRC auto trim locked to target frequency range."]
    SircLocked = 0x01,
}
impl SirccsrTrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrTrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrTrimLock {
    #[inline(always)]
    fn from(val: u8) -> SirccsrTrimLock {
        SirccsrTrimLock::from_bits(val)
    }
}
impl From<SirccsrTrimLock> for u8 {
    #[inline(always)]
    fn from(val: SirccsrTrimLock) -> u8 {
        SirccsrTrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircerr {
    #[doc = "Error not detected with the SIRC trimming."]
    ErrorNotDetected = 0x0,
    #[doc = "Error detected with the SIRC trimming."]
    ErrorDetected = 0x01,
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
    #[doc = "SIRCERR interrupt is not enabled."]
    ErrorNotDetected = 0x0,
    #[doc = "SIRCERR interrupt is enabled."]
    ErrorDetected = 0x01,
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
pub enum SirctcfgTrimsrc {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    Sosc = 0x02,
    _RESERVED_3 = 0x03,
}
impl SirctcfgTrimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirctcfgTrimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirctcfgTrimsrc {
    #[inline(always)]
    fn from(val: u8) -> SirctcfgTrimsrc {
        SirctcfgTrimsrc::from_bits(val)
    }
}
impl From<SirctcfgTrimsrc> for u8 {
    #[inline(always)]
    fn from(val: SirctcfgTrimsrc) -> u8 {
        SirctcfgTrimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircvld {
    #[doc = "SIRC is not enabled or clock is not valid."]
    DisabledOrNotValid = 0x0,
    #[doc = "SIRC is enabled and output clock is valid."]
    EnabledAndValid = 0x01,
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
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
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
    #[doc = "This Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "This Control Status Register cannot be written."]
    WriteDisabled = 0x01,
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
    #[doc = "SOSC Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "SOSC Clock Monitor is enabled and detected an error."]
    EnabledAndError = 0x01,
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
pub enum TrimUnlock {
    #[doc = "SCG Trim Registers locked and not writable."]
    Locked = 0x0,
    #[doc = "SCG Trim registers unlocked and writable."]
    NotLocked = 0x01,
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
