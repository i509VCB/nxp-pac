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
#[doc = "Interrupt Control Nonprivilege"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnp(pub u32);
impl Icnp {
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe0(&self) -> super::vals::IcnpNpe0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IcnpNpe0::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe0(&mut self, val: super::vals::IcnpNpe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe1(&self) -> super::vals::IcnpNpe1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IcnpNpe1::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe1(&mut self, val: super::vals::IcnpNpe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Icnp {
    #[inline(always)]
    fn default() -> Icnp {
        Icnp(0)
    }
}
impl core::fmt::Debug for Icnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icnp")
            .field("npe0", &self.npe0())
            .field("npe1", &self.npe1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icnp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icnp {{ npe0: {:?}, npe1: {:?} }}",
            self.npe0(),
            self.npe1()
        )
    }
}
#[doc = "Interrupt Control Nonsecure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icns(pub u32);
impl Icns {
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::IcnsNse0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IcnsNse0::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::IcnsNse0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::IcnsNse1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IcnsNse1::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::IcnsNse1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Icns {
    #[inline(always)]
    fn default() -> Icns {
        Icns(0)
    }
}
impl core::fmt::Debug for Icns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icns")
            .field("nse0", &self.nse0())
            .field("nse1", &self.nse1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icns {{ nse0: {:?}, nse1: {:?} }}",
            self.nse0(),
            self.nse1()
        )
    }
}
#[doc = "Interrupt Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr0(pub u32);
impl Icr0 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr0Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr0Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr0Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr0Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr0Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr0Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr0Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr0Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr0Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr0Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr0Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr0Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr0 {
    #[inline(always)]
    fn default() -> Icr0 {
        Icr0(0)
    }
}
impl core::fmt::Debug for Icr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr0")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr0 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr1(pub u32);
impl Icr1 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr1Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr1Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr1Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr1Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr1Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr1Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr1Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr1Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr1Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr1Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr1Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr1Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr1 {
    #[inline(always)]
    fn default() -> Icr1 {
        Icr1(0)
    }
}
impl core::fmt::Debug for Icr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr1")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr1 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr10(pub u32);
impl Icr10 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr10Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr10Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr10Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr10Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr10Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr10Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr10Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr10Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr10Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr10Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr10Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr10Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr10 {
    #[inline(always)]
    fn default() -> Icr10 {
        Icr10(0)
    }
}
impl core::fmt::Debug for Icr10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr10")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr10 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr11(pub u32);
impl Icr11 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr11Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr11Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr11Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr11Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr11Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr11Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr11Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr11Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr11Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr11Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr11Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr11Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr11 {
    #[inline(always)]
    fn default() -> Icr11 {
        Icr11(0)
    }
}
impl core::fmt::Debug for Icr11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr11")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr11 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr12(pub u32);
impl Icr12 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr12Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr12Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr12Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr12Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr12Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr12Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr12Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr12Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr12Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr12Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr12Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr12Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr12 {
    #[inline(always)]
    fn default() -> Icr12 {
        Icr12(0)
    }
}
impl core::fmt::Debug for Icr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr12")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr12 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr13(pub u32);
impl Icr13 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr13Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr13Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr13Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr13Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr13Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr13Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr13Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr13Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr13Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr13Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr13Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr13Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr13 {
    #[inline(always)]
    fn default() -> Icr13 {
        Icr13(0)
    }
}
impl core::fmt::Debug for Icr13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr13")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr13 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr14(pub u32);
impl Icr14 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr14Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr14Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr14Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr14Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr14Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr14Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr14Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr14Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr14Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr14Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr14Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr14Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr14 {
    #[inline(always)]
    fn default() -> Icr14 {
        Icr14(0)
    }
}
impl core::fmt::Debug for Icr14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr14")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr14 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr15(pub u32);
impl Icr15 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr15Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr15Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr15Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr15Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr15Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr15Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr15Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr15Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr15Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr15Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr15Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr15Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr15 {
    #[inline(always)]
    fn default() -> Icr15 {
        Icr15(0)
    }
}
impl core::fmt::Debug for Icr15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr15")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr15 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr16(pub u32);
impl Icr16 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr16Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr16Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr16Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr16Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr16Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr16Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr16Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr16Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr16Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr16Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr16Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr16Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr16 {
    #[inline(always)]
    fn default() -> Icr16 {
        Icr16(0)
    }
}
impl core::fmt::Debug for Icr16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr16")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr16 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr17(pub u32);
impl Icr17 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr17Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr17Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr17Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr17Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr17Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr17Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr17Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr17Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr17Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr17Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr17Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr17Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr17 {
    #[inline(always)]
    fn default() -> Icr17 {
        Icr17(0)
    }
}
impl core::fmt::Debug for Icr17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr17")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr17 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr18(pub u32);
impl Icr18 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr18Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr18Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr18Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr18Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr18Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr18Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr18Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr18Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr18Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr18Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr18Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr18Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr18 {
    #[inline(always)]
    fn default() -> Icr18 {
        Icr18(0)
    }
}
impl core::fmt::Debug for Icr18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr18")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr18 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr19(pub u32);
impl Icr19 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr19Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr19Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr19Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr19Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr19Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr19Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr19Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr19Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr19Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr19Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr19Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr19Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr19 {
    #[inline(always)]
    fn default() -> Icr19 {
        Icr19(0)
    }
}
impl core::fmt::Debug for Icr19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr19")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr19 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr2(pub u32);
impl Icr2 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr2Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr2Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr2Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr2Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr2Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr2Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr2Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr2Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr2Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr2Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr2Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr2Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr2 {
    #[inline(always)]
    fn default() -> Icr2 {
        Icr2(0)
    }
}
impl core::fmt::Debug for Icr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr2")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr2 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr20(pub u32);
impl Icr20 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr20Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr20Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr20Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr20Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr20Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr20Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr20Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr20Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr20Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr20Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr20Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr20Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr20 {
    #[inline(always)]
    fn default() -> Icr20 {
        Icr20(0)
    }
}
impl core::fmt::Debug for Icr20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr20")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr20 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr21(pub u32);
impl Icr21 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr21Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr21Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr21Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr21Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr21Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr21Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr21Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr21Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr21Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr21Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr21Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr21Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr21 {
    #[inline(always)]
    fn default() -> Icr21 {
        Icr21(0)
    }
}
impl core::fmt::Debug for Icr21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr21")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr21 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr22(pub u32);
impl Icr22 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr22Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr22Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr22Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr22Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr22Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr22Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr22Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr22Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr22Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr22Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr22Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr22Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr22 {
    #[inline(always)]
    fn default() -> Icr22 {
        Icr22(0)
    }
}
impl core::fmt::Debug for Icr22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr22")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr22 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr23(pub u32);
impl Icr23 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr23Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr23Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr23Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr23Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr23Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr23Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr23Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr23Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr23Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr23Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr23Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr23Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr23 {
    #[inline(always)]
    fn default() -> Icr23 {
        Icr23(0)
    }
}
impl core::fmt::Debug for Icr23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr23")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr23 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr24(pub u32);
impl Icr24 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr24Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr24Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr24Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr24Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr24Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr24Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr24Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr24Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr24Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr24Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr24Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr24Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr24 {
    #[inline(always)]
    fn default() -> Icr24 {
        Icr24(0)
    }
}
impl core::fmt::Debug for Icr24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr24")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr24 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr25(pub u32);
impl Icr25 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr25Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr25Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr25Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr25Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr25Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr25Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr25Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr25Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr25Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr25Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr25Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr25Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr25 {
    #[inline(always)]
    fn default() -> Icr25 {
        Icr25(0)
    }
}
impl core::fmt::Debug for Icr25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr25")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr25 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr26(pub u32);
impl Icr26 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr26Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr26Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr26Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr26Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr26Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr26Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr26Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr26Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr26Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr26Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr26Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr26Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr26 {
    #[inline(always)]
    fn default() -> Icr26 {
        Icr26(0)
    }
}
impl core::fmt::Debug for Icr26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr26")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr26 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr27(pub u32);
impl Icr27 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr27Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr27Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr27Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr27Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr27Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr27Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr27Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr27Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr27Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr27Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr27Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr27Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr27 {
    #[inline(always)]
    fn default() -> Icr27 {
        Icr27(0)
    }
}
impl core::fmt::Debug for Icr27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr27")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr27 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr28(pub u32);
impl Icr28 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr28Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr28Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr28Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr28Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr28Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr28Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr28Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr28Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr28Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr28Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr28Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr28Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr28 {
    #[inline(always)]
    fn default() -> Icr28 {
        Icr28(0)
    }
}
impl core::fmt::Debug for Icr28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr28")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr28 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr29(pub u32);
impl Icr29 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr29Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr29Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr29Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr29Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr29Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr29Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr29Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr29Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr29Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr29Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr29Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr29Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr29 {
    #[inline(always)]
    fn default() -> Icr29 {
        Icr29(0)
    }
}
impl core::fmt::Debug for Icr29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr29")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr29 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr3(pub u32);
impl Icr3 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr3Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr3Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr3Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr3Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr3Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr3Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr3Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr3Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr3Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr3Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr3Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr3Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr3 {
    #[inline(always)]
    fn default() -> Icr3 {
        Icr3(0)
    }
}
impl core::fmt::Debug for Icr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr3")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr3 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr30(pub u32);
impl Icr30 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr30Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr30Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr30Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr30Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr30Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr30Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr30Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr30Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr30Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr30Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr30Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr30Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr30 {
    #[inline(always)]
    fn default() -> Icr30 {
        Icr30(0)
    }
}
impl core::fmt::Debug for Icr30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr30")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr30 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr31(pub u32);
impl Icr31 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr31Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr31Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr31Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr31Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr31Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr31Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr31Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr31Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr31Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr31Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr31Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr31Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr31 {
    #[inline(always)]
    fn default() -> Icr31 {
        Icr31(0)
    }
}
impl core::fmt::Debug for Icr31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr31")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr31 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr4(pub u32);
impl Icr4 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr4Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr4Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr4Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr4Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr4Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr4Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr4Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr4Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr4Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr4Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr4Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr4Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr4 {
    #[inline(always)]
    fn default() -> Icr4 {
        Icr4(0)
    }
}
impl core::fmt::Debug for Icr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr4")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr4 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr5(pub u32);
impl Icr5 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr5Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr5Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr5Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr5Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr5Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr5Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr5Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr5Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr5Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr5Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr5Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr5Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr5 {
    #[inline(always)]
    fn default() -> Icr5 {
        Icr5(0)
    }
}
impl core::fmt::Debug for Icr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr5")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr5 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr6(pub u32);
impl Icr6 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr6Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr6Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr6Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr6Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr6Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr6Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr6Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr6Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr6Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr6Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr6Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr6Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr6 {
    #[inline(always)]
    fn default() -> Icr6 {
        Icr6(0)
    }
}
impl core::fmt::Debug for Icr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr6")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr6 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr7(pub u32);
impl Icr7 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr7Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr7Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr7Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr7Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr7Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr7Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr7Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr7Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr7Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr7Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr7Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr7Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr7 {
    #[inline(always)]
    fn default() -> Icr7 {
        Icr7(0)
    }
}
impl core::fmt::Debug for Icr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr7")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr7 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr8(pub u32);
impl Icr8 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr8Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr8Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr8Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr8Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr8Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr8Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr8Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr8Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr8Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr8Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr8Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr8Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr8 {
    #[inline(always)]
    fn default() -> Icr8 {
        Icr8(0)
    }
}
impl core::fmt::Debug for Icr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr8")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr8 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Control 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr9(pub u32);
impl Icr9 {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Icr9Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Icr9Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Icr9Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> super::vals::Icr9Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Icr9Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: super::vals::Icr9Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Icr9Lk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Icr9Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Icr9Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> super::vals::Icr9Isf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Icr9Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: super::vals::Icr9Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr9 {
    #[inline(always)]
    fn default() -> Icr9 {
        Icr9(0)
    }
}
impl core::fmt::Debug for Icr9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr9")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr9 {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Status Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfr(pub u32);
impl Isfr {
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
impl Default for Isfr {
    #[inline(always)]
    fn default() -> Isfr {
        Isfr(0)
    }
}
impl core::fmt::Debug for Isfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfr")
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
impl defmt::Format for Isfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isfr {{ isf0: {:?}, isf1: {:?}, isf2: {:?}, isf3: {:?}, isf4: {:?}, isf5: {:?}, isf6: {:?}, isf7: {:?}, isf8: {:?}, isf9: {:?}, isf10: {:?}, isf11: {:?}, isf12: {:?}, isf13: {:?}, isf14: {:?}, isf15: {:?}, isf16: {:?}, isf17: {:?}, isf18: {:?}, isf19: {:?}, isf20: {:?}, isf21: {:?}, isf22: {:?}, isf23: {:?}, isf24: {:?}, isf25: {:?}, isf26: {:?}, isf27: {:?}, isf28: {:?}, isf29: {:?}, isf30: {:?}, isf31: {:?} }}",
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
#[doc = "Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Lock PCNS"]
    #[must_use]
    #[inline(always)]
    pub const fn pcns(&self) -> super::vals::Pcns {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcns::from_bits(val as u8)
    }
    #[doc = "Lock PCNS"]
    #[inline(always)]
    pub const fn set_pcns(&mut self, val: super::vals::Pcns) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock ICNS"]
    #[must_use]
    #[inline(always)]
    pub const fn icns(&self) -> super::vals::Icns {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Icns::from_bits(val as u8)
    }
    #[doc = "Lock ICNS"]
    #[inline(always)]
    pub const fn set_icns(&mut self, val: super::vals::Icns) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Lock PCNP"]
    #[must_use]
    #[inline(always)]
    pub const fn pcnp(&self) -> super::vals::Pcnp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pcnp::from_bits(val as u8)
    }
    #[doc = "Lock PCNP"]
    #[inline(always)]
    pub const fn set_pcnp(&mut self, val: super::vals::Pcnp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock ICNP"]
    #[must_use]
    #[inline(always)]
    pub const fn icnp(&self) -> super::vals::Icnp {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Icnp::from_bits(val as u8)
    }
    #[doc = "Lock ICNP"]
    #[inline(always)]
    pub const fn set_icnp(&mut self, val: super::vals::Icnp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("pcns", &self.pcns())
            .field("icns", &self.icns())
            .field("pcnp", &self.pcnp())
            .field("icnp", &self.icnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ pcns: {:?}, icns: {:?}, pcnp: {:?}, icnp: {:?} }}",
            self.pcns(),
            self.icns(),
            self.pcnp(),
            self.icnp()
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
#[doc = "Pin Control Nonprivilege"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnp(pub u32);
impl Pcnp {
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe0(&self) -> super::vals::PcnpNpe0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PcnpNpe0::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe0(&mut self, val: super::vals::PcnpNpe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe1(&self) -> super::vals::PcnpNpe1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PcnpNpe1::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe1(&mut self, val: super::vals::PcnpNpe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe2(&self) -> super::vals::Npe2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Npe2::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe2(&mut self, val: super::vals::Npe2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe3(&self) -> super::vals::Npe3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Npe3::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe3(&mut self, val: super::vals::Npe3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe4(&self) -> super::vals::Npe4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Npe4::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe4(&mut self, val: super::vals::Npe4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe5(&self) -> super::vals::Npe5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Npe5::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe5(&mut self, val: super::vals::Npe5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe6(&self) -> super::vals::Npe6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Npe6::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe6(&mut self, val: super::vals::Npe6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe7(&self) -> super::vals::Npe7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Npe7::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe7(&mut self, val: super::vals::Npe7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe8(&self) -> super::vals::Npe8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Npe8::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe8(&mut self, val: super::vals::Npe8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe9(&self) -> super::vals::Npe9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Npe9::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe9(&mut self, val: super::vals::Npe9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe10(&self) -> super::vals::Npe10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Npe10::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe10(&mut self, val: super::vals::Npe10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe11(&self) -> super::vals::Npe11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Npe11::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe11(&mut self, val: super::vals::Npe11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe12(&self) -> super::vals::Npe12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Npe12::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe12(&mut self, val: super::vals::Npe12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe13(&self) -> super::vals::Npe13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Npe13::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe13(&mut self, val: super::vals::Npe13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe14(&self) -> super::vals::Npe14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Npe14::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe14(&mut self, val: super::vals::Npe14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe15(&self) -> super::vals::Npe15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Npe15::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe15(&mut self, val: super::vals::Npe15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe16(&self) -> super::vals::Npe16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Npe16::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe16(&mut self, val: super::vals::Npe16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe17(&self) -> super::vals::Npe17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Npe17::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe17(&mut self, val: super::vals::Npe17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe18(&self) -> super::vals::Npe18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Npe18::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe18(&mut self, val: super::vals::Npe18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe19(&self) -> super::vals::Npe19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Npe19::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe19(&mut self, val: super::vals::Npe19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe20(&self) -> super::vals::Npe20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Npe20::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe20(&mut self, val: super::vals::Npe20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe21(&self) -> super::vals::Npe21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Npe21::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe21(&mut self, val: super::vals::Npe21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe22(&self) -> super::vals::Npe22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Npe22::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe22(&mut self, val: super::vals::Npe22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe23(&self) -> super::vals::Npe23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Npe23::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe23(&mut self, val: super::vals::Npe23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe24(&self) -> super::vals::Npe24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Npe24::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe24(&mut self, val: super::vals::Npe24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe25(&self) -> super::vals::Npe25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Npe25::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe25(&mut self, val: super::vals::Npe25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe26(&self) -> super::vals::Npe26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Npe26::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe26(&mut self, val: super::vals::Npe26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe27(&self) -> super::vals::Npe27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Npe27::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe27(&mut self, val: super::vals::Npe27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe28(&self) -> super::vals::Npe28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Npe28::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe28(&mut self, val: super::vals::Npe28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe29(&self) -> super::vals::Npe29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Npe29::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe29(&mut self, val: super::vals::Npe29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe30(&self) -> super::vals::Npe30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Npe30::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe30(&mut self, val: super::vals::Npe30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Nonprivilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe31(&self) -> super::vals::Npe31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Npe31::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable"]
    #[inline(always)]
    pub const fn set_npe31(&mut self, val: super::vals::Npe31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pcnp {
    #[inline(always)]
    fn default() -> Pcnp {
        Pcnp(0)
    }
}
impl core::fmt::Debug for Pcnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcnp")
            .field("npe0", &self.npe0())
            .field("npe1", &self.npe1())
            .field("npe2", &self.npe2())
            .field("npe3", &self.npe3())
            .field("npe4", &self.npe4())
            .field("npe5", &self.npe5())
            .field("npe6", &self.npe6())
            .field("npe7", &self.npe7())
            .field("npe8", &self.npe8())
            .field("npe9", &self.npe9())
            .field("npe10", &self.npe10())
            .field("npe11", &self.npe11())
            .field("npe12", &self.npe12())
            .field("npe13", &self.npe13())
            .field("npe14", &self.npe14())
            .field("npe15", &self.npe15())
            .field("npe16", &self.npe16())
            .field("npe17", &self.npe17())
            .field("npe18", &self.npe18())
            .field("npe19", &self.npe19())
            .field("npe20", &self.npe20())
            .field("npe21", &self.npe21())
            .field("npe22", &self.npe22())
            .field("npe23", &self.npe23())
            .field("npe24", &self.npe24())
            .field("npe25", &self.npe25())
            .field("npe26", &self.npe26())
            .field("npe27", &self.npe27())
            .field("npe28", &self.npe28())
            .field("npe29", &self.npe29())
            .field("npe30", &self.npe30())
            .field("npe31", &self.npe31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcnp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcnp {{ npe0: {:?}, npe1: {:?}, npe2: {:?}, npe3: {:?}, npe4: {:?}, npe5: {:?}, npe6: {:?}, npe7: {:?}, npe8: {:?}, npe9: {:?}, npe10: {:?}, npe11: {:?}, npe12: {:?}, npe13: {:?}, npe14: {:?}, npe15: {:?}, npe16: {:?}, npe17: {:?}, npe18: {:?}, npe19: {:?}, npe20: {:?}, npe21: {:?}, npe22: {:?}, npe23: {:?}, npe24: {:?}, npe25: {:?}, npe26: {:?}, npe27: {:?}, npe28: {:?}, npe29: {:?}, npe30: {:?}, npe31: {:?} }}",
            self.npe0(),
            self.npe1(),
            self.npe2(),
            self.npe3(),
            self.npe4(),
            self.npe5(),
            self.npe6(),
            self.npe7(),
            self.npe8(),
            self.npe9(),
            self.npe10(),
            self.npe11(),
            self.npe12(),
            self.npe13(),
            self.npe14(),
            self.npe15(),
            self.npe16(),
            self.npe17(),
            self.npe18(),
            self.npe19(),
            self.npe20(),
            self.npe21(),
            self.npe22(),
            self.npe23(),
            self.npe24(),
            self.npe25(),
            self.npe26(),
            self.npe27(),
            self.npe28(),
            self.npe29(),
            self.npe30(),
            self.npe31()
        )
    }
}
#[doc = "Pin Control Nonsecure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcns(pub u32);
impl Pcns {
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::PcnsNse0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PcnsNse0::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::PcnsNse0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::PcnsNse1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PcnsNse1::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::PcnsNse1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Nse2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Nse2::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Nse2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Nse3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Nse3::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Nse3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Nse4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Nse4::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Nse4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Nse5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Nse5::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Nse5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Nse6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Nse6::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Nse6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Nse7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Nse7::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Nse7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse8(&self) -> super::vals::Nse8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Nse8::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse8(&mut self, val: super::vals::Nse8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse9(&self) -> super::vals::Nse9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Nse9::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse9(&mut self, val: super::vals::Nse9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse10(&self) -> super::vals::Nse10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Nse10::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse10(&mut self, val: super::vals::Nse10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse11(&self) -> super::vals::Nse11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Nse11::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse11(&mut self, val: super::vals::Nse11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse12(&self) -> super::vals::Nse12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Nse12::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse12(&mut self, val: super::vals::Nse12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse13(&self) -> super::vals::Nse13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Nse13::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse13(&mut self, val: super::vals::Nse13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse14(&self) -> super::vals::Nse14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Nse14::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse14(&mut self, val: super::vals::Nse14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse15(&self) -> super::vals::Nse15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Nse15::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse15(&mut self, val: super::vals::Nse15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse16(&self) -> super::vals::Nse16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Nse16::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse16(&mut self, val: super::vals::Nse16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse17(&self) -> super::vals::Nse17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Nse17::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse17(&mut self, val: super::vals::Nse17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse18(&self) -> super::vals::Nse18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Nse18::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse18(&mut self, val: super::vals::Nse18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse19(&self) -> super::vals::Nse19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Nse19::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse19(&mut self, val: super::vals::Nse19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse20(&self) -> super::vals::Nse20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Nse20::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse20(&mut self, val: super::vals::Nse20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse21(&self) -> super::vals::Nse21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Nse21::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse21(&mut self, val: super::vals::Nse21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse22(&self) -> super::vals::Nse22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Nse22::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse22(&mut self, val: super::vals::Nse22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse23(&self) -> super::vals::Nse23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Nse23::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse23(&mut self, val: super::vals::Nse23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse24(&self) -> super::vals::Nse24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Nse24::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse24(&mut self, val: super::vals::Nse24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse25(&self) -> super::vals::Nse25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Nse25::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse25(&mut self, val: super::vals::Nse25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse26(&self) -> super::vals::Nse26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Nse26::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse26(&mut self, val: super::vals::Nse26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse27(&self) -> super::vals::Nse27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Nse27::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse27(&mut self, val: super::vals::Nse27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse28(&self) -> super::vals::Nse28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Nse28::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse28(&mut self, val: super::vals::Nse28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse29(&self) -> super::vals::Nse29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Nse29::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse29(&mut self, val: super::vals::Nse29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse30(&self) -> super::vals::Nse30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Nse30::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse30(&mut self, val: super::vals::Nse30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Nonsecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse31(&self) -> super::vals::Nse31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Nse31::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable"]
    #[inline(always)]
    pub const fn set_nse31(&mut self, val: super::vals::Nse31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pcns {
    #[inline(always)]
    fn default() -> Pcns {
        Pcns(0)
    }
}
impl core::fmt::Debug for Pcns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcns")
            .field("nse0", &self.nse0())
            .field("nse1", &self.nse1())
            .field("nse2", &self.nse2())
            .field("nse3", &self.nse3())
            .field("nse4", &self.nse4())
            .field("nse5", &self.nse5())
            .field("nse6", &self.nse6())
            .field("nse7", &self.nse7())
            .field("nse8", &self.nse8())
            .field("nse9", &self.nse9())
            .field("nse10", &self.nse10())
            .field("nse11", &self.nse11())
            .field("nse12", &self.nse12())
            .field("nse13", &self.nse13())
            .field("nse14", &self.nse14())
            .field("nse15", &self.nse15())
            .field("nse16", &self.nse16())
            .field("nse17", &self.nse17())
            .field("nse18", &self.nse18())
            .field("nse19", &self.nse19())
            .field("nse20", &self.nse20())
            .field("nse21", &self.nse21())
            .field("nse22", &self.nse22())
            .field("nse23", &self.nse23())
            .field("nse24", &self.nse24())
            .field("nse25", &self.nse25())
            .field("nse26", &self.nse26())
            .field("nse27", &self.nse27())
            .field("nse28", &self.nse28())
            .field("nse29", &self.nse29())
            .field("nse30", &self.nse30())
            .field("nse31", &self.nse31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcns {{ nse0: {:?}, nse1: {:?}, nse2: {:?}, nse3: {:?}, nse4: {:?}, nse5: {:?}, nse6: {:?}, nse7: {:?}, nse8: {:?}, nse9: {:?}, nse10: {:?}, nse11: {:?}, nse12: {:?}, nse13: {:?}, nse14: {:?}, nse15: {:?}, nse16: {:?}, nse17: {:?}, nse18: {:?}, nse19: {:?}, nse20: {:?}, nse21: {:?}, nse22: {:?}, nse23: {:?}, nse24: {:?}, nse25: {:?}, nse26: {:?}, nse27: {:?}, nse28: {:?}, nse29: {:?}, nse30: {:?}, nse31: {:?} }}",
            self.nse0(),
            self.nse1(),
            self.nse2(),
            self.nse3(),
            self.nse4(),
            self.nse5(),
            self.nse6(),
            self.nse7(),
            self.nse8(),
            self.nse9(),
            self.nse10(),
            self.nse11(),
            self.nse12(),
            self.nse13(),
            self.nse14(),
            self.nse15(),
            self.nse16(),
            self.nse17(),
            self.nse18(),
            self.nse19(),
            self.nse20(),
            self.nse21(),
            self.nse22(),
            self.nse23(),
            self.nse24(),
            self.nse25(),
            self.nse26(),
            self.nse27(),
            self.nse28(),
            self.nse29(),
            self.nse30(),
            self.nse31()
        )
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
    pub const fn ptco0(&self) -> super::vals::Ptco0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ptco0::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco0(&mut self, val: super::vals::Ptco0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco1(&self) -> super::vals::Ptco1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ptco1::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco1(&mut self, val: super::vals::Ptco1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco2(&self) -> super::vals::Ptco2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ptco2::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco2(&mut self, val: super::vals::Ptco2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco3(&self) -> super::vals::Ptco3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ptco3::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco3(&mut self, val: super::vals::Ptco3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco4(&self) -> super::vals::Ptco4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ptco4::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco4(&mut self, val: super::vals::Ptco4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco5(&self) -> super::vals::Ptco5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ptco5::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco5(&mut self, val: super::vals::Ptco5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco6(&self) -> super::vals::Ptco6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ptco6::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco6(&mut self, val: super::vals::Ptco6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco7(&self) -> super::vals::Ptco7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ptco7::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco7(&mut self, val: super::vals::Ptco7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco8(&self) -> super::vals::Ptco8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ptco8::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco8(&mut self, val: super::vals::Ptco8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco9(&self) -> super::vals::Ptco9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ptco9::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco9(&mut self, val: super::vals::Ptco9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco10(&self) -> super::vals::Ptco10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ptco10::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco10(&mut self, val: super::vals::Ptco10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco11(&self) -> super::vals::Ptco11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ptco11::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco11(&mut self, val: super::vals::Ptco11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco12(&self) -> super::vals::Ptco12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ptco12::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco12(&mut self, val: super::vals::Ptco12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco13(&self) -> super::vals::Ptco13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ptco13::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco13(&mut self, val: super::vals::Ptco13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco14(&self) -> super::vals::Ptco14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ptco14::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco14(&mut self, val: super::vals::Ptco14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco15(&self) -> super::vals::Ptco15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ptco15::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco15(&mut self, val: super::vals::Ptco15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco16(&self) -> super::vals::Ptco16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ptco16::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco16(&mut self, val: super::vals::Ptco16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco17(&self) -> super::vals::Ptco17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ptco17::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco17(&mut self, val: super::vals::Ptco17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco18(&self) -> super::vals::Ptco18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ptco18::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco18(&mut self, val: super::vals::Ptco18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco19(&self) -> super::vals::Ptco19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ptco19::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco19(&mut self, val: super::vals::Ptco19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco20(&self) -> super::vals::Ptco20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ptco20::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco20(&mut self, val: super::vals::Ptco20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco21(&self) -> super::vals::Ptco21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ptco21::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco21(&mut self, val: super::vals::Ptco21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco22(&self) -> super::vals::Ptco22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ptco22::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco22(&mut self, val: super::vals::Ptco22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco23(&self) -> super::vals::Ptco23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ptco23::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco23(&mut self, val: super::vals::Ptco23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco24(&self) -> super::vals::Ptco24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ptco24::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco24(&mut self, val: super::vals::Ptco24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco25(&self) -> super::vals::Ptco25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Ptco25::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco25(&mut self, val: super::vals::Ptco25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco26(&self) -> super::vals::Ptco26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ptco26::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco26(&mut self, val: super::vals::Ptco26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco27(&self) -> super::vals::Ptco27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ptco27::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco27(&mut self, val: super::vals::Ptco27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco28(&self) -> super::vals::Ptco28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ptco28::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco28(&mut self, val: super::vals::Ptco28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco29(&self) -> super::vals::Ptco29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Ptco29::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco29(&mut self, val: super::vals::Ptco29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco30(&self) -> super::vals::Ptco30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ptco30::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco30(&mut self, val: super::vals::Ptco30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco31(&self) -> super::vals::Ptco31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ptco31::from_bits(val as u8)
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco31(&mut self, val: super::vals::Ptco31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("ptco0", &self.ptco0())
            .field("ptco1", &self.ptco1())
            .field("ptco2", &self.ptco2())
            .field("ptco3", &self.ptco3())
            .field("ptco4", &self.ptco4())
            .field("ptco5", &self.ptco5())
            .field("ptco6", &self.ptco6())
            .field("ptco7", &self.ptco7())
            .field("ptco8", &self.ptco8())
            .field("ptco9", &self.ptco9())
            .field("ptco10", &self.ptco10())
            .field("ptco11", &self.ptco11())
            .field("ptco12", &self.ptco12())
            .field("ptco13", &self.ptco13())
            .field("ptco14", &self.ptco14())
            .field("ptco15", &self.ptco15())
            .field("ptco16", &self.ptco16())
            .field("ptco17", &self.ptco17())
            .field("ptco18", &self.ptco18())
            .field("ptco19", &self.ptco19())
            .field("ptco20", &self.ptco20())
            .field("ptco21", &self.ptco21())
            .field("ptco22", &self.ptco22())
            .field("ptco23", &self.ptco23())
            .field("ptco24", &self.ptco24())
            .field("ptco25", &self.ptco25())
            .field("ptco26", &self.ptco26())
            .field("ptco27", &self.ptco27())
            .field("ptco28", &self.ptco28())
            .field("ptco29", &self.ptco29())
            .field("ptco30", &self.ptco30())
            .field("ptco31", &self.ptco31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcor {{ ptco0: {:?}, ptco1: {:?}, ptco2: {:?}, ptco3: {:?}, ptco4: {:?}, ptco5: {:?}, ptco6: {:?}, ptco7: {:?}, ptco8: {:?}, ptco9: {:?}, ptco10: {:?}, ptco11: {:?}, ptco12: {:?}, ptco13: {:?}, ptco14: {:?}, ptco15: {:?}, ptco16: {:?}, ptco17: {:?}, ptco18: {:?}, ptco19: {:?}, ptco20: {:?}, ptco21: {:?}, ptco22: {:?}, ptco23: {:?}, ptco24: {:?}, ptco25: {:?}, ptco26: {:?}, ptco27: {:?}, ptco28: {:?}, ptco29: {:?}, ptco30: {:?}, ptco31: {:?} }}",
            self.ptco0(),
            self.ptco1(),
            self.ptco2(),
            self.ptco3(),
            self.ptco4(),
            self.ptco5(),
            self.ptco6(),
            self.ptco7(),
            self.ptco8(),
            self.ptco9(),
            self.ptco10(),
            self.ptco11(),
            self.ptco12(),
            self.ptco13(),
            self.ptco14(),
            self.ptco15(),
            self.ptco16(),
            self.ptco17(),
            self.ptco18(),
            self.ptco19(),
            self.ptco20(),
            self.ptco21(),
            self.ptco22(),
            self.ptco23(),
            self.ptco24(),
            self.ptco25(),
            self.ptco26(),
            self.ptco27(),
            self.ptco28(),
            self.ptco29(),
            self.ptco30(),
            self.ptco31()
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
    pub const fn pdd0(&self) -> super::vals::Pdd0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdd0::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd0(&mut self, val: super::vals::Pdd0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd1(&self) -> super::vals::Pdd1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdd1::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd1(&mut self, val: super::vals::Pdd1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd2(&self) -> super::vals::Pdd2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdd2::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd2(&mut self, val: super::vals::Pdd2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd3(&self) -> super::vals::Pdd3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdd3::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd3(&mut self, val: super::vals::Pdd3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd4(&self) -> super::vals::Pdd4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdd4::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd4(&mut self, val: super::vals::Pdd4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd5(&self) -> super::vals::Pdd5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdd5::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd5(&mut self, val: super::vals::Pdd5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd6(&self) -> super::vals::Pdd6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdd6::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd6(&mut self, val: super::vals::Pdd6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd7(&self) -> super::vals::Pdd7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdd7::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd7(&mut self, val: super::vals::Pdd7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd8(&self) -> super::vals::Pdd8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdd8::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd8(&mut self, val: super::vals::Pdd8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd9(&self) -> super::vals::Pdd9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdd9::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd9(&mut self, val: super::vals::Pdd9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd10(&self) -> super::vals::Pdd10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdd10::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd10(&mut self, val: super::vals::Pdd10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd11(&self) -> super::vals::Pdd11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdd11::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd11(&mut self, val: super::vals::Pdd11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd12(&self) -> super::vals::Pdd12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdd12::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd12(&mut self, val: super::vals::Pdd12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd13(&self) -> super::vals::Pdd13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdd13::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd13(&mut self, val: super::vals::Pdd13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd14(&self) -> super::vals::Pdd14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdd14::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd14(&mut self, val: super::vals::Pdd14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd15(&self) -> super::vals::Pdd15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdd15::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd15(&mut self, val: super::vals::Pdd15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd16(&self) -> super::vals::Pdd16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdd16::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd16(&mut self, val: super::vals::Pdd16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd17(&self) -> super::vals::Pdd17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdd17::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd17(&mut self, val: super::vals::Pdd17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd18(&self) -> super::vals::Pdd18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdd18::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd18(&mut self, val: super::vals::Pdd18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd19(&self) -> super::vals::Pdd19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdd19::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd19(&mut self, val: super::vals::Pdd19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd20(&self) -> super::vals::Pdd20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdd20::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd20(&mut self, val: super::vals::Pdd20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd21(&self) -> super::vals::Pdd21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdd21::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd21(&mut self, val: super::vals::Pdd21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd22(&self) -> super::vals::Pdd22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdd22::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd22(&mut self, val: super::vals::Pdd22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd23(&self) -> super::vals::Pdd23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdd23::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd23(&mut self, val: super::vals::Pdd23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd24(&self) -> super::vals::Pdd24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdd24::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd24(&mut self, val: super::vals::Pdd24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd25(&self) -> super::vals::Pdd25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdd25::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd25(&mut self, val: super::vals::Pdd25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd26(&self) -> super::vals::Pdd26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdd26::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd26(&mut self, val: super::vals::Pdd26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd27(&self) -> super::vals::Pdd27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdd27::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd27(&mut self, val: super::vals::Pdd27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd28(&self) -> super::vals::Pdd28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdd28::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd28(&mut self, val: super::vals::Pdd28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd29(&self) -> super::vals::Pdd29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdd29::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd29(&mut self, val: super::vals::Pdd29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd30(&self) -> super::vals::Pdd30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pdd30::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd30(&mut self, val: super::vals::Pdd30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd31(&self) -> super::vals::Pdd31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdd31::from_bits(val as u8)
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd31(&mut self, val: super::vals::Pdd31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("pdd0", &self.pdd0())
            .field("pdd1", &self.pdd1())
            .field("pdd2", &self.pdd2())
            .field("pdd3", &self.pdd3())
            .field("pdd4", &self.pdd4())
            .field("pdd5", &self.pdd5())
            .field("pdd6", &self.pdd6())
            .field("pdd7", &self.pdd7())
            .field("pdd8", &self.pdd8())
            .field("pdd9", &self.pdd9())
            .field("pdd10", &self.pdd10())
            .field("pdd11", &self.pdd11())
            .field("pdd12", &self.pdd12())
            .field("pdd13", &self.pdd13())
            .field("pdd14", &self.pdd14())
            .field("pdd15", &self.pdd15())
            .field("pdd16", &self.pdd16())
            .field("pdd17", &self.pdd17())
            .field("pdd18", &self.pdd18())
            .field("pdd19", &self.pdd19())
            .field("pdd20", &self.pdd20())
            .field("pdd21", &self.pdd21())
            .field("pdd22", &self.pdd22())
            .field("pdd23", &self.pdd23())
            .field("pdd24", &self.pdd24())
            .field("pdd25", &self.pdd25())
            .field("pdd26", &self.pdd26())
            .field("pdd27", &self.pdd27())
            .field("pdd28", &self.pdd28())
            .field("pdd29", &self.pdd29())
            .field("pdd30", &self.pdd30())
            .field("pdd31", &self.pdd31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pddr {{ pdd0: {:?}, pdd1: {:?}, pdd2: {:?}, pdd3: {:?}, pdd4: {:?}, pdd5: {:?}, pdd6: {:?}, pdd7: {:?}, pdd8: {:?}, pdd9: {:?}, pdd10: {:?}, pdd11: {:?}, pdd12: {:?}, pdd13: {:?}, pdd14: {:?}, pdd15: {:?}, pdd16: {:?}, pdd17: {:?}, pdd18: {:?}, pdd19: {:?}, pdd20: {:?}, pdd21: {:?}, pdd22: {:?}, pdd23: {:?}, pdd24: {:?}, pdd25: {:?}, pdd26: {:?}, pdd27: {:?}, pdd28: {:?}, pdd29: {:?}, pdd30: {:?}, pdd31: {:?} }}",
            self.pdd0(),
            self.pdd1(),
            self.pdd2(),
            self.pdd3(),
            self.pdd4(),
            self.pdd5(),
            self.pdd6(),
            self.pdd7(),
            self.pdd8(),
            self.pdd9(),
            self.pdd10(),
            self.pdd11(),
            self.pdd12(),
            self.pdd13(),
            self.pdd14(),
            self.pdd15(),
            self.pdd16(),
            self.pdd17(),
            self.pdd18(),
            self.pdd19(),
            self.pdd20(),
            self.pdd21(),
            self.pdd22(),
            self.pdd23(),
            self.pdd24(),
            self.pdd25(),
            self.pdd26(),
            self.pdd27(),
            self.pdd28(),
            self.pdd29(),
            self.pdd30(),
            self.pdd31()
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
    pub const fn pdi0(&self) -> super::vals::Pdi0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdi0::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi0(&mut self, val: super::vals::Pdi0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi1(&self) -> super::vals::Pdi1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdi1::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi1(&mut self, val: super::vals::Pdi1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi2(&self) -> super::vals::Pdi2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdi2::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi2(&mut self, val: super::vals::Pdi2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi3(&self) -> super::vals::Pdi3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdi3::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi3(&mut self, val: super::vals::Pdi3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi4(&self) -> super::vals::Pdi4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdi4::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi4(&mut self, val: super::vals::Pdi4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi5(&self) -> super::vals::Pdi5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdi5::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi5(&mut self, val: super::vals::Pdi5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi6(&self) -> super::vals::Pdi6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdi6::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi6(&mut self, val: super::vals::Pdi6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi7(&self) -> super::vals::Pdi7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdi7::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi7(&mut self, val: super::vals::Pdi7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi8(&self) -> super::vals::Pdi8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdi8::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi8(&mut self, val: super::vals::Pdi8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi9(&self) -> super::vals::Pdi9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdi9::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi9(&mut self, val: super::vals::Pdi9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi10(&self) -> super::vals::Pdi10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdi10::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi10(&mut self, val: super::vals::Pdi10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi11(&self) -> super::vals::Pdi11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdi11::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi11(&mut self, val: super::vals::Pdi11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi12(&self) -> super::vals::Pdi12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdi12::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi12(&mut self, val: super::vals::Pdi12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi13(&self) -> super::vals::Pdi13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdi13::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi13(&mut self, val: super::vals::Pdi13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi14(&self) -> super::vals::Pdi14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdi14::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi14(&mut self, val: super::vals::Pdi14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi15(&self) -> super::vals::Pdi15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdi15::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi15(&mut self, val: super::vals::Pdi15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi16(&self) -> super::vals::Pdi16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdi16::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi16(&mut self, val: super::vals::Pdi16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi17(&self) -> super::vals::Pdi17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdi17::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi17(&mut self, val: super::vals::Pdi17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi18(&self) -> super::vals::Pdi18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdi18::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi18(&mut self, val: super::vals::Pdi18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi19(&self) -> super::vals::Pdi19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdi19::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi19(&mut self, val: super::vals::Pdi19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi20(&self) -> super::vals::Pdi20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdi20::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi20(&mut self, val: super::vals::Pdi20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi21(&self) -> super::vals::Pdi21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdi21::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi21(&mut self, val: super::vals::Pdi21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi22(&self) -> super::vals::Pdi22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdi22::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi22(&mut self, val: super::vals::Pdi22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi23(&self) -> super::vals::Pdi23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdi23::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi23(&mut self, val: super::vals::Pdi23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi24(&self) -> super::vals::Pdi24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdi24::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi24(&mut self, val: super::vals::Pdi24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi25(&self) -> super::vals::Pdi25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdi25::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi25(&mut self, val: super::vals::Pdi25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi26(&self) -> super::vals::Pdi26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdi26::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi26(&mut self, val: super::vals::Pdi26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi27(&self) -> super::vals::Pdi27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdi27::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi27(&mut self, val: super::vals::Pdi27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi28(&self) -> super::vals::Pdi28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdi28::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi28(&mut self, val: super::vals::Pdi28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi29(&self) -> super::vals::Pdi29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdi29::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi29(&mut self, val: super::vals::Pdi29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi30(&self) -> super::vals::Pdi30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pdi30::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi30(&mut self, val: super::vals::Pdi30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi31(&self) -> super::vals::Pdi31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdi31::from_bits(val as u8)
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi31(&mut self, val: super::vals::Pdi31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("pdi0", &self.pdi0())
            .field("pdi1", &self.pdi1())
            .field("pdi2", &self.pdi2())
            .field("pdi3", &self.pdi3())
            .field("pdi4", &self.pdi4())
            .field("pdi5", &self.pdi5())
            .field("pdi6", &self.pdi6())
            .field("pdi7", &self.pdi7())
            .field("pdi8", &self.pdi8())
            .field("pdi9", &self.pdi9())
            .field("pdi10", &self.pdi10())
            .field("pdi11", &self.pdi11())
            .field("pdi12", &self.pdi12())
            .field("pdi13", &self.pdi13())
            .field("pdi14", &self.pdi14())
            .field("pdi15", &self.pdi15())
            .field("pdi16", &self.pdi16())
            .field("pdi17", &self.pdi17())
            .field("pdi18", &self.pdi18())
            .field("pdi19", &self.pdi19())
            .field("pdi20", &self.pdi20())
            .field("pdi21", &self.pdi21())
            .field("pdi22", &self.pdi22())
            .field("pdi23", &self.pdi23())
            .field("pdi24", &self.pdi24())
            .field("pdi25", &self.pdi25())
            .field("pdi26", &self.pdi26())
            .field("pdi27", &self.pdi27())
            .field("pdi28", &self.pdi28())
            .field("pdi29", &self.pdi29())
            .field("pdi30", &self.pdi30())
            .field("pdi31", &self.pdi31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdir {{ pdi0: {:?}, pdi1: {:?}, pdi2: {:?}, pdi3: {:?}, pdi4: {:?}, pdi5: {:?}, pdi6: {:?}, pdi7: {:?}, pdi8: {:?}, pdi9: {:?}, pdi10: {:?}, pdi11: {:?}, pdi12: {:?}, pdi13: {:?}, pdi14: {:?}, pdi15: {:?}, pdi16: {:?}, pdi17: {:?}, pdi18: {:?}, pdi19: {:?}, pdi20: {:?}, pdi21: {:?}, pdi22: {:?}, pdi23: {:?}, pdi24: {:?}, pdi25: {:?}, pdi26: {:?}, pdi27: {:?}, pdi28: {:?}, pdi29: {:?}, pdi30: {:?}, pdi31: {:?} }}",
            self.pdi0(),
            self.pdi1(),
            self.pdi2(),
            self.pdi3(),
            self.pdi4(),
            self.pdi5(),
            self.pdi6(),
            self.pdi7(),
            self.pdi8(),
            self.pdi9(),
            self.pdi10(),
            self.pdi11(),
            self.pdi12(),
            self.pdi13(),
            self.pdi14(),
            self.pdi15(),
            self.pdi16(),
            self.pdi17(),
            self.pdi18(),
            self.pdi19(),
            self.pdi20(),
            self.pdi21(),
            self.pdi22(),
            self.pdi23(),
            self.pdi24(),
            self.pdi25(),
            self.pdi26(),
            self.pdi27(),
            self.pdi28(),
            self.pdi29(),
            self.pdi30(),
            self.pdi31()
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
    pub const fn pdo0(&self) -> super::vals::Pdo0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdo0::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo0(&mut self, val: super::vals::Pdo0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo1(&self) -> super::vals::Pdo1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdo1::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo1(&mut self, val: super::vals::Pdo1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo2(&self) -> super::vals::Pdo2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdo2::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo2(&mut self, val: super::vals::Pdo2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo3(&self) -> super::vals::Pdo3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdo3::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo3(&mut self, val: super::vals::Pdo3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo4(&self) -> super::vals::Pdo4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdo4::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo4(&mut self, val: super::vals::Pdo4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo5(&self) -> super::vals::Pdo5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdo5::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo5(&mut self, val: super::vals::Pdo5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo6(&self) -> super::vals::Pdo6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdo6::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo6(&mut self, val: super::vals::Pdo6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo7(&self) -> super::vals::Pdo7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdo7::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo7(&mut self, val: super::vals::Pdo7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo8(&self) -> super::vals::Pdo8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdo8::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo8(&mut self, val: super::vals::Pdo8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo9(&self) -> super::vals::Pdo9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdo9::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo9(&mut self, val: super::vals::Pdo9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo10(&self) -> super::vals::Pdo10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdo10::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo10(&mut self, val: super::vals::Pdo10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo11(&self) -> super::vals::Pdo11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdo11::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo11(&mut self, val: super::vals::Pdo11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo12(&self) -> super::vals::Pdo12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdo12::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo12(&mut self, val: super::vals::Pdo12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo13(&self) -> super::vals::Pdo13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdo13::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo13(&mut self, val: super::vals::Pdo13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo14(&self) -> super::vals::Pdo14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdo14::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo14(&mut self, val: super::vals::Pdo14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo15(&self) -> super::vals::Pdo15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdo15::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo15(&mut self, val: super::vals::Pdo15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo16(&self) -> super::vals::Pdo16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdo16::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo16(&mut self, val: super::vals::Pdo16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo17(&self) -> super::vals::Pdo17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdo17::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo17(&mut self, val: super::vals::Pdo17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo18(&self) -> super::vals::Pdo18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdo18::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo18(&mut self, val: super::vals::Pdo18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo19(&self) -> super::vals::Pdo19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdo19::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo19(&mut self, val: super::vals::Pdo19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo20(&self) -> super::vals::Pdo20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdo20::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo20(&mut self, val: super::vals::Pdo20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo21(&self) -> super::vals::Pdo21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdo21::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo21(&mut self, val: super::vals::Pdo21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo22(&self) -> super::vals::Pdo22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdo22::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo22(&mut self, val: super::vals::Pdo22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo23(&self) -> super::vals::Pdo23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdo23::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo23(&mut self, val: super::vals::Pdo23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo24(&self) -> super::vals::Pdo24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdo24::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo24(&mut self, val: super::vals::Pdo24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo25(&self) -> super::vals::Pdo25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdo25::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo25(&mut self, val: super::vals::Pdo25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo26(&self) -> super::vals::Pdo26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdo26::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo26(&mut self, val: super::vals::Pdo26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo27(&self) -> super::vals::Pdo27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdo27::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo27(&mut self, val: super::vals::Pdo27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo28(&self) -> super::vals::Pdo28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdo28::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo28(&mut self, val: super::vals::Pdo28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo29(&self) -> super::vals::Pdo29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdo29::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo29(&mut self, val: super::vals::Pdo29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo30(&self) -> super::vals::Pdo30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pdo30::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo30(&mut self, val: super::vals::Pdo30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo31(&self) -> super::vals::Pdo31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdo31::from_bits(val as u8)
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo31(&mut self, val: super::vals::Pdo31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("pdo0", &self.pdo0())
            .field("pdo1", &self.pdo1())
            .field("pdo2", &self.pdo2())
            .field("pdo3", &self.pdo3())
            .field("pdo4", &self.pdo4())
            .field("pdo5", &self.pdo5())
            .field("pdo6", &self.pdo6())
            .field("pdo7", &self.pdo7())
            .field("pdo8", &self.pdo8())
            .field("pdo9", &self.pdo9())
            .field("pdo10", &self.pdo10())
            .field("pdo11", &self.pdo11())
            .field("pdo12", &self.pdo12())
            .field("pdo13", &self.pdo13())
            .field("pdo14", &self.pdo14())
            .field("pdo15", &self.pdo15())
            .field("pdo16", &self.pdo16())
            .field("pdo17", &self.pdo17())
            .field("pdo18", &self.pdo18())
            .field("pdo19", &self.pdo19())
            .field("pdo20", &self.pdo20())
            .field("pdo21", &self.pdo21())
            .field("pdo22", &self.pdo22())
            .field("pdo23", &self.pdo23())
            .field("pdo24", &self.pdo24())
            .field("pdo25", &self.pdo25())
            .field("pdo26", &self.pdo26())
            .field("pdo27", &self.pdo27())
            .field("pdo28", &self.pdo28())
            .field("pdo29", &self.pdo29())
            .field("pdo30", &self.pdo30())
            .field("pdo31", &self.pdo31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdor {{ pdo0: {:?}, pdo1: {:?}, pdo2: {:?}, pdo3: {:?}, pdo4: {:?}, pdo5: {:?}, pdo6: {:?}, pdo7: {:?}, pdo8: {:?}, pdo9: {:?}, pdo10: {:?}, pdo11: {:?}, pdo12: {:?}, pdo13: {:?}, pdo14: {:?}, pdo15: {:?}, pdo16: {:?}, pdo17: {:?}, pdo18: {:?}, pdo19: {:?}, pdo20: {:?}, pdo21: {:?}, pdo22: {:?}, pdo23: {:?}, pdo24: {:?}, pdo25: {:?}, pdo26: {:?}, pdo27: {:?}, pdo28: {:?}, pdo29: {:?}, pdo30: {:?}, pdo31: {:?} }}",
            self.pdo0(),
            self.pdo1(),
            self.pdo2(),
            self.pdo3(),
            self.pdo4(),
            self.pdo5(),
            self.pdo6(),
            self.pdo7(),
            self.pdo8(),
            self.pdo9(),
            self.pdo10(),
            self.pdo11(),
            self.pdo12(),
            self.pdo13(),
            self.pdo14(),
            self.pdo15(),
            self.pdo16(),
            self.pdo17(),
            self.pdo18(),
            self.pdo19(),
            self.pdo20(),
            self.pdo21(),
            self.pdo22(),
            self.pdo23(),
            self.pdo24(),
            self.pdo25(),
            self.pdo26(),
            self.pdo27(),
            self.pdo28(),
            self.pdo29(),
            self.pdo30(),
            self.pdo31()
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
    pub const fn pid0(&self) -> super::vals::Pid0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pid0::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid0(&mut self, val: super::vals::Pid0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid1(&self) -> super::vals::Pid1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pid1::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid1(&mut self, val: super::vals::Pid1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid2(&self) -> super::vals::Pid2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pid2::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid2(&mut self, val: super::vals::Pid2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid3(&self) -> super::vals::Pid3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pid3::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid3(&mut self, val: super::vals::Pid3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid4(&self) -> super::vals::Pid4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pid4::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid4(&mut self, val: super::vals::Pid4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid5(&self) -> super::vals::Pid5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pid5::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid5(&mut self, val: super::vals::Pid5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid6(&self) -> super::vals::Pid6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pid6::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid6(&mut self, val: super::vals::Pid6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid7(&self) -> super::vals::Pid7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pid7::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid7(&mut self, val: super::vals::Pid7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid8(&self) -> super::vals::Pid8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pid8::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid8(&mut self, val: super::vals::Pid8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid9(&self) -> super::vals::Pid9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pid9::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid9(&mut self, val: super::vals::Pid9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid10(&self) -> super::vals::Pid10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pid10::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid10(&mut self, val: super::vals::Pid10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid11(&self) -> super::vals::Pid11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pid11::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid11(&mut self, val: super::vals::Pid11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid12(&self) -> super::vals::Pid12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pid12::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid12(&mut self, val: super::vals::Pid12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid13(&self) -> super::vals::Pid13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pid13::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid13(&mut self, val: super::vals::Pid13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid14(&self) -> super::vals::Pid14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pid14::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid14(&mut self, val: super::vals::Pid14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid15(&self) -> super::vals::Pid15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pid15::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid15(&mut self, val: super::vals::Pid15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid16(&self) -> super::vals::Pid16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pid16::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid16(&mut self, val: super::vals::Pid16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid17(&self) -> super::vals::Pid17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pid17::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid17(&mut self, val: super::vals::Pid17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid18(&self) -> super::vals::Pid18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pid18::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid18(&mut self, val: super::vals::Pid18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid19(&self) -> super::vals::Pid19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pid19::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid19(&mut self, val: super::vals::Pid19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid20(&self) -> super::vals::Pid20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pid20::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid20(&mut self, val: super::vals::Pid20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid21(&self) -> super::vals::Pid21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pid21::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid21(&mut self, val: super::vals::Pid21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid22(&self) -> super::vals::Pid22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pid22::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid22(&mut self, val: super::vals::Pid22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid23(&self) -> super::vals::Pid23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pid23::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid23(&mut self, val: super::vals::Pid23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid24(&self) -> super::vals::Pid24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pid24::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid24(&mut self, val: super::vals::Pid24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid25(&self) -> super::vals::Pid25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pid25::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid25(&mut self, val: super::vals::Pid25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid26(&self) -> super::vals::Pid26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pid26::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid26(&mut self, val: super::vals::Pid26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid27(&self) -> super::vals::Pid27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pid27::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid27(&mut self, val: super::vals::Pid27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid28(&self) -> super::vals::Pid28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pid28::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid28(&mut self, val: super::vals::Pid28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid29(&self) -> super::vals::Pid29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pid29::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid29(&mut self, val: super::vals::Pid29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid30(&self) -> super::vals::Pid30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pid30::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid30(&mut self, val: super::vals::Pid30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid31(&self) -> super::vals::Pid31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pid31::from_bits(val as u8)
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid31(&mut self, val: super::vals::Pid31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("pid0", &self.pid0())
            .field("pid1", &self.pid1())
            .field("pid2", &self.pid2())
            .field("pid3", &self.pid3())
            .field("pid4", &self.pid4())
            .field("pid5", &self.pid5())
            .field("pid6", &self.pid6())
            .field("pid7", &self.pid7())
            .field("pid8", &self.pid8())
            .field("pid9", &self.pid9())
            .field("pid10", &self.pid10())
            .field("pid11", &self.pid11())
            .field("pid12", &self.pid12())
            .field("pid13", &self.pid13())
            .field("pid14", &self.pid14())
            .field("pid15", &self.pid15())
            .field("pid16", &self.pid16())
            .field("pid17", &self.pid17())
            .field("pid18", &self.pid18())
            .field("pid19", &self.pid19())
            .field("pid20", &self.pid20())
            .field("pid21", &self.pid21())
            .field("pid22", &self.pid22())
            .field("pid23", &self.pid23())
            .field("pid24", &self.pid24())
            .field("pid25", &self.pid25())
            .field("pid26", &self.pid26())
            .field("pid27", &self.pid27())
            .field("pid28", &self.pid28())
            .field("pid29", &self.pid29())
            .field("pid30", &self.pid30())
            .field("pid31", &self.pid31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pidr {{ pid0: {:?}, pid1: {:?}, pid2: {:?}, pid3: {:?}, pid4: {:?}, pid5: {:?}, pid6: {:?}, pid7: {:?}, pid8: {:?}, pid9: {:?}, pid10: {:?}, pid11: {:?}, pid12: {:?}, pid13: {:?}, pid14: {:?}, pid15: {:?}, pid16: {:?}, pid17: {:?}, pid18: {:?}, pid19: {:?}, pid20: {:?}, pid21: {:?}, pid22: {:?}, pid23: {:?}, pid24: {:?}, pid25: {:?}, pid26: {:?}, pid27: {:?}, pid28: {:?}, pid29: {:?}, pid30: {:?}, pid31: {:?} }}",
            self.pid0(),
            self.pid1(),
            self.pid2(),
            self.pid3(),
            self.pid4(),
            self.pid5(),
            self.pid6(),
            self.pid7(),
            self.pid8(),
            self.pid9(),
            self.pid10(),
            self.pid11(),
            self.pid12(),
            self.pid13(),
            self.pid14(),
            self.pid15(),
            self.pid16(),
            self.pid17(),
            self.pid18(),
            self.pid19(),
            self.pid20(),
            self.pid21(),
            self.pid22(),
            self.pid23(),
            self.pid24(),
            self.pid25(),
            self.pid26(),
            self.pid27(),
            self.pid28(),
            self.pid29(),
            self.pid30(),
            self.pid31()
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
    pub const fn ptso0(&self) -> super::vals::Ptso0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ptso0::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso0(&mut self, val: super::vals::Ptso0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso1(&self) -> super::vals::Ptso1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ptso1::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso1(&mut self, val: super::vals::Ptso1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso2(&self) -> super::vals::Ptso2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ptso2::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso2(&mut self, val: super::vals::Ptso2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso3(&self) -> super::vals::Ptso3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ptso3::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso3(&mut self, val: super::vals::Ptso3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso4(&self) -> super::vals::Ptso4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ptso4::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso4(&mut self, val: super::vals::Ptso4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso5(&self) -> super::vals::Ptso5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ptso5::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso5(&mut self, val: super::vals::Ptso5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso6(&self) -> super::vals::Ptso6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ptso6::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso6(&mut self, val: super::vals::Ptso6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso7(&self) -> super::vals::Ptso7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ptso7::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso7(&mut self, val: super::vals::Ptso7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso8(&self) -> super::vals::Ptso8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ptso8::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso8(&mut self, val: super::vals::Ptso8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso9(&self) -> super::vals::Ptso9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ptso9::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso9(&mut self, val: super::vals::Ptso9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso10(&self) -> super::vals::Ptso10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ptso10::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso10(&mut self, val: super::vals::Ptso10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso11(&self) -> super::vals::Ptso11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ptso11::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso11(&mut self, val: super::vals::Ptso11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso12(&self) -> super::vals::Ptso12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ptso12::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso12(&mut self, val: super::vals::Ptso12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso13(&self) -> super::vals::Ptso13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ptso13::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso13(&mut self, val: super::vals::Ptso13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso14(&self) -> super::vals::Ptso14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ptso14::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso14(&mut self, val: super::vals::Ptso14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso15(&self) -> super::vals::Ptso15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ptso15::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso15(&mut self, val: super::vals::Ptso15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso16(&self) -> super::vals::Ptso16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ptso16::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso16(&mut self, val: super::vals::Ptso16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso17(&self) -> super::vals::Ptso17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ptso17::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso17(&mut self, val: super::vals::Ptso17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso18(&self) -> super::vals::Ptso18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ptso18::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso18(&mut self, val: super::vals::Ptso18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso19(&self) -> super::vals::Ptso19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ptso19::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso19(&mut self, val: super::vals::Ptso19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso20(&self) -> super::vals::Ptso20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ptso20::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso20(&mut self, val: super::vals::Ptso20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso21(&self) -> super::vals::Ptso21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ptso21::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso21(&mut self, val: super::vals::Ptso21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso22(&self) -> super::vals::Ptso22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ptso22::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso22(&mut self, val: super::vals::Ptso22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso23(&self) -> super::vals::Ptso23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ptso23::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso23(&mut self, val: super::vals::Ptso23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso24(&self) -> super::vals::Ptso24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ptso24::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso24(&mut self, val: super::vals::Ptso24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso25(&self) -> super::vals::Ptso25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Ptso25::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso25(&mut self, val: super::vals::Ptso25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso26(&self) -> super::vals::Ptso26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ptso26::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso26(&mut self, val: super::vals::Ptso26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso27(&self) -> super::vals::Ptso27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ptso27::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso27(&mut self, val: super::vals::Ptso27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso28(&self) -> super::vals::Ptso28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ptso28::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso28(&mut self, val: super::vals::Ptso28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso29(&self) -> super::vals::Ptso29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Ptso29::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso29(&mut self, val: super::vals::Ptso29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso30(&self) -> super::vals::Ptso30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ptso30::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso30(&mut self, val: super::vals::Ptso30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso31(&self) -> super::vals::Ptso31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ptso31::from_bits(val as u8)
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso31(&mut self, val: super::vals::Ptso31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("ptso0", &self.ptso0())
            .field("ptso1", &self.ptso1())
            .field("ptso2", &self.ptso2())
            .field("ptso3", &self.ptso3())
            .field("ptso4", &self.ptso4())
            .field("ptso5", &self.ptso5())
            .field("ptso6", &self.ptso6())
            .field("ptso7", &self.ptso7())
            .field("ptso8", &self.ptso8())
            .field("ptso9", &self.ptso9())
            .field("ptso10", &self.ptso10())
            .field("ptso11", &self.ptso11())
            .field("ptso12", &self.ptso12())
            .field("ptso13", &self.ptso13())
            .field("ptso14", &self.ptso14())
            .field("ptso15", &self.ptso15())
            .field("ptso16", &self.ptso16())
            .field("ptso17", &self.ptso17())
            .field("ptso18", &self.ptso18())
            .field("ptso19", &self.ptso19())
            .field("ptso20", &self.ptso20())
            .field("ptso21", &self.ptso21())
            .field("ptso22", &self.ptso22())
            .field("ptso23", &self.ptso23())
            .field("ptso24", &self.ptso24())
            .field("ptso25", &self.ptso25())
            .field("ptso26", &self.ptso26())
            .field("ptso27", &self.ptso27())
            .field("ptso28", &self.ptso28())
            .field("ptso29", &self.ptso29())
            .field("ptso30", &self.ptso30())
            .field("ptso31", &self.ptso31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psor {{ ptso0: {:?}, ptso1: {:?}, ptso2: {:?}, ptso3: {:?}, ptso4: {:?}, ptso5: {:?}, ptso6: {:?}, ptso7: {:?}, ptso8: {:?}, ptso9: {:?}, ptso10: {:?}, ptso11: {:?}, ptso12: {:?}, ptso13: {:?}, ptso14: {:?}, ptso15: {:?}, ptso16: {:?}, ptso17: {:?}, ptso18: {:?}, ptso19: {:?}, ptso20: {:?}, ptso21: {:?}, ptso22: {:?}, ptso23: {:?}, ptso24: {:?}, ptso25: {:?}, ptso26: {:?}, ptso27: {:?}, ptso28: {:?}, ptso29: {:?}, ptso30: {:?}, ptso31: {:?} }}",
            self.ptso0(),
            self.ptso1(),
            self.ptso2(),
            self.ptso3(),
            self.ptso4(),
            self.ptso5(),
            self.ptso6(),
            self.ptso7(),
            self.ptso8(),
            self.ptso9(),
            self.ptso10(),
            self.ptso11(),
            self.ptso12(),
            self.ptso13(),
            self.ptso14(),
            self.ptso15(),
            self.ptso16(),
            self.ptso17(),
            self.ptso18(),
            self.ptso19(),
            self.ptso20(),
            self.ptso21(),
            self.ptso22(),
            self.ptso23(),
            self.ptso24(),
            self.ptso25(),
            self.ptso26(),
            self.ptso27(),
            self.ptso28(),
            self.ptso29(),
            self.ptso30(),
            self.ptso31()
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
    pub const fn ptto0(&self) -> super::vals::Ptto0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ptto0::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto0(&mut self, val: super::vals::Ptto0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto1(&self) -> super::vals::Ptto1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ptto1::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto1(&mut self, val: super::vals::Ptto1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto2(&self) -> super::vals::Ptto2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ptto2::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto2(&mut self, val: super::vals::Ptto2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto3(&self) -> super::vals::Ptto3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ptto3::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto3(&mut self, val: super::vals::Ptto3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto4(&self) -> super::vals::Ptto4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ptto4::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto4(&mut self, val: super::vals::Ptto4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto5(&self) -> super::vals::Ptto5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ptto5::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto5(&mut self, val: super::vals::Ptto5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto6(&self) -> super::vals::Ptto6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ptto6::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto6(&mut self, val: super::vals::Ptto6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto7(&self) -> super::vals::Ptto7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ptto7::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto7(&mut self, val: super::vals::Ptto7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto8(&self) -> super::vals::Ptto8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ptto8::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto8(&mut self, val: super::vals::Ptto8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto9(&self) -> super::vals::Ptto9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ptto9::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto9(&mut self, val: super::vals::Ptto9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto10(&self) -> super::vals::Ptto10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ptto10::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto10(&mut self, val: super::vals::Ptto10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto11(&self) -> super::vals::Ptto11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ptto11::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto11(&mut self, val: super::vals::Ptto11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto12(&self) -> super::vals::Ptto12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ptto12::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto12(&mut self, val: super::vals::Ptto12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto13(&self) -> super::vals::Ptto13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ptto13::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto13(&mut self, val: super::vals::Ptto13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto14(&self) -> super::vals::Ptto14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ptto14::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto14(&mut self, val: super::vals::Ptto14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto15(&self) -> super::vals::Ptto15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ptto15::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto15(&mut self, val: super::vals::Ptto15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto16(&self) -> super::vals::Ptto16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ptto16::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto16(&mut self, val: super::vals::Ptto16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto17(&self) -> super::vals::Ptto17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ptto17::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto17(&mut self, val: super::vals::Ptto17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto18(&self) -> super::vals::Ptto18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ptto18::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto18(&mut self, val: super::vals::Ptto18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto19(&self) -> super::vals::Ptto19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ptto19::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto19(&mut self, val: super::vals::Ptto19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto20(&self) -> super::vals::Ptto20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ptto20::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto20(&mut self, val: super::vals::Ptto20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto21(&self) -> super::vals::Ptto21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ptto21::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto21(&mut self, val: super::vals::Ptto21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto22(&self) -> super::vals::Ptto22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ptto22::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto22(&mut self, val: super::vals::Ptto22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto23(&self) -> super::vals::Ptto23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ptto23::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto23(&mut self, val: super::vals::Ptto23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto24(&self) -> super::vals::Ptto24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ptto24::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto24(&mut self, val: super::vals::Ptto24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto25(&self) -> super::vals::Ptto25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Ptto25::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto25(&mut self, val: super::vals::Ptto25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto26(&self) -> super::vals::Ptto26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ptto26::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto26(&mut self, val: super::vals::Ptto26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto27(&self) -> super::vals::Ptto27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ptto27::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto27(&mut self, val: super::vals::Ptto27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto28(&self) -> super::vals::Ptto28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ptto28::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto28(&mut self, val: super::vals::Ptto28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto29(&self) -> super::vals::Ptto29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Ptto29::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto29(&mut self, val: super::vals::Ptto29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto30(&self) -> super::vals::Ptto30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ptto30::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto30(&mut self, val: super::vals::Ptto30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto31(&self) -> super::vals::Ptto31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ptto31::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto31(&mut self, val: super::vals::Ptto31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("ptto0", &self.ptto0())
            .field("ptto1", &self.ptto1())
            .field("ptto2", &self.ptto2())
            .field("ptto3", &self.ptto3())
            .field("ptto4", &self.ptto4())
            .field("ptto5", &self.ptto5())
            .field("ptto6", &self.ptto6())
            .field("ptto7", &self.ptto7())
            .field("ptto8", &self.ptto8())
            .field("ptto9", &self.ptto9())
            .field("ptto10", &self.ptto10())
            .field("ptto11", &self.ptto11())
            .field("ptto12", &self.ptto12())
            .field("ptto13", &self.ptto13())
            .field("ptto14", &self.ptto14())
            .field("ptto15", &self.ptto15())
            .field("ptto16", &self.ptto16())
            .field("ptto17", &self.ptto17())
            .field("ptto18", &self.ptto18())
            .field("ptto19", &self.ptto19())
            .field("ptto20", &self.ptto20())
            .field("ptto21", &self.ptto21())
            .field("ptto22", &self.ptto22())
            .field("ptto23", &self.ptto23())
            .field("ptto24", &self.ptto24())
            .field("ptto25", &self.ptto25())
            .field("ptto26", &self.ptto26())
            .field("ptto27", &self.ptto27())
            .field("ptto28", &self.ptto28())
            .field("ptto29", &self.ptto29())
            .field("ptto30", &self.ptto30())
            .field("ptto31", &self.ptto31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ptor {{ ptto0: {:?}, ptto1: {:?}, ptto2: {:?}, ptto3: {:?}, ptto4: {:?}, ptto5: {:?}, ptto6: {:?}, ptto7: {:?}, ptto8: {:?}, ptto9: {:?}, ptto10: {:?}, ptto11: {:?}, ptto12: {:?}, ptto13: {:?}, ptto14: {:?}, ptto15: {:?}, ptto16: {:?}, ptto17: {:?}, ptto18: {:?}, ptto19: {:?}, ptto20: {:?}, ptto21: {:?}, ptto22: {:?}, ptto23: {:?}, ptto24: {:?}, ptto25: {:?}, ptto26: {:?}, ptto27: {:?}, ptto28: {:?}, ptto29: {:?}, ptto30: {:?}, ptto31: {:?} }}",
            self.ptto0(),
            self.ptto1(),
            self.ptto2(),
            self.ptto3(),
            self.ptto4(),
            self.ptto5(),
            self.ptto6(),
            self.ptto7(),
            self.ptto8(),
            self.ptto9(),
            self.ptto10(),
            self.ptto11(),
            self.ptto12(),
            self.ptto13(),
            self.ptto14(),
            self.ptto15(),
            self.ptto16(),
            self.ptto17(),
            self.ptto18(),
            self.ptto19(),
            self.ptto20(),
            self.ptto21(),
            self.ptto22(),
            self.ptto23(),
            self.ptto24(),
            self.ptto25(),
            self.ptto26(),
            self.ptto27(),
            self.ptto28(),
            self.ptto29(),
            self.ptto30(),
            self.ptto31()
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
