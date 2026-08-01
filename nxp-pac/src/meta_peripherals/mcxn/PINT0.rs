#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Pin Interrupts and Pattern Match."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pint0 {
    ptr: *mut u8,
}
unsafe impl Send for Pint0 {}
unsafe impl Sync for Pint0 {}
impl Pint0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin Interrupt Mode."]
    #[inline(always)]
    pub const fn isel(self) -> crate::pac::common::Reg<Isel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable."]
    #[inline(always)]
    pub const fn ienr(self) -> crate::pac::common::Reg<Ienr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set."]
    #[inline(always)]
    pub const fn sienr(self) -> crate::pac::common::Reg<Sienr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear."]
    #[inline(always)]
    pub const fn cienr(self) -> crate::pac::common::Reg<Cienr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable."]
    #[inline(always)]
    pub const fn ienf(self) -> crate::pac::common::Reg<Ienf, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set."]
    #[inline(always)]
    pub const fn sienf(self) -> crate::pac::common::Reg<Sienf, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear."]
    #[inline(always)]
    pub const fn cienf(self) -> crate::pac::common::Reg<Cienf, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Pin Interrupt Rising Edge."]
    #[inline(always)]
    pub const fn rise(self) -> crate::pac::common::Reg<Rise, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pin Interrupt Falling Edge."]
    #[inline(always)]
    pub const fn fall(self) -> crate::pac::common::Reg<Fall, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin Interrupt Status."]
    #[inline(always)]
    pub const fn ist(self) -> crate::pac::common::Reg<Ist, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pattern-Match Interrupt Control."]
    #[inline(always)]
    pub const fn pmctrl(self) -> crate::pac::common::Reg<Pmctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Pattern-Match Interrupt Bit-Slice Source."]
    #[inline(always)]
    pub const fn pmsrc(self) -> crate::pac::common::Reg<Pmsrc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Pattern-Match Interrupt Bit Slice Configuration."]
    #[inline(always)]
    pub const fn pmcfg(self) -> crate::pac::common::Reg<Pmcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienf(pub u32);
impl Cienf {
    #[doc = "Writes 0 to IENF."]
    #[must_use]
    #[inline(always)]
    pub const fn cenaf(&self) -> Cenaf {
        let val = (self.0 >> 0usize) & 0xff;
        Cenaf::from_bits(val as u8)
    }
    #[doc = "Writes 0 to IENF."]
    #[inline(always)]
    pub const fn set_cenaf(&mut self, val: Cenaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cienf {
    #[inline(always)]
    fn default() -> Cienf {
        Cienf(0)
    }
}
impl core::fmt::Debug for Cienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cienf")
            .field("cenaf", &self.cenaf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cienf {{ cenaf: {:?} }}", self.cenaf())
    }
}
#[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienr(pub u32);
impl Cienr {
    #[doc = "Clear bits in IENR."]
    #[must_use]
    #[inline(always)]
    pub const fn cenrl(&self) -> Cenrl {
        let val = (self.0 >> 0usize) & 0xff;
        Cenrl::from_bits(val as u8)
    }
    #[doc = "Clear bits in IENR."]
    #[inline(always)]
    pub const fn set_cenrl(&mut self, val: Cenrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cienr {
    #[inline(always)]
    fn default() -> Cienr {
        Cienr(0)
    }
}
impl core::fmt::Debug for Cienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cienr")
            .field("cenrl", &self.cenrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cienr {{ cenrl: {:?} }}", self.cenrl())
    }
}
#[doc = "Pin Interrupt Falling Edge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fall(pub u32);
impl Fall {
    #[doc = "Falling-Edge Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn fdet(&self) -> Fdet {
        let val = (self.0 >> 0usize) & 0xff;
        Fdet::from_bits(val as u8)
    }
    #[doc = "Falling-Edge Detect."]
    #[inline(always)]
    pub const fn set_fdet(&mut self, val: Fdet) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Fall {
    #[inline(always)]
    fn default() -> Fall {
        Fall(0)
    }
}
impl core::fmt::Debug for Fall {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fall").field("fdet", &self.fdet()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fall {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fall {{ fdet: {:?} }}", self.fdet())
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienf(pub u32);
impl Ienf {
    #[doc = "Enables Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn enaf(&self) -> Enaf {
        let val = (self.0 >> 0usize) & 0xff;
        Enaf::from_bits(val as u8)
    }
    #[doc = "Enables Interrupt."]
    #[inline(always)]
    pub const fn set_enaf(&mut self, val: Enaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ienf {
    #[inline(always)]
    fn default() -> Ienf {
        Ienf(0)
    }
}
impl core::fmt::Debug for Ienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienf").field("enaf", &self.enaf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ienf {{ enaf: {:?} }}", self.enaf())
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienr(pub u32);
impl Ienr {
    #[doc = "Enables Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn enrl(&self) -> Enrl {
        let val = (self.0 >> 0usize) & 0xff;
        Enrl::from_bits(val as u8)
    }
    #[doc = "Enables Interrupt."]
    #[inline(always)]
    pub const fn set_enrl(&mut self, val: Enrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ienr {
    #[inline(always)]
    fn default() -> Ienr {
        Ienr(0)
    }
}
impl core::fmt::Debug for Ienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienr").field("enrl", &self.enrl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ienr {{ enrl: {:?} }}", self.enrl())
    }
}
#[doc = "Pin Interrupt Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isel(pub u32);
impl Isel {
    #[doc = "Interrupt mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pmode(&self) -> Pmode {
        let val = (self.0 >> 0usize) & 0xff;
        Pmode::from_bits(val as u8)
    }
    #[doc = "Interrupt mode."]
    #[inline(always)]
    pub const fn set_pmode(&mut self, val: Pmode) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Isel {
    #[inline(always)]
    fn default() -> Isel {
        Isel(0)
    }
}
impl core::fmt::Debug for Isel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isel")
            .field("pmode", &self.pmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isel {{ pmode: {:?} }}", self.pmode())
    }
}
#[doc = "Pin Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ist(pub u32);
impl Ist {
    #[doc = "Pin Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pstat(&self) -> Pstat {
        let val = (self.0 >> 0usize) & 0xff;
        Pstat::from_bits(val as u8)
    }
    #[doc = "Pin Interrupt Status."]
    #[inline(always)]
    pub const fn set_pstat(&mut self, val: Pstat) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ist {
    #[inline(always)]
    fn default() -> Ist {
        Ist(0)
    }
}
impl core::fmt::Debug for Ist {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ist").field("pstat", &self.pstat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ist {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ist {{ pstat: {:?} }}", self.pstat())
    }
}
#[doc = "Pattern-Match Interrupt Bit Slice Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcfg(pub u32);
impl Pmcfg {
    #[doc = "Determines whether the slice is an endpoint. The slice is not an endpoint. The slice is the endpoint of a product term (minterm). The corresponding pin interrupt in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts(&self, n: usize) -> ProdEndpts {
        assert!(n < 7usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        ProdEndpts::from_bits(val as u8)
    }
    #[doc = "Determines whether the slice is an endpoint. The slice is not an endpoint. The slice is the endpoint of a product term (minterm). The corresponding pin interrupt in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts(&mut self, n: usize, val: ProdEndpts) {
        assert!(n < 7usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Match Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg(&self, n: usize) -> Cfg {
        assert!(n < 8usize);
        let offs = 8usize + n * 3usize;
        let val = (self.0 >> offs) & 0x07;
        Cfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration."]
    #[inline(always)]
    pub const fn set_cfg(&mut self, n: usize, val: Cfg) {
        assert!(n < 8usize);
        let offs = 8usize + n * 3usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
}
impl Default for Pmcfg {
    #[inline(always)]
    fn default() -> Pmcfg {
        Pmcfg(0)
    }
}
impl core::fmt::Debug for Pmcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcfg")
            .field("prod_endpts[0]", &self.prod_endpts(0usize))
            .field("prod_endpts[1]", &self.prod_endpts(1usize))
            .field("prod_endpts[2]", &self.prod_endpts(2usize))
            .field("prod_endpts[3]", &self.prod_endpts(3usize))
            .field("prod_endpts[4]", &self.prod_endpts(4usize))
            .field("prod_endpts[5]", &self.prod_endpts(5usize))
            .field("prod_endpts[6]", &self.prod_endpts(6usize))
            .field("cfg[0]", &self.cfg(0usize))
            .field("cfg[1]", &self.cfg(1usize))
            .field("cfg[2]", &self.cfg(2usize))
            .field("cfg[3]", &self.cfg(3usize))
            .field("cfg[4]", &self.cfg(4usize))
            .field("cfg[5]", &self.cfg(5usize))
            .field("cfg[6]", &self.cfg(6usize))
            .field("cfg[7]", &self.cfg(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmcfg {{ prod_endpts[0]: {:?}, prod_endpts[1]: {:?}, prod_endpts[2]: {:?}, prod_endpts[3]: {:?}, prod_endpts[4]: {:?}, prod_endpts[5]: {:?}, prod_endpts[6]: {:?}, cfg[0]: {:?}, cfg[1]: {:?}, cfg[2]: {:?}, cfg[3]: {:?}, cfg[4]: {:?}, cfg[5]: {:?}, cfg[6]: {:?}, cfg[7]: {:?} }}",
            self.prod_endpts(0usize),
            self.prod_endpts(1usize),
            self.prod_endpts(2usize),
            self.prod_endpts(3usize),
            self.prod_endpts(4usize),
            self.prod_endpts(5usize),
            self.prod_endpts(6usize),
            self.cfg(0usize),
            self.cfg(1usize),
            self.cfg(2usize),
            self.cfg(3usize),
            self.cfg(4usize),
            self.cfg(5usize),
            self.cfg(6usize),
            self.cfg(7usize)
        )
    }
}
#[doc = "Pattern-Match Interrupt Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrl(pub u32);
impl Pmctrl {
    #[doc = "Specifies whether the pin interrupts are controlled by the pin interrupt function or by the pattern-match function. If this value is 0b, interrupts are driven in response to the standard pin interrupt function. If this value is 1b, interrupts are driven in response to pattern matches."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_pmatch(&self) -> SelPmatch {
        let val = (self.0 >> 0usize) & 0x01;
        SelPmatch::from_bits(val as u8)
    }
    #[doc = "Specifies whether the pin interrupts are controlled by the pin interrupt function or by the pattern-match function. If this value is 0b, interrupts are driven in response to the standard pin interrupt function. If this value is 1b, interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub const fn set_sel_pmatch(&mut self, val: SelPmatch) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output, when the specified Boolean expression evaluates to true. If this value is 0b, RXEV output to the CPU is disabled. If this value is 1b, RXEV output to the CPU is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_rxev(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output, when the specified Boolean expression evaluates to true. If this value is 0b, RXEV output to the CPU is disabled. If this value is 1b, RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub const fn set_ena_rxev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pattern Matches."]
    #[must_use]
    #[inline(always)]
    pub const fn pmat(&self) -> Pmat {
        let val = (self.0 >> 24usize) & 0xff;
        Pmat::from_bits(val as u8)
    }
    #[doc = "Pattern Matches."]
    #[inline(always)]
    pub const fn set_pmat(&mut self, val: Pmat) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pmctrl {
    #[inline(always)]
    fn default() -> Pmctrl {
        Pmctrl(0)
    }
}
impl core::fmt::Debug for Pmctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmctrl")
            .field("sel_pmatch", &self.sel_pmatch())
            .field("ena_rxev", &self.ena_rxev())
            .field("pmat", &self.pmat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmctrl {{ sel_pmatch: {:?}, ena_rxev: {=bool:?}, pmat: {:?} }}",
            self.sel_pmatch(),
            self.ena_rxev(),
            self.pmat()
        )
    }
}
#[doc = "Pattern-Match Interrupt Bit-Slice Source."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmsrc(pub u32);
impl Pmsrc {
    #[doc = "Selects the input source for the bit slice."]
    #[must_use]
    #[inline(always)]
    pub const fn src(&self, n: usize) -> Src {
        assert!(n < 8usize);
        let offs = 8usize + n * 3usize;
        let val = (self.0 >> offs) & 0x07;
        Src::from_bits(val as u8)
    }
    #[doc = "Selects the input source for the bit slice."]
    #[inline(always)]
    pub const fn set_src(&mut self, n: usize, val: Src) {
        assert!(n < 8usize);
        let offs = 8usize + n * 3usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
}
impl Default for Pmsrc {
    #[inline(always)]
    fn default() -> Pmsrc {
        Pmsrc(0)
    }
}
impl core::fmt::Debug for Pmsrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmsrc")
            .field("src[0]", &self.src(0usize))
            .field("src[1]", &self.src(1usize))
            .field("src[2]", &self.src(2usize))
            .field("src[3]", &self.src(3usize))
            .field("src[4]", &self.src(4usize))
            .field("src[5]", &self.src(5usize))
            .field("src[6]", &self.src(6usize))
            .field("src[7]", &self.src(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmsrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmsrc {{ src[0]: {:?}, src[1]: {:?}, src[2]: {:?}, src[3]: {:?}, src[4]: {:?}, src[5]: {:?}, src[6]: {:?}, src[7]: {:?} }}",
            self.src(0usize),
            self.src(1usize),
            self.src(2usize),
            self.src(3usize),
            self.src(4usize),
            self.src(5usize),
            self.src(6usize),
            self.src(7usize)
        )
    }
}
#[doc = "Pin Interrupt Rising Edge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rise(pub u32);
impl Rise {
    #[doc = "Rising-Edge Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn rdet(&self) -> Rdet {
        let val = (self.0 >> 0usize) & 0xff;
        Rdet::from_bits(val as u8)
    }
    #[doc = "Rising-Edge Detect."]
    #[inline(always)]
    pub const fn set_rdet(&mut self, val: Rdet) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Rise {
    #[inline(always)]
    fn default() -> Rise {
        Rise(0)
    }
}
impl core::fmt::Debug for Rise {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rise").field("rdet", &self.rdet()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rise {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rise {{ rdet: {:?} }}", self.rdet())
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienf(pub u32);
impl Sienf {
    #[doc = "Write 1 to this address to clear to disable interrupts. Bit a sets bit n in IENF."]
    #[must_use]
    #[inline(always)]
    pub const fn setenaf(&self) -> Setenaf {
        let val = (self.0 >> 0usize) & 0xff;
        Setenaf::from_bits(val as u8)
    }
    #[doc = "Write 1 to this address to clear to disable interrupts. Bit a sets bit n in IENF."]
    #[inline(always)]
    pub const fn set_setenaf(&mut self, val: Setenaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Sienf {
    #[inline(always)]
    fn default() -> Sienf {
        Sienf(0)
    }
}
impl core::fmt::Debug for Sienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sienf")
            .field("setenaf", &self.setenaf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sienf {{ setenaf: {:?} }}", self.setenaf())
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienr(pub u32);
impl Sienr {
    #[doc = "Configures IENR."]
    #[must_use]
    #[inline(always)]
    pub const fn setenrl(&self) -> Setenrl {
        let val = (self.0 >> 0usize) & 0xff;
        Setenrl::from_bits(val as u8)
    }
    #[doc = "Configures IENR."]
    #[inline(always)]
    pub const fn set_setenrl(&mut self, val: Setenrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Sienr {
    #[inline(always)]
    fn default() -> Sienr {
        Sienr(0)
    }
}
impl core::fmt::Debug for Sienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sienr")
            .field("setenrl", &self.setenrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sienr {{ setenrl: {:?} }}", self.setenrl())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cenaf(u8);
impl Cenaf {
    #[doc = "No operation."]
    pub const Cenaf0: Self = Self(0x0);
    #[doc = "LOW-active interrupt selected or falling-edge interrupt disabled."]
    pub const Cenaf1: Self = Self(0x01);
}
impl Cenaf {
    pub const fn from_bits(val: u8) -> Cenaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cenaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Cenaf0"),
            0x01 => f.write_str("Cenaf1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cenaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Cenaf0"),
            0x01 => defmt::write!(f, "Cenaf1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cenaf {
    #[inline(always)]
    fn from(val: u8) -> Cenaf {
        Cenaf::from_bits(val)
    }
}
impl From<Cenaf> for u8 {
    #[inline(always)]
    fn from(val: Cenaf) -> u8 {
        Cenaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cenrl(u8);
impl Cenrl {
    #[doc = "No operation."]
    pub const Cenrl0: Self = Self(0x0);
    #[doc = "Disable rising edge or level interrupt."]
    pub const Cenrl1: Self = Self(0x01);
}
impl Cenrl {
    pub const fn from_bits(val: u8) -> Cenrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cenrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Cenrl0"),
            0x01 => f.write_str("Cenrl1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cenrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Cenrl0"),
            0x01 => defmt::write!(f, "Cenrl1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cenrl {
    #[inline(always)]
    fn from(val: u8) -> Cenrl {
        Cenrl::from_bits(val)
    }
}
impl From<Cenrl> for u8 {
    #[inline(always)]
    fn from(val: Cenrl) -> u8 {
        Cenrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg {
    #[doc = "Constant HIGH."]
    ConstantHigh = 0x0,
    #[doc = "Sticky rising edge."]
    StickyRisingEdge = 0x01,
    #[doc = "Sticky falling edge."]
    StickyFallingEdge = 0x02,
    #[doc = "Sticky rising or falling edge."]
    StickyRisingFallingEdge = 0x03,
    #[doc = "High level."]
    HighLevel = 0x04,
    #[doc = "Low level."]
    LowLevel = 0x05,
    #[doc = "Constant 0."]
    ConstantZero = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)."]
    Event = 0x07,
}
impl Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg {
    #[inline(always)]
    fn from(val: u8) -> Cfg {
        Cfg::from_bits(val)
    }
}
impl From<Cfg> for u8 {
    #[inline(always)]
    fn from(val: Cfg) -> u8 {
        Cfg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enaf(u8);
impl Enaf {
    #[doc = "Disable (set active interrupt level LOW)."]
    pub const Enaf0: Self = Self(0x0);
    #[doc = "Enable (set active interrupt level HIGH)."]
    pub const Enaf1: Self = Self(0x01);
}
impl Enaf {
    pub const fn from_bits(val: u8) -> Enaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Enaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Enaf0"),
            0x01 => f.write_str("Enaf1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Enaf0"),
            0x01 => defmt::write!(f, "Enaf1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Enaf {
    #[inline(always)]
    fn from(val: u8) -> Enaf {
        Enaf::from_bits(val)
    }
}
impl From<Enaf> for u8 {
    #[inline(always)]
    fn from(val: Enaf) -> u8 {
        Enaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enrl(u8);
impl Enrl {
    #[doc = "In bit n disables the corresponding interrupt."]
    pub const Enrl0: Self = Self(0x0);
    #[doc = "In bit n enables the corresponding interrupt."]
    pub const Enrl1: Self = Self(0x01);
}
impl Enrl {
    pub const fn from_bits(val: u8) -> Enrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Enrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Enrl0"),
            0x01 => f.write_str("Enrl1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Enrl0"),
            0x01 => defmt::write!(f, "Enrl1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Enrl {
    #[inline(always)]
    fn from(val: u8) -> Enrl {
        Enrl::from_bits(val)
    }
}
impl From<Enrl> for u8 {
    #[inline(always)]
    fn from(val: Enrl) -> u8 {
        Enrl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fdet(u8);
impl Fdet {
    #[doc = "Read 0- No falling edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation."]
    pub const Fdet0: Self = Self(0x0);
    #[doc = "Read 1- Falling edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear falling-edge detection for this bit."]
    pub const Fdet1: Self = Self(0x01);
}
impl Fdet {
    pub const fn from_bits(val: u8) -> Fdet {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fdet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Fdet0"),
            0x01 => f.write_str("Fdet1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdet {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Fdet0"),
            0x01 => defmt::write!(f, "Fdet1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fdet {
    #[inline(always)]
    fn from(val: u8) -> Fdet {
        Fdet::from_bits(val)
    }
}
impl From<Fdet> for u8 {
    #[inline(always)]
    fn from(val: Fdet) -> u8 {
        Fdet::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pmat(u8);
impl Pmat {
    #[doc = "The corresponding product term is matched by the current state of the appropriate inputs."]
    pub const Pmat1: Self = Self(0x01);
}
impl Pmat {
    pub const fn from_bits(val: u8) -> Pmat {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pmat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Pmat1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Pmat1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pmat {
    #[inline(always)]
    fn from(val: u8) -> Pmat {
        Pmat::from_bits(val)
    }
}
impl From<Pmat> for u8 {
    #[inline(always)]
    fn from(val: Pmat) -> u8 {
        Pmat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pmode(u8);
impl Pmode {
    #[doc = "In bit n configures the interrupt to be edge-sensitive."]
    pub const Isel0: Self = Self(0x0);
    #[doc = "In bit n configures the interrupt to be level-sensitive."]
    pub const Isel1: Self = Self(0x01);
}
impl Pmode {
    pub const fn from_bits(val: u8) -> Pmode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Isel0"),
            0x01 => f.write_str("Isel1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Isel0"),
            0x01 => defmt::write!(f, "Isel1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pmode {
    #[inline(always)]
    fn from(val: u8) -> Pmode {
        Pmode::from_bits(val)
    }
}
impl From<Pmode> for u8 {
    #[inline(always)]
    fn from(val: Pmode) -> u8 {
        Pmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Endpoint."]
    Endpoint = 0x01,
}
impl ProdEndpts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts {
        ProdEndpts::from_bits(val)
    }
}
impl From<ProdEndpts> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts) -> u8 {
        ProdEndpts::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pstat(u8);
impl Pstat {
    #[doc = "Read 0- Interrupt is not requested, Write 0- No operation."]
    pub const Pstat0: Self = Self(0x0);
    #[doc = "Read 1- Interrupt is requested, Write 1 (edge-sensitive)- clear rising- and falling-edge detection for this pin, Write 1 (level-sensitive)- switch the active level for this pin in."]
    pub const Pstat1: Self = Self(0x01);
}
impl Pstat {
    pub const fn from_bits(val: u8) -> Pstat {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Pstat0"),
            0x01 => f.write_str("Pstat1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pstat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Pstat0"),
            0x01 => defmt::write!(f, "Pstat1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pstat {
    #[inline(always)]
    fn from(val: u8) -> Pstat {
        Pstat::from_bits(val)
    }
}
impl From<Pstat> for u8 {
    #[inline(always)]
    fn from(val: Pstat) -> u8 {
        Pstat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rdet(u8);
impl Rdet {
    #[doc = "Read 0- No rising edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation."]
    pub const Rdet0: Self = Self(0x0);
    #[doc = "Read 1- Rising edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear rising-edge detection for this pin."]
    pub const Rdet1: Self = Self(0x01);
}
impl Rdet {
    pub const fn from_bits(val: u8) -> Rdet {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Rdet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Rdet0"),
            0x01 => f.write_str("Rdet1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdet {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Rdet0"),
            0x01 => defmt::write!(f, "Rdet1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Rdet {
    #[inline(always)]
    fn from(val: u8) -> Rdet {
        Rdet::from_bits(val)
    }
}
impl From<Rdet> for u8 {
    #[inline(always)]
    fn from(val: Rdet) -> u8 {
        Rdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelPmatch {
    #[doc = "Pin interrupt."]
    PinInterrupt = 0x0,
    #[doc = "Pattern match."]
    PatternMatch = 0x01,
}
impl SelPmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelPmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelPmatch {
    #[inline(always)]
    fn from(val: u8) -> SelPmatch {
        SelPmatch::from_bits(val)
    }
}
impl From<SelPmatch> for u8 {
    #[inline(always)]
    fn from(val: SelPmatch) -> u8 {
        SelPmatch::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setenaf(u8);
impl Setenaf {
    #[doc = "Writes 0 to IENF."]
    pub const Setenaf0: Self = Self(0x0);
    #[doc = "Select HIGH-active interrupt or enable falling-edge interrupt."]
    pub const Setenaf1: Self = Self(0x01);
}
impl Setenaf {
    pub const fn from_bits(val: u8) -> Setenaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Setenaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Setenaf0"),
            0x01 => f.write_str("Setenaf1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setenaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Setenaf0"),
            0x01 => defmt::write!(f, "Setenaf1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Setenaf {
    #[inline(always)]
    fn from(val: u8) -> Setenaf {
        Setenaf::from_bits(val)
    }
}
impl From<Setenaf> for u8 {
    #[inline(always)]
    fn from(val: Setenaf) -> u8 {
        Setenaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setenrl(u8);
impl Setenrl {
    #[doc = "No operation for interrupt n."]
    pub const Setenrl0: Self = Self(0x0);
    #[doc = "Enable rising edge or level interrupt for interrupt n."]
    pub const Setenrl1: Self = Self(0x01);
}
impl Setenrl {
    pub const fn from_bits(val: u8) -> Setenrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Setenrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Setenrl0"),
            0x01 => f.write_str("Setenrl1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setenrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Setenrl0"),
            0x01 => defmt::write!(f, "Setenrl1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Setenrl {
    #[inline(always)]
    fn from(val: u8) -> Setenrl {
        Setenrl::from_bits(val)
    }
}
impl From<Setenrl> for u8 {
    #[inline(always)]
    fn from(val: Setenrl) -> u8 {
        Setenrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)."]
    Input0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)."]
    Input1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)."]
    Input2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)."]
    Input3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)."]
    Input4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)."]
    Input5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)."]
    Input6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)."]
    Input7 = 0x07,
}
impl Src {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src {
    #[inline(always)]
    fn from(val: u8) -> Src {
        Src::from_bits(val)
    }
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(val: Src) -> u8 {
        Src::to_bits(val)
    }
}
