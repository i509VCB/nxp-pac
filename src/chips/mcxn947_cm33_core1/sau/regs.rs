#[doc = "Security Attribution Unit Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::CtrlEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::CtrlEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "All Non-secure."]
    #[must_use]
    #[inline(always)]
    pub const fn allns(&self) -> super::vals::Allns {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Allns::from_bits(val as u8)
    }
    #[doc = "All Non-secure."]
    #[inline(always)]
    pub const fn set_allns(&mut self, val: super::vals::Allns) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("enable", &self.enable())
            .field("allns", &self.allns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {:?}, allns: {:?} }}",
            self.enable(),
            self.allns()
        )
    }
}
#[doc = "Security Attribution Unit Region Base Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar(pub u32);
impl Rbar {
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[must_use]
    #[inline(always)]
    pub const fn baddr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[inline(always)]
    pub const fn set_baddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        Rbar(0)
    }
}
impl core::fmt::Debug for Rbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbar")
            .field("baddr", &self.baddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rbar {{ baddr: {=u32:?} }}", self.baddr())
    }
}
#[doc = "Security Attribution Unit Region Limit Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlar(pub u32);
impl Rlar {
    #[doc = "Enable. SAU region enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::RlarEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RlarEnable::from_bits(val as u8)
    }
    #[doc = "Enable. SAU region enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::RlarEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[must_use]
    #[inline(always)]
    pub const fn nsc(&self) -> super::vals::Nsc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Nsc::from_bits(val as u8)
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub const fn set_nsc(&mut self, val: super::vals::Nsc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[must_use]
    #[inline(always)]
    pub const fn laddr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub const fn set_laddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rlar {
    #[inline(always)]
    fn default() -> Rlar {
        Rlar(0)
    }
}
impl core::fmt::Debug for Rlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rlar")
            .field("enable", &self.enable())
            .field("nsc", &self.nsc())
            .field("laddr", &self.laddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rlar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rlar {{ enable: {:?}, nsc: {:?}, laddr: {=u32:?} }}",
            self.enable(),
            self.nsc(),
            self.laddr()
        )
    }
}
#[doc = "Security Attribution Unit Region Number Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr(pub u32);
impl Rnr {
    #[doc = "Region number."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Region number."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        Rnr(0)
    }
}
impl core::fmt::Debug for Rnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rnr")
            .field("region", &self.region())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rnr {{ region: {=u8:?} }}", self.region())
    }
}
#[doc = "Secure Fault Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfar(pub u32);
impl Sfar {
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sfar {
    #[inline(always)]
    fn default() -> Sfar {
        Sfar(0)
    }
}
impl core::fmt::Debug for Sfar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sfar")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sfar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sfar {{ address: {=u32:?} }}", self.address())
    }
}
#[doc = "Secure Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfsr(pub u32);
impl Sfsr {
    #[doc = "Invalid entry point."]
    #[must_use]
    #[inline(always)]
    pub const fn invep(&self) -> super::vals::Invep {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Invep::from_bits(val as u8)
    }
    #[doc = "Invalid entry point."]
    #[inline(always)]
    pub const fn set_invep(&mut self, val: super::vals::Invep) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Invalid integrity signature flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invis(&self) -> super::vals::Invis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Invis::from_bits(val as u8)
    }
    #[doc = "Invalid integrity signature flag."]
    #[inline(always)]
    pub const fn set_invis(&mut self, val: super::vals::Invis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Invalid exception return flag."]
    #[must_use]
    #[inline(always)]
    pub const fn inver(&self) -> super::vals::Inver {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Inver::from_bits(val as u8)
    }
    #[doc = "Invalid exception return flag."]
    #[inline(always)]
    pub const fn set_inver(&mut self, val: super::vals::Inver) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Attribution unit violation flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auviol(&self) -> super::vals::Auviol {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Auviol::from_bits(val as u8)
    }
    #[doc = "Attribution unit violation flag."]
    #[inline(always)]
    pub const fn set_auviol(&mut self, val: super::vals::Auviol) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid transition flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invtran(&self) -> super::vals::Invtran {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Invtran::from_bits(val as u8)
    }
    #[doc = "Invalid transition flag."]
    #[inline(always)]
    pub const fn set_invtran(&mut self, val: super::vals::Invtran) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Lazy state preservation error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lsperr(&self) -> super::vals::Lsperr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Lsperr::from_bits(val as u8)
    }
    #[doc = "Lazy state preservation error flag."]
    #[inline(always)]
    pub const fn set_lsperr(&mut self, val: super::vals::Lsperr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure fault address valid."]
    #[must_use]
    #[inline(always)]
    pub const fn sfarvalid(&self) -> super::vals::Sfarvalid {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sfarvalid::from_bits(val as u8)
    }
    #[doc = "Secure fault address valid."]
    #[inline(always)]
    pub const fn set_sfarvalid(&mut self, val: super::vals::Sfarvalid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lazy state error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lserr(&self) -> super::vals::Lserr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Lserr::from_bits(val as u8)
    }
    #[doc = "Lazy state error flag."]
    #[inline(always)]
    pub const fn set_lserr(&mut self, val: super::vals::Lserr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Sfsr {
    #[inline(always)]
    fn default() -> Sfsr {
        Sfsr(0)
    }
}
impl core::fmt::Debug for Sfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sfsr")
            .field("invep", &self.invep())
            .field("invis", &self.invis())
            .field("inver", &self.inver())
            .field("auviol", &self.auviol())
            .field("invtran", &self.invtran())
            .field("lsperr", &self.lsperr())
            .field("sfarvalid", &self.sfarvalid())
            .field("lserr", &self.lserr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sfsr {{ invep: {:?}, invis: {:?}, inver: {:?}, auviol: {:?}, invtran: {:?}, lsperr: {:?}, sfarvalid: {:?}, lserr: {:?} }}",
            self.invep(),
            self.invis(),
            self.inver(),
            self.auviol(),
            self.invtran(),
            self.lsperr(),
            self.sfarvalid(),
            self.lserr()
        )
    }
}
#[doc = "Security Attribution Unit Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u32);
impl Type {
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[must_use]
    #[inline(always)]
    pub const fn sregion(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub const fn set_sregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Type {
    #[inline(always)]
    fn default() -> Type {
        Type(0)
    }
}
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Type")
            .field("sregion", &self.sregion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Type {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Type {{ sregion: {=u8:?} }}", self.sregion())
    }
}
