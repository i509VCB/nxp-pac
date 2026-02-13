#[doc = "AOI0 Boolean Function Term 0 and 1 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Bft01(pub u16);
impl EvtgAoi0Bft01 {
    #[doc = "Product Term 1, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::EvtgAoi0Bft01Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::EvtgAoi0Bft01Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::EvtgAoi0Bft01Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::EvtgAoi0Bft01Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::EvtgAoi0Bft01Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::EvtgAoi0Bft01Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::EvtgAoi0Bft01Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::EvtgAoi0Bft01Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::EvtgAoi0Bft01Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::EvtgAoi0Bft01Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::EvtgAoi0Bft01Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::EvtgAoi0Bft01Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::EvtgAoi0Bft01Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::EvtgAoi0Bft01Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::EvtgAoi0Bft01Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::EvtgAoi0Bft01Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::EvtgAoi0Bft01Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi0Bft01 {
    #[inline(always)]
    fn default() -> EvtgAoi0Bft01 {
        EvtgAoi0Bft01(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Bft01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Bft01")
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
impl defmt::Format for EvtgAoi0Bft01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Bft01 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
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
#[doc = "AOI0 Boolean Function Term 2 and 3 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Bft23(pub u16);
impl EvtgAoi0Bft23 {
    #[doc = "Product Term 3, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::EvtgAoi0Bft23Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::EvtgAoi0Bft23Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::EvtgAoi0Bft23Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::EvtgAoi0Bft23Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::EvtgAoi0Bft23Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::EvtgAoi0Bft23Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::EvtgAoi0Bft23Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::EvtgAoi0Bft23Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::EvtgAoi0Bft23Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::EvtgAoi0Bft23Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::EvtgAoi0Bft23Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::EvtgAoi0Bft23Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::EvtgAoi0Bft23Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::EvtgAoi0Bft23Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::EvtgAoi0Bft23Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::EvtgAoi0Bft23Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::EvtgAoi0Bft23Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi0Bft23 {
    #[inline(always)]
    fn default() -> EvtgAoi0Bft23 {
        EvtgAoi0Bft23(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Bft23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Bft23")
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
impl defmt::Format for EvtgAoi0Bft23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Bft23 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
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
#[doc = "AOI0 Output Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Filt(pub u16);
impl EvtgAoi0Filt {
    #[doc = "Output Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Output Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Output Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for EvtgAoi0Filt {
    #[inline(always)]
    fn default() -> EvtgAoi0Filt {
        EvtgAoi0Filt(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi0Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "AOI1 Boolean Function Term 0 and 1 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Bft01(pub u16);
impl EvtgAoi1Bft01 {
    #[doc = "Product Term 1, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::EvtgAoi1Bft01Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::EvtgAoi1Bft01Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::EvtgAoi1Bft01Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::EvtgAoi1Bft01Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::EvtgAoi1Bft01Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::EvtgAoi1Bft01Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::EvtgAoi1Bft01Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::EvtgAoi1Bft01Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::EvtgAoi1Bft01Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::EvtgAoi1Bft01Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::EvtgAoi1Bft01Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::EvtgAoi1Bft01Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::EvtgAoi1Bft01Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::EvtgAoi1Bft01Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::EvtgAoi1Bft01Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::EvtgAoi1Bft01Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::EvtgAoi1Bft01Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi1Bft01 {
    #[inline(always)]
    fn default() -> EvtgAoi1Bft01 {
        EvtgAoi1Bft01(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Bft01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Bft01")
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
impl defmt::Format for EvtgAoi1Bft01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Bft01 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
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
#[doc = "AOI1 Boolean Function Term 2 and 3 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Bft23(pub u16);
impl EvtgAoi1Bft23 {
    #[doc = "Product Term 3, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::EvtgAoi1Bft23Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::EvtgAoi1Bft23Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::EvtgAoi1Bft23Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::EvtgAoi1Bft23Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::EvtgAoi1Bft23Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::EvtgAoi1Bft23Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::EvtgAoi1Bft23Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::EvtgAoi1Bft23Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, D Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::EvtgAoi1Bft23Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, D Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::EvtgAoi1Bft23Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, C Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::EvtgAoi1Bft23Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, C Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::EvtgAoi1Bft23Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, B Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::EvtgAoi1Bft23Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, B Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::EvtgAoi1Bft23Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, A Input Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::EvtgAoi1Bft23Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::EvtgAoi1Bft23Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, A Input Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::EvtgAoi1Bft23Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi1Bft23 {
    #[inline(always)]
    fn default() -> EvtgAoi1Bft23 {
        EvtgAoi1Bft23(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Bft23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Bft23")
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
impl defmt::Format for EvtgAoi1Bft23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Bft23 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
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
#[doc = "AOI1 Output Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Filt(pub u16);
impl EvtgAoi1Filt {
    #[doc = "Output Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Output Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Output Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for EvtgAoi1Filt {
    #[inline(always)]
    fn default() -> EvtgAoi1Filt {
        EvtgAoi1Filt(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi1Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgCtrl(pub u16);
impl EvtgCtrl {
    #[doc = "Flip flop Initial Value Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn ff_init(&self) -> super::vals::FfInit {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FfInit::from_bits(val as u8)
    }
    #[doc = "Flip flop Initial Value Configuration"]
    #[inline(always)]
    pub const fn set_ff_init(&mut self, val: super::vals::FfInit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Flip-Flop Initial Output Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn init_en(&self) -> super::vals::InitEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::InitEn::from_bits(val as u8)
    }
    #[doc = "Flip-Flop Initial Output Enable Control"]
    #[inline(always)]
    pub const fn set_init_en(&mut self, val: super::vals::InitEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Flip-Flop Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mode_sel(&self) -> super::vals::ModeSel {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::ModeSel::from_bits(val as u8)
    }
    #[doc = "Flip-Flop Mode Selection"]
    #[inline(always)]
    pub const fn set_mode_sel(&mut self, val: super::vals::ModeSel) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u16) & 0x07) << 2usize);
    }
    #[doc = "EVTG Output Feedback Override Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fb_ovrd(&self) -> super::vals::FbOvrd {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::FbOvrd::from_bits(val as u8)
    }
    #[doc = "EVTG Output Feedback Override Control"]
    #[inline(always)]
    pub const fn set_fb_ovrd(&mut self, val: super::vals::FbOvrd) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Synchronize Control"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_ctrl(&self) -> super::vals::SyncCtrl {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SyncCtrl::from_bits(val as u8)
    }
    #[doc = "Synchronize Control"]
    #[inline(always)]
    pub const fn set_sync_ctrl(&mut self, val: super::vals::SyncCtrl) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Force Bypass Control"]
    #[must_use]
    #[inline(always)]
    pub const fn force_bypass(&self) -> super::vals::ForceBypass {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::ForceBypass::from_bits(val as u8)
    }
    #[doc = "Force Bypass Control"]
    #[inline(always)]
    pub const fn set_force_bypass(&mut self, val: super::vals::ForceBypass) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
}
impl Default for EvtgCtrl {
    #[inline(always)]
    fn default() -> EvtgCtrl {
        EvtgCtrl(0)
    }
}
impl core::fmt::Debug for EvtgCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgCtrl")
            .field("ff_init", &self.ff_init())
            .field("init_en", &self.init_en())
            .field("mode_sel", &self.mode_sel())
            .field("fb_ovrd", &self.fb_ovrd())
            .field("sync_ctrl", &self.sync_ctrl())
            .field("force_bypass", &self.force_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgCtrl {{ ff_init: {:?}, init_en: {:?}, mode_sel: {:?}, fb_ovrd: {:?}, sync_ctrl: {:?}, force_bypass: {:?} }}",
            self.ff_init(),
            self.init_en(),
            self.mode_sel(),
            self.fb_ovrd(),
            self.sync_ctrl(),
            self.force_bypass()
        )
    }
}
