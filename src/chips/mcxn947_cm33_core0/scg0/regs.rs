#[doc = "APLL Override Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApllOvrd(pub u32);
impl ApllOvrd {
    #[doc = "APLL Power Enable Override if APLL_OVRD_EN=1"]
    #[must_use]
    #[inline(always)]
    pub const fn apllpwren_ovrd(&self) -> super::vals::ApllpwrenOvrd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApllpwrenOvrd::from_bits(val as u8)
    }
    #[doc = "APLL Power Enable Override if APLL_OVRD_EN=1"]
    #[inline(always)]
    pub const fn set_apllpwren_ovrd(&mut self, val: super::vals::ApllpwrenOvrd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APLL Clock Enable Override if APLL_OVRD_EN=1"]
    #[must_use]
    #[inline(always)]
    pub const fn apllclken_ovrd(&self) -> super::vals::ApllclkenOvrd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ApllclkenOvrd::from_bits(val as u8)
    }
    #[doc = "APLL Clock Enable Override if APLL_OVRD_EN=1"]
    #[inline(always)]
    pub const fn set_apllclken_ovrd(&mut self, val: super::vals::ApllclkenOvrd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "APLL Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apll_ovrd_en(&self) -> super::vals::ApllOvrdEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ApllOvrdEn::from_bits(val as u8)
    }
    #[doc = "APLL Override Enable"]
    #[inline(always)]
    pub const fn set_apll_ovrd_en(&mut self, val: super::vals::ApllOvrdEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ApllOvrd {
    #[inline(always)]
    fn default() -> ApllOvrd {
        ApllOvrd(0)
    }
}
impl core::fmt::Debug for ApllOvrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApllOvrd")
            .field("apllpwren_ovrd", &self.apllpwren_ovrd())
            .field("apllclken_ovrd", &self.apllclken_ovrd())
            .field("apll_ovrd_en", &self.apll_ovrd_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApllOvrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApllOvrd {{ apllpwren_ovrd: {:?}, apllclken_ovrd: {:?}, apll_ovrd_en: {:?} }}",
            self.apllpwren_ovrd(),
            self.apllclken_ovrd(),
            self.apll_ovrd_en()
        )
    }
}
#[doc = "APLL Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllcsr(pub u32);
impl Apllcsr {
    #[doc = "APLL Power Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apllpwren(&self) -> super::vals::Apllpwren {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Apllpwren::from_bits(val as u8)
    }
    #[doc = "APLL Power Enable"]
    #[inline(always)]
    pub const fn set_apllpwren(&mut self, val: super::vals::Apllpwren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APLL Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apllclken(&self) -> super::vals::Apllclken {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Apllclken::from_bits(val as u8)
    }
    #[doc = "APLL Clock Enable"]
    #[inline(always)]
    pub const fn set_apllclken(&mut self, val: super::vals::Apllclken) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "APLL Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apllsten(&self) -> super::vals::Apllsten {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Apllsten::from_bits(val as u8)
    }
    #[doc = "APLL Stop Enable"]
    #[inline(always)]
    pub const fn set_apllsten(&mut self, val: super::vals::Apllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "APLL Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn apllcm(&self) -> super::vals::Apllcm {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Apllcm::from_bits(val as u8)
    }
    #[doc = "APLL Clock Monitor"]
    #[inline(always)]
    pub const fn set_apllcm(&mut self, val: super::vals::Apllcm) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "APLL Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apllcmre(&self) -> super::vals::Apllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Apllcmre::from_bits(val as u8)
    }
    #[doc = "APLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_apllcmre(&mut self, val: super::vals::Apllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::ApllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::ApllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::ApllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "APLL LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn apll_lock(&self) -> super::vals::ApllLock {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::ApllLock::from_bits(val as u8)
    }
    #[doc = "APLL LOCK"]
    #[inline(always)]
    pub const fn set_apll_lock(&mut self, val: super::vals::ApllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "APLL Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn apllsel(&self) -> super::vals::Apllsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Apllsel::from_bits(val as u8)
    }
    #[doc = "APLL Selected"]
    #[inline(always)]
    pub const fn set_apllsel(&mut self, val: super::vals::Apllsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "APLL Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn apllerr(&self) -> super::vals::Apllerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Apllerr::from_bits(val as u8)
    }
    #[doc = "APLL Clock Error"]
    #[inline(always)]
    pub const fn set_apllerr(&mut self, val: super::vals::Apllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "APLL LOCK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apll_lock_ie(&self) -> super::vals::ApllLockIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ApllLockIe::from_bits(val as u8)
    }
    #[doc = "APLL LOCK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_apll_lock_ie(&mut self, val: super::vals::ApllLockIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Apllcsr {
    #[inline(always)]
    fn default() -> Apllcsr {
        Apllcsr(0)
    }
}
impl core::fmt::Debug for Apllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllcsr")
            .field("apllpwren", &self.apllpwren())
            .field("apllclken", &self.apllclken())
            .field("apllsten", &self.apllsten())
            .field("apllcm", &self.apllcm())
            .field("apllcmre", &self.apllcmre())
            .field("lk", &self.lk())
            .field("apll_lock", &self.apll_lock())
            .field("apllsel", &self.apllsel())
            .field("apllerr", &self.apllerr())
            .field("apll_lock_ie", &self.apll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllcsr {{ apllpwren: {:?}, apllclken: {:?}, apllsten: {:?}, apllcm: {:?}, apllcmre: {:?}, lk: {:?}, apll_lock: {:?}, apllsel: {:?}, apllerr: {:?}, apll_lock_ie: {:?} }}",
            self.apllpwren(),
            self.apllclken(),
            self.apllsten(),
            self.apllcm(),
            self.apllcmre(),
            self.lk(),
            self.apll_lock(),
            self.apllsel(),
            self.apllerr(),
            self.apll_lock_ie()
        )
    }
}
#[doc = "APLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllctrl(pub u32);
impl Apllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> super::vals::ApllctrlBypasspostdiv2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ApllctrlBypasspostdiv2::from_bits(val as u8)
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: super::vals::ApllctrlBypasspostdiv2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter"]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> super::vals::ApllctrlLimupoff {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ApllctrlLimupoff::from_bits(val as u8)
    }
    #[doc = "Up Limiter"]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: super::vals::ApllctrlLimupoff) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> super::vals::ApllctrlBanddirect {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ApllctrlBanddirect::from_bits(val as u8)
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: super::vals::ApllctrlBanddirect) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the predivider"]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> super::vals::ApllctrlBypassprediv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ApllctrlBypassprediv::from_bits(val as u8)
    }
    #[doc = "Bypass of the predivider"]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: super::vals::ApllctrlBypassprediv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the postdivider"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> super::vals::ApllctrlBypasspostdiv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ApllctrlBypasspostdiv::from_bits(val as u8)
    }
    #[doc = "Bypass of the postdivider"]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: super::vals::ApllctrlBypasspostdiv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> super::vals::ApllctrlSource {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::ApllctrlSource::from_bits(val as u8)
    }
    #[doc = "Clock Source"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: super::vals::ApllctrlSource) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Apllctrl {
    #[inline(always)]
    fn default() -> Apllctrl {
        Apllctrl(0)
    }
}
impl core::fmt::Debug for Apllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {:?}, limupoff: {:?}, banddirect: {:?}, bypassprediv: {:?}, bypasspostdiv: {:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.source()
        )
    }
}
#[doc = "APLL LOCK Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AplllockCnfg(pub u32);
impl AplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before APLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before APLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for AplllockCnfg {
    #[inline(always)]
    fn default() -> AplllockCnfg {
        AplllockCnfg(0)
    }
}
impl core::fmt::Debug for AplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "APLL M Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllmdiv(pub u32);
impl Apllmdiv {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> super::vals::ApllmdivMreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ApllmdivMreq::from_bits(val as u8)
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: super::vals::ApllmdivMreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllmdiv {
    #[inline(always)]
    fn default() -> Apllmdiv {
        Apllmdiv(0)
    }
}
impl core::fmt::Debug for Apllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllmdiv {{ mdiv: {=u16:?}, mreq: {:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "APLL N Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllndiv(pub u32);
impl Apllndiv {
    #[doc = "Predivider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Predivider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Predivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> super::vals::ApllndivNreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ApllndivNreq::from_bits(val as u8)
    }
    #[doc = "Predivider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: super::vals::ApllndivNreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllndiv {
    #[inline(always)]
    fn default() -> Apllndiv {
        Apllndiv(0)
    }
}
impl core::fmt::Debug for Apllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllndiv {{ ndiv: {=u8:?}, nreq: {:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "APLL P Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllpdiv(pub u32);
impl Apllpdiv {
    #[doc = "Postdivider divider ratio (P-divider)"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Postdivider divider ratio (P-divider)"]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Postdivider ratio change request"]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> super::vals::ApllpdivPreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ApllpdivPreq::from_bits(val as u8)
    }
    #[doc = "Postdivider ratio change request"]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: super::vals::ApllpdivPreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllpdiv {
    #[inline(always)]
    fn default() -> Apllpdiv {
        Apllpdiv(0)
    }
}
impl core::fmt::Debug for Apllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllpdiv {{ pdiv: {=u8:?}, preq: {:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "APLL Spread Spectrum Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscg0(pub u32);
impl Apllsscg0 {
    #[doc = "SS_MDIV"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV"]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Apllsscg0 {
    #[inline(always)]
    fn default() -> Apllsscg0 {
        Apllsscg0(0)
    }
}
impl core::fmt::Debug for Apllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "APLL Spread Spectrum Control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscg1(pub u32);
impl Apllsscg1 {
    #[doc = "SS_MDIV\\[32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]"]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> super::vals::Apllsscg1SsMdivReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Apllsscg1SsMdivReq::from_bits(val as u8)
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: super::vals::Apllsscg1SsMdivReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control"]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> super::vals::Apllsscg1Mc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Apllsscg1Mc::from_bits(val as u8)
    }
    #[doc = "Modulation Waveform Control"]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: super::vals::Apllsscg1Mc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> super::vals::Apllsscg1Dither {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Apllsscg1Dither::from_bits(val as u8)
    }
    #[doc = "Dither Enable"]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: super::vals::Apllsscg1Dither) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> super::vals::Apllsscg1SelSsMdiv {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Apllsscg1SelSsMdiv::from_bits(val as u8)
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: super::vals::Apllsscg1SelSsMdiv) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> super::vals::Apllsscg1SsPd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Apllsscg1SsPd::from_bits(val as u8)
    }
    #[doc = "SSCG Power Down"]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: super::vals::Apllsscg1SsPd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllsscg1 {
    #[inline(always)]
    fn default() -> Apllsscg1 {
        Apllsscg1(0)
    }
}
impl core::fmt::Debug for Apllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {:?}, dither: {:?}, sel_ss_mdiv: {:?}, ss_pd: {:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "APLL SSCG Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscgstat(pub u32);
impl Apllsscgstat {
    #[doc = "SS_MDIV change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> super::vals::ApllsscgstatSsMdivAck {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApllsscgstatSsMdivAck::from_bits(val as u8)
    }
    #[doc = "SS_MDIV change acknowledge"]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: super::vals::ApllsscgstatSsMdivAck) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Apllsscgstat {
    #[inline(always)]
    fn default() -> Apllsscgstat {
        Apllsscgstat(0)
    }
}
impl core::fmt::Debug for Apllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscgstat {{ ss_mdiv_ack: {:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "APLL Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllstat(pub u32);
impl Apllstat {
    #[doc = "Predivider(N) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> super::vals::ApllstatNdivack {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ApllstatNdivack::from_bits(val as u8)
    }
    #[doc = "Predivider(N) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: super::vals::ApllstatNdivack) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback(M) divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> super::vals::ApllstatMdivack {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ApllstatMdivack::from_bits(val as u8)
    }
    #[doc = "Feedback(M) divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: super::vals::ApllstatMdivack) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Postdivider(P) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> super::vals::ApllstatPdivack {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ApllstatPdivack::from_bits(val as u8)
    }
    #[doc = "Postdivider(P) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: super::vals::ApllstatPdivack) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Apllstat {
    #[inline(always)]
    fn default() -> Apllstat {
        Apllstat(0)
    }
}
impl core::fmt::Debug for Apllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllstat")
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllstat {{ ndivack: {:?}, mdivack: {:?}, pdivack: {:?} }}",
            self.ndivack(),
            self.mdivack(),
            self.pdivack()
        )
    }
}
#[doc = "Clock Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "System Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::CsrScs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CsrScs::from_bits(val as u8)
    }
    #[doc = "System Clock Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::CsrScs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "FIRC Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc = "Frequency Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::FirccfgRange {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FirccfgRange::from_bits(val as u8)
    }
    #[doc = "Frequency Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::FirccfgRange) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Firccfg {
    #[inline(always)]
    fn default() -> Firccfg {
        Firccfg(0)
    }
}
impl core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccfg")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Firccfg {{ range: {:?} }}", self.range())
    }
}
#[doc = "FIRC Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc = "FIRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircen(&self) -> super::vals::Fircen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fircen::from_bits(val as u8)
    }
    #[doc = "FIRC Enable"]
    #[inline(always)]
    pub const fn set_fircen(&mut self, val: super::vals::Fircen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIRC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircsten(&self) -> super::vals::Fircsten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fircsten::from_bits(val as u8)
    }
    #[doc = "FIRC Stop Enable"]
    #[inline(always)]
    pub const fn set_fircsten(&mut self, val: super::vals::Fircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn firc_sclk_periph_en(&self) -> super::vals::FircSclkPeriphEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::FircSclkPeriphEn::from_bits(val as u8)
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable"]
    #[inline(always)]
    pub const fn set_firc_sclk_periph_en(&mut self, val: super::vals::FircSclkPeriphEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FIRC 144 MHz Clock to peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> super::vals::FircFclkPeriphEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::FircFclkPeriphEn::from_bits(val as u8)
    }
    #[doc = "FIRC 144 MHz Clock to peripherals Enable"]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: super::vals::FircFclkPeriphEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FIRC 144 MHz Trim Enable (FIRCCFG\\[RANGE\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn firctren(&self) -> super::vals::Firctren {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Firctren::from_bits(val as u8)
    }
    #[doc = "FIRC 144 MHz Trim Enable (FIRCCFG\\[RANGE\\]=1)"]
    #[inline(always)]
    pub const fn set_firctren(&mut self, val: super::vals::Firctren) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FIRC Trim Update"]
    #[must_use]
    #[inline(always)]
    pub const fn firctrup(&self) -> super::vals::Firctrup {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Firctrup::from_bits(val as u8)
    }
    #[doc = "FIRC Trim Update"]
    #[inline(always)]
    pub const fn set_firctrup(&mut self, val: super::vals::Firctrup) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FIRC TRIM LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::FirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "FIRC TRIM LOCK"]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::FirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> super::vals::FirccsrCoarseTrimBypass {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FirccsrCoarseTrimBypass::from_bits(val as u8)
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: super::vals::FirccsrCoarseTrimBypass) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::FirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::FirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::FirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FIRC Valid status"]
    #[must_use]
    #[inline(always)]
    pub const fn fircvld(&self) -> super::vals::Fircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Fircvld::from_bits(val as u8)
    }
    #[doc = "FIRC Valid status"]
    #[inline(always)]
    pub const fn set_fircvld(&mut self, val: super::vals::Fircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIRC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn fircsel(&self) -> super::vals::Fircsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Fircsel::from_bits(val as u8)
    }
    #[doc = "FIRC Selected"]
    #[inline(always)]
    pub const fn set_fircsel(&mut self, val: super::vals::Fircsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FIRC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr(&self) -> super::vals::Fircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Fircerr::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error"]
    #[inline(always)]
    pub const fn set_fircerr(&mut self, val: super::vals::Fircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FIRC Clock Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr_ie(&self) -> super::vals::FircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::FircerrIe::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fircerr_ie(&mut self, val: super::vals::FircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FIRC Accurate Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc_ie(&self) -> super::vals::FircaccIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FircaccIe::from_bits(val as u8)
    }
    #[doc = "FIRC Accurate Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fircacc_ie(&mut self, val: super::vals::FircaccIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FIRC Frequency Accurate"]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc(&self) -> super::vals::Fircacc {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Fircacc::from_bits(val as u8)
    }
    #[doc = "FIRC Frequency Accurate"]
    #[inline(always)]
    pub const fn set_fircacc(&mut self, val: super::vals::Fircacc) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Firccsr {
    #[inline(always)]
    fn default() -> Firccsr {
        Firccsr(0)
    }
}
impl core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccsr")
            .field("fircen", &self.fircen())
            .field("fircsten", &self.fircsten())
            .field("firc_sclk_periph_en", &self.firc_sclk_periph_en())
            .field("firc_fclk_periph_en", &self.firc_fclk_periph_en())
            .field("firctren", &self.firctren())
            .field("firctrup", &self.firctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("fircvld", &self.fircvld())
            .field("fircsel", &self.fircsel())
            .field("fircerr", &self.fircerr())
            .field("fircerr_ie", &self.fircerr_ie())
            .field("fircacc_ie", &self.fircacc_ie())
            .field("fircacc", &self.fircacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firccsr {{ fircen: {:?}, fircsten: {:?}, firc_sclk_periph_en: {:?}, firc_fclk_periph_en: {:?}, firctren: {:?}, firctrup: {:?}, trim_lock: {:?}, coarse_trim_bypass: {:?}, lk: {:?}, fircvld: {:?}, fircsel: {:?}, fircerr: {:?}, fircerr_ie: {:?}, fircacc_ie: {:?}, fircacc: {:?} }}",
            self.fircen(),
            self.fircsten(),
            self.firc_sclk_periph_en(),
            self.firc_fclk_periph_en(),
            self.firctren(),
            self.firctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.fircvld(),
            self.fircsel(),
            self.fircerr(),
            self.fircerr_ie(),
            self.fircacc_ie(),
            self.fircacc()
        )
    }
}
#[doc = "FIRC Auto-trimming Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircstat(pub u32);
impl Fircstat {
    #[doc = "Trim Fine"]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine"]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse"]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse"]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Fircstat {
    #[inline(always)]
    fn default() -> Fircstat {
        Fircstat(0)
    }
}
impl core::fmt::Debug for Fircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircstat")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircstat {{ trimfine: {=u8:?}, trimcoar: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar()
        )
    }
}
#[doc = "FIRC Trim Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctcfg(pub u32);
impl Firctcfg {
    #[doc = "Trim Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::FirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source"]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::FirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FIRC Trim Predivider"]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "FIRC Trim Predivider"]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Firctcfg {
    #[inline(always)]
    fn default() -> Firctcfg {
        Firctcfg(0)
    }
}
impl core::fmt::Debug for Firctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "FIRC Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctrim(pub u32);
impl Firctrim {
    #[doc = "Trim Fine"]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine"]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse"]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse"]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn trimtemp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Trim Temperature"]
    #[inline(always)]
    pub const fn set_trimtemp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Trim Start"]
    #[must_use]
    #[inline(always)]
    pub const fn trimstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Start"]
    #[inline(always)]
    pub const fn set_trimstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Firctrim {
    #[inline(always)]
    fn default() -> Firctrim {
        Firctrim(0)
    }
}
impl core::fmt::Debug for Firctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctrim")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .field("trimtemp", &self.trimtemp())
            .field("trimstart", &self.trimstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctrim {{ trimfine: {=u8:?}, trimcoar: {=u8:?}, trimtemp: {=u8:?}, trimstart: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar(),
            self.trimtemp(),
            self.trimstart()
        )
    }
}
#[doc = "LDO Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldocsr(pub u32);
impl Ldocsr {
    #[doc = "LDO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ldoen(&self) -> super::vals::Ldoen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ldoen::from_bits(val as u8)
    }
    #[doc = "LDO Enable"]
    #[inline(always)]
    pub const fn set_ldoen(&mut self, val: super::vals::Ldoen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO output voltage select"]
    #[must_use]
    #[inline(always)]
    pub const fn vout_sel(&self) -> super::vals::VoutSel {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::VoutSel::from_bits(val as u8)
    }
    #[doc = "LDO output voltage select"]
    #[inline(always)]
    pub const fn set_vout_sel(&mut self, val: super::vals::VoutSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "LDO Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn ldobypass(&self) -> super::vals::Ldobypass {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ldobypass::from_bits(val as u8)
    }
    #[doc = "LDO Bypass"]
    #[inline(always)]
    pub const fn set_ldobypass(&mut self, val: super::vals::Ldobypass) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO VOUT OK Inform."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_ok(&self) -> super::vals::VoutOk {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::VoutOk::from_bits(val as u8)
    }
    #[doc = "LDO VOUT OK Inform."]
    #[inline(always)]
    pub const fn set_vout_ok(&mut self, val: super::vals::VoutOk) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldocsr {
    #[inline(always)]
    fn default() -> Ldocsr {
        Ldocsr(0)
    }
}
impl core::fmt::Debug for Ldocsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldocsr")
            .field("ldoen", &self.ldoen())
            .field("vout_sel", &self.vout_sel())
            .field("ldobypass", &self.ldobypass())
            .field("vout_ok", &self.vout_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldocsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldocsr {{ ldoen: {:?}, vout_sel: {:?}, ldobypass: {:?}, vout_ok: {:?} }}",
            self.ldoen(),
            self.vout_sel(),
            self.ldobypass(),
            self.vout_ok()
        )
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "SOSC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn soscclkpres(&self) -> super::vals::Soscclkpres {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Soscclkpres::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Present"]
    #[inline(always)]
    pub const fn set_soscclkpres(&mut self, val: super::vals::Soscclkpres) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn sircclkpres(&self) -> super::vals::Sircclkpres {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sircclkpres::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Present"]
    #[inline(always)]
    pub const fn set_sircclkpres(&mut self, val: super::vals::Sircclkpres) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FIRC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn fircclkpres(&self) -> super::vals::Fircclkpres {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Fircclkpres::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Present"]
    #[inline(always)]
    pub const fn set_fircclkpres(&mut self, val: super::vals::Fircclkpres) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ROSC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn roscclkpres(&self) -> super::vals::Roscclkpres {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Roscclkpres::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Present"]
    #[inline(always)]
    pub const fn set_roscclkpres(&mut self, val: super::vals::Roscclkpres) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "APLL Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn apllclkpres(&self) -> super::vals::Apllclkpres {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Apllclkpres::from_bits(val as u8)
    }
    #[doc = "APLL Clock Present"]
    #[inline(always)]
    pub const fn set_apllclkpres(&mut self, val: super::vals::Apllclkpres) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SPLL Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spllclkpres(&self) -> super::vals::Spllclkpres {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Spllclkpres::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Present"]
    #[inline(always)]
    pub const fn set_spllclkpres(&mut self, val: super::vals::Spllclkpres) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "UPLL Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn upllclkpres(&self) -> super::vals::Upllclkpres {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Upllclkpres::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Present"]
    #[inline(always)]
    pub const fn set_upllclkpres(&mut self, val: super::vals::Upllclkpres) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("soscclkpres", &self.soscclkpres())
            .field("sircclkpres", &self.sircclkpres())
            .field("fircclkpres", &self.fircclkpres())
            .field("roscclkpres", &self.roscclkpres())
            .field("apllclkpres", &self.apllclkpres())
            .field("spllclkpres", &self.spllclkpres())
            .field("upllclkpres", &self.upllclkpres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {:?}, sircclkpres: {:?}, fircclkpres: {:?}, roscclkpres: {:?}, apllclkpres: {:?}, spllclkpres: {:?}, upllclkpres: {:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres(),
            self.apllclkpres(),
            self.spllclkpres(),
            self.upllclkpres()
        )
    }
}
#[doc = "Run Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "System Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::RccrScs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::RccrScs::from_bits(val as u8)
    }
    #[doc = "System Clock Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::RccrScs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rccr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "ROSC Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosccsr(pub u32);
impl Rosccsr {
    #[doc = "ROSC Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn rosccm(&self) -> super::vals::Rosccm {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Rosccm::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Monitor"]
    #[inline(always)]
    pub const fn set_rosccm(&mut self, val: super::vals::Rosccm) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "ROSC Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rosccmre(&self) -> super::vals::Rosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rosccmre::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_rosccmre(&mut self, val: super::vals::Rosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::RosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::RosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ROSC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn roscvld(&self) -> super::vals::Roscvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Roscvld::from_bits(val as u8)
    }
    #[doc = "ROSC Valid"]
    #[inline(always)]
    pub const fn set_roscvld(&mut self, val: super::vals::Roscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ROSC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn roscsel(&self) -> super::vals::Roscsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Roscsel::from_bits(val as u8)
    }
    #[doc = "ROSC Selected"]
    #[inline(always)]
    pub const fn set_roscsel(&mut self, val: super::vals::Roscsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "ROSC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn roscerr(&self) -> super::vals::Roscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Roscerr::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Error"]
    #[inline(always)]
    pub const fn set_roscerr(&mut self, val: super::vals::Roscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Rosccsr {
    #[inline(always)]
    fn default() -> Rosccsr {
        Rosccsr(0)
    }
}
impl core::fmt::Debug for Rosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rosccsr")
            .field("rosccm", &self.rosccm())
            .field("rosccmre", &self.rosccmre())
            .field("lk", &self.lk())
            .field("roscvld", &self.roscvld())
            .field("roscsel", &self.roscsel())
            .field("roscerr", &self.roscerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rosccsr {{ rosccm: {:?}, rosccmre: {:?}, lk: {:?}, roscvld: {:?}, roscsel: {:?}, roscerr: {:?} }}",
            self.rosccm(),
            self.rosccmre(),
            self.lk(),
            self.roscvld(),
            self.roscsel(),
            self.roscerr()
        )
    }
}
#[doc = "SIRC Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc = "SIRC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sircsten(&self) -> super::vals::Sircsten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sircsten::from_bits(val as u8)
    }
    #[doc = "SIRC Stop Enable"]
    #[inline(always)]
    pub const fn set_sircsten(&mut self, val: super::vals::Sircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock to Peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sirc_clk_periph_en(&self) -> super::vals::SircClkPeriphEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SircClkPeriphEn::from_bits(val as u8)
    }
    #[doc = "SIRC Clock to Peripherals Enable"]
    #[inline(always)]
    pub const fn set_sirc_clk_periph_en(&mut self, val: super::vals::SircClkPeriphEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn sirctren(&self) -> super::vals::Sirctren {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sirctren::from_bits(val as u8)
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)"]
    #[inline(always)]
    pub const fn set_sirctren(&mut self, val: super::vals::Sirctren) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SIRC Trim Update"]
    #[must_use]
    #[inline(always)]
    pub const fn sirctrup(&self) -> super::vals::Sirctrup {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sirctrup::from_bits(val as u8)
    }
    #[doc = "SIRC Trim Update"]
    #[inline(always)]
    pub const fn set_sirctrup(&mut self, val: super::vals::Sirctrup) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SIRC TRIM LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::SirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "SIRC TRIM LOCK"]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::SirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> super::vals::SirccsrCoarseTrimBypass {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SirccsrCoarseTrimBypass::from_bits(val as u8)
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: super::vals::SirccsrCoarseTrimBypass) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SIRC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn sircvld(&self) -> super::vals::Sircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Sircvld::from_bits(val as u8)
    }
    #[doc = "SIRC Valid"]
    #[inline(always)]
    pub const fn set_sircvld(&mut self, val: super::vals::Sircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SIRC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn sircsel(&self) -> super::vals::Sircsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Sircsel::from_bits(val as u8)
    }
    #[doc = "SIRC Selected"]
    #[inline(always)]
    pub const fn set_sircsel(&mut self, val: super::vals::Sircsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "SIRC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr(&self) -> super::vals::Sircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Sircerr::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error"]
    #[inline(always)]
    pub const fn set_sircerr(&mut self, val: super::vals::Sircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIRC Clock Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr_ie(&self) -> super::vals::SircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SircerrIe::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sircerr_ie(&mut self, val: super::vals::SircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Sirccsr {
    #[inline(always)]
    fn default() -> Sirccsr {
        Sirccsr(0)
    }
}
impl core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirccsr")
            .field("sircsten", &self.sircsten())
            .field("sirc_clk_periph_en", &self.sirc_clk_periph_en())
            .field("sirctren", &self.sirctren())
            .field("sirctrup", &self.sirctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("sircvld", &self.sircvld())
            .field("sircsel", &self.sircsel())
            .field("sircerr", &self.sircerr())
            .field("sircerr_ie", &self.sircerr_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirccsr {{ sircsten: {:?}, sirc_clk_periph_en: {:?}, sirctren: {:?}, sirctrup: {:?}, trim_lock: {:?}, coarse_trim_bypass: {:?}, lk: {:?}, sircvld: {:?}, sircsel: {:?}, sircerr: {:?}, sircerr_ie: {:?} }}",
            self.sircsten(),
            self.sirc_clk_periph_en(),
            self.sirctren(),
            self.sirctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.sircvld(),
            self.sircsel(),
            self.sircerr(),
            self.sircerr_ie()
        )
    }
}
#[doc = "SIRC Auto-trimming Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sircstat(pub u32);
impl Sircstat {
    #[doc = "CCO Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim"]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim"]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sircstat {
    #[inline(always)]
    fn default() -> Sircstat {
        Sircstat(0)
    }
}
impl core::fmt::Debug for Sircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sircstat")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sircstat {{ ccotrim: {=u8:?}, cltrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim()
        )
    }
}
#[doc = "SIRC Trim Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctcfg(pub u32);
impl Sirctcfg {
    #[doc = "Trim Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::SirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source"]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::SirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SIRC Trim Predivider"]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Predivider"]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Sirctcfg {
    #[inline(always)]
    fn default() -> Sirctcfg {
        Sirctcfg(0)
    }
}
impl core::fmt::Debug for Sirctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "SIRC Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctrim(pub u32);
impl Sirctrim {
    #[doc = "CCO Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim"]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim"]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temp"]
    #[must_use]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Temp"]
    #[inline(always)]
    pub const fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period"]
    #[must_use]
    #[inline(always)]
    pub const fn fvchtrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period"]
    #[inline(always)]
    pub const fn set_fvchtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sirctrim {
    #[inline(always)]
    fn default() -> Sirctrim {
        Sirctrim(0)
    }
}
impl core::fmt::Debug for Sirctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctrim")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .field("tctrim", &self.tctrim())
            .field("fvchtrim", &self.fvchtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctrim {{ ccotrim: {=u8:?}, cltrim: {=u8:?}, tctrim: {=u8:?}, fvchtrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim(),
            self.tctrim(),
            self.fvchtrim()
        )
    }
}
#[doc = "SOSC Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc = "External Reference Select"]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> super::vals::Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select"]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: super::vals::Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::SosccfgRange {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SosccfgRange::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::SosccfgRange) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sosccfg {
    #[inline(always)]
    fn default() -> Sosccfg {
        Sosccfg(0)
    }
}
impl core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccfg")
            .field("erefs", &self.erefs())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccfg {{ erefs: {:?}, range: {:?} }}",
            self.erefs(),
            self.range()
        )
    }
}
#[doc = "SOSC Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc = "SOSC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscen(&self) -> super::vals::Soscen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Soscen::from_bits(val as u8)
    }
    #[doc = "SOSC Enable"]
    #[inline(always)]
    pub const fn set_soscen(&mut self, val: super::vals::Soscen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SOSC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscsten(&self) -> super::vals::Soscsten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Soscsten::from_bits(val as u8)
    }
    #[doc = "SOSC Stop Enable"]
    #[inline(always)]
    pub const fn set_soscsten(&mut self, val: super::vals::Soscsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SOSC Clock Monitor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sosccm(&self) -> super::vals::Sosccm {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Sosccm::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Enable"]
    #[inline(always)]
    pub const fn set_sosccm(&mut self, val: super::vals::Sosccm) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SOSC Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sosccmre(&self) -> super::vals::Sosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Sosccmre::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_sosccmre(&mut self, val: super::vals::Sosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SOSC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld(&self) -> super::vals::Soscvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Soscvld::from_bits(val as u8)
    }
    #[doc = "SOSC Valid"]
    #[inline(always)]
    pub const fn set_soscvld(&mut self, val: super::vals::Soscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SOSC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn soscsel(&self) -> super::vals::Soscsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Soscsel::from_bits(val as u8)
    }
    #[doc = "SOSC Selected"]
    #[inline(always)]
    pub const fn set_soscsel(&mut self, val: super::vals::Soscsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "SOSC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soscerr(&self) -> super::vals::Soscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Soscerr::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Error"]
    #[inline(always)]
    pub const fn set_soscerr(&mut self, val: super::vals::Soscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SOSC Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld_ie(&self) -> super::vals::SoscvldIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SoscvldIe::from_bits(val as u8)
    }
    #[doc = "SOSC Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_soscvld_ie(&mut self, val: super::vals::SoscvldIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Sosccsr {
    #[inline(always)]
    fn default() -> Sosccsr {
        Sosccsr(0)
    }
}
impl core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccsr")
            .field("soscen", &self.soscen())
            .field("soscsten", &self.soscsten())
            .field("sosccm", &self.sosccm())
            .field("sosccmre", &self.sosccmre())
            .field("lk", &self.lk())
            .field("soscvld", &self.soscvld())
            .field("soscsel", &self.soscsel())
            .field("soscerr", &self.soscerr())
            .field("soscvld_ie", &self.soscvld_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {:?}, soscsten: {:?}, sosccm: {:?}, sosccmre: {:?}, lk: {:?}, soscvld: {:?}, soscsel: {:?}, soscerr: {:?}, soscvld_ie: {:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie()
        )
    }
}
#[doc = "SPLL Override Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpllOvrd(pub u32);
impl SpllOvrd {
    #[doc = "SPLL Power Enable Override if SPLL_OVRD_EN=1"]
    #[must_use]
    #[inline(always)]
    pub const fn spllpwren_ovrd(&self) -> super::vals::SpllpwrenOvrd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SpllpwrenOvrd::from_bits(val as u8)
    }
    #[doc = "SPLL Power Enable Override if SPLL_OVRD_EN=1"]
    #[inline(always)]
    pub const fn set_spllpwren_ovrd(&mut self, val: super::vals::SpllpwrenOvrd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPLL Clock Enable Override if SPLL_OVRD_EN=1"]
    #[must_use]
    #[inline(always)]
    pub const fn spllclken_ovrd(&self) -> super::vals::SpllclkenOvrd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SpllclkenOvrd::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Enable Override if SPLL_OVRD_EN=1"]
    #[inline(always)]
    pub const fn set_spllclken_ovrd(&mut self, val: super::vals::SpllclkenOvrd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SPLL Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_ovrd_en(&self) -> super::vals::SpllOvrdEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SpllOvrdEn::from_bits(val as u8)
    }
    #[doc = "SPLL Override Enable"]
    #[inline(always)]
    pub const fn set_spll_ovrd_en(&mut self, val: super::vals::SpllOvrdEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SpllOvrd {
    #[inline(always)]
    fn default() -> SpllOvrd {
        SpllOvrd(0)
    }
}
impl core::fmt::Debug for SpllOvrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpllOvrd")
            .field("spllpwren_ovrd", &self.spllpwren_ovrd())
            .field("spllclken_ovrd", &self.spllclken_ovrd())
            .field("spll_ovrd_en", &self.spll_ovrd_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpllOvrd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SpllOvrd {{ spllpwren_ovrd: {:?}, spllclken_ovrd: {:?}, spll_ovrd_en: {:?} }}",
            self.spllpwren_ovrd(),
            self.spllclken_ovrd(),
            self.spll_ovrd_en()
        )
    }
}
#[doc = "SPLL Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllcsr(pub u32);
impl Spllcsr {
    #[doc = "SPLL Power Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllpwren(&self) -> super::vals::Spllpwren {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Spllpwren::from_bits(val as u8)
    }
    #[doc = "SPLL Power Enable"]
    #[inline(always)]
    pub const fn set_spllpwren(&mut self, val: super::vals::Spllpwren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPLL Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllclken(&self) -> super::vals::Spllclken {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Spllclken::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Enable"]
    #[inline(always)]
    pub const fn set_spllclken(&mut self, val: super::vals::Spllclken) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SPLL Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllsten(&self) -> super::vals::Spllsten {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Spllsten::from_bits(val as u8)
    }
    #[doc = "SPLL Stop Enable"]
    #[inline(always)]
    pub const fn set_spllsten(&mut self, val: super::vals::Spllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SPLL Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn spllcm(&self) -> super::vals::Spllcm {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Spllcm::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Monitor"]
    #[inline(always)]
    pub const fn set_spllcm(&mut self, val: super::vals::Spllcm) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SPLL Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllcmre(&self) -> super::vals::Spllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Spllcmre::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_spllcmre(&mut self, val: super::vals::Spllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SPLL LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock(&self) -> super::vals::SpllLock {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SpllLock::from_bits(val as u8)
    }
    #[doc = "SPLL LOCK"]
    #[inline(always)]
    pub const fn set_spll_lock(&mut self, val: super::vals::SpllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SPLL Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn spllsel(&self) -> super::vals::Spllsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Spllsel::from_bits(val as u8)
    }
    #[doc = "SPLL Selected"]
    #[inline(always)]
    pub const fn set_spllsel(&mut self, val: super::vals::Spllsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "SPLL Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn spllerr(&self) -> super::vals::Spllerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Spllerr::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Error"]
    #[inline(always)]
    pub const fn set_spllerr(&mut self, val: super::vals::Spllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SPLL LOCK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock_ie(&self) -> super::vals::SpllLockIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SpllLockIe::from_bits(val as u8)
    }
    #[doc = "SPLL LOCK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_spll_lock_ie(&mut self, val: super::vals::SpllLockIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Spllcsr {
    #[inline(always)]
    fn default() -> Spllcsr {
        Spllcsr(0)
    }
}
impl core::fmt::Debug for Spllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllcsr")
            .field("spllpwren", &self.spllpwren())
            .field("spllclken", &self.spllclken())
            .field("spllsten", &self.spllsten())
            .field("spllcm", &self.spllcm())
            .field("spllcmre", &self.spllcmre())
            .field("lk", &self.lk())
            .field("spll_lock", &self.spll_lock())
            .field("spllsel", &self.spllsel())
            .field("spllerr", &self.spllerr())
            .field("spll_lock_ie", &self.spll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllcsr {{ spllpwren: {:?}, spllclken: {:?}, spllsten: {:?}, spllcm: {:?}, spllcmre: {:?}, lk: {:?}, spll_lock: {:?}, spllsel: {:?}, spllerr: {:?}, spll_lock_ie: {:?} }}",
            self.spllpwren(),
            self.spllclken(),
            self.spllsten(),
            self.spllcm(),
            self.spllcmre(),
            self.lk(),
            self.spll_lock(),
            self.spllsel(),
            self.spllerr(),
            self.spll_lock_ie()
        )
    }
}
#[doc = "SPLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllctrl(pub u32);
impl Spllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> super::vals::SpllctrlBypasspostdiv2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SpllctrlBypasspostdiv2::from_bits(val as u8)
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: super::vals::SpllctrlBypasspostdiv2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> super::vals::SpllctrlLimupoff {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SpllctrlLimupoff::from_bits(val as u8)
    }
    #[doc = "Up Limiter."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: super::vals::SpllctrlLimupoff) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> super::vals::SpllctrlBanddirect {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::SpllctrlBanddirect::from_bits(val as u8)
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: super::vals::SpllctrlBanddirect) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the predivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> super::vals::SpllctrlBypassprediv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::SpllctrlBypassprediv::from_bits(val as u8)
    }
    #[doc = "Bypass of the predivider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: super::vals::SpllctrlBypassprediv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the postdivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> super::vals::SpllctrlBypasspostdiv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::SpllctrlBypasspostdiv::from_bits(val as u8)
    }
    #[doc = "Bypass of the postdivider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: super::vals::SpllctrlBypasspostdiv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> super::vals::SpllctrlSource {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::SpllctrlSource::from_bits(val as u8)
    }
    #[doc = "Clock Source"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: super::vals::SpllctrlSource) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Spllctrl {
    #[inline(always)]
    fn default() -> Spllctrl {
        Spllctrl(0)
    }
}
impl core::fmt::Debug for Spllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {:?}, limupoff: {:?}, banddirect: {:?}, bypassprediv: {:?}, bypasspostdiv: {:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.source()
        )
    }
}
#[doc = "SPLL LOCK Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SplllockCnfg(pub u32);
impl SplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for SplllockCnfg {
    #[inline(always)]
    fn default() -> SplllockCnfg {
        SplllockCnfg(0)
    }
}
impl core::fmt::Debug for SplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "SPLL M Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllmdiv(pub u32);
impl Spllmdiv {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> super::vals::SpllmdivMreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SpllmdivMreq::from_bits(val as u8)
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: super::vals::SpllmdivMreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllmdiv {
    #[inline(always)]
    fn default() -> Spllmdiv {
        Spllmdiv(0)
    }
}
impl core::fmt::Debug for Spllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllmdiv {{ mdiv: {=u16:?}, mreq: {:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "SPLL N Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllndiv(pub u32);
impl Spllndiv {
    #[doc = "Predivider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Predivider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Predivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> super::vals::SpllndivNreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SpllndivNreq::from_bits(val as u8)
    }
    #[doc = "Predivider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: super::vals::SpllndivNreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllndiv {
    #[inline(always)]
    fn default() -> Spllndiv {
        Spllndiv(0)
    }
}
impl core::fmt::Debug for Spllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllndiv {{ ndiv: {=u8:?}, nreq: {:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "SPLL P Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllpdiv(pub u32);
impl Spllpdiv {
    #[doc = "Postdivider divider ratio (P-divider)"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Postdivider divider ratio (P-divider)"]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Postdivider ratio change request"]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> super::vals::SpllpdivPreq {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SpllpdivPreq::from_bits(val as u8)
    }
    #[doc = "Postdivider ratio change request"]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: super::vals::SpllpdivPreq) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllpdiv {
    #[inline(always)]
    fn default() -> Spllpdiv {
        Spllpdiv(0)
    }
}
impl core::fmt::Debug for Spllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllpdiv {{ pdiv: {=u8:?}, preq: {:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg0(pub u32);
impl Spllsscg0 {
    #[doc = "SS_MDIV\\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV\\[31:0\\]"]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spllsscg0 {
    #[inline(always)]
    fn default() -> Spllsscg0 {
        Spllsscg0(0)
    }
}
impl core::fmt::Debug for Spllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg1(pub u32);
impl Spllsscg1 {
    #[doc = "SS_MDIV\\[32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]"]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> super::vals::Spllsscg1SsMdivReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Spllsscg1SsMdivReq::from_bits(val as u8)
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: super::vals::Spllsscg1SsMdivReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control"]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> super::vals::Spllsscg1Mc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Spllsscg1Mc::from_bits(val as u8)
    }
    #[doc = "Modulation Waveform Control"]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: super::vals::Spllsscg1Mc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> super::vals::Spllsscg1Dither {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Spllsscg1Dither::from_bits(val as u8)
    }
    #[doc = "Dither Enable"]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: super::vals::Spllsscg1Dither) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> super::vals::Spllsscg1SelSsMdiv {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Spllsscg1SelSsMdiv::from_bits(val as u8)
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: super::vals::Spllsscg1SelSsMdiv) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> super::vals::Spllsscg1SsPd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Spllsscg1SsPd::from_bits(val as u8)
    }
    #[doc = "SSCG Power Down"]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: super::vals::Spllsscg1SsPd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllsscg1 {
    #[inline(always)]
    fn default() -> Spllsscg1 {
        Spllsscg1(0)
    }
}
impl core::fmt::Debug for Spllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {:?}, dither: {:?}, sel_ss_mdiv: {:?}, ss_pd: {:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "SPLL SSCG Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscgstat(pub u32);
impl Spllsscgstat {
    #[doc = "SS_MDIV change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> super::vals::SpllsscgstatSsMdivAck {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SpllsscgstatSsMdivAck::from_bits(val as u8)
    }
    #[doc = "SS_MDIV change acknowledge"]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: super::vals::SpllsscgstatSsMdivAck) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Spllsscgstat {
    #[inline(always)]
    fn default() -> Spllsscgstat {
        Spllsscgstat(0)
    }
}
impl core::fmt::Debug for Spllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscgstat {{ ss_mdiv_ack: {:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "SPLL Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllstat(pub u32);
impl Spllstat {
    #[doc = "Predivider (N) ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> super::vals::SpllstatNdivack {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SpllstatNdivack::from_bits(val as u8)
    }
    #[doc = "Predivider (N) ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: super::vals::SpllstatNdivack) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback (M) divider ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> super::vals::SpllstatMdivack {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SpllstatMdivack::from_bits(val as u8)
    }
    #[doc = "Feedback (M) divider ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: super::vals::SpllstatMdivack) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Postdivider (P) ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> super::vals::SpllstatPdivack {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SpllstatPdivack::from_bits(val as u8)
    }
    #[doc = "Postdivider (P) ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: super::vals::SpllstatPdivack) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Spllstat {
    #[inline(always)]
    fn default() -> Spllstat {
        Spllstat(0)
    }
}
impl core::fmt::Debug for Spllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllstat")
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllstat {{ ndivack: {:?}, mdivack: {:?}, pdivack: {:?} }}",
            self.ndivack(),
            self.mdivack(),
            self.pdivack()
        )
    }
}
#[doc = "Trim Lock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimLock(pub u32);
impl TrimLock {
    #[doc = "TRIM_UNLOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_unlock(&self) -> super::vals::TrimUnlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TrimUnlock::from_bits(val as u8)
    }
    #[doc = "TRIM_UNLOCK"]
    #[inline(always)]
    pub const fn set_trim_unlock(&mut self, val: super::vals::TrimUnlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR_DISABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_disable(&self) -> super::vals::IfrDisable {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IfrDisable::from_bits(val as u8)
    }
    #[doc = "IFR_DISABLE"]
    #[inline(always)]
    pub const fn set_ifr_disable(&mut self, val: super::vals::IfrDisable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIM_LOCK_KEY"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock_key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIM_LOCK_KEY"]
    #[inline(always)]
    pub const fn set_trim_lock_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrimLock {
    #[inline(always)]
    fn default() -> TrimLock {
        TrimLock(0)
    }
}
impl core::fmt::Debug for TrimLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimLock")
            .field("trim_unlock", &self.trim_unlock())
            .field("ifr_disable", &self.ifr_disable())
            .field("trim_lock_key", &self.trim_lock_key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimLock {{ trim_unlock: {:?}, ifr_disable: {:?}, trim_lock_key: {=u16:?} }}",
            self.trim_unlock(),
            self.ifr_disable(),
            self.trim_lock_key()
        )
    }
}
#[doc = "UPLL Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Upllcsr(pub u32);
impl Upllcsr {
    #[doc = "UPLL Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn upllcm(&self) -> super::vals::Upllcm {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Upllcm::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Monitor"]
    #[inline(always)]
    pub const fn set_upllcm(&mut self, val: super::vals::Upllcm) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "UPLL Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn upllcmre(&self) -> super::vals::Upllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Upllcmre::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_upllcmre(&mut self, val: super::vals::Upllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::UpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::UpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::UpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "UPLL Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn upllvld(&self) -> super::vals::Upllvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Upllvld::from_bits(val as u8)
    }
    #[doc = "UPLL Valid"]
    #[inline(always)]
    pub const fn set_upllvld(&mut self, val: super::vals::Upllvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "UPLL Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn upllsel(&self) -> super::vals::Upllsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Upllsel::from_bits(val as u8)
    }
    #[doc = "UPLL Selected"]
    #[inline(always)]
    pub const fn set_upllsel(&mut self, val: super::vals::Upllsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "UPLL Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn upllerr(&self) -> super::vals::Upllerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Upllerr::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Error"]
    #[inline(always)]
    pub const fn set_upllerr(&mut self, val: super::vals::Upllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Upllcsr {
    #[inline(always)]
    fn default() -> Upllcsr {
        Upllcsr(0)
    }
}
impl core::fmt::Debug for Upllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Upllcsr")
            .field("upllcm", &self.upllcm())
            .field("upllcmre", &self.upllcmre())
            .field("lk", &self.lk())
            .field("upllvld", &self.upllvld())
            .field("upllsel", &self.upllsel())
            .field("upllerr", &self.upllerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Upllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Upllcsr {{ upllcm: {:?}, upllcmre: {:?}, lk: {:?}, upllvld: {:?}, upllsel: {:?}, upllerr: {:?} }}",
            self.upllcm(),
            self.upllcmre(),
            self.lk(),
            self.upllvld(),
            self.upllsel(),
            self.upllerr()
        )
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "SCG Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SCG Version Number"]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Verid {{ version: {=u32:?} }}", self.version())
    }
}
