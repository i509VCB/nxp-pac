#[doc = "Master Priviledge Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpr(pub u32);
impl Mpr {
    #[doc = "Master 3 Priviledge, Buffer, Read, Write Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mprot3(&self) -> super::vals::Mprot3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Mprot3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub const fn set_mprot3(&mut self, val: super::vals::Mprot3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Master 2 Priviledge, Buffer, Read, Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mprot2(&self) -> super::vals::Mprot2 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Mprot2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub const fn set_mprot2(&mut self, val: super::vals::Mprot2) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Master 1 Priviledge, Buffer, Read, Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mprot1(&self) -> super::vals::Mprot1 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Mprot1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub const fn set_mprot1(&mut self, val: super::vals::Mprot1) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Master 0 Priviledge, Buffer, Read, Write Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mprot0(&self) -> super::vals::Mprot0 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Mprot0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub const fn set_mprot0(&mut self, val: super::vals::Mprot0) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Mpr {
    #[inline(always)]
    fn default() -> Mpr {
        Mpr(0)
    }
}
impl core::fmt::Debug for Mpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpr")
            .field("mprot3", &self.mprot3())
            .field("mprot2", &self.mprot2())
            .field("mprot1", &self.mprot1())
            .field("mprot0", &self.mprot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mpr {{ mprot3: {:?}, mprot2: {:?}, mprot1: {:?}, mprot0: {:?} }}",
            self.mprot3(),
            self.mprot2(),
            self.mprot1(),
            self.mprot0()
        )
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opacr(pub u32);
impl Opacr {
    #[doc = "Off-platform Peripheral Access Control 7"]
    #[must_use]
    #[inline(always)]
    pub const fn opac7(&self) -> super::vals::Opac7 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Opac7::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 7"]
    #[inline(always)]
    pub const fn set_opac7(&mut self, val: super::vals::Opac7) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Off-platform Peripheral Access Control 6"]
    #[must_use]
    #[inline(always)]
    pub const fn opac6(&self) -> super::vals::Opac6 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Opac6::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 6"]
    #[inline(always)]
    pub const fn set_opac6(&mut self, val: super::vals::Opac6) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Off-platform Peripheral Access Control 5"]
    #[must_use]
    #[inline(always)]
    pub const fn opac5(&self) -> super::vals::Opac5 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Opac5::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 5"]
    #[inline(always)]
    pub const fn set_opac5(&mut self, val: super::vals::Opac5) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Off-platform Peripheral Access Control 4"]
    #[must_use]
    #[inline(always)]
    pub const fn opac4(&self) -> super::vals::Opac4 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Opac4::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 4"]
    #[inline(always)]
    pub const fn set_opac4(&mut self, val: super::vals::Opac4) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Off-platform Peripheral Access Control 3"]
    #[must_use]
    #[inline(always)]
    pub const fn opac3(&self) -> super::vals::Opac3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Opac3::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 3"]
    #[inline(always)]
    pub const fn set_opac3(&mut self, val: super::vals::Opac3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Off-platform Peripheral Access Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn opac2(&self) -> super::vals::Opac2 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Opac2::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 2"]
    #[inline(always)]
    pub const fn set_opac2(&mut self, val: super::vals::Opac2) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Off-platform Peripheral Access Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn opac1(&self) -> super::vals::Opac1 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Opac1::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 1"]
    #[inline(always)]
    pub const fn set_opac1(&mut self, val: super::vals::Opac1) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Off-platform Peripheral Access Control 0"]
    #[must_use]
    #[inline(always)]
    pub const fn opac0(&self) -> super::vals::Opac0 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Opac0::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 0"]
    #[inline(always)]
    pub const fn set_opac0(&mut self, val: super::vals::Opac0) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Opacr {
    #[inline(always)]
    fn default() -> Opacr {
        Opacr(0)
    }
}
impl core::fmt::Debug for Opacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opacr")
            .field("opac7", &self.opac7())
            .field("opac6", &self.opac6())
            .field("opac5", &self.opac5())
            .field("opac4", &self.opac4())
            .field("opac3", &self.opac3())
            .field("opac2", &self.opac2())
            .field("opac1", &self.opac1())
            .field("opac0", &self.opac0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opacr {{ opac7: {:?}, opac6: {:?}, opac5: {:?}, opac4: {:?}, opac3: {:?}, opac2: {:?}, opac1: {:?}, opac0: {:?} }}",
            self.opac7(),
            self.opac6(),
            self.opac5(),
            self.opac4(),
            self.opac3(),
            self.opac2(),
            self.opac1(),
            self.opac0()
        )
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opacr1(pub u32);
impl Opacr1 {
    #[doc = "Off-platform Peripheral Access Control 15"]
    #[must_use]
    #[inline(always)]
    pub const fn opac15(&self) -> super::vals::Opac15 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Opac15::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 15"]
    #[inline(always)]
    pub const fn set_opac15(&mut self, val: super::vals::Opac15) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Off-platform Peripheral Access Control 14"]
    #[must_use]
    #[inline(always)]
    pub const fn opac14(&self) -> super::vals::Opac14 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Opac14::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 14"]
    #[inline(always)]
    pub const fn set_opac14(&mut self, val: super::vals::Opac14) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Off-platform Peripheral Access Control 13"]
    #[must_use]
    #[inline(always)]
    pub const fn opac13(&self) -> super::vals::Opac13 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Opac13::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 13"]
    #[inline(always)]
    pub const fn set_opac13(&mut self, val: super::vals::Opac13) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Off-platform Peripheral Access Control 12"]
    #[must_use]
    #[inline(always)]
    pub const fn opac12(&self) -> super::vals::Opac12 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Opac12::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 12"]
    #[inline(always)]
    pub const fn set_opac12(&mut self, val: super::vals::Opac12) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Off-platform Peripheral Access Control 11"]
    #[must_use]
    #[inline(always)]
    pub const fn opac11(&self) -> super::vals::Opac11 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Opac11::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 11"]
    #[inline(always)]
    pub const fn set_opac11(&mut self, val: super::vals::Opac11) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Off-platform Peripheral Access Control 10"]
    #[must_use]
    #[inline(always)]
    pub const fn opac10(&self) -> super::vals::Opac10 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Opac10::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 10"]
    #[inline(always)]
    pub const fn set_opac10(&mut self, val: super::vals::Opac10) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Off-platform Peripheral Access Control 9"]
    #[must_use]
    #[inline(always)]
    pub const fn opac9(&self) -> super::vals::Opac9 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Opac9::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 9"]
    #[inline(always)]
    pub const fn set_opac9(&mut self, val: super::vals::Opac9) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Off-platform Peripheral Access Control 8"]
    #[must_use]
    #[inline(always)]
    pub const fn opac8(&self) -> super::vals::Opac8 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Opac8::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 8"]
    #[inline(always)]
    pub const fn set_opac8(&mut self, val: super::vals::Opac8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Opacr1 {
    #[inline(always)]
    fn default() -> Opacr1 {
        Opacr1(0)
    }
}
impl core::fmt::Debug for Opacr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opacr1")
            .field("opac15", &self.opac15())
            .field("opac14", &self.opac14())
            .field("opac13", &self.opac13())
            .field("opac12", &self.opac12())
            .field("opac11", &self.opac11())
            .field("opac10", &self.opac10())
            .field("opac9", &self.opac9())
            .field("opac8", &self.opac8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opacr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opacr1 {{ opac15: {:?}, opac14: {:?}, opac13: {:?}, opac12: {:?}, opac11: {:?}, opac10: {:?}, opac9: {:?}, opac8: {:?} }}",
            self.opac15(),
            self.opac14(),
            self.opac13(),
            self.opac12(),
            self.opac11(),
            self.opac10(),
            self.opac9(),
            self.opac8()
        )
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opacr2(pub u32);
impl Opacr2 {
    #[doc = "Off-platform Peripheral Access Control 23"]
    #[must_use]
    #[inline(always)]
    pub const fn opac23(&self) -> super::vals::Opac23 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Opac23::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 23"]
    #[inline(always)]
    pub const fn set_opac23(&mut self, val: super::vals::Opac23) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Off-platform Peripheral Access Control 22"]
    #[must_use]
    #[inline(always)]
    pub const fn opac22(&self) -> super::vals::Opac22 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Opac22::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 22"]
    #[inline(always)]
    pub const fn set_opac22(&mut self, val: super::vals::Opac22) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Off-platform Peripheral Access Control 21"]
    #[must_use]
    #[inline(always)]
    pub const fn opac21(&self) -> super::vals::Opac21 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Opac21::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 21"]
    #[inline(always)]
    pub const fn set_opac21(&mut self, val: super::vals::Opac21) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Off-platform Peripheral Access Control 20"]
    #[must_use]
    #[inline(always)]
    pub const fn opac20(&self) -> super::vals::Opac20 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Opac20::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 20"]
    #[inline(always)]
    pub const fn set_opac20(&mut self, val: super::vals::Opac20) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Off-platform Peripheral Access Control 19"]
    #[must_use]
    #[inline(always)]
    pub const fn opac19(&self) -> super::vals::Opac19 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Opac19::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 19"]
    #[inline(always)]
    pub const fn set_opac19(&mut self, val: super::vals::Opac19) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Off-platform Peripheral Access Control 18"]
    #[must_use]
    #[inline(always)]
    pub const fn opac18(&self) -> super::vals::Opac18 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Opac18::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 18"]
    #[inline(always)]
    pub const fn set_opac18(&mut self, val: super::vals::Opac18) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Off-platform Peripheral Access Control 17"]
    #[must_use]
    #[inline(always)]
    pub const fn opac17(&self) -> super::vals::Opac17 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Opac17::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 17"]
    #[inline(always)]
    pub const fn set_opac17(&mut self, val: super::vals::Opac17) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Off-platform Peripheral Access Control 16"]
    #[must_use]
    #[inline(always)]
    pub const fn opac16(&self) -> super::vals::Opac16 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Opac16::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 16"]
    #[inline(always)]
    pub const fn set_opac16(&mut self, val: super::vals::Opac16) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Opacr2 {
    #[inline(always)]
    fn default() -> Opacr2 {
        Opacr2(0)
    }
}
impl core::fmt::Debug for Opacr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opacr2")
            .field("opac23", &self.opac23())
            .field("opac22", &self.opac22())
            .field("opac21", &self.opac21())
            .field("opac20", &self.opac20())
            .field("opac19", &self.opac19())
            .field("opac18", &self.opac18())
            .field("opac17", &self.opac17())
            .field("opac16", &self.opac16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opacr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opacr2 {{ opac23: {:?}, opac22: {:?}, opac21: {:?}, opac20: {:?}, opac19: {:?}, opac18: {:?}, opac17: {:?}, opac16: {:?} }}",
            self.opac23(),
            self.opac22(),
            self.opac21(),
            self.opac20(),
            self.opac19(),
            self.opac18(),
            self.opac17(),
            self.opac16()
        )
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opacr3(pub u32);
impl Opacr3 {
    #[doc = "Off-platform Peripheral Access Control 31"]
    #[must_use]
    #[inline(always)]
    pub const fn opac31(&self) -> super::vals::Opac31 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Opac31::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 31"]
    #[inline(always)]
    pub const fn set_opac31(&mut self, val: super::vals::Opac31) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Off-platform Peripheral Access Control 30"]
    #[must_use]
    #[inline(always)]
    pub const fn opac30(&self) -> super::vals::Opac30 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Opac30::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 30"]
    #[inline(always)]
    pub const fn set_opac30(&mut self, val: super::vals::Opac30) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Off-platform Peripheral Access Control 29"]
    #[must_use]
    #[inline(always)]
    pub const fn opac29(&self) -> super::vals::Opac29 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Opac29::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 29"]
    #[inline(always)]
    pub const fn set_opac29(&mut self, val: super::vals::Opac29) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Off-platform Peripheral Access Control 28"]
    #[must_use]
    #[inline(always)]
    pub const fn opac28(&self) -> super::vals::Opac28 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Opac28::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 28"]
    #[inline(always)]
    pub const fn set_opac28(&mut self, val: super::vals::Opac28) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Off-platform Peripheral Access Control 27"]
    #[must_use]
    #[inline(always)]
    pub const fn opac27(&self) -> super::vals::Opac27 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Opac27::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 27"]
    #[inline(always)]
    pub const fn set_opac27(&mut self, val: super::vals::Opac27) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Off-platform Peripheral Access Control 26"]
    #[must_use]
    #[inline(always)]
    pub const fn opac26(&self) -> super::vals::Opac26 {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Opac26::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 26"]
    #[inline(always)]
    pub const fn set_opac26(&mut self, val: super::vals::Opac26) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Off-platform Peripheral Access Control 25"]
    #[must_use]
    #[inline(always)]
    pub const fn opac25(&self) -> super::vals::Opac25 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Opac25::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 25"]
    #[inline(always)]
    pub const fn set_opac25(&mut self, val: super::vals::Opac25) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Off-platform Peripheral Access Control 24"]
    #[must_use]
    #[inline(always)]
    pub const fn opac24(&self) -> super::vals::Opac24 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Opac24::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 24"]
    #[inline(always)]
    pub const fn set_opac24(&mut self, val: super::vals::Opac24) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Opacr3 {
    #[inline(always)]
    fn default() -> Opacr3 {
        Opacr3(0)
    }
}
impl core::fmt::Debug for Opacr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opacr3")
            .field("opac31", &self.opac31())
            .field("opac30", &self.opac30())
            .field("opac29", &self.opac29())
            .field("opac28", &self.opac28())
            .field("opac27", &self.opac27())
            .field("opac26", &self.opac26())
            .field("opac25", &self.opac25())
            .field("opac24", &self.opac24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opacr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opacr3 {{ opac31: {:?}, opac30: {:?}, opac29: {:?}, opac28: {:?}, opac27: {:?}, opac26: {:?}, opac25: {:?}, opac24: {:?} }}",
            self.opac31(),
            self.opac30(),
            self.opac29(),
            self.opac28(),
            self.opac27(),
            self.opac26(),
            self.opac25(),
            self.opac24()
        )
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opacr4(pub u32);
impl Opacr4 {
    #[doc = "Off-platform Peripheral Access Control 33"]
    #[must_use]
    #[inline(always)]
    pub const fn opac33(&self) -> super::vals::Opac33 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Opac33::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 33"]
    #[inline(always)]
    pub const fn set_opac33(&mut self, val: super::vals::Opac33) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Off-platform Peripheral Access Control 32"]
    #[must_use]
    #[inline(always)]
    pub const fn opac32(&self) -> super::vals::Opac32 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Opac32::from_bits(val as u8)
    }
    #[doc = "Off-platform Peripheral Access Control 32"]
    #[inline(always)]
    pub const fn set_opac32(&mut self, val: super::vals::Opac32) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Opacr4 {
    #[inline(always)]
    fn default() -> Opacr4 {
        Opacr4(0)
    }
}
impl core::fmt::Debug for Opacr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opacr4")
            .field("opac33", &self.opac33())
            .field("opac32", &self.opac32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opacr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opacr4 {{ opac33: {:?}, opac32: {:?} }}",
            self.opac33(),
            self.opac32()
        )
    }
}
