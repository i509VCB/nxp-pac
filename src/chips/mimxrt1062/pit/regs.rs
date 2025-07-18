#[doc = "Current Timer Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cval(pub u32);
impl Cval {
    #[doc = "Current Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tvl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current Timer Value"]
    #[inline(always)]
    pub const fn set_tvl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cval {
    #[inline(always)]
    fn default() -> Cval {
        Cval(0)
    }
}
impl core::fmt::Debug for Cval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cval").field("tvl", &self.tvl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cval {{ tvl: {=u32:?} }}", self.tvl())
    }
}
#[doc = "PIT Upper Lifetime Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltmr64h(pub u32);
impl Ltmr64h {
    #[doc = "Life Timer value"]
    #[must_use]
    #[inline(always)]
    pub const fn lth(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Life Timer value"]
    #[inline(always)]
    pub const fn set_lth(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ltmr64h {
    #[inline(always)]
    fn default() -> Ltmr64h {
        Ltmr64h(0)
    }
}
impl core::fmt::Debug for Ltmr64h {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ltmr64h").field("lth", &self.lth()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ltmr64h {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ltmr64h {{ lth: {=u32:?} }}", self.lth())
    }
}
#[doc = "PIT Lower Lifetime Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltmr64l(pub u32);
impl Ltmr64l {
    #[doc = "Life Timer value"]
    #[must_use]
    #[inline(always)]
    pub const fn ltl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Life Timer value"]
    #[inline(always)]
    pub const fn set_ltl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ltmr64l {
    #[inline(always)]
    fn default() -> Ltmr64l {
        Ltmr64l(0)
    }
}
impl core::fmt::Debug for Ltmr64l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ltmr64l").field("ltl", &self.ltl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ltmr64l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ltmr64l {{ ltl: {=u32:?} }}", self.ltl())
    }
}
#[doc = "PIT Module Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn frz(&self) -> super::vals::Frz {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Frz::from_bits(val as u8)
    }
    #[doc = "Freeze"]
    #[inline(always)]
    pub const fn set_frz(&mut self, val: super::vals::Frz) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable for PIT"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Module Disable for PIT"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("frz", &self.frz())
            .field("mdis", &self.mdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ frz: {:?}, mdis: {=bool:?} }}",
            self.frz(),
            self.mdis()
        )
    }
}
#[doc = "Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Chain Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn chn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Chain Mode"]
    #[inline(always)]
    pub const fn set_chn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Tctrl {
    #[inline(always)]
    fn default() -> Tctrl {
        Tctrl(0)
    }
}
impl core::fmt::Debug for Tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tctrl")
            .field("ten", &self.ten())
            .field("tie", &self.tie())
            .field("chn", &self.chn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tctrl {{ ten: {=bool:?}, tie: {=bool:?}, chn: {=bool:?} }}",
            self.ten(),
            self.tie(),
            self.chn()
        )
    }
}
#[doc = "Timer Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tflg(pub u32);
impl Tflg {
    #[doc = "Timer Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tflg {
    #[inline(always)]
    fn default() -> Tflg {
        Tflg(0)
    }
}
impl core::fmt::Debug for Tflg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tflg").field("tif", &self.tif()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tflg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tflg {{ tif: {=bool:?} }}", self.tif())
    }
}
