#[doc = "Coprocessor Power Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPPWR(pub u32);
impl CPPWR {
    #[doc = "State UNKNOWN 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SU0(&self) -> super::vals::SU0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SU0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 0."]
    #[inline(always)]
    pub const fn set_SU0(&mut self, val: super::vals::SU0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS0(&self) -> super::vals::SUS0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SUS0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub const fn set_SUS0(&mut self, val: super::vals::SUS0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "State UNKNOWN 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SU1(&self) -> super::vals::SU1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SU1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 1."]
    #[inline(always)]
    pub const fn set_SU1(&mut self, val: super::vals::SU1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS1(&self) -> super::vals::SUS1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SUS1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub const fn set_SUS1(&mut self, val: super::vals::SUS1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "State UNKNOWN 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SU2(&self) -> super::vals::SU2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SU2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 2."]
    #[inline(always)]
    pub const fn set_SU2(&mut self, val: super::vals::SU2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS2(&self) -> super::vals::SUS2 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SUS2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub const fn set_SUS2(&mut self, val: super::vals::SUS2) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "State UNKNOWN 3."]
    #[must_use]
    #[inline(always)]
    pub const fn SU3(&self) -> super::vals::SU3 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SU3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 3."]
    #[inline(always)]
    pub const fn set_SU3(&mut self, val: super::vals::SU3) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS3(&self) -> super::vals::SUS3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SUS3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub const fn set_SUS3(&mut self, val: super::vals::SUS3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "State UNKNOWN 4."]
    #[must_use]
    #[inline(always)]
    pub const fn SU4(&self) -> super::vals::SU4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SU4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 4."]
    #[inline(always)]
    pub const fn set_SU4(&mut self, val: super::vals::SU4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS4(&self) -> super::vals::SUS4 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SUS4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub const fn set_SUS4(&mut self, val: super::vals::SUS4) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "State UNKNOWN 5."]
    #[must_use]
    #[inline(always)]
    pub const fn SU5(&self) -> super::vals::SU5 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SU5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 5."]
    #[inline(always)]
    pub const fn set_SU5(&mut self, val: super::vals::SU5) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS5(&self) -> super::vals::SUS5 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SUS5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub const fn set_SUS5(&mut self, val: super::vals::SUS5) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "State UNKNOWN 6."]
    #[must_use]
    #[inline(always)]
    pub const fn SU6(&self) -> super::vals::SU6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SU6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 6."]
    #[inline(always)]
    pub const fn set_SU6(&mut self, val: super::vals::SU6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS6(&self) -> super::vals::SUS6 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SUS6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub const fn set_SUS6(&mut self, val: super::vals::SUS6) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "State UNKNOWN 7."]
    #[must_use]
    #[inline(always)]
    pub const fn SU7(&self) -> super::vals::SU7 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SU7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 7."]
    #[inline(always)]
    pub const fn set_SU7(&mut self, val: super::vals::SU7) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS7(&self) -> super::vals::SUS7 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SUS7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub const fn set_SUS7(&mut self, val: super::vals::SUS7) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "State UNKNOWN 10."]
    #[must_use]
    #[inline(always)]
    pub const fn SU10(&self) -> super::vals::SU10 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::SU10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 10."]
    #[inline(always)]
    pub const fn set_SU10(&mut self, val: super::vals::SU10) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS10(&self) -> super::vals::SUS10 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SUS10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub const fn set_SUS10(&mut self, val: super::vals::SUS10) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "State UNKNOWN 11."]
    #[must_use]
    #[inline(always)]
    pub const fn SU11(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN 11."]
    #[inline(always)]
    pub const fn set_SU11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[must_use]
    #[inline(always)]
    pub const fn SUS11(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub const fn set_SUS11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for CPPWR {
    #[inline(always)]
    fn default() -> CPPWR {
        CPPWR(0)
    }
}
impl core::fmt::Debug for CPPWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPPWR")
            .field("SU0", &self.SU0())
            .field("SUS0", &self.SUS0())
            .field("SU1", &self.SU1())
            .field("SUS1", &self.SUS1())
            .field("SU2", &self.SU2())
            .field("SUS2", &self.SUS2())
            .field("SU3", &self.SU3())
            .field("SUS3", &self.SUS3())
            .field("SU4", &self.SU4())
            .field("SUS4", &self.SUS4())
            .field("SU5", &self.SU5())
            .field("SUS5", &self.SUS5())
            .field("SU6", &self.SU6())
            .field("SUS6", &self.SUS6())
            .field("SU7", &self.SU7())
            .field("SUS7", &self.SUS7())
            .field("SU10", &self.SU10())
            .field("SUS10", &self.SUS10())
            .field("SU11", &self.SU11())
            .field("SUS11", &self.SUS11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPPWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPPWR {{ SU0: {:?}, SUS0: {:?}, SU1: {:?}, SUS1: {:?}, SU2: {:?}, SUS2: {:?}, SU3: {:?}, SUS3: {:?}, SU4: {:?}, SUS4: {:?}, SU5: {:?}, SUS5: {:?}, SU6: {:?}, SUS6: {:?}, SU7: {:?}, SUS7: {:?}, SU10: {:?}, SUS10: {:?}, SU11: {=bool:?}, SUS11: {=bool:?} }}",
            self.SU0(),
            self.SUS0(),
            self.SU1(),
            self.SUS1(),
            self.SU2(),
            self.SUS2(),
            self.SU3(),
            self.SUS3(),
            self.SU4(),
            self.SUS4(),
            self.SU5(),
            self.SUS5(),
            self.SU6(),
            self.SUS6(),
            self.SU7(),
            self.SUS7(),
            self.SU10(),
            self.SUS10(),
            self.SU11(),
            self.SUS11()
        )
    }
}
