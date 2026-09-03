#[doc = "ADC0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc0Clkdiv(pub u32);
impl MrccAdc0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccAdc0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccAdc0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccAdc0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccAdc0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccAdc0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccAdc0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccAdc0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccAdc0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccAdc0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdc0Clkdiv {
    #[inline(always)]
    fn default() -> MrccAdc0Clkdiv {
        MrccAdc0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdc0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdc0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc0Clksel(pub u32);
impl MrccAdc0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccAdc0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccAdc0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccAdc0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdc0Clksel {
    #[inline(always)]
    fn default() -> MrccAdc0Clksel {
        MrccAdc0Clksel(0)
    }
}
impl core::fmt::Debug for MrccAdc0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdc0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "ADC1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc1Clkdiv(pub u32);
impl MrccAdc1Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccAdc1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccAdc1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccAdc1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccAdc1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccAdc1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccAdc1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccAdc1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccAdc1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccAdc1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdc1Clkdiv {
    #[inline(always)]
    fn default() -> MrccAdc1Clkdiv {
        MrccAdc1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdc1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdc1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc1Clksel(pub u32);
impl MrccAdc1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccAdc1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccAdc1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccAdc1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdc1Clksel {
    #[inline(always)]
    fn default() -> MrccAdc1Clksel {
        MrccAdc1Clksel(0)
    }
}
impl core::fmt::Debug for MrccAdc1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdc1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CLKOUT clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClkdiv(pub u32);
impl MrccClkoutClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccClkoutClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccClkoutClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccClkoutClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccClkoutClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccClkoutClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccClkoutClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccClkoutClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccClkoutClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccClkoutClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccClkoutClkdiv {
    #[inline(always)]
    fn default() -> MrccClkoutClkdiv {
        MrccClkoutClkdiv(0)
    }
}
impl core::fmt::Debug for MrccClkoutClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccClkoutClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CLKOUT clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClksel(pub u32);
impl MrccClkoutClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccClkoutClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccClkoutClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccClkoutClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccClkoutClksel {
    #[inline(always)]
    fn default() -> MrccClkoutClksel {
        MrccClkoutClksel(0)
    }
}
impl core::fmt::Debug for MrccClkoutClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccClkoutClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP0_FUNC clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0FuncClkdiv(pub u32);
impl MrccCmp0FuncClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmp0FuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmp0FuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmp0FuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmp0FuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmp0FuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmp0FuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmp0FuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmp0FuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmp0FuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0FuncClkdiv {
        MrccCmp0FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClkdiv(pub u32);
impl MrccCmp0RrClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmp0RrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmp0RrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmp0RrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmp0RrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmp0RrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmp0RrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmp0RrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmp0RrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmp0RrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0RrClkdiv {
        MrccCmp0RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClksel(pub u32);
impl MrccCmp0RrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCmp0RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCmp0RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCmp0RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp0RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp0RrClksel {
        MrccCmp0RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp0RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP1_FUNC clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1FuncClkdiv(pub u32);
impl MrccCmp1FuncClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmp1FuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmp1FuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmp1FuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmp1FuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmp1FuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmp1FuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmp1FuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmp1FuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmp1FuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1FuncClkdiv {
        MrccCmp1FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClkdiv(pub u32);
impl MrccCmp1RrClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmp1RrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmp1RrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmp1RrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmp1RrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmp1RrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmp1RrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmp1RrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmp1RrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmp1RrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1RrClkdiv {
        MrccCmp1RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClksel(pub u32);
impl MrccCmp1RrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCmp1RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCmp1RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCmp1RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp1RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp1RrClksel {
        MrccCmp1RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp1RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clkdiv(pub u32);
impl MrccCtimer0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimer0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimer0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimer0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimer0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimer0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimer0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimer0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimer0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimer0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer0Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer0Clkdiv {
        MrccCtimer0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clksel(pub u32);
impl MrccCtimer0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimer0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimer0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimer0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer0Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer0Clksel {
        MrccCtimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clkdiv(pub u32);
impl MrccCtimer1Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimer1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimer1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimer1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimer1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimer1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimer1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimer1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimer1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimer1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer1Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer1Clkdiv {
        MrccCtimer1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clksel(pub u32);
impl MrccCtimer1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimer1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimer1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimer1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer1Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer1Clksel {
        MrccCtimer1Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clkdiv(pub u32);
impl MrccCtimer2Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimer2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimer2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimer2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimer2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimer2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimer2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimer2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimer2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimer2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer2Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer2Clkdiv {
        MrccCtimer2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clksel(pub u32);
impl MrccCtimer2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimer2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimer2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimer2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer2Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer2Clksel {
        MrccCtimer2Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clkdiv(pub u32);
impl MrccCtimer3Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimer3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimer3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimer3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimer3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimer3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimer3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimer3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimer3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimer3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer3Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer3Clkdiv {
        MrccCtimer3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clksel(pub u32);
impl MrccCtimer3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimer3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimer3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimer3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer3Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer3Clksel {
        MrccCtimer3Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER4 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clkdiv(pub u32);
impl MrccCtimer4Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimer4ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimer4ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimer4ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimer4ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimer4ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimer4ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimer4ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimer4ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimer4ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer4Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer4Clkdiv {
        MrccCtimer4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER4 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clksel(pub u32);
impl MrccCtimer4Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimer4ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimer4ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimer4ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer4Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer4Clksel {
        MrccCtimer4Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DAC0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clkdiv(pub u32);
impl MrccDac0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccDac0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccDac0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccDac0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccDac0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccDac0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccDac0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccDac0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccDac0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccDac0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDac0Clkdiv {
    #[inline(always)]
    fn default() -> MrccDac0Clkdiv {
        MrccDac0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccDac0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDac0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clksel(pub u32);
impl MrccDac0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccDac0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccDac0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccDac0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccDac0Clksel {
    #[inline(always)]
    fn default() -> MrccDac0Clksel {
        MrccDac0Clksel(0)
    }
}
impl core::fmt::Debug for MrccDac0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDac0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DBG_TRACE clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClkdiv(pub u32);
impl MrccDbgTraceClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccDbgTraceClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccDbgTraceClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccDbgTraceClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccDbgTraceClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccDbgTraceClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccDbgTraceClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccDbgTraceClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccDbgTraceClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccDbgTraceClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDbgTraceClkdiv {
    #[inline(always)]
    fn default() -> MrccDbgTraceClkdiv {
        MrccDbgTraceClkdiv(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDbgTraceClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DBG_TRACE clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClksel(pub u32);
impl MrccDbgTraceClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccDbgTraceClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccDbgTraceClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccDbgTraceClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccDbgTraceClksel {
    #[inline(always)]
    fn default() -> MrccDbgTraceClksel {
        MrccDbgTraceClksel(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDbgTraceClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clkdiv(pub u32);
impl MrccFlexcan0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccFlexcan0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccFlexcan0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccFlexcan0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccFlexcan0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccFlexcan0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccFlexcan0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFlexcan0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFlexcan0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFlexcan0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexcan0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clkdiv {
        MrccFlexcan0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexcan0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clksel(pub u32);
impl MrccFlexcan0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccFlexcan0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccFlexcan0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccFlexcan0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexcan0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clksel {
        MrccFlexcan0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexcan0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXIO0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clkdiv(pub u32);
impl MrccFlexio0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccFlexio0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccFlexio0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccFlexio0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccFlexio0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccFlexio0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccFlexio0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFlexio0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFlexio0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFlexio0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexio0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexio0Clkdiv {
        MrccFlexio0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexio0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clksel(pub u32);
impl MrccFlexio0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccFlexio0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccFlexio0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccFlexio0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexio0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexio0Clksel {
        MrccFlexio0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexio0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FRO_HF_DIV clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFroHfDivClkdiv(pub u32);
impl MrccFroHfDivClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFroHfDivClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFroHfDivClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFroHfDivClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFroHfDivClkdiv {
    #[inline(always)]
    fn default() -> MrccFroHfDivClkdiv {
        MrccFroHfDivClkdiv(0)
    }
}
impl core::fmt::Debug for MrccFroHfDivClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFroHfDivClkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFroHfDivClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFroHfDivClkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "Control Automatic Clock Gating 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc0(pub u32);
impl MrccGlbAcc0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbAcc0 {
    #[inline(always)]
    fn default() -> MrccGlbAcc0 {
        MrccGlbAcc0(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("dma", &self.dma())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.dma(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc1(pub u32);
impl MrccGlbAcc1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MrccGlbAcc1 {
    #[inline(always)]
    fn default() -> MrccGlbAcc1 {
        MrccGlbAcc1(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, romc: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0(pub u32);
impl MrccGlbCc0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbCc0 {
    #[inline(always)]
    fn default() -> MrccGlbCc0 {
        MrccGlbCc0(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("dma", &self.dma())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.dma(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "AHB Clock Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Clr(pub u32);
impl MrccGlbCc0Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc0Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc0Clr {
        MrccGlbCc0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Set(pub u32);
impl MrccGlbCc0Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc0Set {
    #[inline(always)]
    fn default() -> MrccGlbCc0Set {
        MrccGlbCc0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1(pub u32);
impl MrccGlbCc1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MrccGlbCc1 {
    #[inline(always)]
    fn default() -> MrccGlbCc1 {
        MrccGlbCc1(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, romc: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control Clear 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Clr(pub u32);
impl MrccGlbCc1Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc1Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc1Clr {
        MrccGlbCc1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Set(pub u32);
impl MrccGlbCc1Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc1Set {
    #[inline(always)]
    fn default() -> MrccGlbCc1Set {
        MrccGlbCc1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0(pub u32);
impl MrccGlbRst0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbRst0 {
    #[inline(always)]
    fn default() -> MrccGlbRst0 {
        MrccGlbRst0(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("dma", &self.dma())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.dma(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Clr(pub u32);
impl MrccGlbRst0Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst0Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst0Clr {
        MrccGlbRst0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Set(pub u32);
impl MrccGlbRst0Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst0Set {
    #[inline(always)]
    fn default() -> MrccGlbRst0Set {
        MrccGlbRst0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1(pub u32);
impl MrccGlbRst1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for MrccGlbRst1 {
    #[inline(always)]
    fn default() -> MrccGlbRst1 {
        MrccGlbRst1(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Clr(pub u32);
impl MrccGlbRst1Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst1Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst1Clr {
        MrccGlbRst1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Set(pub u32);
impl MrccGlbRst1Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst1Set {
    #[inline(always)]
    fn default() -> MrccGlbRst1Set {
        MrccGlbRst1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "I3C0_FCLK clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClkdiv(pub u32);
impl MrccI3c0FclkClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccI3c0FclkClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccI3c0FclkClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccI3c0FclkClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccI3c0FclkClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccI3c0FclkClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccI3c0FclkClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccI3c0FclkClkdiv {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClkdiv {
        MrccI3c0FclkClkdiv(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccI3c0FclkClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0_FCLK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClksel(pub u32);
impl MrccI3c0FclkClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccI3c0FclkClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccI3c0FclkClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccI3c0FclkClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccI3c0FclkClksel {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClksel {
        MrccI3c0FclkClksel(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccI3c0FclkClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clkdiv(pub u32);
impl MrccLpi2c0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2c0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2c0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2c0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2c0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2c0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2c0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2c0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2c0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2c0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clkdiv {
        MrccLpi2c0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clksel(pub u32);
impl MrccLpi2c0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2c0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2c0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2c0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c0Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clksel {
        MrccLpi2c0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clkdiv(pub u32);
impl MrccLpi2c1Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2c1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2c1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2c1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2c1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2c1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2c1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2c1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2c1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2c1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clkdiv {
        MrccLpi2c1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clksel(pub u32);
impl MrccLpi2c1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2c1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2c1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2c1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c1Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clksel {
        MrccLpi2c1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clkdiv(pub u32);
impl MrccLpi2c2Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2c2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2c2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2c2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2c2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2c2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2c2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2c2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2c2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2c2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clkdiv {
        MrccLpi2c2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clksel(pub u32);
impl MrccLpi2c2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2c2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2c2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2c2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c2Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clksel {
        MrccLpi2c2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clkdiv(pub u32);
impl MrccLpi2c3Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2c3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2c3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2c3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2c3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2c3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2c3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2c3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2c3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2c3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clkdiv {
        MrccLpi2c3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clksel(pub u32);
impl MrccLpi2c3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2c3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2c3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2c3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c3Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clksel {
        MrccLpi2c3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clkdiv(pub u32);
impl MrccLpspi0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpspi0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpspi0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpspi0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpspi0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpspi0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpspi0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpspi0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpspi0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpspi0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi0Clkdiv {
        MrccLpspi0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clksel(pub u32);
impl MrccLpspi0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpspi0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpspi0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpspi0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi0Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi0Clksel {
        MrccLpspi0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clkdiv(pub u32);
impl MrccLpspi1Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpspi1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpspi1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpspi1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpspi1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpspi1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpspi1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpspi1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpspi1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpspi1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi1Clkdiv {
        MrccLpspi1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clksel(pub u32);
impl MrccLpspi1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpspi1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpspi1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpspi1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi1Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi1Clksel {
        MrccLpspi1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPTMR0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clkdiv(pub u32);
impl MrccLptmr0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLptmr0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLptmr0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLptmr0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLptmr0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLptmr0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLptmr0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLptmr0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLptmr0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLptmr0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLptmr0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLptmr0Clkdiv {
        MrccLptmr0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLptmr0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPTMR0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clksel(pub u32);
impl MrccLptmr0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLptmr0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLptmr0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLptmr0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLptmr0Clksel {
    #[inline(always)]
    fn default() -> MrccLptmr0Clksel {
        MrccLptmr0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLptmr0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clkdiv(pub u32);
impl MrccLpuart0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuart0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuart0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuart0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuart0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuart0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuart0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuart0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuart0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuart0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart0Clkdiv {
        MrccLpuart0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clksel(pub u32);
impl MrccLpuart0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuart0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuart0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuart0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart0Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart0Clksel {
        MrccLpuart0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clkdiv(pub u32);
impl MrccLpuart1Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuart1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuart1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuart1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuart1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuart1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuart1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuart1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuart1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuart1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart1Clkdiv {
        MrccLpuart1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clksel(pub u32);
impl MrccLpuart1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuart1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuart1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuart1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart1Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart1Clksel {
        MrccLpuart1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clkdiv(pub u32);
impl MrccLpuart2Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuart2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuart2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuart2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuart2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuart2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuart2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuart2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuart2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuart2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart2Clkdiv {
        MrccLpuart2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clksel(pub u32);
impl MrccLpuart2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuart2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuart2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuart2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart2Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart2Clksel {
        MrccLpuart2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clkdiv(pub u32);
impl MrccLpuart3Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuart3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuart3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuart3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuart3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuart3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuart3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuart3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuart3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuart3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart3Clkdiv {
        MrccLpuart3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clksel(pub u32);
impl MrccLpuart3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuart3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuart3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuart3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart3Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart3Clksel {
        MrccLpuart3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART4 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clkdiv(pub u32);
impl MrccLpuart4Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuart4ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuart4ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuart4ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuart4ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuart4ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuart4ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuart4ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuart4ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuart4ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart4Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart4Clkdiv {
        MrccLpuart4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART4 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clksel(pub u32);
impl MrccLpuart4Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuart4ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuart4ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuart4ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart4Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart4Clksel {
        MrccLpuart4Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "OSTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccOstimer0Clksel(pub u32);
impl MrccOstimer0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccOstimer0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccOstimer0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccOstimer0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccOstimer0Clksel {
    #[inline(always)]
    fn default() -> MrccOstimer0Clksel {
        MrccOstimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccOstimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccOstimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccOstimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccOstimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "SYSTICK clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClkdiv(pub u32);
impl MrccSystickClkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccSystickClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccSystickClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccSystickClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccSystickClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccSystickClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccSystickClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccSystickClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccSystickClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccSystickClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccSystickClkdiv {
    #[inline(always)]
    fn default() -> MrccSystickClkdiv {
        MrccSystickClkdiv(0)
    }
}
impl core::fmt::Debug for MrccSystickClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccSystickClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SYSTICK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClksel(pub u32);
impl MrccSystickClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccSystickClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccSystickClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccSystickClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccSystickClksel {
    #[inline(always)]
    fn default() -> MrccSystickClksel {
        MrccSystickClksel(0)
    }
}
impl core::fmt::Debug for MrccSystickClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccSystickClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccUsb0Clksel(pub u32);
impl MrccUsb0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccUsb0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccUsb0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccUsb0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccUsb0Clksel {
    #[inline(always)]
    fn default() -> MrccUsb0Clksel {
        MrccUsb0Clksel(0)
    }
}
impl core::fmt::Debug for MrccUsb0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccUsb0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccUsb0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccUsb0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "WWDT0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccWwdt0Clkdiv(pub u32);
impl MrccWwdt0Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccWwdt0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccWwdt0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccWwdt0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccWwdt0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccWwdt0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccWwdt0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccWwdt0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccWwdt0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccWwdt0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccWwdt0Clkdiv {
    #[inline(always)]
    fn default() -> MrccWwdt0Clkdiv {
        MrccWwdt0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccWwdt0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccWwdt0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccWwdt0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccWwdt0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
