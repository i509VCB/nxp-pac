#[doc = "GPIO grouped interrupt control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> super::vals::Int {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Int::from_bits(val as u8)
    }
    #[doc = "Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: super::vals::Int) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Combine enabled inputs for group interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn comb(&self) -> super::vals::Comb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Comb::from_bits(val as u8)
    }
    #[doc = "Combine enabled inputs for group interrupt"]
    #[inline(always)]
    pub const fn set_comb(&mut self, val: super::vals::Comb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Group interrupt trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trig(&self) -> super::vals::Trig {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Trig::from_bits(val as u8)
    }
    #[doc = "Group interrupt trigger"]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: super::vals::Trig) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("int", &self.int())
            .field("comb", &self.comb())
            .field("trig", &self.trig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ int: {:?}, comb: {:?}, trig: {:?} }}",
            self.int(),
            self.comb(),
            self.trig()
        )
    }
}
#[doc = "GPIO grouped interrupt port 0 enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortEna(pub u32);
impl PortEna {
    #[doc = "Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ena(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub const fn set_ena(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PortEna {
    #[inline(always)]
    fn default() -> PortEna {
        PortEna(0)
    }
}
impl core::fmt::Debug for PortEna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PortEna").field("ena", &self.ena()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortEna {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PortEna {{ ena: {=u32:?} }}", self.ena())
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortPol(pub u32);
impl PortPol {
    #[doc = "Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PortPol {
    #[inline(always)]
    fn default() -> PortPol {
        PortPol(0)
    }
}
impl core::fmt::Debug for PortPol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PortPol").field("pol", &self.pol()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortPol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PortPol {{ pol: {=u32:?} }}", self.pol())
    }
}
