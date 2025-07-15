#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Istat(pub u32);
impl Istat {
    #[doc = "UART TX Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn uarttx(&self) -> super::vals::Uarttx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Uarttx::from_bits(val as u8)
    }
    #[doc = "UART TX Interrupt"]
    #[inline(always)]
    pub const fn set_uarttx(&mut self, val: super::vals::Uarttx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "UART RX Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn uartrx(&self) -> super::vals::Uartrx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Uartrx::from_bits(val as u8)
    }
    #[doc = "UART RX Interrupt"]
    #[inline(always)]
    pub const fn set_uartrx(&mut self, val: super::vals::Uartrx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn spi(&self) -> super::vals::Spi {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Spi::from_bits(val as u8)
    }
    #[doc = "SPI Interrupt"]
    #[inline(always)]
    pub const fn set_spi(&mut self, val: super::vals::Spi) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Controller Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn i2cm(&self) -> super::vals::I2cm {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::I2cm::from_bits(val as u8)
    }
    #[doc = "I2C Controller Interrupt"]
    #[inline(always)]
    pub const fn set_i2cm(&mut self, val: super::vals::I2cm) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C Subordinate Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn i2cs(&self) -> super::vals::I2cs {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::I2cs::from_bits(val as u8)
    }
    #[doc = "I2C Subordinate Interrupt"]
    #[inline(always)]
    pub const fn set_i2cs(&mut self, val: super::vals::I2cs) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Istat {
    #[inline(always)]
    fn default() -> Istat {
        Istat(0)
    }
}
impl core::fmt::Debug for Istat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Istat")
            .field("uarttx", &self.uarttx())
            .field("uartrx", &self.uartrx())
            .field("spi", &self.spi())
            .field("i2cm", &self.i2cm())
            .field("i2cs", &self.i2cs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Istat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Istat {{ uarttx: {:?}, uartrx: {:?}, spi: {:?}, i2cm: {:?}, i2cs: {:?} }}",
            self.uarttx(),
            self.uartrx(),
            self.spi(),
            self.i2cm(),
            self.i2cs()
        )
    }
}
#[doc = "Peripheral Select and ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselid(pub u32);
impl Pselid {
    #[doc = "Peripheral Select"]
    #[must_use]
    #[inline(always)]
    pub const fn persel(&self) -> super::vals::Persel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Persel::from_bits(val as u8)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub const fn set_persel(&mut self, val: super::vals::Persel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UART Present"]
    #[must_use]
    #[inline(always)]
    pub const fn uartpresent(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "UART Present"]
    #[inline(always)]
    pub const fn set_uartpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spipresent(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Present"]
    #[inline(always)]
    pub const fn set_spipresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C Present"]
    #[must_use]
    #[inline(always)]
    pub const fn i2cpresent(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Present"]
    #[inline(always)]
    pub const fn set_i2cpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LP_FLEXCOMM interface ID"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "LP_FLEXCOMM interface ID"]
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
            .field("uartpresent", &self.uartpresent())
            .field("spipresent", &self.spipresent())
            .field("i2cpresent", &self.i2cpresent())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pselid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pselid {{ persel: {:?}, lock: {=bool:?}, uartpresent: {=bool:?}, spipresent: {=bool:?}, i2cpresent: {=bool:?}, id: {=u32:?} }}",
            self.persel(),
            self.lock(),
            self.uartpresent(),
            self.spipresent(),
            self.i2cpresent(),
            self.id()
        )
    }
}
