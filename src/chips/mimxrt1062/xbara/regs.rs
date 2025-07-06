#[doc = "Crossbar B Select Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel0(pub u16);
impl Sel0 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel1(pub u16);
impl Sel1 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel2(pub u16);
impl Sel2 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel3(pub u16);
impl Sel3 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel7(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel4(pub u16);
impl Sel4 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel5(pub u16);
impl Sel5 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel11(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel6(pub u16);
impl Sel6 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
#[doc = "Crossbar B Select Register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel7(pub u16);
impl Sel7 {
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel14(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[must_use]
    #[inline(always)]
    pub const fn sel15(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub const fn set_sel15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
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
