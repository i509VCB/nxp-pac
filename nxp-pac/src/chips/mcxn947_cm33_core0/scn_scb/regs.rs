#[doc = "Coprocessor Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppwr(pub u32);
impl Cppwr {
    #[doc = "State UNKNOWN 0."]
    #[must_use]
    #[inline(always)]
    pub const fn su0(&self) -> super::vals::Su0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Su0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 0."]
    #[inline(always)]
    pub const fn set_su0(&mut self, val: super::vals::Su0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sus0(&self) -> super::vals::Sus0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sus0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub const fn set_sus0(&mut self, val: super::vals::Sus0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "State UNKNOWN 1."]
    #[must_use]
    #[inline(always)]
    pub const fn su1(&self) -> super::vals::Su1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Su1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 1."]
    #[inline(always)]
    pub const fn set_su1(&mut self, val: super::vals::Su1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[must_use]
    #[inline(always)]
    pub const fn sus1(&self) -> super::vals::Sus1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sus1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub const fn set_sus1(&mut self, val: super::vals::Sus1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "State UNKNOWN 2."]
    #[must_use]
    #[inline(always)]
    pub const fn su2(&self) -> super::vals::Su2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Su2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 2."]
    #[inline(always)]
    pub const fn set_su2(&mut self, val: super::vals::Su2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[must_use]
    #[inline(always)]
    pub const fn sus2(&self) -> super::vals::Sus2 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sus2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub const fn set_sus2(&mut self, val: super::vals::Sus2) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "State UNKNOWN 3."]
    #[must_use]
    #[inline(always)]
    pub const fn su3(&self) -> super::vals::Su3 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Su3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 3."]
    #[inline(always)]
    pub const fn set_su3(&mut self, val: super::vals::Su3) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[must_use]
    #[inline(always)]
    pub const fn sus3(&self) -> super::vals::Sus3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sus3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub const fn set_sus3(&mut self, val: super::vals::Sus3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "State UNKNOWN 4."]
    #[must_use]
    #[inline(always)]
    pub const fn su4(&self) -> super::vals::Su4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Su4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 4."]
    #[inline(always)]
    pub const fn set_su4(&mut self, val: super::vals::Su4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[must_use]
    #[inline(always)]
    pub const fn sus4(&self) -> super::vals::Sus4 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sus4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub const fn set_sus4(&mut self, val: super::vals::Sus4) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "State UNKNOWN 5."]
    #[must_use]
    #[inline(always)]
    pub const fn su5(&self) -> super::vals::Su5 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Su5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 5."]
    #[inline(always)]
    pub const fn set_su5(&mut self, val: super::vals::Su5) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[must_use]
    #[inline(always)]
    pub const fn sus5(&self) -> super::vals::Sus5 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sus5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub const fn set_sus5(&mut self, val: super::vals::Sus5) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "State UNKNOWN 6."]
    #[must_use]
    #[inline(always)]
    pub const fn su6(&self) -> super::vals::Su6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Su6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 6."]
    #[inline(always)]
    pub const fn set_su6(&mut self, val: super::vals::Su6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[must_use]
    #[inline(always)]
    pub const fn sus6(&self) -> super::vals::Sus6 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sus6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub const fn set_sus6(&mut self, val: super::vals::Sus6) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "State UNKNOWN 7."]
    #[must_use]
    #[inline(always)]
    pub const fn su7(&self) -> super::vals::Su7 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Su7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 7."]
    #[inline(always)]
    pub const fn set_su7(&mut self, val: super::vals::Su7) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[must_use]
    #[inline(always)]
    pub const fn sus7(&self) -> super::vals::Sus7 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sus7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub const fn set_sus7(&mut self, val: super::vals::Sus7) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "State UNKNOWN 10."]
    #[must_use]
    #[inline(always)]
    pub const fn su10(&self) -> super::vals::Su10 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Su10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 10."]
    #[inline(always)]
    pub const fn set_su10(&mut self, val: super::vals::Su10) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[must_use]
    #[inline(always)]
    pub const fn sus10(&self) -> super::vals::Sus10 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sus10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub const fn set_sus10(&mut self, val: super::vals::Sus10) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "State UNKNOWN 11."]
    #[must_use]
    #[inline(always)]
    pub const fn su11(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN 11."]
    #[inline(always)]
    pub const fn set_su11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[must_use]
    #[inline(always)]
    pub const fn sus11(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub const fn set_sus11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Cppwr {
    #[inline(always)]
    fn default() -> Cppwr {
        Cppwr(0)
    }
}
impl core::fmt::Debug for Cppwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cppwr")
            .field("su0", &self.su0())
            .field("sus0", &self.sus0())
            .field("su1", &self.su1())
            .field("sus1", &self.sus1())
            .field("su2", &self.su2())
            .field("sus2", &self.sus2())
            .field("su3", &self.su3())
            .field("sus3", &self.sus3())
            .field("su4", &self.su4())
            .field("sus4", &self.sus4())
            .field("su5", &self.su5())
            .field("sus5", &self.sus5())
            .field("su6", &self.su6())
            .field("sus6", &self.sus6())
            .field("su7", &self.su7())
            .field("sus7", &self.sus7())
            .field("su10", &self.su10())
            .field("sus10", &self.sus10())
            .field("su11", &self.su11())
            .field("sus11", &self.sus11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cppwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cppwr {{ su0: {:?}, sus0: {:?}, su1: {:?}, sus1: {:?}, su2: {:?}, sus2: {:?}, su3: {:?}, sus3: {:?}, su4: {:?}, sus4: {:?}, su5: {:?}, sus5: {:?}, su6: {:?}, sus6: {:?}, su7: {:?}, sus7: {:?}, su10: {:?}, sus10: {:?}, su11: {=bool:?}, sus11: {=bool:?} }}",
            self.su0(),
            self.sus0(),
            self.su1(),
            self.sus1(),
            self.su2(),
            self.sus2(),
            self.su3(),
            self.sus3(),
            self.su4(),
            self.sus4(),
            self.su5(),
            self.sus5(),
            self.su6(),
            self.sus6(),
            self.su7(),
            self.sus7(),
            self.su10(),
            self.sus10(),
            self.su11(),
            self.sus11()
        )
    }
}
