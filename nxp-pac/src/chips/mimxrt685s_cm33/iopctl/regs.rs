#[doc = "Special Registers (No GPIO Function)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc15I2cScl(pub u32);
impl Fc15I2cScl {
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> super::vals::Fc15I2cSclFsel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fc15I2cSclFsel::from_bits(val as u8)
    }
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: super::vals::Fc15I2cSclFsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[inline(always)]
    pub const fn set_pupdena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdsel(&self) -> super::vals::Fc15I2cSclPupdsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Fc15I2cSclPupdsel::from_bits(val as u8)
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[inline(always)]
    pub const fn set_pupdsel(&mut self, val: super::vals::Fc15I2cSclPupdsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Buffer Enable. ."]
    #[must_use]
    #[inline(always)]
    pub const fn ibena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Buffer Enable. ."]
    #[inline(always)]
    pub const fn set_ibena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slew Rate Control. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn slewrate(&self) -> super::vals::Fc15I2cSclSlewrate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Fc15I2cSclSlewrate::from_bits(val as u8)
    }
    #[doc = "Slew Rate Control. . ."]
    #[inline(always)]
    pub const fn set_slewrate(&mut self, val: super::vals::Fc15I2cSclSlewrate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Drive Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn fulldrive(&self) -> super::vals::Fc15I2cSclFulldrive {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fc15I2cSclFulldrive::from_bits(val as u8)
    }
    #[doc = "Drive Selector. . ."]
    #[inline(always)]
    pub const fn set_fulldrive(&mut self, val: super::vals::Fc15I2cSclFulldrive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Analog Mux Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn amena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Mux Enable. . ."]
    #[inline(always)]
    pub const fn set_amena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn odena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[inline(always)]
    pub const fn set_odena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input Invert Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn iiena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Invert Enable. . ."]
    #[inline(always)]
    pub const fn set_iiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Fc15I2cScl {
    #[inline(always)]
    fn default() -> Fc15I2cScl {
        Fc15I2cScl(0)
    }
}
impl core::fmt::Debug for Fc15I2cScl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc15I2cScl")
            .field("fsel", &self.fsel())
            .field("pupdena", &self.pupdena())
            .field("pupdsel", &self.pupdsel())
            .field("ibena", &self.ibena())
            .field("slewrate", &self.slewrate())
            .field("fulldrive", &self.fulldrive())
            .field("amena", &self.amena())
            .field("odena", &self.odena())
            .field("iiena", &self.iiena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc15I2cScl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fc15I2cScl {{ fsel: {:?}, pupdena: {=bool:?}, pupdsel: {:?}, ibena: {=bool:?}, slewrate: {:?}, fulldrive: {:?}, amena: {=bool:?}, odena: {=bool:?}, iiena: {=bool:?} }}",
            self.fsel(),
            self.pupdena(),
            self.pupdsel(),
            self.ibena(),
            self.slewrate(),
            self.fulldrive(),
            self.amena(),
            self.odena(),
            self.iiena()
        )
    }
}
#[doc = "Special Registers (No GPIO Function)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc15I2cSda(pub u32);
impl Fc15I2cSda {
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> super::vals::Fc15I2cSdaFsel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fc15I2cSdaFsel::from_bits(val as u8)
    }
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: super::vals::Fc15I2cSdaFsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[inline(always)]
    pub const fn set_pupdena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdsel(&self) -> super::vals::Fc15I2cSdaPupdsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Fc15I2cSdaPupdsel::from_bits(val as u8)
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[inline(always)]
    pub const fn set_pupdsel(&mut self, val: super::vals::Fc15I2cSdaPupdsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Buffer Enable. ."]
    #[must_use]
    #[inline(always)]
    pub const fn ibena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Buffer Enable. ."]
    #[inline(always)]
    pub const fn set_ibena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slew Rate Control. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn slewrate(&self) -> super::vals::Fc15I2cSdaSlewrate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Fc15I2cSdaSlewrate::from_bits(val as u8)
    }
    #[doc = "Slew Rate Control. . ."]
    #[inline(always)]
    pub const fn set_slewrate(&mut self, val: super::vals::Fc15I2cSdaSlewrate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Drive Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn fulldrive(&self) -> super::vals::Fc15I2cSdaFulldrive {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fc15I2cSdaFulldrive::from_bits(val as u8)
    }
    #[doc = "Drive Selector. . ."]
    #[inline(always)]
    pub const fn set_fulldrive(&mut self, val: super::vals::Fc15I2cSdaFulldrive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Analog Mux Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn amena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Mux Enable. . ."]
    #[inline(always)]
    pub const fn set_amena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn odena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[inline(always)]
    pub const fn set_odena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input Invert Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn iiena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Invert Enable. . ."]
    #[inline(always)]
    pub const fn set_iiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Fc15I2cSda {
    #[inline(always)]
    fn default() -> Fc15I2cSda {
        Fc15I2cSda(0)
    }
}
impl core::fmt::Debug for Fc15I2cSda {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc15I2cSda")
            .field("fsel", &self.fsel())
            .field("pupdena", &self.pupdena())
            .field("pupdsel", &self.pupdsel())
            .field("ibena", &self.ibena())
            .field("slewrate", &self.slewrate())
            .field("fulldrive", &self.fulldrive())
            .field("amena", &self.amena())
            .field("odena", &self.odena())
            .field("iiena", &self.iiena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc15I2cSda {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fc15I2cSda {{ fsel: {:?}, pupdena: {=bool:?}, pupdsel: {:?}, ibena: {=bool:?}, slewrate: {:?}, fulldrive: {:?}, amena: {=bool:?}, odena: {=bool:?}, iiena: {=bool:?} }}",
            self.fsel(),
            self.pupdena(),
            self.pupdsel(),
            self.ibena(),
            self.slewrate(),
            self.fulldrive(),
            self.amena(),
            self.odena(),
            self.iiena()
        )
    }
}
#[doc = "iop pad control register for port0 to port5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio(pub u32);
impl Pio {
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> super::vals::Fsel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fsel::from_bits(val as u8)
    }
    #[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: super::vals::Fsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pullup / Pulldown Enable. . ."]
    #[inline(always)]
    pub const fn set_pupdena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn pupdsel(&self) -> super::vals::Pupdsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pupdsel::from_bits(val as u8)
    }
    #[doc = "Pullup or Pulldown Selector. . ."]
    #[inline(always)]
    pub const fn set_pupdsel(&mut self, val: super::vals::Pupdsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Buffer Enable. ."]
    #[must_use]
    #[inline(always)]
    pub const fn ibena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Buffer Enable. ."]
    #[inline(always)]
    pub const fn set_ibena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slew Rate Control. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn slewrate(&self) -> super::vals::Slewrate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Slewrate::from_bits(val as u8)
    }
    #[doc = "Slew Rate Control. . ."]
    #[inline(always)]
    pub const fn set_slewrate(&mut self, val: super::vals::Slewrate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Drive Selector. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn fulldrive(&self) -> super::vals::Fulldrive {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fulldrive::from_bits(val as u8)
    }
    #[doc = "Drive Selector. . ."]
    #[inline(always)]
    pub const fn set_fulldrive(&mut self, val: super::vals::Fulldrive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Analog Mux Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn amena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Mux Enable. . ."]
    #[inline(always)]
    pub const fn set_amena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn odena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Pseudo Output Drain Enable. . ."]
    #[inline(always)]
    pub const fn set_odena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input Invert Enable. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn iiena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Invert Enable. . ."]
    #[inline(always)]
    pub const fn set_iiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Pio {
    #[inline(always)]
    fn default() -> Pio {
        Pio(0)
    }
}
impl core::fmt::Debug for Pio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio")
            .field("fsel", &self.fsel())
            .field("pupdena", &self.pupdena())
            .field("pupdsel", &self.pupdsel())
            .field("ibena", &self.ibena())
            .field("slewrate", &self.slewrate())
            .field("fulldrive", &self.fulldrive())
            .field("amena", &self.amena())
            .field("odena", &self.odena())
            .field("iiena", &self.iiena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio {{ fsel: {:?}, pupdena: {=bool:?}, pupdsel: {:?}, ibena: {=bool:?}, slewrate: {:?}, fulldrive: {:?}, amena: {=bool:?}, odena: {=bool:?}, iiena: {=bool:?} }}",
            self.fsel(),
            self.pupdena(),
            self.pupdsel(),
            self.ibena(),
            self.slewrate(),
            self.fulldrive(),
            self.amena(),
            self.odena(),
            self.iiena()
        )
    }
}
