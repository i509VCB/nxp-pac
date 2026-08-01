#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CACHE64_CTRL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache64Ctrl {
    ptr: *mut u8,
}
unsafe impl Send for Cache64Ctrl {}
unsafe impl Sync for Cache64Ctrl {}
impl Cache64Ctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cache Control."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::pac::common::Reg<Ccr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Cache Line Control."]
    #[inline(always)]
    pub const fn clcr(self) -> crate::pac::common::Reg<Clcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Cache Search Address."]
    #[inline(always)]
    pub const fn csar(self) -> crate::pac::common::Reg<Csar, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Cache Read/Write Value."]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::pac::common::Reg<Ccvr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
}
#[doc = "Cache Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Cache Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn encache(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cache Enable."]
    #[inline(always)]
    pub const fn set_encache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Write Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn enwrbuf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Write Buffer."]
    #[inline(always)]
    pub const fn set_enwrbuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force Write Through Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn frcwt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force Write Through Mode."]
    #[inline(always)]
    pub const fn set_frcwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Forces No Allocation On Cache Misses."]
    #[must_use]
    #[inline(always)]
    pub const fn frcnoallc(&self) -> Frcnoallc {
        let val = (self.0 >> 3usize) & 0x01;
        Frcnoallc::from_bits(val as u8)
    }
    #[doc = "Forces No Allocation On Cache Misses."]
    #[inline(always)]
    pub const fn set_frcnoallc(&mut self, val: Frcnoallc) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalidate Way 0."]
    #[must_use]
    #[inline(always)]
    pub const fn invw0(&self) -> Invw0 {
        let val = (self.0 >> 24usize) & 0x01;
        Invw0::from_bits(val as u8)
    }
    #[doc = "Invalidate Way 0."]
    #[inline(always)]
    pub const fn set_invw0(&mut self, val: Invw0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Push Way 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pushw0(&self) -> Pushw0 {
        let val = (self.0 >> 25usize) & 0x01;
        Pushw0::from_bits(val as u8)
    }
    #[doc = "Push Way 0."]
    #[inline(always)]
    pub const fn set_pushw0(&mut self, val: Pushw0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Invalidate Way 1."]
    #[must_use]
    #[inline(always)]
    pub const fn invw1(&self) -> Invw1 {
        let val = (self.0 >> 26usize) & 0x01;
        Invw1::from_bits(val as u8)
    }
    #[doc = "Invalidate Way 1."]
    #[inline(always)]
    pub const fn set_invw1(&mut self, val: Invw1) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Push Way 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pushw1(&self) -> Pushw1 {
        let val = (self.0 >> 27usize) & 0x01;
        Pushw1::from_bits(val as u8)
    }
    #[doc = "Push Way 1."]
    #[inline(always)]
    pub const fn set_pushw1(&mut self, val: Pushw1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Initiate Cache Command."]
    #[must_use]
    #[inline(always)]
    pub const fn go(&self) -> Go {
        let val = (self.0 >> 31usize) & 0x01;
        Go::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Command."]
    #[inline(always)]
    pub const fn set_go(&mut self, val: Go) {
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
            .field("frcwt", &self.frcwt())
            .field("frcnoallc", &self.frcnoallc())
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
            "Ccr {{ encache: {=bool:?}, enwrbuf: {=bool:?}, frcwt: {=bool:?}, frcnoallc: {:?}, invw0: {:?}, pushw0: {:?}, invw1: {:?}, pushw1: {:?}, go: {:?} }}",
            self.encache(),
            self.enwrbuf(),
            self.frcwt(),
            self.frcnoallc(),
            self.invw0(),
            self.pushw0(),
            self.invw1(),
            self.pushw1(),
            self.go()
        )
    }
}
#[doc = "Cache Read/Write Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccvr(pub u32);
impl Ccvr {
    #[doc = "Cache Read/Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cache Read/Write Data."]
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
#[doc = "Cache Line Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clcr(pub u32);
impl Clcr {
    #[doc = "Initiate Cache Line Command."]
    #[must_use]
    #[inline(always)]
    pub const fn lgo(&self) -> ClcrLgo {
        let val = (self.0 >> 0usize) & 0x01;
        ClcrLgo::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Line Command."]
    #[inline(always)]
    pub const fn set_lgo(&mut self, val: ClcrLgo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Cache Address."]
    #[must_use]
    #[inline(always)]
    pub const fn cacheaddr(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x07ff;
        val as u16
    }
    #[doc = "Cache Address."]
    #[inline(always)]
    pub const fn set_cacheaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 2usize)) | (((val as u32) & 0x07ff) << 2usize);
    }
    #[doc = "Way Select."]
    #[must_use]
    #[inline(always)]
    pub const fn wsel(&self) -> Wsel {
        let val = (self.0 >> 14usize) & 0x01;
        Wsel::from_bits(val as u8)
    }
    #[doc = "Way Select."]
    #[inline(always)]
    pub const fn set_wsel(&mut self, val: Wsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Tag Or Data Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tdsel(&self) -> Tdsel {
        let val = (self.0 >> 16usize) & 0x01;
        Tdsel::from_bits(val as u8)
    }
    #[doc = "Tag Or Data Select."]
    #[inline(always)]
    pub const fn set_tdsel(&mut self, val: Tdsel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Line Command Initial Valid Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lcivb(&self) -> Lcivb {
        let val = (self.0 >> 20usize) & 0x01;
        Lcivb::from_bits(val as u8)
    }
    #[doc = "Line Command Initial Valid Bit."]
    #[inline(always)]
    pub const fn set_lcivb(&mut self, val: Lcivb) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Line Command Initial Modified Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lcimb(&self) -> Lcimb {
        let val = (self.0 >> 21usize) & 0x01;
        Lcimb::from_bits(val as u8)
    }
    #[doc = "Line Command Initial Modified Bit."]
    #[inline(always)]
    pub const fn set_lcimb(&mut self, val: Lcimb) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Line Command Way."]
    #[must_use]
    #[inline(always)]
    pub const fn lcway(&self) -> Lcway {
        let val = (self.0 >> 22usize) & 0x01;
        Lcway::from_bits(val as u8)
    }
    #[doc = "Line Command Way."]
    #[inline(always)]
    pub const fn set_lcway(&mut self, val: Lcway) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Line Command."]
    #[must_use]
    #[inline(always)]
    pub const fn lcmd(&self) -> Lcmd {
        let val = (self.0 >> 24usize) & 0x03;
        Lcmd::from_bits(val as u8)
    }
    #[doc = "Line Command."]
    #[inline(always)]
    pub const fn set_lcmd(&mut self, val: Lcmd) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Line Address Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ladsel(&self) -> Ladsel {
        let val = (self.0 >> 26usize) & 0x01;
        Ladsel::from_bits(val as u8)
    }
    #[doc = "Line Address Select."]
    #[inline(always)]
    pub const fn set_ladsel(&mut self, val: Ladsel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Line Access Type."]
    #[must_use]
    #[inline(always)]
    pub const fn lacc(&self) -> Lacc {
        let val = (self.0 >> 27usize) & 0x01;
        Lacc::from_bits(val as u8)
    }
    #[doc = "Line Access Type."]
    #[inline(always)]
    pub const fn set_lacc(&mut self, val: Lacc) {
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
            "Clcr {{ lgo: {:?}, cacheaddr: {=u16:?}, wsel: {:?}, tdsel: {:?}, lcivb: {:?}, lcimb: {:?}, lcway: {:?}, lcmd: {:?}, ladsel: {:?}, lacc: {:?} }}",
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
#[doc = "Cache Search Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csar(pub u32);
impl Csar {
    #[doc = "Initiate Cache Line Command."]
    #[must_use]
    #[inline(always)]
    pub const fn lgo(&self) -> CsarLgo {
        let val = (self.0 >> 0usize) & 0x01;
        CsarLgo::from_bits(val as u8)
    }
    #[doc = "Initiate Cache Line Command."]
    #[inline(always)]
    pub const fn set_lgo(&mut self, val: CsarLgo) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Physical Address."]
    #[must_use]
    #[inline(always)]
    pub const fn phyaddr(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Physical Address."]
    #[inline(always)]
    pub const fn set_phyaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
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
            .field("phyaddr", &self.phyaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csar {{ lgo: {:?}, phyaddr: {=u32:?} }}",
            self.lgo(),
            self.phyaddr()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClcrLgo {
    #[doc = "Write: no effect; Read: no line command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiate line command; Read: line command active."]
    InitCmd = 0x01,
}
impl ClcrLgo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClcrLgo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClcrLgo {
    #[inline(always)]
    fn from(val: u8) -> ClcrLgo {
        ClcrLgo::from_bits(val)
    }
}
impl From<ClcrLgo> for u8 {
    #[inline(always)]
    fn from(val: ClcrLgo) -> u8 {
        ClcrLgo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsarLgo {
    #[doc = "Write: no effect; Read: no line command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiate line command; Read: line command active."]
    InitCmd = 0x01,
}
impl CsarLgo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsarLgo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsarLgo {
    #[inline(always)]
    fn from(val: u8) -> CsarLgo {
        CsarLgo::from_bits(val)
    }
}
impl From<CsarLgo> for u8 {
    #[inline(always)]
    fn from(val: CsarLgo) -> u8 {
        CsarLgo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frcnoallc {
    #[doc = "Allocation on cache misses."]
    Allconcache = 0x0,
    #[doc = "Forces no allocation on cache misses (FRCWT must be asserted)."]
    Frcno = 0x01,
}
impl Frcnoallc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frcnoallc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frcnoallc {
    #[inline(always)]
    fn from(val: u8) -> Frcnoallc {
        Frcnoallc::from_bits(val)
    }
}
impl From<Frcnoallc> for u8 {
    #[inline(always)]
    fn from(val: Frcnoallc) -> u8 {
        Frcnoallc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Go {
    #[doc = "Write: no effect; Read: no cache command active."]
    NoEffect = 0x0,
    #[doc = "Write: initiates cache command; Read: cache command active."]
    InitCmd = 0x01,
}
impl Go {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Go {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Go {
    #[inline(always)]
    fn from(val: u8) -> Go {
        Go::from_bits(val)
    }
}
impl From<Go> for u8 {
    #[inline(always)]
    fn from(val: Go) -> u8 {
        Go::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invw0 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "Invalidates all lines in way 0."]
    Invw0 = 0x01,
}
impl Invw0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invw0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invw0 {
    #[inline(always)]
    fn from(val: u8) -> Invw0 {
        Invw0::from_bits(val)
    }
}
impl From<Invw0> for u8 {
    #[inline(always)]
    fn from(val: Invw0) -> u8 {
        Invw0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invw1 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "Invalidates all lines in way 1."]
    Invw1 = 0x01,
}
impl Invw1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invw1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invw1 {
    #[inline(always)]
    fn from(val: u8) -> Invw1 {
        Invw1::from_bits(val)
    }
}
impl From<Invw1> for u8 {
    #[inline(always)]
    fn from(val: Invw1) -> u8 {
        Invw1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lacc {
    #[doc = "Read."]
    Read = 0x0,
    #[doc = "Write."]
    Write = 0x01,
}
impl Lacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lacc {
    #[inline(always)]
    fn from(val: u8) -> Lacc {
        Lacc::from_bits(val)
    }
}
impl From<Lacc> for u8 {
    #[inline(always)]
    fn from(val: Lacc) -> u8 {
        Lacc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ladsel {
    #[doc = "Cache."]
    CacheAddr = 0x0,
    #[doc = "Physical."]
    PhysAddr = 0x01,
}
impl Ladsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ladsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ladsel {
    #[inline(always)]
    fn from(val: u8) -> Ladsel {
        Ladsel::from_bits(val)
    }
}
impl From<Ladsel> for u8 {
    #[inline(always)]
    fn from(val: Ladsel) -> u8 {
        Ladsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcimb {
    #[doc = "Initial state 0."]
    Lcimb0 = 0x0,
    #[doc = "Initial state 1."]
    Lcimb1 = 0x01,
}
impl Lcimb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcimb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcimb {
    #[inline(always)]
    fn from(val: u8) -> Lcimb {
        Lcimb::from_bits(val)
    }
}
impl From<Lcimb> for u8 {
    #[inline(always)]
    fn from(val: Lcimb) -> u8 {
        Lcimb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcivb {
    #[doc = "Initial state 0."]
    Lcivb0 = 0x0,
    #[doc = "Initial state 1."]
    Lcivb1 = 0x01,
}
impl Lcivb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcivb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcivb {
    #[inline(always)]
    fn from(val: u8) -> Lcivb {
        Lcivb::from_bits(val)
    }
}
impl From<Lcivb> for u8 {
    #[inline(always)]
    fn from(val: Lcivb) -> u8 {
        Lcivb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcmd {
    #[doc = "Search and read or write."]
    SearchRw = 0x0,
    #[doc = "Invalidate."]
    Invalidate = 0x01,
    #[doc = "Push."]
    Push = 0x02,
    #[doc = "Clear."]
    Clear = 0x03,
}
impl Lcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcmd {
    #[inline(always)]
    fn from(val: u8) -> Lcmd {
        Lcmd::from_bits(val)
    }
}
impl From<Lcmd> for u8 {
    #[inline(always)]
    fn from(val: Lcmd) -> u8 {
        Lcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcway {
    #[doc = "Way 0."]
    Way0 = 0x0,
    #[doc = "Way 1."]
    Way1 = 0x01,
}
impl Lcway {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcway {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcway {
    #[inline(always)]
    fn from(val: u8) -> Lcway {
        Lcway::from_bits(val)
    }
}
impl From<Lcway> for u8 {
    #[inline(always)]
    fn from(val: Lcway) -> u8 {
        Lcway::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pushw0 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "Push all modified lines in way 0."]
    Pushw0 = 0x01,
}
impl Pushw0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pushw0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pushw0 {
    #[inline(always)]
    fn from(val: u8) -> Pushw0 {
        Pushw0::from_bits(val)
    }
}
impl From<Pushw0> for u8 {
    #[inline(always)]
    fn from(val: Pushw0) -> u8 {
        Pushw0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pushw1 {
    #[doc = "No operation."]
    NoOperation = 0x0,
    #[doc = "Push all modified lines in way 1."]
    Pushw1 = 0x01,
}
impl Pushw1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pushw1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pushw1 {
    #[inline(always)]
    fn from(val: u8) -> Pushw1 {
        Pushw1::from_bits(val)
    }
}
impl From<Pushw1> for u8 {
    #[inline(always)]
    fn from(val: Pushw1) -> u8 {
        Pushw1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdsel {
    #[doc = "Data."]
    Data = 0x0,
    #[doc = "Tag."]
    Tag = 0x01,
}
impl Tdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdsel {
    #[inline(always)]
    fn from(val: u8) -> Tdsel {
        Tdsel::from_bits(val)
    }
}
impl From<Tdsel> for u8 {
    #[inline(always)]
    fn from(val: Tdsel) -> u8 {
        Tdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wsel {
    #[doc = "Way 0."]
    Way0 = 0x0,
    #[doc = "Way 1."]
    Way1 = 0x01,
}
impl Wsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wsel {
    #[inline(always)]
    fn from(val: u8) -> Wsel {
        Wsel::from_bits(val)
    }
}
impl From<Wsel> for u8 {
    #[inline(always)]
    fn from(val: Wsel) -> u8 {
        Wsel::to_bits(val)
    }
}
