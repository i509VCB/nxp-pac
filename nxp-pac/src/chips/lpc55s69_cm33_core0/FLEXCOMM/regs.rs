#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PID(pub u32);
impl PID {
    #[doc = "size aperture for the register port on the bus (APB or AHB)."]
    #[must_use]
    #[inline(always)]
    pub const fn APERTURE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "size aperture for the register port on the bus (APB or AHB)."]
    #[inline(always)]
    pub const fn set_APERTURE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn set_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn set_MAJOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PID {
    #[inline(always)]
    fn default() -> PID {
        PID(0)
    }
}
impl core::fmt::Debug for PID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID")
            .field("APERTURE", &self.APERTURE())
            .field("MINOR_REV", &self.MINOR_REV())
            .field("MAJOR_REV", &self.MAJOR_REV())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PID {{ APERTURE: {=u8:?}, MINOR_REV: {=u8:?}, MAJOR_REV: {=u8:?}, ID: {=u16:?} }}",
            self.APERTURE(),
            self.MINOR_REV(),
            self.MAJOR_REV(),
            self.ID()
        )
    }
}
#[doc = "Peripheral Select and Flexcomm ID register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PSELID(pub u32);
impl PSELID {
    #[doc = "Peripheral Select. This field is writable by software."]
    #[must_use]
    #[inline(always)]
    pub const fn PERSEL(&self) -> super::vals::PERSEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::PERSEL::from_bits(val as u8)
    }
    #[doc = "Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub const fn set_PERSEL(&mut self, val: super::vals::PERSEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn USARTPRESENT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_USARTPRESENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn SPIPRESENT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_SPIPRESENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn I2CPRESENT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_I2CPRESENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn I2SPRESENT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn set_I2SPRESENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flexcomm ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Flexcomm ID."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for PSELID {
    #[inline(always)]
    fn default() -> PSELID {
        PSELID(0)
    }
}
impl core::fmt::Debug for PSELID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSELID")
            .field("PERSEL", &self.PERSEL())
            .field("LOCK", &self.LOCK())
            .field("USARTPRESENT", &self.USARTPRESENT())
            .field("SPIPRESENT", &self.SPIPRESENT())
            .field("I2CPRESENT", &self.I2CPRESENT())
            .field("I2SPRESENT", &self.I2SPRESENT())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PSELID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PSELID {{ PERSEL: {:?}, LOCK: {=bool:?}, USARTPRESENT: {=bool:?}, SPIPRESENT: {=bool:?}, I2CPRESENT: {=bool:?}, I2SPRESENT: {=bool:?}, ID: {=u32:?} }}",
            self.PERSEL(),
            self.LOCK(),
            self.USARTPRESENT(),
            self.SPIPRESENT(),
            self.I2CPRESENT(),
            self.I2SPRESENT(),
            self.ID()
        )
    }
}
