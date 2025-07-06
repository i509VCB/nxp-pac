#[doc = "Interrupt Active Bit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iabr(pub u32);
impl Iabr {
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active0(&self) -> super::vals::Active0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Active0::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active0(&mut self, val: super::vals::Active0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active1(&self) -> super::vals::Active1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Active1::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active1(&mut self, val: super::vals::Active1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active2(&self) -> super::vals::Active2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Active2::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active2(&mut self, val: super::vals::Active2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active3(&self) -> super::vals::Active3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Active3::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active3(&mut self, val: super::vals::Active3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active4(&self) -> super::vals::Active4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Active4::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active4(&mut self, val: super::vals::Active4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active5(&self) -> super::vals::Active5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Active5::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active5(&mut self, val: super::vals::Active5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active6(&self) -> super::vals::Active6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Active6::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active6(&mut self, val: super::vals::Active6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active7(&self) -> super::vals::Active7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Active7::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active7(&mut self, val: super::vals::Active7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active8(&self) -> super::vals::Active8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Active8::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active8(&mut self, val: super::vals::Active8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active9(&self) -> super::vals::Active9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Active9::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active9(&mut self, val: super::vals::Active9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active10(&self) -> super::vals::Active10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Active10::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active10(&mut self, val: super::vals::Active10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active11(&self) -> super::vals::Active11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Active11::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active11(&mut self, val: super::vals::Active11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active12(&self) -> super::vals::Active12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Active12::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active12(&mut self, val: super::vals::Active12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active13(&self) -> super::vals::Active13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Active13::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active13(&mut self, val: super::vals::Active13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active14(&self) -> super::vals::Active14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Active14::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active14(&mut self, val: super::vals::Active14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active15(&self) -> super::vals::Active15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Active15::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active15(&mut self, val: super::vals::Active15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active16(&self) -> super::vals::Active16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Active16::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active16(&mut self, val: super::vals::Active16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active17(&self) -> super::vals::Active17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Active17::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active17(&mut self, val: super::vals::Active17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active18(&self) -> super::vals::Active18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Active18::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active18(&mut self, val: super::vals::Active18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active19(&self) -> super::vals::Active19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Active19::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active19(&mut self, val: super::vals::Active19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active20(&self) -> super::vals::Active20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Active20::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active20(&mut self, val: super::vals::Active20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active21(&self) -> super::vals::Active21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Active21::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active21(&mut self, val: super::vals::Active21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active22(&self) -> super::vals::Active22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Active22::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active22(&mut self, val: super::vals::Active22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active23(&self) -> super::vals::Active23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Active23::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active23(&mut self, val: super::vals::Active23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active24(&self) -> super::vals::Active24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Active24::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active24(&mut self, val: super::vals::Active24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active25(&self) -> super::vals::Active25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Active25::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active25(&mut self, val: super::vals::Active25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active26(&self) -> super::vals::Active26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Active26::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active26(&mut self, val: super::vals::Active26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active27(&self) -> super::vals::Active27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Active27::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active27(&mut self, val: super::vals::Active27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active28(&self) -> super::vals::Active28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Active28::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active28(&mut self, val: super::vals::Active28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active29(&self) -> super::vals::Active29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Active29::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active29(&mut self, val: super::vals::Active29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active30(&self) -> super::vals::Active30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Active30::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active30(&mut self, val: super::vals::Active30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn active31(&self) -> super::vals::Active31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Active31::from_bits(val as u8)
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_active31(&mut self, val: super::vals::Active31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Iabr {
    #[inline(always)]
    fn default() -> Iabr {
        Iabr(0)
    }
}
impl core::fmt::Debug for Iabr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iabr")
            .field("active0", &self.active0())
            .field("active1", &self.active1())
            .field("active2", &self.active2())
            .field("active3", &self.active3())
            .field("active4", &self.active4())
            .field("active5", &self.active5())
            .field("active6", &self.active6())
            .field("active7", &self.active7())
            .field("active8", &self.active8())
            .field("active9", &self.active9())
            .field("active10", &self.active10())
            .field("active11", &self.active11())
            .field("active12", &self.active12())
            .field("active13", &self.active13())
            .field("active14", &self.active14())
            .field("active15", &self.active15())
            .field("active16", &self.active16())
            .field("active17", &self.active17())
            .field("active18", &self.active18())
            .field("active19", &self.active19())
            .field("active20", &self.active20())
            .field("active21", &self.active21())
            .field("active22", &self.active22())
            .field("active23", &self.active23())
            .field("active24", &self.active24())
            .field("active25", &self.active25())
            .field("active26", &self.active26())
            .field("active27", &self.active27())
            .field("active28", &self.active28())
            .field("active29", &self.active29())
            .field("active30", &self.active30())
            .field("active31", &self.active31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iabr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iabr {{ active0: {:?}, active1: {:?}, active2: {:?}, active3: {:?}, active4: {:?}, active5: {:?}, active6: {:?}, active7: {:?}, active8: {:?}, active9: {:?}, active10: {:?}, active11: {:?}, active12: {:?}, active13: {:?}, active14: {:?}, active15: {:?}, active16: {:?}, active17: {:?}, active18: {:?}, active19: {:?}, active20: {:?}, active21: {:?}, active22: {:?}, active23: {:?}, active24: {:?}, active25: {:?}, active26: {:?}, active27: {:?}, active28: {:?}, active29: {:?}, active30: {:?}, active31: {:?} }}",
            self.active0(),
            self.active1(),
            self.active2(),
            self.active3(),
            self.active4(),
            self.active5(),
            self.active6(),
            self.active7(),
            self.active8(),
            self.active9(),
            self.active10(),
            self.active11(),
            self.active12(),
            self.active13(),
            self.active14(),
            self.active15(),
            self.active16(),
            self.active17(),
            self.active18(),
            self.active19(),
            self.active20(),
            self.active21(),
            self.active22(),
            self.active23(),
            self.active24(),
            self.active25(),
            self.active26(),
            self.active27(),
            self.active28(),
            self.active29(),
            self.active30(),
            self.active31()
        )
    }
}
#[doc = "Interrupt Clear Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icer(pub u32);
impl Icer {
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena0(&self) -> super::vals::Clrena0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clrena0::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena0(&mut self, val: super::vals::Clrena0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena1(&self) -> super::vals::Clrena1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Clrena1::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena1(&mut self, val: super::vals::Clrena1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena2(&self) -> super::vals::Clrena2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Clrena2::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena2(&mut self, val: super::vals::Clrena2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena3(&self) -> super::vals::Clrena3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Clrena3::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena3(&mut self, val: super::vals::Clrena3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena4(&self) -> super::vals::Clrena4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Clrena4::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena4(&mut self, val: super::vals::Clrena4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena5(&self) -> super::vals::Clrena5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Clrena5::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena5(&mut self, val: super::vals::Clrena5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena6(&self) -> super::vals::Clrena6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clrena6::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena6(&mut self, val: super::vals::Clrena6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena7(&self) -> super::vals::Clrena7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Clrena7::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena7(&mut self, val: super::vals::Clrena7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena8(&self) -> super::vals::Clrena8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Clrena8::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena8(&mut self, val: super::vals::Clrena8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena9(&self) -> super::vals::Clrena9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Clrena9::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena9(&mut self, val: super::vals::Clrena9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena10(&self) -> super::vals::Clrena10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Clrena10::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena10(&mut self, val: super::vals::Clrena10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena11(&self) -> super::vals::Clrena11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrena11::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena11(&mut self, val: super::vals::Clrena11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena12(&self) -> super::vals::Clrena12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Clrena12::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena12(&mut self, val: super::vals::Clrena12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena13(&self) -> super::vals::Clrena13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Clrena13::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena13(&mut self, val: super::vals::Clrena13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena14(&self) -> super::vals::Clrena14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Clrena14::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena14(&mut self, val: super::vals::Clrena14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena15(&self) -> super::vals::Clrena15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Clrena15::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena15(&mut self, val: super::vals::Clrena15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena16(&self) -> super::vals::Clrena16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Clrena16::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena16(&mut self, val: super::vals::Clrena16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena17(&self) -> super::vals::Clrena17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Clrena17::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena17(&mut self, val: super::vals::Clrena17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena18(&self) -> super::vals::Clrena18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Clrena18::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena18(&mut self, val: super::vals::Clrena18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena19(&self) -> super::vals::Clrena19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Clrena19::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena19(&mut self, val: super::vals::Clrena19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena20(&self) -> super::vals::Clrena20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Clrena20::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena20(&mut self, val: super::vals::Clrena20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena21(&self) -> super::vals::Clrena21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Clrena21::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena21(&mut self, val: super::vals::Clrena21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena22(&self) -> super::vals::Clrena22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Clrena22::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena22(&mut self, val: super::vals::Clrena22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena23(&self) -> super::vals::Clrena23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Clrena23::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena23(&mut self, val: super::vals::Clrena23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena24(&self) -> super::vals::Clrena24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Clrena24::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena24(&mut self, val: super::vals::Clrena24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena25(&self) -> super::vals::Clrena25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Clrena25::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena25(&mut self, val: super::vals::Clrena25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena26(&self) -> super::vals::Clrena26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Clrena26::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena26(&mut self, val: super::vals::Clrena26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena27(&self) -> super::vals::Clrena27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clrena27::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena27(&mut self, val: super::vals::Clrena27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena28(&self) -> super::vals::Clrena28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Clrena28::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena28(&mut self, val: super::vals::Clrena28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena29(&self) -> super::vals::Clrena29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Clrena29::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena29(&mut self, val: super::vals::Clrena29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena30(&self) -> super::vals::Clrena30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Clrena30::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena30(&mut self, val: super::vals::Clrena30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrena31(&self) -> super::vals::Clrena31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Clrena31::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_clrena31(&mut self, val: super::vals::Clrena31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Icer {
    #[inline(always)]
    fn default() -> Icer {
        Icer(0)
    }
}
impl core::fmt::Debug for Icer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icer")
            .field("clrena0", &self.clrena0())
            .field("clrena1", &self.clrena1())
            .field("clrena2", &self.clrena2())
            .field("clrena3", &self.clrena3())
            .field("clrena4", &self.clrena4())
            .field("clrena5", &self.clrena5())
            .field("clrena6", &self.clrena6())
            .field("clrena7", &self.clrena7())
            .field("clrena8", &self.clrena8())
            .field("clrena9", &self.clrena9())
            .field("clrena10", &self.clrena10())
            .field("clrena11", &self.clrena11())
            .field("clrena12", &self.clrena12())
            .field("clrena13", &self.clrena13())
            .field("clrena14", &self.clrena14())
            .field("clrena15", &self.clrena15())
            .field("clrena16", &self.clrena16())
            .field("clrena17", &self.clrena17())
            .field("clrena18", &self.clrena18())
            .field("clrena19", &self.clrena19())
            .field("clrena20", &self.clrena20())
            .field("clrena21", &self.clrena21())
            .field("clrena22", &self.clrena22())
            .field("clrena23", &self.clrena23())
            .field("clrena24", &self.clrena24())
            .field("clrena25", &self.clrena25())
            .field("clrena26", &self.clrena26())
            .field("clrena27", &self.clrena27())
            .field("clrena28", &self.clrena28())
            .field("clrena29", &self.clrena29())
            .field("clrena30", &self.clrena30())
            .field("clrena31", &self.clrena31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icer {{ clrena0: {:?}, clrena1: {:?}, clrena2: {:?}, clrena3: {:?}, clrena4: {:?}, clrena5: {:?}, clrena6: {:?}, clrena7: {:?}, clrena8: {:?}, clrena9: {:?}, clrena10: {:?}, clrena11: {:?}, clrena12: {:?}, clrena13: {:?}, clrena14: {:?}, clrena15: {:?}, clrena16: {:?}, clrena17: {:?}, clrena18: {:?}, clrena19: {:?}, clrena20: {:?}, clrena21: {:?}, clrena22: {:?}, clrena23: {:?}, clrena24: {:?}, clrena25: {:?}, clrena26: {:?}, clrena27: {:?}, clrena28: {:?}, clrena29: {:?}, clrena30: {:?}, clrena31: {:?} }}",
            self.clrena0(),
            self.clrena1(),
            self.clrena2(),
            self.clrena3(),
            self.clrena4(),
            self.clrena5(),
            self.clrena6(),
            self.clrena7(),
            self.clrena8(),
            self.clrena9(),
            self.clrena10(),
            self.clrena11(),
            self.clrena12(),
            self.clrena13(),
            self.clrena14(),
            self.clrena15(),
            self.clrena16(),
            self.clrena17(),
            self.clrena18(),
            self.clrena19(),
            self.clrena20(),
            self.clrena21(),
            self.clrena22(),
            self.clrena23(),
            self.clrena24(),
            self.clrena25(),
            self.clrena26(),
            self.clrena27(),
            self.clrena28(),
            self.clrena29(),
            self.clrena30(),
            self.clrena31()
        )
    }
}
#[doc = "Interrupt Clear Pending Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icpr(pub u32);
impl Icpr {
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend0(&self) -> super::vals::Clrpend0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clrpend0::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend0(&mut self, val: super::vals::Clrpend0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend1(&self) -> super::vals::Clrpend1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Clrpend1::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend1(&mut self, val: super::vals::Clrpend1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend2(&self) -> super::vals::Clrpend2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Clrpend2::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend2(&mut self, val: super::vals::Clrpend2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend3(&self) -> super::vals::Clrpend3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Clrpend3::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend3(&mut self, val: super::vals::Clrpend3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend4(&self) -> super::vals::Clrpend4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Clrpend4::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend4(&mut self, val: super::vals::Clrpend4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend5(&self) -> super::vals::Clrpend5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Clrpend5::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend5(&mut self, val: super::vals::Clrpend5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend6(&self) -> super::vals::Clrpend6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clrpend6::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend6(&mut self, val: super::vals::Clrpend6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend7(&self) -> super::vals::Clrpend7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Clrpend7::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend7(&mut self, val: super::vals::Clrpend7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend8(&self) -> super::vals::Clrpend8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Clrpend8::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend8(&mut self, val: super::vals::Clrpend8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend9(&self) -> super::vals::Clrpend9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Clrpend9::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend9(&mut self, val: super::vals::Clrpend9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend10(&self) -> super::vals::Clrpend10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Clrpend10::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend10(&mut self, val: super::vals::Clrpend10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend11(&self) -> super::vals::Clrpend11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrpend11::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend11(&mut self, val: super::vals::Clrpend11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend12(&self) -> super::vals::Clrpend12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Clrpend12::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend12(&mut self, val: super::vals::Clrpend12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend13(&self) -> super::vals::Clrpend13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Clrpend13::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend13(&mut self, val: super::vals::Clrpend13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend14(&self) -> super::vals::Clrpend14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Clrpend14::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend14(&mut self, val: super::vals::Clrpend14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend15(&self) -> super::vals::Clrpend15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Clrpend15::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend15(&mut self, val: super::vals::Clrpend15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend16(&self) -> super::vals::Clrpend16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Clrpend16::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend16(&mut self, val: super::vals::Clrpend16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend17(&self) -> super::vals::Clrpend17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Clrpend17::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend17(&mut self, val: super::vals::Clrpend17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend18(&self) -> super::vals::Clrpend18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Clrpend18::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend18(&mut self, val: super::vals::Clrpend18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend19(&self) -> super::vals::Clrpend19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Clrpend19::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend19(&mut self, val: super::vals::Clrpend19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend20(&self) -> super::vals::Clrpend20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Clrpend20::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend20(&mut self, val: super::vals::Clrpend20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend21(&self) -> super::vals::Clrpend21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Clrpend21::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend21(&mut self, val: super::vals::Clrpend21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend22(&self) -> super::vals::Clrpend22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Clrpend22::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend22(&mut self, val: super::vals::Clrpend22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend23(&self) -> super::vals::Clrpend23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Clrpend23::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend23(&mut self, val: super::vals::Clrpend23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend24(&self) -> super::vals::Clrpend24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Clrpend24::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend24(&mut self, val: super::vals::Clrpend24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend25(&self) -> super::vals::Clrpend25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Clrpend25::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend25(&mut self, val: super::vals::Clrpend25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend26(&self) -> super::vals::Clrpend26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Clrpend26::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend26(&mut self, val: super::vals::Clrpend26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend27(&self) -> super::vals::Clrpend27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clrpend27::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend27(&mut self, val: super::vals::Clrpend27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend28(&self) -> super::vals::Clrpend28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Clrpend28::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend28(&mut self, val: super::vals::Clrpend28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend29(&self) -> super::vals::Clrpend29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Clrpend29::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend29(&mut self, val: super::vals::Clrpend29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend30(&self) -> super::vals::Clrpend30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Clrpend30::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend30(&mut self, val: super::vals::Clrpend30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn clrpend31(&self) -> super::vals::Clrpend31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Clrpend31::from_bits(val as u8)
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_clrpend31(&mut self, val: super::vals::Clrpend31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Icpr {
    #[inline(always)]
    fn default() -> Icpr {
        Icpr(0)
    }
}
impl core::fmt::Debug for Icpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icpr")
            .field("clrpend0", &self.clrpend0())
            .field("clrpend1", &self.clrpend1())
            .field("clrpend2", &self.clrpend2())
            .field("clrpend3", &self.clrpend3())
            .field("clrpend4", &self.clrpend4())
            .field("clrpend5", &self.clrpend5())
            .field("clrpend6", &self.clrpend6())
            .field("clrpend7", &self.clrpend7())
            .field("clrpend8", &self.clrpend8())
            .field("clrpend9", &self.clrpend9())
            .field("clrpend10", &self.clrpend10())
            .field("clrpend11", &self.clrpend11())
            .field("clrpend12", &self.clrpend12())
            .field("clrpend13", &self.clrpend13())
            .field("clrpend14", &self.clrpend14())
            .field("clrpend15", &self.clrpend15())
            .field("clrpend16", &self.clrpend16())
            .field("clrpend17", &self.clrpend17())
            .field("clrpend18", &self.clrpend18())
            .field("clrpend19", &self.clrpend19())
            .field("clrpend20", &self.clrpend20())
            .field("clrpend21", &self.clrpend21())
            .field("clrpend22", &self.clrpend22())
            .field("clrpend23", &self.clrpend23())
            .field("clrpend24", &self.clrpend24())
            .field("clrpend25", &self.clrpend25())
            .field("clrpend26", &self.clrpend26())
            .field("clrpend27", &self.clrpend27())
            .field("clrpend28", &self.clrpend28())
            .field("clrpend29", &self.clrpend29())
            .field("clrpend30", &self.clrpend30())
            .field("clrpend31", &self.clrpend31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icpr {{ clrpend0: {:?}, clrpend1: {:?}, clrpend2: {:?}, clrpend3: {:?}, clrpend4: {:?}, clrpend5: {:?}, clrpend6: {:?}, clrpend7: {:?}, clrpend8: {:?}, clrpend9: {:?}, clrpend10: {:?}, clrpend11: {:?}, clrpend12: {:?}, clrpend13: {:?}, clrpend14: {:?}, clrpend15: {:?}, clrpend16: {:?}, clrpend17: {:?}, clrpend18: {:?}, clrpend19: {:?}, clrpend20: {:?}, clrpend21: {:?}, clrpend22: {:?}, clrpend23: {:?}, clrpend24: {:?}, clrpend25: {:?}, clrpend26: {:?}, clrpend27: {:?}, clrpend28: {:?}, clrpend29: {:?}, clrpend30: {:?}, clrpend31: {:?} }}",
            self.clrpend0(),
            self.clrpend1(),
            self.clrpend2(),
            self.clrpend3(),
            self.clrpend4(),
            self.clrpend5(),
            self.clrpend6(),
            self.clrpend7(),
            self.clrpend8(),
            self.clrpend9(),
            self.clrpend10(),
            self.clrpend11(),
            self.clrpend12(),
            self.clrpend13(),
            self.clrpend14(),
            self.clrpend15(),
            self.clrpend16(),
            self.clrpend17(),
            self.clrpend18(),
            self.clrpend19(),
            self.clrpend20(),
            self.clrpend21(),
            self.clrpend22(),
            self.clrpend23(),
            self.clrpend24(),
            self.clrpend25(),
            self.clrpend26(),
            self.clrpend27(),
            self.clrpend28(),
            self.clrpend29(),
            self.clrpend30(),
            self.clrpend31()
        )
    }
}
#[doc = "Interrupt Priority Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipr(pub u32);
impl Ipr {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pri_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pri_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pri_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pri_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ipr {
    #[inline(always)]
    fn default() -> Ipr {
        Ipr(0)
    }
}
impl core::fmt::Debug for Ipr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipr")
            .field("pri_0", &self.pri_0())
            .field("pri_1", &self.pri_1())
            .field("pri_2", &self.pri_2())
            .field("pri_3", &self.pri_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipr {{ pri_0: {=u8:?}, pri_1: {=u8:?}, pri_2: {=u8:?}, pri_3: {=u8:?} }}",
            self.pri_0(),
            self.pri_1(),
            self.pri_2(),
            self.pri_3()
        )
    }
}
#[doc = "Interrupt Set Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iser(pub u32);
impl Iser {
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena0(&self) -> super::vals::Setena0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Setena0::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena0(&mut self, val: super::vals::Setena0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena1(&self) -> super::vals::Setena1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Setena1::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena1(&mut self, val: super::vals::Setena1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena2(&self) -> super::vals::Setena2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Setena2::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena2(&mut self, val: super::vals::Setena2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena3(&self) -> super::vals::Setena3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Setena3::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena3(&mut self, val: super::vals::Setena3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena4(&self) -> super::vals::Setena4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Setena4::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena4(&mut self, val: super::vals::Setena4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena5(&self) -> super::vals::Setena5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Setena5::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena5(&mut self, val: super::vals::Setena5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena6(&self) -> super::vals::Setena6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Setena6::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena6(&mut self, val: super::vals::Setena6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena7(&self) -> super::vals::Setena7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Setena7::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena7(&mut self, val: super::vals::Setena7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena8(&self) -> super::vals::Setena8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Setena8::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena8(&mut self, val: super::vals::Setena8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena9(&self) -> super::vals::Setena9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Setena9::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena9(&mut self, val: super::vals::Setena9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena10(&self) -> super::vals::Setena10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Setena10::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena10(&mut self, val: super::vals::Setena10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena11(&self) -> super::vals::Setena11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Setena11::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena11(&mut self, val: super::vals::Setena11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena12(&self) -> super::vals::Setena12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Setena12::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena12(&mut self, val: super::vals::Setena12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena13(&self) -> super::vals::Setena13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Setena13::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena13(&mut self, val: super::vals::Setena13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena14(&self) -> super::vals::Setena14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Setena14::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena14(&mut self, val: super::vals::Setena14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena15(&self) -> super::vals::Setena15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Setena15::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena15(&mut self, val: super::vals::Setena15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena16(&self) -> super::vals::Setena16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Setena16::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena16(&mut self, val: super::vals::Setena16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena17(&self) -> super::vals::Setena17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Setena17::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena17(&mut self, val: super::vals::Setena17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena18(&self) -> super::vals::Setena18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Setena18::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena18(&mut self, val: super::vals::Setena18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena19(&self) -> super::vals::Setena19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Setena19::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena19(&mut self, val: super::vals::Setena19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena20(&self) -> super::vals::Setena20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Setena20::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena20(&mut self, val: super::vals::Setena20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena21(&self) -> super::vals::Setena21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Setena21::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena21(&mut self, val: super::vals::Setena21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena22(&self) -> super::vals::Setena22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Setena22::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena22(&mut self, val: super::vals::Setena22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena23(&self) -> super::vals::Setena23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Setena23::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena23(&mut self, val: super::vals::Setena23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena24(&self) -> super::vals::Setena24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Setena24::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena24(&mut self, val: super::vals::Setena24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena25(&self) -> super::vals::Setena25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Setena25::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena25(&mut self, val: super::vals::Setena25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena26(&self) -> super::vals::Setena26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Setena26::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena26(&mut self, val: super::vals::Setena26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena27(&self) -> super::vals::Setena27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Setena27::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena27(&mut self, val: super::vals::Setena27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena28(&self) -> super::vals::Setena28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Setena28::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena28(&mut self, val: super::vals::Setena28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena29(&self) -> super::vals::Setena29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Setena29::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena29(&mut self, val: super::vals::Setena29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena30(&self) -> super::vals::Setena30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Setena30::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena30(&mut self, val: super::vals::Setena30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setena31(&self) -> super::vals::Setena31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Setena31::from_bits(val as u8)
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_setena31(&mut self, val: super::vals::Setena31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Iser {
    #[inline(always)]
    fn default() -> Iser {
        Iser(0)
    }
}
impl core::fmt::Debug for Iser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iser")
            .field("setena0", &self.setena0())
            .field("setena1", &self.setena1())
            .field("setena2", &self.setena2())
            .field("setena3", &self.setena3())
            .field("setena4", &self.setena4())
            .field("setena5", &self.setena5())
            .field("setena6", &self.setena6())
            .field("setena7", &self.setena7())
            .field("setena8", &self.setena8())
            .field("setena9", &self.setena9())
            .field("setena10", &self.setena10())
            .field("setena11", &self.setena11())
            .field("setena12", &self.setena12())
            .field("setena13", &self.setena13())
            .field("setena14", &self.setena14())
            .field("setena15", &self.setena15())
            .field("setena16", &self.setena16())
            .field("setena17", &self.setena17())
            .field("setena18", &self.setena18())
            .field("setena19", &self.setena19())
            .field("setena20", &self.setena20())
            .field("setena21", &self.setena21())
            .field("setena22", &self.setena22())
            .field("setena23", &self.setena23())
            .field("setena24", &self.setena24())
            .field("setena25", &self.setena25())
            .field("setena26", &self.setena26())
            .field("setena27", &self.setena27())
            .field("setena28", &self.setena28())
            .field("setena29", &self.setena29())
            .field("setena30", &self.setena30())
            .field("setena31", &self.setena31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iser {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iser {{ setena0: {:?}, setena1: {:?}, setena2: {:?}, setena3: {:?}, setena4: {:?}, setena5: {:?}, setena6: {:?}, setena7: {:?}, setena8: {:?}, setena9: {:?}, setena10: {:?}, setena11: {:?}, setena12: {:?}, setena13: {:?}, setena14: {:?}, setena15: {:?}, setena16: {:?}, setena17: {:?}, setena18: {:?}, setena19: {:?}, setena20: {:?}, setena21: {:?}, setena22: {:?}, setena23: {:?}, setena24: {:?}, setena25: {:?}, setena26: {:?}, setena27: {:?}, setena28: {:?}, setena29: {:?}, setena30: {:?}, setena31: {:?} }}",
            self.setena0(),
            self.setena1(),
            self.setena2(),
            self.setena3(),
            self.setena4(),
            self.setena5(),
            self.setena6(),
            self.setena7(),
            self.setena8(),
            self.setena9(),
            self.setena10(),
            self.setena11(),
            self.setena12(),
            self.setena13(),
            self.setena14(),
            self.setena15(),
            self.setena16(),
            self.setena17(),
            self.setena18(),
            self.setena19(),
            self.setena20(),
            self.setena21(),
            self.setena22(),
            self.setena23(),
            self.setena24(),
            self.setena25(),
            self.setena26(),
            self.setena27(),
            self.setena28(),
            self.setena29(),
            self.setena30(),
            self.setena31()
        )
    }
}
#[doc = "Interrupt Set Pending Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ispr(pub u32);
impl Ispr {
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend0(&self) -> super::vals::Setpend0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Setpend0::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend0(&mut self, val: super::vals::Setpend0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend1(&self) -> super::vals::Setpend1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Setpend1::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend1(&mut self, val: super::vals::Setpend1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend2(&self) -> super::vals::Setpend2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Setpend2::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend2(&mut self, val: super::vals::Setpend2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend3(&self) -> super::vals::Setpend3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Setpend3::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend3(&mut self, val: super::vals::Setpend3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend4(&self) -> super::vals::Setpend4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Setpend4::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend4(&mut self, val: super::vals::Setpend4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend5(&self) -> super::vals::Setpend5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Setpend5::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend5(&mut self, val: super::vals::Setpend5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend6(&self) -> super::vals::Setpend6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Setpend6::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend6(&mut self, val: super::vals::Setpend6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend7(&self) -> super::vals::Setpend7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Setpend7::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend7(&mut self, val: super::vals::Setpend7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend8(&self) -> super::vals::Setpend8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Setpend8::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend8(&mut self, val: super::vals::Setpend8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend9(&self) -> super::vals::Setpend9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Setpend9::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend9(&mut self, val: super::vals::Setpend9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend10(&self) -> super::vals::Setpend10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Setpend10::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend10(&mut self, val: super::vals::Setpend10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend11(&self) -> super::vals::Setpend11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Setpend11::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend11(&mut self, val: super::vals::Setpend11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend12(&self) -> super::vals::Setpend12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Setpend12::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend12(&mut self, val: super::vals::Setpend12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend13(&self) -> super::vals::Setpend13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Setpend13::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend13(&mut self, val: super::vals::Setpend13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend14(&self) -> super::vals::Setpend14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Setpend14::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend14(&mut self, val: super::vals::Setpend14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend15(&self) -> super::vals::Setpend15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Setpend15::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend15(&mut self, val: super::vals::Setpend15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend16(&self) -> super::vals::Setpend16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Setpend16::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend16(&mut self, val: super::vals::Setpend16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend17(&self) -> super::vals::Setpend17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Setpend17::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend17(&mut self, val: super::vals::Setpend17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend18(&self) -> super::vals::Setpend18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Setpend18::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend18(&mut self, val: super::vals::Setpend18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend19(&self) -> super::vals::Setpend19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Setpend19::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend19(&mut self, val: super::vals::Setpend19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend20(&self) -> super::vals::Setpend20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Setpend20::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend20(&mut self, val: super::vals::Setpend20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend21(&self) -> super::vals::Setpend21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Setpend21::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend21(&mut self, val: super::vals::Setpend21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend22(&self) -> super::vals::Setpend22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Setpend22::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend22(&mut self, val: super::vals::Setpend22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend23(&self) -> super::vals::Setpend23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Setpend23::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend23(&mut self, val: super::vals::Setpend23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend24(&self) -> super::vals::Setpend24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Setpend24::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend24(&mut self, val: super::vals::Setpend24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend25(&self) -> super::vals::Setpend25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Setpend25::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend25(&mut self, val: super::vals::Setpend25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend26(&self) -> super::vals::Setpend26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Setpend26::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend26(&mut self, val: super::vals::Setpend26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend27(&self) -> super::vals::Setpend27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Setpend27::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend27(&mut self, val: super::vals::Setpend27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend28(&self) -> super::vals::Setpend28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Setpend28::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend28(&mut self, val: super::vals::Setpend28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend29(&self) -> super::vals::Setpend29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Setpend29::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend29(&mut self, val: super::vals::Setpend29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend30(&self) -> super::vals::Setpend30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Setpend30::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend30(&mut self, val: super::vals::Setpend30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn setpend31(&self) -> super::vals::Setpend31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Setpend31::from_bits(val as u8)
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_setpend31(&mut self, val: super::vals::Setpend31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ispr {
    #[inline(always)]
    fn default() -> Ispr {
        Ispr(0)
    }
}
impl core::fmt::Debug for Ispr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ispr")
            .field("setpend0", &self.setpend0())
            .field("setpend1", &self.setpend1())
            .field("setpend2", &self.setpend2())
            .field("setpend3", &self.setpend3())
            .field("setpend4", &self.setpend4())
            .field("setpend5", &self.setpend5())
            .field("setpend6", &self.setpend6())
            .field("setpend7", &self.setpend7())
            .field("setpend8", &self.setpend8())
            .field("setpend9", &self.setpend9())
            .field("setpend10", &self.setpend10())
            .field("setpend11", &self.setpend11())
            .field("setpend12", &self.setpend12())
            .field("setpend13", &self.setpend13())
            .field("setpend14", &self.setpend14())
            .field("setpend15", &self.setpend15())
            .field("setpend16", &self.setpend16())
            .field("setpend17", &self.setpend17())
            .field("setpend18", &self.setpend18())
            .field("setpend19", &self.setpend19())
            .field("setpend20", &self.setpend20())
            .field("setpend21", &self.setpend21())
            .field("setpend22", &self.setpend22())
            .field("setpend23", &self.setpend23())
            .field("setpend24", &self.setpend24())
            .field("setpend25", &self.setpend25())
            .field("setpend26", &self.setpend26())
            .field("setpend27", &self.setpend27())
            .field("setpend28", &self.setpend28())
            .field("setpend29", &self.setpend29())
            .field("setpend30", &self.setpend30())
            .field("setpend31", &self.setpend31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ispr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ispr {{ setpend0: {:?}, setpend1: {:?}, setpend2: {:?}, setpend3: {:?}, setpend4: {:?}, setpend5: {:?}, setpend6: {:?}, setpend7: {:?}, setpend8: {:?}, setpend9: {:?}, setpend10: {:?}, setpend11: {:?}, setpend12: {:?}, setpend13: {:?}, setpend14: {:?}, setpend15: {:?}, setpend16: {:?}, setpend17: {:?}, setpend18: {:?}, setpend19: {:?}, setpend20: {:?}, setpend21: {:?}, setpend22: {:?}, setpend23: {:?}, setpend24: {:?}, setpend25: {:?}, setpend26: {:?}, setpend27: {:?}, setpend28: {:?}, setpend29: {:?}, setpend30: {:?}, setpend31: {:?} }}",
            self.setpend0(),
            self.setpend1(),
            self.setpend2(),
            self.setpend3(),
            self.setpend4(),
            self.setpend5(),
            self.setpend6(),
            self.setpend7(),
            self.setpend8(),
            self.setpend9(),
            self.setpend10(),
            self.setpend11(),
            self.setpend12(),
            self.setpend13(),
            self.setpend14(),
            self.setpend15(),
            self.setpend16(),
            self.setpend17(),
            self.setpend18(),
            self.setpend19(),
            self.setpend20(),
            self.setpend21(),
            self.setpend22(),
            self.setpend23(),
            self.setpend24(),
            self.setpend25(),
            self.setpend26(),
            self.setpend27(),
            self.setpend28(),
            self.setpend29(),
            self.setpend30(),
            self.setpend31()
        )
    }
}
#[doc = "Interrupt Target Non-secure Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itns(pub u32);
impl Itns {
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints0(&self) -> super::vals::Ints0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ints0::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints0(&mut self, val: super::vals::Ints0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints1(&self) -> super::vals::Ints1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ints1::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints1(&mut self, val: super::vals::Ints1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints2(&self) -> super::vals::Ints2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ints2::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints2(&mut self, val: super::vals::Ints2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints3(&self) -> super::vals::Ints3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ints3::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints3(&mut self, val: super::vals::Ints3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints4(&self) -> super::vals::Ints4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ints4::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints4(&mut self, val: super::vals::Ints4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints5(&self) -> super::vals::Ints5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ints5::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints5(&mut self, val: super::vals::Ints5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints6(&self) -> super::vals::Ints6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ints6::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints6(&mut self, val: super::vals::Ints6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints7(&self) -> super::vals::Ints7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ints7::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints7(&mut self, val: super::vals::Ints7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints8(&self) -> super::vals::Ints8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ints8::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints8(&mut self, val: super::vals::Ints8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints9(&self) -> super::vals::Ints9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ints9::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints9(&mut self, val: super::vals::Ints9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints10(&self) -> super::vals::Ints10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ints10::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints10(&mut self, val: super::vals::Ints10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints11(&self) -> super::vals::Ints11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ints11::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints11(&mut self, val: super::vals::Ints11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints12(&self) -> super::vals::Ints12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ints12::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints12(&mut self, val: super::vals::Ints12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints13(&self) -> super::vals::Ints13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ints13::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints13(&mut self, val: super::vals::Ints13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints14(&self) -> super::vals::Ints14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ints14::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints14(&mut self, val: super::vals::Ints14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints15(&self) -> super::vals::Ints15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ints15::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints15(&mut self, val: super::vals::Ints15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints16(&self) -> super::vals::Ints16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ints16::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints16(&mut self, val: super::vals::Ints16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints17(&self) -> super::vals::Ints17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ints17::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints17(&mut self, val: super::vals::Ints17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints18(&self) -> super::vals::Ints18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ints18::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints18(&mut self, val: super::vals::Ints18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints19(&self) -> super::vals::Ints19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ints19::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints19(&mut self, val: super::vals::Ints19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints20(&self) -> super::vals::Ints20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ints20::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints20(&mut self, val: super::vals::Ints20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints21(&self) -> super::vals::Ints21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ints21::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints21(&mut self, val: super::vals::Ints21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints22(&self) -> super::vals::Ints22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ints22::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints22(&mut self, val: super::vals::Ints22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints23(&self) -> super::vals::Ints23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ints23::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints23(&mut self, val: super::vals::Ints23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints24(&self) -> super::vals::Ints24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ints24::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints24(&mut self, val: super::vals::Ints24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints25(&self) -> super::vals::Ints25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Ints25::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints25(&mut self, val: super::vals::Ints25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints26(&self) -> super::vals::Ints26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ints26::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints26(&mut self, val: super::vals::Ints26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints27(&self) -> super::vals::Ints27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ints27::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints27(&mut self, val: super::vals::Ints27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints28(&self) -> super::vals::Ints28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ints28::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints28(&mut self, val: super::vals::Ints28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints29(&self) -> super::vals::Ints29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Ints29::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints29(&mut self, val: super::vals::Ints29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints30(&self) -> super::vals::Ints30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ints30::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints30(&mut self, val: super::vals::Ints30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ints31(&self) -> super::vals::Ints31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ints31::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_ints31(&mut self, val: super::vals::Ints31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Itns {
    #[inline(always)]
    fn default() -> Itns {
        Itns(0)
    }
}
impl core::fmt::Debug for Itns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itns")
            .field("ints0", &self.ints0())
            .field("ints1", &self.ints1())
            .field("ints2", &self.ints2())
            .field("ints3", &self.ints3())
            .field("ints4", &self.ints4())
            .field("ints5", &self.ints5())
            .field("ints6", &self.ints6())
            .field("ints7", &self.ints7())
            .field("ints8", &self.ints8())
            .field("ints9", &self.ints9())
            .field("ints10", &self.ints10())
            .field("ints11", &self.ints11())
            .field("ints12", &self.ints12())
            .field("ints13", &self.ints13())
            .field("ints14", &self.ints14())
            .field("ints15", &self.ints15())
            .field("ints16", &self.ints16())
            .field("ints17", &self.ints17())
            .field("ints18", &self.ints18())
            .field("ints19", &self.ints19())
            .field("ints20", &self.ints20())
            .field("ints21", &self.ints21())
            .field("ints22", &self.ints22())
            .field("ints23", &self.ints23())
            .field("ints24", &self.ints24())
            .field("ints25", &self.ints25())
            .field("ints26", &self.ints26())
            .field("ints27", &self.ints27())
            .field("ints28", &self.ints28())
            .field("ints29", &self.ints29())
            .field("ints30", &self.ints30())
            .field("ints31", &self.ints31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Itns {{ ints0: {:?}, ints1: {:?}, ints2: {:?}, ints3: {:?}, ints4: {:?}, ints5: {:?}, ints6: {:?}, ints7: {:?}, ints8: {:?}, ints9: {:?}, ints10: {:?}, ints11: {:?}, ints12: {:?}, ints13: {:?}, ints14: {:?}, ints15: {:?}, ints16: {:?}, ints17: {:?}, ints18: {:?}, ints19: {:?}, ints20: {:?}, ints21: {:?}, ints22: {:?}, ints23: {:?}, ints24: {:?}, ints25: {:?}, ints26: {:?}, ints27: {:?}, ints28: {:?}, ints29: {:?}, ints30: {:?}, ints31: {:?} }}",
            self.ints0(),
            self.ints1(),
            self.ints2(),
            self.ints3(),
            self.ints4(),
            self.ints5(),
            self.ints6(),
            self.ints7(),
            self.ints8(),
            self.ints9(),
            self.ints10(),
            self.ints11(),
            self.ints12(),
            self.ints13(),
            self.ints14(),
            self.ints15(),
            self.ints16(),
            self.ints17(),
            self.ints18(),
            self.ints19(),
            self.ints20(),
            self.ints21(),
            self.ints22(),
            self.ints23(),
            self.ints24(),
            self.ints25(),
            self.ints26(),
            self.ints27(),
            self.ints28(),
            self.ints29(),
            self.ints30(),
            self.ints31()
        )
    }
}
#[doc = "Software Trigger Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stir(pub u32);
impl Stir {
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-479."]
    #[must_use]
    #[inline(always)]
    pub const fn intid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-479."]
    #[inline(always)]
    pub const fn set_intid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Stir {
    #[inline(always)]
    fn default() -> Stir {
        Stir(0)
    }
}
impl core::fmt::Debug for Stir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stir")
            .field("intid", &self.intid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stir {{ intid: {=u16:?} }}", self.intid())
    }
}
