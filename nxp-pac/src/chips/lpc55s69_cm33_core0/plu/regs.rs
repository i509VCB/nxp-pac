#[doc = "LUTn input x MUX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutInpMux(pub u32);
impl LutInpMux {
    #[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[must_use]
    #[inline(always)]
    pub const fn lutn_inpx(&self) -> super::vals::LutnInpx {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LutnInpx::from_bits(val as u8)
    }
    #[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
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
#[doc = "Specifies the Truth Table contents for LUTLUTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutTruth(pub u32);
impl LutTruth {
    #[doc = "Specifies the Truth Table contents for LUT0.."]
    #[must_use]
    #[inline(always)]
    pub const fn lutn_truth(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    pub const fn set_lutn_truth(&mut self, val: u32) {
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
            .field("lutn_truth", &self.lutn_truth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutTruth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutTruth {{ lutn_truth: {=u32:?} }}", self.lutn_truth())
    }
}
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutputMux(pub u32);
impl OutputMux {
    #[doc = "Selects the source to be connected to PLU Output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn outputn(&self) -> super::vals::Outputn {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Outputn::from_bits(val as u8)
    }
    #[doc = "Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub const fn set_outputn(&mut self, val: super::vals::Outputn) {
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
            .field("outputn", &self.outputn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutputMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutputMux {{ outputn: {:?} }}", self.outputn())
    }
}
#[doc = "Provides the current state of the 8 designated PLU Outputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputs(pub u32);
impl Outputs {
    #[doc = "Provides the current state of the 8 designated PLU Outputs.."]
    #[must_use]
    #[inline(always)]
    pub const fn output_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Provides the current state of the 8 designated PLU Outputs.."]
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
#[doc = "Wakeup interrupt control for PLU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeintCtrl(pub u32);
impl WakeintCtrl {
    #[doc = "Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "control input of the PLU, add filtering for glitch."]
    #[must_use]
    #[inline(always)]
    pub const fn filter_mode(&self) -> super::vals::FilterMode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::FilterMode::from_bits(val as u8)
    }
    #[doc = "control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub const fn set_filter_mode(&mut self, val: super::vals::FilterMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "hclk is divided by 2**filter_clksel."]
    #[must_use]
    #[inline(always)]
    pub const fn filter_clksel(&self) -> super::vals::FilterClksel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::FilterClksel::from_bits(val as u8)
    }
    #[doc = "hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub const fn set_filter_clksel(&mut self, val: super::vals::FilterClksel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[must_use]
    #[inline(always)]
    pub const fn latch_enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
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
