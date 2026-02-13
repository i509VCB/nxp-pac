#[doc = "Global Interrupt Control High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gichr(pub u32);
impl Gichr {
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe16(&self) -> super::vals::Giwe16 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Giwe16::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe16(&mut self, val: super::vals::Giwe16) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe17(&self) -> super::vals::Giwe17 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Giwe17::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe17(&mut self, val: super::vals::Giwe17) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe18(&self) -> super::vals::Giwe18 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Giwe18::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe18(&mut self, val: super::vals::Giwe18) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe19(&self) -> super::vals::Giwe19 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Giwe19::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe19(&mut self, val: super::vals::Giwe19) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe20(&self) -> super::vals::Giwe20 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Giwe20::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe20(&mut self, val: super::vals::Giwe20) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe21(&self) -> super::vals::Giwe21 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Giwe21::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe21(&mut self, val: super::vals::Giwe21) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe22(&self) -> super::vals::Giwe22 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Giwe22::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe22(&mut self, val: super::vals::Giwe22) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe23(&self) -> super::vals::Giwe23 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Giwe23::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe23(&mut self, val: super::vals::Giwe23) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe24(&self) -> super::vals::Giwe24 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Giwe24::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe24(&mut self, val: super::vals::Giwe24) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe25(&self) -> super::vals::Giwe25 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Giwe25::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe25(&mut self, val: super::vals::Giwe25) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe26(&self) -> super::vals::Giwe26 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Giwe26::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe26(&mut self, val: super::vals::Giwe26) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe27(&self) -> super::vals::Giwe27 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Giwe27::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe27(&mut self, val: super::vals::Giwe27) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe28(&self) -> super::vals::Giwe28 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Giwe28::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe28(&mut self, val: super::vals::Giwe28) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe29(&self) -> super::vals::Giwe29 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Giwe29::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe29(&mut self, val: super::vals::Giwe29) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe30(&self) -> super::vals::Giwe30 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Giwe30::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe30(&mut self, val: super::vals::Giwe30) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe31(&self) -> super::vals::Giwe31 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Giwe31::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe31(&mut self, val: super::vals::Giwe31) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gichr {
    #[inline(always)]
    fn default() -> Gichr {
        Gichr(0)
    }
}
impl core::fmt::Debug for Gichr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gichr")
            .field("giwe16", &self.giwe16())
            .field("giwe17", &self.giwe17())
            .field("giwe18", &self.giwe18())
            .field("giwe19", &self.giwe19())
            .field("giwe20", &self.giwe20())
            .field("giwe21", &self.giwe21())
            .field("giwe22", &self.giwe22())
            .field("giwe23", &self.giwe23())
            .field("giwe24", &self.giwe24())
            .field("giwe25", &self.giwe25())
            .field("giwe26", &self.giwe26())
            .field("giwe27", &self.giwe27())
            .field("giwe28", &self.giwe28())
            .field("giwe29", &self.giwe29())
            .field("giwe30", &self.giwe30())
            .field("giwe31", &self.giwe31())
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gichr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gichr {{ giwe16: {:?}, giwe17: {:?}, giwe18: {:?}, giwe19: {:?}, giwe20: {:?}, giwe21: {:?}, giwe22: {:?}, giwe23: {:?}, giwe24: {:?}, giwe25: {:?}, giwe26: {:?}, giwe27: {:?}, giwe28: {:?}, giwe29: {:?}, giwe30: {:?}, giwe31: {:?}, giwd: {=u16:?} }}",
            self.giwe16(),
            self.giwe17(),
            self.giwe18(),
            self.giwe19(),
            self.giwe20(),
            self.giwe21(),
            self.giwe22(),
            self.giwe23(),
            self.giwe24(),
            self.giwe25(),
            self.giwe26(),
            self.giwe27(),
            self.giwe28(),
            self.giwe29(),
            self.giwe30(),
            self.giwe31(),
            self.giwd()
        )
    }
}
#[doc = "Global Interrupt Control Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Giclr(pub u32);
impl Giclr {
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe0(&self) -> super::vals::Giwe0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Giwe0::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe0(&mut self, val: super::vals::Giwe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe1(&self) -> super::vals::Giwe1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Giwe1::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe1(&mut self, val: super::vals::Giwe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe2(&self) -> super::vals::Giwe2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Giwe2::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe2(&mut self, val: super::vals::Giwe2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe3(&self) -> super::vals::Giwe3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Giwe3::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe3(&mut self, val: super::vals::Giwe3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe4(&self) -> super::vals::Giwe4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Giwe4::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe4(&mut self, val: super::vals::Giwe4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe5(&self) -> super::vals::Giwe5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Giwe5::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe5(&mut self, val: super::vals::Giwe5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe6(&self) -> super::vals::Giwe6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Giwe6::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe6(&mut self, val: super::vals::Giwe6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe7(&self) -> super::vals::Giwe7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Giwe7::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe7(&mut self, val: super::vals::Giwe7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe8(&self) -> super::vals::Giwe8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Giwe8::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe8(&mut self, val: super::vals::Giwe8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe9(&self) -> super::vals::Giwe9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Giwe9::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe9(&mut self, val: super::vals::Giwe9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe10(&self) -> super::vals::Giwe10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Giwe10::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe10(&mut self, val: super::vals::Giwe10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe11(&self) -> super::vals::Giwe11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Giwe11::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe11(&mut self, val: super::vals::Giwe11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe12(&self) -> super::vals::Giwe12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Giwe12::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe12(&mut self, val: super::vals::Giwe12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe13(&self) -> super::vals::Giwe13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Giwe13::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe13(&mut self, val: super::vals::Giwe13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe14(&self) -> super::vals::Giwe14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Giwe14::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe14(&mut self, val: super::vals::Giwe14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe15(&self) -> super::vals::Giwe15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Giwe15::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe15(&mut self, val: super::vals::Giwe15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Giclr {
    #[inline(always)]
    fn default() -> Giclr {
        Giclr(0)
    }
}
impl core::fmt::Debug for Giclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Giclr")
            .field("giwe0", &self.giwe0())
            .field("giwe1", &self.giwe1())
            .field("giwe2", &self.giwe2())
            .field("giwe3", &self.giwe3())
            .field("giwe4", &self.giwe4())
            .field("giwe5", &self.giwe5())
            .field("giwe6", &self.giwe6())
            .field("giwe7", &self.giwe7())
            .field("giwe8", &self.giwe8())
            .field("giwe9", &self.giwe9())
            .field("giwe10", &self.giwe10())
            .field("giwe11", &self.giwe11())
            .field("giwe12", &self.giwe12())
            .field("giwe13", &self.giwe13())
            .field("giwe14", &self.giwe14())
            .field("giwe15", &self.giwe15())
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Giclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Giclr {{ giwe0: {:?}, giwe1: {:?}, giwe2: {:?}, giwe3: {:?}, giwe4: {:?}, giwe5: {:?}, giwe6: {:?}, giwe7: {:?}, giwe8: {:?}, giwe9: {:?}, giwe10: {:?}, giwe11: {:?}, giwe12: {:?}, giwe13: {:?}, giwe14: {:?}, giwe15: {:?}, giwd: {=u16:?} }}",
            self.giwe0(),
            self.giwe1(),
            self.giwe2(),
            self.giwe3(),
            self.giwe4(),
            self.giwe5(),
            self.giwe6(),
            self.giwe7(),
            self.giwe8(),
            self.giwe9(),
            self.giwe10(),
            self.giwe11(),
            self.giwe12(),
            self.giwe13(),
            self.giwe14(),
            self.giwe15(),
            self.giwd()
        )
    }
}
#[doc = "Interrupt Control index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("irqc", &self.irqc())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ irqc: {:?}, isf: {:?} }}",
            self.irqc(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Status Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfr0(pub u32);
impl Isfr0 {
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf0(&self) -> super::vals::Isf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Isf0::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf0(&mut self, val: super::vals::Isf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf1(&self) -> super::vals::Isf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Isf1::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf1(&mut self, val: super::vals::Isf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf2(&self) -> super::vals::Isf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Isf2::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf2(&mut self, val: super::vals::Isf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf3(&self) -> super::vals::Isf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Isf3::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf3(&mut self, val: super::vals::Isf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf4(&self) -> super::vals::Isf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Isf4::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf4(&mut self, val: super::vals::Isf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf5(&self) -> super::vals::Isf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Isf5::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf5(&mut self, val: super::vals::Isf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf6(&self) -> super::vals::Isf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Isf6::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf6(&mut self, val: super::vals::Isf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf7(&self) -> super::vals::Isf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Isf7::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf7(&mut self, val: super::vals::Isf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf8(&self) -> super::vals::Isf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Isf8::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf8(&mut self, val: super::vals::Isf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf9(&self) -> super::vals::Isf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Isf9::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf9(&mut self, val: super::vals::Isf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf10(&self) -> super::vals::Isf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Isf10::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf10(&mut self, val: super::vals::Isf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf11(&self) -> super::vals::Isf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Isf11::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf11(&mut self, val: super::vals::Isf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf12(&self) -> super::vals::Isf12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Isf12::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf12(&mut self, val: super::vals::Isf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf13(&self) -> super::vals::Isf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Isf13::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf13(&mut self, val: super::vals::Isf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf14(&self) -> super::vals::Isf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Isf14::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf14(&mut self, val: super::vals::Isf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf15(&self) -> super::vals::Isf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Isf15::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf15(&mut self, val: super::vals::Isf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf16(&self) -> super::vals::Isf16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Isf16::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf16(&mut self, val: super::vals::Isf16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf17(&self) -> super::vals::Isf17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Isf17::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf17(&mut self, val: super::vals::Isf17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf18(&self) -> super::vals::Isf18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Isf18::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf18(&mut self, val: super::vals::Isf18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf19(&self) -> super::vals::Isf19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Isf19::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf19(&mut self, val: super::vals::Isf19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf20(&self) -> super::vals::Isf20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Isf20::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf20(&mut self, val: super::vals::Isf20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf21(&self) -> super::vals::Isf21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Isf21::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf21(&mut self, val: super::vals::Isf21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf22(&self) -> super::vals::Isf22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Isf22::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf22(&mut self, val: super::vals::Isf22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf23(&self) -> super::vals::Isf23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Isf23::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf23(&mut self, val: super::vals::Isf23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf24(&self) -> super::vals::Isf24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Isf24::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf24(&mut self, val: super::vals::Isf24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf25(&self) -> super::vals::Isf25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Isf25::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf25(&mut self, val: super::vals::Isf25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf26(&self) -> super::vals::Isf26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Isf26::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf26(&mut self, val: super::vals::Isf26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf27(&self) -> super::vals::Isf27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Isf27::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf27(&mut self, val: super::vals::Isf27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf28(&self) -> super::vals::Isf28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Isf28::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf28(&mut self, val: super::vals::Isf28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf29(&self) -> super::vals::Isf29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Isf29::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf29(&mut self, val: super::vals::Isf29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf30(&self) -> super::vals::Isf30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Isf30::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf30(&mut self, val: super::vals::Isf30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf31(&self) -> super::vals::Isf31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Isf31::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf31(&mut self, val: super::vals::Isf31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Isfr0 {
    #[inline(always)]
    fn default() -> Isfr0 {
        Isfr0(0)
    }
}
impl core::fmt::Debug for Isfr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfr0")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .field("isf16", &self.isf16())
            .field("isf17", &self.isf17())
            .field("isf18", &self.isf18())
            .field("isf19", &self.isf19())
            .field("isf20", &self.isf20())
            .field("isf21", &self.isf21())
            .field("isf22", &self.isf22())
            .field("isf23", &self.isf23())
            .field("isf24", &self.isf24())
            .field("isf25", &self.isf25())
            .field("isf26", &self.isf26())
            .field("isf27", &self.isf27())
            .field("isf28", &self.isf28())
            .field("isf29", &self.isf29())
            .field("isf30", &self.isf30())
            .field("isf31", &self.isf31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isfr0 {{ isf0: {:?}, isf1: {:?}, isf2: {:?}, isf3: {:?}, isf4: {:?}, isf5: {:?}, isf6: {:?}, isf7: {:?}, isf8: {:?}, isf9: {:?}, isf10: {:?}, isf11: {:?}, isf12: {:?}, isf13: {:?}, isf14: {:?}, isf15: {:?}, isf16: {:?}, isf17: {:?}, isf18: {:?}, isf19: {:?}, isf20: {:?}, isf21: {:?}, isf22: {:?}, isf23: {:?}, isf24: {:?}, isf25: {:?}, isf26: {:?}, isf27: {:?}, isf28: {:?}, isf29: {:?}, isf30: {:?}, isf31: {:?} }}",
            self.isf0(),
            self.isf1(),
            self.isf2(),
            self.isf3(),
            self.isf4(),
            self.isf5(),
            self.isf6(),
            self.isf7(),
            self.isf8(),
            self.isf9(),
            self.isf10(),
            self.isf11(),
            self.isf12(),
            self.isf13(),
            self.isf14(),
            self.isf15(),
            self.isf16(),
            self.isf17(),
            self.isf18(),
            self.isf19(),
            self.isf20(),
            self.isf21(),
            self.isf22(),
            self.isf23(),
            self.isf24(),
            self.isf25(),
            self.isf26(),
            self.isf27(),
            self.isf28(),
            self.isf29(),
            self.isf30(),
            self.isf31()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Interrupt Number"]
    #[must_use]
    #[inline(always)]
    pub const fn irqnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt Number"]
    #[inline(always)]
    pub const fn set_irqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            .field("irqnum", &self.irqnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ irqnum: {=u8:?} }}", self.irqnum())
    }
}
#[doc = "Port Clear Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco(&self, n: usize) -> super::vals::Ptco {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Ptco::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco(&mut self, n: usize, val: super::vals::Ptco) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pcor {
    #[inline(always)]
    fn default() -> Pcor {
        Pcor(0)
    }
}
impl core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcor")
            .field("ptco[0]", &self.ptco(0usize))
            .field("ptco[1]", &self.ptco(1usize))
            .field("ptco[2]", &self.ptco(2usize))
            .field("ptco[3]", &self.ptco(3usize))
            .field("ptco[4]", &self.ptco(4usize))
            .field("ptco[5]", &self.ptco(5usize))
            .field("ptco[6]", &self.ptco(6usize))
            .field("ptco[7]", &self.ptco(7usize))
            .field("ptco[8]", &self.ptco(8usize))
            .field("ptco[9]", &self.ptco(9usize))
            .field("ptco[10]", &self.ptco(10usize))
            .field("ptco[11]", &self.ptco(11usize))
            .field("ptco[12]", &self.ptco(12usize))
            .field("ptco[13]", &self.ptco(13usize))
            .field("ptco[14]", &self.ptco(14usize))
            .field("ptco[15]", &self.ptco(15usize))
            .field("ptco[16]", &self.ptco(16usize))
            .field("ptco[17]", &self.ptco(17usize))
            .field("ptco[18]", &self.ptco(18usize))
            .field("ptco[19]", &self.ptco(19usize))
            .field("ptco[20]", &self.ptco(20usize))
            .field("ptco[21]", &self.ptco(21usize))
            .field("ptco[22]", &self.ptco(22usize))
            .field("ptco[23]", &self.ptco(23usize))
            .field("ptco[24]", &self.ptco(24usize))
            .field("ptco[25]", &self.ptco(25usize))
            .field("ptco[26]", &self.ptco(26usize))
            .field("ptco[27]", &self.ptco(27usize))
            .field("ptco[28]", &self.ptco(28usize))
            .field("ptco[29]", &self.ptco(29usize))
            .field("ptco[30]", &self.ptco(30usize))
            .field("ptco[31]", &self.ptco(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcor {{ ptco[0]: {:?}, ptco[1]: {:?}, ptco[2]: {:?}, ptco[3]: {:?}, ptco[4]: {:?}, ptco[5]: {:?}, ptco[6]: {:?}, ptco[7]: {:?}, ptco[8]: {:?}, ptco[9]: {:?}, ptco[10]: {:?}, ptco[11]: {:?}, ptco[12]: {:?}, ptco[13]: {:?}, ptco[14]: {:?}, ptco[15]: {:?}, ptco[16]: {:?}, ptco[17]: {:?}, ptco[18]: {:?}, ptco[19]: {:?}, ptco[20]: {:?}, ptco[21]: {:?}, ptco[22]: {:?}, ptco[23]: {:?}, ptco[24]: {:?}, ptco[25]: {:?}, ptco[26]: {:?}, ptco[27]: {:?}, ptco[28]: {:?}, ptco[29]: {:?}, ptco[30]: {:?}, ptco[31]: {:?} }}",
            self.ptco(0usize),
            self.ptco(1usize),
            self.ptco(2usize),
            self.ptco(3usize),
            self.ptco(4usize),
            self.ptco(5usize),
            self.ptco(6usize),
            self.ptco(7usize),
            self.ptco(8usize),
            self.ptco(9usize),
            self.ptco(10usize),
            self.ptco(11usize),
            self.ptco(12usize),
            self.ptco(13usize),
            self.ptco(14usize),
            self.ptco(15usize),
            self.ptco(16usize),
            self.ptco(17usize),
            self.ptco(18usize),
            self.ptco(19usize),
            self.ptco(20usize),
            self.ptco(21usize),
            self.ptco(22usize),
            self.ptco(23usize),
            self.ptco(24usize),
            self.ptco(25usize),
            self.ptco(26usize),
            self.ptco(27usize),
            self.ptco(28usize),
            self.ptco(29usize),
            self.ptco(30usize),
            self.ptco(31usize)
        )
    }
}
#[doc = "Port Data Direction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd(&self, n: usize) -> super::vals::Pdd {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Pdd::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd(&mut self, n: usize, val: super::vals::Pdd) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pddr {
    #[inline(always)]
    fn default() -> Pddr {
        Pddr(0)
    }
}
impl core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pddr")
            .field("pdd[0]", &self.pdd(0usize))
            .field("pdd[1]", &self.pdd(1usize))
            .field("pdd[2]", &self.pdd(2usize))
            .field("pdd[3]", &self.pdd(3usize))
            .field("pdd[4]", &self.pdd(4usize))
            .field("pdd[5]", &self.pdd(5usize))
            .field("pdd[6]", &self.pdd(6usize))
            .field("pdd[7]", &self.pdd(7usize))
            .field("pdd[8]", &self.pdd(8usize))
            .field("pdd[9]", &self.pdd(9usize))
            .field("pdd[10]", &self.pdd(10usize))
            .field("pdd[11]", &self.pdd(11usize))
            .field("pdd[12]", &self.pdd(12usize))
            .field("pdd[13]", &self.pdd(13usize))
            .field("pdd[14]", &self.pdd(14usize))
            .field("pdd[15]", &self.pdd(15usize))
            .field("pdd[16]", &self.pdd(16usize))
            .field("pdd[17]", &self.pdd(17usize))
            .field("pdd[18]", &self.pdd(18usize))
            .field("pdd[19]", &self.pdd(19usize))
            .field("pdd[20]", &self.pdd(20usize))
            .field("pdd[21]", &self.pdd(21usize))
            .field("pdd[22]", &self.pdd(22usize))
            .field("pdd[23]", &self.pdd(23usize))
            .field("pdd[24]", &self.pdd(24usize))
            .field("pdd[25]", &self.pdd(25usize))
            .field("pdd[26]", &self.pdd(26usize))
            .field("pdd[27]", &self.pdd(27usize))
            .field("pdd[28]", &self.pdd(28usize))
            .field("pdd[29]", &self.pdd(29usize))
            .field("pdd[30]", &self.pdd(30usize))
            .field("pdd[31]", &self.pdd(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pddr {{ pdd[0]: {:?}, pdd[1]: {:?}, pdd[2]: {:?}, pdd[3]: {:?}, pdd[4]: {:?}, pdd[5]: {:?}, pdd[6]: {:?}, pdd[7]: {:?}, pdd[8]: {:?}, pdd[9]: {:?}, pdd[10]: {:?}, pdd[11]: {:?}, pdd[12]: {:?}, pdd[13]: {:?}, pdd[14]: {:?}, pdd[15]: {:?}, pdd[16]: {:?}, pdd[17]: {:?}, pdd[18]: {:?}, pdd[19]: {:?}, pdd[20]: {:?}, pdd[21]: {:?}, pdd[22]: {:?}, pdd[23]: {:?}, pdd[24]: {:?}, pdd[25]: {:?}, pdd[26]: {:?}, pdd[27]: {:?}, pdd[28]: {:?}, pdd[29]: {:?}, pdd[30]: {:?}, pdd[31]: {:?} }}",
            self.pdd(0usize),
            self.pdd(1usize),
            self.pdd(2usize),
            self.pdd(3usize),
            self.pdd(4usize),
            self.pdd(5usize),
            self.pdd(6usize),
            self.pdd(7usize),
            self.pdd(8usize),
            self.pdd(9usize),
            self.pdd(10usize),
            self.pdd(11usize),
            self.pdd(12usize),
            self.pdd(13usize),
            self.pdd(14usize),
            self.pdd(15usize),
            self.pdd(16usize),
            self.pdd(17usize),
            self.pdd(18usize),
            self.pdd(19usize),
            self.pdd(20usize),
            self.pdd(21usize),
            self.pdd(22usize),
            self.pdd(23usize),
            self.pdd(24usize),
            self.pdd(25usize),
            self.pdd(26usize),
            self.pdd(27usize),
            self.pdd(28usize),
            self.pdd(29usize),
            self.pdd(30usize),
            self.pdd(31usize)
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdir {
    #[inline(always)]
    fn default() -> Pdir {
        Pdir(0)
    }
}
impl core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdir")
            .field("pdi[0]", &self.pdi(0usize))
            .field("pdi[1]", &self.pdi(1usize))
            .field("pdi[2]", &self.pdi(2usize))
            .field("pdi[3]", &self.pdi(3usize))
            .field("pdi[4]", &self.pdi(4usize))
            .field("pdi[5]", &self.pdi(5usize))
            .field("pdi[6]", &self.pdi(6usize))
            .field("pdi[7]", &self.pdi(7usize))
            .field("pdi[8]", &self.pdi(8usize))
            .field("pdi[9]", &self.pdi(9usize))
            .field("pdi[10]", &self.pdi(10usize))
            .field("pdi[11]", &self.pdi(11usize))
            .field("pdi[12]", &self.pdi(12usize))
            .field("pdi[13]", &self.pdi(13usize))
            .field("pdi[14]", &self.pdi(14usize))
            .field("pdi[15]", &self.pdi(15usize))
            .field("pdi[16]", &self.pdi(16usize))
            .field("pdi[17]", &self.pdi(17usize))
            .field("pdi[18]", &self.pdi(18usize))
            .field("pdi[19]", &self.pdi(19usize))
            .field("pdi[20]", &self.pdi(20usize))
            .field("pdi[21]", &self.pdi(21usize))
            .field("pdi[22]", &self.pdi(22usize))
            .field("pdi[23]", &self.pdi(23usize))
            .field("pdi[24]", &self.pdi(24usize))
            .field("pdi[25]", &self.pdi(25usize))
            .field("pdi[26]", &self.pdi(26usize))
            .field("pdi[27]", &self.pdi(27usize))
            .field("pdi[28]", &self.pdi(28usize))
            .field("pdi[29]", &self.pdi(29usize))
            .field("pdi[30]", &self.pdi(30usize))
            .field("pdi[31]", &self.pdi(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdir {{ pdi[0]: {=bool:?}, pdi[1]: {=bool:?}, pdi[2]: {=bool:?}, pdi[3]: {=bool:?}, pdi[4]: {=bool:?}, pdi[5]: {=bool:?}, pdi[6]: {=bool:?}, pdi[7]: {=bool:?}, pdi[8]: {=bool:?}, pdi[9]: {=bool:?}, pdi[10]: {=bool:?}, pdi[11]: {=bool:?}, pdi[12]: {=bool:?}, pdi[13]: {=bool:?}, pdi[14]: {=bool:?}, pdi[15]: {=bool:?}, pdi[16]: {=bool:?}, pdi[17]: {=bool:?}, pdi[18]: {=bool:?}, pdi[19]: {=bool:?}, pdi[20]: {=bool:?}, pdi[21]: {=bool:?}, pdi[22]: {=bool:?}, pdi[23]: {=bool:?}, pdi[24]: {=bool:?}, pdi[25]: {=bool:?}, pdi[26]: {=bool:?}, pdi[27]: {=bool:?}, pdi[28]: {=bool:?}, pdi[29]: {=bool:?}, pdi[30]: {=bool:?}, pdi[31]: {=bool:?} }}",
            self.pdi(0usize),
            self.pdi(1usize),
            self.pdi(2usize),
            self.pdi(3usize),
            self.pdi(4usize),
            self.pdi(5usize),
            self.pdi(6usize),
            self.pdi(7usize),
            self.pdi(8usize),
            self.pdi(9usize),
            self.pdi(10usize),
            self.pdi(11usize),
            self.pdi(12usize),
            self.pdi(13usize),
            self.pdi(14usize),
            self.pdi(15usize),
            self.pdi(16usize),
            self.pdi(17usize),
            self.pdi(18usize),
            self.pdi(19usize),
            self.pdi(20usize),
            self.pdi(21usize),
            self.pdi(22usize),
            self.pdi(23usize),
            self.pdi(24usize),
            self.pdi(25usize),
            self.pdi(26usize),
            self.pdi(27usize),
            self.pdi(28usize),
            self.pdi(29usize),
            self.pdi(30usize),
            self.pdi(31usize)
        )
    }
}
#[doc = "Port Data Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdor {
    #[inline(always)]
    fn default() -> Pdor {
        Pdor(0)
    }
}
impl core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdor")
            .field("pdo[0]", &self.pdo(0usize))
            .field("pdo[1]", &self.pdo(1usize))
            .field("pdo[2]", &self.pdo(2usize))
            .field("pdo[3]", &self.pdo(3usize))
            .field("pdo[4]", &self.pdo(4usize))
            .field("pdo[5]", &self.pdo(5usize))
            .field("pdo[6]", &self.pdo(6usize))
            .field("pdo[7]", &self.pdo(7usize))
            .field("pdo[8]", &self.pdo(8usize))
            .field("pdo[9]", &self.pdo(9usize))
            .field("pdo[10]", &self.pdo(10usize))
            .field("pdo[11]", &self.pdo(11usize))
            .field("pdo[12]", &self.pdo(12usize))
            .field("pdo[13]", &self.pdo(13usize))
            .field("pdo[14]", &self.pdo(14usize))
            .field("pdo[15]", &self.pdo(15usize))
            .field("pdo[16]", &self.pdo(16usize))
            .field("pdo[17]", &self.pdo(17usize))
            .field("pdo[18]", &self.pdo(18usize))
            .field("pdo[19]", &self.pdo(19usize))
            .field("pdo[20]", &self.pdo(20usize))
            .field("pdo[21]", &self.pdo(21usize))
            .field("pdo[22]", &self.pdo(22usize))
            .field("pdo[23]", &self.pdo(23usize))
            .field("pdo[24]", &self.pdo(24usize))
            .field("pdo[25]", &self.pdo(25usize))
            .field("pdo[26]", &self.pdo(26usize))
            .field("pdo[27]", &self.pdo(27usize))
            .field("pdo[28]", &self.pdo(28usize))
            .field("pdo[29]", &self.pdo(29usize))
            .field("pdo[30]", &self.pdo(30usize))
            .field("pdo[31]", &self.pdo(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdor {{ pdo[0]: {=bool:?}, pdo[1]: {=bool:?}, pdo[2]: {=bool:?}, pdo[3]: {=bool:?}, pdo[4]: {=bool:?}, pdo[5]: {=bool:?}, pdo[6]: {=bool:?}, pdo[7]: {=bool:?}, pdo[8]: {=bool:?}, pdo[9]: {=bool:?}, pdo[10]: {=bool:?}, pdo[11]: {=bool:?}, pdo[12]: {=bool:?}, pdo[13]: {=bool:?}, pdo[14]: {=bool:?}, pdo[15]: {=bool:?}, pdo[16]: {=bool:?}, pdo[17]: {=bool:?}, pdo[18]: {=bool:?}, pdo[19]: {=bool:?}, pdo[20]: {=bool:?}, pdo[21]: {=bool:?}, pdo[22]: {=bool:?}, pdo[23]: {=bool:?}, pdo[24]: {=bool:?}, pdo[25]: {=bool:?}, pdo[26]: {=bool:?}, pdo[27]: {=bool:?}, pdo[28]: {=bool:?}, pdo[29]: {=bool:?}, pdo[30]: {=bool:?}, pdo[31]: {=bool:?} }}",
            self.pdo(0usize),
            self.pdo(1usize),
            self.pdo(2usize),
            self.pdo(3usize),
            self.pdo(4usize),
            self.pdo(5usize),
            self.pdo(6usize),
            self.pdo(7usize),
            self.pdo(8usize),
            self.pdo(9usize),
            self.pdo(10usize),
            self.pdo(11usize),
            self.pdo(12usize),
            self.pdo(13usize),
            self.pdo(14usize),
            self.pdo(15usize),
            self.pdo(16usize),
            self.pdo(17usize),
            self.pdo(18usize),
            self.pdo(19usize),
            self.pdo(20usize),
            self.pdo(21usize),
            self.pdo(22usize),
            self.pdo(23usize),
            self.pdo(24usize),
            self.pdo(25usize),
            self.pdo(26usize),
            self.pdo(27usize),
            self.pdo(28usize),
            self.pdo(29usize),
            self.pdo(30usize),
            self.pdo(31usize)
        )
    }
}
#[doc = "Pin Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u8);
impl Pdr {
    #[doc = "Pin Data (I/O)"]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> super::vals::Pd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pd::from_bits(val as u8)
    }
    #[doc = "Pin Data (I/O)"]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: super::vals::Pd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr").field("pd", &self.pd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pdr {{ pd: {:?} }}", self.pd())
    }
}
#[doc = "Port Input Disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self, n: usize) -> super::vals::Pid {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Pid::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid(&mut self, n: usize, val: super::vals::Pid) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        Pidr(0)
    }
}
impl core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pidr")
            .field("pid[0]", &self.pid(0usize))
            .field("pid[1]", &self.pid(1usize))
            .field("pid[2]", &self.pid(2usize))
            .field("pid[3]", &self.pid(3usize))
            .field("pid[4]", &self.pid(4usize))
            .field("pid[5]", &self.pid(5usize))
            .field("pid[6]", &self.pid(6usize))
            .field("pid[7]", &self.pid(7usize))
            .field("pid[8]", &self.pid(8usize))
            .field("pid[9]", &self.pid(9usize))
            .field("pid[10]", &self.pid(10usize))
            .field("pid[11]", &self.pid(11usize))
            .field("pid[12]", &self.pid(12usize))
            .field("pid[13]", &self.pid(13usize))
            .field("pid[14]", &self.pid(14usize))
            .field("pid[15]", &self.pid(15usize))
            .field("pid[16]", &self.pid(16usize))
            .field("pid[17]", &self.pid(17usize))
            .field("pid[18]", &self.pid(18usize))
            .field("pid[19]", &self.pid(19usize))
            .field("pid[20]", &self.pid(20usize))
            .field("pid[21]", &self.pid(21usize))
            .field("pid[22]", &self.pid(22usize))
            .field("pid[23]", &self.pid(23usize))
            .field("pid[24]", &self.pid(24usize))
            .field("pid[25]", &self.pid(25usize))
            .field("pid[26]", &self.pid(26usize))
            .field("pid[27]", &self.pid(27usize))
            .field("pid[28]", &self.pid(28usize))
            .field("pid[29]", &self.pid(29usize))
            .field("pid[30]", &self.pid(30usize))
            .field("pid[31]", &self.pid(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pidr {{ pid[0]: {:?}, pid[1]: {:?}, pid[2]: {:?}, pid[3]: {:?}, pid[4]: {:?}, pid[5]: {:?}, pid[6]: {:?}, pid[7]: {:?}, pid[8]: {:?}, pid[9]: {:?}, pid[10]: {:?}, pid[11]: {:?}, pid[12]: {:?}, pid[13]: {:?}, pid[14]: {:?}, pid[15]: {:?}, pid[16]: {:?}, pid[17]: {:?}, pid[18]: {:?}, pid[19]: {:?}, pid[20]: {:?}, pid[21]: {:?}, pid[22]: {:?}, pid[23]: {:?}, pid[24]: {:?}, pid[25]: {:?}, pid[26]: {:?}, pid[27]: {:?}, pid[28]: {:?}, pid[29]: {:?}, pid[30]: {:?}, pid[31]: {:?} }}",
            self.pid(0usize),
            self.pid(1usize),
            self.pid(2usize),
            self.pid(3usize),
            self.pid(4usize),
            self.pid(5usize),
            self.pid(6usize),
            self.pid(7usize),
            self.pid(8usize),
            self.pid(9usize),
            self.pid(10usize),
            self.pid(11usize),
            self.pid(12usize),
            self.pid(13usize),
            self.pid(14usize),
            self.pid(15usize),
            self.pid(16usize),
            self.pid(17usize),
            self.pid(18usize),
            self.pid(19usize),
            self.pid(20usize),
            self.pid(21usize),
            self.pid(22usize),
            self.pid(23usize),
            self.pid(24usize),
            self.pid(25usize),
            self.pid(26usize),
            self.pid(27usize),
            self.pid(28usize),
            self.pid(29usize),
            self.pid(30usize),
            self.pid(31usize)
        )
    }
}
#[doc = "Port Set Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso(&self, n: usize) -> super::vals::Ptso {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Ptso::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso(&mut self, n: usize, val: super::vals::Ptso) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Psor {
    #[inline(always)]
    fn default() -> Psor {
        Psor(0)
    }
}
impl core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psor")
            .field("ptso[0]", &self.ptso(0usize))
            .field("ptso[1]", &self.ptso(1usize))
            .field("ptso[2]", &self.ptso(2usize))
            .field("ptso[3]", &self.ptso(3usize))
            .field("ptso[4]", &self.ptso(4usize))
            .field("ptso[5]", &self.ptso(5usize))
            .field("ptso[6]", &self.ptso(6usize))
            .field("ptso[7]", &self.ptso(7usize))
            .field("ptso[8]", &self.ptso(8usize))
            .field("ptso[9]", &self.ptso(9usize))
            .field("ptso[10]", &self.ptso(10usize))
            .field("ptso[11]", &self.ptso(11usize))
            .field("ptso[12]", &self.ptso(12usize))
            .field("ptso[13]", &self.ptso(13usize))
            .field("ptso[14]", &self.ptso(14usize))
            .field("ptso[15]", &self.ptso(15usize))
            .field("ptso[16]", &self.ptso(16usize))
            .field("ptso[17]", &self.ptso(17usize))
            .field("ptso[18]", &self.ptso(18usize))
            .field("ptso[19]", &self.ptso(19usize))
            .field("ptso[20]", &self.ptso(20usize))
            .field("ptso[21]", &self.ptso(21usize))
            .field("ptso[22]", &self.ptso(22usize))
            .field("ptso[23]", &self.ptso(23usize))
            .field("ptso[24]", &self.ptso(24usize))
            .field("ptso[25]", &self.ptso(25usize))
            .field("ptso[26]", &self.ptso(26usize))
            .field("ptso[27]", &self.ptso(27usize))
            .field("ptso[28]", &self.ptso(28usize))
            .field("ptso[29]", &self.ptso(29usize))
            .field("ptso[30]", &self.ptso(30usize))
            .field("ptso[31]", &self.ptso(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psor {{ ptso[0]: {:?}, ptso[1]: {:?}, ptso[2]: {:?}, ptso[3]: {:?}, ptso[4]: {:?}, ptso[5]: {:?}, ptso[6]: {:?}, ptso[7]: {:?}, ptso[8]: {:?}, ptso[9]: {:?}, ptso[10]: {:?}, ptso[11]: {:?}, ptso[12]: {:?}, ptso[13]: {:?}, ptso[14]: {:?}, ptso[15]: {:?}, ptso[16]: {:?}, ptso[17]: {:?}, ptso[18]: {:?}, ptso[19]: {:?}, ptso[20]: {:?}, ptso[21]: {:?}, ptso[22]: {:?}, ptso[23]: {:?}, ptso[24]: {:?}, ptso[25]: {:?}, ptso[26]: {:?}, ptso[27]: {:?}, ptso[28]: {:?}, ptso[29]: {:?}, ptso[30]: {:?}, ptso[31]: {:?} }}",
            self.ptso(0usize),
            self.ptso(1usize),
            self.ptso(2usize),
            self.ptso(3usize),
            self.ptso(4usize),
            self.ptso(5usize),
            self.ptso(6usize),
            self.ptso(7usize),
            self.ptso(8usize),
            self.ptso(9usize),
            self.ptso(10usize),
            self.ptso(11usize),
            self.ptso(12usize),
            self.ptso(13usize),
            self.ptso(14usize),
            self.ptso(15usize),
            self.ptso(16usize),
            self.ptso(17usize),
            self.ptso(18usize),
            self.ptso(19usize),
            self.ptso(20usize),
            self.ptso(21usize),
            self.ptso(22usize),
            self.ptso(23usize),
            self.ptso(24usize),
            self.ptso(25usize),
            self.ptso(26usize),
            self.ptso(27usize),
            self.ptso(28usize),
            self.ptso(29usize),
            self.ptso(30usize),
            self.ptso(31usize)
        )
    }
}
#[doc = "Port Toggle Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ptor {
    #[inline(always)]
    fn default() -> Ptor {
        Ptor(0)
    }
}
impl core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptor")
            .field("ptto[0]", &self.ptto(0usize))
            .field("ptto[1]", &self.ptto(1usize))
            .field("ptto[2]", &self.ptto(2usize))
            .field("ptto[3]", &self.ptto(3usize))
            .field("ptto[4]", &self.ptto(4usize))
            .field("ptto[5]", &self.ptto(5usize))
            .field("ptto[6]", &self.ptto(6usize))
            .field("ptto[7]", &self.ptto(7usize))
            .field("ptto[8]", &self.ptto(8usize))
            .field("ptto[9]", &self.ptto(9usize))
            .field("ptto[10]", &self.ptto(10usize))
            .field("ptto[11]", &self.ptto(11usize))
            .field("ptto[12]", &self.ptto(12usize))
            .field("ptto[13]", &self.ptto(13usize))
            .field("ptto[14]", &self.ptto(14usize))
            .field("ptto[15]", &self.ptto(15usize))
            .field("ptto[16]", &self.ptto(16usize))
            .field("ptto[17]", &self.ptto(17usize))
            .field("ptto[18]", &self.ptto(18usize))
            .field("ptto[19]", &self.ptto(19usize))
            .field("ptto[20]", &self.ptto(20usize))
            .field("ptto[21]", &self.ptto(21usize))
            .field("ptto[22]", &self.ptto(22usize))
            .field("ptto[23]", &self.ptto(23usize))
            .field("ptto[24]", &self.ptto(24usize))
            .field("ptto[25]", &self.ptto(25usize))
            .field("ptto[26]", &self.ptto(26usize))
            .field("ptto[27]", &self.ptto(27usize))
            .field("ptto[28]", &self.ptto(28usize))
            .field("ptto[29]", &self.ptto(29usize))
            .field("ptto[30]", &self.ptto(30usize))
            .field("ptto[31]", &self.ptto(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ptor {{ ptto[0]: {=bool:?}, ptto[1]: {=bool:?}, ptto[2]: {=bool:?}, ptto[3]: {=bool:?}, ptto[4]: {=bool:?}, ptto[5]: {=bool:?}, ptto[6]: {=bool:?}, ptto[7]: {=bool:?}, ptto[8]: {=bool:?}, ptto[9]: {=bool:?}, ptto[10]: {=bool:?}, ptto[11]: {=bool:?}, ptto[12]: {=bool:?}, ptto[13]: {=bool:?}, ptto[14]: {=bool:?}, ptto[15]: {=bool:?}, ptto[16]: {=bool:?}, ptto[17]: {=bool:?}, ptto[18]: {=bool:?}, ptto[19]: {=bool:?}, ptto[20]: {=bool:?}, ptto[21]: {=bool:?}, ptto[22]: {=bool:?}, ptto[23]: {=bool:?}, ptto[24]: {=bool:?}, ptto[25]: {=bool:?}, ptto[26]: {=bool:?}, ptto[27]: {=bool:?}, ptto[28]: {=bool:?}, ptto[29]: {=bool:?}, ptto[30]: {=bool:?}, ptto[31]: {=bool:?} }}",
            self.ptto(0usize),
            self.ptto(1usize),
            self.ptto(2usize),
            self.ptto(3usize),
            self.ptto(4usize),
            self.ptto(5usize),
            self.ptto(6usize),
            self.ptto(7usize),
            self.ptto(8usize),
            self.ptto(9usize),
            self.ptto(10usize),
            self.ptto(11usize),
            self.ptto(12usize),
            self.ptto(13usize),
            self.ptto(14usize),
            self.ptto(15usize),
            self.ptto(16usize),
            self.ptto(17usize),
            self.ptto(18usize),
            self.ptto(19usize),
            self.ptto(20usize),
            self.ptto(21usize),
            self.ptto(22usize),
            self.ptto(23usize),
            self.ptto(24usize),
            self.ptto(25usize),
            self.ptto(26usize),
            self.ptto(27usize),
            self.ptto(28usize),
            self.ptto(29usize),
            self.ptto(30usize),
            self.ptto(31usize)
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
