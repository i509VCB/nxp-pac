#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt01(pub u16);
impl Bfcrt01 {
    #[doc = "Product term 1, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt01 {
    #[inline(always)]
    fn default() -> Bfcrt01 {
        Bfcrt01(0)
    }
}
impl core::fmt::Debug for Bfcrt01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt01")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt01 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt23(pub u16);
impl Bfcrt23 {
    #[doc = "Product term 3, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt23 {
    #[inline(always)]
    fn default() -> Bfcrt23 {
        Bfcrt23(0)
    }
}
impl core::fmt::Debug for Bfcrt23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt23")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt23 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
