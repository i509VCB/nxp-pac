#[doc = "Interrupt Active Bit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IABR(pub u32);
impl IABR {
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Active state bits."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Active state bits."]
    #[inline(always)]
    pub const fn set_ACTIVE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IABR {
    #[inline(always)]
    fn default() -> IABR {
        IABR(0)
    }
}
impl core::fmt::Debug for IABR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IABR")
            .field("ACTIVE0", &self.ACTIVE0())
            .field("ACTIVE1", &self.ACTIVE1())
            .field("ACTIVE2", &self.ACTIVE2())
            .field("ACTIVE3", &self.ACTIVE3())
            .field("ACTIVE4", &self.ACTIVE4())
            .field("ACTIVE5", &self.ACTIVE5())
            .field("ACTIVE6", &self.ACTIVE6())
            .field("ACTIVE7", &self.ACTIVE7())
            .field("ACTIVE8", &self.ACTIVE8())
            .field("ACTIVE9", &self.ACTIVE9())
            .field("ACTIVE10", &self.ACTIVE10())
            .field("ACTIVE11", &self.ACTIVE11())
            .field("ACTIVE12", &self.ACTIVE12())
            .field("ACTIVE13", &self.ACTIVE13())
            .field("ACTIVE14", &self.ACTIVE14())
            .field("ACTIVE15", &self.ACTIVE15())
            .field("ACTIVE16", &self.ACTIVE16())
            .field("ACTIVE17", &self.ACTIVE17())
            .field("ACTIVE18", &self.ACTIVE18())
            .field("ACTIVE19", &self.ACTIVE19())
            .field("ACTIVE20", &self.ACTIVE20())
            .field("ACTIVE21", &self.ACTIVE21())
            .field("ACTIVE22", &self.ACTIVE22())
            .field("ACTIVE23", &self.ACTIVE23())
            .field("ACTIVE24", &self.ACTIVE24())
            .field("ACTIVE25", &self.ACTIVE25())
            .field("ACTIVE26", &self.ACTIVE26())
            .field("ACTIVE27", &self.ACTIVE27())
            .field("ACTIVE28", &self.ACTIVE28())
            .field("ACTIVE29", &self.ACTIVE29())
            .field("ACTIVE30", &self.ACTIVE30())
            .field("ACTIVE31", &self.ACTIVE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IABR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IABR {{ ACTIVE0: {=bool:?}, ACTIVE1: {=bool:?}, ACTIVE2: {=bool:?}, ACTIVE3: {=bool:?}, ACTIVE4: {=bool:?}, ACTIVE5: {=bool:?}, ACTIVE6: {=bool:?}, ACTIVE7: {=bool:?}, ACTIVE8: {=bool:?}, ACTIVE9: {=bool:?}, ACTIVE10: {=bool:?}, ACTIVE11: {=bool:?}, ACTIVE12: {=bool:?}, ACTIVE13: {=bool:?}, ACTIVE14: {=bool:?}, ACTIVE15: {=bool:?}, ACTIVE16: {=bool:?}, ACTIVE17: {=bool:?}, ACTIVE18: {=bool:?}, ACTIVE19: {=bool:?}, ACTIVE20: {=bool:?}, ACTIVE21: {=bool:?}, ACTIVE22: {=bool:?}, ACTIVE23: {=bool:?}, ACTIVE24: {=bool:?}, ACTIVE25: {=bool:?}, ACTIVE26: {=bool:?}, ACTIVE27: {=bool:?}, ACTIVE28: {=bool:?}, ACTIVE29: {=bool:?}, ACTIVE30: {=bool:?}, ACTIVE31: {=bool:?} }}",
            self.ACTIVE0(),
            self.ACTIVE1(),
            self.ACTIVE2(),
            self.ACTIVE3(),
            self.ACTIVE4(),
            self.ACTIVE5(),
            self.ACTIVE6(),
            self.ACTIVE7(),
            self.ACTIVE8(),
            self.ACTIVE9(),
            self.ACTIVE10(),
            self.ACTIVE11(),
            self.ACTIVE12(),
            self.ACTIVE13(),
            self.ACTIVE14(),
            self.ACTIVE15(),
            self.ACTIVE16(),
            self.ACTIVE17(),
            self.ACTIVE18(),
            self.ACTIVE19(),
            self.ACTIVE20(),
            self.ACTIVE21(),
            self.ACTIVE22(),
            self.ACTIVE23(),
            self.ACTIVE24(),
            self.ACTIVE25(),
            self.ACTIVE26(),
            self.ACTIVE27(),
            self.ACTIVE28(),
            self.ACTIVE29(),
            self.ACTIVE30(),
            self.ACTIVE31()
        )
    }
}
#[doc = "Interrupt Clear Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICER(pub u32);
impl ICER {
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt clear-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-enable bits."]
    #[inline(always)]
    pub const fn set_CLRENA31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ICER {
    #[inline(always)]
    fn default() -> ICER {
        ICER(0)
    }
}
impl core::fmt::Debug for ICER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICER")
            .field("CLRENA0", &self.CLRENA0())
            .field("CLRENA1", &self.CLRENA1())
            .field("CLRENA2", &self.CLRENA2())
            .field("CLRENA3", &self.CLRENA3())
            .field("CLRENA4", &self.CLRENA4())
            .field("CLRENA5", &self.CLRENA5())
            .field("CLRENA6", &self.CLRENA6())
            .field("CLRENA7", &self.CLRENA7())
            .field("CLRENA8", &self.CLRENA8())
            .field("CLRENA9", &self.CLRENA9())
            .field("CLRENA10", &self.CLRENA10())
            .field("CLRENA11", &self.CLRENA11())
            .field("CLRENA12", &self.CLRENA12())
            .field("CLRENA13", &self.CLRENA13())
            .field("CLRENA14", &self.CLRENA14())
            .field("CLRENA15", &self.CLRENA15())
            .field("CLRENA16", &self.CLRENA16())
            .field("CLRENA17", &self.CLRENA17())
            .field("CLRENA18", &self.CLRENA18())
            .field("CLRENA19", &self.CLRENA19())
            .field("CLRENA20", &self.CLRENA20())
            .field("CLRENA21", &self.CLRENA21())
            .field("CLRENA22", &self.CLRENA22())
            .field("CLRENA23", &self.CLRENA23())
            .field("CLRENA24", &self.CLRENA24())
            .field("CLRENA25", &self.CLRENA25())
            .field("CLRENA26", &self.CLRENA26())
            .field("CLRENA27", &self.CLRENA27())
            .field("CLRENA28", &self.CLRENA28())
            .field("CLRENA29", &self.CLRENA29())
            .field("CLRENA30", &self.CLRENA30())
            .field("CLRENA31", &self.CLRENA31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICER {{ CLRENA0: {=bool:?}, CLRENA1: {=bool:?}, CLRENA2: {=bool:?}, CLRENA3: {=bool:?}, CLRENA4: {=bool:?}, CLRENA5: {=bool:?}, CLRENA6: {=bool:?}, CLRENA7: {=bool:?}, CLRENA8: {=bool:?}, CLRENA9: {=bool:?}, CLRENA10: {=bool:?}, CLRENA11: {=bool:?}, CLRENA12: {=bool:?}, CLRENA13: {=bool:?}, CLRENA14: {=bool:?}, CLRENA15: {=bool:?}, CLRENA16: {=bool:?}, CLRENA17: {=bool:?}, CLRENA18: {=bool:?}, CLRENA19: {=bool:?}, CLRENA20: {=bool:?}, CLRENA21: {=bool:?}, CLRENA22: {=bool:?}, CLRENA23: {=bool:?}, CLRENA24: {=bool:?}, CLRENA25: {=bool:?}, CLRENA26: {=bool:?}, CLRENA27: {=bool:?}, CLRENA28: {=bool:?}, CLRENA29: {=bool:?}, CLRENA30: {=bool:?}, CLRENA31: {=bool:?} }}",
            self.CLRENA0(),
            self.CLRENA1(),
            self.CLRENA2(),
            self.CLRENA3(),
            self.CLRENA4(),
            self.CLRENA5(),
            self.CLRENA6(),
            self.CLRENA7(),
            self.CLRENA8(),
            self.CLRENA9(),
            self.CLRENA10(),
            self.CLRENA11(),
            self.CLRENA12(),
            self.CLRENA13(),
            self.CLRENA14(),
            self.CLRENA15(),
            self.CLRENA16(),
            self.CLRENA17(),
            self.CLRENA18(),
            self.CLRENA19(),
            self.CLRENA20(),
            self.CLRENA21(),
            self.CLRENA22(),
            self.CLRENA23(),
            self.CLRENA24(),
            self.CLRENA25(),
            self.CLRENA26(),
            self.CLRENA27(),
            self.CLRENA28(),
            self.CLRENA29(),
            self.CLRENA30(),
            self.CLRENA31()
        )
    }
}
#[doc = "Interrupt Clear Pending Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICPR(pub u32);
impl ICPR {
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt clear-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt clear-pending bits."]
    #[inline(always)]
    pub const fn set_CLRPEND31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ICPR {
    #[inline(always)]
    fn default() -> ICPR {
        ICPR(0)
    }
}
impl core::fmt::Debug for ICPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPR")
            .field("CLRPEND0", &self.CLRPEND0())
            .field("CLRPEND1", &self.CLRPEND1())
            .field("CLRPEND2", &self.CLRPEND2())
            .field("CLRPEND3", &self.CLRPEND3())
            .field("CLRPEND4", &self.CLRPEND4())
            .field("CLRPEND5", &self.CLRPEND5())
            .field("CLRPEND6", &self.CLRPEND6())
            .field("CLRPEND7", &self.CLRPEND7())
            .field("CLRPEND8", &self.CLRPEND8())
            .field("CLRPEND9", &self.CLRPEND9())
            .field("CLRPEND10", &self.CLRPEND10())
            .field("CLRPEND11", &self.CLRPEND11())
            .field("CLRPEND12", &self.CLRPEND12())
            .field("CLRPEND13", &self.CLRPEND13())
            .field("CLRPEND14", &self.CLRPEND14())
            .field("CLRPEND15", &self.CLRPEND15())
            .field("CLRPEND16", &self.CLRPEND16())
            .field("CLRPEND17", &self.CLRPEND17())
            .field("CLRPEND18", &self.CLRPEND18())
            .field("CLRPEND19", &self.CLRPEND19())
            .field("CLRPEND20", &self.CLRPEND20())
            .field("CLRPEND21", &self.CLRPEND21())
            .field("CLRPEND22", &self.CLRPEND22())
            .field("CLRPEND23", &self.CLRPEND23())
            .field("CLRPEND24", &self.CLRPEND24())
            .field("CLRPEND25", &self.CLRPEND25())
            .field("CLRPEND26", &self.CLRPEND26())
            .field("CLRPEND27", &self.CLRPEND27())
            .field("CLRPEND28", &self.CLRPEND28())
            .field("CLRPEND29", &self.CLRPEND29())
            .field("CLRPEND30", &self.CLRPEND30())
            .field("CLRPEND31", &self.CLRPEND31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICPR {{ CLRPEND0: {=bool:?}, CLRPEND1: {=bool:?}, CLRPEND2: {=bool:?}, CLRPEND3: {=bool:?}, CLRPEND4: {=bool:?}, CLRPEND5: {=bool:?}, CLRPEND6: {=bool:?}, CLRPEND7: {=bool:?}, CLRPEND8: {=bool:?}, CLRPEND9: {=bool:?}, CLRPEND10: {=bool:?}, CLRPEND11: {=bool:?}, CLRPEND12: {=bool:?}, CLRPEND13: {=bool:?}, CLRPEND14: {=bool:?}, CLRPEND15: {=bool:?}, CLRPEND16: {=bool:?}, CLRPEND17: {=bool:?}, CLRPEND18: {=bool:?}, CLRPEND19: {=bool:?}, CLRPEND20: {=bool:?}, CLRPEND21: {=bool:?}, CLRPEND22: {=bool:?}, CLRPEND23: {=bool:?}, CLRPEND24: {=bool:?}, CLRPEND25: {=bool:?}, CLRPEND26: {=bool:?}, CLRPEND27: {=bool:?}, CLRPEND28: {=bool:?}, CLRPEND29: {=bool:?}, CLRPEND30: {=bool:?}, CLRPEND31: {=bool:?} }}",
            self.CLRPEND0(),
            self.CLRPEND1(),
            self.CLRPEND2(),
            self.CLRPEND3(),
            self.CLRPEND4(),
            self.CLRPEND5(),
            self.CLRPEND6(),
            self.CLRPEND7(),
            self.CLRPEND8(),
            self.CLRPEND9(),
            self.CLRPEND10(),
            self.CLRPEND11(),
            self.CLRPEND12(),
            self.CLRPEND13(),
            self.CLRPEND14(),
            self.CLRPEND15(),
            self.CLRPEND16(),
            self.CLRPEND17(),
            self.CLRPEND18(),
            self.CLRPEND19(),
            self.CLRPEND20(),
            self.CLRPEND21(),
            self.CLRPEND22(),
            self.CLRPEND23(),
            self.CLRPEND24(),
            self.CLRPEND25(),
            self.CLRPEND26(),
            self.CLRPEND27(),
            self.CLRPEND28(),
            self.CLRPEND29(),
            self.CLRPEND30(),
            self.CLRPEND31()
        )
    }
}
#[doc = "Interrupt Priority Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IPR(pub u32);
impl IPR {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PRI_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PRI_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PRI_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PRI_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for IPR {
    #[inline(always)]
    fn default() -> IPR {
        IPR(0)
    }
}
impl core::fmt::Debug for IPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPR")
            .field("PRI_0", &self.PRI_0())
            .field("PRI_1", &self.PRI_1())
            .field("PRI_2", &self.PRI_2())
            .field("PRI_3", &self.PRI_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IPR {{ PRI_0: {=u8:?}, PRI_1: {=u8:?}, PRI_2: {=u8:?}, PRI_3: {=u8:?} }}",
            self.PRI_0(),
            self.PRI_1(),
            self.PRI_2(),
            self.PRI_3()
        )
    }
}
#[doc = "Interrupt Set Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISER(pub u32);
impl ISER {
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt set-enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-enable bits."]
    #[inline(always)]
    pub const fn set_SETENA31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ISER {
    #[inline(always)]
    fn default() -> ISER {
        ISER(0)
    }
}
impl core::fmt::Debug for ISER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISER")
            .field("SETENA0", &self.SETENA0())
            .field("SETENA1", &self.SETENA1())
            .field("SETENA2", &self.SETENA2())
            .field("SETENA3", &self.SETENA3())
            .field("SETENA4", &self.SETENA4())
            .field("SETENA5", &self.SETENA5())
            .field("SETENA6", &self.SETENA6())
            .field("SETENA7", &self.SETENA7())
            .field("SETENA8", &self.SETENA8())
            .field("SETENA9", &self.SETENA9())
            .field("SETENA10", &self.SETENA10())
            .field("SETENA11", &self.SETENA11())
            .field("SETENA12", &self.SETENA12())
            .field("SETENA13", &self.SETENA13())
            .field("SETENA14", &self.SETENA14())
            .field("SETENA15", &self.SETENA15())
            .field("SETENA16", &self.SETENA16())
            .field("SETENA17", &self.SETENA17())
            .field("SETENA18", &self.SETENA18())
            .field("SETENA19", &self.SETENA19())
            .field("SETENA20", &self.SETENA20())
            .field("SETENA21", &self.SETENA21())
            .field("SETENA22", &self.SETENA22())
            .field("SETENA23", &self.SETENA23())
            .field("SETENA24", &self.SETENA24())
            .field("SETENA25", &self.SETENA25())
            .field("SETENA26", &self.SETENA26())
            .field("SETENA27", &self.SETENA27())
            .field("SETENA28", &self.SETENA28())
            .field("SETENA29", &self.SETENA29())
            .field("SETENA30", &self.SETENA30())
            .field("SETENA31", &self.SETENA31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ISER {{ SETENA0: {=bool:?}, SETENA1: {=bool:?}, SETENA2: {=bool:?}, SETENA3: {=bool:?}, SETENA4: {=bool:?}, SETENA5: {=bool:?}, SETENA6: {=bool:?}, SETENA7: {=bool:?}, SETENA8: {=bool:?}, SETENA9: {=bool:?}, SETENA10: {=bool:?}, SETENA11: {=bool:?}, SETENA12: {=bool:?}, SETENA13: {=bool:?}, SETENA14: {=bool:?}, SETENA15: {=bool:?}, SETENA16: {=bool:?}, SETENA17: {=bool:?}, SETENA18: {=bool:?}, SETENA19: {=bool:?}, SETENA20: {=bool:?}, SETENA21: {=bool:?}, SETENA22: {=bool:?}, SETENA23: {=bool:?}, SETENA24: {=bool:?}, SETENA25: {=bool:?}, SETENA26: {=bool:?}, SETENA27: {=bool:?}, SETENA28: {=bool:?}, SETENA29: {=bool:?}, SETENA30: {=bool:?}, SETENA31: {=bool:?} }}",
            self.SETENA0(),
            self.SETENA1(),
            self.SETENA2(),
            self.SETENA3(),
            self.SETENA4(),
            self.SETENA5(),
            self.SETENA6(),
            self.SETENA7(),
            self.SETENA8(),
            self.SETENA9(),
            self.SETENA10(),
            self.SETENA11(),
            self.SETENA12(),
            self.SETENA13(),
            self.SETENA14(),
            self.SETENA15(),
            self.SETENA16(),
            self.SETENA17(),
            self.SETENA18(),
            self.SETENA19(),
            self.SETENA20(),
            self.SETENA21(),
            self.SETENA22(),
            self.SETENA23(),
            self.SETENA24(),
            self.SETENA25(),
            self.SETENA26(),
            self.SETENA27(),
            self.SETENA28(),
            self.SETENA29(),
            self.SETENA30(),
            self.SETENA31()
        )
    }
}
#[doc = "Interrupt Set Pending Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISPR(pub u32);
impl ISPR {
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt set-pending bits."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt set-pending bits."]
    #[inline(always)]
    pub const fn set_SETPEND31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ISPR {
    #[inline(always)]
    fn default() -> ISPR {
        ISPR(0)
    }
}
impl core::fmt::Debug for ISPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPR")
            .field("SETPEND0", &self.SETPEND0())
            .field("SETPEND1", &self.SETPEND1())
            .field("SETPEND2", &self.SETPEND2())
            .field("SETPEND3", &self.SETPEND3())
            .field("SETPEND4", &self.SETPEND4())
            .field("SETPEND5", &self.SETPEND5())
            .field("SETPEND6", &self.SETPEND6())
            .field("SETPEND7", &self.SETPEND7())
            .field("SETPEND8", &self.SETPEND8())
            .field("SETPEND9", &self.SETPEND9())
            .field("SETPEND10", &self.SETPEND10())
            .field("SETPEND11", &self.SETPEND11())
            .field("SETPEND12", &self.SETPEND12())
            .field("SETPEND13", &self.SETPEND13())
            .field("SETPEND14", &self.SETPEND14())
            .field("SETPEND15", &self.SETPEND15())
            .field("SETPEND16", &self.SETPEND16())
            .field("SETPEND17", &self.SETPEND17())
            .field("SETPEND18", &self.SETPEND18())
            .field("SETPEND19", &self.SETPEND19())
            .field("SETPEND20", &self.SETPEND20())
            .field("SETPEND21", &self.SETPEND21())
            .field("SETPEND22", &self.SETPEND22())
            .field("SETPEND23", &self.SETPEND23())
            .field("SETPEND24", &self.SETPEND24())
            .field("SETPEND25", &self.SETPEND25())
            .field("SETPEND26", &self.SETPEND26())
            .field("SETPEND27", &self.SETPEND27())
            .field("SETPEND28", &self.SETPEND28())
            .field("SETPEND29", &self.SETPEND29())
            .field("SETPEND30", &self.SETPEND30())
            .field("SETPEND31", &self.SETPEND31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ISPR {{ SETPEND0: {=bool:?}, SETPEND1: {=bool:?}, SETPEND2: {=bool:?}, SETPEND3: {=bool:?}, SETPEND4: {=bool:?}, SETPEND5: {=bool:?}, SETPEND6: {=bool:?}, SETPEND7: {=bool:?}, SETPEND8: {=bool:?}, SETPEND9: {=bool:?}, SETPEND10: {=bool:?}, SETPEND11: {=bool:?}, SETPEND12: {=bool:?}, SETPEND13: {=bool:?}, SETPEND14: {=bool:?}, SETPEND15: {=bool:?}, SETPEND16: {=bool:?}, SETPEND17: {=bool:?}, SETPEND18: {=bool:?}, SETPEND19: {=bool:?}, SETPEND20: {=bool:?}, SETPEND21: {=bool:?}, SETPEND22: {=bool:?}, SETPEND23: {=bool:?}, SETPEND24: {=bool:?}, SETPEND25: {=bool:?}, SETPEND26: {=bool:?}, SETPEND27: {=bool:?}, SETPEND28: {=bool:?}, SETPEND29: {=bool:?}, SETPEND30: {=bool:?}, SETPEND31: {=bool:?} }}",
            self.SETPEND0(),
            self.SETPEND1(),
            self.SETPEND2(),
            self.SETPEND3(),
            self.SETPEND4(),
            self.SETPEND5(),
            self.SETPEND6(),
            self.SETPEND7(),
            self.SETPEND8(),
            self.SETPEND9(),
            self.SETPEND10(),
            self.SETPEND11(),
            self.SETPEND12(),
            self.SETPEND13(),
            self.SETPEND14(),
            self.SETPEND15(),
            self.SETPEND16(),
            self.SETPEND17(),
            self.SETPEND18(),
            self.SETPEND19(),
            self.SETPEND20(),
            self.SETPEND21(),
            self.SETPEND22(),
            self.SETPEND23(),
            self.SETPEND24(),
            self.SETPEND25(),
            self.SETPEND26(),
            self.SETPEND27(),
            self.SETPEND28(),
            self.SETPEND29(),
            self.SETPEND30(),
            self.SETPEND31()
        )
    }
}
#[doc = "Interrupt Target Non-secure Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ITNS(pub u32);
impl ITNS {
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS0(&self) -> super::vals::INTS0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INTS0::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS0(&mut self, val: super::vals::INTS0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS1(&self) -> super::vals::INTS1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::INTS1::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS1(&mut self, val: super::vals::INTS1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS2(&self) -> super::vals::INTS2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::INTS2::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS2(&mut self, val: super::vals::INTS2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS3(&self) -> super::vals::INTS3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::INTS3::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS3(&mut self, val: super::vals::INTS3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS4(&self) -> super::vals::INTS4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::INTS4::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS4(&mut self, val: super::vals::INTS4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS5(&self) -> super::vals::INTS5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::INTS5::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS5(&mut self, val: super::vals::INTS5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS6(&self) -> super::vals::INTS6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::INTS6::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS6(&mut self, val: super::vals::INTS6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS7(&self) -> super::vals::INTS7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::INTS7::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS7(&mut self, val: super::vals::INTS7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS8(&self) -> super::vals::INTS8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::INTS8::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS8(&mut self, val: super::vals::INTS8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS9(&self) -> super::vals::INTS9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::INTS9::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS9(&mut self, val: super::vals::INTS9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS10(&self) -> super::vals::INTS10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::INTS10::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS10(&mut self, val: super::vals::INTS10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS11(&self) -> super::vals::INTS11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::INTS11::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS11(&mut self, val: super::vals::INTS11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS12(&self) -> super::vals::INTS12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::INTS12::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS12(&mut self, val: super::vals::INTS12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS13(&self) -> super::vals::INTS13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::INTS13::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS13(&mut self, val: super::vals::INTS13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS14(&self) -> super::vals::INTS14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::INTS14::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS14(&mut self, val: super::vals::INTS14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS15(&self) -> super::vals::INTS15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::INTS15::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS15(&mut self, val: super::vals::INTS15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS16(&self) -> super::vals::INTS16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::INTS16::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS16(&mut self, val: super::vals::INTS16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS17(&self) -> super::vals::INTS17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::INTS17::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS17(&mut self, val: super::vals::INTS17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS18(&self) -> super::vals::INTS18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::INTS18::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS18(&mut self, val: super::vals::INTS18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS19(&self) -> super::vals::INTS19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::INTS19::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS19(&mut self, val: super::vals::INTS19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS20(&self) -> super::vals::INTS20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::INTS20::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS20(&mut self, val: super::vals::INTS20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS21(&self) -> super::vals::INTS21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::INTS21::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS21(&mut self, val: super::vals::INTS21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS22(&self) -> super::vals::INTS22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::INTS22::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS22(&mut self, val: super::vals::INTS22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS23(&self) -> super::vals::INTS23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::INTS23::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS23(&mut self, val: super::vals::INTS23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS24(&self) -> super::vals::INTS24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::INTS24::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS24(&mut self, val: super::vals::INTS24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS25(&self) -> super::vals::INTS25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::INTS25::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS25(&mut self, val: super::vals::INTS25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS26(&self) -> super::vals::INTS26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::INTS26::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS26(&mut self, val: super::vals::INTS26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS27(&self) -> super::vals::INTS27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::INTS27::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS27(&mut self, val: super::vals::INTS27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS28(&self) -> super::vals::INTS28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::INTS28::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS28(&mut self, val: super::vals::INTS28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS29(&self) -> super::vals::INTS29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::INTS29::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS29(&mut self, val: super::vals::INTS29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS30(&self) -> super::vals::INTS30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::INTS30::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS30(&mut self, val: super::vals::INTS30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[must_use]
    #[inline(always)]
    pub const fn INTS31(&self) -> super::vals::INTS31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::INTS31::from_bits(val as u8)
    }
    #[doc = "Interrupt Targets Non-secure bits."]
    #[inline(always)]
    pub const fn set_INTS31(&mut self, val: super::vals::INTS31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ITNS {
    #[inline(always)]
    fn default() -> ITNS {
        ITNS(0)
    }
}
impl core::fmt::Debug for ITNS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITNS")
            .field("INTS0", &self.INTS0())
            .field("INTS1", &self.INTS1())
            .field("INTS2", &self.INTS2())
            .field("INTS3", &self.INTS3())
            .field("INTS4", &self.INTS4())
            .field("INTS5", &self.INTS5())
            .field("INTS6", &self.INTS6())
            .field("INTS7", &self.INTS7())
            .field("INTS8", &self.INTS8())
            .field("INTS9", &self.INTS9())
            .field("INTS10", &self.INTS10())
            .field("INTS11", &self.INTS11())
            .field("INTS12", &self.INTS12())
            .field("INTS13", &self.INTS13())
            .field("INTS14", &self.INTS14())
            .field("INTS15", &self.INTS15())
            .field("INTS16", &self.INTS16())
            .field("INTS17", &self.INTS17())
            .field("INTS18", &self.INTS18())
            .field("INTS19", &self.INTS19())
            .field("INTS20", &self.INTS20())
            .field("INTS21", &self.INTS21())
            .field("INTS22", &self.INTS22())
            .field("INTS23", &self.INTS23())
            .field("INTS24", &self.INTS24())
            .field("INTS25", &self.INTS25())
            .field("INTS26", &self.INTS26())
            .field("INTS27", &self.INTS27())
            .field("INTS28", &self.INTS28())
            .field("INTS29", &self.INTS29())
            .field("INTS30", &self.INTS30())
            .field("INTS31", &self.INTS31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ITNS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ITNS {{ INTS0: {:?}, INTS1: {:?}, INTS2: {:?}, INTS3: {:?}, INTS4: {:?}, INTS5: {:?}, INTS6: {:?}, INTS7: {:?}, INTS8: {:?}, INTS9: {:?}, INTS10: {:?}, INTS11: {:?}, INTS12: {:?}, INTS13: {:?}, INTS14: {:?}, INTS15: {:?}, INTS16: {:?}, INTS17: {:?}, INTS18: {:?}, INTS19: {:?}, INTS20: {:?}, INTS21: {:?}, INTS22: {:?}, INTS23: {:?}, INTS24: {:?}, INTS25: {:?}, INTS26: {:?}, INTS27: {:?}, INTS28: {:?}, INTS29: {:?}, INTS30: {:?}, INTS31: {:?} }}",
            self.INTS0(),
            self.INTS1(),
            self.INTS2(),
            self.INTS3(),
            self.INTS4(),
            self.INTS5(),
            self.INTS6(),
            self.INTS7(),
            self.INTS8(),
            self.INTS9(),
            self.INTS10(),
            self.INTS11(),
            self.INTS12(),
            self.INTS13(),
            self.INTS14(),
            self.INTS15(),
            self.INTS16(),
            self.INTS17(),
            self.INTS18(),
            self.INTS19(),
            self.INTS20(),
            self.INTS21(),
            self.INTS22(),
            self.INTS23(),
            self.INTS24(),
            self.INTS25(),
            self.INTS26(),
            self.INTS27(),
            self.INTS28(),
            self.INTS29(),
            self.INTS30(),
            self.INTS31()
        )
    }
}
#[doc = "Software Trigger Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIR(pub u32);
impl STIR {
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-479."]
    #[must_use]
    #[inline(always)]
    pub const fn INTID(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-479."]
    #[inline(always)]
    pub const fn set_INTID(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for STIR {
    #[inline(always)]
    fn default() -> STIR {
        STIR(0)
    }
}
impl core::fmt::Debug for STIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIR")
            .field("INTID", &self.INTID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIR {{ INTID: {=u16:?} }}", self.INTID())
    }
}
