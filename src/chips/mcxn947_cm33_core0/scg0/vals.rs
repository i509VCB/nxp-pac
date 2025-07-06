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
pub enum ApllLockIe {
    #[doc = "APLL_LOCK interrupt is not enabled"]
    NOT_APLL = 0x0,
    #[doc = "APLL_LOCK interrupt is enabled"]
    APLL = 0x01,
}
impl ApllLockIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllLockIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllLockIe {
    #[inline(always)]
    fn from(val: u8) -> ApllLockIe {
        ApllLockIe::from_bits(val)
    }
}
impl From<ApllLockIe> for u8 {
    #[inline(always)]
    fn from(val: ApllLockIe) -> u8 {
        ApllLockIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllOvrdEn {
    #[doc = "APLL override is disabled"]
    DISABLED = 0x0,
    #[doc = "APLL override is enabled"]
    ENABLED = 0x01,
}
impl ApllOvrdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllOvrdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllOvrdEn {
    #[inline(always)]
    fn from(val: u8) -> ApllOvrdEn {
        ApllOvrdEn::from_bits(val)
    }
}
impl From<ApllOvrdEn> for u8 {
    #[inline(always)]
    fn from(val: ApllOvrdEn) -> u8 {
        ApllOvrdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllclken {
    #[doc = "APLL clock is disabled"]
    DISABLED = 0x0,
    #[doc = "APLL clock is enabled"]
    ENABLED = 0x01,
}
impl Apllclken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllclken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllclken {
    #[inline(always)]
    fn from(val: u8) -> Apllclken {
        Apllclken::from_bits(val)
    }
}
impl From<Apllclken> for u8 {
    #[inline(always)]
    fn from(val: Apllclken) -> u8 {
        Apllclken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllclkenOvrd {
    #[doc = "APLL clock is disabled"]
    DISABLED = 0x0,
    #[doc = "APLL clock is enabled"]
    ENABLED = 0x01,
}
impl ApllclkenOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllclkenOvrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllclkenOvrd {
    #[inline(always)]
    fn from(val: u8) -> ApllclkenOvrd {
        ApllclkenOvrd::from_bits(val)
    }
}
impl From<ApllclkenOvrd> for u8 {
    #[inline(always)]
    fn from(val: ApllclkenOvrd) -> u8 {
        ApllclkenOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllclkpres {
    #[doc = "APLL clock source is not present"]
    NOPRES = 0x0,
    #[doc = "APLL clock source is present"]
    PRES = 0x01,
}
impl Apllclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllclkpres {
    #[inline(always)]
    fn from(val: u8) -> Apllclkpres {
        Apllclkpres::from_bits(val)
    }
}
impl From<Apllclkpres> for u8 {
    #[inline(always)]
    fn from(val: Apllclkpres) -> u8 {
        Apllclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllcm {
    #[doc = "APLL Clock Monitor is disabled"]
    DISABLED = 0x0,
    #[doc = "APLL Clock Monitor is enabled"]
    ENABLED = 0x01,
}
impl Apllcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllcm {
    #[inline(always)]
    fn from(val: u8) -> Apllcm {
        Apllcm::from_bits(val)
    }
}
impl From<Apllcm> for u8 {
    #[inline(always)]
    fn from(val: Apllcm) -> u8 {
        Apllcm::to_bits(val)
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
pub enum ApllctrlBanddirect {
    #[doc = "The bandwidth is changed synchronously with the feedback-divider"]
    DISABLED = 0x0,
    #[doc = "Modifies the bandwidth of the PLL directly"]
    ENABLED = 0x01,
}
impl ApllctrlBanddirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlBanddirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlBanddirect {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlBanddirect {
        ApllctrlBanddirect::from_bits(val)
    }
}
impl From<ApllctrlBanddirect> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlBanddirect) -> u8 {
        ApllctrlBanddirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlBypasspostdiv {
    #[doc = "Use the postdivider."]
    DISABLED = 0x0,
    #[doc = "Bypass of the postdivider"]
    ENABLED = 0x01,
}
impl ApllctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlBypasspostdiv {
        ApllctrlBypasspostdiv::from_bits(val)
    }
}
impl From<ApllctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlBypasspostdiv) -> u8 {
        ApllctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlBypasspostdiv2 {
    #[doc = "Use the divide-by-2 divider in the postdivider"]
    NOT_BYPASSED = 0x0,
    #[doc = "Bypass of the divide-by-2 divider in the postdivider"]
    BYPASSED = 0x01,
}
impl ApllctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlBypasspostdiv2 {
        ApllctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<ApllctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlBypasspostdiv2) -> u8 {
        ApllctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlBypassprediv {
    #[doc = "Use the predivider."]
    DISABLED = 0x0,
    #[doc = "Bypass of the predivider."]
    ENABLED = 0x01,
}
impl ApllctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlBypassprediv {
        ApllctrlBypassprediv::from_bits(val)
    }
}
impl From<ApllctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlBypassprediv) -> u8 {
        ApllctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlLimupoff {
    #[doc = "Application set to non-Spectrum and Fractional applications."]
    DISABLED = 0x0,
    #[doc = "Application set to Spectrum and Fractional applications."]
    ENABLED = 0x01,
}
impl ApllctrlLimupoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlLimupoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlLimupoff {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlLimupoff {
        ApllctrlLimupoff::from_bits(val)
    }
}
impl From<ApllctrlLimupoff> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlLimupoff) -> u8 {
        ApllctrlLimupoff::to_bits(val)
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
pub enum ApllmdivMreq {
    #[doc = "Feedback ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Feedback ratio change is requested"]
    ENABLED = 0x01,
}
impl ApllmdivMreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllmdivMreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllmdivMreq {
    #[inline(always)]
    fn from(val: u8) -> ApllmdivMreq {
        ApllmdivMreq::from_bits(val)
    }
}
impl From<ApllmdivMreq> for u8 {
    #[inline(always)]
    fn from(val: ApllmdivMreq) -> u8 {
        ApllmdivMreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllndivNreq {
    #[doc = "Predivider ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Predivider ratio change is requested"]
    ENABLED = 0x01,
}
impl ApllndivNreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllndivNreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllndivNreq {
    #[inline(always)]
    fn from(val: u8) -> ApllndivNreq {
        ApllndivNreq::from_bits(val)
    }
}
impl From<ApllndivNreq> for u8 {
    #[inline(always)]
    fn from(val: ApllndivNreq) -> u8 {
        ApllndivNreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllpdivPreq {
    #[doc = "Postdivider ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Postdivider ratio change is requested"]
    ENABLED = 0x01,
}
impl ApllpdivPreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllpdivPreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllpdivPreq {
    #[inline(always)]
    fn from(val: u8) -> ApllpdivPreq {
        ApllpdivPreq::from_bits(val)
    }
}
impl From<ApllpdivPreq> for u8 {
    #[inline(always)]
    fn from(val: ApllpdivPreq) -> u8 {
        ApllpdivPreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllpwren {
    #[doc = "APLL clock is powered off"]
    DISABLED = 0x0,
    #[doc = "APLL clock is powered on"]
    ENABLED = 0x01,
}
impl Apllpwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllpwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllpwren {
    #[inline(always)]
    fn from(val: u8) -> Apllpwren {
        Apllpwren::from_bits(val)
    }
}
impl From<Apllpwren> for u8 {
    #[inline(always)]
    fn from(val: Apllpwren) -> u8 {
        Apllpwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllpwrenOvrd {
    #[doc = "APLL clock is powered off"]
    DISABLED = 0x0,
    #[doc = "APLL clock is powered on"]
    ENABLED = 0x01,
}
impl ApllpwrenOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllpwrenOvrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllpwrenOvrd {
    #[inline(always)]
    fn from(val: u8) -> ApllpwrenOvrd {
        ApllpwrenOvrd::from_bits(val)
    }
}
impl From<ApllpwrenOvrd> for u8 {
    #[inline(always)]
    fn from(val: ApllpwrenOvrd) -> u8 {
        ApllpwrenOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsel {
    #[doc = "APLL is not the system clock source"]
    NOT_APLL = 0x0,
    #[doc = "APLL is the system clock source"]
    APLL = 0x01,
}
impl Apllsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsel {
    #[inline(always)]
    fn from(val: u8) -> Apllsel {
        Apllsel::from_bits(val)
    }
}
impl From<Apllsel> for u8 {
    #[inline(always)]
    fn from(val: Apllsel) -> u8 {
        Apllsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsscg1Dither {
    #[doc = "Dither is not enabled"]
    DISABLED = 0x0,
    #[doc = "Dither is enabled"]
    ENABLED = 0x01,
}
impl Apllsscg1Dither {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1Dither {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1Dither {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1Dither {
        Apllsscg1Dither::from_bits(val)
    }
}
impl From<Apllsscg1Dither> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1Dither) -> u8 {
        Apllsscg1Dither::to_bits(val)
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
pub enum Apllsscg1SelSsMdiv {
    #[doc = "Feedback divider ratio is MDIV\\[15:0\\]"]
    DISABLED = 0x0,
    #[doc = "Feedback divider ratio is SS_MDIV\\[32:0\\]"]
    ENABLED = 0x01,
}
impl Apllsscg1SelSsMdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1SelSsMdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1SelSsMdiv {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1SelSsMdiv {
        Apllsscg1SelSsMdiv::from_bits(val)
    }
}
impl From<Apllsscg1SelSsMdiv> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1SelSsMdiv) -> u8 {
        Apllsscg1SelSsMdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsscg1SsMdivReq {
    #[doc = "SS_MDIV change is not requested"]
    DISABLED = 0x0,
    #[doc = "SS_MDIV change is requested"]
    ENABLED = 0x01,
}
impl Apllsscg1SsMdivReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1SsMdivReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1SsMdivReq {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1SsMdivReq {
        Apllsscg1SsMdivReq::from_bits(val)
    }
}
impl From<Apllsscg1SsMdivReq> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1SsMdivReq) -> u8 {
        Apllsscg1SsMdivReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsscg1SsPd {
    #[doc = "SSCG is powered on"]
    DISABLED = 0x0,
    #[doc = "SSCG is powered off"]
    ENABLED = 0x01,
}
impl Apllsscg1SsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1SsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1SsPd {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1SsPd {
        Apllsscg1SsPd::from_bits(val)
    }
}
impl From<Apllsscg1SsPd> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1SsPd) -> u8 {
        Apllsscg1SsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllsscgstatSsMdivAck {
    #[doc = "The SS_MDIV, MF, MR, and MC ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The SS_MDIV, MF, MR, and MC ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl ApllsscgstatSsMdivAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllsscgstatSsMdivAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllsscgstatSsMdivAck {
    #[inline(always)]
    fn from(val: u8) -> ApllsscgstatSsMdivAck {
        ApllsscgstatSsMdivAck::from_bits(val)
    }
}
impl From<ApllsscgstatSsMdivAck> for u8 {
    #[inline(always)]
    fn from(val: ApllsscgstatSsMdivAck) -> u8 {
        ApllsscgstatSsMdivAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllstatMdivack {
    #[doc = "The feedback (M) ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The feedback (M) ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl ApllstatMdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllstatMdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllstatMdivack {
    #[inline(always)]
    fn from(val: u8) -> ApllstatMdivack {
        ApllstatMdivack::from_bits(val)
    }
}
impl From<ApllstatMdivack> for u8 {
    #[inline(always)]
    fn from(val: ApllstatMdivack) -> u8 {
        ApllstatMdivack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllstatNdivack {
    #[doc = "The predivider (N) ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The predivider (N) ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl ApllstatNdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllstatNdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllstatNdivack {
    #[inline(always)]
    fn from(val: u8) -> ApllstatNdivack {
        ApllstatNdivack::from_bits(val)
    }
}
impl From<ApllstatNdivack> for u8 {
    #[inline(always)]
    fn from(val: ApllstatNdivack) -> u8 {
        ApllstatNdivack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllstatPdivack {
    #[doc = "The postdivider (P) ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The postdivider (P) ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl ApllstatPdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllstatPdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllstatPdivack {
    #[inline(always)]
    fn from(val: u8) -> ApllstatPdivack {
        ApllstatPdivack::from_bits(val)
    }
}
impl From<ApllstatPdivack> for u8 {
    #[inline(always)]
    fn from(val: ApllstatPdivack) -> u8 {
        ApllstatPdivack::to_bits(val)
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
pub enum FircFclkPeriphEn {
    #[doc = "FIRC 144 MHz to peripherals is disabled"]
    DISABLED = 0x0,
    #[doc = "FIRC 144 MHz to peripherals is enabled"]
    ENABLED = 0x01,
}
impl FircFclkPeriphEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircFclkPeriphEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircFclkPeriphEn {
    #[inline(always)]
    fn from(val: u8) -> FircFclkPeriphEn {
        FircFclkPeriphEn::from_bits(val)
    }
}
impl From<FircFclkPeriphEn> for u8 {
    #[inline(always)]
    fn from(val: FircFclkPeriphEn) -> u8 {
        FircFclkPeriphEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircSclkPeriphEn {
    #[doc = "FIRC 48 MHz to peripherals is disabled"]
    DISABLED = 0x0,
    #[doc = "FIRC 48 MHz to peripherals is enabled"]
    ENABLED = 0x01,
}
impl FircSclkPeriphEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircSclkPeriphEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircSclkPeriphEn {
    #[inline(always)]
    fn from(val: u8) -> FircSclkPeriphEn {
        FircSclkPeriphEn::from_bits(val)
    }
}
impl From<FircSclkPeriphEn> for u8 {
    #[inline(always)]
    fn from(val: FircSclkPeriphEn) -> u8 {
        FircSclkPeriphEn::to_bits(val)
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
pub enum Fircclkpres {
    #[doc = "FIRC clock source is not present"]
    NOPRES = 0x0,
    #[doc = "FIRC clock source is present"]
    PRES = 0x01,
}
impl Fircclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircclkpres {
    #[inline(always)]
    fn from(val: u8) -> Fircclkpres {
        Fircclkpres::from_bits(val)
    }
}
impl From<Fircclkpres> for u8 {
    #[inline(always)]
    fn from(val: Fircclkpres) -> u8 {
        Fircclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirccsrCoarseTrimBypass {
    #[doc = "FIRC coarse auto trim is not bypassed"]
    NOT_BYPASSED = 0x0,
    #[doc = "FIRC coarse auto trim is bypassed"]
    BYPASSED = 0x01,
}
impl FirccsrCoarseTrimBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrCoarseTrimBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrCoarseTrimBypass {
    #[inline(always)]
    fn from(val: u8) -> FirccsrCoarseTrimBypass {
        FirccsrCoarseTrimBypass::from_bits(val)
    }
}
impl From<FirccsrCoarseTrimBypass> for u8 {
    #[inline(always)]
    fn from(val: FirccsrCoarseTrimBypass) -> u8 {
        FirccsrCoarseTrimBypass::to_bits(val)
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
pub enum Fircen {
    #[doc = "FIRC is disabled"]
    DISABLED = 0x0,
    #[doc = "FIRC is enabled"]
    ENABLED = 0x01,
}
impl Fircen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircen {
    #[inline(always)]
    fn from(val: u8) -> Fircen {
        Fircen::from_bits(val)
    }
}
impl From<Fircen> for u8 {
    #[inline(always)]
    fn from(val: Fircen) -> u8 {
        Fircen::to_bits(val)
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
pub enum Fircsel {
    #[doc = "FIRC is not the system clock source"]
    NOT_FIRC = 0x0,
    #[doc = "FIRC is the system clock source"]
    FIRC = 0x01,
}
impl Fircsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircsel {
    #[inline(always)]
    fn from(val: u8) -> Fircsel {
        Fircsel::from_bits(val)
    }
}
impl From<Fircsel> for u8 {
    #[inline(always)]
    fn from(val: Fircsel) -> u8 {
        Fircsel::to_bits(val)
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
pub enum Firctren {
    #[doc = "Disables trimming FIRC to an external clock source"]
    DISABLED = 0x0,
    #[doc = "Enables trimming FIRC to an external clock source"]
    ENABLED = 0x01,
}
impl Firctren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Firctren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Firctren {
    #[inline(always)]
    fn from(val: u8) -> Firctren {
        Firctren::from_bits(val)
    }
}
impl From<Firctren> for u8 {
    #[inline(always)]
    fn from(val: Firctren) -> u8 {
        Firctren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Firctrup {
    #[doc = "Disables FIRC trimming updates"]
    DISABLED = 0x0,
    #[doc = "Enables FIRC trimming updates"]
    ENABLED = 0x01,
}
impl Firctrup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Firctrup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Firctrup {
    #[inline(always)]
    fn from(val: u8) -> Firctrup {
        Firctrup::from_bits(val)
    }
}
impl From<Firctrup> for u8 {
    #[inline(always)]
    fn from(val: Firctrup) -> u8 {
        Firctrup::to_bits(val)
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
pub enum Ldobypass {
    #[doc = "LDO is not bypassed"]
    DISABLED = 0x0,
    #[doc = "LDO is bypassed"]
    ENABLED = 0x01,
}
impl Ldobypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldobypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldobypass {
    #[inline(always)]
    fn from(val: u8) -> Ldobypass {
        Ldobypass::from_bits(val)
    }
}
impl From<Ldobypass> for u8 {
    #[inline(always)]
    fn from(val: Ldobypass) -> u8 {
        Ldobypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldoen {
    #[doc = "LDO is disabled"]
    DISABLED = 0x0,
    #[doc = "LDO is enabled"]
    ENABLED = 0x01,
}
impl Ldoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldoen {
    #[inline(always)]
    fn from(val: u8) -> Ldoen {
        Ldoen::from_bits(val)
    }
}
impl From<Ldoen> for u8 {
    #[inline(always)]
    fn from(val: Ldoen) -> u8 {
        Ldoen::to_bits(val)
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
pub enum Roscclkpres {
    #[doc = "ROSC clock source is not present"]
    NOPRES = 0x0,
    #[doc = "ROSC clock source is present"]
    PRES = 0x01,
}
impl Roscclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscclkpres {
    #[inline(always)]
    fn from(val: u8) -> Roscclkpres {
        Roscclkpres::from_bits(val)
    }
}
impl From<Roscclkpres> for u8 {
    #[inline(always)]
    fn from(val: Roscclkpres) -> u8 {
        Roscclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rosccm {
    #[doc = "ROSC clock monitor is disabled"]
    DISABLED = 0x0,
    #[doc = "ROSC clock monitor is enabled"]
    ENABLED = 0x01,
}
impl Rosccm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rosccm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rosccm {
    #[inline(always)]
    fn from(val: u8) -> Rosccm {
        Rosccm::from_bits(val)
    }
}
impl From<Rosccm> for u8 {
    #[inline(always)]
    fn from(val: Rosccm) -> u8 {
        Rosccm::to_bits(val)
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
pub enum Roscsel {
    #[doc = "ROSC is not the system clock source"]
    NOT_ROSC = 0x0,
    #[doc = "ROSC is the system clock source"]
    ROSC = 0x01,
}
impl Roscsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscsel {
    #[inline(always)]
    fn from(val: u8) -> Roscsel {
        Roscsel::from_bits(val)
    }
}
impl From<Roscsel> for u8 {
    #[inline(always)]
    fn from(val: Roscsel) -> u8 {
        Roscsel::to_bits(val)
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
pub enum SircClkPeriphEn {
    #[doc = "SIRC clock to peripherals is disabled"]
    DISABLED = 0x0,
    #[doc = "SIRC clock to peripherals is enabled"]
    ENABLED = 0x01,
}
impl SircClkPeriphEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SircClkPeriphEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SircClkPeriphEn {
    #[inline(always)]
    fn from(val: u8) -> SircClkPeriphEn {
        SircClkPeriphEn::from_bits(val)
    }
}
impl From<SircClkPeriphEn> for u8 {
    #[inline(always)]
    fn from(val: SircClkPeriphEn) -> u8 {
        SircClkPeriphEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircclkpres {
    #[doc = "SIRC clock source is not present"]
    NOPRES = 0x0,
    #[doc = "SIRC clock source is present"]
    PRES = 0x01,
}
impl Sircclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircclkpres {
    #[inline(always)]
    fn from(val: u8) -> Sircclkpres {
        Sircclkpres::from_bits(val)
    }
}
impl From<Sircclkpres> for u8 {
    #[inline(always)]
    fn from(val: Sircclkpres) -> u8 {
        Sircclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirccsrCoarseTrimBypass {
    #[doc = "SIRC coarse auto-trim is not bypassed"]
    NOT_BYPASSED = 0x0,
    #[doc = "SIRC coarse auto-trim is bypassed"]
    BYPASSED = 0x01,
}
impl SirccsrCoarseTrimBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrCoarseTrimBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrCoarseTrimBypass {
    #[inline(always)]
    fn from(val: u8) -> SirccsrCoarseTrimBypass {
        SirccsrCoarseTrimBypass::from_bits(val)
    }
}
impl From<SirccsrCoarseTrimBypass> for u8 {
    #[inline(always)]
    fn from(val: SirccsrCoarseTrimBypass) -> u8 {
        SirccsrCoarseTrimBypass::to_bits(val)
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
pub enum Sircsel {
    #[doc = "SIRC is not the system clock source"]
    NOT_SIRC = 0x0,
    #[doc = "SIRC is the system clock source"]
    SIRC = 0x01,
}
impl Sircsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircsel {
    #[inline(always)]
    fn from(val: u8) -> Sircsel {
        Sircsel::from_bits(val)
    }
}
impl From<Sircsel> for u8 {
    #[inline(always)]
    fn from(val: Sircsel) -> u8 {
        Sircsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircsten {
    #[doc = "SIRC is disabled in Deep Sleep mode"]
    DISABLED = 0x0,
    #[doc = "SIRC is enabled in Deep Sleep mode"]
    ENABLED = 0x01,
}
impl Sircsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircsten {
    #[inline(always)]
    fn from(val: u8) -> Sircsten {
        Sircsten::from_bits(val)
    }
}
impl From<Sircsten> for u8 {
    #[inline(always)]
    fn from(val: Sircsten) -> u8 {
        Sircsten::to_bits(val)
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
pub enum Sirctren {
    #[doc = "Disables trimming SIRC to an external clock source"]
    DISABLED = 0x0,
    #[doc = "Enables trimming SIRC to an external clock source"]
    ENABLED = 0x01,
}
impl Sirctren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sirctren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sirctren {
    #[inline(always)]
    fn from(val: u8) -> Sirctren {
        Sirctren::from_bits(val)
    }
}
impl From<Sirctren> for u8 {
    #[inline(always)]
    fn from(val: Sirctren) -> u8 {
        Sirctren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sirctrup {
    #[doc = "Disables SIRC trimming updates"]
    DISABLED = 0x0,
    #[doc = "Enables SIRC trimming updates"]
    ENABLED = 0x01,
}
impl Sirctrup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sirctrup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sirctrup {
    #[inline(always)]
    fn from(val: u8) -> Sirctrup {
        Sirctrup::from_bits(val)
    }
}
impl From<Sirctrup> for u8 {
    #[inline(always)]
    fn from(val: Sirctrup) -> u8 {
        Sirctrup::to_bits(val)
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
pub enum Soscclkpres {
    #[doc = "SOSC clock source is not present"]
    NOPRES = 0x0,
    #[doc = "SOSC clock source is present"]
    PRES = 0x01,
}
impl Soscclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscclkpres {
    #[inline(always)]
    fn from(val: u8) -> Soscclkpres {
        Soscclkpres::from_bits(val)
    }
}
impl From<Soscclkpres> for u8 {
    #[inline(always)]
    fn from(val: Soscclkpres) -> u8 {
        Soscclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sosccm {
    #[doc = "SOSC Clock Monitor is disabled"]
    DISABLED = 0x0,
    #[doc = "SOSC Clock Monitor is enabled"]
    ENABLED = 0x01,
}
impl Sosccm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sosccm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sosccm {
    #[inline(always)]
    fn from(val: u8) -> Sosccm {
        Sosccm::from_bits(val)
    }
}
impl From<Sosccm> for u8 {
    #[inline(always)]
    fn from(val: Sosccm) -> u8 {
        Sosccm::to_bits(val)
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
pub enum Soscen {
    #[doc = "SOSC is disabled"]
    DISABLED = 0x0,
    #[doc = "SOSC is enabled"]
    ENABLED = 0x01,
}
impl Soscen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscen {
    #[inline(always)]
    fn from(val: u8) -> Soscen {
        Soscen::from_bits(val)
    }
}
impl From<Soscen> for u8 {
    #[inline(always)]
    fn from(val: Soscen) -> u8 {
        Soscen::to_bits(val)
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
pub enum Soscsel {
    #[doc = "SOSC is not the system clock source"]
    NOT_SOSC = 0x0,
    #[doc = "SOSC is the system clock source"]
    SOSC = 0x01,
}
impl Soscsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscsel {
    #[inline(always)]
    fn from(val: u8) -> Soscsel {
        Soscsel::from_bits(val)
    }
}
impl From<Soscsel> for u8 {
    #[inline(always)]
    fn from(val: Soscsel) -> u8 {
        Soscsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soscsten {
    #[doc = "SOSC is disabled in Deep Sleep mode"]
    DISABLED = 0x0,
    #[doc = "SOSC is enabled in Deep Sleep mode only if SOSCEN is set"]
    ENABLED = 0x01,
}
impl Soscsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscsten {
    #[inline(always)]
    fn from(val: u8) -> Soscsten {
        Soscsten::from_bits(val)
    }
}
impl From<Soscsten> for u8 {
    #[inline(always)]
    fn from(val: Soscsten) -> u8 {
        Soscsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soscvld {
    #[doc = "SOSC is not enabled or clock is not valid"]
    DISABLED = 0x0,
    #[doc = "SOSC is enabled and output clock is valid"]
    ENABLED = 0x01,
}
impl Soscvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscvld {
    #[inline(always)]
    fn from(val: u8) -> Soscvld {
        Soscvld::from_bits(val)
    }
}
impl From<Soscvld> for u8 {
    #[inline(always)]
    fn from(val: Soscvld) -> u8 {
        Soscvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SoscvldIe {
    #[doc = "SOSCVLD interrupt is not enabled"]
    NOT_SOSC = 0x0,
    #[doc = "SOSCVLD interrupt is enabled"]
    SOSC = 0x01,
}
impl SoscvldIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SoscvldIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SoscvldIe {
    #[inline(always)]
    fn from(val: u8) -> SoscvldIe {
        SoscvldIe::from_bits(val)
    }
}
impl From<SoscvldIe> for u8 {
    #[inline(always)]
    fn from(val: SoscvldIe) -> u8 {
        SoscvldIe::to_bits(val)
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
pub enum SpllLockIe {
    #[doc = "SPLL_LOCK interrupt is not enabled"]
    NOT_SPLL = 0x0,
    #[doc = "SPLL_LOCK interrupt is enabled"]
    SPLL = 0x01,
}
impl SpllLockIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllLockIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllLockIe {
    #[inline(always)]
    fn from(val: u8) -> SpllLockIe {
        SpllLockIe::from_bits(val)
    }
}
impl From<SpllLockIe> for u8 {
    #[inline(always)]
    fn from(val: SpllLockIe) -> u8 {
        SpllLockIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllOvrdEn {
    #[doc = "SPLL override is disabled"]
    DISABLED = 0x0,
    #[doc = "SPLL override is enabled"]
    ENABLED = 0x01,
}
impl SpllOvrdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllOvrdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllOvrdEn {
    #[inline(always)]
    fn from(val: u8) -> SpllOvrdEn {
        SpllOvrdEn::from_bits(val)
    }
}
impl From<SpllOvrdEn> for u8 {
    #[inline(always)]
    fn from(val: SpllOvrdEn) -> u8 {
        SpllOvrdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllclken {
    #[doc = "SPLL clock is disabled"]
    DISABLED = 0x0,
    #[doc = "SPLL clock is enabled"]
    ENABLED = 0x01,
}
impl Spllclken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllclken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllclken {
    #[inline(always)]
    fn from(val: u8) -> Spllclken {
        Spllclken::from_bits(val)
    }
}
impl From<Spllclken> for u8 {
    #[inline(always)]
    fn from(val: Spllclken) -> u8 {
        Spllclken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllclkenOvrd {
    #[doc = "SPLL clock is disabled"]
    DISABLED = 0x0,
    #[doc = "SPLL clock is enabled"]
    ENABLED = 0x01,
}
impl SpllclkenOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllclkenOvrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllclkenOvrd {
    #[inline(always)]
    fn from(val: u8) -> SpllclkenOvrd {
        SpllclkenOvrd::from_bits(val)
    }
}
impl From<SpllclkenOvrd> for u8 {
    #[inline(always)]
    fn from(val: SpllclkenOvrd) -> u8 {
        SpllclkenOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllclkpres {
    #[doc = "SPLL clock source is not present"]
    NOPRES = 0x0,
    #[doc = "SPLL clock source is present"]
    PRES = 0x01,
}
impl Spllclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllclkpres {
    #[inline(always)]
    fn from(val: u8) -> Spllclkpres {
        Spllclkpres::from_bits(val)
    }
}
impl From<Spllclkpres> for u8 {
    #[inline(always)]
    fn from(val: Spllclkpres) -> u8 {
        Spllclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllcm {
    #[doc = "SPLL Clock Monitor is disabled"]
    DISABLED = 0x0,
    #[doc = "SPLL Clock Monitor is enabled"]
    ENABLED = 0x01,
}
impl Spllcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllcm {
    #[inline(always)]
    fn from(val: u8) -> Spllcm {
        Spllcm::from_bits(val)
    }
}
impl From<Spllcm> for u8 {
    #[inline(always)]
    fn from(val: Spllcm) -> u8 {
        Spllcm::to_bits(val)
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
pub enum SpllctrlBanddirect {
    #[doc = "The bandwidth is changed synchronously with the feedback-divider"]
    DISABLED = 0x0,
    #[doc = "Modifies the bandwidth of the PLL directly"]
    ENABLED = 0x01,
}
impl SpllctrlBanddirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlBanddirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlBanddirect {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlBanddirect {
        SpllctrlBanddirect::from_bits(val)
    }
}
impl From<SpllctrlBanddirect> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlBanddirect) -> u8 {
        SpllctrlBanddirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllctrlBypasspostdiv {
    #[doc = "Use the postdivider"]
    DISABLED = 0x0,
    #[doc = "Bypass of the postdivider"]
    ENABLED = 0x01,
}
impl SpllctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlBypasspostdiv {
        SpllctrlBypasspostdiv::from_bits(val)
    }
}
impl From<SpllctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlBypasspostdiv) -> u8 {
        SpllctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllctrlBypasspostdiv2 {
    #[doc = "Use the divide-by-2 divider in the postdivider."]
    NOT_BYPASSED = 0x0,
    #[doc = "Bypass of the divide-by-2 divider in the postdivider"]
    BYPASSED = 0x01,
}
impl SpllctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlBypasspostdiv2 {
        SpllctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<SpllctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlBypasspostdiv2) -> u8 {
        SpllctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllctrlBypassprediv {
    #[doc = "Use the predivider"]
    DISABLED = 0x0,
    #[doc = "Bypass of the predivider"]
    ENABLED = 0x01,
}
impl SpllctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlBypassprediv {
        SpllctrlBypassprediv::from_bits(val)
    }
}
impl From<SpllctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlBypassprediv) -> u8 {
        SpllctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllctrlLimupoff {
    #[doc = "Application set to non-Spectrum and Fractional applications."]
    DISABLED = 0x0,
    #[doc = "Application set to Spectrum and Fractional applications."]
    ENABLED = 0x01,
}
impl SpllctrlLimupoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllctrlLimupoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllctrlLimupoff {
    #[inline(always)]
    fn from(val: u8) -> SpllctrlLimupoff {
        SpllctrlLimupoff::from_bits(val)
    }
}
impl From<SpllctrlLimupoff> for u8 {
    #[inline(always)]
    fn from(val: SpllctrlLimupoff) -> u8 {
        SpllctrlLimupoff::to_bits(val)
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
pub enum SpllmdivMreq {
    #[doc = "Feedback ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Feedback ratio change is requested"]
    ENABLED = 0x01,
}
impl SpllmdivMreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllmdivMreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllmdivMreq {
    #[inline(always)]
    fn from(val: u8) -> SpllmdivMreq {
        SpllmdivMreq::from_bits(val)
    }
}
impl From<SpllmdivMreq> for u8 {
    #[inline(always)]
    fn from(val: SpllmdivMreq) -> u8 {
        SpllmdivMreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllndivNreq {
    #[doc = "Predivider ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Predivider ratio change is requested"]
    ENABLED = 0x01,
}
impl SpllndivNreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllndivNreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllndivNreq {
    #[inline(always)]
    fn from(val: u8) -> SpllndivNreq {
        SpllndivNreq::from_bits(val)
    }
}
impl From<SpllndivNreq> for u8 {
    #[inline(always)]
    fn from(val: SpllndivNreq) -> u8 {
        SpllndivNreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllpdivPreq {
    #[doc = "Postdivider ratio change is not requested"]
    DISABLED = 0x0,
    #[doc = "Postdivider ratio change is requested"]
    ENABLED = 0x01,
}
impl SpllpdivPreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllpdivPreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllpdivPreq {
    #[inline(always)]
    fn from(val: u8) -> SpllpdivPreq {
        SpllpdivPreq::from_bits(val)
    }
}
impl From<SpllpdivPreq> for u8 {
    #[inline(always)]
    fn from(val: SpllpdivPreq) -> u8 {
        SpllpdivPreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllpwren {
    #[doc = "SPLL clock is powered off"]
    DISABLED = 0x0,
    #[doc = "SPLL clock is powered on"]
    ENABLED = 0x01,
}
impl Spllpwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllpwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllpwren {
    #[inline(always)]
    fn from(val: u8) -> Spllpwren {
        Spllpwren::from_bits(val)
    }
}
impl From<Spllpwren> for u8 {
    #[inline(always)]
    fn from(val: Spllpwren) -> u8 {
        Spllpwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllpwrenOvrd {
    #[doc = "SPLL clock is powered off"]
    DISABLED = 0x0,
    #[doc = "SPLL clock is powered on"]
    ENABLED = 0x01,
}
impl SpllpwrenOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllpwrenOvrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllpwrenOvrd {
    #[inline(always)]
    fn from(val: u8) -> SpllpwrenOvrd {
        SpllpwrenOvrd::from_bits(val)
    }
}
impl From<SpllpwrenOvrd> for u8 {
    #[inline(always)]
    fn from(val: SpllpwrenOvrd) -> u8 {
        SpllpwrenOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsel {
    #[doc = "SPLL is not the system clock source"]
    NOT_SPLL = 0x0,
    #[doc = "SPLL is the system clock source"]
    SPLL = 0x01,
}
impl Spllsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsel {
    #[inline(always)]
    fn from(val: u8) -> Spllsel {
        Spllsel::from_bits(val)
    }
}
impl From<Spllsel> for u8 {
    #[inline(always)]
    fn from(val: Spllsel) -> u8 {
        Spllsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsscg1Dither {
    #[doc = "Dither is not enabled"]
    DISABLED = 0x0,
    #[doc = "Dither is enabled"]
    ENABLED = 0x01,
}
impl Spllsscg1Dither {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1Dither {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1Dither {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1Dither {
        Spllsscg1Dither::from_bits(val)
    }
}
impl From<Spllsscg1Dither> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1Dither) -> u8 {
        Spllsscg1Dither::to_bits(val)
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
pub enum Spllsscg1SelSsMdiv {
    #[doc = "Feedback divider ratio is MDIV\\[15:0\\]"]
    DISABLED = 0x0,
    #[doc = "Feedback divider ratio is SS_MDIV\\[32:0\\]"]
    ENABLED = 0x01,
}
impl Spllsscg1SelSsMdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1SelSsMdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1SelSsMdiv {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1SelSsMdiv {
        Spllsscg1SelSsMdiv::from_bits(val)
    }
}
impl From<Spllsscg1SelSsMdiv> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1SelSsMdiv) -> u8 {
        Spllsscg1SelSsMdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsscg1SsMdivReq {
    #[doc = "SS_MDIV change is not requested"]
    DISABLED = 0x0,
    #[doc = "SS_MDIV change is requested"]
    ENABLED = 0x01,
}
impl Spllsscg1SsMdivReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1SsMdivReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1SsMdivReq {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1SsMdivReq {
        Spllsscg1SsMdivReq::from_bits(val)
    }
}
impl From<Spllsscg1SsMdivReq> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1SsMdivReq) -> u8 {
        Spllsscg1SsMdivReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsscg1SsPd {
    #[doc = "SSCG is powered on"]
    DISABLED = 0x0,
    #[doc = "SSCG is powered off"]
    ENABLED = 0x01,
}
impl Spllsscg1SsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1SsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1SsPd {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1SsPd {
        Spllsscg1SsPd::from_bits(val)
    }
}
impl From<Spllsscg1SsPd> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1SsPd) -> u8 {
        Spllsscg1SsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllsscgstatSsMdivAck {
    #[doc = "The SS_MDIV, MF, MR, and MC ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The SS_MDIV, MF, MR, and MC ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl SpllsscgstatSsMdivAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllsscgstatSsMdivAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllsscgstatSsMdivAck {
    #[inline(always)]
    fn from(val: u8) -> SpllsscgstatSsMdivAck {
        SpllsscgstatSsMdivAck::from_bits(val)
    }
}
impl From<SpllsscgstatSsMdivAck> for u8 {
    #[inline(always)]
    fn from(val: SpllsscgstatSsMdivAck) -> u8 {
        SpllsscgstatSsMdivAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllstatMdivack {
    #[doc = "The feedback (M) ratio change is not accepted by the analog PLL."]
    DISABLED = 0x0,
    #[doc = "The feedback (M) ratio change is accepted by the analog PLL."]
    ENABLED = 0x01,
}
impl SpllstatMdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllstatMdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllstatMdivack {
    #[inline(always)]
    fn from(val: u8) -> SpllstatMdivack {
        SpllstatMdivack::from_bits(val)
    }
}
impl From<SpllstatMdivack> for u8 {
    #[inline(always)]
    fn from(val: SpllstatMdivack) -> u8 {
        SpllstatMdivack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllstatNdivack {
    #[doc = "The predivider (N) ratio change is not accepted by the analog PLL."]
    DISABLED = 0x0,
    #[doc = "The predivider (N) ratio change is accepted by the analog PLL."]
    ENABLED = 0x01,
}
impl SpllstatNdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllstatNdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllstatNdivack {
    #[inline(always)]
    fn from(val: u8) -> SpllstatNdivack {
        SpllstatNdivack::from_bits(val)
    }
}
impl From<SpllstatNdivack> for u8 {
    #[inline(always)]
    fn from(val: SpllstatNdivack) -> u8 {
        SpllstatNdivack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllstatPdivack {
    #[doc = "The postdivider (P) ratio change is not accepted by the analog PLL"]
    DISABLED = 0x0,
    #[doc = "The postdivider (P) ratio change is accepted by the analog PLL"]
    ENABLED = 0x01,
}
impl SpllstatPdivack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllstatPdivack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllstatPdivack {
    #[inline(always)]
    fn from(val: u8) -> SpllstatPdivack {
        SpllstatPdivack::from_bits(val)
    }
}
impl From<SpllstatPdivack> for u8 {
    #[inline(always)]
    fn from(val: SpllstatPdivack) -> u8 {
        SpllstatPdivack::to_bits(val)
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
pub enum Upllclkpres {
    #[doc = "UPLL clock source is not present"]
    NOPRES = 0x0,
    #[doc = "UPLL clock source is present"]
    PRES = 0x01,
}
impl Upllclkpres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllclkpres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllclkpres {
    #[inline(always)]
    fn from(val: u8) -> Upllclkpres {
        Upllclkpres::from_bits(val)
    }
}
impl From<Upllclkpres> for u8 {
    #[inline(always)]
    fn from(val: Upllclkpres) -> u8 {
        Upllclkpres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllcm {
    #[doc = "UPLL Clock Monitor is disabled"]
    DISABLED = 0x0,
    #[doc = "UPLL Clock Monitor is enabled"]
    ENABLED = 0x01,
}
impl Upllcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllcm {
    #[inline(always)]
    fn from(val: u8) -> Upllcm {
        Upllcm::from_bits(val)
    }
}
impl From<Upllcm> for u8 {
    #[inline(always)]
    fn from(val: Upllcm) -> u8 {
        Upllcm::to_bits(val)
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
pub enum Upllsel {
    #[doc = "UPLL is not the system clock source"]
    NOT_USB_PLL = 0x0,
    #[doc = "UPLL is the system clock source"]
    USB_PLL = 0x01,
}
impl Upllsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllsel {
    #[inline(always)]
    fn from(val: u8) -> Upllsel {
        Upllsel::from_bits(val)
    }
}
impl From<Upllsel> for u8 {
    #[inline(always)]
    fn from(val: Upllsel) -> u8 {
        Upllsel::to_bits(val)
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
pub enum VoutOk {
    #[doc = "LDO output VOUT is not OK"]
    DISABLED = 0x0,
    #[doc = "LDO output VOUT is OK"]
    ENABLED = 0x01,
}
impl VoutOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoutOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoutOk {
    #[inline(always)]
    fn from(val: u8) -> VoutOk {
        VoutOk::from_bits(val)
    }
}
impl From<VoutOk> for u8 {
    #[inline(always)]
    fn from(val: VoutOk) -> u8 {
        VoutOk::to_bits(val)
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
