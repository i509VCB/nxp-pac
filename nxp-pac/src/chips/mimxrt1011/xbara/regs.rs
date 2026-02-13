#[doc = "Crossbar A Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u16);
impl Ctrl0 {
    #[doc = "DMA Enable for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn den0(&self) -> super::vals::Den0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Den0::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_den0(&mut self, val: super::vals::Den0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn ien0(&self) -> super::vals::Ien0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ien0::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_ien0(&mut self, val: super::vals::Ien0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn edge0(&self) -> super::vals::Edge0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edge0::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_edge0(&mut self, val: super::vals::Edge0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge detection status for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn sts0(&self) -> super::vals::Sts0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sts0::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_sts0(&mut self, val: super::vals::Sts0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "DMA Enable for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn den1(&self) -> super::vals::Den1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Den1::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_den1(&mut self, val: super::vals::Den1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn ien1(&self) -> super::vals::Ien1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ien1::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_ien1(&mut self, val: super::vals::Ien1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn edge1(&self) -> super::vals::Edge1 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Edge1::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_edge1(&mut self, val: super::vals::Edge1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Edge detection status for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn sts1(&self) -> super::vals::Sts1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sts1::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_sts1(&mut self, val: super::vals::Sts1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("den0", &self.den0())
            .field("ien0", &self.ien0())
            .field("edge0", &self.edge0())
            .field("sts0", &self.sts0())
            .field("den1", &self.den1())
            .field("ien1", &self.ien1())
            .field("edge1", &self.edge1())
            .field("sts1", &self.sts1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl0 {{ den0: {:?}, ien0: {:?}, edge0: {:?}, sts0: {:?}, den1: {:?}, ien1: {:?}, edge1: {:?}, sts1: {:?} }}",
            self.den0(),
            self.ien0(),
            self.edge0(),
            self.sts0(),
            self.den1(),
            self.ien1(),
            self.edge1(),
            self.sts1()
        )
    }
}
#[doc = "Crossbar A Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u16);
impl Ctrl1 {
    #[doc = "DMA Enable for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn den2(&self) -> super::vals::Den2 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Den2::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_den2(&mut self, val: super::vals::Den2) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn ien2(&self) -> super::vals::Ien2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ien2::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_ien2(&mut self, val: super::vals::Ien2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn edge2(&self) -> super::vals::Edge2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edge2::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_edge2(&mut self, val: super::vals::Edge2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge detection status for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn sts2(&self) -> super::vals::Sts2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sts2::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_sts2(&mut self, val: super::vals::Sts2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "DMA Enable for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn den3(&self) -> super::vals::Den3 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Den3::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_den3(&mut self, val: super::vals::Den3) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn ien3(&self) -> super::vals::Ien3 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ien3::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_ien3(&mut self, val: super::vals::Ien3) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn edge3(&self) -> super::vals::Edge3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Edge3::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_edge3(&mut self, val: super::vals::Edge3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Edge detection status for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn sts3(&self) -> super::vals::Sts3 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sts3::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_sts3(&mut self, val: super::vals::Sts3) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("den2", &self.den2())
            .field("ien2", &self.ien2())
            .field("edge2", &self.edge2())
            .field("sts2", &self.sts2())
            .field("den3", &self.den3())
            .field("ien3", &self.ien3())
            .field("edge3", &self.edge3())
            .field("sts3", &self.sts3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ den2: {:?}, ien2: {:?}, edge2: {:?}, sts2: {:?}, den3: {:?}, ien3: {:?}, edge3: {:?}, sts3: {:?} }}",
            self.den2(),
            self.ien2(),
            self.edge2(),
            self.sts2(),
            self.den3(),
            self.ien3(),
            self.edge3(),
            self.sts3()
        )
    }
}
#[doc = "Crossbar A Select Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel0(pub u16);
impl Sel0 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel0 {
    #[inline(always)]
    fn default() -> Sel0 {
        Sel0(0)
    }
}
impl core::fmt::Debug for Sel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel0")
            .field("sel0", &self.sel0())
            .field("sel1", &self.sel1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel0 {{ sel0: {=u8:?}, sel1: {=u8:?} }}",
            self.sel0(),
            self.sel1()
        )
    }
}
#[doc = "Crossbar A Select Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel1(pub u16);
impl Sel1 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel1 {
    #[inline(always)]
    fn default() -> Sel1 {
        Sel1(0)
    }
}
impl core::fmt::Debug for Sel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel1")
            .field("sel2", &self.sel2())
            .field("sel3", &self.sel3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel1 {{ sel2: {=u8:?}, sel3: {=u8:?} }}",
            self.sel2(),
            self.sel3()
        )
    }
}
#[doc = "Crossbar A Select Register 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel10(pub u16);
impl Sel10 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel20(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel21(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel10 {
    #[inline(always)]
    fn default() -> Sel10 {
        Sel10(0)
    }
}
impl core::fmt::Debug for Sel10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel10")
            .field("sel20", &self.sel20())
            .field("sel21", &self.sel21())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel10 {{ sel20: {=u8:?}, sel21: {=u8:?} }}",
            self.sel20(),
            self.sel21()
        )
    }
}
#[doc = "Crossbar A Select Register 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel11(pub u16);
impl Sel11 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT22 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel22(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT22 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT23 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel23(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT23 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel11 {
    #[inline(always)]
    fn default() -> Sel11 {
        Sel11(0)
    }
}
impl core::fmt::Debug for Sel11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel11")
            .field("sel22", &self.sel22())
            .field("sel23", &self.sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel11 {{ sel22: {=u8:?}, sel23: {=u8:?} }}",
            self.sel22(),
            self.sel23()
        )
    }
}
#[doc = "Crossbar A Select Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel12(pub u16);
impl Sel12 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel24(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel25(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel12 {
    #[inline(always)]
    fn default() -> Sel12 {
        Sel12(0)
    }
}
impl core::fmt::Debug for Sel12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel12")
            .field("sel24", &self.sel24())
            .field("sel25", &self.sel25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel12 {{ sel24: {=u8:?}, sel25: {=u8:?} }}",
            self.sel24(),
            self.sel25()
        )
    }
}
#[doc = "Crossbar A Select Register 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel13(pub u16);
impl Sel13 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel26(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel27(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel13 {
    #[inline(always)]
    fn default() -> Sel13 {
        Sel13(0)
    }
}
impl core::fmt::Debug for Sel13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel13")
            .field("sel26", &self.sel26())
            .field("sel27", &self.sel27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel13 {{ sel26: {=u8:?}, sel27: {=u8:?} }}",
            self.sel26(),
            self.sel27()
        )
    }
}
#[doc = "Crossbar A Select Register 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel14(pub u16);
impl Sel14 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel28(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel29(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel14 {
    #[inline(always)]
    fn default() -> Sel14 {
        Sel14(0)
    }
}
impl core::fmt::Debug for Sel14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel14")
            .field("sel28", &self.sel28())
            .field("sel29", &self.sel29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel14 {{ sel28: {=u8:?}, sel29: {=u8:?} }}",
            self.sel28(),
            self.sel29()
        )
    }
}
#[doc = "Crossbar A Select Register 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel15(pub u16);
impl Sel15 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel30(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel31(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel31(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel15 {
    #[inline(always)]
    fn default() -> Sel15 {
        Sel15(0)
    }
}
impl core::fmt::Debug for Sel15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel15")
            .field("sel30", &self.sel30())
            .field("sel31", &self.sel31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel15 {{ sel30: {=u8:?}, sel31: {=u8:?} }}",
            self.sel30(),
            self.sel31()
        )
    }
}
#[doc = "Crossbar A Select Register 16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel16(pub u16);
impl Sel16 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel32(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel33(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel33(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel16 {
    #[inline(always)]
    fn default() -> Sel16 {
        Sel16(0)
    }
}
impl core::fmt::Debug for Sel16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel16")
            .field("sel32", &self.sel32())
            .field("sel33", &self.sel33())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel16 {{ sel32: {=u8:?}, sel33: {=u8:?} }}",
            self.sel32(),
            self.sel33()
        )
    }
}
#[doc = "Crossbar A Select Register 17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel17(pub u16);
impl Sel17 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel34(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel34(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel35(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel35(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel17 {
    #[inline(always)]
    fn default() -> Sel17 {
        Sel17(0)
    }
}
impl core::fmt::Debug for Sel17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel17")
            .field("sel34", &self.sel34())
            .field("sel35", &self.sel35())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel17 {{ sel34: {=u8:?}, sel35: {=u8:?} }}",
            self.sel34(),
            self.sel35()
        )
    }
}
#[doc = "Crossbar A Select Register 18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel18(pub u16);
impl Sel18 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel36(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel36(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel37(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel37(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel18 {
    #[inline(always)]
    fn default() -> Sel18 {
        Sel18(0)
    }
}
impl core::fmt::Debug for Sel18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel18")
            .field("sel36", &self.sel36())
            .field("sel37", &self.sel37())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel18 {{ sel36: {=u8:?}, sel37: {=u8:?} }}",
            self.sel36(),
            self.sel37()
        )
    }
}
#[doc = "Crossbar A Select Register 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel19(pub u16);
impl Sel19 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel38(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel38(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel39(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel39(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel19 {
    #[inline(always)]
    fn default() -> Sel19 {
        Sel19(0)
    }
}
impl core::fmt::Debug for Sel19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel19")
            .field("sel38", &self.sel38())
            .field("sel39", &self.sel39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel19 {{ sel38: {=u8:?}, sel39: {=u8:?} }}",
            self.sel38(),
            self.sel39()
        )
    }
}
#[doc = "Crossbar A Select Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel2(pub u16);
impl Sel2 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel2 {
    #[inline(always)]
    fn default() -> Sel2 {
        Sel2(0)
    }
}
impl core::fmt::Debug for Sel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel2")
            .field("sel4", &self.sel4())
            .field("sel5", &self.sel5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel2 {{ sel4: {=u8:?}, sel5: {=u8:?} }}",
            self.sel4(),
            self.sel5()
        )
    }
}
#[doc = "Crossbar A Select Register 20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel20(pub u16);
impl Sel20 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel40(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel40(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel41(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel41(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel20 {
    #[inline(always)]
    fn default() -> Sel20 {
        Sel20(0)
    }
}
impl core::fmt::Debug for Sel20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel20")
            .field("sel40", &self.sel40())
            .field("sel41", &self.sel41())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel20 {{ sel40: {=u8:?}, sel41: {=u8:?} }}",
            self.sel40(),
            self.sel41()
        )
    }
}
#[doc = "Crossbar A Select Register 21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel21(pub u16);
impl Sel21 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel42(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel42(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel43(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel43(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel21 {
    #[inline(always)]
    fn default() -> Sel21 {
        Sel21(0)
    }
}
impl core::fmt::Debug for Sel21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel21")
            .field("sel42", &self.sel42())
            .field("sel43", &self.sel43())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel21 {{ sel42: {=u8:?}, sel43: {=u8:?} }}",
            self.sel42(),
            self.sel43()
        )
    }
}
#[doc = "Crossbar A Select Register 22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel22(pub u16);
impl Sel22 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel44(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel44(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel45(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel45(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel22 {
    #[inline(always)]
    fn default() -> Sel22 {
        Sel22(0)
    }
}
impl core::fmt::Debug for Sel22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel22")
            .field("sel44", &self.sel44())
            .field("sel45", &self.sel45())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel22 {{ sel44: {=u8:?}, sel45: {=u8:?} }}",
            self.sel44(),
            self.sel45()
        )
    }
}
#[doc = "Crossbar A Select Register 23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel23(pub u16);
impl Sel23 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel46(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel46(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel47(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel47(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel23 {
    #[inline(always)]
    fn default() -> Sel23 {
        Sel23(0)
    }
}
impl core::fmt::Debug for Sel23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel23")
            .field("sel46", &self.sel46())
            .field("sel47", &self.sel47())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel23 {{ sel46: {=u8:?}, sel47: {=u8:?} }}",
            self.sel46(),
            self.sel47()
        )
    }
}
#[doc = "Crossbar A Select Register 24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel24(pub u16);
impl Sel24 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel48(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel48(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel49(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel49(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel24 {
    #[inline(always)]
    fn default() -> Sel24 {
        Sel24(0)
    }
}
impl core::fmt::Debug for Sel24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel24")
            .field("sel48", &self.sel48())
            .field("sel49", &self.sel49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel24 {{ sel48: {=u8:?}, sel49: {=u8:?} }}",
            self.sel48(),
            self.sel49()
        )
    }
}
#[doc = "Crossbar A Select Register 25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel25(pub u16);
impl Sel25 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT50 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel50(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT50 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel50(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT51 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel51(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT51 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel51(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel25 {
    #[inline(always)]
    fn default() -> Sel25 {
        Sel25(0)
    }
}
impl core::fmt::Debug for Sel25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel25")
            .field("sel50", &self.sel50())
            .field("sel51", &self.sel51())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel25 {{ sel50: {=u8:?}, sel51: {=u8:?} }}",
            self.sel50(),
            self.sel51()
        )
    }
}
#[doc = "Crossbar A Select Register 26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel26(pub u16);
impl Sel26 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT52 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel52(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT52 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel52(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT53 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel53(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT53 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel53(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel26 {
    #[inline(always)]
    fn default() -> Sel26 {
        Sel26(0)
    }
}
impl core::fmt::Debug for Sel26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel26")
            .field("sel52", &self.sel52())
            .field("sel53", &self.sel53())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel26 {{ sel52: {=u8:?}, sel53: {=u8:?} }}",
            self.sel52(),
            self.sel53()
        )
    }
}
#[doc = "Crossbar A Select Register 27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel27(pub u16);
impl Sel27 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel54(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel54(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel55(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel55(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel27 {
    #[inline(always)]
    fn default() -> Sel27 {
        Sel27(0)
    }
}
impl core::fmt::Debug for Sel27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel27")
            .field("sel54", &self.sel54())
            .field("sel55", &self.sel55())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel27 {{ sel54: {=u8:?}, sel55: {=u8:?} }}",
            self.sel54(),
            self.sel55()
        )
    }
}
#[doc = "Crossbar A Select Register 28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel28(pub u16);
impl Sel28 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel56(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel56(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel57(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel57(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel28 {
    #[inline(always)]
    fn default() -> Sel28 {
        Sel28(0)
    }
}
impl core::fmt::Debug for Sel28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel28")
            .field("sel56", &self.sel56())
            .field("sel57", &self.sel57())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel28 {{ sel56: {=u8:?}, sel57: {=u8:?} }}",
            self.sel56(),
            self.sel57()
        )
    }
}
#[doc = "Crossbar A Select Register 29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel29(pub u16);
impl Sel29 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel58(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel58(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel59(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel59(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel29 {
    #[inline(always)]
    fn default() -> Sel29 {
        Sel29(0)
    }
}
impl core::fmt::Debug for Sel29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel29")
            .field("sel58", &self.sel58())
            .field("sel59", &self.sel59())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel29 {{ sel58: {=u8:?}, sel59: {=u8:?} }}",
            self.sel58(),
            self.sel59()
        )
    }
}
#[doc = "Crossbar A Select Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel3(pub u16);
impl Sel3 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel7(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel3 {
    #[inline(always)]
    fn default() -> Sel3 {
        Sel3(0)
    }
}
impl core::fmt::Debug for Sel3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel3")
            .field("sel6", &self.sel6())
            .field("sel7", &self.sel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel3 {{ sel6: {=u8:?}, sel7: {=u8:?} }}",
            self.sel6(),
            self.sel7()
        )
    }
}
#[doc = "Crossbar A Select Register 30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel30(pub u16);
impl Sel30 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel60(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel60(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel61(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel61(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel30 {
    #[inline(always)]
    fn default() -> Sel30 {
        Sel30(0)
    }
}
impl core::fmt::Debug for Sel30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel30")
            .field("sel60", &self.sel60())
            .field("sel61", &self.sel61())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel30 {{ sel60: {=u8:?}, sel61: {=u8:?} }}",
            self.sel60(),
            self.sel61()
        )
    }
}
#[doc = "Crossbar A Select Register 31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel31(pub u16);
impl Sel31 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel62(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel62(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel63(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel63(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel31 {
    #[inline(always)]
    fn default() -> Sel31 {
        Sel31(0)
    }
}
impl core::fmt::Debug for Sel31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel31")
            .field("sel62", &self.sel62())
            .field("sel63", &self.sel63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel31 {{ sel62: {=u8:?}, sel63: {=u8:?} }}",
            self.sel62(),
            self.sel63()
        )
    }
}
#[doc = "Crossbar A Select Register 32"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel32(pub u16);
impl Sel32 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel65(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel65(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel32 {
    #[inline(always)]
    fn default() -> Sel32 {
        Sel32(0)
    }
}
impl core::fmt::Debug for Sel32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel32")
            .field("sel64", &self.sel64())
            .field("sel65", &self.sel65())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel32 {{ sel64: {=u8:?}, sel65: {=u8:?} }}",
            self.sel64(),
            self.sel65()
        )
    }
}
#[doc = "Crossbar A Select Register 33"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel33(pub u16);
impl Sel33 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel66(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel66(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel67(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel67(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel33 {
    #[inline(always)]
    fn default() -> Sel33 {
        Sel33(0)
    }
}
impl core::fmt::Debug for Sel33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel33")
            .field("sel66", &self.sel66())
            .field("sel67", &self.sel67())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel33 {{ sel66: {=u8:?}, sel67: {=u8:?} }}",
            self.sel66(),
            self.sel67()
        )
    }
}
#[doc = "Crossbar A Select Register 34"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel34(pub u16);
impl Sel34 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel68(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel68(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel69(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel69(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel34 {
    #[inline(always)]
    fn default() -> Sel34 {
        Sel34(0)
    }
}
impl core::fmt::Debug for Sel34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel34")
            .field("sel68", &self.sel68())
            .field("sel69", &self.sel69())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel34 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel34 {{ sel68: {=u8:?}, sel69: {=u8:?} }}",
            self.sel68(),
            self.sel69()
        )
    }
}
#[doc = "Crossbar A Select Register 35"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel35(pub u16);
impl Sel35 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel70(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel70(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel71(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel71(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel35 {
    #[inline(always)]
    fn default() -> Sel35 {
        Sel35(0)
    }
}
impl core::fmt::Debug for Sel35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel35")
            .field("sel70", &self.sel70())
            .field("sel71", &self.sel71())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel35 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel35 {{ sel70: {=u8:?}, sel71: {=u8:?} }}",
            self.sel70(),
            self.sel71()
        )
    }
}
#[doc = "Crossbar A Select Register 36"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel36(pub u16);
impl Sel36 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel72(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel72(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel73(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel73(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel36 {
    #[inline(always)]
    fn default() -> Sel36 {
        Sel36(0)
    }
}
impl core::fmt::Debug for Sel36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel36")
            .field("sel72", &self.sel72())
            .field("sel73", &self.sel73())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel36 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel36 {{ sel72: {=u8:?}, sel73: {=u8:?} }}",
            self.sel72(),
            self.sel73()
        )
    }
}
#[doc = "Crossbar A Select Register 37"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel37(pub u16);
impl Sel37 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT74 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel74(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT74 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel74(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT75 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel75(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT75 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel75(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel37 {
    #[inline(always)]
    fn default() -> Sel37 {
        Sel37(0)
    }
}
impl core::fmt::Debug for Sel37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel37")
            .field("sel74", &self.sel74())
            .field("sel75", &self.sel75())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel37 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel37 {{ sel74: {=u8:?}, sel75: {=u8:?} }}",
            self.sel74(),
            self.sel75()
        )
    }
}
#[doc = "Crossbar A Select Register 38"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel38(pub u16);
impl Sel38 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT76 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel76(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT76 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel76(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT77 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel77(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT77 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel77(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel38 {
    #[inline(always)]
    fn default() -> Sel38 {
        Sel38(0)
    }
}
impl core::fmt::Debug for Sel38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel38")
            .field("sel76", &self.sel76())
            .field("sel77", &self.sel77())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel38 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel38 {{ sel76: {=u8:?}, sel77: {=u8:?} }}",
            self.sel76(),
            self.sel77()
        )
    }
}
#[doc = "Crossbar A Select Register 39"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel39(pub u16);
impl Sel39 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel78(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel78(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel79(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel79(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel39 {
    #[inline(always)]
    fn default() -> Sel39 {
        Sel39(0)
    }
}
impl core::fmt::Debug for Sel39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel39")
            .field("sel78", &self.sel78())
            .field("sel79", &self.sel79())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel39 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel39 {{ sel78: {=u8:?}, sel79: {=u8:?} }}",
            self.sel78(),
            self.sel79()
        )
    }
}
#[doc = "Crossbar A Select Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel4(pub u16);
impl Sel4 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel4 {
    #[inline(always)]
    fn default() -> Sel4 {
        Sel4(0)
    }
}
impl core::fmt::Debug for Sel4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel4")
            .field("sel8", &self.sel8())
            .field("sel9", &self.sel9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel4 {{ sel8: {=u8:?}, sel9: {=u8:?} }}",
            self.sel8(),
            self.sel9()
        )
    }
}
#[doc = "Crossbar A Select Register 40"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel40(pub u16);
impl Sel40 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT80 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel80(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT80 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel80(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT81 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel81(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT81 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel81(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel40 {
    #[inline(always)]
    fn default() -> Sel40 {
        Sel40(0)
    }
}
impl core::fmt::Debug for Sel40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel40")
            .field("sel80", &self.sel80())
            .field("sel81", &self.sel81())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel40 {{ sel80: {=u8:?}, sel81: {=u8:?} }}",
            self.sel80(),
            self.sel81()
        )
    }
}
#[doc = "Crossbar A Select Register 41"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel41(pub u16);
impl Sel41 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel82(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel82(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel83(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel83(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel41 {
    #[inline(always)]
    fn default() -> Sel41 {
        Sel41(0)
    }
}
impl core::fmt::Debug for Sel41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel41")
            .field("sel82", &self.sel82())
            .field("sel83", &self.sel83())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel41 {{ sel82: {=u8:?}, sel83: {=u8:?} }}",
            self.sel82(),
            self.sel83()
        )
    }
}
#[doc = "Crossbar A Select Register 42"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel42(pub u16);
impl Sel42 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel84(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel84(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel85(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel85(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel42 {
    #[inline(always)]
    fn default() -> Sel42 {
        Sel42(0)
    }
}
impl core::fmt::Debug for Sel42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel42")
            .field("sel84", &self.sel84())
            .field("sel85", &self.sel85())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel42 {{ sel84: {=u8:?}, sel85: {=u8:?} }}",
            self.sel84(),
            self.sel85()
        )
    }
}
#[doc = "Crossbar A Select Register 43"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel43(pub u16);
impl Sel43 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel86(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel86(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel87(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel87(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel43 {
    #[inline(always)]
    fn default() -> Sel43 {
        Sel43(0)
    }
}
impl core::fmt::Debug for Sel43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel43")
            .field("sel86", &self.sel86())
            .field("sel87", &self.sel87())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel43 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel43 {{ sel86: {=u8:?}, sel87: {=u8:?} }}",
            self.sel86(),
            self.sel87()
        )
    }
}
#[doc = "Crossbar A Select Register 44"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel44(pub u16);
impl Sel44 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel88(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel88(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel89(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel89(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel44 {
    #[inline(always)]
    fn default() -> Sel44 {
        Sel44(0)
    }
}
impl core::fmt::Debug for Sel44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel44")
            .field("sel88", &self.sel88())
            .field("sel89", &self.sel89())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel44 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel44 {{ sel88: {=u8:?}, sel89: {=u8:?} }}",
            self.sel88(),
            self.sel89()
        )
    }
}
#[doc = "Crossbar A Select Register 45"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel45(pub u16);
impl Sel45 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel90(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel90(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel91(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel91(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel45 {
    #[inline(always)]
    fn default() -> Sel45 {
        Sel45(0)
    }
}
impl core::fmt::Debug for Sel45 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel45")
            .field("sel90", &self.sel90())
            .field("sel91", &self.sel91())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel45 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel45 {{ sel90: {=u8:?}, sel91: {=u8:?} }}",
            self.sel90(),
            self.sel91()
        )
    }
}
#[doc = "Crossbar A Select Register 46"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel46(pub u16);
impl Sel46 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel92(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel92(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel93(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel93(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel46 {
    #[inline(always)]
    fn default() -> Sel46 {
        Sel46(0)
    }
}
impl core::fmt::Debug for Sel46 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel46")
            .field("sel92", &self.sel92())
            .field("sel93", &self.sel93())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel46 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel46 {{ sel92: {=u8:?}, sel93: {=u8:?} }}",
            self.sel92(),
            self.sel93()
        )
    }
}
#[doc = "Crossbar A Select Register 47"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel47(pub u16);
impl Sel47 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel94(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel94(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel95(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel95(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel47 {
    #[inline(always)]
    fn default() -> Sel47 {
        Sel47(0)
    }
}
impl core::fmt::Debug for Sel47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel47")
            .field("sel94", &self.sel94())
            .field("sel95", &self.sel95())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel47 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel47 {{ sel94: {=u8:?}, sel95: {=u8:?} }}",
            self.sel94(),
            self.sel95()
        )
    }
}
#[doc = "Crossbar A Select Register 48"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel48(pub u16);
impl Sel48 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel96(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel96(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel97(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel97(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel48 {
    #[inline(always)]
    fn default() -> Sel48 {
        Sel48(0)
    }
}
impl core::fmt::Debug for Sel48 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel48")
            .field("sel96", &self.sel96())
            .field("sel97", &self.sel97())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel48 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel48 {{ sel96: {=u8:?}, sel97: {=u8:?} }}",
            self.sel96(),
            self.sel97()
        )
    }
}
#[doc = "Crossbar A Select Register 49"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel49(pub u16);
impl Sel49 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel98(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel98(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel99(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel99(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel49 {
    #[inline(always)]
    fn default() -> Sel49 {
        Sel49(0)
    }
}
impl core::fmt::Debug for Sel49 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel49")
            .field("sel98", &self.sel98())
            .field("sel99", &self.sel99())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel49 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel49 {{ sel98: {=u8:?}, sel99: {=u8:?} }}",
            self.sel98(),
            self.sel99()
        )
    }
}
#[doc = "Crossbar A Select Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel5(pub u16);
impl Sel5 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel11(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel5 {
    #[inline(always)]
    fn default() -> Sel5 {
        Sel5(0)
    }
}
impl core::fmt::Debug for Sel5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel5")
            .field("sel10", &self.sel10())
            .field("sel11", &self.sel11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel5 {{ sel10: {=u8:?}, sel11: {=u8:?} }}",
            self.sel10(),
            self.sel11()
        )
    }
}
#[doc = "Crossbar A Select Register 50"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel50(pub u16);
impl Sel50 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel100(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel100(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel101(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel101(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel50 {
    #[inline(always)]
    fn default() -> Sel50 {
        Sel50(0)
    }
}
impl core::fmt::Debug for Sel50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel50")
            .field("sel100", &self.sel100())
            .field("sel101", &self.sel101())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel50 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel50 {{ sel100: {=u8:?}, sel101: {=u8:?} }}",
            self.sel100(),
            self.sel101()
        )
    }
}
#[doc = "Crossbar A Select Register 51"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel51(pub u16);
impl Sel51 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel102(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel102(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel103(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel103(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel51 {
    #[inline(always)]
    fn default() -> Sel51 {
        Sel51(0)
    }
}
impl core::fmt::Debug for Sel51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel51")
            .field("sel102", &self.sel102())
            .field("sel103", &self.sel103())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel51 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel51 {{ sel102: {=u8:?}, sel103: {=u8:?} }}",
            self.sel102(),
            self.sel103()
        )
    }
}
#[doc = "Crossbar A Select Register 52"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel52(pub u16);
impl Sel52 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel104(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel104(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel105(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel105(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel52 {
    #[inline(always)]
    fn default() -> Sel52 {
        Sel52(0)
    }
}
impl core::fmt::Debug for Sel52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel52")
            .field("sel104", &self.sel104())
            .field("sel105", &self.sel105())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel52 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel52 {{ sel104: {=u8:?}, sel105: {=u8:?} }}",
            self.sel104(),
            self.sel105()
        )
    }
}
#[doc = "Crossbar A Select Register 53"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel53(pub u16);
impl Sel53 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel106(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel106(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel107(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel107(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel53 {
    #[inline(always)]
    fn default() -> Sel53 {
        Sel53(0)
    }
}
impl core::fmt::Debug for Sel53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel53")
            .field("sel106", &self.sel106())
            .field("sel107", &self.sel107())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel53 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel53 {{ sel106: {=u8:?}, sel107: {=u8:?} }}",
            self.sel106(),
            self.sel107()
        )
    }
}
#[doc = "Crossbar A Select Register 54"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel54(pub u16);
impl Sel54 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel108(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel108(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel109(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel109(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel54 {
    #[inline(always)]
    fn default() -> Sel54 {
        Sel54(0)
    }
}
impl core::fmt::Debug for Sel54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel54")
            .field("sel108", &self.sel108())
            .field("sel109", &self.sel109())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel54 {{ sel108: {=u8:?}, sel109: {=u8:?} }}",
            self.sel108(),
            self.sel109()
        )
    }
}
#[doc = "Crossbar A Select Register 55"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel55(pub u16);
impl Sel55 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel110(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel110(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel111(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel111(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel55 {
    #[inline(always)]
    fn default() -> Sel55 {
        Sel55(0)
    }
}
impl core::fmt::Debug for Sel55 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel55")
            .field("sel110", &self.sel110())
            .field("sel111", &self.sel111())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel55 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel55 {{ sel110: {=u8:?}, sel111: {=u8:?} }}",
            self.sel110(),
            self.sel111()
        )
    }
}
#[doc = "Crossbar A Select Register 56"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel56(pub u16);
impl Sel56 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel112(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel112(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel113(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel113(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel56 {
    #[inline(always)]
    fn default() -> Sel56 {
        Sel56(0)
    }
}
impl core::fmt::Debug for Sel56 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel56")
            .field("sel112", &self.sel112())
            .field("sel113", &self.sel113())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel56 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel56 {{ sel112: {=u8:?}, sel113: {=u8:?} }}",
            self.sel112(),
            self.sel113()
        )
    }
}
#[doc = "Crossbar A Select Register 57"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel57(pub u16);
impl Sel57 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel114(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel114(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel115(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel115(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel57 {
    #[inline(always)]
    fn default() -> Sel57 {
        Sel57(0)
    }
}
impl core::fmt::Debug for Sel57 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel57")
            .field("sel114", &self.sel114())
            .field("sel115", &self.sel115())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel57 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel57 {{ sel114: {=u8:?}, sel115: {=u8:?} }}",
            self.sel114(),
            self.sel115()
        )
    }
}
#[doc = "Crossbar A Select Register 58"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel58(pub u16);
impl Sel58 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel116(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel116(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel117(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel117(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel58 {
    #[inline(always)]
    fn default() -> Sel58 {
        Sel58(0)
    }
}
impl core::fmt::Debug for Sel58 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel58")
            .field("sel116", &self.sel116())
            .field("sel117", &self.sel117())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel58 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel58 {{ sel116: {=u8:?}, sel117: {=u8:?} }}",
            self.sel116(),
            self.sel117()
        )
    }
}
#[doc = "Crossbar A Select Register 59"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel59(pub u16);
impl Sel59 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel118(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel118(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel119(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel119(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel59 {
    #[inline(always)]
    fn default() -> Sel59 {
        Sel59(0)
    }
}
impl core::fmt::Debug for Sel59 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel59")
            .field("sel118", &self.sel118())
            .field("sel119", &self.sel119())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel59 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel59 {{ sel118: {=u8:?}, sel119: {=u8:?} }}",
            self.sel118(),
            self.sel119()
        )
    }
}
#[doc = "Crossbar A Select Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel6(pub u16);
impl Sel6 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel6 {
    #[inline(always)]
    fn default() -> Sel6 {
        Sel6(0)
    }
}
impl core::fmt::Debug for Sel6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel6")
            .field("sel12", &self.sel12())
            .field("sel13", &self.sel13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel6 {{ sel12: {=u8:?}, sel13: {=u8:?} }}",
            self.sel12(),
            self.sel13()
        )
    }
}
#[doc = "Crossbar A Select Register 60"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel60(pub u16);
impl Sel60 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT120 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel120(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT120 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel120(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT121 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel121(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT121 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel121(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel60 {
    #[inline(always)]
    fn default() -> Sel60 {
        Sel60(0)
    }
}
impl core::fmt::Debug for Sel60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel60")
            .field("sel120", &self.sel120())
            .field("sel121", &self.sel121())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel60 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel60 {{ sel120: {=u8:?}, sel121: {=u8:?} }}",
            self.sel120(),
            self.sel121()
        )
    }
}
#[doc = "Crossbar A Select Register 61"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel61(pub u16);
impl Sel61 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel122(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel122(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel123(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel123(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel61 {
    #[inline(always)]
    fn default() -> Sel61 {
        Sel61(0)
    }
}
impl core::fmt::Debug for Sel61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel61")
            .field("sel122", &self.sel122())
            .field("sel123", &self.sel123())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel61 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel61 {{ sel122: {=u8:?}, sel123: {=u8:?} }}",
            self.sel122(),
            self.sel123()
        )
    }
}
#[doc = "Crossbar A Select Register 62"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel62(pub u16);
impl Sel62 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel124(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel124(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel125(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel125(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel62 {
    #[inline(always)]
    fn default() -> Sel62 {
        Sel62(0)
    }
}
impl core::fmt::Debug for Sel62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel62")
            .field("sel124", &self.sel124())
            .field("sel125", &self.sel125())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel62 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel62 {{ sel124: {=u8:?}, sel125: {=u8:?} }}",
            self.sel124(),
            self.sel125()
        )
    }
}
#[doc = "Crossbar A Select Register 63"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel63(pub u16);
impl Sel63 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel126(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel126(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel127(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel127(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel63 {
    #[inline(always)]
    fn default() -> Sel63 {
        Sel63(0)
    }
}
impl core::fmt::Debug for Sel63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel63")
            .field("sel126", &self.sel126())
            .field("sel127", &self.sel127())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel63 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel63 {{ sel126: {=u8:?}, sel127: {=u8:?} }}",
            self.sel126(),
            self.sel127()
        )
    }
}
#[doc = "Crossbar A Select Register 64"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel64(pub u16);
impl Sel64 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel128(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel128(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel129(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel129(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel64 {
    #[inline(always)]
    fn default() -> Sel64 {
        Sel64(0)
    }
}
impl core::fmt::Debug for Sel64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel64")
            .field("sel128", &self.sel128())
            .field("sel129", &self.sel129())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel64 {{ sel128: {=u8:?}, sel129: {=u8:?} }}",
            self.sel128(),
            self.sel129()
        )
    }
}
#[doc = "Crossbar A Select Register 65"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel65(pub u16);
impl Sel65 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel130(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel130(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel131(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel131(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel65 {
    #[inline(always)]
    fn default() -> Sel65 {
        Sel65(0)
    }
}
impl core::fmt::Debug for Sel65 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel65")
            .field("sel130", &self.sel130())
            .field("sel131", &self.sel131())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel65 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel65 {{ sel130: {=u8:?}, sel131: {=u8:?} }}",
            self.sel130(),
            self.sel131()
        )
    }
}
#[doc = "Crossbar A Select Register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel7(pub u16);
impl Sel7 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel14(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel15(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel7 {
    #[inline(always)]
    fn default() -> Sel7 {
        Sel7(0)
    }
}
impl core::fmt::Debug for Sel7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel7")
            .field("sel14", &self.sel14())
            .field("sel15", &self.sel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel7 {{ sel14: {=u8:?}, sel15: {=u8:?} }}",
            self.sel14(),
            self.sel15()
        )
    }
}
#[doc = "Crossbar A Select Register 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel8(pub u16);
impl Sel8 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel17(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel8 {
    #[inline(always)]
    fn default() -> Sel8 {
        Sel8(0)
    }
}
impl core::fmt::Debug for Sel8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel8")
            .field("sel16", &self.sel16())
            .field("sel17", &self.sel17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel8 {{ sel16: {=u8:?}, sel17: {=u8:?} }}",
            self.sel16(),
            self.sel17()
        )
    }
}
#[doc = "Crossbar A Select Register 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel9(pub u16);
impl Sel9 {
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel18(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel19(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u16) & 0x7f) << 8usize);
    }
}
impl Default for Sel9 {
    #[inline(always)]
    fn default() -> Sel9 {
        Sel9(0)
    }
}
impl core::fmt::Debug for Sel9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel9")
            .field("sel18", &self.sel18())
            .field("sel19", &self.sel19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sel9 {{ sel18: {=u8:?}, sel19: {=u8:?} }}",
            self.sel18(),
            self.sel19()
        )
    }
}
