#[doc = "Policy Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polsel(pub u32);
impl Polsel {
    #[doc = "Policy Select for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_policy(&self) -> super::vals::Reg0Policy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Reg0Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn set_reg0_policy(&mut self, val: super::vals::Reg0Policy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Policy Select for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_policy(&self) -> super::vals::Reg1Policy {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Reg1Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn set_reg1_policy(&mut self, val: super::vals::Reg1Policy) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Policy Select for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn reg02_policy(&self) -> super::vals::Reg02Policy {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Reg02Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn set_reg02_policy(&mut self, val: super::vals::Reg02Policy) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Polsel {
    #[inline(always)]
    fn default() -> Polsel {
        Polsel(0)
    }
}
impl core::fmt::Debug for Polsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polsel")
            .field("reg0_policy", &self.reg0_policy())
            .field("reg1_policy", &self.reg1_policy())
            .field("reg02_policy", &self.reg02_policy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Polsel {{ reg0_policy: {:?}, reg1_policy: {:?}, reg02_policy: {:?} }}",
            self.reg0_policy(),
            self.reg1_policy(),
            self.reg02_policy()
        )
    }
}
#[doc = "Region 0 Top Boundary"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg0Top(pub u32);
impl Reg0Top {
    #[doc = "Upper limit of Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Upper limit of Region 0"]
    #[inline(always)]
    pub const fn set_reg0_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 10usize)) | (((val as u32) & 0x0001_ffff) << 10usize);
    }
}
impl Default for Reg0Top {
    #[inline(always)]
    fn default() -> Reg0Top {
        Reg0Top(0)
    }
}
impl core::fmt::Debug for Reg0Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg0Top")
            .field("reg0_top", &self.reg0_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg0Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reg0Top {{ reg0_top: {=u32:?} }}", self.reg0_top())
    }
}
#[doc = "Region 1 Top Boundary"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1Top(pub u32);
impl Reg1Top {
    #[doc = "Upper limit of Region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Upper limit of Region 1"]
    #[inline(always)]
    pub const fn set_reg1_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 10usize)) | (((val as u32) & 0x0001_ffff) << 10usize);
    }
}
impl Default for Reg1Top {
    #[inline(always)]
    fn default() -> Reg1Top {
        Reg1Top(0)
    }
}
impl core::fmt::Debug for Reg1Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1Top")
            .field("reg1_top", &self.reg1_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reg1Top {{ reg1_top: {=u32:?} }}", self.reg1_top())
    }
}
