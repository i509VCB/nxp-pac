#[doc = "SysTick Calibration Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCalib(pub u32);
impl SystCalib {
    #[doc = "Reload value to use for 10ms timing"]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value to use for 10ms timing"]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact"]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Skew {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Skew) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates whether the device provides an alternative reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Noref {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides an alternative reference clock"]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Noref) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SystCalib {
    #[inline(always)]
    fn default() -> SystCalib {
        SystCalib(0)
    }
}
impl core::fmt::Debug for SystCalib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCalib")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCalib {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystCalib {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "SysTick Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCsr(pub u32);
impl SystCsr {
    #[doc = "Enable/disable systick counter"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable/disable systick counter"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Systick interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tickint(&self) -> super::vals::Tickint {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tickint::from_bits(val as u8)
    }
    #[doc = "Enable Systick interrupt."]
    #[inline(always)]
    pub const fn set_tickint(&mut self, val: super::vals::Tickint) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock source selection."]
    #[must_use]
    #[inline(always)]
    pub const fn clksource(&self) -> super::vals::Clksource {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Clksource::from_bits(val as u8)
    }
    #[doc = "Clock source selection."]
    #[inline(always)]
    pub const fn set_clksource(&mut self, val: super::vals::Clksource) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    #[must_use]
    #[inline(always)]
    pub const fn countflag(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    #[inline(always)]
    pub const fn set_countflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SystCsr {
    #[inline(always)]
    fn default() -> SystCsr {
        SystCsr(0)
    }
}
impl core::fmt::Debug for SystCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCsr")
            .field("enable", &self.enable())
            .field("tickint", &self.tickint())
            .field("clksource", &self.clksource())
            .field("countflag", &self.countflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystCsr {{ enable: {:?}, tickint: {:?}, clksource: {:?}, countflag: {=bool:?} }}",
            self.enable(),
            self.tickint(),
            self.clksource(),
            self.countflag()
        )
    }
}
#[doc = "SysTick Current Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCvr(pub u32);
impl SystCvr {
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[inline(always)]
    pub const fn set_current(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for SystCvr {
    #[inline(always)]
    fn default() -> SystCvr {
        SystCvr(0)
    }
}
impl core::fmt::Debug for SystCvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCvr")
            .field("current", &self.current())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystCvr {{ current: {=u32:?} }}", self.current())
    }
}
#[doc = "SysTick Reload Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystRvr(pub u32);
impl SystRvr {
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[must_use]
    #[inline(always)]
    pub const fn reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    pub const fn set_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for SystRvr {
    #[inline(always)]
    fn default() -> SystRvr {
        SystRvr(0)
    }
}
impl core::fmt::Debug for SystRvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystRvr")
            .field("reload", &self.reload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystRvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystRvr {{ reload: {=u32:?} }}", self.reload())
    }
}
