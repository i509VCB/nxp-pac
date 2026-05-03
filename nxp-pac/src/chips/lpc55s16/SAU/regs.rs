#[doc = "Security Attribution Unit Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "All Non-secure."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLNS(&self) -> super::vals::ALLNS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ALLNS::from_bits(val as u8)
    }
    #[doc = "All Non-secure."]
    #[inline(always)]
    pub const fn set_ALLNS(&mut self, val: super::vals::ALLNS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("ENABLE", &self.ENABLE())
            .field("ALLNS", &self.ALLNS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ENABLE: {=bool:?}, ALLNS: {:?} }}",
            self.ENABLE(),
            self.ALLNS()
        )
    }
}
#[doc = "Security Attribution Unit Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RBAR(pub u32);
impl RBAR {
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[must_use]
    #[inline(always)]
    pub const fn BADDR(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[inline(always)]
    pub const fn set_BADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RBAR {
    #[inline(always)]
    fn default() -> RBAR {
        RBAR(0)
    }
}
impl core::fmt::Debug for RBAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR")
            .field("BADDR", &self.BADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RBAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RBAR {{ BADDR: {=u32:?} }}", self.BADDR())
    }
}
#[doc = "Security Attribution Unit Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RLAR(pub u32);
impl RLAR {
    #[doc = "Enable. SAU region enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> super::vals::RLAR_ENABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RLAR_ENABLE::from_bits(val as u8)
    }
    #[doc = "Enable. SAU region enable."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: super::vals::RLAR_ENABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[must_use]
    #[inline(always)]
    pub const fn NSC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub const fn set_NSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[must_use]
    #[inline(always)]
    pub const fn LADDR(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub const fn set_LADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RLAR {
    #[inline(always)]
    fn default() -> RLAR {
        RLAR(0)
    }
}
impl core::fmt::Debug for RLAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLAR")
            .field("ENABLE", &self.ENABLE())
            .field("NSC", &self.NSC())
            .field("LADDR", &self.LADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RLAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RLAR {{ ENABLE: {:?}, NSC: {=bool:?}, LADDR: {=u32:?} }}",
            self.ENABLE(),
            self.NSC(),
            self.LADDR()
        )
    }
}
#[doc = "Security Attribution Unit Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RNR(pub u32);
impl RNR {
    #[doc = "Region number."]
    #[must_use]
    #[inline(always)]
    pub const fn REGION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Region number."]
    #[inline(always)]
    pub const fn set_REGION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RNR {
    #[inline(always)]
    fn default() -> RNR {
        RNR(0)
    }
}
impl core::fmt::Debug for RNR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNR")
            .field("REGION", &self.REGION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RNR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RNR {{ REGION: {=u8:?} }}", self.REGION())
    }
}
#[doc = "Secure Fault Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SFAR(pub u32);
impl SFAR {
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SFAR {
    #[inline(always)]
    fn default() -> SFAR {
        SFAR(0)
    }
}
impl core::fmt::Debug for SFAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFAR")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SFAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SFAR {{ ADDRESS: {=u32:?} }}", self.ADDRESS())
    }
}
#[doc = "Secure Fault Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SFSR(pub u32);
impl SFSR {
    #[doc = "Invalid entry point."]
    #[must_use]
    #[inline(always)]
    pub const fn INVEP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid entry point."]
    #[inline(always)]
    pub const fn set_INVEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Invalid integrity signature flag."]
    #[must_use]
    #[inline(always)]
    pub const fn INVIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid integrity signature flag."]
    #[inline(always)]
    pub const fn set_INVIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invalid exception return flag."]
    #[must_use]
    #[inline(always)]
    pub const fn INVER(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid exception return flag."]
    #[inline(always)]
    pub const fn set_INVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Attribution unit violation flag."]
    #[must_use]
    #[inline(always)]
    pub const fn AUVIOL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Attribution unit violation flag."]
    #[inline(always)]
    pub const fn set_AUVIOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid transition flag."]
    #[must_use]
    #[inline(always)]
    pub const fn INVTRAN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid transition flag."]
    #[inline(always)]
    pub const fn set_INVTRAN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Lazy state preservation error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LSPERR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lazy state preservation error flag."]
    #[inline(always)]
    pub const fn set_LSPERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure fault address valid."]
    #[must_use]
    #[inline(always)]
    pub const fn SFARVALID(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Secure fault address valid."]
    #[inline(always)]
    pub const fn set_SFARVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lazy state error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LSERR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Lazy state error flag."]
    #[inline(always)]
    pub const fn set_LSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SFSR {
    #[inline(always)]
    fn default() -> SFSR {
        SFSR(0)
    }
}
impl core::fmt::Debug for SFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR")
            .field("INVEP", &self.INVEP())
            .field("INVIS", &self.INVIS())
            .field("INVER", &self.INVER())
            .field("AUVIOL", &self.AUVIOL())
            .field("INVTRAN", &self.INVTRAN())
            .field("LSPERR", &self.LSPERR())
            .field("SFARVALID", &self.SFARVALID())
            .field("LSERR", &self.LSERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SFSR {{ INVEP: {=bool:?}, INVIS: {=bool:?}, INVER: {=bool:?}, AUVIOL: {=bool:?}, INVTRAN: {=bool:?}, LSPERR: {=bool:?}, SFARVALID: {=bool:?}, LSERR: {=bool:?} }}",
            self.INVEP(),
            self.INVIS(),
            self.INVER(),
            self.AUVIOL(),
            self.INVTRAN(),
            self.LSPERR(),
            self.SFARVALID(),
            self.LSERR()
        )
    }
}
#[doc = "Security Attribution Unit Type Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TYPE(pub u32);
impl TYPE {
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[must_use]
    #[inline(always)]
    pub const fn SREGION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub const fn set_SREGION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TYPE {
    #[inline(always)]
    fn default() -> TYPE {
        TYPE(0)
    }
}
impl core::fmt::Debug for TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TYPE")
            .field("SREGION", &self.SREGION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TYPE {{ SREGION: {=u8:?} }}", self.SREGION())
    }
}
