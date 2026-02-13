#[doc = "Cache control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Cache enable"]
    #[must_use]
    #[inline(always)]
    pub const fn encache(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cache enable"]
    #[inline(always)]
    pub const fn set_encache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Write Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn enwrbuf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Write Buffer"]
    #[inline(always)]
    pub const fn set_enwrbuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invalidate Way 0"]
    #[must_use]
    #[inline(always)]
    pub const fn invw0(&self) -> super::vals::Invw0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Invw0::from_bits(val as u8)
    }
    #[doc = "Invalidate Way 0"]
    #[inline(always)]
    pub const fn set_invw0(&mut self, val: super::vals::Invw0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Push Way 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pushw0(&self) -> super::vals::Pushw0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pushw0::from_bits(val as u8)
    }
    #[doc = "Push Way 0"]
    #[inline(always)]
    pub const fn set_pushw0(&mut self, val: super::vals::Pushw0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Invalidate Way 1"]
    #[must_use]
    #[inline(always)]
    pub const fn invw1(&self) -> super::vals::Invw1 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Invw1::from_bits(val as u8)
    }
    #[doc = "Invalidate Way 1"]
    #[inline(always)]
    pub const fn set_invw1(&mut self, val: super::vals::Invw1) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Push Way 1"]
    #[must_use]
    #[inline(always)]
    pub const fn pushw1(&self) -> super::vals::Pushw1 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pushw1::from_bits(val as u8)
    }
    #[doc = "Push Way 1"]
    #[inline(always)]
    pub const fn set_pushw1(&mut self, val: super::vals::Pushw1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Initiate Cache Command"]
    #[must_use]
    #[inline(always)]
    pub const fn go(&self) -> super::vals::Go {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Go::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Command"]
    #[inline(always)]
    pub const fn set_go(&mut self, val: super::vals::Go) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("encache", &self.encache())
            .field("enwrbuf", &self.enwrbuf())
            .field("invw0", &self.invw0())
            .field("pushw0", &self.pushw0())
            .field("invw1", &self.invw1())
            .field("pushw1", &self.pushw1())
            .field("go", &self.go())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ encache: {=bool:?}, enwrbuf: {=bool:?}, invw0: {:?}, pushw0: {:?}, invw1: {:?}, pushw1: {:?}, go: {:?} }}",
            self.encache(),
            self.enwrbuf(),
            self.invw0(),
            self.pushw0(),
            self.invw1(),
            self.pushw1(),
            self.go()
        )
    }
}
#[doc = "Cache read/write value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccvr(pub u32);
impl Ccvr {
    #[doc = "Cache read/write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cache read/write Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ccvr {
    #[inline(always)]
    fn default() -> Ccvr {
        Ccvr(0)
    }
}
impl core::fmt::Debug for Ccvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccvr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccvr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Cache line control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clcr(pub u32);
impl Clcr {
    #[doc = "Initiate Cache Line Command"]
    #[must_use]
    #[inline(always)]
    pub const fn lgo(&self) -> super::vals::ClcrLgo {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClcrLgo::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Line Command"]
    #[inline(always)]
    pub const fn set_lgo(&mut self, val: super::vals::ClcrLgo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Cache address"]
    #[must_use]
    #[inline(always)]
    pub const fn cacheaddr(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x0fff;
        val as u16
    }
    #[doc = "Cache address"]
    #[inline(always)]
    pub const fn set_cacheaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 2usize)) | (((val as u32) & 0x0fff) << 2usize);
    }
    #[doc = "Way select"]
    #[must_use]
    #[inline(always)]
    pub const fn wsel(&self) -> super::vals::Wsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Wsel::from_bits(val as u8)
    }
    #[doc = "Way select"]
    #[inline(always)]
    pub const fn set_wsel(&mut self, val: super::vals::Wsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Tag/Data Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tdsel(&self) -> super::vals::Tdsel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tdsel::from_bits(val as u8)
    }
    #[doc = "Tag/Data Select"]
    #[inline(always)]
    pub const fn set_tdsel(&mut self, val: super::vals::Tdsel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Line Command Initial Valid Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lcivb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Line Command Initial Valid Bit"]
    #[inline(always)]
    pub const fn set_lcivb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Line Command Initial Modified Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lcimb(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Line Command Initial Modified Bit"]
    #[inline(always)]
    pub const fn set_lcimb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Line Command Way"]
    #[must_use]
    #[inline(always)]
    pub const fn lcway(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Line Command Way"]
    #[inline(always)]
    pub const fn set_lcway(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Line Command"]
    #[must_use]
    #[inline(always)]
    pub const fn lcmd(&self) -> super::vals::Lcmd {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Lcmd::from_bits(val as u8)
    }
    #[doc = "Line Command"]
    #[inline(always)]
    pub const fn set_lcmd(&mut self, val: super::vals::Lcmd) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Line Address Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ladsel(&self) -> super::vals::Ladsel {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ladsel::from_bits(val as u8)
    }
    #[doc = "Line Address Select"]
    #[inline(always)]
    pub const fn set_ladsel(&mut self, val: super::vals::Ladsel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Line access type"]
    #[must_use]
    #[inline(always)]
    pub const fn lacc(&self) -> super::vals::Lacc {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Lacc::from_bits(val as u8)
    }
    #[doc = "Line access type"]
    #[inline(always)]
    pub const fn set_lacc(&mut self, val: super::vals::Lacc) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Clcr {
    #[inline(always)]
    fn default() -> Clcr {
        Clcr(0)
    }
}
impl core::fmt::Debug for Clcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clcr")
            .field("lgo", &self.lgo())
            .field("cacheaddr", &self.cacheaddr())
            .field("wsel", &self.wsel())
            .field("tdsel", &self.tdsel())
            .field("lcivb", &self.lcivb())
            .field("lcimb", &self.lcimb())
            .field("lcway", &self.lcway())
            .field("lcmd", &self.lcmd())
            .field("ladsel", &self.ladsel())
            .field("lacc", &self.lacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clcr {{ lgo: {:?}, cacheaddr: {=u16:?}, wsel: {:?}, tdsel: {:?}, lcivb: {=bool:?}, lcimb: {=bool:?}, lcway: {=bool:?}, lcmd: {:?}, ladsel: {:?}, lacc: {:?} }}",
            self.lgo(),
            self.cacheaddr(),
            self.wsel(),
            self.tdsel(),
            self.lcivb(),
            self.lcimb(),
            self.lcway(),
            self.lcmd(),
            self.ladsel(),
            self.lacc()
        )
    }
}
#[doc = "Cache search address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csar(pub u32);
impl Csar {
    #[doc = "Initiate Cache Line Command"]
    #[must_use]
    #[inline(always)]
    pub const fn lgo(&self) -> super::vals::CsarLgo {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsarLgo::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Line Command"]
    #[inline(always)]
    pub const fn set_lgo(&mut self, val: super::vals::CsarLgo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Physical Address"]
    #[must_use]
    #[inline(always)]
    pub const fn phyaddr27_1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Physical Address"]
    #[inline(always)]
    pub const fn set_phyaddr27_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 1usize)) | (((val as u32) & 0x07ff_ffff) << 1usize);
    }
    #[doc = "Physical Address"]
    #[must_use]
    #[inline(always)]
    pub const fn phyaddr31_29(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Physical Address"]
    #[inline(always)]
    pub const fn set_phyaddr31_29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Csar {
    #[inline(always)]
    fn default() -> Csar {
        Csar(0)
    }
}
impl core::fmt::Debug for Csar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csar")
            .field("lgo", &self.lgo())
            .field("phyaddr27_1", &self.phyaddr27_1())
            .field("phyaddr31_29", &self.phyaddr31_29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csar {{ lgo: {:?}, phyaddr27_1: {=u32:?}, phyaddr31_29: {=u8:?} }}",
            self.lgo(),
            self.phyaddr27_1(),
            self.phyaddr31_29()
        )
    }
}
