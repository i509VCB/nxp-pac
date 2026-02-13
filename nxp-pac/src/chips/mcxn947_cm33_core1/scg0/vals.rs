#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllLock {
    #[doc = "APLL is not powered on or not locked"]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "APLL is locked"]
    ENABLED_AND_VALID = 0x01,
}
impl ApllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllLock {
    #[inline(always)]
    fn from(val: u8) -> ApllLock {
        ApllLock::from_bits(val)
    }
}
impl From<ApllLock> for u8 {
    #[inline(always)]
    fn from(val: ApllLock) -> u8 {
        ApllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected"]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected"]
    GENERATE_RESET = 0x01,
}
impl Apllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllcmre {
    #[inline(always)]
    fn from(val: u8) -> Apllcmre {
        Apllcmre::from_bits(val)
    }
}
impl From<Apllcmre> for u8 {
    #[inline(always)]
    fn from(val: Apllcmre) -> u8 {
        Apllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllcsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl ApllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> ApllcsrLk {
        ApllcsrLk::from_bits(val)
    }
}
impl From<ApllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: ApllcsrLk) -> u8 {
        ApllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlSource {
    #[doc = "SOSC"]
    SOSC = 0x0,
    #[doc = "FIRC 48 MHz clock. FIRC_SCLK_PERIPH_EN must be set to use FIRC 48 MHz clock."]
    FIRC = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "No clock"]
    RSVD = 0x03,
}
impl ApllctrlSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlSource {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlSource {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlSource {
        ApllctrlSource::from_bits(val)
    }
}
impl From<ApllctrlSource> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlSource) -> u8 {
        ApllctrlSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllerr {
    #[doc = "APLL Clock Monitor is disabled or has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "APLL Clock Monitor is enabled and detected an error"]
    ENABLED_AND_ERROR = 0x01,
}
impl Apllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllerr {
    #[inline(always)]
    fn from(val: u8) -> Apllerr {
        Apllerr::from_bits(val)
    }
}
impl From<Apllerr> for u8 {
    #[inline(always)]
    fn from(val: Apllerr) -> u8 {
        Apllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsscg1Mc {
    #[doc = "MC\\[1:0\\] no compensation"]
    NO_COMP = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "MC\\[1:0\\] maximum compensation"]
    MAX_COMP = 0x03,
}
impl Apllsscg1Mc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1Mc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1Mc {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1Mc {
        Apllsscg1Mc::from_bits(val)
    }
}
impl From<Apllsscg1Mc> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1Mc) -> u8 {
        Apllsscg1Mc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsten {
    #[doc = "APLL is disabled in Deep Sleep mode"]
    DISABLED_IN_STOP = 0x0,
    #[doc = "APLL is enabled in Deep Sleep mode"]
    ENABLED_IN_STOP = 0x01,
}
impl Apllsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsten {
    #[inline(always)]
    fn from(val: u8) -> Apllsten {
        Apllsten::from_bits(val)
    }
}
impl From<Apllsten> for u8 {
    #[inline(always)]
    fn from(val: Apllsten) -> u8 {
        Apllsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsrScs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC"]
    SOSC = 0x01,
    #[doc = "SIRC"]
    SIRC = 0x02,
    #[doc = "FIRC"]
    FIRC = 0x03,
    #[doc = "ROSC"]
    ROSC = 0x04,
    #[doc = "APLL"]
    APLL = 0x05,
    #[doc = "SPLL"]
    SPLL = 0x06,
    #[doc = "UPLL"]
    UPLL = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CsrScs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsrScs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    #[doc = "External reference clock selected. LDO can be disabled in this case."]
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
    #[doc = "FIRC is enabled and output clock is accurate. The clock is accurate after 4096 clock cycles of 144 MHz (RANGE=1) or 1365 clock cycles of 48 MHz(RANGE=0) from the FIRC analog."]
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
pub enum FirccfgRange {
    #[doc = "48 MHz FIRC clock selected"]
    FIRC_48MHZ = 0x0,
    #[doc = "144 MHz FIRC clock selected"]
    FIRC_144MHZ = 0x01,
}
impl FirccfgRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccfgRange {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccfgRange {
    #[inline(always)]
    fn from(val: u8) -> FirccfgRange {
        FirccfgRange::from_bits(val)
    }
}
impl From<FirccfgRange> for u8 {
    #[inline(always)]
    fn from(val: FirccfgRange) -> u8 {
        FirccfgRange::to_bits(val)
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
pub enum FirccsrTrimLock {
    #[doc = "FIRC auto trim not locked to target frequency range"]
    FIRC_NOT_LOCKED = 0x0,
    #[doc = "FIRC auto trim locked to target frequency range"]
    FIRC_LOCKED = 0x01,
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
pub enum FirctcfgTrimsrc {
    #[doc = "USB0 Start of Frame (1 kHz). This option does not use TRIMDIV"]
    USB0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC"]
    SOSC = 0x02,
    #[doc = "ROSC"]
    ROSC = 0x03,
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
pub enum RccrScs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC"]
    SOSC = 0x01,
    #[doc = "SIRC"]
    SIRC = 0x02,
    #[doc = "FIRC"]
    FIRC = 0x03,
    #[doc = "ROSC"]
    ROSC = 0x04,
    #[doc = "APLL"]
    APLL = 0x05,
    #[doc = "SPLL"]
    SPLL = 0x06,
    #[doc = "UPLL"]
    UPLL = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RccrScs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RccrScs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Rosccmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected"]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected"]
    GENERATE_RESET = 0x01,
}
impl Rosccmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rosccmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rosccmre {
    #[inline(always)]
    fn from(val: u8) -> Rosccmre {
        Rosccmre::from_bits(val)
    }
}
impl From<Rosccmre> for u8 {
    #[inline(always)]
    fn from(val: Rosccmre) -> u8 {
        Rosccmre::to_bits(val)
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
    #[doc = "ROSC Clock Monitor is disabled or has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "ROSC Clock Monitor is enabled and detected an RTC loss of clock error"]
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
pub enum SirccsrTrimLock {
    #[doc = "SIRC auto trim not locked to target frequency range"]
    SIRC_NOT_LOCKED = 0x0,
    #[doc = "SIRC auto trim locked to target frequency range"]
    SIRC_LOCKED = 0x01,
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
pub enum SirctcfgTrimsrc {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC"]
    SOSC = 0x02,
    #[doc = "ROSC (32.768 kHz)"]
    ROSC = 0x03,
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
pub enum SosccfgRange {
    #[doc = "Frequency range select of 16-20 MHz."]
    FREQ_16TO20MHZ = 0x0,
    #[doc = "Frequency range select of 20-30 MHz."]
    LOW_FREQ = 0x01,
    #[doc = "Frequency range select of 30-50 MHz."]
    MEDIUM_FREQ = 0x02,
    #[doc = "Frequency range select of 50-66 MHz."]
    HIGH_FREQ = 0x03,
}
impl SosccfgRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SosccfgRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SosccfgRange {
    #[inline(always)]
    fn from(val: u8) -> SosccfgRange {
        SosccfgRange::from_bits(val)
    }
}
impl From<SosccfgRange> for u8 {
    #[inline(always)]
    fn from(val: SosccfgRange) -> u8 {
        SosccfgRange::to_bits(val)
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
pub enum SpllctrlSource {
    #[doc = "SOSC"]
    SOSC = 0x0,
    #[doc = "FIRC 48 MHz clock. FIRC_SCLK_PERIPH_EN must be set to use FIRC 48 MHz clock."]
    FIRC = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "No clock"]
    RSVD = 0x03,
}
impl SpllctrlSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlSource {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlSource {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlSource {
        SpllctrlSource::from_bits(val)
    }
}
impl From<SpllctrlSource> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlSource) -> u8 {
        SpllctrlSource::to_bits(val)
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
pub enum Spllsscg1Mc {
    #[doc = "MC\\[1:0\\] no compensation"]
    NO_COMP = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "MC\\[1:0\\] maximum compensation"]
    MAX_COMP = 0x03,
}
impl Spllsscg1Mc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1Mc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1Mc {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1Mc {
        Spllsscg1Mc::from_bits(val)
    }
}
impl From<Spllsscg1Mc> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1Mc) -> u8 {
        Spllsscg1Mc::to_bits(val)
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
pub enum TrimUnlock {
    #[doc = "SCG Trim registers are locked and not writable."]
    LOCKED = 0x0,
    #[doc = "SCG Trim registers are unlocked and writable."]
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
pub enum Upllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected"]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected"]
    GENERATE_RESET = 0x01,
}
impl Upllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllcmre {
    #[inline(always)]
    fn from(val: u8) -> Upllcmre {
        Upllcmre::from_bits(val)
    }
}
impl From<Upllcmre> for u8 {
    #[inline(always)]
    fn from(val: Upllcmre) -> u8 {
        Upllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UpllcsrLk {
    #[doc = "Control Status Register can be written"]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written"]
    WRITE_DISABLED = 0x01,
}
impl UpllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UpllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UpllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> UpllcsrLk {
        UpllcsrLk::from_bits(val)
    }
}
impl From<UpllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: UpllcsrLk) -> u8 {
        UpllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllerr {
    #[doc = "UPLL Clock Monitor is disabled or has not detected an error"]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "UPLL Clock Monitor is enabled and detected an error"]
    ENABLED_AND_ERROR = 0x01,
}
impl Upllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllerr {
    #[inline(always)]
    fn from(val: u8) -> Upllerr {
        Upllerr::from_bits(val)
    }
}
impl From<Upllerr> for u8 {
    #[inline(always)]
    fn from(val: Upllerr) -> u8 {
        Upllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllvld {
    #[doc = "UPLL is not enabled or clock is not valid"]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "UPLL is enabled and output clock is valid"]
    ENABLED_AND_VALID = 0x01,
}
impl Upllvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllvld {
    #[inline(always)]
    fn from(val: u8) -> Upllvld {
        Upllvld::from_bits(val)
    }
}
impl From<Upllvld> for u8 {
    #[inline(always)]
    fn from(val: Upllvld) -> u8 {
        Upllvld::to_bits(val)
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
