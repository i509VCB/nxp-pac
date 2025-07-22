#[doc = "Keypad Data Direction Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kddr(pub u16);
impl Kddr {
    #[doc = "Keypad Row Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn krdd(&self) -> super::vals::Krdd {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Krdd::from_bits(val as u8)
    }
    #[doc = "Keypad Row Data Direction"]
    #[inline(always)]
    pub const fn set_krdd(&mut self, val: super::vals::Krdd) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "Keypad Column Data Direction Register"]
    #[must_use]
    #[inline(always)]
    pub const fn kcdd(&self) -> super::vals::Kcdd {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Kcdd::from_bits(val as u8)
    }
    #[doc = "Keypad Column Data Direction Register"]
    #[inline(always)]
    pub const fn set_kcdd(&mut self, val: super::vals::Kcdd) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Kddr {
    #[inline(always)]
    fn default() -> Kddr {
        Kddr(0)
    }
}
impl core::fmt::Debug for Kddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kddr")
            .field("krdd", &self.krdd())
            .field("kcdd", &self.kcdd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Kddr {{ krdd: {:?}, kcdd: {:?} }}",
            self.krdd(),
            self.kcdd()
        )
    }
}
#[doc = "Keypad Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpcr(pub u16);
impl Kpcr {
    #[doc = "Keypad Row Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn kre(&self) -> super::vals::Kre {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Kre::from_bits(val as u8)
    }
    #[doc = "Keypad Row Enable"]
    #[inline(always)]
    pub const fn set_kre(&mut self, val: super::vals::Kre) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "Keypad Column Strobe Open-Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn kco(&self) -> super::vals::Kco {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Kco::from_bits(val as u8)
    }
    #[doc = "Keypad Column Strobe Open-Drain Enable"]
    #[inline(always)]
    pub const fn set_kco(&mut self, val: super::vals::Kco) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Kpcr {
    #[inline(always)]
    fn default() -> Kpcr {
        Kpcr(0)
    }
}
impl core::fmt::Debug for Kpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpcr")
            .field("kre", &self.kre())
            .field("kco", &self.kco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kpcr {{ kre: {:?}, kco: {:?} }}", self.kre(), self.kco())
    }
}
#[doc = "Keypad Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpdr(pub u16);
impl Kpdr {
    #[doc = "Keypad Row Data"]
    #[must_use]
    #[inline(always)]
    pub const fn krd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Keypad Row Data"]
    #[inline(always)]
    pub const fn set_krd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Keypad Column Data"]
    #[must_use]
    #[inline(always)]
    pub const fn kcd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Keypad Column Data"]
    #[inline(always)]
    pub const fn set_kcd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Kpdr {
    #[inline(always)]
    fn default() -> Kpdr {
        Kpdr(0)
    }
}
impl core::fmt::Debug for Kpdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpdr")
            .field("krd", &self.krd())
            .field("kcd", &self.kcd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Kpdr {{ krd: {=u8:?}, kcd: {=u8:?} }}",
            self.krd(),
            self.kcd()
        )
    }
}
#[doc = "Keypad Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpsr(pub u16);
impl Kpsr {
    #[doc = "Keypad Key Depress"]
    #[must_use]
    #[inline(always)]
    pub const fn kpkd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Keypad Key Depress"]
    #[inline(always)]
    pub const fn set_kpkd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Keypad Key Release"]
    #[must_use]
    #[inline(always)]
    pub const fn kpkr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Keypad Key Release"]
    #[inline(always)]
    pub const fn set_kpkr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Key Depress Synchronizer Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn kdsc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Key Depress Synchronizer Clear"]
    #[inline(always)]
    pub const fn set_kdsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Key Release Synchronizer Set"]
    #[must_use]
    #[inline(always)]
    pub const fn krss(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Key Release Synchronizer Set"]
    #[inline(always)]
    pub const fn set_krss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Keypad Key Depress Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn kdie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Keypad Key Depress Interrupt Enable"]
    #[inline(always)]
    pub const fn set_kdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Keypad Release Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn krie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Keypad Release Interrupt Enable"]
    #[inline(always)]
    pub const fn set_krie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Kpsr {
    #[inline(always)]
    fn default() -> Kpsr {
        Kpsr(0)
    }
}
impl core::fmt::Debug for Kpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpsr")
            .field("kpkd", &self.kpkd())
            .field("kpkr", &self.kpkr())
            .field("kdsc", &self.kdsc())
            .field("krss", &self.krss())
            .field("kdie", &self.kdie())
            .field("krie", &self.krie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Kpsr {{ kpkd: {=bool:?}, kpkr: {=bool:?}, kdsc: {=bool:?}, krss: {=bool:?}, kdie: {=bool:?}, krie: {=bool:?} }}",
            self.kpkd(),
            self.kpkr(),
            self.kdsc(),
            self.krss(),
            self.kdie(),
            self.krie()
        )
    }
}
