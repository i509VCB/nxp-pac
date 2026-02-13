#[doc = "CMP0_FUNC clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpFuncClkdiv(pub u32);
impl CmpFuncClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CmpFuncClkdiv {
    #[inline(always)]
    fn default() -> CmpFuncClkdiv {
        CmpFuncClkdiv(0)
    }
}
impl core::fmt::Debug for CmpFuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpFuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpFuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmpFuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpRrClkdiv(pub u32);
impl CmpRrClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CmpRrClkdiv {
    #[inline(always)]
    fn default() -> CmpRrClkdiv {
        CmpRrClkdiv(0)
    }
}
impl core::fmt::Debug for CmpRrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpRrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpRrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmpRrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpRrClksel(pub u32);
impl CmpRrClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CmpRrClksel {
    #[inline(always)]
    fn default() -> CmpRrClksel {
        CmpRrClksel(0)
    }
}
impl core::fmt::Debug for CmpRrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpRrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpRrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpRrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtimerClkdiv(pub u32);
impl CtimerClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtimerClkdiv {
    #[inline(always)]
    fn default() -> CtimerClkdiv {
        CtimerClkdiv(0)
    }
}
impl core::fmt::Debug for CtimerClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtimerClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtimerClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtimerClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtimerClksel(pub u32);
impl CtimerClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::CtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::CtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CtimerClksel {
    #[inline(always)]
    fn default() -> CtimerClksel {
        CtimerClksel(0)
    }
}
impl core::fmt::Debug for CtimerClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtimerClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtimerClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CtimerClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DAC0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacClkdiv(pub u32);
impl DacClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for DacClkdiv {
    #[inline(always)]
    fn default() -> DacClkdiv {
        DacClkdiv(0)
    }
}
impl core::fmt::Debug for DacClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DacClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacClksel(pub u32);
impl DacClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::DacClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::DacClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::DacClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for DacClksel {
    #[inline(always)]
    fn default() -> DacClksel {
        DacClksel(0)
    }
}
impl core::fmt::Debug for DacClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexcanClkdiv(pub u32);
impl FlexcanClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FlexcanClkdiv {
    #[inline(always)]
    fn default() -> FlexcanClkdiv {
        FlexcanClkdiv(0)
    }
}
impl core::fmt::Debug for FlexcanClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexcanClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexcanClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexcanClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexcanClksel(pub u32);
impl FlexcanClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::FlexcanClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FlexcanClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::FlexcanClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FlexcanClksel {
    #[inline(always)]
    fn default() -> FlexcanClksel {
        FlexcanClksel(0)
    }
}
impl core::fmt::Debug for FlexcanClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexcanClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexcanClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexcanClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXIO0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioClkdiv(pub u32);
impl FlexioClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FlexioClkdiv {
    #[inline(always)]
    fn default() -> FlexioClkdiv {
        FlexioClkdiv(0)
    }
}
impl core::fmt::Debug for FlexioClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexioClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioClksel(pub u32);
impl FlexioClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::FlexioClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FlexioClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::FlexioClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FlexioClksel {
    #[inline(always)]
    fn default() -> FlexioClksel {
        FlexioClksel(0)
    }
}
impl core::fmt::Debug for FlexioClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexioClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "AHB Clock Control Clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCcClr(pub u32);
impl GlbCcClr {
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
impl Default for GlbCcClr {
    #[inline(always)]
    fn default() -> GlbCcClr {
        GlbCcClr(0)
    }
}
impl core::fmt::Debug for GlbCcClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCcClr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCcClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbCcClr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCcSet(pub u32);
impl GlbCcSet {
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
impl Default for GlbCcSet {
    #[inline(always)]
    fn default() -> GlbCcSet {
        GlbCcSet(0)
    }
}
impl core::fmt::Debug for GlbCcSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCcSet")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCcSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbCcSet {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRstClr(pub u32);
impl GlbRstClr {
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
impl Default for GlbRstClr {
    #[inline(always)]
    fn default() -> GlbRstClr {
        GlbRstClr(0)
    }
}
impl core::fmt::Debug for GlbRstClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRstClr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRstClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbRstClr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRstSet(pub u32);
impl GlbRstSet {
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
impl Default for GlbRstSet {
    #[inline(always)]
    fn default() -> GlbRstSet {
        GlbRstSet(0)
    }
}
impl core::fmt::Debug for GlbRstSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRstSet")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRstSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbRstSet {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "I3C0_FCLK clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3cFclkClkdiv(pub u32);
impl I3cFclkClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3cFclkClkdiv {
    #[inline(always)]
    fn default() -> I3cFclkClkdiv {
        I3cFclkClkdiv(0)
    }
}
impl core::fmt::Debug for I3cFclkClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3cFclkClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3cFclkClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3cFclkClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0_FCLK clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3cFclkClksel(pub u32);
impl I3cFclkClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::FclkClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FclkClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::FclkClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3cFclkClksel {
    #[inline(always)]
    fn default() -> I3cFclkClksel {
        I3cFclkClksel(0)
    }
}
impl core::fmt::Debug for I3cFclkClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3cFclkClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3cFclkClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3cFclkClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cClkdiv(pub u32);
impl Lpi2cClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Lpi2cClkdiv {
    #[inline(always)]
    fn default() -> Lpi2cClkdiv {
        Lpi2cClkdiv(0)
    }
}
impl core::fmt::Debug for Lpi2cClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpi2cClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cClksel(pub u32);
impl Lpi2cClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Lpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Lpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lpi2cClksel {
    #[inline(always)]
    fn default() -> Lpi2cClksel {
        Lpi2cClksel(0)
    }
}
impl core::fmt::Debug for Lpi2cClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2cClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiClkdiv(pub u32);
impl LpspiClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for LpspiClkdiv {
    #[inline(always)]
    fn default() -> LpspiClkdiv {
        LpspiClkdiv(0)
    }
}
impl core::fmt::Debug for LpspiClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpspiClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiClksel(pub u32);
impl LpspiClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::LpspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::LpspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::LpspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LpspiClksel {
    #[inline(always)]
    fn default() -> LpspiClksel {
        LpspiClksel(0)
    }
}
impl core::fmt::Debug for LpspiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpspiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPTMR0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LptmrClkdiv(pub u32);
impl LptmrClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for LptmrClkdiv {
    #[inline(always)]
    fn default() -> LptmrClkdiv {
        LptmrClkdiv(0)
    }
}
impl core::fmt::Debug for LptmrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LptmrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LptmrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LptmrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPTMR0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LptmrClksel(pub u32);
impl LptmrClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::LptmrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::LptmrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::LptmrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LptmrClksel {
    #[inline(always)]
    fn default() -> LptmrClksel {
        LptmrClksel(0)
    }
}
impl core::fmt::Debug for LptmrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LptmrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LptmrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LptmrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpuartClkdiv(pub u32);
impl LpuartClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for LpuartClkdiv {
    #[inline(always)]
    fn default() -> LpuartClkdiv {
        LpuartClkdiv(0)
    }
}
impl core::fmt::Debug for LpuartClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpuartClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpuartClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpuartClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpuartClksel(pub u32);
impl LpuartClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::LpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::LpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::LpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LpuartClksel {
    #[inline(always)]
    fn default() -> LpuartClksel {
        LpuartClksel(0)
    }
}
impl core::fmt::Debug for LpuartClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpuartClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpuartClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpuartClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "ADCx clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdcClkdiv(pub u32);
impl MrccAdcClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdcClkdiv {
    #[inline(always)]
    fn default() -> MrccAdcClkdiv {
        MrccAdcClkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdcClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdcClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdcClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdcClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADCx clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdcClksel(pub u32);
impl MrccAdcClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::AdcClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::AdcClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::AdcClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdcClksel {
    #[inline(always)]
    fn default() -> MrccAdcClksel {
        MrccAdcClksel(0)
    }
}
impl core::fmt::Debug for MrccAdcClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdcClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdcClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdcClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CLKOUT clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClkdiv(pub u32);
impl MrccClkoutClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
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
#[doc = "CLKOUT clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClksel(pub u32);
impl MrccClkoutClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::ClkoutClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ClkoutClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::ClkoutClkselMux) {
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
#[doc = "DBG_TRACE clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClkdiv(pub u32);
impl MrccDbgTraceClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
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
#[doc = "DBG_TRACE clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClksel(pub u32);
impl MrccDbgTraceClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::DbgTraceClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DbgTraceClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::DbgTraceClkselMux) {
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
#[doc = "Control Automatic Clock Gating 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc0(pub u32);
impl MrccGlbAcc0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
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
            self.smartdma0(),
            self.dma0(),
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
            self.flexpwm0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc1(pub u32);
impl MrccGlbAcc1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0"]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0"]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0"]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Control Automatic Clock Gating 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc2(pub u32);
impl MrccGlbAcc2 {
    #[doc = "RAMA"]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA"]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB"]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC"]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC"]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC"]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for MrccGlbAcc2 {
    #[inline(always)]
    fn default() -> MrccGlbAcc2 {
        MrccGlbAcc2(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc2")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0(pub u32);
impl MrccGlbCc0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
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
            self.smartdma0(),
            self.dma0(),
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
            self.flexpwm0()
        )
    }
}
#[doc = "AHB Clock Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1(pub u32);
impl MrccGlbCc1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TDET0"]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TDET0"]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0"]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0"]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0"]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("tdet0", &self.tdet0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, tdet0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.tdet0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "AHB Clock Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc2(pub u32);
impl MrccGlbCc2 {
    #[doc = "RAMA"]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA"]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB"]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC"]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC"]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC"]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for MrccGlbCc2 {
    #[inline(always)]
    fn default() -> MrccGlbCc2 {
        MrccGlbCc2(0)
    }
}
impl core::fmt::Debug for MrccGlbCc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc2")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
        )
    }
}
#[doc = "Peripheral Reset Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0(pub u32);
impl MrccGlbRst0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.smartdma0(),
            self.dma0(),
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
            self.flexpwm0()
        )
    }
}
#[doc = "Peripheral Reset Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1(pub u32);
impl MrccGlbRst1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("trng0", &self.trng0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, trng0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.trng0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Peripheral Reset Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst2(pub u32);
impl MrccGlbRst2 {
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for MrccGlbRst2 {
    #[inline(always)]
    fn default() -> MrccGlbRst2 {
        MrccGlbRst2(0)
    }
}
impl core::fmt::Debug for MrccGlbRst2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst2")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst2 {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?} }}",
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0()
        )
    }
}
#[doc = "SYSTICK clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClkdiv(pub u32);
impl MrccSystickClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
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
#[doc = "SYSTICK clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClksel(pub u32);
impl MrccSystickClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::SystickClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SystickClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::SystickClkselMux) {
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
#[doc = "OSTIMER0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OstimerClksel(pub u32);
impl OstimerClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::OstimerClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::OstimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::OstimerClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for OstimerClksel {
    #[inline(always)]
    fn default() -> OstimerClksel {
        OstimerClksel(0)
    }
}
impl core::fmt::Debug for OstimerClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OstimerClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OstimerClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OstimerClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbClkdiv(pub u32);
impl UsbClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbClkdiv {
    #[inline(always)]
    fn default() -> UsbClkdiv {
        UsbClkdiv(0)
    }
}
impl core::fmt::Debug for UsbClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "USB0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbClksel(pub u32);
impl UsbClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::UsbClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::UsbClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::UsbClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for UsbClksel {
    #[inline(always)]
    fn default() -> UsbClksel {
        UsbClksel(0)
    }
}
impl core::fmt::Debug for UsbClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "WWDT0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WwdtClkdiv(pub u32);
impl WwdtClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for WwdtClkdiv {
    #[inline(always)]
    fn default() -> WwdtClkdiv {
        WwdtClkdiv(0)
    }
}
impl core::fmt::Debug for WwdtClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WwdtClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WwdtClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WwdtClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
