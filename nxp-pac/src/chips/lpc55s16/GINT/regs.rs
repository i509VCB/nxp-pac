#[doc = "GPIO grouped interrupt control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> super::vals::INT {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INT::from_bits(val as u8)
    }
    #[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub const fn set_INT(&mut self, val: super::vals::INT) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Combine enabled inputs for group interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn COMB(&self) -> super::vals::COMB {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::COMB::from_bits(val as u8)
    }
    #[doc = "Combine enabled inputs for group interrupt."]
    #[inline(always)]
    pub const fn set_COMB(&mut self, val: super::vals::COMB) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Group interrupt trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG(&self) -> super::vals::TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TRIG::from_bits(val as u8)
    }
    #[doc = "Group interrupt trigger."]
    #[inline(always)]
    pub const fn set_TRIG(&mut self, val: super::vals::TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("INT", &self.INT())
            .field("COMB", &self.COMB())
            .field("TRIG", &self.TRIG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ INT: {:?}, COMB: {:?}, TRIG: {:?} }}",
            self.INT(),
            self.COMB(),
            self.TRIG()
        )
    }
}
#[doc = "GPIO grouped interrupt port 0 enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORT_ENA(pub u32);
impl PORT_ENA {
    #[doc = "Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub const fn set_ENA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PORT_ENA {
    #[inline(always)]
    fn default() -> PORT_ENA {
        PORT_ENA(0)
    }
}
impl core::fmt::Debug for PORT_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_ENA")
            .field("ENA", &self.ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORT_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PORT_ENA {{ ENA: {=u32:?} }}", self.ENA())
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORT_POL(pub u32);
impl PORT_POL {
    #[doc = "Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn POL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub const fn set_POL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PORT_POL {
    #[inline(always)]
    fn default() -> PORT_POL {
        PORT_POL(0)
    }
}
impl core::fmt::Debug for PORT_POL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_POL")
            .field("POL", &self.POL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORT_POL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PORT_POL {{ POL: {=u32:?} }}", self.POL())
    }
}
