#[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutInpMux(pub u32);
impl LutInpMux {
    #[doc = "Selects the input source to be connected to LUTn_INPx"]
    #[must_use]
    #[inline(always)]
    pub const fn lutn_inpx(&self) -> super::vals::LutnInpx {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LutnInpx::from_bits(val as u8)
    }
    #[doc = "Selects the input source to be connected to LUTn_INPx"]
    #[inline(always)]
    pub const fn set_lutn_inpx(&mut self, val: super::vals::LutnInpx) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LutInpMux {
    #[inline(always)]
    fn default() -> LutInpMux {
        LutInpMux(0)
    }
}
impl core::fmt::Debug for LutInpMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LutInpMux")
            .field("lutn_inpx", &self.lutn_inpx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutInpMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutInpMux {{ lutn_inpx: {:?} }}", self.lutn_inpx())
    }
}
#[doc = "PLU LUT truth table"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutTruth(pub u32);
impl LutTruth {
    #[doc = "LUT truth table"]
    #[must_use]
    #[inline(always)]
    pub const fn lut_truth(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LUT truth table"]
    #[inline(always)]
    pub const fn set_lut_truth(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LutTruth {
    #[inline(always)]
    fn default() -> LutTruth {
        LutTruth(0)
    }
}
impl core::fmt::Debug for LutTruth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LutTruth")
            .field("lut_truth", &self.lut_truth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutTruth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutTruth {{ lut_truth: {=u32:?} }}", self.lut_truth())
    }
}
#[doc = "PLU output multiplexer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutputMux(pub u32);
impl OutputMux {
    #[doc = "Selects the source to be connected to PLU output n."]
    #[must_use]
    #[inline(always)]
    pub const fn output(&self) -> super::vals::Output {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Output::from_bits(val as u8)
    }
    #[doc = "Selects the source to be connected to PLU output n."]
    #[inline(always)]
    pub const fn set_output(&mut self, val: super::vals::Output) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for OutputMux {
    #[inline(always)]
    fn default() -> OutputMux {
        OutputMux(0)
    }
}
impl core::fmt::Debug for OutputMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutputMux")
            .field("output", &self.output())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutputMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutputMux {{ output: {:?} }}", self.output())
    }
}
#[doc = "PLU outputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputs(pub u32);
impl Outputs {
    #[doc = "Output state"]
    #[must_use]
    #[inline(always)]
    pub const fn output_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output state"]
    #[inline(always)]
    pub const fn set_output_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Outputs {
    #[inline(always)]
    fn default() -> Outputs {
        Outputs(0)
    }
}
impl core::fmt::Debug for Outputs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outputs")
            .field("output_state", &self.output_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputs {{ output_state: {=u8:?} }}",
            self.output_state()
        )
    }
}
#[doc = "Wakeup interrupt control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeintCtrl(pub u32);
impl WakeintCtrl {
    #[doc = "Interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Filter Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_mode(&self) -> super::vals::FilterMode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::FilterMode::from_bits(val as u8)
    }
    #[doc = "Filter Mode"]
    #[inline(always)]
    pub const fn set_filter_mode(&mut self, val: super::vals::FilterMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Filter clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_clksel(&self) -> super::vals::FilterClksel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::FilterClksel::from_bits(val as u8)
    }
    #[doc = "Filter clock select"]
    #[inline(always)]
    pub const fn set_filter_clksel(&mut self, val: super::vals::FilterClksel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Latch the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn latch_enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Latch the interrupt"]
    #[inline(always)]
    pub const fn set_latch_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write to clear wakeint_latched"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_clear(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear wakeint_latched"]
    #[inline(always)]
    pub const fn set_intr_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for WakeintCtrl {
    #[inline(always)]
    fn default() -> WakeintCtrl {
        WakeintCtrl(0)
    }
}
impl core::fmt::Debug for WakeintCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeintCtrl")
            .field("mask", &self.mask())
            .field("filter_mode", &self.filter_mode())
            .field("filter_clksel", &self.filter_clksel())
            .field("latch_enable", &self.latch_enable())
            .field("intr_clear", &self.intr_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeintCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WakeintCtrl {{ mask: {=u8:?}, filter_mode: {:?}, filter_clksel: {:?}, latch_enable: {=bool:?}, intr_clear: {=bool:?} }}",
            self.mask(),
            self.filter_mode(),
            self.filter_clksel(),
            self.latch_enable(),
            self.intr_clear()
        )
    }
}
