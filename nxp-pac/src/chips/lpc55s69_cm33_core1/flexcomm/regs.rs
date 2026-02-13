#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pid(pub u32);
impl Pid {
    #[doc = "size aperture for the register port on the bus (APB or AHB)."]
    #[must_use]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "size aperture for the register port on the bus (APB or AHB)."]
    #[inline(always)]
    pub const fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pid {
    #[inline(always)]
    fn default() -> Pid {
        Pid(0)
    }
}
impl core::fmt::Debug for Pid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pid")
            .field("aperture", &self.aperture())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pid {{ aperture: {=u8:?}, minor_rev: {=u8:?}, major_rev: {=u8:?}, id: {=u16:?} }}",
            self.aperture(),
            self.minor_rev(),
            self.major_rev(),
            self.id()
        )
    }
}
#[doc = "Peripheral Select and Flexcomm ID register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselid(pub u32);
impl Pselid {
    #[doc = "Peripheral Select. This field is writable by software."]
    #[must_use]
    #[inline(always)]
    pub const fn persel(&self) -> super::vals::Persel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Persel::from_bits(val as u8)
    }
    #[doc = "Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub const fn set_persel(&mut self, val: super::vals::Persel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn usartpresent(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_usartpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn spipresent(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_spipresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cpresent(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_i2cpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn i2spresent(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_i2spresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flexcomm ID."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Flexcomm ID."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Pselid {
    #[inline(always)]
    fn default() -> Pselid {
        Pselid(0)
    }
}
impl core::fmt::Debug for Pselid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pselid")
            .field("persel", &self.persel())
            .field("lock", &self.lock())
            .field("usartpresent", &self.usartpresent())
            .field("spipresent", &self.spipresent())
            .field("i2cpresent", &self.i2cpresent())
            .field("i2spresent", &self.i2spresent())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pselid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pselid {{ persel: {:?}, lock: {=bool:?}, usartpresent: {=bool:?}, spipresent: {=bool:?}, i2cpresent: {=bool:?}, i2spresent: {=bool:?}, id: {=u32:?} }}",
            self.persel(),
            self.lock(),
            self.usartpresent(),
            self.spipresent(),
            self.i2cpresent(),
            self.i2spresent(),
            self.id()
        )
    }
}
