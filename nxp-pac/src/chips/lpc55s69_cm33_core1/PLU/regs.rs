#[doc = "LUTn input x MUX."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LUT_INP_MUX(pub u32);
impl LUT_INP_MUX {
    #[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[must_use]
    #[inline(always)]
    pub const fn LUTn_INPx(&self) -> super::vals::LUTn_INPx {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LUTn_INPx::from_bits(val as u8)
    }
    #[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[inline(always)]
    pub const fn set_LUTn_INPx(&mut self, val: super::vals::LUTn_INPx) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LUT_INP_MUX {
    #[inline(always)]
    fn default() -> LUT_INP_MUX {
        LUT_INP_MUX(0)
    }
}
impl core::fmt::Debug for LUT_INP_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUT_INP_MUX")
            .field("LUTn_INPx", &self.LUTn_INPx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LUT_INP_MUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LUT_INP_MUX {{ LUTn_INPx: {:?} }}", self.LUTn_INPx())
    }
}
#[doc = "Specifies the Truth Table contents for LUTLUTn."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LUT_TRUTH(pub u32);
impl LUT_TRUTH {
    #[doc = "Specifies the Truth Table contents for LUT0.."]
    #[must_use]
    #[inline(always)]
    pub const fn LUTn_TRUTH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies the Truth Table contents for LUT0.."]
    #[inline(always)]
    pub const fn set_LUTn_TRUTH(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LUT_TRUTH {
    #[inline(always)]
    fn default() -> LUT_TRUTH {
        LUT_TRUTH(0)
    }
}
impl core::fmt::Debug for LUT_TRUTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUT_TRUTH")
            .field("LUTn_TRUTH", &self.LUTn_TRUTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LUT_TRUTH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LUT_TRUTH {{ LUTn_TRUTH: {=u32:?} }}", self.LUTn_TRUTH())
    }
}
#[doc = "Provides the current state of the 8 designated PLU Outputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTPUTS(pub u32);
impl OUTPUTS {
    #[doc = "Provides the current state of the 8 designated PLU Outputs.."]
    #[must_use]
    #[inline(always)]
    pub const fn OUTPUT_STATE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Provides the current state of the 8 designated PLU Outputs.."]
    #[inline(always)]
    pub const fn set_OUTPUT_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for OUTPUTS {
    #[inline(always)]
    fn default() -> OUTPUTS {
        OUTPUTS(0)
    }
}
impl core::fmt::Debug for OUTPUTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTS")
            .field("OUTPUT_STATE", &self.OUTPUT_STATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTPUTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OUTPUTS {{ OUTPUT_STATE: {=u8:?} }}",
            self.OUTPUT_STATE()
        )
    }
}
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTPUT_MUX(pub u32);
impl OUTPUT_MUX {
    #[doc = "Selects the source to be connected to PLU Output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn OUTPUTn(&self) -> super::vals::OUTPUTn {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OUTPUTn::from_bits(val as u8)
    }
    #[doc = "Selects the source to be connected to PLU Output 0."]
    #[inline(always)]
    pub const fn set_OUTPUTn(&mut self, val: super::vals::OUTPUTn) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for OUTPUT_MUX {
    #[inline(always)]
    fn default() -> OUTPUT_MUX {
        OUTPUT_MUX(0)
    }
}
impl core::fmt::Debug for OUTPUT_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUT_MUX")
            .field("OUTPUTn", &self.OUTPUTn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTPUT_MUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUTPUT_MUX {{ OUTPUTn: {:?} }}", self.OUTPUTn())
    }
}
#[doc = "Wakeup interrupt control for PLU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEINT_CTRL(pub u32);
impl WAKEINT_CTRL {
    #[doc = "Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "control input of the PLU, add filtering for glitch."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTER_MODE(&self) -> super::vals::FILTER_MODE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::FILTER_MODE::from_bits(val as u8)
    }
    #[doc = "control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub const fn set_FILTER_MODE(&mut self, val: super::vals::FILTER_MODE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "hclk is divided by 2**filter_clksel."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTER_CLKSEL(&self) -> super::vals::FILTER_CLKSEL {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::FILTER_CLKSEL::from_bits(val as u8)
    }
    #[doc = "hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub const fn set_FILTER_CLKSEL(&mut self, val: super::vals::FILTER_CLKSEL) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "latch the interrupt , then can be cleared with next bit INTR_CLEAR."]
    #[must_use]
    #[inline(always)]
    pub const fn LATCH_ENABLE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "latch the interrupt , then can be cleared with next bit INTR_CLEAR."]
    #[inline(always)]
    pub const fn set_LATCH_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write to clear wakeint_latched."]
    #[must_use]
    #[inline(always)]
    pub const fn INTR_CLEAR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear wakeint_latched."]
    #[inline(always)]
    pub const fn set_INTR_CLEAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for WAKEINT_CTRL {
    #[inline(always)]
    fn default() -> WAKEINT_CTRL {
        WAKEINT_CTRL(0)
    }
}
impl core::fmt::Debug for WAKEINT_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEINT_CTRL")
            .field("MASK", &self.MASK())
            .field("FILTER_MODE", &self.FILTER_MODE())
            .field("FILTER_CLKSEL", &self.FILTER_CLKSEL())
            .field("LATCH_ENABLE", &self.LATCH_ENABLE())
            .field("INTR_CLEAR", &self.INTR_CLEAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAKEINT_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAKEINT_CTRL {{ MASK: {=u8:?}, FILTER_MODE: {:?}, FILTER_CLKSEL: {:?}, LATCH_ENABLE: {=bool:?}, INTR_CLEAR: {=bool:?} }}",
            self.MASK(),
            self.FILTER_MODE(),
            self.FILTER_CLKSEL(),
            self.LATCH_ENABLE(),
            self.INTR_CLEAR()
        )
    }
}
